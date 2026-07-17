use std::collections::HashMap;
use std::time::Instant;
use windows_rdl::{item_names, write_to_file, writer};

// The committed winmds and their RDL corpora. `tool_roundtrip` re-derives each RDL corpus
// from its committed winmd and relies on `git diff` to catch any drift, without re-running the
// expensive SDK scrape/merge (it needs neither libclang nor NuGet). Paths are relative to the
// workspace root, matching `tool_win32`/`tool_wdk`/`tool_winrt`.

const WINRT_WINMD: &str = "crates/libs/bindgen/default/Windows.winmd";
const WINRT_RDL: &str = "metadata/winrt";

const WIN32_WINMD: &str = "crates/libs/bindgen/default/Windows.Win32.winmd";
const WIN32_RDL: &str = "metadata/win32";
const WIN32_SEED: &str = "metadata.rdl";

const WDK_WINMD: &str = "crates/libs/bindgen/default/Windows.Wdk.winmd";
const WDK_RDL: &str = "metadata/wdk";

// The flat namespace the header-scraped Win32 and WDK surfaces share.
const WIN32_ROOT: &str = "Windows.Win32";

fn main() {
    let time = Instant::now();

    winrt();
    partitioned("Win32", WIN32_WINMD, WIN32_RDL, Some(WIN32_SEED));
    partitioned("WDK", WDK_WINMD, WDK_RDL, None);

    println!(
        "roundtripped WinRT, Win32 and WDK metadata in {:.2}s",
        time.elapsed().as_secs_f32()
    );
}

/// WinRT round-trips cleanly: the RDL partition key is the namespace, which the winmd carries,
/// so the corpus is reconstructed from `Windows.winmd` alone (one `<namespace>.rdl` per
/// namespace). The `Windows.Win32`/`Windows.Wdk` exclusions guard against any stray references
/// the merged WinRT winmd might carry.
fn winrt() {
    std::fs::create_dir_all(WINRT_RDL)
        .unwrap_or_else(|e| panic!("failed to create `{WINRT_RDL}`: {e}"));

    writer()
        .input(WINRT_WINMD)
        .filters(["Windows", "!Windows.Win32", "!Windows.Wdk"])
        .split(true)
        .output(WINRT_RDL)
        .write()
        .unwrap_or_else(|e| panic!("WinRT roundtrip failed: {e}"));
}

/// Win32 and WDK partition their RDL by *defining header*, which the flat winmd does not carry.
/// The committed corpus is the authority for that layout, so the item-name -> header-stem map is
/// recovered from it (exactly as `merge_arch_rdl` does), the winmd is decompiled through that
/// map, and the hand-authored seed (if any) is restored verbatim afterwards (the partition writer
/// clears every `*.rdl`, seed included). This validates that the winmd's *content* round-trips to
/// the committed RDL under the committed layout.
fn partitioned(label: &str, winmd: &str, rdl_dir: &str, seed: Option<&str>) {
    let seed_text = seed.map(|name| {
        let path = format!("{rdl_dir}/{name}");
        std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read seed `{path}`: {e}"))
    });

    // Build the item-name -> header-stem map from the committed corpus. Files are sorted for a
    // deterministic map, and the seed is skipped so its attribute-definition types stay in the
    // seed file rather than being scattered into a header partition.
    let mut rdl_paths: Vec<_> = std::fs::read_dir(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to read `{rdl_dir}`: {e}"))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rdl"))
        .filter(|path| path.file_name().and_then(|n| n.to_str()) != seed)
        .collect();
    rdl_paths.sort();

    let mut map = HashMap::<String, String>::new();
    for path in &rdl_paths {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| panic!("invalid rdl file name `{}`", path.display()))
            .to_string();
        let path_str = path.to_string_lossy().replace('\\', "/");
        for name in item_names(&path_str, WIN32_ROOT)
            .unwrap_or_else(|e| panic!("failed to read item names from `{path_str}`: {e}"))
        {
            map.entry(name).or_insert_with(|| stem.clone());
        }
    }

    writer()
        .input(winmd)
        .partition(map)
        .output(rdl_dir)
        .write()
        .unwrap_or_else(|e| panic!("{label} roundtrip failed: {e}"));

    if let (Some(name), Some(text)) = (seed, seed_text) {
        write_to_file(&format!("{rdl_dir}/{name}"), text)
            .unwrap_or_else(|e| panic!("failed to restore seed `{rdl_dir}/{name}`: {e}"));
    }
}
