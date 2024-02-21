use std::sync::Arc;

use tauri::{InvokeError, State};

use crate::{
    config::{Config, ConfigManager},
    keyboard::Key,
};

#[tauri::command]
pub fn get_config(config_manager: State<Arc<dyn ConfigManager>>) -> Result<Config, InvokeError> {
    let config = config_manager.config().map_err(InvokeError::from_anyhow)?;
    Ok(config)
}

#[tauri::command]
pub fn set_is_running(
    config_manager: State<Arc<dyn ConfigManager>>,
    is_running: bool,
) -> Result<(), InvokeError> {
    config_manager
        .set_is_running(is_running)
        .map_err(InvokeError::from_anyhow)?;

    Ok(())
}

#[tauri::command]
pub fn set_activate_key(
    config_manager: State<Arc<dyn ConfigManager>>,
    key: Key,
) -> Result<(), InvokeError> {
    config_manager
        .set_activate_key(key)
        .map_err(InvokeError::from_anyhow)?;
    Ok(())
}

#[tauri::command]
pub fn set_deactivate_key(
    config_manager: State<Arc<dyn ConfigManager>>,
    key: Key,
) -> Result<(), InvokeError> {
    config_manager
        .set_deactivate_key(key)
        .map_err(InvokeError::from_anyhow)?;
    Ok(())
}
