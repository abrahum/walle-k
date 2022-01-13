use std::fs;

use rs_khl::prelude::Config as KHLConfig;
use serde::{Deserialize, Serialize};
use walle_core::config::ImplConfig;

const CONFIG_PATH: &str = "Walle-K.toml";

#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct Config {
    pub(crate) khl: KHLConfig,
    pub(crate) ob: ImplConfig,
}

impl Config {
    pub(crate) fn load_from_file() -> Self {
        if let Ok(f) = fs::read_to_string(CONFIG_PATH) {
            toml::from_str(&f).unwrap_or_default()
        } else {
            let config = Self::default();
            config.save_to_file();
            config
        }
    }

    pub(crate) fn save_to_file(&self) {
        let s = toml::to_string(&self).unwrap();
        fs::write(CONFIG_PATH, s).unwrap();
    }
}
