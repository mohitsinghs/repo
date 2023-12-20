use crate::path_extra::*;
use anyhow::{Error, Result};
use serde_json::{json, to_string, Map, Value};
use std::{collections::BTreeMap, mem::replace, path::PathBuf};

pub fn as_path_names(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter()
        .map(|d| d.base().unwrap_or_default())
        .collect()
}

pub fn as_paths(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter().map(|d| d.to_string()).collect()
}

pub fn as_json(dirs: Vec<PathBuf>) -> Result<String> {
    let map: BTreeMap<String, String> = dirs
        .into_iter()
        .map(|d| (d.base().unwrap_or_default(), d.to_string()))
        .collect();
    to_string(&map).map_err(Error::msg)
}

fn common_path(dirs: &[PathBuf]) -> PathBuf {
    let initial = dirs.get(0).unwrap();
    let (min, max) = dirs.iter().fold((initial, initial), |acc, val| {
        (acc.0.min(val), acc.1.max(val))
    });
    min.components()
        .zip(max.components())
        .map_while(|(n, x)| if x == n { Some(x.as_os_str()) } else { None })
        .collect::<PathBuf>()
}

fn add_child(target: &mut Value, val: Value) {
    if let Some(target_obj) = target.as_object_mut() {
        if let Some(_children) = target_obj.get_mut("_children") {
            if let Some(arr) = _children.as_array_mut() {
                arr.push(val)
            }
        } else {
            target_obj.insert("_children".to_string(), json!([val].to_vec()));
        }
    } else if let Some(arr) = target.as_array_mut() {
        arr.push(val);
    }
}

pub fn as_tree(dirs: Vec<PathBuf>) -> Result<String> {
    let common = if dirs.len() == 1 {
        dirs[0].parent().unwrap().to_path_buf()
    } else {
        common_path(&dirs)
    };
    let val = dirs
        .iter()
        .map(|dir| dir.strip_prefix(&common).unwrap())
        .fold(json!(Map::new()), |mut acc, val| {
            let len = val.components().count();
            let mut comp_iter = val.components().enumerate();
            let original = common.join(val).to_string();
            let mut _ref = &mut acc;
            while let Some((count, p)) = comp_iter.next() {
                let remaining = len - (count + 1);
                let current = p.to_str();
                if remaining == 0 {
                    let value = json!({
                        "location": original,
                        "label": current,
                    });
                    add_child(_ref, value)
                } else if remaining > 1 {
                    if _ref.is_array() {
                        let children = replace(_ref, json!(Map::new()));
                        let mut _ref_obj = _ref.as_object_mut().unwrap();
                        _ref_obj.insert("_children".to_string(), json!(children));
                    }
                    _ref = _ref
                        .as_object_mut()
                        .unwrap()
                        .entry(current)
                        .or_insert(json!(Map::new()));
                } else if !_ref.is_null() {
                    let (_, base) = comp_iter.next().unwrap();
                    let value = json!({
                        "location": original,
                        "label": base.to_str(),
                    });
                    if let Some(_obj) = _ref.as_object_mut() {
                        if let Some(_key) = _obj.get_mut(current) {
                            add_child(_key, value);
                        } else {
                            _obj.insert(current.to_string(), json!([value].to_vec()));
                        }
                    } else if _ref.is_array() {
                        let children = replace(_ref, json!(Map::new()));
                        let mut _ref_obj = _ref.as_object_mut().unwrap();
                        _ref_obj.insert("_children".to_string(), json!(children));
                        _ref_obj.insert(current.to_string(), json!([value].to_vec()));
                    }
                }
            }
            acc
        });
    Ok(to_string(&val)?)
}
