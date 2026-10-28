//! Refresh WinRT metadata from the pinned SDK Contracts NuGet package.
//!
//! Change CONTRACTS_VERSION and run `cargo run -p tool-winrt` from the repo root. Review the
//! `metadata/winrt` diff, then run the downstream generators listed in `.github/workflows/gen.yml`.
//! This pin is independent of tool-win32's SDK_VERSION.

use helpers::nuget_package;
use windows_metadata::merge;

// Compiled from the committed RDL snapshot and embedded by windows-default.
const WINMD: &str = "crates/libs/default/Windows.winmd";
const RDL_DIR: &str = "metadata/winrt";
const OUT_DIR: &str = "target/winrt";
const CONTRACTS_ID: &str = "microsoft.windows.sdk.contracts";
const CONTRACTS_VERSION: &str = "10.0.28000.2270";
const CONTRACTS_SUBDIR: &str = "ref/netstandard2.0";

fn main() {
    let time = std::time::Instant::now();

    let contracts_dir = nuget_package(CONTRACTS_ID, CONTRACTS_VERSION).join(CONTRACTS_SUBDIR);

    let mut inputs: Vec<String> = std::fs::read_dir(&contracts_dir)
        .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", contracts_dir.display()))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|x| x == "winmd"))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect();
    inputs.sort();

    assert!(
        !inputs.is_empty(),
        "no `.winmd` files found under `{}`",
        contracts_dir.display()
    );

    // The merge drops runtime-class methods and Property/Event tables; bindgen uses interface
    // accessors to reconstruct properties and events.
    std::fs::create_dir_all(OUT_DIR)
        .unwrap_or_else(|e| panic!("failed to create `{OUT_DIR}`: {e}"));
    let merged = format!("{OUT_DIR}/Windows.merged.winmd");
    merge()
        .inputs(&inputs)
        .output(&merged)
        .merge()
        .unwrap_or_else(|e| panic!("failed to merge WinRT contracts into `{merged}`: {e}"));

    // Round-trip through committed RDL so metadata changes have a readable diff.
    // Exclude Win32 namespaces from the WinRT snapshot.
    windows_rdl::writer()
        .input(&merged)
        .filters(["Windows", "!Windows.Win32"])
        .split()
        .output(RDL_DIR)
        .write()
        .unwrap_or_else(|e| panic!("failed to write WinRT RDL to `{RDL_DIR}`: {e}"));

    windows_rdl::reader()
        .input(RDL_DIR)
        .output(WINMD)
        .write()
        .unwrap_or_else(|e| panic!("failed to compile `{WINMD}` from `{RDL_DIR}`: {e}"));

    println!(
        "generated `{RDL_DIR}` and `{WINMD}` from {} contract winmd(s) in {:.2}s",
        inputs.len(),
        time.elapsed().as_secs_f32()
    );
}
