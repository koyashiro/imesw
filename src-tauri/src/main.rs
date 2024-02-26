// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handler;
mod hook;
mod ime_manager;
mod keyboard;
mod setting;
mod setup;
mod tray;
mod window;

fn main() {
    if let Err(e) = tauri::Builder::default()
        .setup(setup::setup)
        .invoke_handler(handler::handler())
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::system_tray_event)
        .on_window_event(window::window_event)
        .run(tauri::generate_context!())
    {
        eprintln!("Failed to run tauri application: {e}");
        std::process::exit(1);
    }
}
