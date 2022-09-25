use super::LoadedDatabase;
use crate::{fdo_magic, read_bytes, MIME};
use fnv::FnvHashMap;
use petgraph::prelude::*;
use std::{path::Path, sync::Arc};

pub struct FdoMagic {
  ldb: Arc<LoadedDatabase>,
}

impl FdoMagic {
  pub fn new(ldb: Arc<LoadedDatabase>) -> Self {
    Self { ldb }
  }
}

impl crate::Checker for FdoMagic {
  /// Test against all rules
  fn from_u8(&self, file: &[u8], mimetype: &str) -> bool {
    // Get magic ruleset
    let rules = super::rules(&self.ldb);
    let graph = match rules.get(mimetype) {
      Some(item) => item,
      None => return false, // No rule for this mime
    };

    // Check all rulesets
    for x in graph.externals(Incoming) {
      if fdo_magic::check::from_u8_walker(file, mimetype, graph, x, true) {
        return true;
      }
    }

    false
  }

  /// This only exists for the case of a direct match_filepath call
  /// and even then we could probably get rid of this...
  fn from_filepath(&self, filepath: &Path, mimetype: &str) -> bool {
    // Get magic ruleset
    let rules = super::rules(&self.ldb);
    let magic_rules = match rules.get(mimetype) {
      Some(item) => item,
      None => return false, // No rule for this mime
    };

    // Get # of bytes to read
    let mut scanlen = 0;
    for x in magic_rules.raw_nodes() {
      let y = &x.weight;
      let tmplen = y.start_off as usize + y.val.len() + y.region_len as usize;

      if tmplen > scanlen {
        scanlen = tmplen;
      }
    }

    let b = match read_bytes(filepath, scanlen) {
      Ok(x) => x,
      Err(_) => return false,
    };
    self.from_u8(b.as_slice(), mimetype)
  }

  fn get_supported(&self) -> Vec<MIME> {
    super::init::get_supported(&self.ldb)
  }

  fn get_subclasses(&self) -> Vec<(MIME, MIME)> {
    super::init::get_subclasses(&self.ldb)
  }

  fn get_aliaslist(&self) -> FnvHashMap<MIME, MIME> {
    super::init::get_aliaslist(&self.ldb)
  }
}
