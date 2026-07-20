use std::io::Write;
use windows_bindgen::bindgen;

const FILTER: &str = "crates/tools/composition/src/composition.txt";

fn main() {
    let time = std::time::Instant::now();

    // System stack: Windows.UI.Composition lives in Windows.winmd; Windows.Win32.winmd
    // supplies ICompositorDesktopInterop and the HWND/BOOL types used to host a visual
    // tree in a plain window. Flat + minimal keeps the crate's own surface small and
    // namespace-free (see docs/crates/windows-composition.md).
    bindgen([
        "--in",
        "crates/libs/bindgen/default/Windows.winmd",
        "crates/libs/bindgen/default/Windows.Win32.winmd",
        "--out",
        "crates/libs/composition/src/bindings.rs",
        "--minimal",
        "--dead-code",
        "--flat",
        "--filter",
        "--etc",
        FILTER,
    ]);

    // Lifted stack: Microsoft.UI.Composition (Microsoft.UI.winmd) mirrors the system
    // API, so the same wrapper source compiles against it. The filter is derived from
    // the one above by dropping system-only regions and rewriting the namespace, so a
    // single filter stays the source of truth for both stacks. Windows.winmd resolves
    // the shared foundation types (Color, TimeSpan, IVector, numerics).
    let lifted_filter = write_lifted_filter();
    bindgen([
        "--in",
        "crates/tools/reactor/winmd/Microsoft.UI.winmd",
        "crates/libs/bindgen/default/Windows.winmd",
        "--out",
        "crates/libs/composition/src/bindings_lifted.rs",
        "--minimal",
        "--dead-code",
        "--flat",
        "--filter",
        "--etc",
        lifted_filter.to_str().expect("utf-8 temp path"),
    ]);

    println!(
        "tool_composition: generated system + lifted bindings in {:.2}s",
        time.elapsed().as_secs_f32()
    );
}

/// Derives the lifted filter from `composition.txt` and writes it to a scratch
/// file under `target/` (never committed), returning its path.
fn write_lifted_filter() -> std::path::PathBuf {
    let source = std::fs::read_to_string(FILTER).expect("read composition.txt");

    let mut lifted = String::with_capacity(source.len());
    let mut in_system_only = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "// region: system-only" {
            in_system_only = true;
            continue;
        }
        if trimmed == "// endregion" {
            in_system_only = false;
            continue;
        }
        if in_system_only {
            continue;
        }
        lifted.push_str(&line.replace("Windows.UI.Composition.", "Microsoft.UI.Composition."));
        lifted.push('\n');
    }

    let path = std::path::Path::new("target").join("tool_composition_lifted.txt");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create target dir");
    }
    let mut file = std::fs::File::create(&path).expect("create lifted filter");
    file.write_all(lifted.as_bytes())
        .expect("write lifted filter");
    path
}
