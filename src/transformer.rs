use crate::path_extra::*;
use anyhow::{Error, Result};
use serde_json::{json, to_string_pretty, Map};
use std::{collections::BTreeMap, path::PathBuf};

pub fn as_path_names(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter()
        .map(|d| d.x_base().unwrap_or_default())
        .collect()
}

pub fn as_paths(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter().map(|d| d.x_string()).collect()
}

pub fn as_json(dirs: Vec<PathBuf>) -> Result<String> {
    let map: BTreeMap<String, String> = dirs
        .into_iter()
        .map(|d| (d.x_base().unwrap_or_default(), d.x_string()))
        .collect();
    serde_json::ser::to_string_pretty(&map).map_err(Error::msg)
}

fn common_path(dirs: &Vec<PathBuf>) -> PathBuf {
    let initial = dirs.get(0).unwrap();
    let (min, max) = dirs.iter().fold((initial, initial), |acc, val| {
        (acc.0.min(val), acc.1.max(val))
    });
    min.components()
        .into_iter()
        .zip(max.components().into_iter())
        .map_while(|(n, x)| if x == n { Some(x.as_os_str()) } else { None })
        .collect::<PathBuf>()
}

pub fn as_tree(dirs: Vec<PathBuf>) -> Result<String> {
    let common = common_path(&dirs);
    let val = dirs
        .iter()
        .map(|dir| dir.strip_prefix(&common).unwrap())
        .fold(json!(Map::new()), |mut acc, val| {
            let len = val.components().count();
            let mut comp_iter = val.components().into_iter().enumerate();
            let original = common.join(val.clone()).x_string();
            let mut _ref = &mut acc;
            while let Some((count, p)) = comp_iter.next() {
                let key = p.x_str();
                let remaining = len - (count + 1);
                if remaining >= 2 {
                    _ref = _ref
                        .as_object_mut()
                        .unwrap()
                        .entry(key)
                        .or_insert(json!(Map::new()));
                } else if !_ref.is_null() {
                    let (_, base) = comp_iter.next().unwrap();
                    let value = json!({
                        "location": original,
                        "label": base.x_str(),
                    });
                    if let Some(_obj) = _ref.as_object_mut() {
                        if let Some(_key) = _obj.get_mut(key) {
                            if let Some(_key_obj) = _key.as_object_mut() {
                                if let Some(_core) = _key_obj.get_mut("_core") {
                                    _core.as_array_mut().and_then(|arr| Some(arr.push(value)));
                                } else {
                                    _key_obj.insert("_core".to_string(), json!([value].to_vec()));
                                }
                            } else if let Some(arr) = _key.as_array_mut() {
                                arr.push(value);
                            }
                        } else {
                            _ref.as_object_mut()
                                .unwrap()
                                .insert(key.to_string(), json!([value].to_vec()));
                        }
                    } else if let Some(arr) = _ref.as_array_mut() {
                        arr.push(value);
                    }
                }
            }
            acc
        });
    Ok(to_string_pretty(&val)?)
}
