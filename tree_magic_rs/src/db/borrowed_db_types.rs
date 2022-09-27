use std::marker::PhantomData;

use crate::db::{Alias, MagicRule, Subclass};

use super::BuildeableDbProvider;

#[derive(Clone, Copy)]
pub struct BorrowedAlias<'a> {
  alias: &'a str,
  name: &'a str,
}
impl<'a> BorrowedAlias<'a> {
  #[must_use]
  pub fn new(alias: &'a str, name: &'a str) -> Self {
    Self { alias, name }
  }
}
impl<'a> Alias for BorrowedAlias<'a> {
  fn alias(&self) -> &str {
    self.alias
  }
  fn name(&self) -> &str {
    self.name
  }
}

#[derive(Clone, Copy)]
pub struct BorrowedMagicRule<'a> {
  priority: u32,
  mime_type: &'a str,
  indent_level: u32,
  start_off: u32,
  val: &'a [u8],
  mask: &'a [u8],
  word_len: u32,
  region_len: u32,
}
impl<'a> BorrowedMagicRule<'a> {
  #[must_use]
  pub fn new(
    priority: u32,
    mime_type: &'a str,
    indent_level: u32,
    start_off: u32,
    val: &'a [u8],
    mask: &'a [u8],
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
impl<'a> MagicRule for BorrowedMagicRule<'a> {
  fn priority(&self) -> u32 {
    self.priority
  }

  fn mime_type(&self) -> &str {
    self.mime_type
  }

  fn indent_level(&self) -> u32 {
    self.indent_level
  }

  fn start_off(&self) -> u32 {
    self.start_off
  }

  fn val(&self) -> &[u8] {
    self.val
  }

  fn mask(&self) -> &[u8] {
    self.mask
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
pub struct BorrowedSubclass<'a> {
  _data: PhantomData<&'a ()>,
}
impl<'a> BorrowedSubclass<'a> {
  #[must_use]
  pub fn new() -> Self {
    Self { _data: PhantomData }
  }
}
impl<'a> Subclass for BorrowedSubclass<'a> {
  fn str(&self) -> &str {
    todo!()
  }
}

pub type BorrowedBuildableDb<'a> =
  BuildeableDbProvider<BorrowedMagicRule<'a>, BorrowedAlias<'a>, BorrowedSubclass<'a>>;
