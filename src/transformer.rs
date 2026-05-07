use crate::{
    git::{detect_provider, load_aliases, resolve_url},
    path_extra::*,
};
use anyhow::{Error, Result};
use serde::{ser::SerializeMap, Serialize, Serializer};
use serde_json::{json, to_string, Map};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Serialize)]
pub struct RepoMeta {
    location: PathBuf,
    label: String,
    branch: String,
    provider: String,
}

#[derive(Default)]
pub struct TrieNode {
    repos: Vec<RepoMeta>,
    subdirs: HashMap<String, TrieNode>,
}

impl TrieNode {
    fn insert(&mut self, components: Vec<&str>, repo: RepoMeta) {
        if let Some((_, dirs)) = components.split_last() {
            let mut current: &mut TrieNode = self;
            for dir in dirs {
                current = current.subdirs.entry(dir.to_string()).or_default()
            }
            current.repos.push(repo)
        }
    }
}

impl Serialize for TrieNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.subdirs.is_empty() {
            self.repos.serialize(serializer)
        } else {
            let mut map = serializer.serialize_map(None)?;
            if !self.repos.is_empty() {
                map.serialize_entry("_children", &self.repos)?;
            }
            for (k, v) in &self.subdirs {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    }
}

pub fn as_path_names(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter()
        .map(|d| d.base().unwrap_or_default())
        .collect()
}

pub fn as_paths(dirs: Vec<PathBuf>) -> Vec<String> {
    dirs.into_iter().map(|d| d.to_string()).collect()
}

pub fn as_json(dirs: Vec<PathBuf>) -> Result<String> {
    let aliases = load_aliases();
    let mut map = HashMap::new();

    for val in dirs {
        let (branch, provider) = find_meta(&val, &aliases);
        let label = val.base().unwrap_or_default();
        let value = json!({
            "location": val.to_string(),
            "label": label,
            "branch": branch,
            "provider": provider
        });
        map.insert(label, value);
    }
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

fn find_meta(loc: &PathBuf, aliases: &HashMap<String, String>) -> (String, String) {
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
                .and_then(|line| {
                    let url = line.split("=").nth(1).map(|s| s.trim().to_string())?;
                    let resolved_url = resolve_url(&url, aliases);
                    Some(detect_provider(&resolved_url))
                })
        })
        .unwrap_or_else(|| "unknown".to_string());

    (branch, provider)
}

// build a tree out of detected paths
pub fn as_tree(dirs: Vec<PathBuf>) -> Result<String> {
    if dirs.is_empty() {
        return Ok("{}".to_string());
    }
    let aliases = load_aliases();
    // if there's only one directory, use its parent
    // as the common path otherwise, find the common path
    let common = if dirs.len() == 1 {
        dirs[0].parent().unwrap().to_path_buf()
    } else {
        common_path(&dirs)
    };

    let mut root = TrieNode::default();
    for dir in dirs {
        let stripped = dir.strip_prefix(&common).unwrap();
        let components: Vec<&str> = stripped
            .components()
            .map(|c| c.as_os_str().to_str().unwrap_or_default())
            .collect();
        let (branch, provider) = find_meta(&dir, &aliases);
        let label = dir.base().unwrap_or_default();

        let meta = RepoMeta {
            location: dir.clone(),
            label,
            branch,
            provider,
        };
        root.insert(components, meta);
    }
    if root.subdirs.is_empty() && !root.repos.is_empty() {
        let mut map = Map::new();
        map.insert("_children".to_string(), json!(&root.repos));
        return Ok(to_string(&map)?);
    } else {
        return Ok(to_string(&root)?);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

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
