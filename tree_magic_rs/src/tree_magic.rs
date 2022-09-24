use crate::basetype;
use crate::fdo_magic;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use petgraph::prelude::*;
use std::path::Path;

pub type MIME = &'static str;
pub type TypeStruct = DiGraph<MIME, u32>;

/// TODO
pub struct MimeDatabase {
  /// Information about currently loaded MIME types
  ///
  /// The `graph` contains subclass relations between all given mimes.
  /// (EX: `application/json` -> `text/plain` -> `application/octet-stream`)
  /// This is a `petgraph` DiGraph, so you can walk the tree if needed.
  ///
  /// The `hash` is a mapping between MIME types and nodes on the graph.
  /// The root of the graph is "all/all", so start traversing there unless
  /// you need to jump to a particular node.
  graph: TypeStruct,
  checker_support: FnvHashMap<MIME, &'static dyn Checker>,
  aliases: FnvHashMap<MIME, MIME>,
}
impl MimeDatabase {
  // Initialize filetype graph
  fn graph_init() -> TypeStruct {
    let mut graph = DiGraph::<MIME, u32>::new();
    let mut added_mimes = FnvHashMap::<MIME, NodeIndex>::default();

    // Get list of MIME types and MIME relations
    let mut mimelist = Vec::<MIME>::new();
    let mut edgelist_raw = Vec::<(MIME, MIME)>::new();
    for &c in CHECKERS {
      mimelist.extend(c.get_supported());
      edgelist_raw.extend(c.get_subclasses());
    }
    mimelist.sort_unstable();
    mimelist.dedup();
    let mimelist = mimelist;

    // Create all nodes
    for mimetype in mimelist.iter() {
      let node = graph.add_node(mimetype);
      added_mimes.insert(mimetype, node);
    }

    let mut edge_list = FnvHashSet::<(NodeIndex, NodeIndex)>::with_capacity_and_hasher(
      edgelist_raw.len(),
      Default::default(),
    );
    for x in edgelist_raw {
      let child_raw = x.0;
      let parent_raw = x.1;

      let parent = match added_mimes.get(&parent_raw) {
        Some(node) => *node,
        None => {
          continue;
        },
      };

      let child = match added_mimes.get(&child_raw) {
        Some(node) => *node,
        None => {
          continue;
        },
      };

      edge_list.insert((child, parent));
    }

    graph.extend_with_edges(&edge_list);

    //Add to applicaton/octet-stream, all/all, or text/plain, depending on top-level
    //(We'll just do it here because having the graph makes it really nice)
    let added_mimes_tmp = added_mimes.clone();
    let node_text = match added_mimes_tmp.get("text/plain") {
      Some(x) => *x,
      None => {
        let node = graph.add_node("text/plain");
        added_mimes.insert("text/plain", node);
        node
      },
    };
    let node_octet = match added_mimes_tmp.get("application/octet-stream") {
      Some(x) => *x,
      None => {
        let node = graph.add_node("application/octet-stream");
        added_mimes.insert("application/octet-stream", node);
        node
      },
    };
    let node_allall = match added_mimes_tmp.get("all/all") {
      Some(x) => *x,
      None => {
        let node = graph.add_node("all/all");
        added_mimes.insert("all/all", node);
        node
      },
    };
    let node_allfiles = match added_mimes_tmp.get("all/allfiles") {
      Some(x) => *x,
      None => {
        let node = graph.add_node("all/allfiles");
        added_mimes.insert("all/allfiles", node);
        node
      },
    };

    let mut edge_list_2 = FnvHashSet::<(NodeIndex, NodeIndex)>::default();
    for mimenode in graph.externals(Incoming) {
      let mimetype = &graph[mimenode];
      let toplevel = mimetype.split('/').next().unwrap_or("");

      if mimenode == node_text
        || mimenode == node_octet
        || mimenode == node_allfiles
        || mimenode == node_allall
      {
        continue;
      }

      if toplevel == "text" {
        edge_list_2.insert((node_text, mimenode));
      } else if toplevel == "inode" {
        edge_list_2.insert((node_allall, mimenode));
      } else {
        edge_list_2.insert((node_octet, mimenode));
      }
    }
    // Don't add duplicate entries
    graph.extend_with_edges(edge_list_2.difference(&edge_list));

    graph
  }
  fn aliases_init() -> FnvHashMap<MIME, MIME> {
    let mut out = FnvHashMap::<MIME, MIME>::default();
    for &c in CHECKERS {
      out.extend(c.get_aliaslist());
    }
    out
  }
  /// Mappings between modules and supported mimes
  fn checker_support_init() -> FnvHashMap<MIME, &'static dyn Checker> {
    let mut out = FnvHashMap::<MIME, &'static dyn Checker>::default();
    for &c in CHECKERS {
      for m in c.get_supported() {
        out.insert(m, c);
      }
    }
    out
  }

  /// Creates a new MimeDatabase from a magic file blob.
  pub fn new(magic_blob: &[u8]) -> MimeDatabase {
    MimeDatabase {
      graph: Self::graph_init(),
      checker_support: Self::checker_support_init(),
      aliases: Self::aliases_init(),
    }
  }

  /// Just the part of from_*_node that walks the graph
  fn typegraph_walker<T, F>(&self, parentnode: NodeIndex, input: &T, matchfn: F) -> Option<MIME>
  where
    T: ?Sized,
    F: Fn(&str, &T) -> bool,
  {
    // Pull most common types towards top
    let mut children: Vec<NodeIndex> = self
      .graph
      .neighbors_directed(parentnode, Outgoing)
      .collect();

    for i in 0..children.len() {
      let x = children[i];
      if TYPEORDER.contains(&self.graph[x]) {
        children.remove(i);
        children.insert(0, x);
      }
    }

    // Walk graph
    for childnode in children {
      let mimetype = &self.graph[childnode];

      let result = matchfn(mimetype, input);
      match result {
        true => match self.typegraph_walker(childnode, input, matchfn) {
          Some(foundtype) => return Some(foundtype),
          None => return Some(mimetype),
        },
        false => continue,
      }
    }

    None
  }

  /// Internal function. Checks if an alias exists, and if it does,
  /// then runs `from_u8`.
  fn match_u8_noalias(&self, mimetype: &str, bytes: &[u8]) -> bool {
    match self.checker_support.get(mimetype) {
      None => false,
      Some(y) => y.from_u8(bytes, mimetype),
    }
  }
  /// Transforms an alias into it's real type
  fn get_alias<'a>(&'a self, mimetype: &'a str) -> &'a str {
    match self.aliases.get(mimetype) {
      Some(x) => x,
      None => mimetype,
    }
  }
  /// Gets the type of a file from a raw bytestream, starting at a certain node
  /// in the type graph.
  ///
  /// Returns MIME as string wrapped in Some if a type matches, or
  /// None if no match is found under the given node.
  /// Retreive the node from the `TYPE.hash` HashMap, using the MIME as the key.
  ///
  /// # Panics
  /// Will panic if the given node is not found in the graph.
  /// As the graph is immutable, this should not happen if the node index comes from
  /// TYPE.hash.
  fn from_u8_node(&self, parentnode: NodeIndex, bytes: &[u8]) -> Option<MIME> {
    self.typegraph_walker(parentnode, bytes, |mimetype, bytes| {
      self.match_u8_noalias(mimetype, bytes)
    })
  }
  /// Checks if the given bytestream matches the given MIME type.
  ///
  /// Returns true or false if it matches or not. If the given MIME type is not known,
  /// the function will always return false.
  /// If mimetype is an alias of a known MIME, the file will be checked agains that MIME.
  ///
  /// # Examples
  /// ```rust
  /// // Load a GIF file
  /// let input: &[u8] = include_bytes!("../tests/image/gif");
  ///
  /// // Check if the MIME and the file are a match
  /// let result = tree_magic_mini::match_u8("image/gif", input);
  /// assert_eq!(result, true);
  /// ```
  pub fn match_u8(&self, mimetype: &str, bytes: &[u8]) -> bool {
    self.match_u8_noalias(self.get_alias(mimetype), bytes)
  }
  /// Gets the type of a file from a byte stream.
  ///
  /// Returns MIME as string.
  ///
  /// # Examples
  /// ```rust
  /// // Load a GIF file
  /// let input: &[u8] = include_bytes!("../tests/image/gif");
  ///
  /// // Find the MIME type of the GIF
  /// let result = tree_magic_mini::from_u8(input);
  /// assert_eq!(result, "image/gif");
  /// ```
  pub fn from_u8(&self, bytes: &[u8]) -> MIME {
    let node = match self.graph.externals(Incoming).next() {
      Some(foundnode) => foundnode,
      None => panic!("No filetype definitions are loaded."),
    };
    self.from_u8_node(node, bytes).unwrap()
  }
  /// Internal function. Checks if an alias exists, and if it does,
  /// then runs `from_filepath`.
  fn match_filepath_noalias(&self, mimetype: &str, filepath: &Path) -> bool {
    match self.checker_support.get(mimetype) {
      None => false,
      Some(c) => c.from_filepath(&filepath, mimetype),
    }
  }
  /// Check if the given filepath matches the given MIME type.
  ///
  /// Returns true or false if it matches or not, or an Error if the file could
  /// not be read. If the given MIME type is not known, it will always return false.
  ///
  /// # Examples
  /// ```rust
  /// use std::path::Path;
  ///
  /// // Get path to a GIF file
  /// let path: &Path = Path::new("tests/image/gif");
  ///
  /// // Check if the MIME and the file are a match
  /// let result = tree_magic_mini::match_filepath("image/gif", path);
  /// assert_eq!(result, true);
  /// ```
  pub fn match_filepath(&self, mimetype: &str, filepath: &Path) -> bool {
    self.match_filepath_noalias(self.get_alias(mimetype), filepath)
  }

  /// Gets the type of a file from a filepath, starting at a certain node
  /// in the type graph.
  ///
  /// Returns MIME as string wrapped in Some if a type matches, or
  /// None if the file is not found or cannot be opened.
  /// Retreive the node from the `TYPE.hash` FnvHashMap, using the MIME as the key.
  ///
  /// # Panics
  /// Will panic if the given node is not found in the graph.
  /// As the graph is immutable, this should not happen if the node index comes from
  /// `TYPE.hash`.
  fn from_filepath_node(&self, parentnode: NodeIndex, filepath: &Path) -> Option<MIME> {
    // We're actually just going to thunk this down to a u8
    // unless we're checking via basetype for speed reasons.

    // Ensure it's at least a application/octet-stream
    if !self.match_filepath("application/octet-stream", filepath) {
      // Check the other base types
      return self.typegraph_walker(parentnode, filepath, |mimetype, bytes| {
        self.match_filepath_noalias(mimetype, bytes)
      });
    }

    // Load the first 2K of file and parse as u8
    // for batch processing like this

    let b = match read_bytes(filepath, 2048) {
      Ok(x) => x,
      Err(_) => return None,
    };

    self.from_u8_node(parentnode, b.as_slice())
  }
  /// Gets the type of a file from a filepath.
  ///
  /// Does not look at file name or extension, just the contents.
  /// Returns MIME as string wrapped in Some if a type matches, or
  /// None if the file is not found or cannot be opened.
  ///
  /// # Examples
  /// ```rust
  /// use std::path::Path;
  ///
  /// // Get path to a GIF file
  /// let path: &Path = Path::new("tests/image/gif");
  ///
  /// // Find the MIME type of the GIF
  /// let result = tree_magic_mini::from_filepath(path);
  /// assert_eq!(result, Some("image/gif"));
  /// ```
  pub fn from_filepath(&self, filepath: &Path) -> Option<MIME> {
    let node = match self.graph.externals(Incoming).next() {
      Some(foundnode) => foundnode,
      None => panic!("No filetype definitions are loaded."),
    };

    self.from_filepath_node(node, filepath)
  }
}

/// Check these types first
/// TODO: Poll these from the checkers? Feels a bit arbitrary
const TYPEORDER: [&str; 6] = [
  "image/png",
  "image/jpeg",
  "image/gif",
  "application/zip",
  "application/x-msdos-executable",
  "application/pdf",
];

pub(crate) trait Checker: Send + Sync {
  fn from_u8(&self, file: &[u8], mimetype: &str) -> bool;
  fn from_filepath(&self, filepath: &Path, mimetype: &str) -> bool;
  fn get_supported(&self) -> Vec<MIME>;
  fn get_subclasses(&self) -> Vec<(MIME, MIME)>;
  fn get_aliaslist(&self) -> FnvHashMap<MIME, MIME>;
}

static CHECKERS: &[&'static dyn Checker] = &[
  &fdo_magic::builtin::check::FdoMagic,
  &basetype::check::BaseType,
];

/// Reads the given number of bytes from a file
pub fn read_bytes(filepath: &Path, bytecount: usize) -> Result<Vec<u8>, std::io::Error> {
  use std::fs::File;
  use std::io::prelude::*;

  let mut b = Vec::<u8>::with_capacity(bytecount);
  let f = File::open(filepath)?;
  f.take(bytecount as u64).read_to_end(&mut b)?;
  Ok(b)
}
