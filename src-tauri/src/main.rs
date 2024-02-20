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

use tauri::{
    CustomMenuItem, Manager, PhysicalSize, SystemTray,
    SystemTrayEvent::{DoubleClick, LeftClick, MenuItemClick},
    SystemTrayMenu, Window,
    WindowEvent::{CloseRequested, Resized},
};

use crate::config::{ConfigManager, ConfigManagerImpl};
use crate::ime::ImeActivatorImpl;
use crate::keyboard::KeyboardImpl;

fn main() {
    tauri::Builder::default()
        .setup({
            move |app| {
                let config_manager: Arc<RwLock<dyn ConfigManager>> = Arc::new(RwLock::new({
                    let mut config_manager = ConfigManagerImpl::new();
                    config_manager.load_or_init()?;
                    config_manager
                }));

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

                const OPEN_CUSTOM_MENU_ITEM_ID: &str = "open";
                const QUIT_CUSTOM_MENU_ITEM_ID: &str = "quit";
                SystemTray::new()
                    .with_menu(
                        SystemTrayMenu::new()
                            .add_item(CustomMenuItem::new(OPEN_CUSTOM_MENU_ITEM_ID, "Open"))
                            .add_item(CustomMenuItem::new(QUIT_CUSTOM_MENU_ITEM_ID, "Quit")),
                    )
                    .on_event({
                        let open_window = move |w: &Window| {
                            w.unminimize().expect("Failed to unminimize the window");
                            w.show().expect("Failed to show the window");
                            w.set_focus()
                                .expect("Failed to set the focus to the window");
                        };
                        move |e| match e {
                            LeftClick { .. } | DoubleClick { .. } => open_window(&main_window),
                            MenuItemClick { id, .. } => match id.as_str() {
                                OPEN_CUSTOM_MENU_ITEM_ID => open_window(&main_window),
                                QUIT_CUSTOM_MENU_ITEM_ID => {
                                    main_window.close().expect("Failed to close the window")
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    })
                    .build(app)?;

                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![
            command::get_config,
            command::set_is_running,
            command::set_activate_key,
            command::set_deactivate_key,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("Failed to run tauri application: {e}");
            process::exit(1);
        });
}
