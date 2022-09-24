//! Read magic file bundled in crate

use super::MagicRule;
use crate::MIME;
use fnv::FnvHashMap;
use lazy_static::lazy_static;
use petgraph::prelude::*;

/// Preload alias list
lazy_static! {
  static ref ALIASES: FnvHashMap<MIME, MIME> = init::get_aliaslist();
}

/// Load magic file before anything else.
lazy_static! {
  static ref ALLRULES: FnvHashMap<MIME, DiGraph<MagicRule<'static>, u32>> = rules();
}

pub mod check;
pub mod init;

mod runtime;

fn rules() -> FnvHashMap<MIME, DiGraph<MagicRule<'static>, u32>> {
  runtime::LoadedDatabase::load_xdg_shared_magic()
    .unwrap()
    .rules()
    .unwrap()
}
