use anyhow::{Context, Error, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, write};

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Root {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub depth: usize,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Conf {
    pub roots: Vec<Root>,
}

impl Conf {
    pub fn read() -> Result<Self> {
        let conf_file = config_dir().unwrap_or_default().join("repo.yml");
        if conf_file.exists() && conf_file.is_file() {
            let content = read_to_string(conf_file)?;
            let data: Conf = serde_yaml::from_str(&content)?;
            Ok(data)
        } else {
            Ok(Conf::default())
        }
    }

    pub fn write() -> Result<()> {
        let conf_loc = config_dir().unwrap_or_default();
        if !conf_loc.exists() {
            create_dir_all(conf_loc.as_path()).context("failed to write config")?;
        }
        let conf_file = conf_loc.join("repo.yml");
        if !conf_file.exists() {
            let data = serde_yaml::to_string(&Conf::default())?;
            write(conf_file, &data).map_err(Error::from)
        } else {
            Ok(())
        }
    }
}
