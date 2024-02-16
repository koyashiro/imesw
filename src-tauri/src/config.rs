use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::keyboard::Key;

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
    fn get_config(&self) -> Config;

    fn set_is_running(&mut self, is_running: bool);

    fn set_activate_key(&mut self, key: Key);

    fn set_deactivate_key(&mut self, key: Key);
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConfigManagerImpl {
    is_running: bool,
    activate_key: Key,
    deactivate_key: Key,
}

impl ConfigManagerImpl {
    pub fn new(config: Config) -> Self {
        Self {
            is_running: config.is_running,
            activate_key: config.activate_key,
            deactivate_key: config.deactivate_key,
        }
    }
}

impl Default for ConfigManagerImpl {
    fn default() -> Self {
        let config = Config::default();
        Self::new(config)
    }
}

impl ConfigManager for ConfigManagerImpl {
    fn get_config(&self) -> Config {
        Config {
            is_running: self.is_running,
            activate_key: self.activate_key.clone(),
            deactivate_key: self.deactivate_key.clone(),
        }
    }

    fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    fn set_activate_key(&mut self, key: Key) {
        if key == self.deactivate_key {
            self.deactivate_key = self.activate_key.clone();
        }

        self.activate_key = key;
    }

    fn set_deactivate_key(&mut self, key: Key) {
        if key == self.activate_key {
            self.activate_key = self.deactivate_key.clone();
        }

        self.deactivate_key = key;
    }
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
        let config = Config {
            is_running: true,
            activate_key: Key::RightShift,
            deactivate_key: Key::LeftCtrl,
        };

        assert_eq!(
            ConfigManagerImpl::new(config.clone()),
            ConfigManagerImpl {
                is_running: config.is_running,
                activate_key: config.activate_key,
                deactivate_key: config.deactivate_key
            }
        );
    }

    #[test]
    fn test_config_manager_default() {
        let config = Config::default();

        assert_eq!(
            ConfigManagerImpl::default(),
            ConfigManagerImpl {
                is_running: config.is_running,
                activate_key: config.activate_key,
                deactivate_key: config.deactivate_key
            }
        );
    }

    #[test]
    fn test_config_manager_get_config() {
        let config_manager = ConfigManagerImpl::default();

        assert_eq!(config_manager.get_config(), Config::default());
    }

    #[test]
    fn test_config_manager_set_is_running() {
        let mut config_manager = ConfigManagerImpl::default();

        config_manager.set_is_running(true);
        assert!(config_manager.get_config().is_running);

        config_manager.set_is_running(false);
        assert!(!config_manager.get_config().is_running);
    }

    #[test]
    fn test_config_manager_set_activate_key() {
        let mut config_manager = ConfigManagerImpl::default();

        config_manager.set_activate_key(Key::RightShift);
        assert_eq!(config_manager.get_config().activate_key, Key::RightShift);

        config_manager.set_activate_key(Key::LeftCtrl);
        assert_eq!(config_manager.get_config().activate_key, Key::LeftCtrl);
    }

    #[test]
    fn test_config_manager_set_deactivate_key() {
        let mut config_manager = ConfigManagerImpl::default();

        config_manager.set_deactivate_key(Key::RightCtrl);
        assert_eq!(config_manager.get_config().deactivate_key, Key::RightCtrl);

        config_manager.set_deactivate_key(Key::LeftShift);
        assert_eq!(config_manager.get_config().deactivate_key, Key::LeftShift);
    }
}
