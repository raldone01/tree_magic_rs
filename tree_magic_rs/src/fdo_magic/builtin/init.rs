use super::{runtime, LoadedDatabase};
use crate::MIME;
use fnv::FnvHashMap;

pub fn get_aliaslist(ldb: &runtime::LoadedDatabase) -> FnvHashMap<MIME, MIME> {
  ldb
    .aliases()
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
pub fn get_supported(ldb: &LoadedDatabase) -> Vec<MIME> {
  super::rules(ldb).keys().cloned().collect()
}

/// Get list of parent -> child subclass links
pub fn get_subclasses(ldb: &runtime::LoadedDatabase) -> Vec<(MIME, MIME)> {
  ldb
    .subclasses()
    .lines()
    .map(|line| {
      let mut parts = line.split_whitespace();

      let child = parts.next().unwrap();
      let child = super::init::get_aliaslist(ldb)
        .get(child)
        .copied()
        .unwrap_or(child);

      let parent = parts.next().unwrap();
      let parent = super::init::get_aliaslist(ldb)
        .get(parent)
        .copied()
        .unwrap_or(parent);

      (parent, child)
    })
    .collect()
}
