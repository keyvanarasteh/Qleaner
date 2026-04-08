use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use sysinfo::{Disks, System};
use tokio_util::sync::CancellationToken;

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
pub struct LeftoverItem {
    pub id: String,
    pub path: String,
    pub name: String,
    pub bundle_id: String,
    pub detection_source: String,
    pub category: String,
    pub confidence: String,
    pub hint: String,
    pub size: u64,
    pub size_human: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_percent: f32,
    pub cpu_count: usize,
    pub cpu_temp: f32,
    pub memory: MemoryStats,
    pub disk: DiskStats,
    pub network: NetworkStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub tx_human: String,
    pub rx_human: String,
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

use sysinfo::Networks;

pub static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();
pub static DISKS: OnceLock<Mutex<Disks>> = OnceLock::new();
pub static NETWORKS: OnceLock<Mutex<Networks>> = OnceLock::new();

pub struct CleanerState {
    pub scan_results: tokio::sync::Mutex<Vec<CacheLocation>>,
    pub scan_in_progress: tokio::sync::Mutex<bool>,
    pub leftover_results: tokio::sync::Mutex<Vec<LeftoverItem>>,
    pub leftover_scan_in_progress: tokio::sync::Mutex<bool>,
    pub cancel_token: tokio::sync::Mutex<CancellationToken>,
    pub size_cache: tokio::sync::Mutex<HashMap<String, u64>>,
}

impl CleanerState {
    pub fn new() -> Self {
        SYSTEM.get_or_init(|| Mutex::new(System::new_all()));
        DISKS.get_or_init(|| Mutex::new(Disks::new_with_refreshed_list()));
        NETWORKS.get_or_init(|| Mutex::new(Networks::new_with_refreshed_list()));
        Self {
            scan_results: tokio::sync::Mutex::new(Vec::new()),
            scan_in_progress: tokio::sync::Mutex::new(false),
            leftover_results: tokio::sync::Mutex::new(Vec::new()),
            leftover_scan_in_progress: tokio::sync::Mutex::new(false),
            cancel_token: tokio::sync::Mutex::new(CancellationToken::new()),
            size_cache: tokio::sync::Mutex::new(HashMap::new()),
        }
    }
}

impl Default for CleanerState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResponse {
    pub freed_bytes: u64,
    pub files_deleted: u32,
    pub errors: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuditHistoryItem {
    pub id: i64,
    pub path: String,
    pub size_reclaimed: i64,
    pub timestamp: String,
    pub signature: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScheduleItem {
    pub id: i64,
    pub cron_expr: String,
    pub is_active: bool,
}
