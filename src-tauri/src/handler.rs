use std::sync::{Arc, RwLock};

use tauri::{AppHandle, Invoke, InvokeError, Manager, State};

use crate::{
    keyboard::Key,
    setting::{Setting, SettingWriter},
    tray::SystemTrayUpdater,
};

#[tauri::command]
pub fn get_setting(setting: State<Arc<RwLock<Setting>>>) -> Result<Setting, InvokeError> {
    let setting = setting
        .read()
        .map_err(|e| InvokeError::from_anyhow(anyhow::anyhow!(e.to_string())))?
        .clone();

    Ok(setting)
}

#[tauri::command]
pub fn set_is_running(
    app_handle: AppHandle,
    setting: State<Arc<RwLock<Setting>>>,
    system_tray_updater: State<Arc<dyn SystemTrayUpdater>>,
    setting_writer: State<Arc<dyn SettingWriter>>,
    is_running: bool,
) -> Result<(), InvokeError> {
    let mut setting = setting
        .write()
        .map_err(|e| InvokeError::from_anyhow(anyhow::anyhow!(e.to_string())))?;
    setting.is_running = is_running;

    app_handle.emit_all("reload_setting", ())?;

    system_tray_updater
        .update(&setting)
        .map_err(InvokeError::from_anyhow)?;

    setting_writer
        .write_to_file(&setting)
        .map_err(InvokeError::from_anyhow)?;

    Ok(())
}

#[tauri::command]
pub fn set_activate_key(
    app_handle: AppHandle,
    setting: State<Arc<RwLock<Setting>>>,
    system_tray_updater: State<Arc<dyn SystemTrayUpdater>>,
    setting_writer: State<Arc<dyn SettingWriter>>,
    key: Key,
) -> Result<(), InvokeError> {
    let mut setting = setting
        .write()
        .map_err(|e| InvokeError::from_anyhow(anyhow::anyhow!(e.to_string())))?;
    setting.activate_key = key;

    app_handle.emit_all("reload_setting", ())?;

    system_tray_updater
        .update(&setting)
        .map_err(InvokeError::from_anyhow)?;

    setting_writer
        .write_to_file(&setting)
        .map_err(InvokeError::from_anyhow)?;

    Ok(())
}

#[tauri::command]
pub fn set_deactivate_key(
    app_handle: AppHandle,
    setting: State<Arc<RwLock<Setting>>>,
    system_tray_updater: State<Arc<dyn SystemTrayUpdater>>,
    setting_writer: State<Arc<dyn SettingWriter>>,
    key: Key,
) -> Result<(), InvokeError> {
    let mut setting = setting
        .write()
        .map_err(|e| InvokeError::from_anyhow(anyhow::anyhow!(e.to_string())))?;
    setting.deactivate_key = key;

    app_handle.emit_all("reload_setting", ())?;

    system_tray_updater
        .update(&setting)
        .map_err(InvokeError::from_anyhow)?;

    setting_writer
        .write_to_file(&setting)
        .map_err(InvokeError::from_anyhow)?;

    Ok(())
}

pub fn handler() -> impl Fn(Invoke) + Send + Sync + 'static {
    tauri::generate_handler![
        get_setting,
        set_is_running,
        set_activate_key,
        set_deactivate_key,
    ]
}
