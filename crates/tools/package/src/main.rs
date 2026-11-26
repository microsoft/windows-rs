/// Generates the published `windows` and `windows-sys` package crates from the pinned metadata in
/// `crates/tools/package/compat`.
///
/// This is separated from `tool-bindings` because package generation uses `--package` mode
/// (per-namespace files + Cargo.toml feature gates) which is an internal bindgen feature not
/// intended for external use.
fn main() {
    let time = std::time::Instant::now();
    windows_bindgen::bindgen(["--etc", "crates/tools/package/src/sys.txt"]);
    windows_bindgen::bindgen(["--etc", "crates/tools/package/src/windows.txt"]);
    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
