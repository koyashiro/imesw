use std::{fmt::Debug, sync::Arc};

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray,
    SystemTrayEvent::{self, DoubleClick, LeftClick, MenuItemClick},
    SystemTrayMenu, SystemTrayMenuItemHandle,
};

use crate::config::{Config, ConfigManager};

pub const OPEN_CUSTOM_MENU_ITEM_ID: &str = "open";
pub const IS_RUNNING_CUSTOM_MENU_ITEM_ID: &str = "is_running";
pub const QUIT_CUSTOM_MENU_ITEM_ID: &str = "quit";

pub trait SystemTrayUpdater: Debug + Send + Sync + 'static {
    fn update(&self, config: &Config) -> anyhow::Result<()>;
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
    fn update(&self, config: &Config) -> anyhow::Result<()> {
        self.system_tray_menu_item_handle
            .set_selected(config.is_running)?;
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
            .add_item(CustomMenuItem::new(QUIT_CUSTOM_MENU_ITEM_ID, "Quit")),
    )
}

pub fn system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
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
}
