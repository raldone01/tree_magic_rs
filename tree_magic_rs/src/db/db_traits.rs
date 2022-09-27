//! This module contains the traits that make up the database.
use std::{collections::VecDeque, path::PathBuf};
use thiserror::Error;

/// https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-0.21.html#s2_layout
pub trait MagicRule {
  #[must_use]
  fn priority(&self) -> u32;
  #[must_use]
  fn mime_type(&self) -> &str;
  #[must_use]
  fn indent_level(&self) -> u32;
  #[must_use]
  fn start_off(&self) -> u32;
  #[must_use]
  fn val(&self) -> &[u8];
  #[must_use]
  fn mask(&self) -> &[u8];
  #[must_use]
  fn word_len(&self) -> u32;
  #[must_use]
  fn region_len(&self) -> u32;
}

pub trait Alias {
  /// The name to be mapped
  #[must_use]
  fn alias(&self) -> &str;
  /// The name to map to
  #[must_use]
  fn name(&self) -> &str;
}

pub trait Subclass {
  //FIXME: was: String No idea what this should be
  #[must_use]
  fn str(&self) -> &str;
}

// TODO: Create fast impls of MagicRule and Alias and Subclass and a FastDbProvider

/// Database providers should implement this trait.
pub trait DbProvider<'a> {
  type MagicRule: MagicRule + ?Sized;
  /// All magic rules
  #[must_use]
  fn iter_magic_rules(&'a self) -> Box<dyn Iterator<Item = &Self::MagicRule> + 'a>;
  type Alias: Alias + ?Sized;
  /// All aliases
  // fn aliases<'a>(&'a self) -> impl Iterator<Item = impl Alias + 'a>; NOT OBJECT SAFE!
  // fn foreach_alias<C: FnMut(impl Alias)>(&self, f: C);
  #[must_use]
  fn iter_aliases(&'a self) -> Box<dyn Iterator<Item = &Self::Alias> + 'a>;
  type Subclass: Subclass + ?Sized;
  /// Alls subclass files
  #[must_use]
  fn iter_subclasses(&'a self) -> Box<dyn Iterator<Item = &Self::Subclass> + 'a>;
}
