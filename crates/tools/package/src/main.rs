/// Generates the published `windows` and `windows-sys` package crates.
///
/// This is separated from `tool_bindings` because package generation uses
/// `--package` mode (per-namespace files + Cargo.toml feature gates) which
/// is an internal bindgen feature not intended for external use.
fn main() {
    let time = std::time::Instant::now();

    // The `windows-sys` crate (sys-style package).
    windows_bindgen::bindgen(["--etc", "crates/tools/package/src/sys.txt"]);

    // The `windows` crate (full-fidelity package).
    windows_bindgen::bindgen(["--etc", "crates/tools/package/src/windows.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
