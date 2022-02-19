//! This crate provides two MediaType structs: [`MediaType`] and [`MediaTypeBuf`].
//!
//! - [`MediaType`] does not copy data during parsing
//!     and just holds reference to the original string. It is also const-constructible.
//! - [`MediaTypeBuf`] is an owned version of [`MediaType`].
//!
//! [`MadiaType`]: ./struct.MediaType.html
//! [`MediaTypeBuf`]: ./struct.MediaTypeBuf.html
//!
//!  ```
//! use mediatype::{consts::*, MediaType, MediaTypeBuf};
//!
//! const TEXT_PLAIN: MediaType = MediaType::new(TEXT, PLAIN);
//! let text_plain: MediaTypeBuf = "text/plain".parse().unwrap();
//!
//! assert_eq!(text_plain, TEXT_PLAIN);
//!
//! match (text_plain.ty(), text_plain.subty()) {
//!     ("text", "plain") => println!("plain text!"),
//!     ("text", _) => println!("structured text"),
//!     _ => println!("not text"),
//! }
//! ```

#![forbid(unsafe_code)]
#![forbid(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod consts;
mod media_type;
mod media_type_buf;
mod name;
mod params;
mod parse;
mod serde;

pub use media_type::*;
pub use media_type_buf::*;
pub use name::*;
pub use params::*;
pub use parse::*;
