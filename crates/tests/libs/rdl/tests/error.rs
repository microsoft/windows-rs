use windows_rdl::*;

#[test]
pub fn error_display() {
    let e = Error::new("message", "file_name.rdl", 2, 3);

    let s = format!("{e}");

    assert_eq!(s, "\nerror: message\n --> file_name.rdl:2:4");
}

// ── `#[arch(...)]` error cases ────────────────────────────────────────────────

/// Helper: try to compile an RDL string and return the error message if it fails.
fn compile_rdl_err(rdl: &str) -> Option<String> {
    let out =
        std::env::temp_dir().join(format!("test_arch_err_{}_{}.winmd", std::process::id(), {
            use std::sync::atomic::{AtomicU32, Ordering};
            static N: AtomicU32 = AtomicU32::new(0);
            N.fetch_add(1, Ordering::Relaxed)
        }));
    let result = reader()
        .input_str(rdl)
        .output(out.to_str().unwrap())
        .write();
    let _ = std::fs::remove_file(&out);
    result.err().map(|e| e.to_string())
}

#[test]
fn arch_invalid_name_is_error() {
    let err = compile_rdl_err(
        r#"
#[win32]
mod Test {
    #[arch(X86_32)]
    struct A { x: u8, }
}
    "#,
    );
    let msg = err.expect("expected compile error for unknown arch name");
    assert!(
        msg.contains("invalid `arch` value"),
        "error should mention `arch` value; got: {msg}"
    );
}

#[test]
fn arch_missing_args_is_error() {
    let err = compile_rdl_err(
        r#"
#[win32]
mod Test {
    #[arch]
    struct A { x: u8, }
}
    "#,
    );
    let msg = err.expect("expected compile error for `arch` without args");
    assert!(
        msg.contains("arch"),
        "error should mention `arch`; got: {msg}"
    );
}
