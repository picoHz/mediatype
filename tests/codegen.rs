use std::{fmt::Write, fs};

#[test]
fn generated_code_is_fresh() {
    let mut fresh = generate_consts(
        "Name",
        "src/consts/names.txt",
        "src/consts/names.rs",
        &[
            ("vnd.", "vnd", "Vendor subtypes starting with `vnd.`."),
            ("x-", "x_", "Unregistered subtypes starting with `x-`."),
            ("", "", ""),
        ],
        NAMES_HEADER,
    );

    fresh = fresh
        && generate_consts(
            "Value",
            "src/consts/values.txt",
            "src/consts/values.rs",
            &[("", "", "")],
            VALUES_HEADER,
        );

    if !fresh {
        panic!("generated code is not fresh, please commit updates");
    }
}

fn generate_consts(
    ty: &str,
    input: &str,
    dst: &str,
    prefixes: &[(&str, &str, &str)],
    header: &str,
) -> bool {
    let mut prefixes = prefixes
        .iter()
        .map(|&(pf, name, comment)| (pf, name, comment, String::with_capacity(1024)))
        .collect::<Vec<_>>();

    let input = fs::read_to_string(&input).expect("failed to read input file");
    for line in input.lines() {
        let (ident, name) = if let Some(pair) = line.split_once('=') {
            pair
        } else {
            (line, line)
        };

        if let Some((pf, _, _, out)) = prefixes.iter_mut().find(|(pf, ..)| ident.starts_with(pf)) {
            let ident = upper_snake_case(ident.trim_start_matches(*pf));
            let indent = if pf.is_empty() { "" } else { "    " };
            writeln!(out, "{}/// `{}`", indent, name).unwrap();
            writeln!(
                out,
                "{}pub const {}: crate::{} = crate::{}::new_unchecked(\"{}\");",
                indent, ident, ty, ty, name
            )
            .unwrap();
        }
    }

    let mut new = String::with_capacity(1024);
    new.push_str(header);
    new.push_str("\n\n");
    for (pf, name, comment, out) in prefixes {
        if !pf.is_empty() {
            writeln!(new, "/// {}", comment).unwrap();
            writeln!(new, "pub mod {} {{", name).unwrap();
        }

        new.push_str(&out);

        if !pf.is_empty() {
            new.push_str("}\n\n");
        }
    }

    let old = fs::read_to_string(dst).expect("failed to read destination file");
    if old != new {
        fs::write(dst, &new).expect("failed to write destination file");
    }

    old == new
}

fn upper_snake_case(s: &str) -> String {
    let s = s
        .split_inclusive(char::is_uppercase)
        .map(|chunk| {
            if chunk.ends_with(char::is_uppercase) {
                let prefix = chunk.trim_end_matches(char::is_uppercase);
                if prefix.ends_with(char::is_lowercase) {
                    return format!("{}_{}", prefix, chunk.split_at(chunk.len() - 1).1);
                }
            }
            chunk.to_string()
        })
        .collect::<String>()
        .replace('+', "_plus")
        .replace(|c| !char::is_ascii_alphanumeric(&c), "_")
        .to_ascii_uppercase();

    if s.starts_with(char::is_numeric) {
        format!("_{}", s)
    } else {
        s
    }
}

const NAMES_HEADER: &str = r#"//! Predefined names, @generated in tests/codegen.rs
//!
//! # Sources
//! - <https://en.wikipedia.org/wiki/Media_type>
//! - <https://www.iana.org/assignments/media-types/media-types.xhtml>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_type>
//! - <https://developer.mozilla.org/en-US/docs/Glossary/Quality_values>
//! - <https://datatracker.ietf.org/doc/html/rfc3676>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types>"#;

const VALUES_HEADER: &str = r#"//! Predefined parameter values, @generated in tests/codegen.rs
//!
//! # Sources
//! - <https://www.iana.org/assignments/character-sets/character-sets.xhtml>
//! - <https://datatracker.ietf.org/doc/html/rfc3676>"#;
