use std::{
    fmt::Debug,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
    sync::RwLock,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{keyboard::Key, tray::SystemTrayUpdater};

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

pub trait ConfigManager: Debug + Send + Sync + 'static {
    fn load(&self) -> anyhow::Result<()>;

    fn save(&self) -> anyhow::Result<()>;

    fn config(&self) -> anyhow::Result<Config>;

    fn set_config(&self, config: Config) -> anyhow::Result<()>;

    fn set_is_running(&self, is_running: bool) -> anyhow::Result<()>;

    fn toggle_is_running(&self) -> anyhow::Result<()>;

    fn set_activate_key(&self, key: Key) -> anyhow::Result<()>;

    fn set_deactivate_key(&self, key: Key) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct ConfigManagerImpl<T: SystemTrayUpdater> {
    config: RwLock<Config>,
    system_tray_updater: T,
}

impl<T: SystemTrayUpdater> ConfigManagerImpl<T> {
    pub fn new(config: Config, system_tray_updater: T) -> Self {
        system_tray_updater.update(&config).unwrap();
        Self {
            config: RwLock::new(config),
            system_tray_updater,
        }
    }

    pub fn update_system_tray(&self) -> anyhow::Result<()> {
        let config = self.config()?;
        self.system_tray_updater.update(&config)?;
        Ok(())
    }
}

impl<T: SystemTrayUpdater> ConfigManager for ConfigManagerImpl<T> {
    fn load(&self) -> anyhow::Result<()> {
        let config = read_config()?;
        self.set_config(config)?;
        self.update_system_tray()?;
        Ok(())
    }

    fn save(&self) -> anyhow::Result<()> {
        let config = self.config()?;
        write_config(&config)?;
        Ok(())
    }

    fn config(&self) -> anyhow::Result<Config> {
        let config = self
            .config
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone();
        Ok(config)
    }

    fn set_config(&self, config: Config) -> anyhow::Result<()> {
        {
            *self
                .config
                .write()
                .map_err(|e| anyhow::anyhow!(e.to_string()))? = config;
        }
        self.update_system_tray()?;
        self.save()?;
        Ok(())
    }

    fn set_is_running(&self, is_running: bool) -> anyhow::Result<()> {
        {
            let mut config = self
                .config
                .write()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            config.is_running = is_running;
        }
        self.update_system_tray()?;
        self.save()?;
        Ok(())
    }

    fn toggle_is_running(&self) -> anyhow::Result<()> {
        {
            let mut config = self
                .config
                .write()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            config.is_running = !config.is_running;
        }
        self.update_system_tray()?;
        self.save()?;
        Ok(())
    }

    fn set_activate_key(&self, key: Key) -> anyhow::Result<()> {
        {
            let mut config = self
                .config
                .write()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            if key == config.deactivate_key {
                config.deactivate_key = config.activate_key.clone();
            }
            config.activate_key = key;
        }
        self.update_system_tray()?;
        self.save()?;
        Ok(())
    }

    fn set_deactivate_key(&self, key: Key) -> anyhow::Result<()> {
        {
            let mut config = self
                .config
                .write()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            if key == config.activate_key {
                config.activate_key = config.deactivate_key.clone();
            }
            config.deactivate_key = key;
        }
        self.update_system_tray()?;
        self.save()?;
        Ok(())
    }
}

pub fn read_config() -> anyhow::Result<Config> {
    let file = File::open(SETTINGS_FILE.as_path())?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

pub fn read_or_default() -> Config {
    let file = match File::open(SETTINGS_FILE.as_path()) {
        Ok(file) => file,
        Err(_) => return Config::default(),
    };
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_default()
}

pub fn write_config(config: &Config) -> anyhow::Result<()> {
    fs::create_dir_all(CONFIG_DIR.as_path())?;
    let file = File::create(SETTINGS_FILE.as_path())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // assert_eq!(ConfigManagerImpl::new(), ConfigManagerImpl::default(),);
    }

    #[test]
    fn test_config_manager_get_config() {
        // let config = Config::default();

        // let config_manager = ConfigManagerImpl {
        //     config: config.clone(),
        // };

        // assert_eq!(config_manager.get_config(), &config);
    }

    #[test]
    fn test_config_manager_set_is_running() {
        // let mut config_manager = ConfigManagerImpl::default();

        // config_manager.set_is_running(true).unwrap();
        // assert!(config_manager.get_config().is_running);

        // config_manager.set_is_running(false).unwrap();
        // assert!(!config_manager.get_config().is_running);
    }

    #[test]
    fn test_config_manager_set_activate_key() {
        // let mut config_manager = ConfigManagerImpl::default();

        // config_manager.set_activate_key(Key::RightShift).unwrap();
        // assert_eq!(config_manager.get_config().activate_key, Key::RightShift);

        // config_manager.set_activate_key(Key::LeftCtrl).unwrap();
        // assert_eq!(config_manager.get_config().activate_key, Key::LeftCtrl);
    }

    #[test]
    fn test_config_manager_set_deactivate_key() {
        // let mut config_manager = ConfigManagerImpl::default();

        // config_manager.set_deactivate_key(Key::RightCtrl).unwrap();
        // assert_eq!(config_manager.get_config().deactivate_key, Key::RightCtrl);

        // config_manager.set_deactivate_key(Key::LeftShift).unwrap();
        // assert_eq!(config_manager.get_config().deactivate_key, Key::LeftShift);
    }
}
