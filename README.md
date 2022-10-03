<div align="center">

# MediaType

MIME Media-type parsing for Rust

[![Crates.io](https://img.shields.io/crates/v/mediatype.svg)](https://crates.io/crates/mediatype)
[![dependency status](https://deps.rs/crate/mediatype/0.19.3/status.svg)](https://deps.rs/crate/mediatype/0.19.3)
[![GitHub license](https://img.shields.io/github/license/picoHz/mediatype.svg)](https://github.com/picoHz/mediatype/blob/main/LICENSE)
[![Rustdoc](https://img.shields.io/badge/doc-rustdoc-green.svg)](https://docs.rs/mediatype)
![Rust](https://github.com/picoHz/mediatype/workflows/Rust/badge.svg)

</div>

- [Parsing](#parsing)
- [Construction](#construction)
- [Parameters](#parameters)
  - [Case Sensitivity](#case-sensitivity)
  - [Duplicate Parameter Names](#duplicate-parameter-names)
- [Owned Type](#owned-type)
- [MediaTypeList](#mediatypelist)
- [Serialize and Deserialize](serialize-and-deserialize)

## Parsing

`MediaType::parse` runs a zero-copy parsing: A `MediaType` borrows the input string instead of copying it. 

If you need an owned type, use [`MediaTypeBuf`](#owned-type).

```rust
use mediatype::{names::*, values::*, MediaType};

let madia_type = "image/svg+xml; charset=UTF-8";
let svg = MediaType::parse(madia_type).unwrap();

assert_eq!(svg.ty, IMAGE);
assert_eq!(svg.subty, SVG);
assert_eq!(svg.suffix, Some(XML));
assert_eq!(svg.get_param(CHARSET), Some(UTF_8));
```

## Construction

`MediaType` is const-constructible. It can be defined as a constant. 

Predefind names and values are defined in [`names`](https://docs.rs/mediatype/latest/mediatype/names/index.html) and [`values`](https://docs.rs/mediatype/latest/mediatype/values/index.html) modules.

```rust
use mediatype::{names::*, values::*, media_type, MediaType};

const TEXT_PLAIN: MediaType = MediaType::new(TEXT, PLAIN);

const IMAGE_SVG: MediaType = 
  MediaType::from_parts(TEXT, PLAIN, Some(XML), &[(CHARSET, UTF_8)]);

const TEXT_MARKDOWN: MediaType = 
  media_type!(TEXT/MARKDOWN; CHARSET=UTF_8);
```

## Parameters

### Case Sensitivity

Comparisons are case-insensitive except parameter values.

```rust
let text_plain_lower = MediaType::parse("text/plain; charset=UTF-8").unwrap();
let text_plain_upper = MediaType::parse("TEXT/PLAIN; CHARSET=UTF-8").unwrap();

assert_eq!(text_plain_lower, text_plain_upper);
assert_eq!(text_plain_lower.ty(), "Text");
assert_eq!(text_plain_upper.subty(), "Plain");
assert!(text_plain_lower != 
  MediaType::parse("text/plain; charset=utf-8").unwrap());
```

### Duplicate Parameter Names

The parser does not report duplicate parameter names as an error, but `MediaType` recognizes only the last value.

```rust
let text_plain = MediaType::parse(
  "text/plain; charset=US-ASCII; charset=UTF-8").unwrap();

assert_eq!(
    text_plain.to_string(),
    "text/plain; charset=US-ASCII; charset=UTF-8"
);

// Return the last charset value.
assert_eq!(text_plain.get_param(CHARSET), Some(UTF_8));

// Compare the last charset value.
assert_eq!(
    text_plain,
    MediaType::parse("text/plain; charset=UTF-8").unwrap()
);
```

## Owned Type
 
[`MediaTypeBuf`](https://docs.rs/mediatype/latest/mediatype/struct.MediaTypeBuf.html) is an owned version of `MediaType`.
It is immutable but optimized for minimal stack and heap usage.

```rust
use mediatype::{names::*, values::*, MediaType, MediaTypeBuf};

let text_plain: MediaTypeBuf = "text/plain; charset=UTF-8".parse().unwrap();
assert_eq!(text_plain.get_param(CHARSET).unwrap(), UTF_8);

// Convert to MediaType
let mut text_markdown: MediaType = text_plain.to_ref();
text_markdown.subty = MARKDOWN;
assert_eq!(text_markdown.to_string(), "text/markdown; charset=UTF-8");
```

## MediaTypeList

[`MediaTypeList`](https://docs.rs/mediatype/latest/mediatype/struct.MediaTypeList.html) parses a comma-separated list of `MediaType`s used in the HTTP `Accept` header. ([RFC 7231](https://www.rfc-editor.org/rfc/rfc7231#section-5.3.2))

```rust
use mediatype::{MediaType, MediaTypeList};

let mut list = MediaTypeList::new(
    "text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8",
);
assert_eq!(list.next(), Some(MediaType::parse("text/html")));
assert_eq!(list.next(), Some(MediaType::parse("application/xhtml+xml")));
assert_eq!(list.next(), Some(MediaType::parse("application/xml;q=0.9")));
assert_eq!(list.next(), Some(MediaType::parse("*/*;q=0.8")));
assert_eq!(list.next(), None);
```

## Serialize and Deserialize

To enable serialization and deserialization, specify `serde` feature in `Cargo.toml`.

```toml
mediatype = { version = "...", features = ["serde"] }
```

```rust
let json = r#"
    [
        "text/plain",
        "image/svg+xml; charset=UTF-8"
    ]
"#;

let decoded: Vec<MediaType> = serde_json::from_str(json).unwrap();
```
