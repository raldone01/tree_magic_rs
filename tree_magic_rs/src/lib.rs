#![deny(missing_docs)]
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![allow(clippy::doc_markdown)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]
#![feature(alloc_layout_extra)]
#![feature(ptr_as_uninit)]
#![feature(pointer_byte_offsets)]
/*!
TODO: COPY README
*/

mod basetype;
mod fdo_magic;

pub mod db;

mod tree_magic;
pub use tree_magic::MimeDatabase;

use tree_magic::{read_bytes, Checker, MIME};
