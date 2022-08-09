use crate::path_extra::*;
use std::cmp::max;
use std::path::PathBuf;

struct Matched {
    path: PathBuf,
    length: usize,
}

impl Matched {
    pub fn new(path: PathBuf, length: usize) -> Self {
        Self { path, length }
    }
}

fn lcs(t1: String, t2: String) -> usize {
    let mut mat: Vec<Vec<usize>> = vec![vec![0; t2.len() + 1]; t1.len() + 1];
    for (i, ci) in t1.chars().enumerate() {
        for (j, cj) in t2.chars().enumerate() {
            mat[i + 1][j + 1] = if ci == cj {
                mat[i][j] + 1
            } else {
                max(mat[i][j + 1], mat[i + 1][j])
            }
        }
    }
    mat[t1.len()][t2.len()] as usize
}

pub fn find_match(terms: Vec<&str>, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    if let Some((base, others)) = terms.split_last() {
        let matched = dirs
            .into_iter()
            .filter(|p| p.base().map(|n| n.contains(base)).unwrap_or(false));
        if others.is_empty() {
            return matched
                .into_iter()
                .map(|pb| {
                    let match_len = pb.base().unwrap_or_default().len();
                    Matched::new(pb, match_len)
                })
                .min_by_key(|m| m.length)
                .map(|m| m.path);
        } else {
            return matched
                .into_iter()
                .map(|pb| {
                    let match_len = lcs(
                        others.join("").to_lowercase(),
                        pb.parent()
                            .map(|p| p.to_string().to_lowercase())
                            .unwrap_or_default(),
                    );
                    Matched::new(pb, match_len)
                })
                .max_by_key(|m| m.length)
                .map(|m| m.path);
        }
    }
    None
}
