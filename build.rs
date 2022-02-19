use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

const PREFIXES: &[(&str, &str)] = &[
    ("x-", "names_generated.x_.rs"),
    ("vnd.", "names_generated.vnd.rs"),
    ("", "names_generated.rs"),
];

const INPUT: &str = "src/consts/names.txt";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut prefixes = PREFIXES
        .iter()
        .map(|(pf, file)| {
            let dest_path = Path::new(&out_dir).join(file);
            let out = File::create(dest_path).unwrap();
            (pf, out)
        })
        .collect::<Vec<_>>();

    let file = File::open(INPUT).unwrap();
    for line in BufReader::new(file).lines().flatten() {
        let (ident, name) = if let Some(pair) = line.split_once('=') {
            pair
        } else {
            (line.as_str(), line.as_str())
        };

        if let Some((&pf, out)) = prefixes.iter_mut().find(|(&pf, _)| ident.starts_with(pf)) {
            let ident = ident
                .trim_start_matches(pf)
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
                .replace(|c| !char::is_alphanumeric(c), "_")
                .to_ascii_uppercase();
            let ident = if ident.starts_with(char::is_numeric) {
                format!("_{}", ident)
            } else {
                ident
            };
            writeln!(out, "/// `{}`", name).unwrap();
            writeln!(
                out,
                "pub const {}: crate::Name = crate::Name(\"{}\");",
                ident, name
            )
            .unwrap();
        }
    }

    println!("cargo:rerun-if-changed={}", INPUT);
}
