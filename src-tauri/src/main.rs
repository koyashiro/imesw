// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod hook;
mod ime;
mod keyboard;
mod tray;

use std::{process, sync::Arc};

use tauri::{
    Manager, PhysicalSize,
    WindowEvent::{CloseRequested, Resized},
};
use tray::SystemTrayUpdaterImpl;

use crate::config::{ConfigManager, ConfigManagerImpl};
use crate::ime::ImeActivatorImpl;
use crate::keyboard::KeyboardImpl;

fn main() {
    tauri::Builder::default()
        .system_tray(tray::system_tray())
        .setup(move |app| {
            let config = config::read_or_default();

            let system_tray_handle = app
                .tray_handle()
                .get_item(tray::IS_RUNNING_CUSTOM_MENU_ITEM_ID);
            let system_tray_updater = SystemTrayUpdaterImpl::new(system_tray_handle);
            let config_manager: Arc<dyn ConfigManager> =
                Arc::new(ConfigManagerImpl::new(config, system_tray_updater));

            hook::init(
                config_manager.clone(),
                Box::new(ImeActivatorImpl),
                Box::new(KeyboardImpl),
            )?;

            app.manage(config_manager);

            let main_window_label = &app.config().tauri.windows[0].label;
            let main_window = app.get_window(main_window_label).unwrap();
            main_window.on_window_event({
                let w = main_window.clone();
                move |e| match e {
                    CloseRequested { api, .. } => {
                        w.hide().expect("Failed to hide the window");
                        api.prevent_close();
                    }
                    Resized(PhysicalSize {
                        width: 0,
                        height: 0,
                    }) => {
                        w.hide().expect("Failed to hide the window");
                    }
                    _ => (),
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::get_config,
            command::set_is_running,
            command::set_activate_key,
            command::set_deactivate_key,
        ])
        .on_system_tray_event(tray::system_tray_event)
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("Failed to run tauri application: {e}");
            process::exit(1);
        });
}
