// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod cleaner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(cleaner::CleanerState::new())
        .invoke_handler(tauri::generate_handler![
            cleaner::start_scan,
            cleaner::get_scan_results,
            cleaner::clean_items,
            cleaner::get_system_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

