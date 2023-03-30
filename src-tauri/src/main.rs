// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod gui;
mod ime;
mod keyboard;
mod watcher;

use gui::handler;

fn main() {
    tauri::Builder::default()
        .setup(gui::setup)
        .invoke_handler(tauri::generate_handler![
            handler::get_enabled,
            handler::set_enabled
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(gui::run_callback);
}
