use std::sync::Arc;

use tauri::{InvokeError, State};

use crate::{
    config::{Config, ConfigManager},
    keyboard::Key,
    tray::SystemTrayUpdater,
};

#[tauri::command]
pub fn get_config(config_manager: State<Arc<dyn ConfigManager>>) -> Result<Config, InvokeError> {
    let config = config_manager.config().map_err(InvokeError::from_anyhow)?;
    Ok(config)
}

#[tauri::command]
pub fn set_is_running(
    config_manager: State<Arc<dyn ConfigManager>>,
    system_tray_updater: State<Arc<dyn SystemTrayUpdater>>,
    is_running: bool,
) -> Result<(), InvokeError> {
    config_manager
        .set_is_running(is_running)
        .map_err(InvokeError::from_anyhow)?;
    let config = config_manager.config().map_err(InvokeError::from_anyhow)?;
    system_tray_updater
        .update(&config)
        .map_err(InvokeError::from_anyhow)?;
    Ok(())
}

#[tauri::command]
pub fn set_activate_key(
    config_manager: State<Arc<dyn ConfigManager>>,
    system_tray_updater: State<Arc<dyn SystemTrayUpdater>>,
    key: Key,
) -> Result<(), InvokeError> {
    config_manager
        .set_activate_key(key)
        .map_err(InvokeError::from_anyhow)?;
    let config = config_manager.config().map_err(InvokeError::from_anyhow)?;
    system_tray_updater
        .update(&config)
        .map_err(InvokeError::from_anyhow)?;
    Ok(())
}

#[tauri::command]
pub fn set_deactivate_key(
    config_manager: State<Arc<dyn ConfigManager>>,
    system_tray_updater: State<Arc<dyn SystemTrayUpdater>>,
    key: Key,
) -> Result<(), InvokeError> {
    config_manager
        .set_deactivate_key(key)
        .map_err(InvokeError::from_anyhow)?;
    let config = config_manager.config().map_err(InvokeError::from_anyhow)?;
    system_tray_updater
        .update(&config)
        .map_err(InvokeError::from_anyhow)?;
    Ok(())
}
