use std::sync::{Arc, RwLock};

use tauri::{InvokeError, State};

use crate::{
    config::{Config, ConfigManager},
    keyboard::Key,
};

#[tauri::command]
pub fn get_config(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
) -> Result<Config, InvokeError> {
    let config_manager = config_manager.read().map_err(|e| {
        InvokeError::from_anyhow(anyhow::anyhow!("failed to read config_manager: {e}"))
    })?;

    let config = config_manager.get_config();

    Ok(config)
}

#[tauri::command]
pub fn set_is_running(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
    is_running: bool,
) -> Result<(), InvokeError> {
    let mut config_manager = config_manager.write().map_err(|e| {
        InvokeError::from_anyhow(anyhow::anyhow!("failed to write config_manager: {e}"))
    })?;

    config_manager.set_is_running(is_running);

    Ok(())
}

#[tauri::command]
pub fn set_activate_key(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
    key: Key,
) -> Result<(), InvokeError> {
    let mut config_manager = config_manager.write().map_err(|e| {
        InvokeError::from_anyhow(anyhow::anyhow!("failed to write config_manager: {e}"))
    })?;

    config_manager.set_activate_key(key);

    Ok(())
}

#[tauri::command]
pub fn set_deactivate_key(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
    key: Key,
) -> Result<(), InvokeError> {
    let mut config_manager = config_manager.write().map_err(|e| {
        InvokeError::from_anyhow(anyhow::anyhow!("failed to write config_manager: {e}"))
    })?;

    config_manager.set_deactivate_key(key);

    Ok(())
}
