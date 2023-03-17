use std::sync::Mutex;

use tauri::{
    App, AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    WindowBuilder, WindowEvent, WindowUrl,
};

use crate::watcher::Watcher;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage({
        let mut watcher = Watcher::new();
        watcher.start();
        Mutex::new(watcher)
    });

    const MAIN_WINDOW_LABEL: &str = "main";
    let window = WindowBuilder::new(
        app,
        MAIN_WINDOW_LABEL,
        WindowUrl::App("index".parse().unwrap()),
    )
    .build()
    .unwrap();

    window.on_window_event({
        let w = window.clone();
        move |e| {
            if let WindowEvent::CloseRequested { api, .. } = e {
                w.hide().unwrap();
                api.prevent_close();
            }
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
            let w = window;
            move |e| match e {
                SystemTrayEvent::DoubleClick { .. } => {
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    OPEN_CUSTOM_MENU_ITEM_ID => {
                        w.show().unwrap();
                        w.set_focus().unwrap();
                    }
                    QUIT_CUSTOM_MENU_ITEM_ID => {
                        w.close().unwrap();
                    }
                    _ => (),
                },
                _ => (),
            }
        })
        .build(app)
        .unwrap();

    Ok(())
}

pub mod handler {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    #[tauri::command]
    pub fn greet(name: &str) -> String {
        format!("Hello, {name}! You've been greeted from Rust!")
    }
}

pub fn run_callback(app: &AppHandle, event: RunEvent) {
    if let RunEvent::Exit = event {
        app.state::<Mutex<Watcher>>()
            .inner()
            .lock()
            .unwrap()
            .stop()
            .unwrap();
    };
}
