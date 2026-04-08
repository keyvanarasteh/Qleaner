// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Elevate privileges natively (UAC on Windows, Pkexec/Sudo on Linux, Osascript on Mac)
    // Required to gain access to system-level directory sweeps (e.g. Windows Prefetch)
    sudo::escalate_if_needed().expect("Failed to escalate privileges");
    tauri_app_lib::run()
}
