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
    static ref ALLRULES: FnvHashMap<MIME, DiGraph<MagicRule<'static>, u32>> =
        super::ruleset::from_u8(include_bytes!("magic")).unwrap_or(FnvHashMap::default());
}

pub mod check;
pub mod init;
