use std::{ops::Sub, path::PathBuf};

use nom::AsBytes;
use thiserror::Error;

/// https://specifications.freedesktop.org/shared-mime-info-spec/shared-mime-info-spec-0.21.html#s2_layout
pub trait MagicRule {
  fn mime_type(&self) -> &str;
  fn indent_level(&self) -> u32;
  fn start_off(&self) -> u32;
  fn val(&self) -> &[u8];
  fn mask(&self) -> &[u8];
  fn word_len(&self) -> u32;
  fn region_len(&self) -> u32;
}

pub trait Alias {
  /// The name to be mapped
  fn alias(&self) -> &str;
  /// The name to map to
  fn name(&self) -> &str;
}

pub trait Subclass {
  //FIXME: was: String No idea what this should be
  fn str(&self) -> &str;
}

/// Database providers should implement this trait.
pub trait DbProvider<'a> {
  type MagicRule: MagicRule + ?Sized;
  /// All magic rules
  fn iter_magic_rules(&'a self) -> Box<dyn Iterator<Item = &'a Self::MagicRule> + 'a>;
  type Alias: Alias + ?Sized;
  /// All aliases
  // fn aliases<'a>(&'a self) -> impl Iterator<Item = impl Alias + 'a>; NOT OBJECT SAFE!
  // fn foreach_alias<C: FnMut(impl Alias)>(&self, f: C);
  fn iter_aliases(&'a self) -> Box<dyn Iterator<Item = &'a Self::Alias>>;
  type Subclass: Subclass + ?Sized;
  /// Alls subclass files
  fn iter_subclasses(&'a self) -> Box<dyn Iterator<Item = &'a Self::Subclass>>;
}

// TODO:
// enum Dbs {
//   Stacked(StackedDbProvider),
//   Buildeable(BuildeableDb),
//   // MimeCache(MimeCacheDb),
//   // XdgSharedMimeCache(XdgSharedMimeCacheDb),
//   // Custom(CustomDb),
// }

type DynDbProvider<'a> = Box<
  dyn DbProvider<
    'a,
    Alias = dyn Alias + 'a,
    Subclass = dyn Subclass + 'a,
    MagicRule = dyn MagicRule + 'a,
  >,
>;

pub struct StackedDbProvider<'a> {
  dbs: Vec<DynDbProvider<'a>>,
}
impl<'a> StackedDbProvider<'a> {
  pub fn new() -> Self {
    Self { dbs: Vec::new() }
  }
  pub fn add_db(&mut self, db: DynDbProvider<'a>) {
    self.dbs.push(db)
  }
}
impl<'a> DbProvider<'a> for StackedDbProvider<'a> {
  type MagicRule = dyn MagicRule;

  fn iter_magic_rules(&'a self) -> Box<dyn Iterator<Item = &'a Self::MagicRule> + 'a> {
    let iter = self.dbs.iter().flat_map(|db| db.iter_magic_rules());
    Box::new(iter)
  }

  type Alias = dyn Alias;

  fn iter_aliases(&'a self) -> Box<dyn Iterator<Item = &'a Self::Alias>> {
    todo!()
  }

  type Subclass = dyn Subclass;

  fn iter_subclasses(&'a self) -> Box<dyn Iterator<Item = &'a Self::Subclass>> {
    todo!()
  }
}

pub struct BorrowedAlias<'a> {
  alias: &'a str,
  name: &'a str,
}
impl<'a> Alias for BorrowedAlias<'a> {
  fn alias(&self) -> &str {
    self.alias
  }

  fn name(&self) -> &str {
    self.name
  }
}

pub struct BuildeableDb {
  magic_rules: Vec<u8>,
  alias_string: String,
  subclass_string: String,
}
impl BuildeableDb {
  pub fn new() -> Self {
    Self {
      magic_rules: Vec::new(),
      alias_string: String::new(),
      subclass_string: String::new(),
    }
  }
  /// Magic rules MUST end with a newline.
  pub fn add_magic_rules(&mut self, magic_rules: &[u8]) {
    self.magic_rules.extend_from_slice(magic_rules)
  }
  /// Aliases MUST end with a newline.
  pub fn add_aliases(&mut self, alias_string: &str) {
    self.alias_string += alias_string;
  }
  pub fn add_subclasses(&mut self, subclass_string: &str) {
    self.subclass_string += subclass_string;
  }
}

// impl DbProvider for BuildeableDb {
//   fn magic_rules(&self) -> &[u8] {
//     self.magic_rules.as_bytes()
//   }

//   fn aliases<'a>(&'a self) -> impl Iterator<Item = impl Alias + 'a> {
//     self.alias_string.split('\n').map(|str| {
//       // TODO: Fix
//       let mut splt = str.split(' ');
//       BorrowedAlias {
//         alias: splt.next().unwrap(),
//         name: splt.next().unwrap(),
//       }
//     })
//   }

//   fn subclasses(&self) -> String {
//     todo!()
//   }
// }

/// Struct that contains the raw database as stored on disk.
pub struct XdbGuts {
  /// All magic files concatenated
  magic_rules: Vec<u8>,
  /// All alias files concatenated
  alias_string: String,
  /// Alls subclass files concatenated
  subclass_string: String,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum LoadXdbSharedMagicError {
  #[error("No such file or directory: {file}")]
  NotFound { file: PathBuf },
  #[error("Platform {unsupported_platform} is not supported")]
  UnsupportedPlatform { unsupported_platform: &'static str },
}

// impl DbGuts {
//   pub fn new_from_xdg_shared_magic() -> Result<Self, LoadXdbSharedMagicError> {}
//   pub fn new_from_xdg_shared_magic_cache() -> Result<Self, LoadXdbSharedMagicError> {}
//   pub fn new_from_();
// }
