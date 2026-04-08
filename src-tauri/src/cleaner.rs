use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};
use sysinfo::{Disks, System};
use ignore::WalkBuilder;
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::fs;
use tokio_util::sync::CancellationToken;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum CleanerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Trash error: {0}")]
    Trash(#[from] trash::Error),
}

// Needed so Tauri can return CleanerError natively in invoke calls
impl Serialize for CleanerError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLocation {
    pub id: String,
    pub path: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub hint: String,
    pub impact: String,
    pub risk: String,
    pub size: u64,
    pub size_human: String,
    pub selected: bool,
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_percent: f32,
    pub cpu_count: usize,
    pub memory: MemoryStats,
    pub disk: DiskStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percent: f32,
    pub total_human: String,
    pub used_human: String,
    pub free_human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percent: f32,
    pub total_human: String,
    pub used_human: String,
    pub free_human: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current: usize,
    pub total: usize,
    pub percent: u8,
    pub current_location: String,
    pub found_count: usize,
    pub total_size: u64,
}

// ============================================================================
// GLOBALS & STATE
// ============================================================================

static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();
static DISKS: OnceLock<Mutex<Disks>> = OnceLock::new();

pub struct CleanerState {
    pub scan_results: tokio::sync::Mutex<Vec<CacheLocation>>,
    pub scan_in_progress: tokio::sync::Mutex<bool>,
    pub cancel_token: tokio::sync::Mutex<CancellationToken>,
    pub size_cache: tokio::sync::Mutex<HashMap<String, u64>>,
}

impl CleanerState {
    pub fn new() -> Self {
        SYSTEM.get_or_init(|| Mutex::new(System::new_all()));
        DISKS.get_or_init(|| Mutex::new(Disks::new_with_refreshed_list()));
        Self {
            scan_results: tokio::sync::Mutex::new(Vec::new()),
            scan_in_progress: tokio::sync::Mutex::new(false),
            cancel_token: tokio::sync::Mutex::new(CancellationToken::new()),
            size_cache: tokio::sync::Mutex::new(HashMap::new()),
        }
    }
}

// ============================================================================
// LOGIC
// ============================================================================

pub fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn get_directory_size(path: &Path) -> u64 {
    let total_size = std::sync::atomic::AtomicU64::new(0);
    WalkBuilder::new(path)
        .standard_filters(false)
        .follow_links(false) // Task 16: Symlink Safeties
        .threads(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4))
        .build_parallel()
        .run(|| {
            Box::new(|result| {
                if let Ok(entry) = result {
                    if let Ok(metadata) = std::fs::symlink_metadata(entry.path()) {
                        if metadata.is_file() {
                            total_size.fetch_add(metadata.len(), std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });
    total_size.into_inner()
}

pub fn get_cache_locations() -> Vec<CacheLocation> {
    let os = std::env::consts::OS;
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let mut locations = Vec::new();

    if os == "macos" {
        locations.push(CacheLocation {
            id: "user_caches".into(),
            path: home.join("Library/Caches").to_string_lossy().to_string(),
            name: "User Caches".into(),
            description: "Application caches for the current user".into(),
            category: "System".into(),
            hint: "Speeds up loading times.".into(),
            impact: "Apps will regenerate cache.".into(),
            risk: "low".into(),
            size: 0,
            size_human: "0B".into(),
            selected: true,
            exists: false,
        });
        locations.push(CacheLocation {
            id: "system_logs".into(),
            path: home.join("Library/Logs").to_string_lossy().to_string(),
            name: "User Logs".into(),
            description: "Log files generated by applications".into(),
            category: "System".into(),
            hint: "Used for debugging.".into(),
            impact: "Safe to delete.".into(),
            risk: "low".into(),
            size: 0,
            size_human: "0B".into(),
            selected: true,
            exists: false,
        });
    } else if os == "windows" {
        locations.push(CacheLocation {
            id: "temp_files".into(),
            path: std::env::temp_dir().to_string_lossy().to_string(), // Task 18: Custom Temporary Directory Fallbacks
            name: "Temporary Files".into(),
            description: "System Temp folder".into(),
            category: "System".into(),
            hint: "Temporary files created by apps.".into(),
            impact: "Safe to delete.".into(),
            risk: "low".into(),
            size: 0,
            size_human: "0B".into(),
            selected: true,
            exists: false,
        });
        if let Some(local_appdata) = dirs::data_local_dir() {
            locations.push(CacheLocation {
                id: "local_temp".into(),
                path: local_appdata.join("Temp").to_string_lossy().to_string(),
                name: "User Temp".into(),
                description: "User localized temporary files".into(),
                category: "System".into(),
                hint: "Safe to clean.".into(),
                impact: "Safe to delete.".into(),
                risk: "low".into(),
                size: 0,
                size_human: "0B".into(),
                selected: true,
                exists: false,
            });
        }
    } else if let Some(cache_dir) = dirs::cache_dir() {
        locations.push(CacheLocation {
            id: "user_caches".into(),
            path: cache_dir.to_string_lossy().to_string(),
            name: "User Caches".into(),
            description: "User cache directory (~/.cache)".into(),
            category: "System".into(),
            hint: "Speeds up loading times.".into(),
            impact: "Apps will regenerate cache.".into(),
            risk: "low".into(),
            size: 0,
            size_human: "0B".into(),
            selected: true,
            exists: false,
        });
    }

    locations
}

// ============================================================================
// TAURI COMMANDS
// ============================================================================

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

    // Reset token and grab local clones
    let token = CancellationToken::new();
    *state.cancel_token.lock().await = token.clone();
    
    let sizes_cache = state.size_cache.lock().await.clone();

    let mut locations = get_cache_locations();
    let total = locations.len();
    
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ScanProgress>(64);
    
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        // Task 10: Batched Emissions (Throttle events to max ~60Hz)
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
                // Task 12: File Size Caching
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
            
            // Simulating artificial delay for cool scanning effect (if required)
            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        }

        let state = app_worker.state::<CleanerState>();
        
        // Update global sizes cache
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
                // Task 7: Granular Deletion over directory contents
                let mut read_dir = fs::read_dir(&path).await?;
                while let Ok(Some(entry)) = read_dir.next_entry().await {
                    let child_path = entry.path();
                    
                    // Task 13: Exclude locked files by handling metadata errors explicitly
                    let Ok(file_type) = entry.file_type().await else {
                        continue; 
                    };
                    
                    // Task 16: Symlink Safeties (Do not traverse or delete symlinks)
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
                // Invalidate this location from cache since we cleaned it
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

