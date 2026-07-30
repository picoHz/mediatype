/// # Sources
/// - <https://en.wikipedia.org/wiki/Media_type>
/// - <https://www.iana.org/assignments/media-types/media-types.xhtml>
/// - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_type>
/// - <https://developer.mozilla.org/en-US/docs/Glossary/Quality_values>
/// - <https://datatracker.ietf.org/doc/html/rfc3676>
/// - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types>
pub mod names {
    include!(concat!(env!("OUT_DIR"), "/names.rs"));
}

/// # Sources
/// - <https://www.iana.org/assignments/character-sets/character-sets.xhtml>
/// - <https://datatracker.ietf.org/doc/html/rfc3676>
pub mod values {
    include!(concat!(env!("OUT_DIR"), "/values.rs"));
}
