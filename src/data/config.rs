use std::{fs::File, path::PathBuf};

use druid::{Data, Lens};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

use crate::core::cache::mkdir_if_not_exists;

use super::Nav;

const APP_NAME: &str = "Mangetsu";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub theme: Theme,
    pub last_route: Option<Nav>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            last_route: Default::default(),
        }
    }
}

impl Config {
    fn app_dirs() -> Option<AppDirs> {
        const USE_XDG_ON_MACOS: bool = false;

        AppDirs::new(Some(APP_NAME), USE_XDG_ON_MACOS)
    }

    pub fn cache_dir() -> Option<PathBuf> {
        Self::app_dirs().map(|dirs| dirs.cache_dir)
    }

    pub fn config_dir() -> Option<PathBuf> {
        Self::app_dirs().map(|dirs| dirs.config_dir)
    }

    pub fn download_dir() -> Option<PathBuf> {
        platform_dirs::UserDirs::new().map(|dir| dir.document_dir.join(APP_NAME))
    }

    fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join(CONFIG_FILENAME))
    }

    pub fn load() -> Option<Config> {
        let path = Self::config_path().expect("Failed to get config path");
        if let Ok(file) = File::open(&path) {
            log::info!("loading config: {:?}", &path);
            Some(serde_json::from_reader(file).expect("Failed to read config"))
        } else {
            None
        }
    }

    pub fn save(&self) {
        let dir = Self::config_dir().expect("Failed to get config dir");
        let path = Self::config_path().expect("Failed to get config path");
        mkdir_if_not_exists(&dir).expect("Failed to create config dir");
        let file = File::create(&path).expect("Failed to create config");
        serde_json::to_writer_pretty(file, self).expect("Failed to write config");
        log::info!("saved config: {:?}", &path);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Data, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        })
    }
}
