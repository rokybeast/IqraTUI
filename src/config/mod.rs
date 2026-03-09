pub mod keybindings;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::models::ViewMode;
use crate::icons::IconStyle;
use crate::theme::ThemeType;
use keybindings::Keybindings;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_theme")]
    pub theme: ThemeType,
    #[serde(default = "default_mode")]
    pub default_mode: String,
    #[serde(default = "default_true")]
    pub show_arabic: bool,
    #[serde(default = "default_true")]
    pub show_romanized: bool,
    #[serde(default = "default_true")]
    pub show_english: bool,
    #[serde(default)]
    pub keybindings: Keybindings,
    #[serde(default)]
    pub tts: TtsConfig,
    #[serde(default)]
    pub salah: SalahConfig,
    #[serde(default)]
    pub icon_style: IconStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TtsConfig {
    #[serde(default = "default_reciter")]
    pub reciter_id: u16,
    #[serde(default)]
    pub auto_next: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalahConfig {
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
    #[serde(default = "default_method")]
    pub method: u8,
    #[serde(default = "default_true")]
    pub show_in_status: bool,
}

fn default_theme() -> ThemeType {
    ThemeType::Dark
}

fn default_mode() -> String {
    "paginated".to_string()
}

fn default_true() -> bool {
    true
}

fn default_reciter() -> u16 {
    7
}

fn default_method() -> u8 {
    2
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeType::Dark,
            default_mode: "paginated".to_string(),
            show_arabic: true,
            show_romanized: true,
            show_english: true,
            keybindings: Keybindings::default(),
            tts: TtsConfig::default(),
            salah: SalahConfig::default(),
            icon_style: IconStyle::default(),
        }
    }
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            reciter_id: 7,
            auto_next: false,
        }
    }
}

impl Default for SalahConfig {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            method: 2,
            show_in_status: true,
        }
    }
}

impl AppConfig {
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("iqra")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: AppConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(AppConfig::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn default_view_mode(&self) -> ViewMode {
        match self.default_mode.as_str() {
            "scroll" => ViewMode::Scroll,
            _ => ViewMode::Paginated,
        }
    }
}
