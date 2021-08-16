fn update_file(path: &std::path::Path, pattern: &str) -> std::io::Result<()> {
    const SHIM: &str = "use format_build_macro::";
    const SHIM_CURLY: &str = "use format_build_macro::{";

    let before = String::from_utf8(std::fs::read(path)?).expect("Failed to parse UTF-8");
    let after = before.replace(pattern, SHIM);

    if before != after {
        std::fs::write(path, after)?;

        let mut cmd = ::std::process::Command::new("rustfmt");
        cmd.arg(&path);
        cmd.output()?;

        let contents = String::from_utf8(std::fs::read(&path)?).expect("Failed to parse UTF-8");

        if contents.contains(SHIM_CURLY) {
            let after = contents.replace(SHIM, pattern);
            std::fs::write(path, after)?;
        } else if let Some(macro_start) = contents.find(SHIM) {
            // If curly braces have been removed, find the `use` statement and terminating semicolon
            let usetree_start = macro_start + SHIM.len();
            let semi = contents[usetree_start..].find(';').expect("Semi");

            let indent = contents[..macro_start]
                .chars()
                .rev()
                .take_while(|&c| c == ' ')
                .count();

            let indent = " ".repeat(indent);

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
            std::fs::write(path, contents)?;
        }
    }

    Ok(())
}

fn update_dir(dir: &std::path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                update_dir(&path)?;
            } else if path.file_name() == Some(std::ffi::OsStr::new("build.rs")) {
                update_file(&path, "windows::build! ")?;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let dir: std::path::PathBuf = std::env::args()
        .nth(1)
        .unwrap_or_else(winmd_reader::workspace_dir)
        .into();

    update_dir(&dir)?;

    let bindings = dir.join("crates/bindings/src/main.rs");

    if bindings.exists() {
        update_file(&bindings, "let tokens = windows_macros::generate! ")?;
    }

    Ok(())
}
