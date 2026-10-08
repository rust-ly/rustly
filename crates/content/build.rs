//! Embeds every file in the repo's `content/` folder into the crate as
//! `FILES: &[(path, text)]`, with paths relative to that folder.

use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../content");
    println!("cargo::rerun-if-changed={}", root.display());

    let mut paths = Vec::new();
    walk(&root, &mut paths);
    paths.sort();

    let mut out = String::from("pub(crate) static FILES: &[(&str, &str)] = &[\n");
    for path in paths {
        let relative = path.strip_prefix(&root).unwrap();
        let relative = relative.to_str().unwrap().replace('\\', "/");
        let text = fs::read_to_string(&path).unwrap();
        writeln!(out, "    ({relative:?}, {text:?}),").unwrap();
    }
    out.push_str("];\n");
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("files.rs"),
        out,
    )
    .unwrap();
}

fn walk(dir: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let hidden = path.file_name().unwrap().to_string_lossy().starts_with('.');
        if hidden {
            continue;
        } else if path.is_dir() {
            walk(&path, paths);
        } else {
            paths.push(path);
        }
    }
}
