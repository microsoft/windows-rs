//! Unified test-fixture harness. See `data/README.md` for the fixture format.

use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

// Reference winmd auto-passed to the `clang` group so individual fixtures
// don't have to declare it (most C++ headers reference Win32 types).
const DEFAULT_REFERENCE: &str = "../../../libs/bindgen/default";

fn run_fixture(group: &str, name: &str) {
    let fixture = Fixture::new(group, name);
    match group {
        "rdl" => run_rdl(&fixture),
        "clang" => {
            // libclang.dll on the Windows CI runner is 64-bit only.
            if cfg!(not(target_pointer_width = "64")) {
                eprintln!("skipping clang/{name} on non-64-bit target");
                return;
            }
            run_clang(&fixture);
        }
        "bindgen" => run_bindgen(&fixture),
        "error" => run_error(&fixture),
        "merge" => run_merge(&fixture),
        "winmd_to_rdl" => run_winmd_to_rdl(&fixture),
        other => panic!("unknown fixture group {other:?}"),
    }
}

struct Fixture {
    group: String,
    name: String,
    dir: PathBuf,
    scratch: PathBuf,
}

impl Fixture {
    fn new(group: &str, name: &str) -> Self {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dir = manifest.join("data").join(group).join(name);
        assert!(dir.is_dir(), "fixture directory missing: {}", dir.display());

        let scratch = PathBuf::from(env!("OUT_DIR"))
            .join("scratch")
            .join(group)
            .join(name);
        // Wipe so a previous run's leftovers can't cause a stale-pass.
        if scratch.exists() {
            std::fs::remove_dir_all(&scratch).unwrap();
        }
        std::fs::create_dir_all(&scratch).unwrap();

        Self {
            group: group.into(),
            name: name.into(),
            dir,
            scratch,
        }
    }

    fn input(&self, file: &str) -> PathBuf {
        self.dir.join(file)
    }

    fn scratch(&self, file: &str) -> PathBuf {
        self.scratch.join(file)
    }

    /// Panic with the harness's standard `[group/name] stage: error` framing.
    fn fail(&self, stage: impl std::fmt::Display, error: impl std::fmt::Display) -> ! {
        panic!("[{}/{}] {stage}: {error}", self.group, self.name);
    }

    fn config(&self) -> FixtureConfig {
        let path = self.dir.join("fixture.toml");
        if path.is_file() {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
            FixtureConfig::parse(&text)
        } else {
            FixtureConfig::default()
        }
    }
}

/// A tiny key=value parser for `fixture.toml`. The format is a strict subset
/// of TOML so authors can add real TOML structure later without breaking
/// existing fixtures.
///
/// Supported lines:
/// ```text
/// # comments
/// key = "string"
/// key = true
/// key = false
/// key = ["string", "string"]
/// ```
#[derive(Default)]
struct FixtureConfig {
    filter: Option<String>,
    no_allow: bool,
    no_comment: bool,
    noexcept: bool,
    specific_deps: bool,
    references: Vec<String>,
    /// `winmd_to_rdl` only: prebuilt winmd (or directory) to consume.
    winmd_input: Option<String>,
    /// `error` only: which stage must fail. `"reader"` (default),
    /// `"reader_no_input"`, or `"writer"`.
    kind: Option<String>,
    /// `merge` only: per-input arch tagging, e.g. `"input-x86.rdl=X86"`.
    /// Arches are `X86`/`X64`/`Arm64` or `|`-joined. When set, the harness
    /// uses `Merger::arch_input` so types present in only some arches get
    /// a `SupportedArchitecture` attribute.
    arch_inputs: Vec<String>,
    /// `rdl` only: run the writer once per entry, each `"<expected>=<filter[;filter...]>"`.
    /// `;` separates multiple `writer.filter(...)` calls in a single invocation.
    outputs: Vec<String>,
}

impl FixtureConfig {
    fn parse(text: &str) -> Self {
        let mut cfg = Self::default();
        for (lineno, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line.split_once('=').unwrap_or_else(|| {
                panic!("fixture.toml line {}: missing '=': {raw:?}", lineno + 1)
            });
            let key = key.trim();
            let value = value.trim();
            match key {
                "filter" => cfg.filter = Some(parse_string(value)),
                "no_allow" => cfg.no_allow = parse_bool(value),
                "no_comment" => cfg.no_comment = parse_bool(value),
                "noexcept" => cfg.noexcept = parse_bool(value),
                "specific_deps" => cfg.specific_deps = parse_bool(value),
                "references" => cfg.references = parse_string_list(value),
                "winmd_input" => cfg.winmd_input = Some(parse_string(value)),
                "kind" => cfg.kind = Some(parse_string(value)),
                "arch_inputs" => cfg.arch_inputs = parse_string_list(value),
                "outputs" => cfg.outputs = parse_string_list(value),
                other => panic!("fixture.toml: unknown key {other:?}"),
            }
        }
        cfg
    }
}

fn parse_string(value: &str) -> String {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();
    assert!(
        bytes.len() >= 2 && bytes.first() == Some(&b'"') && bytes.last() == Some(&b'"'),
        "expected quoted string, got {value:?}"
    );
    trimmed[1..trimmed.len() - 1].to_string()
}

fn parse_bool(value: &str) -> bool {
    match value.trim() {
        "true" => true,
        "false" => false,
        other => panic!("expected bool, got {other:?}"),
    }
}

fn parse_string_list(value: &str) -> Vec<String> {
    let trimmed = value.trim();
    assert!(
        trimmed.starts_with('[') && trimmed.ends_with(']'),
        "expected array, got {value:?}"
    );
    let inner = &trimmed[1..trimmed.len() - 1];
    if inner.trim().is_empty() {
        return Vec::new();
    }
    inner.split(',').map(|s| parse_string(s.trim())).collect()
}

// ---------------------------------------------------------------------------
// Group runners
// ---------------------------------------------------------------------------

fn run_rdl(f: &Fixture) {
    let input = f.input("input.rdl");
    let winmd = f.scratch("out.winmd");

    let cfg = f.config();

    let mut reader = windows_rdl::reader();
    reader.input(input.to_str().unwrap());
    for r in &cfg.references {
        reader.input(r);
    }
    reader
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap_or_else(|e| f.fail("reader", e));

    // With no `outputs` declared, run a single writer with `filter` (default
    // "Test") against `expected.rdl`. With `outputs` declared, run once per
    // entry so a fixture can diff several filtered slices against own goldens.
    let invocations: Vec<(String, Vec<String>)> = if cfg.outputs.is_empty() {
        let filter = cfg.filter.as_deref().unwrap_or("Test");
        vec![("expected.rdl".to_string(), vec![filter.to_string()])]
    } else {
        cfg.outputs.iter().map(|s| parse_output_spec(s)).collect()
    };

    for (i, (expected, filters)) in invocations.iter().enumerate() {
        let actual_rdl = f.scratch(&format!("out{i}.rdl"));
        let mut writer = windows_rdl::writer();
        writer.input(winmd.to_str().unwrap());
        for r in &cfg.references {
            writer.input(r);
        }
        writer.output(actual_rdl.to_str().unwrap());
        for filter in filters {
            writer.filter(filter);
        }
        writer
            .write()
            .unwrap_or_else(|e| f.fail(format_args!("writer({expected})"), e));

        diff_or_update(&actual_rdl, &f.input(expected));
    }
}

/// Parse one `outputs = [...]` entry of the form `"<expected>=<filter[;filter...]>"`.
fn parse_output_spec(s: &str) -> (String, Vec<String>) {
    let (expected, filters) = s.split_once('=').unwrap_or_else(|| {
        panic!("`outputs` entry {s:?} must be of the form \"<expected>=<filter-spec>\"")
    });
    let filters: Vec<String> = filters.split(';').map(|f| f.trim().to_string()).collect();
    assert!(
        !filters.is_empty() && filters.iter().all(|f| !f.is_empty()),
        "`outputs` entry {s:?} has an empty filter spec"
    );
    (expected.trim().to_string(), filters)
}

fn run_clang(f: &Fixture) {
    let input = f.input("input.h");
    let rdl_intermediate = f.scratch("intermediate.rdl");
    let winmd = f.scratch("out.winmd");
    let actual_rdl = f.scratch("out.rdl");

    let cfg = f.config();
    let filter = cfg.filter.as_deref().unwrap_or("Test");

    let mut clang = windows_rdl::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input(input.to_str().unwrap())
        .input(DEFAULT_REFERENCE);
    for r in &cfg.references {
        clang.input(r);
    }
    clang
        .output(rdl_intermediate.to_str().unwrap())
        .namespace("Test")
        .library("test.dll")
        .write()
        .unwrap_or_else(|e| f.fail("clang", e));

    let mut reader = windows_rdl::reader();
    reader.input(rdl_intermediate.to_str().unwrap());
    reader.input(DEFAULT_REFERENCE);
    for r in &cfg.references {
        reader.input(r);
    }
    reader
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap_or_else(|e| f.fail("reader", e));

    let mut writer = windows_rdl::writer();
    writer.input(winmd.to_str().unwrap());
    writer.input(DEFAULT_REFERENCE);
    for r in &cfg.references {
        writer.input(r);
    }
    writer
        .output(actual_rdl.to_str().unwrap())
        .filter(filter)
        .write()
        .unwrap_or_else(|e| f.fail("writer", e));

    diff_or_update(&actual_rdl, &f.input("expected.rdl"));
}

fn run_bindgen(f: &Fixture) {
    let input = f.input("input.rdl");
    let winmd = f.scratch("out.winmd");
    let actual_rs = f.scratch("out.rs");

    let cfg = f.config();
    let filter = cfg.filter.as_deref().unwrap_or("Test");

    let mut reader = windows_rdl::reader();
    reader.input(input.to_str().unwrap());
    for r in &cfg.references {
        reader.input(r);
    }
    reader
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap_or_else(|e| f.fail("reader", e));

    let mut bindgen = windows_bindgen::builder();
    bindgen
        .input(winmd.to_str().unwrap())
        .output(actual_rs.to_str().unwrap())
        .filter(filter);
    if cfg.no_allow {
        bindgen.no_allow();
    }
    if cfg.no_comment {
        bindgen.no_comment();
    }
    if cfg.specific_deps {
        bindgen.specific_deps();
    }
    if cfg.noexcept {
        bindgen.noexcept();
    }
    bindgen.write().unwrap();

    diff_or_update(&actual_rs, &f.input("expected.rs"));
}

fn run_error(f: &Fixture) {
    let cfg = f.config();
    match cfg.kind.as_deref().unwrap_or("reader") {
        "reader" => run_error_reader(f),
        "reader_no_input" => run_error_reader_no_input(f),
        "writer" => run_error_writer(f),
        other => panic!(
            "[{}/{}] unknown error fixture kind {other:?}; expected \"reader\", \"reader_no_input\", or \"writer\"",
            f.group, f.name
        ),
    }
}

fn run_error_reader(f: &Fixture) {
    let input = f.input("input.rdl");
    let scratch_winmd = f.scratch("out.winmd");

    let err = windows_rdl::reader()
        .input(input.to_str().unwrap())
        .output(scratch_winmd.to_str().unwrap())
        .write()
        .err()
        .unwrap_or_else(|| {
            panic!(
                "[{}/{}] expected reader to fail but it succeeded",
                f.group, f.name
            )
        });

    let actual = format_error(&err, &input);
    diff_or_update_string(&actual, &f.input("expected.err"));
}

/// `kind = "reader_no_input"`: run the reader with no input file. Used today
/// only by `invalid_output`, which feeds output `.` and expects "invalid
/// output". If a future fixture needs a different output target, lift this
/// to a `reader_output = "..."` knob.
fn run_error_reader_no_input(f: &Fixture) {
    let err = windows_rdl::reader()
        .output(".")
        .write()
        .err()
        .unwrap_or_else(|| {
            panic!(
                "[{}/{}] expected reader to fail but it succeeded",
                f.group, f.name
            )
        });

    // Drop the path/line suffix (it's machine-dependent) and reframe the
    // message with the same `\nerror: ...\n` shape used elsewhere.
    let actual = format!("\nerror: {}\n", err.message);
    diff_or_update_string(&actual, &f.input("expected.err"));
}

/// `kind = "writer"`: compile every `defs-*.rdl` and `input.rdl` to winmds,
/// then run the writer on `input.winmd` *alone* and assert it fails.
fn run_error_writer(f: &Fixture) {
    let mut defs: Vec<PathBuf> = std::fs::read_dir(&f.dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("defs-") && n.ends_with(".rdl"))
        })
        .collect();
    defs.sort();
    assert!(
        !defs.is_empty(),
        "writer-error fixture {} needs at least one defs-*.rdl file",
        f.name
    );

    let mut def_winmds = Vec::with_capacity(defs.len());
    for def_rdl in &defs {
        let stem = def_rdl
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("defs-*.rdl path has a UTF-8 file stem");
        let winmd = f.scratch(&format!("{stem}.winmd"));
        windows_rdl::reader()
            .input(def_rdl.to_str().unwrap())
            .output(winmd.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| f.fail(format_args!("reader({})", def_rdl.display()), e));
        def_winmds.push(winmd);
    }

    let input = f.input("input.rdl");
    let input_winmd = f.scratch("input.winmd");
    let mut reader = windows_rdl::reader();
    reader.input(input.to_str().unwrap());
    for w in &def_winmds {
        reader.input(w.to_str().unwrap());
    }
    reader
        .output(input_winmd.to_str().unwrap())
        .write()
        .unwrap_or_else(|e| f.fail("reader(input)", e));

    let actual_rdl = f.scratch("out.rdl");
    let err = windows_rdl::writer()
        .input(input_winmd.to_str().unwrap())
        .output(actual_rdl.to_str().unwrap())
        .write()
        .err()
        .unwrap_or_else(|| {
            panic!(
                "[{}/{}] expected writer to fail but it succeeded",
                f.group, f.name
            )
        });

    // Writer errors carry no file path or line info, so Display is portable.
    let actual = format!("{err}");
    diff_or_update_string(&actual, &f.input("expected.err"));
}

fn run_merge(f: &Fixture) {
    let cfg = f.config();

    // Lexical order so merge ordering is deterministic.
    let mut inputs: Vec<PathBuf> = std::fs::read_dir(&f.dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("input-") && n.ends_with(".rdl"))
        })
        .collect();
    inputs.sort();
    assert!(
        inputs.len() >= 2,
        "merge fixture {} needs at least two input-*.rdl files",
        f.name
    );

    let arch_map = parse_arch_inputs(&cfg.arch_inputs);

    let mut merger = windows_metadata::merge();
    for (i, rdl) in inputs.iter().enumerate() {
        let winmd = f.scratch(&format!("part{i}.winmd"));
        windows_rdl::reader()
            .input(rdl.to_str().unwrap())
            .output(winmd.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| f.fail(format_args!("reader({})", rdl.display()), e));
        if arch_map.is_empty() {
            merger.input(winmd.to_str().unwrap());
        } else {
            let basename = rdl.file_name().and_then(|s| s.to_str()).unwrap();
            let bits = arch_map.iter().find(|(k, _)| k == basename).map(|(_, v)| *v)
                .unwrap_or_else(|| panic!(
                    "[{}/{}] arch_inputs missing entry for {basename}; declare it as `\"{basename}=<arch>\"`",
                    f.group, f.name
                ));
            merger.arch_input(winmd.to_str().unwrap(), bits);
        }
    }

    let merged = f.scratch("merged.winmd");
    merger
        .output(merged.to_str().unwrap())
        .merge()
        .unwrap_or_else(|e| f.fail("merge", e));

    let actual_rdl = f.scratch("out.rdl");
    let filter = cfg.filter.as_deref().unwrap_or("Test");
    windows_rdl::writer()
        .input(merged.to_str().unwrap())
        .output(actual_rdl.to_str().unwrap())
        .filter(filter)
        .write()
        .unwrap_or_else(|e| f.fail("writer", e));

    diff_or_update(&actual_rdl, &f.input("expected.rdl"));
}

/// Parse `arch_inputs = ["input-x86.rdl=X86", ...]` into `(filename, bits)`
/// pairs (X86=1, X64=2, Arm64=4; `|`-joinable).
fn parse_arch_inputs(entries: &[String]) -> Vec<(String, i32)> {
    entries
        .iter()
        .map(|s| {
            let (name, arches) = s.split_once('=').unwrap_or_else(|| {
                panic!("`arch_inputs` entry {s:?} must be of the form \"<file>=<arch[|arch...]>\"")
            });
            let bits = arches
                .split('|')
                .map(|a| match a.trim() {
                    "X86" => 1,
                    "X64" => 2,
                    "Arm64" => 4,
                    other => panic!(
                        "`arch_inputs` entry {s:?}: unknown arch {other:?}; expected X86, X64, or Arm64"
                    ),
                })
                .fold(0, |acc, b| acc | b);
            (name.trim().to_string(), bits)
        })
        .collect()
}

/// Writer-only: filter a prebuilt winmd to RDL. `winmd_input` and `filter`
/// are required; paths resolve relative to the fixture directory.
fn run_winmd_to_rdl(f: &Fixture) {
    let cfg = f.config();
    let winmd_input = cfg.winmd_input.as_deref().unwrap_or_else(|| {
        panic!(
            "[{}/{}] winmd_to_rdl fixture requires `winmd_input = \"...\"` in fixture.toml",
            f.group, f.name
        )
    });
    let filter = cfg.filter.as_deref().unwrap_or_else(|| {
        panic!(
            "[{}/{}] winmd_to_rdl fixture requires `filter = \"...\"` in fixture.toml",
            f.group, f.name
        )
    });

    let resolved_input = f.dir.join(winmd_input);

    let actual_rdl = f.scratch("out.rdl");
    let mut writer = windows_rdl::writer();
    writer.input(resolved_input.to_str().unwrap());
    for r in &cfg.references {
        let resolved = f.dir.join(r);
        writer.input(resolved.to_str().unwrap());
    }
    writer
        .output(actual_rdl.to_str().unwrap())
        .filter(filter)
        .write()
        .unwrap_or_else(|e| f.fail("writer", e));

    diff_or_update(&actual_rdl, &f.input("expected.rdl"));
}

// ---------------------------------------------------------------------------
// Diffing / golden-file helpers
// ---------------------------------------------------------------------------

fn update_mode() -> bool {
    matches!(
        std::env::var("UPDATE_GOLDEN").as_deref(),
        Ok("1") | Ok("true")
    )
}

fn diff_or_update(actual_path: &Path, expected_path: &Path) {
    let actual = std::fs::read_to_string(actual_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", actual_path.display()));
    diff_or_update_string(&actual, expected_path);
}

fn diff_or_update_string(actual: &str, expected_path: &Path) {
    if update_mode() {
        std::fs::write(expected_path, actual)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", expected_path.display()));
        return;
    }
    let expected = std::fs::read_to_string(expected_path).unwrap_or_else(|e| {
        panic!(
            "failed to read golden file {} (rerun with UPDATE_GOLDEN=1 to create it): {e}",
            expected_path.display()
        )
    });
    if actual != expected {
        panic!(
            "golden mismatch for {path}\n--- expected ---\n{expected}\n--- actual ---\n{actual}\n--- end ---\n\
             rerun with UPDATE_GOLDEN=1 to update.",
            path = expected_path.display(),
        );
    }
}

/// Render a `windows_rdl::Error` portably for `expected.err` by replacing the
/// machine-dependent absolute path with the input basename.
fn format_error(err: &windows_rdl::Error, input: &Path) -> String {
    let basename = input
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("input.rdl");
    let rendered = format!("{err}");
    rendered.replace(&err.file_name, basename)
}
