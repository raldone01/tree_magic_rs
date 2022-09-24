# tree_magic_rs

[![Rust-Main-CI](https://github.com/raldone01/tree_magic_rs/actions/workflows/rust_main.yml/badge.svg)](https://github.com/raldone01/tree_magic_rs/actions/workflows/rust_main.yml)
[![docs.rs](https://docs.rs/tree_magic_rs/badge.svg)](https://docs.rs/tree_magic_rs)
[![crates.io](https://img.shields.io/crates/v/tree_magic_rs.svg)](https://crates.io/crates/tree_magic_rs)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes almost straight from the crate docs from the source. Double check the doc tests. -->

## What can this crate do?

`tree_magic_rs` is a Rust crate that determines the MIME type a given file
or byte stream.

# Example
```rust
// Load a GIF file
let input: &[u8] = include_bytes!("../tests/image/gif");
// Find the MIME type of the GIF
let result = tree_magic_mini::from_u8(input);
assert_eq!(result, "image/gif");
// Check if the MIME and the file are a match
let result = tree_magic_mini::match_u8("image/gif", input);
assert_eq!(result, true);
```

## The MIME database

`tree_magic_rs` can optionally attempt to load the shared MIME info
database from the standard locations at runtime.

**Warning the magic database files themselves are licensed under the GPL so you can not embed them into your binary if you are not using GPL.**

## Development history

This is a fork of the [tree_magic_mini](https://crates.io/crates/tree_magic_mini)
crate. It includes the following changes:

* Removed option to embed mime database.
* Added tools to create non gpl database.
* Enabled loading of a custom database file.

### Architecture

`tree_magic` is split up into different "checker" modules. Each checker handles a certain set of filetypes, and only those. For instance, the `basetype` checker handles the `inode/*` and `text/plain` types, while the `fdo_magic` checker handles anything with a magic number. Th idea here is that instead of following the `libmagic` route of having one magic descriptor format that fits every file, we can specialize and choose the checker that suits the file format best.

During library initialization, each checker is queried for the types is supports and the parent->child relations between them. During this time, the checkers can load any rules, schemas, etc. into memory. A big philosophy here is that **time during the checking phase is many times more valuable than during the init phase**. The library only gets initialized once, and the library can check thousands of files during a program's lifetime.

From the list of file types and relations, a directed graph is built, and each node is added to a hash map. The library user can use these directly to find parents, children, etc. of a given MIME if needed.

When a file needs to be checked against a certain MIME (match_*), each checker is queried to see if it supports that type, and if so, it runs the checker. If the checker returns true, it must be that type.

When a file needs it's MIME type found (from_*), the library starts at the `all/all` node of the type graph (or whichever node the user specifies) and walks down the tree. If a match is found, it continues searching down that branch. If no match is found, it retrieves the deepest MIME type found.

## License

[MIT License](https://github.com/raldone01/tree_magic_rs/blob/main/LICENSE)