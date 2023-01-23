use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result, Write},
    path::Path,
};

fn main() -> Result<()> {
    generate_consts(
        "Name",
        "src/consts/names.txt",
        &[
            ("x-", "names_generated.x_.rs"),
            ("vnd.", "names_generated.vnd.rs"),
            ("", "names_generated.rs"),
        ],
    )?;

    generate_consts(
        "Value",
        "src/consts/values.txt",
        &[("", "values_generated.rs")],
    )?;

    Ok(())
}

fn generate_consts(ty: &str, input: &str, prefixes: &[(&str, &str)]) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut prefixes = prefixes
        .iter()
        .map(|(pf, file)| {
            let dest_path = Path::new(&out_dir).join(file);
            let out = File::create(dest_path).unwrap();
            (pf, out)
        })
        .collect::<Vec<_>>();

    let file = File::open(input)?;
    for line in BufReader::new(file).lines().flatten() {
        let (ident, name) = if let Some(pair) = line.split_once('=') {
            pair
        } else {
            (line.as_str(), line.as_str())
        };

        if let Some((&pf, out)) = prefixes.iter_mut().find(|(&pf, _)| ident.starts_with(pf)) {
            let ident = upper_snake_case(ident.trim_start_matches(pf));
            writeln!(out, "/// `{}`", name)?;
            writeln!(
                out,
                "pub const {}: crate::{} = crate::{}::new_unchecked(\"{}\");",
                ident, ty, ty, name
            )?;
        }
    }

    println!("cargo:rerun-if-changed={}", input);

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
