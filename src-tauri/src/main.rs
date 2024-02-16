// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod hook;
mod ime;
mod keyboard;

use std::{
    process,
    sync::{Arc, RwLock},
};

use config::{ConfigManager, ConfigManagerImpl};
use ime::ImeActivatorImpl;
use keyboard::KeyboardImpl;

fn main() {
    let config_manager: Arc<RwLock<dyn ConfigManager>> =
        Arc::new(RwLock::new(ConfigManagerImpl::default()));

    hook::init(
        config_manager.clone(),
        Box::new(ImeActivatorImpl),
        Box::new(KeyboardImpl),
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to initialize hook: {e}");
        process::exit(1);
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::get_config,
            command::set_is_running,
            command::set_activate_key,
            command::set_deactivate_key,
        ])
        .manage(config_manager)
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("Failed to run tauri application: {e}");
            process::exit(1);
        });
}
