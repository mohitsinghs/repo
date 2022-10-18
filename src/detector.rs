use crate::{
    config::Root,
    matcher::{is_match, split_matches},
};
use dirs::home_dir;
use ignore::{WalkBuilder, WalkState};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Sender},
};

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

#[cfg(target_os = "macos")]
fn system_paths() -> Vec<PathBuf> {
    let home = home_dir().unwrap();
    // these are the biggest culprits for slowness on mac
    // these will be ignored unless someone adds these manually
    ["Library", "Applications"]
        .map(|base| home.join(base))
        .to_vec()
}

pub fn find_git_repos(
    loc: &Path,
    depth: Option<usize>,
    base_term: Option<&str>,
    other_terms: &str,
) -> Vec<PathBuf> {
    let (tx, rx) = mpsc::channel();
    let mut entries: Vec<PathBuf> = Vec::new();
    WalkBuilder::new(loc)
        .max_depth(depth)
        .build_parallel()
        .run(|| {
            let ttx = tx.clone();
            Box::new(move |res| {
                if let Ok(entry) = res {
                    if entry.depth() == 0 {
                        return WalkState::Continue;
                    }
                    let path = entry.path();

                    #[cfg(target_os = "macos")]
                    if system_paths().contains(&path.to_path_buf()) {
                        return WalkState::Skip;
                    }

                    if let Some(t) = base_term {
                        walk(
                            path,
                            &ttx,
                            is_git_repo(path) && is_match(&path, t, other_terms),
                        )
                    } else {
                        walk(path, &ttx, is_git_repo(path))
                    }
                } else {
                    WalkState::Skip
                }
            })
        });
    drop(tx);
    while let Ok(entry) = rx.recv() {
        entries.push(entry)
    }
    entries
}

pub fn traverse_roots(roots: Vec<Root>, terms: Option<Vec<&str>>) -> Vec<PathBuf> {
    // FIXME: find a better way to extract and pass this
    let (base, others) = split_matches(terms.unwrap_or_default());
    let intermediate = others.unwrap_or_default();
    let other_terms = intermediate.as_str();

    if roots.is_empty() {
        let home = home_dir().unwrap_or_default();
        find_git_repos(home.as_path(), None, base, other_terms)
    } else {
        roots
            .into_iter()
            .filter(|root| Path::new(&root.path).is_dir())
            .flat_map(|root| {
                if root.depth == 0 {
                    find_git_repos(Path::new(&root.path), None, base, other_terms)
                } else {
                    find_git_repos(Path::new(&root.path), Some(root.depth), base, other_terms)
                }
            })
            .collect()
    }
}

pub fn is_git_repo(loc: &Path) -> bool {
    loc.is_dir() && loc.join(".git").join("config").exists()
}
