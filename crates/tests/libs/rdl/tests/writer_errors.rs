// Tests that writer error conditions return Err instead of panicking.

use std::sync::atomic::{AtomicU32, Ordering};

/// Per-process unique counter so parallel tests don't share temp paths.
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn unique_path(suffix: &str) -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "test_writer_errors_{}_{n}{suffix}",
        std::process::id()
    ));
    path
}

/// Helper: compile an RDL string to a temp winmd file and return its path.
fn compile_rdl_to_winmd(rdl: &str) -> std::path::PathBuf {
    let winmd = unique_path(".winmd");
    windows_rdl::reader()
        .input_str(rdl)
        .output(winmd.to_str().expect("temp_dir is valid UTF-8"))
        .write()
        .expect("reader should succeed for well-formed RDL");
    winmd
}

// ── I/O failure tests ────────────────────────────────────────────────────────

#[test]
fn writer_returns_err_for_bad_output_path() {
    // Use an existing regular file as a "blocker": create_dir_all cannot turn
    // a file into a directory, so writing to <blocker>/output.rdl returns Err
    // on both Linux and Windows.
    let blocker = unique_path(".blocker");
    std::fs::write(&blocker, b"").expect("should create blocker file");

    let output = blocker.join("output.rdl");
    let result = windows_rdl::writer()
        .output(output.to_str().expect("temp_dir is valid UTF-8"))
        .write();

    let _ = std::fs::remove_file(&blocker);

    assert!(result.is_err(), "expected Err for unwritable output path");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("failed to"),
        "error message should mention 'failed to'; got: {msg}"
    );
}

#[test]
fn writer_split_returns_err_for_bad_output_dir() {
    // In split mode the writer creates <output>/<namespace>.rdl files; the
    // parent of each such file is <output> itself.  Using an existing regular
    // file as <output> makes create_dir_all fail on both Linux and Windows.
    let winmd = compile_rdl_to_winmd(
        r#"
#[win32]
mod Test {
    struct Point { x: i32, y: i32, }
}
    "#,
    );

    let blocker = unique_path(".blocker");
    std::fs::write(&blocker, b"").expect("should create blocker file");

    let result = windows_rdl::writer()
        .input(winmd.to_str().expect("temp_dir is valid UTF-8"))
        .output(blocker.to_str().expect("temp_dir is valid UTF-8"))
        .split(true)
        .write();

    // Clean up before asserting so temp files are always removed.
    let _ = std::fs::remove_file(&winmd);
    let _ = std::fs::remove_file(&blocker);

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
        .input(winmd.to_str().expect("temp_dir is valid UTF-8"))
        .output(outpath.to_str().expect("temp_dir is valid UTF-8"))
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
        .input(winmd.to_str().expect("temp_dir is valid UTF-8"))
        .output(outpath.to_str().expect("temp_dir is valid UTF-8"))
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
        .input(winmd.to_str().expect("temp_dir is valid UTF-8"))
        .output(outpath.to_str().expect("temp_dir is valid UTF-8"))
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
        .input(winmd.to_str().expect("temp_dir is valid UTF-8"))
        .output(outpath.to_str().expect("temp_dir is valid UTF-8"))
        .filter("Test")
        .write();

    let _ = std::fs::remove_file(&winmd);
    let _ = std::fs::remove_file(&outpath);

    assert!(
        result.is_ok(),
        "writer should succeed for a valid interface"
    );
}
