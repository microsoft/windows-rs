use windows_metadata::reader::TypeIndex;
use windows_metadata::HasAttributes;
use windows_rdl::*;

/// Read back the GuidAttribute for the named type from `winmd` and assert it equals `expected`.
/// `expected` is a lower-case hyphenated GUID string "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".
fn assert_guid(winmd: &str, namespace: &str, name: &str, expected: &str) {
    let files = TypeIndex::read(winmd).expect("failed to read winmd");
    let ty = files.expect(namespace, name);
    let attr = ty
        .find_attribute("GuidAttribute")
        .expect("GuidAttribute not found");

    let values: Vec<_> = attr.value().into_iter().map(|(_, v)| v).collect();

    let d1 = match values[0] {
        windows_metadata::Value::U32(v) => v,
        _ => panic!("unexpected type for d1"),
    };
    let d2 = match values[1] {
        windows_metadata::Value::U16(v) => v,
        _ => panic!("unexpected type for d2"),
    };
    let d3 = match values[2] {
        windows_metadata::Value::U16(v) => v,
        _ => panic!("unexpected type for d3"),
    };
    let d4: Vec<u8> = values[3..]
        .iter()
        .map(|v| match *v {
            windows_metadata::Value::U8(b) => b,
            _ => panic!("unexpected type for d4 byte"),
        })
        .collect();

    let actual = format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]
    );
    assert_eq!(actual, expected, "{namespace}.{name} GUID mismatch");
}

#[test]
fn guid_derive() {
    Reader::new()
        .input("tests/guid-derive.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/guid-derive.winmd")
        .write()
        .unwrap();

    // IEmpty: no methods → "Test.IEmpty:"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IEmpty",
        "93f72119-d661-5bef-a41c-d6bf160de60a",
    );

    // ISimple: fn get_Value(&self) -> i32  → "Test.ISimple:get_Value(I32);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "ISimple",
        "e2b97695-90c7-5ec8-a50b-ebd2c14cc21c",
    );

    // IWithParam: fn Add(&self, a: i32, b: i32) -> i32  → "Test.IWithParam:Add(I32,I32,I32);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IWithParam",
        "d9541331-4000-54cc-9930-25d4dcac4e9e",
    );

    // IWithString: fn get_Name(&self) -> String  → "Test.IWithString:get_Name(String);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IWithString",
        "a35e5162-a2cc-5615-8020-db6aecaab204",
    );

    // IHandler (delegate): fn Invoke(arg: i32) -> u32  → "Test.IHandler:Invoke(I32,U32);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IHandler",
        "e9a79fae-34d3-5cc3-9424-7904b061d096",
    );

    // IExplicitDelegate: WinRT delegate with explicit Guid — derivation must be suppressed.
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IExplicitDelegate",
        "00000001-0002-0003-0405-060708090a0b",
    );

    // IExplicitInterface: WinRT interface with explicit Guid — derivation must be suppressed.
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IExplicitInterface",
        "00000011-0012-0013-1415-161718191a1b",
    );

    // ITypeExercise: exercises all type_to_string variants — primitive returns (Bool, I8, …),
    // pointer params (PtrConst(I32,1), RefConst(I32), PtrMut(I32,2)).
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "ITypeExercise",
        "556f3952-b277-5c0d-b864-8c73529cf6e2",
    );

    // IWin32: Win32 interface with a derived Guid → "Test.IWin32:"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IWin32",
        "9a1e6fa8-6f1f-5234-b2d0-40f90d191be6",
    );

    // IWin32Explicit: Win32 interface with explicit Guid — derivation must be suppressed.
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IWin32Explicit",
        "00000021-0022-0023-2425-262728292a2b",
    );

    // ICompareWithMidl: exercises array params (Array(T)) and all scalar WinRT primitive types.
    // Each method has an in-param, an array param, and a return type, e.g.:
    //   "Bool(Bool,Array(Bool),Bool);"
    // The Object method uses *mut Object: "Object(PtrMut(Object,1),Array(PtrMut(Object,1)),PtrMut(Object,1));"
    assert_guid(
        "tests/guid-derive.winmd",
        "Sample",
        "ICompareWithMidl",
        "3f9f1bc8-e8be-5939-b0d4-652564052a23",
    );
}
