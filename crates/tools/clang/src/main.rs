//! Validator for the libclang pin owned by the `windows-clang` crate. Like the other
//! `gen`-matrix tools it is run by CI and "validates by running": it writes nothing, so the
//! tree stays clean, and any inconsistency in how libclang is pinned fails the run loudly.
//!
//! The metadata scrapers (`tool_win32`/`tool_wdk`/`tool_winrt`/`tool_webview`) all parse with
//! the pinned libclang, so a drift between `LIBCLANG_VERSION` and the download URLs that fetch it
//! would silently change the generated corpus. This tool is the paired guardian of that pin,
//! exactly as `tool_win32` guards the SDK pin and `tool_wdk` reads it back from `tool_win32`.

use std::path::Path;
use windows_clang::LIBCLANG_VERSION;

/// The `windows-clang` source declaring the libclang pin (version + the download URLs that
/// fetch that exact build).
const PROVISION: &str = "crates/libs/clang/src/provision.rs";

/// The pinned download URL(s), which must embed [`LIBCLANG_VERSION`]. `libclang.dll` comes from the
/// `libclang.runtime.win-<arch>` NuGet packages fetched by version (no URL to drift); only the
/// clang resource-header component is a literal URL.
const PINNED_URLS: &[&str] = &["CLANG_RESOURCE_URL"];

/// Workflows scanned for a `KyleMayes/install-llvm-action` Windows step. CI self-provisions
/// libclang from NuGet, so none install LLVM today; this is a defensive guard: if such a step were
/// reintroduced, its major must match [`LIBCLANG_VERSION`] (a Linux-guarded step is ignored — a
/// Linux runner only consumes generated code, never scrapes).
const WORKFLOWS: &[&str] = &[".github/workflows/clippy.yml", ".github/workflows/test.yml"];

fn main() {
    // `tool_clang path` prints the directory holding the pinned `libclang.dll` (respecting an
    // existing `LIBCLANG_PATH`). CI's `test.yml` captures it into `LIBCLANG_PATH` for the
    // `test_clang` suite, so the multithreaded test runner never has to call the `unsafe` `set_var`.
    if std::env::args().nth(1).as_deref() == Some("path") {
        if let Some(dir) = std::env::var_os("LIBCLANG_PATH") {
            println!("{}", Path::new(&dir).display());
        } else {
            println!("{}", windows_clang::libclang_dir().display());
        }
        return;
    }

    // 1. Each pinned download URL must reference the pinned version, so a version bump that
    //    forgets a URL fails here rather than silently fetching a stale libclang.
    for name in PINNED_URLS {
        let url = helpers::read_str_const(PROVISION, name);
        assert!(
            url.contains(LIBCLANG_VERSION),
            "libclang pin drift: `{name}` in `{PROVISION}` does not reference \
             LIBCLANG_VERSION `{LIBCLANG_VERSION}`:\n  {url}"
        );
    }

    // 2. Guard against a reintroduced `install-llvm-action` whose major disagrees with the pin.
    let major = LIBCLANG_VERSION
        .split('.')
        .next()
        .expect("LIBCLANG_VERSION has a major component");
    for path in WORKFLOWS {
        for (line_no, version) in windows_llvm_versions(path) {
            assert!(
                version == major,
                "libclang pin drift: `{path}` installs LLVM `{version}` for a Windows job but \
                 LIBCLANG_VERSION is `{LIBCLANG_VERSION}` (major `{major}`) [line {line_no}]"
            );
        }
    }

    println!("clang pin OK: libclang {LIBCLANG_VERSION}");
}

/// The `version: "N"` value of every `KyleMayes/install-llvm-action` step in a workflow that is
/// *not* guarded by `runner.os == 'Linux'`, paired with its 1-based line number. A tiny
/// purpose-built scan (no YAML dependency): steps begin at a `- ` sequence item, and within an
/// install-llvm step the single `version:` field carries the LLVM major.
fn windows_llvm_versions(path: &str) -> Vec<(usize, String)> {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read `{path}`: {e}"));
    let mut out = Vec::new();
    let mut is_linux = false;
    let mut in_llvm_step = false;
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("- ") {
            // New step: reset the per-step state.
            is_linux = false;
            in_llvm_step = false;
        }
        if trimmed.contains("runner.os == 'Linux'") {
            is_linux = true;
        }
        if trimmed.contains("KyleMayes/install-llvm-action") {
            in_llvm_step = true;
        }
        if in_llvm_step
            && !is_linux
            && let Some(version) = trimmed.strip_prefix("version:")
        {
            out.push((i + 1, version.trim().trim_matches('"').to_string()));
        }
    }
    out
}
