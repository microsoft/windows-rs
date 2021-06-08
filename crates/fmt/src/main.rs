use std::ffi::OsStr;
use std::io::Result;
use std::io::Write;
use std::path::*;
use std::process::Command;
use std::process::Stdio;

fn format_file(file: &Path, pattern: &str) -> Result<()> {
    const SHIM: &str = "use format_build_macro::";
    const SHIM_CURLY: &str = "use format_build_macro::{";

    let contents = std::fs::read(file)?;
    let contents = String::from_utf8(contents).expect("Failed to parse UTF-8");

    // Replace macro call with our `use` pattern
    let contents = contents.replace(pattern, SHIM);

    // Spawn `rustfmt` and pipe string through it, instead of writing it to disk
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(contents.as_bytes())?;
    drop(stdin);

    // Read formatted output from `rustfmt`
    let output = child.wait_with_output().expect("Failed to read stdout");
    let contents = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");

    // Some build macros contain a single item, where curly braces are removed
    if contents.contains(SHIM_CURLY) {
        std::fs::write(file, contents.replace(SHIM, pattern))?;
    } else if let Some(macro_start) = contents.find(SHIM) {
        // If curly braces have been removed, find the `use` statement and terminating semicolon
        let usetree_start = macro_start + SHIM.len();
        let semi = contents[usetree_start..].find(';').expect("Semi");

        let indent = contents[..macro_start]
            .chars()
            .rev()
            .take_while(|&c| c == ' ')
            .count();

        let indent = std::iter::repeat(' ').take(indent).collect::<String>();

        // Replace `use` with macro call and insert curly braces around the UseTree
        let macro_ = format!(
            "{}{{\n    {}{},\n{}}}",
            pattern,
            indent,
            contents[usetree_start..usetree_start + semi].replace('\n', "\n    "),
            indent,
        );
        let mut contents = contents;
        contents.replace_range(macro_start..usetree_start + semi, &macro_);
        std::fs::write(file, contents)?;
    }

    Ok(())
}

fn walk_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        for entry in path.read_dir()? {
            walk_path(&entry?.path())?;
        }
    } else if path.file_name() == Some(OsStr::new("build.rs")) {
        format_file(path, "windows::build! ")?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let dir = windows_gen::workspace_dir();
    walk_path(&dir)?;

    format_file(
        &dir.join("crates/bindings/src/main.rs"),
        "let tokens = windows_macros::generate! ",
    )
}
