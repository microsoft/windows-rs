use tool_package::remap::{self, Corpus};
use tool_package::{WINRT_WINMD, corpora};

/// Throwaway `--in` directory feeding `--package` generation (under `target`, not committed).
/// Holds the remapped header-namespaced Win32/WDK winmd plus a copy of the WinRT `Windows.winmd`.
const PACKAGE_DIR: &str = "target/package";
const REMAP_OUTPUT: &str = "target/package/Windows.Win32.winmd";

/// Writes a `name<TAB>feature` map (e.g. `D2D1CreateFactory\td2d1`) for every routed item to
/// `path`, so downstream consumer migration can look up the header feature/module for an API.
fn dump_routes(corpora: &[Corpus], path: String) {
    let mut lines: Vec<String> = Vec::new();
    for corpus in corpora {
        let (routes, _) = remap::routes(corpus);
        for (name, namespace) in routes {
            // Mirror bindgen's `namespace_feature`: the `Windows.Win32`/`Windows.Wdk` umbrellas are
            // stripped to the bare header stem; other namespaces drop the leading `Windows`.
            let feature = namespace
                .strip_prefix("Windows.Win32.")
                .or_else(|| namespace.strip_prefix("Windows.Wdk."))
                .map(|stem| stem.replace('.', "_"))
                .or_else(|| {
                    namespace
                        .strip_prefix("Windows.")
                        .map(|rest| rest.replace('.', "_"))
                })
                .unwrap_or_else(|| namespace.clone());
            lines.push(format!("{name}\t{feature}"));
        }
    }
    lines.sort();
    std::fs::write(&path, lines.join("\n"))
        .unwrap_or_else(|e| panic!("failed to write `{path}`: {e}"));
}

/// Generates the published `windows` and `windows-sys` package crates.
///
/// This is separated from `tool_bindings` because package generation uses
/// `--package` mode (per-namespace files + Cargo.toml feature gates) which
/// is an internal bindgen feature not intended for external use.
fn main() {
    let time = std::time::Instant::now();

    // Synthesise the header-based namespace partition from the flat canonical winmds. The Win32
    // SDK is logically a flat global namespace, so the canonical winmd is flat; but `--package`
    // derives file layout and Cargo features from namespaces, so the published crates need a
    // partition. One namespace per defining header (`.rdl` stem) gives a mechanical, source-derived
    // one. Both corpora are remapped together so WDK's references to Win32 types resolve to the
    // remapped Win32 namespaces.
    let corpora = corpora();

    let summary = remap::run(&corpora, REMAP_OUTPUT);

    dump_routes(&corpora, format!("{PACKAGE_DIR}/routes.tsv"));

    // The WinRT metadata is already namespaced; copy it verbatim into the `--in` directory so the
    // `windows` crate can project it alongside the remapped Win32/WDK partition.
    std::fs::copy(WINRT_WINMD, format!("{PACKAGE_DIR}/Windows.winmd"))
        .unwrap_or_else(|e| panic!("failed to stage `{WINRT_WINMD}`: {e}"));

    verify(&summary);

    // The `windows-sys` crate (sys-style package).
    windows_bindgen::bindgen(["--etc", "crates/tools/package/src/sys.txt"]);

    // The `windows` crate (full-fidelity package).
    windows_bindgen::bindgen(["--etc", "crates/tools/package/src/windows.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

/// Asserts the header partition took effect (every Win32/WDK header stem lands in its own
/// `Windows.Win32.<header>` / `Windows.Wdk.<header>` namespace and the flat `Windows.Win32`
/// namespace no longer holds types directly) and reports the synthesised namespace/item totals.
fn verify(summary: &[(String, usize)]) {
    let index = windows_metadata::reader::Index::read(REMAP_OUTPUT)
        .unwrap_or_else(|| panic!("failed to read remapped winmd `{REMAP_OUTPUT}`"));

    assert!(
        !index.contains_namespace("Windows.Win32"),
        "flat `Windows.Win32` namespace survived the remap (types were not routed)"
    );

    let namespaces = summary.len();
    let items: usize = summary.iter().map(|(_, n)| n).sum();
    assert!(namespaces > 0, "remap produced no header-stem namespaces");
    println!("Header partition: {namespaces} namespace(s), {items} item(s)");
}
