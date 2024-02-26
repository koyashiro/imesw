use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray,
    SystemTrayEvent::{self, DoubleClick, LeftClick, MenuItemClick},
    SystemTrayMenu,
    SystemTrayMenuItem::Separator,
    SystemTrayMenuItemHandle,
};

use crate::setting::{Setting, SettingWriter};

pub const OPEN_CUSTOM_MENU_ITEM_ID: &str = "open";
pub const IS_RUNNING_CUSTOM_MENU_ITEM_ID: &str = "is_running";
pub const QUIT_CUSTOM_MENU_ITEM_ID: &str = "quit";

pub trait SystemTrayUpdater: Debug + Send + Sync + 'static {
    fn update(&self, setting: &Setting) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct SystemTrayUpdaterImpl {
    system_tray_menu_item_handle: SystemTrayMenuItemHandle,
}

impl SystemTrayUpdaterImpl {
    pub fn new(system_tray_menu_item_handle: SystemTrayMenuItemHandle) -> Self {
        Self {
            system_tray_menu_item_handle,
        }
    }
}

impl SystemTrayUpdater for SystemTrayUpdaterImpl {
    fn update(&self, setting: &Setting) -> anyhow::Result<()> {
        self.system_tray_menu_item_handle
            .set_selected(setting.is_running)?;
        Ok(())
    }
}

pub fn system_tray() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new(OPEN_CUSTOM_MENU_ITEM_ID, "Open"))
            .add_item(CustomMenuItem::new(
                IS_RUNNING_CUSTOM_MENU_ITEM_ID,
                "Active",
            ))
            .add_native_item(Separator)
            .add_item(CustomMenuItem::new(QUIT_CUSTOM_MENU_ITEM_ID, "Quit")),
    )
}

pub fn system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    let main_window = app_handle
        .get_window(&app_handle.config().tauri.windows[0].label)
        .unwrap();
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
    let setting = app_handle.state::<Arc<RwLock<Setting>>>();
    match event {
        LeftClick { .. } | DoubleClick { .. } => open_window(),
        MenuItemClick { id, .. } => match id.as_str() {
            OPEN_CUSTOM_MENU_ITEM_ID => open_window(),
            IS_RUNNING_CUSTOM_MENU_ITEM_ID => {
                let mut s = setting.write().expect("Failed to write setting");
                s.is_running = !s.is_running;

                let setting_writer = app_handle.state::<Arc<dyn SettingWriter>>();
                setting_writer
                    .write_to_file(&s)
                    .expect("Failed to write setting");

                let system_tray_updater = app_handle.state::<Arc<dyn SystemTrayUpdater>>();
                system_tray_updater
                    .update(&s)
                    .expect("Failed to update system tray");

                app_handle
                    .emit_all("reload_setting", ())
                    .expect("Failed to emit event");
            }
            QUIT_CUSTOM_MENU_ITEM_ID => {
                close_window();
            }
            _ => (),
        },
        _ => (),
    }
}
