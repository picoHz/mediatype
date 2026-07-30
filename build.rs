use std::{fmt::Write as _, fs, io::Write as _, path::Path};

fn main() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=src/consts/names.txt");
    println!("cargo::rerun-if-changed=src/consts/values.txt");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    generate_consts(
        "Name",
        Path::new("src/consts/names.txt"),
        &out_dir.join("names.rs"),
        &[
            ("vnd.", "vnd", "Vendor subtypes starting with `vnd.`."),
            ("x-", "x_", "Unregistered subtypes starting with `x-`."),
            ("", "", ""),
        ],
    )?;

    generate_consts(
        "Value",
        Path::new("src/consts/values.txt"),
        &out_dir.join("values.rs"),
        &[("", "", "")],
    )?;

    Ok(())
}

fn generate_consts(
    ty: &str,
    input: &Path,
    dst: &Path,
    prefixes: &[(&str, &str, &str)],
) -> std::io::Result<()> {
    let mut prefixes: Vec<_> = prefixes
        .iter()
        .map(|&(pf, name, comment)| (pf, name, comment, String::with_capacity(1024)))
        .collect();

    let input = fs::read_to_string(input).expect("failed to read input file");
    for line in input.lines() {
        let (ident, name) = if let Some(pair) = line.split_once('=') {
            pair
        } else {
            (line, line)
        };

        if let Some((pf, _, _, out)) = prefixes.iter_mut().find(|(pf, ..)| ident.starts_with(pf)) {
            let ident = upper_snake_case(ident.trim_start_matches(*pf));
            let indent = if pf.is_empty() { "" } else { "    " };
            writeln!(out, "{indent}/// `{name}`").unwrap();
            writeln!(
                out,
                "{indent}pub const {ident}: crate::{ty} = crate::{ty}::new_unchecked(\"{name}\");"
            )
            .unwrap();
        }
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dst)?;
    for (pf, name, comment, out) in prefixes {
        if !pf.is_empty() {
            writeln!(file, "/// {comment}")?;
            writeln!(file, "pub mod {name} {{")?;
        }

        write!(file, "{out}")?;

        if !pf.is_empty() {
            writeln!(file, "}}")?;
        }
    }
    Ok(())
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
