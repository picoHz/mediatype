//! Predefined names.
//!
//! # Sources
//! - <https://en.wikipedia.org/wiki/Media_type>
//! - <https://www.iana.org/assignments/media-types/media-types.xhtml>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_type>
//! - <https://developer.mozilla.org/en-US/docs/Glossary/Quality_values>

include!(concat!(env!("OUT_DIR"), "/names_generated.rs"));

/// Vendor subtypes starting with `vnd.`.
pub mod vnd {
    include!(concat!(env!("OUT_DIR"), "/names_generated.vnd.rs"));
}

/// Unregistered subtypes starting with `x-`.
pub mod x_ {
    include!(concat!(env!("OUT_DIR"), "/names_generated.x_.rs"));
}
