//! Predefined names.
//!
//! # Sources
//! - <https://en.wikipedia.org/wiki/Media_type>
//! - <https://www.iana.org/assignments/media-types/media-types.xhtml>

include!(concat!(env!("OUT_DIR"), "/names_generated.rs"));

/// Vendor subtypes starting with `vnd.`.
pub mod vnd {
    include!(concat!(env!("OUT_DIR"), "/names_generated.vnd.rs"));
}

/// Unregistered subtypes starting with `x-`.
pub mod x_ {
    include!(concat!(env!("OUT_DIR"), "/names_generated.x_.rs"));
}
