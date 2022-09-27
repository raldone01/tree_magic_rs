use std::marker::PhantomData;

use crate::db::{Alias, MagicRule, Subclass};

use super::BuildeableDbProvider;

#[derive(Clone)]
pub struct OwnedAlias {
  alias: String,
  name: String,
}
impl OwnedAlias {
  #[must_use]
  pub fn new(alias: String, name: String) -> Self {
    Self { alias, name }
  }
}
impl Alias for OwnedAlias {
  fn alias(&self) -> &str {
    &self.alias
  }
  fn name(&self) -> &str {
    &self.name
  }
}

#[derive(Clone)]
pub struct OwnedMagicRule {
  priority: u32,
  mime_type: String,
  indent_level: u32,
  start_off: u32,
  val: Box<[u8]>,
  mask: Box<[u8]>,
  word_len: u32,
  region_len: u32,
}
impl OwnedMagicRule {
  #[must_use]
  pub fn new(
    priority: u32,
    mime_type: String,
    indent_level: u32,
    start_off: u32,
    val: Box<[u8]>,
    mask: Box<[u8]>,
    word_len: u32,
    region_len: u32,
  ) -> Self {
    Self {
      priority,
      mime_type,
      indent_level,
      start_off,
      val,
      mask,
      word_len,
      region_len,
    }
  }
}
impl MagicRule for OwnedMagicRule {
  fn priority(&self) -> u32 {
    self.priority
  }

  fn mime_type(&self) -> &str {
    &self.mime_type
  }

  fn indent_level(&self) -> u32 {
    self.indent_level
  }

  fn start_off(&self) -> u32 {
    self.start_off
  }

  fn val(&self) -> &[u8] {
    &self.val
  }

  fn mask(&self) -> &[u8] {
    &self.mask
  }

  fn word_len(&self) -> u32 {
    self.word_len
  }

  fn region_len(&self) -> u32 {
    self.region_len
  }
}

// TODO: !
#[derive(Clone, Copy)]
pub struct OwnedSubclass {
  _data: u8,
}
impl OwnedSubclass {
  #[must_use]
  pub fn new() -> Self {
    Self { _data: 1 }
  }
}
impl Subclass for OwnedSubclass {
  fn str(&self) -> &str {
    todo!()
  }
}

pub type OwnedBuildableDb = BuildeableDbProvider<OwnedMagicRule, OwnedAlias, OwnedSubclass>;
