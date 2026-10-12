//! Validator for the libclang pin owned by the generator helpers. Like the other
//! `gen`-matrix tools it is run by CI and "validates by running": it writes nothing, so the
//! tree stays clean, and any inconsistency in how libclang is pinned fails the run loudly.
//!
//! The metadata scrapers (`tool-win32`/`tool-winrt`/`tool-webview`) all parse with
//! the pinned libclang, so a drift between the loaded `libclang.dll` and the clang builtin
//! resource headers would silently change the generated metadata. Both are keyed off the single
//! `LIBCLANG_VERSION` const (the NuGet DLL by version, the headers by the derived `llvmorg-<ver>`
//! git tag), so this tool confirms that pin loads and self-provisions rather than checking a
//! version literal for drift - there is only one to bump.
//!
//! `inventory` stages the parsed RDL census; `sources` stages compiler-backed source discovery
//! and unresolved-group reports without invoking the extractor or changing canonical metadata.
//! `pilot` checks the frozen SDK/COM scope and attempts strict replacement capture with the
//! matched compiler. Incomplete output remains diagnostic staging and returns nonzero.

use helpers::LIBCLANG_VERSION;
use std::path::Path;

mod inventory;
mod pilot;
mod sources;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("pilot") {
        if let Err(error) = pilot::run(std::env::args().skip(2)) {
            eprintln!("{error}");
            std::process::exit(1);
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("native-layout") {
        assert_eq!(std::env::args().len(), 2, "usage: tool-clang native-layout");
        let output = std::process::Command::new("powershell.exe")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"])
            .arg("crates\\libs\\clang2\\native\\build.ps1")
            .args(["-Version", LIBCLANG_VERSION])
            .output()
            .unwrap();
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("{}", stdout.lines().last().unwrap());
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("sources") {
        if let Err(error) = sources::run() {
            eprintln!("{error}");
            std::process::exit(1);
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("inventory") {
        assert_eq!(std::env::args().len(), 2, "usage: tool-clang inventory");
        inventory::run().unwrap();
        return;
    }

    // `tool-clang path` prints the directory holding the pinned `libclang.dll` (respecting an
    // existing `LIBCLANG_PATH`). CI's `test.yml` captures it into `LIBCLANG_PATH` for the
    // `windows-clang` tests, so the multithreaded test runner never calls the unsafe `set_var`.
    if std::env::args().nth(1).as_deref() == Some("path") {
        if let Some(dir) = std::env::var_os("LIBCLANG_PATH") {
            println!("{}", Path::new(&dir).display());
        } else {
            println!("{}", helpers::libclang_dir().display());
        }
        return;
    }

    // Fetch + load the pinned `libclang.dll` and assert it reports `LIBCLANG_VERSION`. This is the
    // same provisioning as the scrapers, so a broken or missing pin fails here rather than
    // during a scrape.
    helpers::ensure_libclang();
    helpers::assert_libclang_version();

    println!("clang pin OK: libclang {LIBCLANG_VERSION}");
}
