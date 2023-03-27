use std::sync::Mutex;

use tauri::{
    App, AppHandle, CustomMenuItem, Manager, PhysicalSize, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, Window, WindowBuilder, WindowEvent, WindowUrl,
};

use crate::watcher::Watcher;

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage({
        let mut watcher = Watcher::new();
        watcher.start();
        Mutex::new(watcher)
    });

    const MAIN_WINDOW_LABEL: &str = "main";
    let window =
        WindowBuilder::new(app, MAIN_WINDOW_LABEL, WindowUrl::App("index".parse()?)).build()?;

    window.on_window_event({
        let w = window.clone();
        move |e| match e {
            WindowEvent::CloseRequested { api, .. } => {
                w.hide().expect("Failed to hide the window");
                api.prevent_close();
            }
            WindowEvent::Resized(PhysicalSize {
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
            let w = window;
            fn open_window(w: &Window) {
                w.unminimize().expect("Failed to unminimize the window");
                w.show().expect("Failed to show the window");
                w.set_focus()
                    .expect("Failed to set the focus to the window");
            }
            move |e| match e {
                SystemTrayEvent::LeftClick { .. } | SystemTrayEvent::DoubleClick { .. } => {
                    open_window(&w)
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    OPEN_CUSTOM_MENU_ITEM_ID => open_window(&w),
                    QUIT_CUSTOM_MENU_ITEM_ID => w.close().expect("Failed to close the window"),
                    _ => (),
                },
                _ => (),
            }
        })
        .build(app)?;

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
            .expect("Failed to acquire lock on Watcher.")
            .stop()
            .expect("Failed to stop Watcher.");
    };
}
