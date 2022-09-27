use super::parse_magic_rule::{parse_magic_file, MagicRuleParseError};
use crate::db::{BuildeableDbProvider, OwnedAlias, OwnedMagicRule, OwnedSubclass};
use derive_more::{Add, Sum};
use std::{
  fs::File,
  io::Read,
  path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharedMimeDbProviderError {
  #[error("IO error")]
  IoError {
    file_path: PathBuf,
    source: std::io::Error,
  },
  #[error("Error parsing magic file")]
  MagicRuleParseError {
    file_path: PathBuf,
    source: MagicRuleParseError,
  },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, Sum)]
pub struct LoadResult {
  pub magic_rules_num: usize,
  pub aliases_num: usize,
  pub subclasses_num: usize,
}

/// Loads the database from the disk.
pub trait SharedMimeDbProviderExt {
  /// Attempts to load the mime database from well known directories.
  /// Only returns an error if no file could be opened or if there was an error reading a file.
  fn load_from_xdg_shared_magic_default(
    &mut self,
  ) -> Result<LoadResult, (LoadResult, Vec<SharedMimeDbProviderError>)>;
  /// Attempts to load the mime database from a user specified directory.
  /// Only returns an error if no file could be opened or if there was an error reading a file.
  fn load_from_xdg_shared_magic_dir(
    &mut self,
    dir: &Path,
  ) -> Result<LoadResult, (LoadResult, Vec<SharedMimeDbProviderError>)>;
  /// Attempts to load magic rules from a user specified file.
  /// Returns the amount loaded magic rules.
  /// Returns an error if the file could be opened or there was an error reading a file.
  fn load_magic_rules_file(
    &mut self,
    file: &Path,
  ) -> Result<usize, (usize, SharedMimeDbProviderError)>;
  /// Attempts to load aliases from a user specified file.
  /// Returns the amount loaded aliases.
  /// Returns an error if the file could be opened or there was an error reading a file.
  fn load_aliases_file(&mut self, file: &Path)
    -> Result<usize, (usize, SharedMimeDbProviderError)>;
  /// Attempts to load subclasses from a user specified file.
  /// Returns the amount loaded subclasses.
  /// Returns an error if the file could be opened or there was an error reading a file.
  fn load_subclasses_file(
    &mut self,
    file: &Path,
  ) -> Result<usize, (usize, SharedMimeDbProviderError)>;
}
impl SharedMimeDbProviderExt for BuildeableDbProvider<OwnedMagicRule, OwnedAlias, OwnedSubclass> {
  fn load_from_xdg_shared_magic_default(
    &mut self,
  ) -> Result<LoadResult, (LoadResult, Vec<SharedMimeDbProviderError>)> {
    // FIXME: Once Path::new is const
    const SEARCH_DIRS: &[&str; 3] = &[
      "$HOME/.local/share/mime",
      "/usr/local/share/mime",
      "/usr/share/mime",
    ];
    let errs = Vec::new();
    let load_result = SEARCH_DIRS
      .iter()
      .map(|str_path| {
        let dir = Path::new(str_path);
        let result = self.load_from_xdg_shared_magic_dir(dir);
        match result {
          Err(err) => {
            errs.extend(err.1);
            err.0
          },
          Ok(ok) => ok,
        }
      })
      .sum();
    if !errs.is_empty() {
      return Err((load_result, errs));
    }
    Ok(load_result)
  }

  fn load_from_xdg_shared_magic_dir(
    &mut self,
    dir: &Path,
  ) -> Result<LoadResult, (LoadResult, Vec<SharedMimeDbProviderError>)> {
    let errs = Vec::new();

    let unpack = |packed: Result<usize, (usize, SharedMimeDbProviderError)>| -> usize {
      match packed {
        Err(err) => {
          errs.push(err.1);
          err.0
        },
        Ok(ok) => ok,
      }
    };

    let load_result = LoadResult {
      magic_rules_num: unpack(self.load_magic_rules_file(&dir.join("magic"))),
      aliases_num: unpack(self.load_aliases_file(&dir.join("aliases"))),
      subclasses_num: unpack(self.load_subclasses_file(&dir.join("subclasses"))),
    };
    if !errs.is_empty() {
      return Err((load_result, errs));
    }
    Ok(load_result)
  }

  fn load_magic_rules_file(
    &mut self,
    file_path: &Path,
  ) -> Result<usize, (usize, SharedMimeDbProviderError)> {
    let file = File::open(file_path);
    let Ok(file) = file else {
      return Ok(0);
    };

    fn unpack_usize(packed: Result<usize, (usize, MagicRuleParseError)>) -> usize {
      match packed {
        Err(err) => err.0,
        Ok(ok) => ok,
      }
    }

    let mut io_error = Ok(());
    let bytes = file.bytes().scan(&mut io_error, until_err);
    let parse_result = parse_magic_file(bytes, &mut self.magic_rules_mut());
    let loaded = unpack_usize(parse_result);
    if let Err(io_error) = io_error {
      return Err((
        loaded,
        SharedMimeDbProviderError::IoError {
          file_path: file_path.to_owned(),
          source: io_error,
        },
      ));
    }
    parse_result.map_err(|err| {
      (
        err.0,
        SharedMimeDbProviderError::MagicRuleParseError {
          file_path: file_path.to_owned(),
          source: err.1,
        },
      )
    })
  }

  fn load_aliases_file(
    &mut self,
    file: &Path,
  ) -> Result<usize, (usize, SharedMimeDbProviderError)> {
    todo!()
  }

  fn load_subclasses_file(
    &mut self,
    file: &Path,
  ) -> Result<usize, (usize, SharedMimeDbProviderError)> {
    todo!()
  }
}
/// https://stackoverflow.com/a/63120052/4479969
fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
  match item {
    Ok(item) => Some(item),
    Err(e) => {
      **err = Err(e);
      None
    },
  }
}
fn count_errs<T, E>(errs: &mut &mut Vec<E>, item: Result<T, E>) -> Option<T> {
  match item {
    Ok(item) => Some(item),
    Err(e) => {
      (*errs).push(e);
      None
    },
  }
}
