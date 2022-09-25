use crate::MIME;
use fnv::FnvHashMap;

pub fn get_supported() -> Vec<MIME<'static>> {
  super::TYPES.to_vec()
}

/// Returns Vec of parent->child relations
pub fn get_subclasses() -> Vec<(MIME<'static>, MIME<'static>)> {
  vec![
    ("all/all", "all/allfiles"),
    ("all/all", "inode/directory"),
    ("all/allfiles", "application/octet-stream"),
    ("application/octet-stream", "text/plain"),
  ]
}

pub fn get_aliaslist() -> FnvHashMap<MIME<'static>, MIME<'static>> {
  FnvHashMap::default()
}
