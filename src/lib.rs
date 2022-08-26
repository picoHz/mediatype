//! This crate provides two media type structs: [`MediaType`] and [`MediaTypeBuf`].
//!
//! - [`MediaType`] does not copy data during parsing
//!     and borrows the original string. It is also const-constructible.
//! - [`MediaTypeBuf`] is an owned  and immutable version of [`MediaType`].
//!
//! [`MadiaType`]: ./struct.MediaType.html
//! [`MediaTypeBuf`]: ./struct.MediaTypeBuf.html
//!
//!  ```
//! use mediatype::{names::*, MediaType, MediaTypeBuf};
//!
//! const TEXT_PLAIN: MediaType = MediaType::new(TEXT, PLAIN);
//! let text_plain: MediaTypeBuf = "text/plain".parse().unwrap();
//!
//! assert_eq!(text_plain, TEXT_PLAIN);
//! ```
//!
//! # Letter case handling
//!
//! [`MediaType`] and [`MediaTypeBuf`] preserve the original string's letter case.
//! Comparisons for [`MediaType`], [`MediaTypeBuf`] and [`Name`] are case-insensitive
//! except parameter values.
//!
//! [`MadiaType`]: ./struct.MediaType.html
//! [`MediaTypeBuf`]: ./struct.MediaTypeBuf.html
//! [`Name`]: ./struct.Name.html
//!
//!  ```
//! # use mediatype::{names::*, MediaType, MediaTypeBuf};
//! let lower: MediaTypeBuf = "text/plain; charset=UTF-8".parse().unwrap();
//! let upper: MediaTypeBuf = "TEXT/PLAIN; CHARSET=UTF-8".parse().unwrap();
//!
//! assert_eq!(lower.as_str(), "text/plain; charset=UTF-8");
//! assert_eq!(upper.as_str(), "TEXT/PLAIN; CHARSET=UTF-8");
//! assert_eq!(lower, upper);
//! assert_eq!(lower.ty(), "Text");
//! assert_eq!(upper.subty(), "Plain");
//! ```

#![forbid(unsafe_code)]
#![forbid(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod consts;
mod error;
mod media_type;
mod media_type_buf;
mod media_type_list;
mod name;
mod params;
mod parse;
mod serde;
mod value;

pub use consts::*;
pub use error::*;
pub use media_type::*;
pub use media_type_buf::*;
pub use media_type_list::*;
pub use name::*;
pub use params::*;
pub use value::*;

/// Convenient macro to construct a [`MediaType`].
///
/// [`MadiaType`]: ./struct.MediaType.html
///
///
/// ```
/// # use mediatype::media_type;
/// assert_eq!(media_type!(TEXT/PLAIN).to_string(), "text/plain");
/// assert_eq!(media_type!(IMAGE/x_::ICON).to_string(), "image/x-icon");
///
/// assert_eq!(
///     media_type!(IMAGE/SVG+XML; CHARSET=UTF_8).to_string(),
///     "image/svg+xml; charset=UTF-8"
/// );
/// assert_eq!(
///     media_type!(APPLICATION/vnd::OPENSTREETMAP_DATA+XML).to_string(),
///     "application/vnd.openstreetmap.data+xml"
/// );
/// ```
#[macro_export]
macro_rules! media_type {
    ($ty:ident / $prefix:ident $(:: $subty:ident)* $(;$name:ident = $value:ident)*) => {
        $crate::MediaType::from_parts(
            $crate::names::$ty,
            $crate::names::$prefix $(::$subty)*,
            None,
            &[$(($crate::names::$name, $crate::values::$value)),*],
        )
    };
    ($ty:ident / $prefix:ident $(:: $subty:ident)* + $suffix:ident $(;$name:ident = $value:ident)*) => {
        $crate::MediaType::from_parts(
            $crate::names::$ty,
            $crate::names::$prefix $(::$subty)*,
            Some($crate::names::$suffix),
            &[$(($crate::names::$name, $crate::values::$value)),*],
        )
    };
}
