<div align="center">

# MediaType

MIME Media-type parsing for Rust

[![Crates.io](https://img.shields.io/crates/v/mediatype.svg)](https://crates.io/crates/mediatype)
[![GitHub license](https://img.shields.io/github/license/picoHz/mediatype.svg)](https://github.com/picoHz/mediatype/blob/main/LICENSE)
[![Rustdoc](https://img.shields.io/badge/doc-rustdoc-green.svg)](https://docs.rs/mediatype)
![Rust](https://github.com/picoHz/mediatype/workflows/Rust/badge.svg)

</div>

- [Parsing](#parsing)
- [Construction](#construction)
- [Mutation](#mutation)
- [Parameters](#parameters)
  - [Case Sensitivity](#case-sensitivity)
  - [Duplicate Parameter Names](#duplicate-parameter-names)
  - [Quoted String](#mutation)
- [Owned Type](#owned-type)

## Parsing

`MediaType::parse` runs a zero-copy parsing: A `MediaType` borrows the input string instead of copying it. If you need an owned type, use [`MediaTypeBuf`](#owned-type).

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

`MediaType` is const-construtible. It can be defained as a constant. 
Predefind names and values are defined in [`names`](https://docs.rs/mediatype/latest/mediatype/names/index.html) and [`values`](https://docs.rs/mediatype/latest/mediatype/values/index.html) modules.

```rust
use mediatype::{names::*, values::*, MediaType};

const TEXT_PLAIN: MediaType = MediaType::new(TEXT, PLAIN);
const IMAGE_SVG: MediaType = MediaType::from_parts(TEXT, PLAIN, Some(XML), &[(CHARSET, UTF_8)]);
```

## Mutation

```rust
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
assert!(text_plain_lower != MediaType::parse("text/plain; charset=utf-8").unwrap());
```

### Duplicate Parameter Names

The parser does not report duplicate parameter names as an error, but `MediaType` recognizes only the last value.

```rust
let text_plain = MediaType::parse("text/plain; charset=US-ASCII; charset=UTF-8").unwrap();

assert_eq!(
    text_plain.to_string(),
    "text/plain; charset=US-ASCII; charset=UTF-8"
);

// Returns the last charset value.
assert_eq!(text_plain.get_param(CHARSET), Some(UTF_8));

// Compares the last charset value.
assert_eq!(
    text_plain,
    MediaType::parse("text/plain; charset=UTF-8").unwrap()
);
```

### Quoted String

```rust
let text_plain = MediaType::parse("text/plain; message=\"Hello world!\"").unwrap();
let message = text_plain.get_param(Name::new("message").unwrap()).unwrap();

assert_eq!(message, "Hello world!");
assert_eq!(message.as_str(), "\"Hello world!\"");
assert_eq!(message.unquoted_str(), "Hello world!");
```

```rust
let mut text_plain = MediaType::parse("text/plain").unwrap();

let quoted = Value::quote("Hello world!");
let value = Value::new(&quoted).unwrap();
text_plain.set_param(Name::new("message").unwrap(), value);

assert_eq!(
    text_plain.to_string(),
    "text/plain; message=\"Hello world!\""
);
```

## Owned Type
 
[`MediaTypeBuf`](https://docs.rs/mediatype/latest/mediatype/struct.MediaTypeBuf.html) is an owned and immutable version of `MediaType`.

```rust
use mediatype::{names::*, values::*, MediaType, MediaTypeBuf};

let text_plain: MediaTypeBuf = "text/plain; charset=UTF-8".parse().unwrap();
assert_eq!(text_plain.get_param(CHARSET).unwrap(), UTF_8);

let mut text_markdown: MediaType = text_plain.to_ref();
text_markdown.subty = MARKDOWN;
assert_eq!(text_markdown.to_string(), "text/markdown; charset=UTF-8");
```
