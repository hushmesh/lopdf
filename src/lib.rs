#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", doc = include_str!("../README.md"))]
#![forbid(unsafe_code)]
#![deny(clippy::all)]

#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error!("feature \"alloc\" or feature \"std\" must be enabled");

#[cfg(feature = "std")]
include!("./with_std.rs");

#[cfg(not(feature = "std"))]
include!("./without_std.rs");

mod stdlib {
    #[cfg(feature = "std")]
    pub use crate::with_std::*;
    #[cfg(not(feature = "std"))]
    pub use crate::without_std::*;
}

#[macro_use]
mod object;
mod datetime;
pub use crate::object::{Dictionary, Object, ObjectId, Stream, StringFormat};

mod document;
mod incremental_document;
mod object_stream;
#[cfg(any(feature = "pom_parser", feature = "nom_parser"))]
pub use object_stream::ObjectStream;
pub mod xref;
pub use crate::document::Document;
pub use crate::incremental_document::IncrementalDocument;

mod bookmarks;
pub use crate::bookmarks::Bookmark;
mod outlines;
pub use crate::outlines::Outline;
mod destinations;
pub use crate::destinations::Destination;
mod toc;
pub use crate::toc::Toc;
pub mod content;
mod creator;
mod encodings;
pub mod encryption;
mod error;
pub use error::XrefError;
pub mod filters;
#[cfg(not(feature = "nom_parser"))]
#[cfg(feature = "pom_parser")]
mod parser;
#[cfg(feature = "nom_parser")]
#[path = "nom_parser.rs"]
mod parser;
mod parser_aux;
mod processor;
#[cfg(any(feature = "pom_parser", feature = "nom_parser"))]
mod reader;
#[cfg(any(feature = "pom_parser", feature = "nom_parser"))]
pub use reader::Reader;
mod rc4;
mod writer;
pub mod xobject;
pub use error::{Error, Result};
