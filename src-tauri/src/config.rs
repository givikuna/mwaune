use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: Theme,
    pub custom_genres: Vec<String>,
    pub calm_mode_default: bool,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Gruvbox,
    Nordic,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path: PathBuf = Self::get_config_path()?;
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        std::fs::write(
            Self::get_config_path()?,
            serde_json::to_string_pretty(self)?,
        )?;
        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
        Ok(home.join(".mwaune/config.json"))
    }
}

fn default_data_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".mwaune")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::Gruvbox,
            custom_genres: Vec::new(),
            calm_mode_default: false,
            data_dir: default_data_dir(),
        }
    }
}
