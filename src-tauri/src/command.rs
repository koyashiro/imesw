use std::{
    error::Error,
    sync::{Arc, RwLock},
};

use tauri::{InvokeError, State};

use crate::{
    config::{Config, ConfigManager},
    keyboard::Key,
};

#[tauri::command]
pub fn get_config(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
) -> Result<Config, InvokeError> {
    let config_manager = config_manager.read().map_err(into_read_invoke_error)?;

    let config = config_manager.get_config();

    Ok(config)
}

#[tauri::command]
pub fn set_is_running(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
    is_running: bool,
) -> Result<(), InvokeError> {
    let mut config_manager = config_manager.write().map_err(into_write_invoke_error)?;

    config_manager.set_is_running(is_running);

    Ok(())
}

#[tauri::command]
pub fn set_activate_key(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
    key: Key,
) -> Result<(), InvokeError> {
    let mut config_manager = config_manager.write().map_err(into_write_invoke_error)?;

    config_manager.set_activate_key(key);

    Ok(())
}

#[tauri::command]
pub fn set_deactivate_key(
    config_manager: State<'_, Arc<RwLock<dyn ConfigManager>>>,
    key: Key,
) -> Result<(), InvokeError> {
    let mut config_manager = config_manager.write().map_err(into_write_invoke_error)?;

    config_manager.set_deactivate_key(key);

    Ok(())
}

fn into_read_invoke_error(e: impl Error) -> InvokeError {
    InvokeError::from_anyhow(anyhow::anyhow!("failed to read config_manager: {e}"))
}

fn into_write_invoke_error(e: impl Error) -> InvokeError {
    InvokeError::from_anyhow(anyhow::anyhow!("failed to write config_manager: {e}"))
}
