// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ime;
mod keyboard;
mod watcher;

use std::sync::{Arc, Mutex};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent};

use crate::watcher::Watcher;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

fn main() {
    println!("tauri start");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let mut watcher = Watcher::new();
            watcher.start();
            app.manage(Arc::new(Mutex::new(watcher)));
            Ok(())
        })
        .system_tray({
            let open = CustomMenuItem::new("open".to_string(), "Open");
            let quit = CustomMenuItem::new("quit".to_string(), "Quit");
            let tray_menu = SystemTrayMenu::new().add_item(open).add_item(quit);
            SystemTray::new().with_menu(tray_menu)
        })
        .on_window_event(|h| {
            if let WindowEvent::CloseRequested { api, .. } = &h.event() {
                let w = h.window();
                w.hide().unwrap();
                api.prevent_close();
            }
        })
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::DoubleClick { .. } => {
                let w = app.get_window("main").unwrap();
                w.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open" => {
                    let w = app.get_window("main").unwrap();
                    w.show().unwrap();
                }
                "quit" => {
                    let mut watcher = app.state::<Arc<Mutex<Watcher>>>().inner().lock().unwrap();
                    watcher.stop().unwrap();

                    let w = app.get_window("main").unwrap();
                    w.close().unwrap();
                }
                _ => (),
            },
            _ => (),
        })
        .run(tauri::generate_context!())
        .expect("error while building tauri application");

    println!("tauri end");
}
