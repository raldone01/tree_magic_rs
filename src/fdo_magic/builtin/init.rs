use crate::MIME;
use fnv::FnvHashMap;

pub fn get_aliaslist() -> FnvHashMap<MIME, MIME> {
    include_str!("aliases")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            (a, b)
        })
        .collect()
}

/// Get list of supported MIME types
pub fn get_supported() -> Vec<MIME> {
    super::ALLRULES.keys().cloned().collect()
}

/// Get list of parent -> child subclass links
pub fn get_subclasses() -> Vec<(MIME, MIME)> {
    include_str!("subclasses")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let child = parts.next().unwrap();
            let child = super::ALIASES.get(child).copied().unwrap_or(child);

            let parent = parts.next().unwrap();
            let parent = super::ALIASES.get(parent).copied().unwrap_or(parent);

            (parent, child)
        })
        .collect()
}
