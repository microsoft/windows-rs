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
