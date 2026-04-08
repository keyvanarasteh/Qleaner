#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc, clippy::module_name_repetitions)]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Emitter
};

mod cleaner;
pub mod cli;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let file_appender = tracing_appender::rolling::daily(
        dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from(".")).join("qleaner"),
        "app.log",
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    Box::leak(Box::new(guard)); // Leak the guard strategically keeping background logging active entirely
    
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    tracing::info!("Qleaner Core Telemetry initialized. Booting Tauri Engine...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_config_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            if !app_dir.exists() {
                let _ = std::fs::create_dir_all(&app_dir);
            }
            let pool = tauri::async_runtime::block_on(crate::cleaner::db::init_db(&app_dir))
                .expect("Failed to initialize SQLite Audit database.");
            app.manage(pool);

            let open_i = MenuItem::with_id(app, "open", "Open Qleaner", true, None::<&str>)?;
            let clean_i = MenuItem::with_id(app, "clean_now", "Clean Now", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_i, &clean_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().expect("Default window icon missing").clone())
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
            cleaner::get_audit_logs,
            cleaner::get_schedules,
            cleaner::add_schedule,
            cleaner::delete_schedule,
            cleaner::toggle_schedule,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

