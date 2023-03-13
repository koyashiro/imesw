// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ime;
mod keyboard;
mod watcher;

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

use crate::watcher::Watcher;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    println!("tauri start");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let mut watcher = Watcher::new();
            watcher.start();
            app.manage(Arc::new(Mutex::new(watcher)));
            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(move |app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    app.state::<Arc<Mutex<Watcher>>>()
                        .lock()
                        .unwrap()
                        .stop()
                        .unwrap();
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("tauri end");
}
