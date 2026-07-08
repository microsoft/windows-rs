use windows_clang::nuget_package;
use windows_metadata::merge;

/// The committed, canonical WinRT winmd. Checked in as `windows-bindgen`'s default WinRT
/// metadata (selected with `--in default`), so downstream filters resolve against a stable
/// in-repo winmd without re-running this tool. Re-derived from the pinned SDK Contracts
/// package on every run; treat it as generated output.
const WINMD: &str = "crates/libs/bindgen/default/Windows.winmd";

/// The NuGet package that ships the per-contract WinRT `.winmd` files. Fetched into the
/// NuGet global cache on first use, exactly like `tool_win32`'s SDK packages.
const CONTRACTS_ID: &str = "microsoft.windows.sdk.contracts";

/// Pinned SDK Contracts version. The winmd is merged from this exact package so the WinRT
/// surface is reproducible on any machine and in CI. Bumping it is a deliberate, reviewable
/// change: the new package is restored and `Windows.winmd` regenerated.
const CONTRACTS_VERSION: &str = "10.0.28000.2270";

/// The per-contract winmds live under this subtree in the package (`Windows.Foundation.winmd`,
/// `Windows.Globalization.winmd`, …), one file per API contract.
const CONTRACTS_SUBDIR: &str = "ref/netstandard2.0";

fn main() {
    let time = std::time::Instant::now();

    // Fetch the pinned Contracts package on demand (shared NuGet cache with the other tools)
    // so a fresh checkout regenerates without a manual `nuget restore`.
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

    // Merge the per-contract winmds into one, replacing the external `mdmerge` tool with
    // `windows-metadata`'s in-house merger (the same one `tool_win32`/`tool_wdk` use). The
    // per-contract WinRT runtime-class methods and Property/Event tables are dropped by the
    // merge; `windows-bindgen` reconstructs properties/events from the surviving `get_`/
    // `put_`/`add_`/`remove_` accessor methods on the interfaces and never reads class methods.
    merge()
        .inputs(&inputs)
        .output(WINMD)
        .merge()
        .unwrap_or_else(|e| panic!("failed to merge WinRT contracts into `{WINMD}`: {e}"));

    println!(
        "generated `{WINMD}` from {} contract winmd(s) in {:?}",
        inputs.len(),
        time.elapsed()
    );
}
