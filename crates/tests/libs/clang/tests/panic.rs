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
/// reference metadata must be resolved to their underlying primitive type.
/// Previously, the generated RDL contained the bare typedef name which then
/// failed to parse with "type not found".
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
    // MY_BYTE (typedef unsigned char) should resolve to u8.
    assert!(
        rdl.contains("value: u8"),
        "MY_BYTE should resolve to u8; got:\n{rdl}"
    );
    // MY_WORD (typedef unsigned short) should resolve to u16.
    assert!(
        rdl.contains("count: u16"),
        "MY_WORD should resolve to u16; got:\n{rdl}"
    );
    // The typedef names must NOT appear as undefined type names in the struct.
    assert!(
        !rdl.contains("MY_BYTE") && !rdl.contains("MY_WORD"),
        "typedef names from included header must not appear in RDL; got:\n{rdl}"
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
