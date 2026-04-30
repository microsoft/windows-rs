//! Unified test-fixture harness.
//!
//! This crate is the reference implementation of the fixture-driven test
//! format described in `docs/test-todo.md`. Each fixture lives in
//! `crates/tests/fixtures/harness/data/<group>/<name>/` and consists of a
//! small set of input files plus golden output files. `build.rs` discovers
//! every fixture and emits one `#[test]` per fixture; this file dispatches
//! by `group` to perform the appropriate check.
//!
//! Supported groups (phase 1):
//!
//! | group     | inputs                       | check                                               |
//! |-----------|------------------------------|-----------------------------------------------------|
//! | `rdl`     | `input.rdl`                  | RDL → winmd → RDL, diff vs. `expected.rdl`          |
//! | `clang`   | `input.h`                    | Clang → RDL → winmd → RDL, diff vs. `expected.rdl`  |
//! | `bindgen` | `input.rdl` (+ `fixture.toml`) | RDL → winmd → bindgen, diff vs. `expected.rs`     |
//! | `error`   | `input.rdl` (+ `expected.err`) | reader fails with the expected error message      |
//! | `merge`   | `input-*.rdl`                | RDL+RDL → winmd → merge → RDL, diff vs. `expected.rdl` |
//! | `winmd_to_rdl` | `fixture.toml` only (`winmd_input`, `filter`) | writer reads a prebuilt winmd, diff vs. `expected.rdl` |
//!
//! Set `UPDATE_GOLDEN=1` (or `UPDATE_GOLDEN=true`) to overwrite `expected.*`
//! with the actual output instead of asserting equality. This is the only
//! way to add new fixtures: drop the inputs into a new directory and run
//! `UPDATE_GOLDEN=1 cargo test -p test_fixtures`.
//!
//! Each test writes scratch files under a unique
//! `$OUT_DIR/scratch/<group>/<name>/` directory so concurrent tests never
//! contend on the filesystem; `cargo test` then runs every fixture in
//! parallel.

use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

/// Entry point invoked by every generated `#[test]`.
fn run_fixture(group: &str, name: &str) {
    let fixture = Fixture::new(group, name);
    match group {
        "rdl" => run_rdl(&fixture),
        "clang" => {
            // libclang.dll on the Windows CI runner is 64-bit only; the legacy
            // `tests/roundtrip/clang` crate skips the whole suite on 32-bit
            // targets via `#![cfg(target_pointer_width = "64")]`. Mirror that
            // here at runtime so 32-bit builds still discover/compile the
            // generated tests but skip the clang group.
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
        // CARGO_MANIFEST_DIR points at the harness crate root regardless of
        // how cargo is invoked, so fixture lookup is location-independent.
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dir = manifest.join("data").join(group).join(name);
        assert!(dir.is_dir(), "fixture directory missing: {}", dir.display());

        // OUT_DIR is the build script's per-crate output directory. Test
        // binaries can read it via env! at compile time.
        let scratch = PathBuf::from(env!("OUT_DIR"))
            .join("scratch")
            .join(group)
            .join(name);
        // Wipe and recreate so a previous run's leftovers can never cause
        // a stale-pass on a subsequent run.
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

/// A deliberately tiny key=value parser for `fixture.toml`. We only need a
/// handful of declarative knobs (filter, no_allow, no_comment, specific_deps,
/// references); pulling in a full TOML dependency for that would be
/// disproportionate. The format is a strict subset of TOML so authors can
/// add real TOML structure later without breaking existing fixtures.
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
    specific_deps: bool,
    references: Vec<String>,
    /// For the `winmd_to_rdl` group: the prebuilt winmd (or directory) the
    /// writer should consume. There is no `input.rdl`; the fixture is a
    /// pure "filter a winmd to RDL" check used today by the legacy
    /// `nested-arches`, `nested-packing` and `default-interface` tests.
    winmd_input: Option<String>,
    /// For `error` fixtures: which stage is expected to fail. Defaults to
    /// `"reader"` (the legacy behaviour). `"writer"` means the harness must
    /// successfully compile inputs to a winmd via the reader and then assert
    /// that the *writer* fails when fed that winmd back without the
    /// referenced `defs-*.rdl` winmds.
    ///
    /// `"reader_no_input"` runs the reader with no input file; it exists for
    /// the single legacy `invalid_output` test which asserts the reader
    /// rejects an output path of `.`.
    kind: Option<String>,
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
                "specific_deps" => cfg.specific_deps = parse_bool(value),
                "references" => cfg.references = parse_string_list(value),
                "winmd_input" => cfg.winmd_input = Some(parse_string(value)),
                "kind" => cfg.kind = Some(parse_string(value)),
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
    let actual_rdl = f.scratch("out.rdl");

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
        .unwrap_or_else(|e| panic!("[{}/{}] reader: {e}", f.group, f.name));

    let mut writer = windows_rdl::writer();
    writer.input(winmd.to_str().unwrap());
    for r in &cfg.references {
        writer.input(r);
    }
    writer
        .output(actual_rdl.to_str().unwrap())
        .filter(filter)
        .write()
        .unwrap_or_else(|e| panic!("[{}/{}] writer: {e}", f.group, f.name));

    diff_or_update(&actual_rdl, &f.input("expected.rdl"));
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
        .input(input.to_str().unwrap());
    for r in &cfg.references {
        clang.input(r);
    }
    clang
        .output(rdl_intermediate.to_str().unwrap())
        .namespace("Test")
        .library("test.dll")
        .write()
        .unwrap_or_else(|e| panic!("[{}/{}] clang: {e}", f.group, f.name));

    let mut reader = windows_rdl::reader();
    reader.input(rdl_intermediate.to_str().unwrap());
    for r in &cfg.references {
        reader.input(r);
    }
    reader
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap_or_else(|e| panic!("[{}/{}] reader: {e}", f.group, f.name));

    let mut writer = windows_rdl::writer();
    writer.input(winmd.to_str().unwrap());
    for r in &cfg.references {
        writer.input(r);
    }
    writer
        .output(actual_rdl.to_str().unwrap())
        .filter(filter)
        .write()
        .unwrap_or_else(|e| panic!("[{}/{}] writer: {e}", f.group, f.name));

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
        .unwrap_or_else(|e| panic!("[{}/{}] reader: {e}", f.group, f.name));

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

    // The Display impl rewrites paths relative to the input filename which
    // varies per machine; use the raw `message`/`line`/`column` fields so
    // expected.err is portable.
    let actual = format_error(&err, &input);
    diff_or_update_string(&actual, &f.input("expected.err"));
}

/// `kind = "reader_no_input"`: run the reader with *no* input file, asserting
/// it fails. This is a niche shape used today by the single legacy
/// `invalid_output` test which feeds an output path of `.` to the reader and
/// expects "invalid output". The harness assumes the output target is `.`
/// (the only construction the test actually exercises) — if a future fixture
/// needs a different output, lift this to a `reader_output = "..."` knob.
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

    // The Display impl includes ` --> <path>:<line>:<col>` whose `<path>` is
    // the absolute output path, which differs across machines. The legacy
    // test asserted only on the prefix `error: invalid output\n --> .`, so
    // we drop the path/line suffix and render the message with the same
    // leading-`\n` + trailing-`\n` framing that `windows_rdl::Error`'s
    // `Display` impl uses for every other reader-error fixture, keeping
    // expected.err visually consistent across the `error/` group. If this
    // shape ever outgrows a single fixture we should reuse `format_error`
    // instead.
    let actual = format!("\nerror: {}\n", err.message);
    diff_or_update_string(&actual, &f.input("expected.err"));
}

/// Writer-error layout (`kind = "writer"` in fixture.toml):
///
/// ```text
/// data/error/<name>/
///     defs-*.rdl     # one or more dependency RDLs (compiled to winmds)
///     input.rdl      # uses types from the defs above
///     fixture.toml   # kind = "writer"
///     expected.err   # writer error (no path/line; Display = "\nerror: <msg>")
/// ```
///
/// The harness:
///   1. Compiles each `defs-*.rdl` to its own scratch winmd via the reader.
///   2. Compiles `input.rdl` (with the def winmds as additional inputs) to a
///      scratch winmd via the reader. Both reader steps must succeed.
///   3. Runs the *writer* on `input.rdl`'s winmd alone (no def winmds) and
///      asserts it fails. The error is matched against `expected.err`.
///
/// This composes with the future "reference-winmd" coverage from
/// `docs/test-todo.md` §6.3 and replaces the bespoke
/// `writer_errors_on_missing_enum_type` test in `tests/libs/rdl/tests/panic.rs`.
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

    // Step 1: compile each defs-*.rdl to its own winmd. Use the source file
    // stem so scratch artifacts (e.g. `defs-platform.winmd`) are easy to map
    // back to their inputs when debugging a failure.
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
            .unwrap_or_else(|e| {
                panic!(
                    "[{}/{}] reader({}): {e}",
                    f.group,
                    f.name,
                    def_rdl.display()
                )
            });
        def_winmds.push(winmd);
    }

    // Step 2: compile input.rdl referencing the def winmds.
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
        .unwrap_or_else(|e| panic!("[{}/{}] reader(input): {e}", f.group, f.name));

    // Step 3: run the writer on input.winmd ALONE — must fail.
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

    // Writer errors carry no file path or line info (writer_err! sets
    // file_name="" and line=0), so the Display rendering is portable as-is.
    let actual = format!("{err}");
    diff_or_update_string(&actual, &f.input("expected.err"));
}

fn run_merge(f: &Fixture) {
    // Discover input-*.rdl files in lexical order so merge ordering is
    // deterministic and matches authors' intuition (input-a.rdl, input-b.rdl).
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

    let mut merger = windows_metadata::merge();
    for (i, rdl) in inputs.iter().enumerate() {
        let winmd = f.scratch(&format!("part{i}.winmd"));
        windows_rdl::reader()
            .input(rdl.to_str().unwrap())
            .output(winmd.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| panic!("[{}/{}] reader({}): {e}", f.group, f.name, rdl.display()));
        merger.input(winmd.to_str().unwrap());
    }

    let merged = f.scratch("merged.winmd");
    merger
        .output(merged.to_str().unwrap())
        .merge()
        .unwrap_or_else(|e| panic!("[{}/{}] merge: {e}", f.group, f.name));

    let actual_rdl = f.scratch("out.rdl");
    let cfg = f.config();
    let filter = cfg.filter.as_deref().unwrap_or("Test");
    windows_rdl::writer()
        .input(merged.to_str().unwrap())
        .output(actual_rdl.to_str().unwrap())
        .filter(filter)
        .write()
        .unwrap_or_else(|e| panic!("[{}/{}] writer: {e}", f.group, f.name));

    diff_or_update(&actual_rdl, &f.input("expected.rdl"));
}

/// Writer-only "filter a prebuilt winmd to RDL" fixture.
///
/// Layout:
/// ```text
/// data/winmd_to_rdl/<name>/
///     fixture.toml   # winmd_input = "<path>"; filter = "<type>"; (references optional)
///     expected.rdl
/// ```
///
/// There is no `input.rdl` — this group exists for tests that consume a large
/// prebuilt winmd (today: `crates/libs/bindgen/default/Windows*.winmd`) and
/// want to diff a small filtered RDL slice. Replaces the bespoke
/// `nested-arches.rs`, `nested-packing.rs` and `default-interface.rs` tests
/// listed in `docs/test-todo.md` Phase 4 deferred table.
///
/// `winmd_input` is required; `filter` is required (a whole-winmd dump would
/// produce an unwieldy golden file and is not the point of these tests).
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

    // `winmd_input` and `references` paths are resolved relative to the
    // fixture directory so authors can use `../../../...` the same way the
    // legacy roundtrip crates do.
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
        .unwrap_or_else(|e| panic!("[{}/{}] writer: {e}", f.group, f.name));

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

/// Produce a portable rendering of a `windows_rdl::Error` for `expected.err`.
/// The default `Display` impl embeds the absolute path of the input file
/// which differs across machines; we replace that with the basename so
/// goldens are reproducible.
fn format_error(err: &windows_rdl::Error, input: &Path) -> String {
    let basename = input
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("input.rdl");
    let rendered = format!("{err}");
    rendered.replace(&err.file_name, basename)
}
