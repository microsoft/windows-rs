/// Shared helpers for the code generators.
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Convert `PascalCase` to `snake_case`.
pub fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// Run `rustfmt` on generated Rust code.
pub fn rustfmt(code: &str) -> String {
    let mut child = Command::new("rustfmt")
        .arg("--edition=2024")
        .arg("--config-path")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../rustfmt.toml"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "rustfmt failed");
    String::from_utf8(output.stdout).unwrap()
}
