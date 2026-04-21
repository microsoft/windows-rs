#![cfg(target_pointer_width = "64")]

fn should_panic(input: &str) {
    windows_rdl::clang()
        .input_str(input)
        .output(".rdl")
        .namespace("Test")
        .write()
        .unwrap();
}

#[test]
fn success() {
    windows_rdl::clang()
        .args(["-x", "c++"])
        .input_str(
            r#"
struct Thing {
};
        "#,
        )
        .output("tests/success.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}

/// When reference winmd files are provided, types that are already defined in
/// the reference should be suppressed from the output and any uses of those
/// types should refer to them via their fully-qualified metadata namespace.
#[test]
fn reference() {
    let output = "tests/reference.rdl";
    windows_rdl::clang()
        .args(["-x", "c++"])
        .input_str("typedef void* HANDLE; HANDLE __stdcall TestHandle();")
        .input("../../../libs/bindgen/default")
        .output(output)
        .namespace("Test")
        .library("test.dll")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    // HANDLE is defined in Windows.Win32.Foundation, so the typedef should be
    // suppressed and the function return type should use the qualified name.
    assert!(
        !rdl.contains("type HANDLE"),
        "HANDLE typedef should be suppressed when it exists in the reference"
    );
    assert!(
        rdl.contains("Windows::Win32::Foundation::HANDLE"),
        "HANDLE should be referred to by its fully-qualified metadata name"
    );
}

/// Typedefs defined in included (non-main) headers that are not present in any
/// reference metadata must be emitted as `type` items in the RDL so that the
/// generated RDL is self-contained and parses without "type not found" errors.
/// The typedef name must be preserved in struct fields (not resolved away).
#[test]
fn typedef_from_included_header() {
    let output = "tests/typedef_in_struct.rdl";
    windows_rdl::clang()
        .args(["-x", "c++"])
        .input("tests/typedef_in_struct.h")
        .output(output)
        .namespace("Test")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    // The typedef names must be preserved in the struct fields.
    assert!(
        rdl.contains("value: MY_BYTE"),
        "MY_BYTE should be preserved as the field type; got:\n{rdl}"
    );
    assert!(
        rdl.contains("count: MY_WORD"),
        "MY_WORD should be preserved as the field type; got:\n{rdl}"
    );
    // The typedef definitions must also be emitted so the RDL is self-contained.
    assert!(
        rdl.contains("type MY_BYTE"),
        "MY_BYTE typedef definition must be emitted; got:\n{rdl}"
    );
    assert!(
        rdl.contains("type MY_WORD"),
        "MY_WORD typedef definition must be emitted; got:\n{rdl}"
    );
}

/// Verifies that the loaded libclang is at least the minimum tested major
/// version (18).  Older LLVM/Clang releases are known to behave unreliably
/// with windows-rdl (e.g. incorrect `is_from_main_file()` results for
/// declarations inside `extern "C" {}` blocks).
#[test]
fn clang_min_version() {
    let version = windows_rdl::clang_version().expect("failed to get clang version");
    // The version string contains "clang version <MAJOR>.<MINOR>.<PATCH>"
    // and may be prefixed by a vendor string, e.g.
    //   "clang version 18.1.0 (...)"
    //   "Ubuntu clang version 18.1.3 (1ubuntu1)"
    const PREFIX: &str = "clang version ";
    let major: u32 = version
        .find(PREFIX)
        .map(|pos| &version[pos + PREFIX.len()..])
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or_else(|| panic!("unexpected clang version string: {version:?}"));
    assert!(
        major >= 18,
        "libclang {major} is below the minimum tested version (18); got: {version:?}"
    );
}

#[test]
#[should_panic(expected = "error: expected ';' after struct\n --> .h:3:2")]
fn semicolon_expected() {
    should_panic(
        r#"
struct Thing {
}
        "#,
    );
}
