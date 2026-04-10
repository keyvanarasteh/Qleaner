// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--cli") {
        #[cfg(unix)]
        sudo::escalate_if_needed().expect("Failed to escalate privileges");
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(async {
            tauri_app_lib::cli::execute().await;
        });
        return;
    }

    // Elevate privileges natively (UAC on Windows, Pkexec/Sudo on Linux, Osascript on Mac)
    // Required to gain access to system-level directory sweeps (e.g. Windows Prefetch)
    // sudo::escalate_if_needed().expect("Failed to escalate privileges");
    tauri_app_lib::run()
}
