use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::path_extra::*;
use std::path::{Path, PathBuf};

pub fn is_match(path: &Path, match_term: &str) -> bool {
    let matcher = SkimMatcherV2::default();
    matcher
        .fuzzy_match(&path.to_string().to_lowercase(), match_term)
        .is_some()
}

pub fn find_match(terms: Vec<&str>, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    let matcher = SkimMatcherV2::default();
    let match_term = terms.join("").to_lowercase();
    let mut matched: Vec<(i64, PathBuf)> = dirs
        .into_iter()
        .filter_map(|p| {
            matcher
                .fuzzy_match(&p.to_string().to_lowercase(), &match_term)
                .map(|score| (score, p))
        })
        .collect();
    matched.sort_by_key(|(score, _)| -score);
    matched.first().map(|(_, pb)| pb.to_owned())
}
