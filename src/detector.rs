use crate::config::Root;
use dirs::home_dir;
use ignore::{WalkBuilder, WalkState};
use slog::{info, Logger};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Sender},
};

pub fn matches(path: &Path, term: &str) -> bool {
    path.to_str().map(|s| s.contains(term)).unwrap_or_default()
}

pub fn walk(path: &Path, ttx: &Sender<PathBuf>, is_match: bool) -> WalkState {
    if is_match {
        let s = ttx.send(path.to_path_buf());
        if s.is_err() {
            return WalkState::Quit;
        }
        WalkState::Skip
    } else if path.is_dir() {
        WalkState::Continue
    } else {
        WalkState::Skip
    }
}

pub fn find_git_repos(
    loc: &Path,
    depth: Option<usize>,
    term: Option<&str>,
    log: Logger,
) -> Vec<PathBuf> {
    let (tx, rx) = mpsc::channel();
    let mut entries: Vec<PathBuf> = Vec::new();
    info!(log, "started walking"; "location" => loc.to_str());
    WalkBuilder::new(loc)
        .max_depth(depth)
        .build_parallel()
        .run(|| {
            let ttx = tx.clone();
            Box::new(move |res| {
                if let Ok(entry) = res {
                    let path = entry.path();
                    if let Some(t) = term {
                        walk(path, &ttx, is_git_repo(path) && matches(path, t))
                    } else {
                        walk(path, &ttx, is_git_repo(path))
                    }
                } else {
                    WalkState::Quit
                }
            })
        });
    drop(tx);
    info!(log, "collecting journey"; "location" => loc.to_str());
    while let Ok(entry) = rx.recv() {
        entries.push(entry)
    }
    info!(log, "collected entries"; "count" => entries.len(), "location" => loc.to_str());
    entries
}

pub fn traverse_roots(roots: Vec<Root>, term: Option<&str>, log: Logger) -> Vec<PathBuf> {
    if roots.is_empty() {
        info!(log, "no roots defined. using home");
        let home = home_dir().unwrap_or_default();
        info!(log, "detected home" ;"home" => home.to_str());
        find_git_repos(home.as_path(), None, term, log.clone())
    } else {
        info!(log, "scanning roots");
        roots
            .into_iter()
            .filter(|root| Path::new(&root.path).is_dir())
            .flat_map(|root| {
                info!(log, "inside root" ;"root" => &root.path);
                if root.depth == 0 {
                    find_git_repos(Path::new(&root.path), None, term, log.clone())
                } else {
                    find_git_repos(Path::new(&root.path), Some(root.depth), term, log.clone())
                }
            })
            .collect()
    }
}

pub fn is_git_repo(loc: &Path) -> bool {
    loc.join(".git").join("config").exists()
}
