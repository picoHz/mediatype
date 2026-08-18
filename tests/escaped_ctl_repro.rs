//! Regression test for issue #30: an *escaped* CR/LF inside a quoted parameter
//! value must be rejected, not just a raw CR/LF.
//!
//! RFC 7230 §3.2.6: CR and LF are neither `qdtext` nor a valid `quoted-pair`, so
//! a backslash in front of them does not make them legal. Before the fix the
//! `_ if escaped` catch-all in `parse_quoted_value` preceded the CR/LF rejection
//! arm, so a `\` immediately before a CR/LF caused the control character to be
//! consumed as a quoted-pair and round-tripped verbatim through `Display` — a
//! CRLF-injection vector. PR #28 added the CR/LF arm but only exercised the raw
//! form, so the escaped form regressed silently.

use mediatype::{MediaType, MediaTypeBuf, MediaTypeError};

// `text/plain; x="a\<LF>Injected: yes"` — a real backslash followed by a real
// LF inside the quoted value. `\\` is a literal backslash, `\n` is a literal LF.
const ESCAPED_LF: &str = "text/plain; x=\"a\\\nInjected: yes\"";
// The escaped-CR variant: a real backslash followed by a real CR.
const ESCAPED_CR: &str = "text/plain; x=\"a\\\rInjected: yes\"";
// A legitimate escaped-quote quoted value: `text/plain; x="a\"b"`.
const ESCAPED_QUOTE: &str = "text/plain; x=\"a\\\"b\"";

#[test]
fn media_type_parse_rejects_escaped_lf() {
    assert!(matches!(
        MediaType::parse(ESCAPED_LF),
        Err(MediaTypeError::InvalidParamValue)
    ));
}

#[test]
fn media_type_parse_rejects_escaped_cr() {
    assert!(matches!(
        MediaType::parse(ESCAPED_CR),
        Err(MediaTypeError::InvalidParamValue)
    ));
}

#[test]
fn media_type_buf_rejects_escaped_lf() {
    assert!(matches!(
        MediaTypeBuf::from_string(ESCAPED_LF.to_string()),
        Err(MediaTypeError::InvalidParamValue)
    ));
    assert!(matches!(
        ESCAPED_LF.parse::<MediaTypeBuf>(),
        Err(MediaTypeError::InvalidParamValue)
    ));
}

#[test]
fn media_type_buf_rejects_escaped_cr() {
    assert!(matches!(
        MediaTypeBuf::from_string(ESCAPED_CR.to_string()),
        Err(MediaTypeError::InvalidParamValue)
    ));
    assert!(matches!(
        ESCAPED_CR.parse::<MediaTypeBuf>(),
        Err(MediaTypeError::InvalidParamValue)
    ));
}

#[test]
fn escaped_quote_still_parses() {
    // The fix must not regress legitimate quoted-pairs: `\"` is a valid escaped
    // quote and the value must still parse through both APIs.
    assert!(MediaType::parse(ESCAPED_QUOTE).is_ok());
    assert!(MediaTypeBuf::from_string(ESCAPED_QUOTE.to_string()).is_ok());
    assert!(ESCAPED_QUOTE.parse::<MediaTypeBuf>().is_ok());
}
