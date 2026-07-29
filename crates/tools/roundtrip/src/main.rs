use std::collections::HashMap;
use std::time::Instant;
use windows_rdl::{item_names, reader, write_to_file, writer};

// The committed RDL is the source of truth. `tool_roundtrip` re-derives each winmd from its
// committed RDL, decompiles it back, and relies on `git diff` to catch any drift, without
// re-running the expensive SDK scrape/merge (it needs neither libclang nor NuGet). The single
// committed `Windows.Win32.winmd` is `tool_win32`'s phase-C merge of the um and km surfaces (with
// same-named enums unioned) - a lossy transform that cannot be decompiled back to the split
// Win32 and WDK RDL - so the round-trip works from that split RDL, compiling each winmd on demand.
// Paths are relative to the workspace root, matching `tool_win32`/`tool_winrt`.

const WINRT_WINMD: &str = "crates/libs/bindgen/default/Windows.winmd";
const WINRT_RDL: &str = "metadata/winrt";

const WIN32_RDL: &str = "metadata/win32";
const WIN32_SEED: &str = "metadata/metadata.rdl";
const WIN32_UM_WINMD: &str = "target/roundtrip/Windows.Win32.um.winmd";

const WDK_RDL: &str = "metadata/wdk";
const WDK_KM_WINMD: &str = "target/roundtrip/Windows.Win32.km.winmd";

// The WinRT projection winmd, a resolution reference so the um compile resolves `ABI::Windows::*`.
const WINRT_RESOLUTION: &str = "crates/libs/bindgen/default/Windows.winmd";

// The flat namespace the header-scraped Win32 and WDK surfaces share.
const WIN32_ROOT: &str = "Windows.Win32";

fn main() {
    let time = Instant::now();

    std::fs::create_dir_all("target/roundtrip")
        .unwrap_or_else(|e| panic!("failed to create `target/roundtrip`: {e}"));

    winrt();

    // Win32: compile the committed RDL (+ seed + WinRT resolution) to a um winmd, then
    // decompile it back under the committed header layout.
    compile(
        &[WIN32_RDL, WIN32_SEED, WINRT_RESOLUTION],
        WIN32_UM_WINMD,
        "Win32",
    );
    partitioned("Win32", WIN32_UM_WINMD, WIN32_RDL, Some(WIN32_SEED));

    // WDK: compile the committed km RDL against the um winmd (its Win32 dependencies resolve
    // there), then decompile it back. Only WDK-defined types are emitted, so the round-trip
    // reproduces `metadata/wdk`.
    compile(&[WDK_RDL, WIN32_UM_WINMD], WDK_KM_WINMD, "WDK");
    partitioned("WDK", WDK_KM_WINMD, WDK_RDL, None);

    println!(
        "roundtripped WinRT, Win32 and WDK metadata in {:.2}s",
        time.elapsed().as_secs_f32()
    );
}

/// Compiles RDL and winmd references into a single winmd.
fn compile(inputs: &[&str], output: &str, label: &str) {
    reader()
        .inputs(inputs)
        .output(output)
        .write()
        .unwrap_or_else(|e| panic!("{label} winmd compile failed: {e}"));
}

/// WinRT round-trips cleanly: the RDL partition key is the namespace, which the winmd carries,
/// so the RDL is reconstructed from `Windows.winmd` alone (one `<namespace>.rdl` per
/// namespace). The `Windows.Win32` exclusion guards against any stray references the merged
/// WinRT winmd might carry.
fn winrt() {
    std::fs::create_dir_all(WINRT_RDL)
        .unwrap_or_else(|e| panic!("failed to create `{WINRT_RDL}`: {e}"));

    writer()
        .input(WINRT_WINMD)
        .filters(["Windows", "!Windows.Win32"])
        .split(true)
        .output(WINRT_RDL)
        .write()
        .unwrap_or_else(|e| panic!("WinRT roundtrip failed: {e}"));
}

/// Win32 and WDK partition their RDL by *defining header*, which the flat winmd does not carry.
/// The committed RDL is the authority for that layout, so the item-name -> header-stem map is
/// recovered from it (exactly as `merge_arch_rdl` does), the winmd is decompiled through that
/// map, and the hand-authored seed (if any) is restored verbatim afterwards. The seed lives
/// outside `rdl_dir` (the partition writer clears every `*.rdl` in it), so restoring it merely
/// rewrites identical bytes. This validates that the winmd's *content* round-trips to the
/// committed RDL under the committed layout.
fn partitioned(label: &str, winmd: &str, rdl_dir: &str, seed: Option<&str>) {
    let seed_name = seed.and_then(|path| {
        std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
    });
    let seed_text = seed.map(|path| {
        std::fs::read(path).unwrap_or_else(|e| panic!("failed to read seed `{path}`: {e}"))
    });

    // Build the item-name -> header-stem map from the committed RDL. Files are sorted for a
    // deterministic map, and the seed is skipped so its attribute-definition types stay in the
    // seed file rather than being scattered into a header partition.
    let mut rdl_paths: Vec<_> = std::fs::read_dir(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to read `{rdl_dir}`: {e}"))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rdl"))
        .filter(|path| path.file_name().and_then(|n| n.to_str()) != seed_name)
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

    if let (Some(path), Some(text)) = (seed, seed_text) {
        write_to_file(path, text)
            .unwrap_or_else(|e| panic!("failed to restore seed `{path}`: {e}"));
    }
}
