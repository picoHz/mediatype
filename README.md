<div align="center">

# MediaType

MIME Media-type parsing for Rust

[![Crates.io](https://img.shields.io/crates/v/mediatype.svg)](https://crates.io/crates/mediatype)
[![GitHub license](https://img.shields.io/github/license/picoHz/mediatype.svg)](https://github.com/picoHz/mediatype/blob/main/LICENSE)
[![Rustdoc](https://img.shields.io/badge/doc-rustdoc-green.svg)](https://docs.rs/mediatype)
![Rust](https://github.com/picoHz/mediatype/workflows/Rust/badge.svg)

</div>

This crate provides two MediaType structs: 
[`MediaType`](https://docs.rs/mediatype/latest/mediatype/struct.MediaType.html) and 
[`MediaTypeBuf`](https://docs.rs/mediatype/latest/mediatype/struct.MediaTypeBuf.html).

## [`MediaType`](https://docs.rs/mediatype/latest/mediatype/struct.MediaType.html) 

This does not copy data during parsing and borrows the original string. 
It is also const-constructible.

```rust
use mediatype::{names::*, MediaType, Value};

let mut multipart = MediaType::new(MULTIPART, FORM_DATA);

let boundary = Value::new("dyEV84n7XNJ").unwrap();
multipart.set_param(BOUNDARY, boundary);
assert_eq!(
     multipart.to_string(),
     "multipart/form-data; boundary=dyEV84n7XNJ"
);

multipart.subty = RELATED;
assert_eq!(
    multipart.to_string(),
    "multipart/related; boundary=dyEV84n7XNJ"
);

const IMAGE_SVG: MediaType = MediaType::from_parts(IMAGE, SVG, Some(XML), None);
let svg = MediaType::parse("IMAGE/SVG+XML").unwrap();
assert_eq!(svg, IMAGE_SVG);
```

## [`MediaTypeBuf`](https://docs.rs/mediatype/latest/mediatype/struct.MediaTypeBuf.html) 

This is an owned and immutable version of `MediaType`.

```rust
use mediatype::{names::*, values::*, MediaType, MediaTypeBuf};

let text_plain: MediaTypeBuf = "text/plain; charset=UTF-8".parse().unwrap();
assert_eq!(text_plain.get_param(CHARSET).unwrap(), UTF_8);

let mut text_plain: MediaType = text_plain.to_ref();
text_plain.subty = MARKDOWN;
assert_eq!(text_plain.to_string(), "text/markdown; charset=UTF-8");
```
