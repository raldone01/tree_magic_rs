//! Read magic file bundled in crate

use super::MagicRule;
use crate::MIME;
use fnv::FnvHashMap;
use petgraph::prelude::*;

pub mod check;
pub use check::FdoMagic;
pub mod init;

mod runtime;
pub use runtime::LoadedDatabase;

fn rules<'a>(ldb: &'a runtime::LoadedDatabase) -> FnvHashMap<MIME, DiGraph<MagicRule<'a>, u32>> {
  ldb.rules().unwrap()
}
