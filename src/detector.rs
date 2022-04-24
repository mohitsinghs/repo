use dirs::home_dir;
use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

use crate::config::Root;

pub fn find_git_repos(loc: &Path, depth: Option<usize>, term: Option<&str>) -> Vec<DirEntry> {
    let walker = WalkBuilder::new(loc).max_depth(depth).build();
    walker
        .into_iter()
        .filter_map(|res| res.ok())
        .filter(|d| {
            let p = d.path();
            if let Some(t) = term {
                is_git_repo(p) && p.to_str().map(|s| s.contains(t)).unwrap_or_default()
            } else {
                is_git_repo(p)
            }
        })
        .collect()
}

pub fn traverse_roots(roots: Vec<Root>, term: Option<&str>) -> Vec<DirEntry> {
    if roots.is_empty() {
        let home = home_dir().unwrap_or_default();
        find_git_repos(home.as_path(), None, term)
    } else {
        roots
            .into_iter()
            .filter(|root| Path::new(&root.path).is_dir())
            .flat_map(|root| {
                if root.depth == 0 {
                    find_git_repos(Path::new(&root.path), None, term)
                } else {
                    find_git_repos(Path::new(&root.path), Some(root.depth), term)
                }
            })
            .collect()
    }
}

pub fn as_path_names(dirs: Vec<DirEntry>) -> Vec<String> {
    dirs.into_iter()
        .map(|d| d.path().file_name().unwrap().to_str().unwrap().to_string())
        .collect()
}

pub fn as_paths(dirs: Vec<DirEntry>) -> Vec<String> {
    dirs.into_iter()
        .map(|d| d.path().to_str().unwrap().to_string())
        .collect()
}

pub fn find_path(term: &str, dirs: Vec<DirEntry>) -> Option<DirEntry> {
    dirs.into_iter()
        .find(|d| d.path().to_str().map(|s| s.contains(term)).unwrap_or(false))
}

pub fn is_git_repo(loc: &Path) -> bool {
    loc.join(".git").join("config").exists()
}
