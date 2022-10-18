use crate::path_extra::*;
use std::cmp::max;
use std::path::{Path, PathBuf};

struct Matched {
    path: PathBuf,
    length: usize,
}

impl Matched {
    pub fn new(path: PathBuf, length: usize) -> Self {
        Self { path, length }
    }
}

pub fn split_matches(terms: Vec<&str>) -> (Option<&str>, Option<String>) {
    if let Some((base, others)) = terms.split_last() {
        return (Some(base), Some(others.join("")));
    } else {
        return (None, None);
    }
}

pub fn is_subsequence(to_be_matched: &str, to_match_in: &str) -> bool {
    let mut i = 0;
    let mut j = 0;
    let candidate_bytes = to_be_matched.as_bytes();
    let lookup_bytes = to_match_in.as_bytes();

    while i < to_be_matched.len() && j < to_match_in.len() {
        if candidate_bytes[i] == lookup_bytes[j] {
            i += 1;
        }
        j += 1;
    }
    i == to_be_matched.len()
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

pub fn is_match(path: &Path, base_term: &str, other_terms: &str) -> bool {
    let is_sub = is_subsequence(base_term, &path.base().unwrap_or_default().to_lowercase());
    if !other_terms.is_empty() {
        is_sub
            && is_subsequence(
                other_terms,
                path.parent()
                    .map(|p| p.to_string().to_lowercase())
                    .unwrap_or_default()
                    .as_str(),
            )
    } else {
        is_sub
    }
}

pub fn find_match(terms: Vec<&str>, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    if let Some((base, others)) = terms.split_last() {
        let matched = dirs
            .into_iter()
            .filter(|p| is_subsequence(base, &p.base().unwrap_or_default()));
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
