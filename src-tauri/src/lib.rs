// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

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
            cleaner::cancel_scan,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

