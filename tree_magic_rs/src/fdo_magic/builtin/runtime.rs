///! Attempts to load the magic database from well-known directories.
use std::fs::File;
use std::io::Read;

use fnv::FnvHashMap;
use petgraph::prelude::DiGraph;

use super::MagicRule;
use crate::fdo_magic::ruleset;
use crate::MIME;

pub struct LoadedDatabase {
  /// The files loaded into memory
  runtime_rules: Vec<Vec<u8>>,
  alias_string: String,
  subclass_string: String,
}

impl LoadedDatabase {
  /// Load the magic database from the predefined locations in the XDG standard
  pub fn load_xdg_shared_magic() -> Result<Self, String> {
    const SEARCH_PATHS: &[&str; 3] = &[
      "/usr/share/mime/magic",
      "/usr/local/share/mime/magic",
      "$HOME/.local/share/mime/magic",
    ];

    let files: Vec<Vec<u8>> = SEARCH_PATHS
      .iter()
      .map(|p| File::open(p).ok())
      .filter_map(|f| f)
      .map(|mut f| {
        let mut buf = vec![];
        f.read_to_end(&mut buf)
          .map_err(|e| format!("Failed to read magic file bytes: {:#?}", e))?;
        Ok(buf)
      })
      .collect::<Result<_, String>>()?;

    if files.is_empty() {
      Err("No MIME magic files found in the XDG default paths".to_string())
    } else {
      Ok(Self {
        runtime_rules: files,
        alias_string: load_aliases(),
        subclass_string: load_subclasses(),
      })
    }
  }

  pub fn rules(&self) -> Result<FnvHashMap<MIME, DiGraph<MagicRule<'static>, u32>>, String> {
    let files = self.runtime_rules;
    ruleset::from_multiple(&files)
  }

  pub fn aliases(&self) -> &str {
    &self.alias_string
  }

  pub fn subclasses(&self) -> &str {
    &self.subclass_string
  }
}

/// Load a number of files at `paths` and concatenate them together with a newline
fn load_concat_strings(paths: &[&str]) -> String {
  let strings: Vec<String> = paths
    .iter()
    .map(|p| File::open(p).ok())
    .filter_map(|f| f)
    .map(|mut f| {
      let mut s = String::new();
      f.read_to_string(&mut s)
        .expect("Failed to read aliases from file");
      s
    })
    .collect();

  strings.join("\n")
}

/// Load the magic aliases from the XDG standard locations and concatenate them together
fn load_aliases() -> String {
  const SEARCH_PATHS: &[&str; 3] = &[
    "/usr/share/mime/aliases",
    "/usr/local/share/mime/aliases",
    "$HOME/.local/share/mime/aliases",
  ];

  load_concat_strings(SEARCH_PATHS)
}

/// Load the subclass definitions from the XDG standard locations and concatenate them together
fn load_subclasses() -> String {
  const SEARCH_PATHS: &[&str; 3] = &[
    "/usr/share/mime/subclasses",
    "/usr/local/share/mime/subclasses",
    "$HOME/.local/share/mime/subclasses",
  ];

  load_concat_strings(SEARCH_PATHS)
}
