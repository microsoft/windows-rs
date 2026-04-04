// Tests that writer error conditions return Err instead of panicking.

use std::sync::atomic::{AtomicU32, Ordering};

/// Per-process unique counter so parallel tests don't share temp paths.
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn unique_path(suffix: &str) -> String {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("/tmp/test_writer_errors_{}_{n}{suffix}", std::process::id())
}

/// Helper: compile an RDL string to a temp winmd file and return its path.
fn compile_rdl_to_winmd(rdl: &str) -> String {
    let winmd = unique_path(".winmd");
    windows_rdl::reader()
        .input_str(rdl)
        .output(&winmd)
        .write()
        .expect("reader should succeed for well-formed RDL");
    winmd
}

// ── I/O failure tests ────────────────────────────────────────────────────────

#[test]
fn writer_returns_err_for_bad_output_path() {
    // Writing to a path whose parent directory cannot be created should return
    // Err rather than panic.
    let result = windows_rdl::writer()
        .output("/nonexistent/deeply/nested/file.rdl")
        .write();

    assert!(result.is_err(), "expected Err for unwritable output path");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("failed to"),
        "error message should mention 'failed to'; got: {msg}"
    );
}

#[test]
fn writer_split_returns_err_for_bad_output_dir() {
    // In split mode the writer tries to create namespace files inside the
    // output directory.  An unwritable parent should return Err.
    let winmd = compile_rdl_to_winmd(
        r#"
#[win32]
mod Test {
    struct Point { x: i32, y: i32, }
}
    "#,
    );

    let result = windows_rdl::writer()
        .input(&winmd)
        .output("/nonexistent/deeply/nested/split_dir")
        .split(true)
        .write();

    // Clean up before asserting so the temp file is always removed.
    let _ = std::fs::remove_file(&winmd);

    assert!(
        result.is_err(),
        "expected Err for unwritable split output directory"
    );
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("failed to"),
        "error message should mention 'failed to'; got: {msg}"
    );
}

// ── Happy-path smoke tests (writer returns Ok for valid metadata) ─────────────

#[test]
fn writer_succeeds_for_callback() {
    let winmd = compile_rdl_to_winmd(
        r#"
#[win32]
mod Test {
    extern fn Handler(value: i32);
}
    "#,
    );

    let outpath = unique_path(".rdl");
    let result = windows_rdl::writer()
        .input(&winmd)
        .output(&outpath)
        .filter("Test")
        .write();

    let _ = std::fs::remove_file(&winmd);
    let _ = std::fs::remove_file(&outpath);

    assert!(result.is_ok(), "writer should succeed for a valid callback");
}

#[test]
fn writer_succeeds_for_delegate() {
    let winmd = compile_rdl_to_winmd(
        r#"
#[winrt]
mod Test {
    delegate fn Handler(value: i32);
}
    "#,
    );

    let outpath = unique_path(".rdl");
    let result = windows_rdl::writer()
        .input(&winmd)
        .output(&outpath)
        .filter("Test")
        .write();

    let _ = std::fs::remove_file(&winmd);
    let _ = std::fs::remove_file(&outpath);

    assert!(result.is_ok(), "writer should succeed for a valid delegate");
}

#[test]
fn writer_succeeds_for_enum() {
    let winmd = compile_rdl_to_winmd(
        r#"
#[win32]
mod Test {
    #[repr(u32)]
    enum Color { Red = 0, Green = 1, Blue = 2, }
}
    "#,
    );

    let outpath = unique_path(".rdl");
    let result = windows_rdl::writer()
        .input(&winmd)
        .output(&outpath)
        .filter("Test")
        .write();

    let _ = std::fs::remove_file(&winmd);
    let _ = std::fs::remove_file(&outpath);

    assert!(result.is_ok(), "writer should succeed for a valid enum");
}

#[test]
fn writer_succeeds_for_interface() {
    let winmd = compile_rdl_to_winmd(
        r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(&self) -> i32;
    }
}
    "#,
    );

    let outpath = unique_path(".rdl");
    let result = windows_rdl::writer()
        .input(&winmd)
        .output(&outpath)
        .filter("Test")
        .write();

    let _ = std::fs::remove_file(&winmd);
    let _ = std::fs::remove_file(&outpath);

    assert!(
        result.is_ok(),
        "writer should succeed for a valid interface"
    );
}
