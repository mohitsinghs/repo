use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::path_extra::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn is_match(path: &Path, match_term: Arc<Vec<&str>>) -> bool {
    if let Some((base_term, parent_terms)) = match_term.split_last() {
        let matcher = SkimMatcherV2::default();
        let base_match = matcher
            .fuzzy_match(&path.base_lowercase(), base_term.as_ref())
            .is_some();
        if base_match {
            if parent_terms.is_empty() {
                return true;
            }
            return matcher
                .fuzzy_match(
                    &path
                        .parent()
                        .map(|p| p.to_string().to_lowercase())
                        .unwrap_or_default(),
                    &parent_terms.join("").to_lowercase(),
                )
                .is_some();
        }
    }
    false
}

pub fn find_match(terms: Vec<&str>, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    if let Some((base_term, parent_terms)) = terms.split_last() {
        let matcher = SkimMatcherV2::default();
        let mut matched: Vec<(i64, PathBuf)> = dirs
            .into_iter()
            .filter_map(|p| {
                if p.base_lowercase() == base_term.to_lowercase() {
                    return Some((i64::MAX, p));
                }
                matcher
                    .fuzzy_match(&p.base_lowercase(), base_term.as_ref())
                    .map(|score| (score, p))
            })
            .collect();

        if parent_terms.is_empty() {
            matched.sort_by_key(|(score, _)| -score);
            return matched.first().map(|(_, pb)| pb.to_owned());
        } else {
            let mut result: Vec<(i64, PathBuf)> = matched
                .into_iter()
                .filter_map(|(_, pb)| {
                    matcher
                        .fuzzy_match(
                            &pb.parent()
                                .map(|p| p.to_string().to_lowercase())
                                .unwrap_or_default(),
                            &parent_terms.join("").to_lowercase(),
                        )
                        .map(|score| (score, pb))
                })
                .collect();
            result.sort_by_key(|(score, _)| -score);
            return result.first().map(|(_, pb)| pb.to_owned());
        }
    }
    None
}
