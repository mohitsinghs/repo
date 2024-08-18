use crate::path_extra::*;
use anyhow::{Error, Result};
use serde_json::{json, to_string, Map, Value};
use std::{collections::BTreeMap, fs, mem::replace, path::PathBuf};

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

// a port of python's os.path.commonpath
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

/**
* add a child to target object. If target is an object, we insert current
* child under a key `_children` as an array and if target is an array
* we push current value to it directly.
* */
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

fn find_meta(loc: &PathBuf) -> (String, String) {
    let branch = fs::read_to_string(loc.join(".git/HEAD"))
        .ok()
        .and_then(|head_content| {
            head_content
                .strip_prefix("ref: refs/heads/")
                .map(|s| s.trim().to_string())
                .or_else(|| Some("detached".to_string()))
        })
        .unwrap_or_else(|| "unknown".to_string());

    let provider = fs::read_to_string(loc.join(".git/config"))
        .ok()
        .and_then(|config_content| {
            config_content
                .lines()
                .find(|line| line.contains("url ="))
                .map(|line| {
                    if line.contains("github.com") {
                        "github".to_string()
                    } else if line.contains("gitlab.com") {
                        "gitlab".to_string()
                    } else if line.contains("bitbucket.org") {
                        "bitbucket".to_string()
                    } else {
                        "unknown".to_string()
                    }
                })
        })
        .unwrap_or_else(|| "unknown".to_string());

    (branch, provider)
}

// build a tree out of detected paths
pub fn as_tree(dirs: Vec<PathBuf>) -> Result<String> {
    // if there's only one directory, use its parent
    // as the common path otherwise, find the common path
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
            let mut parts = val.components().enumerate();
            let original = common.join(val);
            let mut _ref = &mut acc;
            while let Some((count, part)) = parts.next() {
                let remaining = len - (count + 1);
                let current = part.to_str();
                if remaining == 0 {
                    let (branch, provider) = find_meta(&original);
                    let value = json!({
                        "location": original,
                        "label": current,
                        "branch": branch,
                        "provider": provider
                    });
                    add_child(_ref, value)
                } else if remaining > 1 {
                    // we've got a directory, but we've array of repos, so
                    // let convert that to object and move array under `_children`
                    if _ref.is_array() {
                        let children = replace(_ref, json!(Map::new()));
                        let mut _ref_obj = _ref.as_object_mut().unwrap();
                        _ref_obj.insert("_children".to_string(), json!(children));
                    }
                    // finally let's create empty object under our new object
                    // with directory name as key and descend to it
                    _ref = _ref
                        .as_object_mut()
                        .unwrap()
                        .entry(current)
                        .or_insert(json!(Map::new()));
                } else {
                    // the only part remaining should be a git repo
                    let (_, base) = parts.next().unwrap();
                    let (branch, provider) = find_meta(&original);
                    let value = json!({
                        "location": original,
                        "label": base.to_str(),
                        "branch": branch,
                        "provider": provider
                    });
                    // now let's insert this repo with current directory as key
                    if let Some(_obj) = _ref.as_object_mut() {
                        if let Some(_key) = _obj.get_mut(current) {
                            add_child(_key, value);
                        } else {
                            _obj.insert(current.to_string(), json!([value].to_vec()));
                        }
                    }
                }
            }
            acc
        });

    Ok(to_string(&val)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_json_eq(expected: &str, actual: &str) {
        let expected_json: Value = serde_json::from_str(expected).unwrap();
        let actual_json: Value = serde_json::from_str(actual).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn test_as_tree_single_directory() {
        let dirs = vec![PathBuf::from("/root/dir1")];
        let result = as_tree(dirs).unwrap();
        assert_json_eq(
            &result,
            r#"{"_children":[{"label":"dir1","location":"/root/dir1","branch": "unknown", "provider": "unknown"}]}"#,
        );
    }

    #[test]
    fn test_as_tree_common_path() {
        let dirs = vec![
            PathBuf::from("/root/dir1/repo1"),
            PathBuf::from("/root/dir1/repo2"),
            PathBuf::from("/root/dir2/repo3"),
        ];
        let result = as_tree(dirs).unwrap();
        assert_json_eq(
            &result,
            r#"{"dir1":[{"label":"repo1","location":"/root/dir1/repo1","branch": "unknown", "provider": "unknown"},{"label":"repo2","location":"/root/dir1/repo2","branch": "unknown", "provider": "unknown"}],"dir2":[{"label":"repo3","location":"/root/dir2/repo3","branch": "unknown", "provider": "unknown"}]}"#,
        );
    }

    #[test]
    fn test_as_tree_different_levels() {
        let dirs = vec![
            PathBuf::from("/repo"),
            PathBuf::from("/root/dir1/repo1"),
            PathBuf::from("/root/dir1/dir2/dir3/dir4/repo4"),
            PathBuf::from("/root/dir1/dir2/repo2"),
            PathBuf::from("/root/dir1/dir2/repo3"),
        ];
        let result = as_tree(dirs).unwrap();
        assert_json_eq(
            &result,
            r#"{"_children":[{"label":"repo","location":"/repo","branch": "unknown", "provider": "unknown"}],"root":{"dir1":{"_children":[{"label":"repo1","location":"/root/dir1/repo1","branch": "unknown", "provider": "unknown"}],"dir2":{"_children":[{"label":"repo2","location":"/root/dir1/dir2/repo2","branch": "unknown", "provider": "unknown"},{"label":"repo3","location":"/root/dir1/dir2/repo3","branch": "unknown", "provider": "unknown"}],"dir3":{"dir4":[{"label":"repo4","location":"/root/dir1/dir2/dir3/dir4/repo4","branch": "unknown", "provider": "unknown"}]}}}}}"#,
        );
    }

    #[test]
    fn test_as_tree_shared_subdirectories() {
        let dirs = vec![
            PathBuf::from("/root/dir1/shared/repo1"),
            PathBuf::from("/root/dir2/shared/repo2"),
        ];
        let result = as_tree(dirs).unwrap();
        assert_json_eq(
            &result,
            r#"{"dir1":{"shared":[{"label":"repo1","location":"/root/dir1/shared/repo1","branch": "unknown", "provider": "unknown"}]},"dir2":{"shared":[{"label":"repo2","location":"/root/dir2/shared/repo2","branch": "unknown", "provider": "unknown"}]}}"#,
        );
    }
}
