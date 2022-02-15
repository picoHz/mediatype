<div align="center">

# MediaType

MediaType for Rust

[![Crates.io](https://img.shields.io/crates/v/mediatype.svg)](https://crates.io/crates/mediatype)
[![GitHub license](https://img.shields.io/github/license/picoHz/mediatype.svg)](https://github.com/picoHz/mediatype/blob/master/LICENSE)
[![Rustdoc](https://img.shields.io/badge/doc-rustdoc-green.svg)](https://docs.rs/mediatype)

</div>

```rust
use mediatype::{consts::*, MediaType, MediaTypeBuf};

const TEXT_PLAIN: MediaType = MediaType::new(TEXT, PLAIN);
let text_plain: MediaTypeBuf = "text/plain".parse().unwrap();

assert_eq!(text_plain, TEXT_PLAIN);

match (text_plain.ty(), text_plain.subty()) {
    ("text", "plain") => println!("plain text!"),
    ("text", _) => println!("structured text"),
    _ => println!("not text"),
}
```
