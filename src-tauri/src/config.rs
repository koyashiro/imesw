use std::{
    fmt::Debug,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::keyboard::Key;

static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs_next::config_dir()
        .expect("Failed to get config dir")
        .join(env!("CARGO_PKG_NAME"))
});
static SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("settings.json"));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "isRunning")]
    pub is_running: bool,

    #[serde(rename = "activateKey")]
    pub activate_key: Key,

    #[serde(rename = "deactivateKey")]
    pub deactivate_key: Key,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_running: true,
            activate_key: Key::RightAlt,
            deactivate_key: Key::LeftAlt,
        }
    }
}

pub trait ConfigManager: Debug + Send + Sync + AsRef<Config> + 'static {
    fn get_config(&self) -> &Config {
        self.as_ref()
    }

    fn set_config(&mut self, config: Config) -> anyhow::Result<()>;

    fn set_is_running(&mut self, is_running: bool) -> anyhow::Result<()>;

    fn set_activate_key(&mut self, key: Key) -> anyhow::Result<()>;

    fn set_deactivate_key(&mut self, key: Key) -> anyhow::Result<()>;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ConfigManagerImpl {
    config: Config,
}

impl ConfigManagerImpl {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn load(&mut self) -> anyhow::Result<()> {
        let config = read_config()?;
        self.set_config(config)?;
        Ok(())
    }

    pub fn load_or_init(&mut self) -> anyhow::Result<()> {
        match read_config() {
            Ok(config) => self.set_config(config),
            Err(_) => self.save(),
        }?;
        Ok(())
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let config = self.get_config();
        write_config(config)?;
        Ok(())
    }
}

impl AsRef<Config> for ConfigManagerImpl {
    fn as_ref(&self) -> &Config {
        &self.config
    }
}

impl From<Config> for ConfigManagerImpl {
    fn from(config: Config) -> Self {
        Self { config }
    }
}

impl ConfigManager for ConfigManagerImpl {
    fn set_config(&mut self, config: Config) -> anyhow::Result<()> {
        self.config = config;
        self.save()?;
        Ok(())
    }

    fn set_is_running(&mut self, is_running: bool) -> anyhow::Result<()> {
        self.config.is_running = is_running;
        self.save()?;
        Ok(())
    }

    fn set_activate_key(&mut self, key: Key) -> anyhow::Result<()> {
        if key == self.config.deactivate_key {
            self.config.deactivate_key = self.config.activate_key.clone();
        }

        self.config.activate_key = key;
        self.save()?;
        Ok(())
    }

    fn set_deactivate_key(&mut self, key: Key) -> anyhow::Result<()> {
        if key == self.config.activate_key {
            self.config.activate_key = self.config.deactivate_key.clone();
        }

        self.config.deactivate_key = key;
        self.save()?;
        Ok(())
    }
}

fn read_config() -> anyhow::Result<Config> {
    let file = File::open(SETTINGS_FILE.as_path())?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn write_config(config: &Config) -> anyhow::Result<()> {
    fs::create_dir_all(CONFIG_DIR.as_path())?;
    let file = File::create(SETTINGS_FILE.as_path())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::{Config, ConfigManager, ConfigManagerImpl};

    #[test]
    fn test_config_new() {
        assert_eq!(
            Config::default(),
            Config {
                is_running: true,
                activate_key: Key::RightAlt,
                deactivate_key: Key::LeftAlt,
            }
        );
    }

    #[test]
    fn test_config_manager_new() {
        assert_eq!(ConfigManagerImpl::new(), ConfigManagerImpl::default(),);
    }

    #[test]
    fn test_config_manager_get_config() {
        let config = Config::default();

        let config_manager = ConfigManagerImpl {
            config: config.clone(),
        };

        assert_eq!(config_manager.get_config(), &config);
    }

    #[test]
    fn test_config_manager_set_is_running() {
        let mut config_manager = ConfigManagerImpl::default();

        config_manager.set_is_running(true).unwrap();
        assert!(config_manager.get_config().is_running);

        config_manager.set_is_running(false).unwrap();
        assert!(!config_manager.get_config().is_running);
    }

    #[test]
    fn test_config_manager_set_activate_key() {
        let mut config_manager = ConfigManagerImpl::default();

        config_manager.set_activate_key(Key::RightShift).unwrap();
        assert_eq!(config_manager.get_config().activate_key, Key::RightShift);

        config_manager.set_activate_key(Key::LeftCtrl).unwrap();
        assert_eq!(config_manager.get_config().activate_key, Key::LeftCtrl);
    }

    #[test]
    fn test_config_manager_set_deactivate_key() {
        let mut config_manager = ConfigManagerImpl::default();

        config_manager.set_deactivate_key(Key::RightCtrl).unwrap();
        assert_eq!(config_manager.get_config().deactivate_key, Key::RightCtrl);

        config_manager.set_deactivate_key(Key::LeftShift).unwrap();
        assert_eq!(config_manager.get_config().deactivate_key, Key::LeftShift);
    }
}
