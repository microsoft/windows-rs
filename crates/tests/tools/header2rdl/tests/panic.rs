use std::sync::atomic::{AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Write `contents` to a temporary header file and return its path.
fn temp_header(contents: &str) -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("header2rdl_panic_{}_{n}.h", std::process::id()));
    std::fs::write(&path, contents).unwrap();
    path
}

fn expect_error(header: &str) -> String {
    let path = temp_header(header);
    tool_header2rdl::converter()
        .file(&path)
        .namespace("Test")
        .convert()
        .expect_err("convert should have failed but succeeded")
}

#[test]
fn typo_in_typedef_errors() {
    if !tool_header2rdl::is_available() {
        return;
    }
    let err = expect_error("typodef int FOO;\n");
    assert!(
        err.contains("error: unknown type name 'typodef'"),
        "unexpected error: {err}"
    );
    assert!(err.contains(" --> "), "error missing location: {err}");
}
