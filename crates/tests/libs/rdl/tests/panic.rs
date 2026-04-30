use std::sync::atomic::{AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Compile `rdl` to a temp `.winmd` file (panicking on failure) and return the path.
fn compile_to_winmd(rdl: &str, extra_input: Option<&str>) -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("rdl_panic_{}_{n}.winmd", std::process::id()));
    let mut r = windows_rdl::reader();
    r.input_str(rdl);
    if let Some(extra) = extra_input {
        r.input(extra);
    }
    r.output(path.to_str().unwrap()).write().unwrap();
    path
}

/// Writing a winmd whose custom attribute carries an enum-valued argument must
/// fail with an error when the winmd that defines the enum type is not provided
/// as an input to the writer.
#[test]
#[should_panic(expected = "error: enum type `Defs::Platform` not found in the metadata index")]
fn writer_errors_on_missing_enum_type() {
    // Step 1: compile the enum + attribute definition into its own winmd.
    let defs_winmd = compile_to_winmd(
        r#"
#[win32]
mod Defs {
    #[repr(i32)]
    #[flags]
    enum Platform { X86 = 1, X64 = 2, Arm64 = 4, }
    attribute SupportedPlatformAttribute { fn(value: Platform); }
}
        "#,
        None,
    );

    // Step 2: compile the struct that uses the attribute, referencing defs_winmd.
    let types_winmd = compile_to_winmd(
        r#"
#[win32]
mod Test {
    #[Defs::SupportedPlatform(X86)]
    struct A { x: u8, }
}
        "#,
        Some(defs_winmd.to_str().unwrap()),
    );

    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let out = std::env::temp_dir().join(format!("rdl_panic_{}_{n}.rdl", std::process::id()));

    // Step 3: write types_winmd WITHOUT providing defs_winmd — must fail.
    windows_rdl::writer()
        .input(types_winmd.to_str().unwrap())
        .output(out.to_str().unwrap())
        .write()
        .unwrap();
}
