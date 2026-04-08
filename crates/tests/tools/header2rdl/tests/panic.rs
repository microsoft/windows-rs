use std::sync::atomic::{AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Write `contents` to a temporary header file and return its path.
fn temp_header(contents: &str) -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("header2rdl_panic_{}_{n}.h", std::process::id()));
    std::fs::write(&path, contents).unwrap();
    path
}

fn should_panic(header: &str) {
    let path = temp_header(header);
    tool_header2rdl::converter()
        .input(path.to_str().unwrap())
        .namespace("Test")
        .convert()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: unknown type name 'typodef'\\n --> \\\"")]
fn typo_in_typedef_errors() {
    should_panic("typodef int FOO;\n");
}
