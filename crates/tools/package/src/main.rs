use tool_package::remap::{self, RemapPlan};
use tool_package::{WINRT_WINMD, remap_plan};

/// Throwaway `--in` directory feeding `--package` generation (under `target`, not committed).
/// Holds the remapped header-namespaced Win32/WDK winmd plus a copy of the WinRT `Windows.winmd`.
const PACKAGE_DIR: &str = "target/package";
const REMAP_OUTPUT: &str = "target/package/Windows.Win32.winmd";
const PROBE_DIR: &str = "target/package-bindgen2";

/// Writes a `name<TAB>feature` map (e.g. `D2D1CreateFactory\td2d1`) for every routed item to
/// `path`, so downstream consumer migration can look up the header feature/module for an API.
fn dump_routes(plan: &RemapPlan, path: String) {
    let mut lines: Vec<String> = Vec::new();
    let (routes, _) = remap::routes(plan);
    for (name, namespace) in routes {
        // Mirror bindgen's `namespace_feature`: the `Windows.Win32` umbrella is stripped to
        // the bare header stem; other namespaces drop the leading `Windows`.
        let feature = namespace
            .strip_prefix("Windows.Win32.")
            .map(|stem| stem.replace('.', "_"))
            .or_else(|| {
                namespace
                    .strip_prefix("Windows.")
                    .map(|rest| rest.replace('.', "_"))
            })
            .unwrap_or_else(|| namespace.clone());
        lines.push(format!("{name}\t{feature}"));
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

    // Synthesise the header-based namespace partition from the flat canonical winmd. The Win32
    // SDK is logically a flat global namespace, so the canonical winmd is flat; but `--package`
    // derives file layout and Cargo features from namespaces, so the published crates need a
    // partition. One namespace per defining header (`.rdl` stem) gives a mechanical, source-derived
    // one. The Win32 and WDK RDL directories are read together so WDK's references to Win32 types
    // resolve to the remapped Win32 namespaces.
    let plan = remap_plan();

    let summary = remap::run(&plan, REMAP_OUTPUT);

    dump_routes(&plan, format!("{PACKAGE_DIR}/routes.tsv"));

    // The WinRT metadata is already namespaced; copy it verbatim into the `--in` directory so the
    // `windows` crate can project it alongside the remapped Win32/WDK partition.
    std::fs::copy(WINRT_WINMD, format!("{PACKAGE_DIR}/Windows.winmd"))
        .unwrap_or_else(|e| panic!("failed to stage `{WINRT_WINMD}`: {e}"));

    verify(&summary);

    if std::env::args().any(|arg| arg == "--bindgen2-probe") {
        bindgen2_probe();
    } else {
        // The `windows-sys` crate (sys-style package).
        windows_bindgen::bindgen(["--etc", "crates/tools/package/src/sys.txt"]);

        // The `windows` crate (full-fidelity package).
        windows_bindgen::bindgen(["--etc", "crates/tools/package/src/windows.txt"]);
    }

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn bindgen2_probe() {
    let sys = probe_package(
        "crates/tools/package/src/sys.txt",
        "crates/libs/sys",
        &format!("{PROBE_DIR}/sys"),
    );
    let windows = probe_package(
        "crates/tools/package/src/windows.txt",
        "crates/libs/windows",
        &format!("{PROBE_DIR}/windows"),
    );

    eprintln!("bindgen2 package parity:");
    eprintln!("  windows-sys: {sys}");
    eprintln!("  windows Win32: {}", windows.win32);
    eprintln!("  windows WinRT: {}", windows.winrt);
    eprintln!("  manifests: {}", sys.manifest + windows.manifest);
    eprintln!(
        "  manifest dependencies: {} missing, {} extra",
        sys.manifest_missing_dependencies + windows.manifest_missing_dependencies,
        sys.manifest_extra_dependencies + windows.manifest_extra_dependencies
    );

    let differences = sys.total() + windows.total();
    assert_eq!(
        differences, 0,
        "bindgen2 package output differs in {differences} file(s)"
    );
}

#[derive(Default)]
struct PackageDiff {
    win32: usize,
    winrt: usize,
    manifest: usize,
    manifest_missing_dependencies: usize,
    manifest_extra_dependencies: usize,
    missing: usize,
    extra: usize,
}

impl PackageDiff {
    const fn total(&self) -> usize {
        self.win32 + self.winrt + self.manifest + self.missing + self.extra
    }
}

impl std::fmt::Display for PackageDiff {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} source, {} manifest, {} missing, {} extra",
            self.win32 + self.winrt,
            self.manifest,
            self.missing,
            self.extra
        )
    }
}

fn probe_package(commands: &str, expected: &str, actual: &str) -> PackageDiff {
    let actual = std::path::Path::new(actual);
    if actual.exists() {
        std::fs::remove_dir_all(actual).unwrap();
    }
    std::fs::create_dir_all(actual).unwrap();
    std::fs::copy(
        std::path::Path::new(expected).join("Cargo.toml"),
        actual.join("Cargo.toml"),
    )
    .unwrap();

    windows_bindgen2::command_file(commands)
        .unwrap()
        .output(actual)
        .write()
        .unwrap();

    compare_package(std::path::Path::new(expected), actual)
}

fn compare_package(expected: &std::path::Path, actual: &std::path::Path) -> PackageDiff {
    let expected_files = package_files(expected);
    let actual_files = package_files(actual);
    let mut result = PackageDiff::default();

    for path in expected_files.union(&actual_files) {
        let expected_path = expected.join(path);
        let actual_path = actual.join(path);
        if !expected_path.exists() {
            result.extra += 1;
        } else if !actual_path.exists() {
            result.missing += 1;
        } else if std::fs::read(&expected_path).unwrap() != std::fs::read(&actual_path).unwrap() {
            if path == std::path::Path::new("Cargo.toml") {
                result.manifest += 1;
                let (missing, extra) = compare_manifest_dependencies(
                    &std::fs::read_to_string(expected_path).unwrap(),
                    &std::fs::read_to_string(actual_path).unwrap(),
                );
                result.manifest_missing_dependencies += missing;
                result.manifest_extra_dependencies += extra;
            } else if path.starts_with("src/Windows/Win32") {
                result.win32 += 1;
            } else {
                result.winrt += 1;
            }
        }
    }
    result
}

fn compare_manifest_dependencies(expected: &str, actual: &str) -> (usize, usize) {
    fn dependencies(manifest: &str) -> std::collections::BTreeSet<(String, String)> {
        let Some((_, features)) = manifest.split_once("# generated features") else {
            return std::collections::BTreeSet::new();
        };
        let mut result = std::collections::BTreeSet::new();
        for line in features.lines() {
            let Some((feature, dependencies)) = line.split_once(" = [") else {
                continue;
            };
            for dependency in dependencies.split('"').skip(1).step_by(2) {
                result.insert((feature.to_string(), dependency.to_string()));
            }
        }
        result
    }

    let expected = dependencies(expected);
    let actual = dependencies(actual);
    (
        expected.difference(&actual).count(),
        actual.difference(&expected).count(),
    )
}

fn package_files(root: &std::path::Path) -> std::collections::BTreeSet<std::path::PathBuf> {
    fn collect(
        root: &std::path::Path,
        path: &std::path::Path,
        result: &mut std::collections::BTreeSet<std::path::PathBuf>,
    ) {
        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect(root, &path, result);
            } else if path.file_name().is_some_and(|name| name == "mod.rs") {
                let path = path.strip_prefix(root).unwrap();
                if path.starts_with("src/Windows") {
                    result.insert(path.to_path_buf());
                }
            }
        }
    }

    let mut result = std::collections::BTreeSet::from([std::path::PathBuf::from("Cargo.toml")]);
    let src = root.join("src");
    if src.exists() {
        collect(root, &src, &mut result);
    }
    result
}

/// Asserts the header partition took effect (every Win32/WDK header stem lands in its own
/// `Windows.Win32.<header>` namespace and the flat `Windows.Win32` namespace no longer holds types
/// directly) and reports the synthesised namespace/item totals.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_dependency_comparison_counts_edges() {
        let expected = r#"
# generated features
alpha = ["beta", "gamma"]
empty = []
"#;
        let actual = r#"
# generated features
alpha = ["gamma", "delta"]
empty = []
"#;
        assert_eq!(compare_manifest_dependencies(expected, actual), (1, 1));
    }
}
