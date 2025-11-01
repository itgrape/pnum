pub mod error;

use error::{PnumError, PnumResult};
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Default)]
pub struct Config {
    pub recursive: bool,
    pub ignore_hidden: bool,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

fn normalize_ext(ext: &str) -> String {
    // remove leading dot and convert to lowercase
    ext.trim().strip_prefix('.').unwrap_or(ext.trim()).to_ascii_lowercase()
}

pub fn count_extensions(path: &Path, config: &Config) -> PnumResult<HashMap<String, u32>> {
    if !path.is_dir() {
        return Err(PnumError::InvalidPath(path.to_path_buf()));
    }

    let includes: HashSet<String> = config.include.iter().map(|s| normalize_ext(s)).collect();
    let excludes: HashSet<String> = config.exclude.iter().map(|s| normalize_ext(s)).collect();

    let mut walker = WalkBuilder::new(path);
    walker.hidden(config.ignore_hidden);
    if !config.recursive {
        walker.max_depth(Some(1));
    }

    let counts = walker
        .build()
        .into_iter()
        .filter_map(|entry_res| entry_res.ok())
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext_str| ext_str.to_string())
        })
        .par_bridge()
        .filter(|ext_str| {
            let ext = ext_str.as_str();
            let included = includes.is_empty() || includes.contains(ext);
            let excluded = !excludes.is_empty() && excludes.contains(ext);
            included && !excluded
        })
        .fold(HashMap::new, |mut map, ext| {
            *map.entry(ext).or_insert(0) += 1;
            map
        })
        .reduce(HashMap::new, |mut map_a, map_b| {
            for (ext, count) in map_b {
                *map_a.entry(ext).or_insert(0) += count;
            }
            map_a
        });

    Ok(counts)
}
