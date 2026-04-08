use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Emitter
};

mod cleaner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let open_i = MenuItem::with_id(app, "open", "Open Qleaner", true, None::<&str>)?;
            let clean_i = MenuItem::with_id(app, "clean_now", "Clean Now", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_i, &clean_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app: &tauri::AppHandle, event| match event.id.as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "clean_now" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("start-clean-shortcut", ());
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .manage(cleaner::CleanerState::new())
        .invoke_handler(tauri::generate_handler![
            cleaner::start_scan,
            cleaner::get_scan_results,
            cleaner::clean_items,
            cleaner::get_system_stats,
            cleaner::cancel_scan,
            cleaner::start_leftover_scan,
            cleaner::get_leftover_results,
            cleaner::clean_leftovers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

