#![deny(missing_docs)]
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![allow(clippy::doc_markdown)]
/*!
TODO: COPY README
*/

mod basetype;
mod fdo_magic;

mod tree_magic;
pub use tree_magic::MimeDatabase;

use tree_magic::{read_bytes, Checker, MIME};
