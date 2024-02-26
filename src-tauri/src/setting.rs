use std::{fs::File, path::PathBuf};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::keyboard::Key;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
static SETTING_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs_next::config_dir()
        .expect("Failed to get config dir")
        .join(PKG_NAME)
});
static SETTING_FILE: Lazy<PathBuf> = Lazy::new(|| SETTING_DIR.join(format!("{PKG_NAME}.json")));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setting {
    #[serde(rename = "isRunning")]
    pub is_running: bool,

    #[serde(rename = "activateKey")]
    pub activate_key: Key,

    #[serde(rename = "deactivateKey")]
    pub deactivate_key: Key,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            is_running: false,
            activate_key: Key::RightAlt,
            deactivate_key: Key::LeftAlt,
        }
    }
}

pub trait SettingReader: Send + Sync + 'static {
    fn read_from_file(&self) -> anyhow::Result<Setting>;
}

#[derive(Debug, Clone)]
pub struct SettingReaderImpl;

impl SettingReader for SettingReaderImpl {
    fn read_from_file(&self) -> anyhow::Result<Setting> {
        let file = File::open(SETTING_FILE.as_path())?;
        let setting = serde_json::from_reader(file)?;
        Ok(setting)
    }
}

pub trait SettingWriter: Send + Sync + 'static {
    fn write_to_file(&self, setting: &Setting) -> anyhow::Result<()>;
}

impl SettingWriter for SettingReaderImpl {
    fn write_to_file(&self, setting: &Setting) -> anyhow::Result<()> {
        let file = File::create(SETTING_FILE.as_path())?;
        serde_json::to_writer_pretty(file, setting)?;
        Ok(())
    }
}
