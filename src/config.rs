use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub clipboard_timeout: u64,
    pub auto_lock_minutes: u64,
    pub theme: String,
    pub window_center: bool,
    pub vault_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clipboard_timeout: 20,
            auto_lock_minutes: 5,
            theme: "dark".to_string(),
            window_center: true,
            vault_path: Self::default_vault_path(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        let config_dir = config_path.parent().unwrap();
        
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("keytui")
            .join("config.toml")
    }

    fn default_vault_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("keytui")
            .join("vault.db")
    }
}
