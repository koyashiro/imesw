// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod hook;
mod ime;
mod keyboard;

use std::{process, sync::Arc};

use tauri::{
    CustomMenuItem, Manager, PhysicalSize, SystemTray,
    SystemTrayEvent::{DoubleClick, LeftClick, MenuItemClick},
    SystemTrayMenu,
    WindowEvent::{CloseRequested, Resized},
};

use crate::config::{ConfigManager, ConfigManagerImpl};
use crate::ime::ImeActivatorImpl;
use crate::keyboard::KeyboardImpl;

fn main() {
    const OPEN_CUSTOM_MENU_ITEM_ID: &str = "open";
    const IS_RUNNING_CUSTOM_MENU_ITEM_ID: &str = "is_running";
    const QUIT_CUSTOM_MENU_ITEM_ID: &str = "quit";

    tauri::Builder::default()
        .setup({
            move |app| {
                let app_handle = app.handle();

                let config_manager: Arc<dyn ConfigManager> =
                    Arc::new(ConfigManagerImpl::new(app_handle));
                config_manager.load_or_init()?;

                hook::init(
                    config_manager.clone(),
                    Box::new(ImeActivatorImpl),
                    Box::new(KeyboardImpl),
                )?;

                app.manage(config_manager.clone());

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
            }
        })
        .invoke_handler(tauri::generate_handler![
            command::get_config,
            command::set_is_running,
            command::set_activate_key,
            command::set_deactivate_key,
        ])
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new(OPEN_CUSTOM_MENU_ITEM_ID, "Open"))
                    .add_item(CustomMenuItem::new(
                        IS_RUNNING_CUSTOM_MENU_ITEM_ID,
                        "Active",
                    ))
                    .add_item(CustomMenuItem::new(QUIT_CUSTOM_MENU_ITEM_ID, "Quit")),
            ),
        )
        .on_system_tray_event(|app_handle, event| {
            let main_window_label = &app_handle.config().tauri.windows[0].label;
            let main_window = app_handle.get_window(main_window_label).unwrap();
            let main_window = &main_window;
            let open_window = move || {
                main_window
                    .unminimize()
                    .expect("Failed to unminimize the window");
                main_window.show().expect("Failed to show the window");
                main_window
                    .set_focus()
                    .expect("Failed to set the focus to the window");
            };
            let close_window = move || main_window.close().expect("Failed to close the window");
            let config_manager = app_handle.state::<Arc<dyn ConfigManager>>();
            match event {
                LeftClick { .. } | DoubleClick { .. } => open_window(),
                MenuItemClick { id, .. } => match id.as_str() {
                    OPEN_CUSTOM_MENU_ITEM_ID => open_window(),
                    IS_RUNNING_CUSTOM_MENU_ITEM_ID => {
                        config_manager
                            .toggle_is_running()
                            .expect("Failed to set is_running");
                    }
                    QUIT_CUSTOM_MENU_ITEM_ID => {
                        close_window();
                    }
                    _ => (),
                },
                _ => (),
            }
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("Failed to run tauri application: {e}");
            process::exit(1);
        });
}
