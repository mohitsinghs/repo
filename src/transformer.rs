use crate::path_extra::*;
use anyhow::{Error, Result};
use std::{collections::BTreeMap, path::PathBuf};

pub fn as_path_names(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter()
        .map(|d| d.ex_base().unwrap_or_default())
        .collect()
}

pub fn as_paths(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter().map(|d| d.ex_string()).collect()
}

pub fn as_json(dirs: Vec<PathBuf>) -> Result<String> {
    let map: BTreeMap<String, String> = dirs
        .into_iter()
        .map(|d| (d.ex_base().unwrap_or_default(), d.ex_string()))
        .collect();
    serde_json::ser::to_string_pretty(&map).map_err(Error::msg)
}
