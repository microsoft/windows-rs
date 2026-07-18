//! Header-based namespace remap for the published `windows` / `windows-sys` crates.
//!
//! The canonical Win32/WDK winmds are flat (`Windows.Win32`), which is the right shape for
//! every flat/minimal consumer but leaves `--package` generation (which derives file layout and
//! Cargo features from namespaces) with nothing to partition on. This module synthesises the
//! "optional downstream map over the flat namespace": it routes every type/function/constant to
//! a namespace named after its defining header under the corpus root (`wdm.rdl` -> `Windows.Wdk.wdm`)
//! and rewrites the flat winmds into that partition via `windows_metadata::remap`.
//!
//! The partition is metadata-derived — by default one namespace per `.rdl` file, with a small
//! curated allowlist of header-name prefixes (`FOLD_PREFIXES`) that fold obviously-related headers
//! into one family namespace (`d2d1`, `d2d1_1`, `d2d1effects` -> `Windows.d2d`). There is no
//! automatic name heuristic (it mis-groups prefix collisions like `msinkaut` under `msi`), no
//! synthetic catch-all bucket, and no preserved `Win32_Foundation` special case.

use std::collections::HashMap;

/// A flat corpus to remap: the committed per-header RDL directory (the routing signal), the flat
/// winmd compiled from it, and the target namespace root the headers hang under.
pub struct Corpus {
    /// Committed per-header RDL directory, e.g. `metadata/wdk`.
    pub rdl_dir: &'static str,
    /// Flat winmd compiled from `rdl_dir`, e.g. `crates/libs/bindgen/default/Windows.Wdk.winmd`.
    pub winmd: &'static str,
    /// Target namespace root, e.g. `Windows.Wdk` (headers become `Windows.Wdk.<stem>`).
    pub root: &'static str,
}

/// The single flat namespace every canonical Win32/WDK `.rdl` file declares.
const FLAT_NAMESPACE: &str = "Windows.Win32";

/// Curated header-name prefixes whose headers are safe to fold into a single family namespace
/// (`d2d1`, `d2d1_1`, `d2d1effects`, `d2dbasetypes`, … -> `Windows.d2d`).
///
/// This is deliberately an explicit allowlist rather than an automatic name heuristic. A purely
/// name-based rule (fold any header into the shortest existing header-stem that is a prefix of it)
/// mis-groups headers that merely share a prefix — `msinkaut` (Ink) under `msi` (Installer),
/// `playsoundapi` under `pla`, `icmpapi` (ICMP) under `icm` — and cannot be told apart from real
/// families like `sql`/`sqlext` without curation. So instead we assert the safe prefixes here,
/// each verified to cover only genuinely related headers. A header matching no prefix keeps its
/// own name. The longest matching prefix wins (the current list has no overlaps).
const FOLD_PREFIXES: &[&str] = &[
    "bits",
    "d2d",
    "d3d10",
    "d3d11",
    "d3d12",
    "d3d9",
    "dcomp",
    "dwrite",
    "dxgi",
    "functiondiscovery",
    "msxml",
    "ro",
    "rpc",
    "winbio",
    "windns",
    "winusb",
    "wlan",
    "ws2",
    "wsd",
    "xps",
];

/// Maps a header stem to its namespace stem: the longest curated fold prefix it starts with, or
/// the stem itself when none match.
fn fold_stem(stem: &str) -> &str {
    FOLD_PREFIXES
        .iter()
        .filter(|prefix| stem.starts_with(**prefix))
        .max_by_key(|prefix| prefix.len())
        .map_or(stem, |prefix| *prefix)
}

/// Builds the `name -> namespace` route map for a corpus by walking each `.rdl` file's declared
/// item names. Returns the routes plus a per-namespace item-count summary for reporting.
pub fn routes(corpus: &Corpus) -> (HashMap<String, String>, Vec<(String, usize)>) {
    let mut per_file: Vec<(String, Vec<String>)> = Vec::new();

    let mut entries: Vec<std::path::PathBuf> = std::fs::read_dir(corpus.rdl_dir)
        .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", corpus.rdl_dir))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rdl"))
        .collect();
    entries.sort();

    for path in &entries {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("rdl file has a stem");
        let path_str = path.to_string_lossy().replace('\\', "/");
        let names = windows_rdl::item_names(&path_str, FLAT_NAMESPACE)
            .unwrap_or_else(|e| panic!("failed to read `{path_str}`: {e:?}"));
        // Files that declare nothing under the flat namespace (e.g. the metadata seed, whose
        // types live in `Windows.Win32.Metadata`) contribute no routes and are skipped.
        if !names.is_empty() {
            per_file.push((stem.to_string(), names));
        }
    }

    let mut map = HashMap::new();
    let mut summary: HashMap<String, usize> = HashMap::new();

    for (stem, names) in &per_file {
        let namespace = format!("{}.{}", corpus.root, fold_stem(stem));
        for name in names {
            map.insert(name.clone(), namespace.clone());
        }
        *summary.entry(namespace).or_default() += names.len();
    }

    let mut summary: Vec<(String, usize)> = summary.into_iter().collect();
    summary.sort();
    (map, summary)
}

/// Remaps the flat winmds of every corpus into a single header-namespaced winmd at `output`.
///
/// All corpora are read together so cross-corpus references (a WDK type naming a Win32 type)
/// resolve to the referent's remapped namespace. Returns the combined per-namespace summary.
pub fn run(corpora: &[Corpus], output: &str) -> Vec<(String, usize)> {
    let mut map = HashMap::new();
    let mut summary = Vec::new();
    for corpus in corpora {
        let (routes, corpus_summary) = routes(corpus);
        map.extend(routes);
        summary.extend(corpus_summary);
    }
    summary.sort();

    if let Some(parent) = std::path::Path::new(output).parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", parent.display()));
    }

    let mut remapper = windows_metadata::remap();
    remapper
        .source(FLAT_NAMESPACE)
        .fallback(FLAT_NAMESPACE)
        .routes(map)
        .output(output);
    for corpus in corpora {
        remapper.input(corpus.winmd);
    }
    remapper
        .remap()
        .unwrap_or_else(|e| panic!("failed to remap into `{output}`: {e:?}"));

    summary
}
