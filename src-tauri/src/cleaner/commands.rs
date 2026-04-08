use std::path::PathBuf;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::fs;
use tokio_util::sync::CancellationToken;

use super::error::CleanerError;
use super::models::{CacheLocation, CleanerState, ScanProgress, SystemStats, MemoryStats, DiskStats, SYSTEM, DISKS};
use super::scanner::{get_directory_size, human_readable_size};
use super::detectors::get_cache_locations;

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

            let path = PathBuf::from(&loc.path);
            loc.exists = fs::try_exists(&path).await.unwrap_or(false);
            if loc.exists {
                let size = if let Some(cached_size) = sizes_cache.get(&loc.path) {
                    *cached_size
                } else {
                    let path_clone = path.clone();
                    let computed_size = tokio::task::spawn_blocking(move || {
                        get_directory_size(&path_clone)
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
            
            // Simulating artificial delay for cool scanning effect
            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
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
pub async fn clean_items(items: Vec<String>, state: State<'_, CleanerState>) -> Result<u64, CleanerError> {
    let results = state.scan_results.lock().await.clone();
    let mut freed_space = 0;

    for id in items {
        if let Some(loc) = results.iter().find(|l| l.id == id) {
            let path = PathBuf::from(&loc.path);
            let exists = fs::try_exists(&path).await.unwrap_or(false);
            if exists {
                let mut read_dir = fs::read_dir(&path).await?;
                while let Ok(Some(entry)) = read_dir.next_entry().await {
                    let child_path = entry.path();
                    
                    let Ok(file_type) = entry.file_type().await else {
                        continue; 
                    };
                    
                    if file_type.is_symlink() {
                        continue;
                    }
                    
                    let is_dir = file_type.is_dir();
                    
                    if trash::delete(&child_path).is_err() {
                        if is_dir {
                            let _ = fs::remove_dir_all(&child_path).await;
                        } else {
                            let _ = fs::remove_file(&child_path).await;
                        }
                    }
                }
                
                freed_space += loc.size;
                state.size_cache.lock().await.remove(&loc.path);
            }
        }
    }

    Ok(freed_space)
}

#[tauri::command]
pub fn get_system_stats() -> SystemStats {
    let mut sys = SYSTEM.get().expect("SYSTEM not initialized").lock().unwrap();
    let mut disks = DISKS.get().expect("DISKS not initialized").lock().unwrap();
    
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    for disk in disks.list_mut() {
        disk.refresh();
    }

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

    SystemStats {
        cpu_percent: sys.global_cpu_usage(),
        cpu_count: sys.cpus().len(),
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
        }
    }
}
