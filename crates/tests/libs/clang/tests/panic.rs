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

/// When interface GUIDs are defined via a macro (e.g. the Windows SDK's
/// `MIDL_INTERFACE`), each interface must receive its own GUID rather than
/// the first one seen in the translation unit.
#[test]
fn midl_interface_macro_guid() {
    let output = "tests/midl_interface_macro.rdl";
    windows_rdl::clang()
        .args(["-x", "c++", "-fms-extensions"])
        .input_str(
            r#"
#define MIDL_INTERFACE(x) struct __declspec(uuid(x))

MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
IBase {
    virtual int __stdcall QueryInterface(void* riid, void** ppvObject) = 0;
    virtual unsigned int __stdcall AddRef() = 0;
    virtual unsigned int __stdcall Release() = 0;
};

MIDL_INTERFACE("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")
IDerived : public IBase {
public:
    virtual int __stdcall GetIids(unsigned int* iidCount, void** iids) = 0;
    virtual int __stdcall GetRuntimeClassName(void** className) = 0;
    virtual int __stdcall GetTrustLevel(int* trustLevel) = 0;
};
"#,
        )
        .output(output)
        .namespace("Test")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    assert!(
        rdl.contains("0x00000000_0000_0000_c000_000000000046"),
        "IBase should have its own GUID; got:\n{rdl}"
    );
    assert!(
        rdl.contains("0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90"),
        "IDerived should have its own GUID, not IBase's; got:\n{rdl}"
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
