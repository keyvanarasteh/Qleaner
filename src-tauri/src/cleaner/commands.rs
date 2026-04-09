use std::path::PathBuf;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::fs;
use tokio_util::sync::CancellationToken;
use anyhow::Context;

use super::error::CleanerError;
use super::models::{CacheLocation, CleanerState, ScanProgress, SystemStats, MemoryStats, DiskStats, NetworkStats, SYSTEM, DISKS, NETWORKS, CleanResponse};
use super::scanner::{get_directory_size, human_readable_size};
use super::detectors::get_cache_locations;
#[cfg(target_os = "macos")]
use super::detectors::{
    get_installed_bundle_ids, detect_container_orphans, 
    detect_group_container_orphans, detect_preference_orphans, 
    detect_app_support_orphans, detect_launch_agent_orphans, detect_cache_orphans
};
use tokio::io::{AsyncWriteExt, AsyncSeekExt};

#[cfg(unix)]
fn is_owned_by_current_user(meta: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    // Safe UID check — no unsafe block needed
    let current_uid = nix::unistd::getuid().as_raw();
    meta.uid() == current_uid
}

#[cfg(not(unix))]
fn is_owned_by_current_user(_meta: &std::fs::Metadata) -> bool {
    true
}

async fn retry_remove<F, Fut>(mut action: F, max_attempts: u32) -> std::io::Result<()>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = std::io::Result<()>>,
{
    let mut attempts = 0;
    loop {
        match action().await {
            Ok(_) => return Ok(()),
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(e);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}

async fn secure_shred_file(path: &std::path::Path) -> std::io::Result<()> {
    if let Ok(metadata) = tokio::fs::metadata(path).await {
        if metadata.is_dir() { return Ok(()); }
        let size = metadata.len();
        if size == 0 { return Ok(()); }
        
        let chunk_size = size.min(1024 * 1024) as usize;
        let mut f = tokio::fs::OpenOptions::new().write(true).open(path).await?;
        
        // Pass 1: Zeros
        let zeros = vec![0u8; chunk_size];
        let mut written = 0;
        while written < size {
            let to_write = (size - written).min(chunk_size as u64) as usize;
            f.write_all(&zeros[..to_write]).await?;
            written += to_write as u64;
        }
        f.sync_all().await?;

        // Pass 2: Ones
        let ones = vec![0xFFu8; chunk_size];
        written = 0;
        f.seek(std::io::SeekFrom::Start(0)).await?;
        while written < size {
            let to_write = (size - written).min(chunk_size as u64) as usize;
            f.write_all(&ones[..to_write]).await?;
            written += to_write as u64;
        }
        f.sync_all().await?;

        // Pass 3: Random
        written = 0;
        f.seek(std::io::SeekFrom::Start(0)).await?;
        use rand::RngCore;
        let mut random_bytes = vec![0u8; chunk_size];
        while written < size {
            let to_write = (size - written).min(chunk_size as u64) as usize;
            {
                let mut rng = rand::thread_rng();
                rng.fill_bytes(&mut random_bytes[..to_write]);
            }
            f.write_all(&random_bytes[..to_write]).await?;
            written += to_write as u64;
        }
        f.sync_all().await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn cancel_scan(state: State<'_, CleanerState>) -> Result<(), CleanerError> {
    state.cancel_token.lock().await.cancel();
    Ok(())
}

#[tauri::command]
pub async fn start_scan(app: AppHandle, state: State<'_, CleanerState>) -> Result<(), CleanerError> {
    let mut in_progress = state.scan_in_progress.lock().await;
    if *in_progress {
        return Ok(());
    }
    *in_progress = true;
    drop(in_progress);

    let token = CancellationToken::new();
    *state.cancel_token.lock().await = token.clone();
    
    let sizes_cache = state.size_cache.lock().await.clone();
    let mut locations = get_cache_locations();
    let total = locations.len();
    
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ScanProgress>(64);
    
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        let throttle_dur = tokio::time::Duration::from_millis(16);
        let mut last_emit = tokio::time::Instant::now();
        
        while let Some(progress) = rx.recv().await {
            if progress.percent == 100 || last_emit.elapsed() >= throttle_dur {
                let _ = app_clone.emit("scan-progress", &progress);
                last_emit = tokio::time::Instant::now();
            }
        }
    });

    let app_worker = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut found_count = 0;
        let mut total_size = 0;
        let mut local_cache_updates = HashMap::new();

        for (i, loc) in locations.iter_mut().enumerate() {
            if token.is_cancelled() {
                break;
            }

            let _ = tx.send(ScanProgress {
                current: i,
                total,
                percent: ((i as f32 / total as f32) * 100.0) as u8,
                current_location: loc.path.clone(),
                found_count,
                total_size,
            }).await;

            let path_str = loc.path.clone();

            if path_str.starts_with("docker://") {
                if let Some(s) = fetch_docker_size(&path_str).await {
                    loc.exists = true;
                    loc.size = s;
                    loc.size_human = human_readable_size(s);
                    if s > 0 {
                        found_count += 1;
                        total_size += s;
                    }
                } else {
                    loc.exists = false;
                }
            } else {
                let path = PathBuf::from(&loc.path);
                loc.exists = fs::try_exists(&path).await.unwrap_or(false);
            if loc.exists {
                let size = if let Some(cached_size) = sizes_cache.get(&loc.path) {
                    *cached_size
                } else {
                    let path_clone = path.clone();
                    let token_clone = token.clone();
                    let computed_size = tokio::task::spawn_blocking(move || {
                        get_directory_size(&path_clone, token_clone)
                    }).await.unwrap_or(0);
                    
                    local_cache_updates.insert(loc.path.clone(), computed_size);
                    computed_size
                };
                
                loc.size = size;
                loc.size_human = human_readable_size(size);
                if size > 0 {
                    found_count += 1;
                    total_size += size;
                }
            }
        }
            // Yield to allow UI events to be processed
            tokio::task::yield_now().await;
        }

        let state = app_worker.state::<CleanerState>();
        
        let mut global_cache = state.size_cache.lock().await;
        for (k, v) in local_cache_updates {
            global_cache.insert(k, v);
        }
        drop(global_cache);

        let mut scan_results = state.scan_results.lock().await;
        *scan_results = locations.clone();
        
        let mut in_progress = state.scan_in_progress.lock().await;
        *in_progress = false;

        let _ = tx.send(ScanProgress {
            current: total,
            total,
            percent: 100,
            current_location: if token.is_cancelled() { "Scan cancelled".into() } else { "Scan complete".into() },
            found_count,
            total_size,
        }).await;
    });

    Ok(())
}

#[tauri::command]
pub async fn get_scan_results(state: State<'_, CleanerState>) -> Result<Vec<CacheLocation>, CleanerError> {
    Ok(state.scan_results.lock().await.clone())
}

#[tauri::command]
pub async fn clean_items(
    items: Vec<String>,
    dry_run: Option<bool>,
    use_shredding: Option<bool>,
    state: State<'_, CleanerState>,
    db_pool: State<'_, sqlx::SqlitePool>,
) -> Result<CleanResponse, CleanerError> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut is_chrome_running = false;
    let mut is_firefox_running = false;

    for process in sys.processes().values() {
        let name = process.name().to_string_lossy().to_lowercase();
        if name.contains("chrome") { is_chrome_running = true; }
        if name.contains("firefox") { is_firefox_running = true; }
    }

    let is_dry_run = dry_run.unwrap_or(false);
    let results = state.scan_results.lock().await.clone();
    
    let mut freed_space = 0;
    let mut files_deleted = 0;
    let mut errors = Vec::new();

    for id in items {
        if let Some(loc) = results.iter().find(|l| l.id == id) {
            let path_lower = loc.path.to_lowercase();
            
            if path_lower.contains("..") {
                errors.push(format!("Path traversal blocked securely: {}", loc.path));
                continue;
            }
            
            if is_chrome_running && (path_lower.contains("google/chrome") || path_lower.contains("google\\chrome")) {
                errors.push(format!("Cannot clean Chrome cache while process is running: {}", loc.path));
                continue;
            }
            if is_firefox_running && (path_lower.contains("mozilla/firefox") || path_lower.contains("mozilla\\firefox")) {
                errors.push(format!("Cannot clean Firefox cache while process is running: {}", loc.path));
                continue;
            }

            #[cfg(not(feature = "dangerous-clean"))]
            if path_lower.contains("system") || path_lower.contains("/var/root") || path_lower.contains("windows\\system") {
                errors.push(format!("Access restricted. Recompile with dangerous-clean feature."));
                continue;
            }

            if loc.path.starts_with("docker://") {
                if !is_dry_run {
                    perform_docker_clean(&loc.path).await;
                    state.size_cache.lock().await.remove(&loc.path);
                }
                freed_space += loc.size;
                files_deleted += 1;
                continue;
            }

            let path = PathBuf::from(&loc.path);
            let exists = fs::try_exists(&path).await.unwrap_or(false);
            if exists {
                if is_dry_run {
                    freed_space += loc.size;
                    // Count top level items that would be deleted
                    if let Ok(mut read_dir) = fs::read_dir(&path).await {
                        while let Ok(Some(_)) = read_dir.next_entry().await {
                            files_deleted += 1;
                        }
                    }
                    continue;
                }

                // Normal execution
                let mut read_dir = match fs::read_dir(&path).await {
                    Ok(rd) => rd,
                    Err(e) => {
                        errors.push(format!("Cannot read dir {}: {}", path.display(), e));
                        continue;
                    }
                };
                
                while let Ok(Some(entry)) = read_dir.next_entry().await {
                    let child_path = entry.path();
                    
                    let Ok(file_type) = entry.file_type().await else {
                        continue; 
                    };
                    
                    if let Ok(meta) = entry.metadata().await {
                        if !is_owned_by_current_user(&meta) {
                            continue; // Skip root-owned caches gracefully
                        }
                    } else {
                        continue;
                    }
                    
                    // Duplicate ownership check removed (FIX-03)

                    if file_type.is_symlink() {
                        continue;
                    }
                    
                    let is_dir = file_type.is_dir();
                    
                    let file_name_lower = child_path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                    if file_name_lower.ends_with(".db") || file_name_lower.ends_with(".sqlite") || file_name_lower == "cache.db" {
                        let wal_path = child_path.with_extension("db-wal");
                        let lock_path = child_path.with_extension("lock");
                        if fs::try_exists(&wal_path).await.unwrap_or(false) || fs::try_exists(&lock_path).await.unwrap_or(false) {
                            continue; // Prevent SQLite/DB corruption by skipping locked files
                        }
                    }

                    let mut use_trash = true;
                    if use_shredding.unwrap_or(false) {
                        if !is_dir {
                            let _ = secure_shred_file(&child_path).await;
                            use_trash = false; // Bypass trash for thoroughly shredded blocks
                        }
                    }
                    
                    let trashed = use_trash && trash::delete(&child_path).is_ok();
                    
                    if !trashed {
                        if is_dir {
                            if let Err(e) = retry_remove(|| {
                                let p = child_path.clone();
                                async move { fs::remove_dir_all(&p).await }
                            }, 3).await {
                                errors.push(format!("Failed deleting dir {}: {}", child_path.display(), e));
                            } else {
                                files_deleted += 1;
                            }
                        } else {
                            if let Err(e) = retry_remove(|| {
                                let p = child_path.clone();
                                async move { fs::remove_file(&p).await }
                            }, 3).await {
                                errors.push(format!("Failed deleting file {}: {}", child_path.display(), e));
                            } else {
                                files_deleted += 1;
                            }
                        }
                    } else {
                        files_deleted += 1;
                    }
                }
                
                freed_space += loc.size;
                state.size_cache.lock().await.remove(&loc.path);
                
                if !is_dry_run {
                    let secret = b"QleanerTelemetryCryptoIntegrity";
                    let _ = super::db::insert_audit_log(&db_pool, &loc.path, loc.size, secret).await;
                }
            }
        }
    }

    state.size_cache.lock().await.shrink_to_fit();

    Ok(CleanResponse {
        freed_bytes: freed_space,
        files_deleted,
        errors,
    })
}

#[tauri::command]
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn get_system_stats() -> SystemStats {
    let mut sys = SYSTEM.get().expect("SYSTEM not initialized").lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut disks = DISKS.get().expect("DISKS not initialized").lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut net = NETWORKS.get().expect("NETWORKS not initialized").lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    for disk in disks.list_mut() {
        disk.refresh();
    }
    net.refresh(true);

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let free_mem = total_mem - used_mem;

    let dt = total_mem as f32;
    let du = used_mem as f32;
    let mem_percent = if dt > 0.0 { (du / dt) * 100.0 } else { 0.0 };

    let mut disk_total = 0;
    let mut disk_used = 0;
    
    for disk in disks.list() {
        disk_total += disk.total_space();
        let avail = disk.available_space();
        disk_used += disk.total_space().saturating_sub(avail);
    }

    let disk_free = disk_total - disk_used;
    let disk_percent = if disk_total > 0 {
        ((disk_used as f64 / disk_total as f64) * 100.0) as f32
    } else {
        0.0
    };

    let components = sysinfo::Components::new_with_refreshed_list();
    let mut cpu_temp = 0.0;
    for component in &components {
        let label = component.label().to_lowercase();
        if label.contains("cpu") || label.contains("core") || label.contains("tctl") {
            cpu_temp = component.temperature().unwrap_or(0.0);
            break;
        }
    }
    
    let mut tx_bytes = 0;
    let mut rx_bytes = 0;
    for (_name, net_data) in net.iter() {
        tx_bytes += net_data.transmitted();
        rx_bytes += net_data.received();
    }

    SystemStats {
        cpu_percent: sys.global_cpu_usage(),
        cpu_count: sys.cpus().len(),
        cpu_temp,
        memory: MemoryStats {
            total: total_mem,
            used: used_mem,
            free: free_mem,
            percent: mem_percent,
            total_human: human_readable_size(total_mem),
            used_human: human_readable_size(used_mem),
            free_human: human_readable_size(free_mem),
        },
        disk: DiskStats {
            total: disk_total,
            used: disk_used,
            free: disk_free,
            percent: disk_percent,
            total_human: human_readable_size(disk_total),
            used_human: human_readable_size(disk_used),
            free_human: human_readable_size(disk_free),
        },
        network: NetworkStats {
            tx_bytes,
            rx_bytes,
            tx_human: format!("{}/s", human_readable_size(tx_bytes)),
            rx_human: format!("{}/s", human_readable_size(rx_bytes)),
        }
    }
}

// ----------------------------------------------------
// LEFTOVER SCAN COMMANDS
// ----------------------------------------------------

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn start_leftover_scan(app: AppHandle, state: State<'_, CleanerState>) -> Result<(), CleanerError> {
    let mut in_progress = state.leftover_scan_in_progress.lock().await;
    if *in_progress {
        return Ok(());
    }
    *in_progress = true;
    drop(in_progress);

    let token = CancellationToken::new();
    *state.cancel_token.lock().await = token.clone();
    
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ScanProgress>(64);
    let app_clone = app.clone();
    
    tauri::async_runtime::spawn(async move {
        let throttle_dur = tokio::time::Duration::from_millis(16);
        let mut last_emit = tokio::time::Instant::now();
        while let Some(progress) = rx.recv().await {
            if progress.percent == 100 || last_emit.elapsed() >= throttle_dur {
                let _ = app_clone.emit("leftover-scan-progress", &progress);
                last_emit = tokio::time::Instant::now();
            }
        }
    });

    let app_worker = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut total_size = 0;
        let mut found_count = 0;
        let mut all_orphans = Vec::new();

        let _ = tx.send(ScanProgress { current: 0, total: 6, percent: 5, current_location: "Getting installed apps...".into(), found_count, total_size }).await;
        
        let installed = get_installed_bundle_ids();

        let _ = tx.send(ScanProgress { current: 1, total: 6, percent: 15, current_location: "Scanning containers...".into(), found_count, total_size }).await;
        let mut containers = detect_container_orphans(&installed);
        for c in &containers { total_size += c.size; found_count += 1; }
        all_orphans.append(&mut containers);

        let _ = tx.send(ScanProgress { current: 2, total: 6, percent: 30, current_location: "Scanning group containers...".into(), found_count, total_size }).await;
        let mut group = detect_group_container_orphans(&installed);
        for c in &group { total_size += c.size; found_count += 1; }
        all_orphans.append(&mut group);

        let _ = tx.send(ScanProgress { current: 3, total: 6, percent: 50, current_location: "Scanning preferences...".into(), found_count, total_size }).await;
        let mut prefs = detect_preference_orphans(&installed, None);
        for c in &prefs { total_size += c.size; found_count += 1; }
        all_orphans.append(&mut prefs);

        let _ = tx.send(ScanProgress { current: 4, total: 6, percent: 70, current_location: "Scanning App Support...".into(), found_count, total_size }).await;
        let mut support = detect_app_support_orphans(&installed, None);
        for c in &support { total_size += c.size; found_count += 1; }
        all_orphans.append(&mut support);

        let _ = tx.send(ScanProgress { current: 5, total: 6, percent: 85, current_location: "Scanning Launch Agents...".into(), found_count, total_size }).await;
        let mut launch = detect_launch_agent_orphans(&installed, None);
        for c in &launch { total_size += c.size; found_count += 1; }
        all_orphans.append(&mut launch);

        let _ = tx.send(ScanProgress { current: 6, total: 6, percent: 95, current_location: "Scanning generic caches...".into(), found_count, total_size }).await;
        let mut caches = detect_cache_orphans(&installed, None);
        for c in &caches { total_size += c.size; found_count += 1; }
        all_orphans.append(&mut caches);

        let state = app_worker.state::<CleanerState>();
        let mut results = state.leftover_results.lock().await;
        *results = all_orphans;

        let mut in_progress = state.leftover_scan_in_progress.lock().await;
        *in_progress = false;

        let _ = tx.send(ScanProgress {
            current: 6,
            total: 6,
            percent: 100,
            current_location: "Scan complete".into(),
            found_count,
            total_size,
        }).await;
    });

    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn start_leftover_scan(_app: AppHandle, _state: State<'_, CleanerState>) -> Result<(), CleanerError> {
    // Leftover orphan scanning is currently macOS-only
    Ok(())
}

#[tauri::command]
pub async fn get_leftover_results(state: State<'_, CleanerState>) -> Result<Vec<super::models::LeftoverItem>, CleanerError> {
    Ok(state.leftover_results.lock().await.clone())
}

#[tauri::command]
pub async fn clean_leftovers(
    items: Vec<String>, 
    state: State<'_, CleanerState>,
    db_pool: State<'_, sqlx::SqlitePool>,
) -> Result<u64, CleanerError> {
    let results = state.leftover_results.lock().await.clone();
    let mut freed_space = 0;

    for id in items {
        if let Some(loc) = results.iter().find(|l| l.id == id) {
            if loc.path.contains("..") { continue; }
            let path = PathBuf::from(&loc.path);
            let exists = fs::try_exists(&path).await.unwrap_or(false);
            if exists {
                if let Ok(metadata) = fs::symlink_metadata(&path).await {
                    let is_dir = metadata.is_dir();
                    if trash::delete(&path).is_err() {
                        if is_dir {
                            let _ = fs::remove_dir_all(&path).await;
                        } else {
                            let _ = fs::remove_file(&path).await;
                        }
                    }
                    freed_space += loc.size;
                    
                    let secret = b"QleanerTelemetryCryptoIntegrity";
                    let _ = super::db::insert_audit_log(&db_pool, &loc.path, loc.size, secret).await;
                }
            }
        }
    }

    Ok(freed_space)
}

use sqlx::Row;

#[tauri::command]
pub async fn get_audit_logs(db_pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<Vec<super::models::AuditHistoryItem>, super::error::CleanerError> {
    let rows = sqlx::query("SELECT id, path, size_reclaimed, timestamp, signature FROM audit_logs ORDER BY timestamp DESC")
        .fetch_all(&*db_pool)
        .await
        .context("Failed retrieving audit logs from SQLite")?;
    let mut logs = Vec::new();
    for row in rows {
        logs.push(super::models::AuditHistoryItem {
            id: row.try_get("id").unwrap_or(0i64),
            path: row.try_get("path").unwrap_or_default(),
            size_reclaimed: row.try_get("size_reclaimed").unwrap_or(0i64),
            timestamp: row.try_get("timestamp").unwrap_or_default(),
            signature: row.try_get("signature").unwrap_or_default(),
        });
    }
    
    Ok(logs)
}

#[tauri::command]
pub async fn get_schedules(db_pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<Vec<super::models::ScheduleItem>, super::error::CleanerError> {
    let rows = sqlx::query("SELECT id, cron_expr, is_active FROM schedules ORDER BY id ASC")
        .fetch_all(&*db_pool)
        .await
        .context("Failed loading scheduling entries from SQLite")?;
        
    let mut schedules = Vec::new();
    for row in rows {
        schedules.push(super::models::ScheduleItem {
            id: row.try_get("id").unwrap_or(0i64),
            cron_expr: row.try_get("cron_expr").unwrap_or_default(),
            is_active: row.try_get("is_active").unwrap_or(false),
        });
    }
    Ok(schedules)
}

#[tauri::command]
pub async fn add_schedule(db_pool: tauri::State<'_, sqlx::SqlitePool>, cron_expr: String) -> Result<(), super::error::CleanerError> {
    sqlx::query("INSERT INTO schedules (cron_expr, is_active) VALUES (?, 1)")
        .bind(cron_expr)
        .execute(&*db_pool)
        .await
        .context("Failed adding schedule to SQLite database")?;
    Ok(())
}

#[tauri::command]
pub async fn delete_schedule(db_pool: tauri::State<'_, sqlx::SqlitePool>, id: i64) -> Result<(), super::error::CleanerError> {
    sqlx::query("DELETE FROM schedules WHERE id = ?")
        .bind(id)
        .execute(&*db_pool)
        .await
        .context("Failed deleting schedule from SQLite database")?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_schedule(db_pool: tauri::State<'_, sqlx::SqlitePool>, id: i64, is_active: bool) -> Result<(), super::error::CleanerError> {
    sqlx::query("UPDATE schedules SET is_active = ? WHERE id = ?")
        .bind(is_active)
        .bind(id)
        .execute(&*db_pool)
        .await
        .context("Failed toggling schedule active state in database")?;
    Ok(())
}


pub(crate) async fn fetch_docker_size(uri: &str) -> Option<u64> {
    if let Ok(output) = tokio::process::Command::new("docker").args(["system", "df", "--format", "{{json .}}"]).output().await {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            let target_type = match uri {
                "docker://build_cache" => "Build Cache",
                "docker://dangling_images" => "Images",
                "docker://stopped_containers" => "Containers",
                "docker://volumes" => "Local Volumes",
                _ => return None,
            };
            
            for line in stdout.lines() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(t) = json.get("Type").and_then(|v| v.as_str()) {
                        if t == target_type {
                            let reclaimable = json.get("ReclaimableSize").and_then(serde_json::Value::as_u64).unwrap_or(0);
                            return Some(reclaimable);
                        }
                    }
                }
            }
        }
    }
    None
}

pub(crate) async fn perform_docker_clean(uri: &str) {
    match uri {
        "docker://build_cache" => {
            let _ = tokio::process::Command::new("docker").args(["builder", "prune", "-a", "-f"]).output().await;
        }
        "docker://dangling_images" => {
            let _ = tokio::process::Command::new("docker").args(["image", "prune", "-f"]).output().await;
        }
        "docker://stopped_containers" => {
            let _ = tokio::process::Command::new("docker").args(["container", "prune", "-f"]).output().await;
        }
        "docker://volumes" => {
            let _ = tokio::process::Command::new("docker").args(["volume", "prune", "-f"]).output().await;
        }
        _ => {}
    }
}

#[tauri::command]
pub async fn check_system_disk_access() -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        let tcc_path = "/Library/Application Support/com.apple.TCC";
        match tokio::fs::metadata(tcc_path).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(false),
            Err(_) => Ok(true), // Fallback if TCC doesn't exist logically
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(true)
    }
}

#[tauri::command]
pub async fn open_privacy_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let _ = tokio::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .spawn();
    }
    Ok(())
}

