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

    // ISimple: fn get_Value(&self) -> i32  → "Test.ISimple:HRESULT get_Value(Int32*);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "ISimple",
        "edb47bff-51f8-5d11-8cee-758fd914e40a",
    );

    // IWithParam: fn Add(&self, a: i32, b: i32) -> i32  → "Test.IWithParam:HRESULT Add(Int32,Int32,Int32*);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IWithParam",
        "f973f502-0793-56e3-ab53-1fd25ec517d0",
    );

    // IWithString: fn get_Name(&self) -> String  → "Test.IWithString:HRESULT get_Name(String*);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IWithString",
        "fd7b1ce0-5024-52b3-9551-75e3ba22f958",
    );

    // IHandler (delegate): fn Invoke(arg: i32) -> u32  → "Test.IHandler:HRESULT Invoke(Int32,UInt32*);"
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "IHandler",
        "0121059b-6e73-5866-878e-2bc9151d46bc",
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

    // ITypeExercise: exercises all type_to_string_extra paths — primitives as return values
    // (Boolean*, Int8*, UInt8*, Int16*, UInt16*, Int64*, UInt64*, Single*, Double*, IntPtr*,
    // UIntPtr*, Object*), const-pointer/ref params (Int32&), and double-mut pointer (Int32**).
    assert_guid(
        "tests/guid-derive.winmd",
        "Test",
        "ITypeExercise",
        "86928075-d2ea-5324-a2e5-0672fed22949",
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

    // ICompareWithMidl: validates against the GUID midlrt.exe assigns to the equivalent C# MIDL3
    // interface. Array params ([In] T[] b) expand to (b_len: u32, b: *mut T) → (UInt32, T*).
    // The `object` type (Object) is already a pointer, so [in] object a → Object*, its
    // array elements → Object**, and the [out,retval] return → Object**.
    assert_guid(
        "tests/guid-derive.winmd",
        "Sample",
        "ICompareWithMidl",
        "382ceef6-493d-5722-9320-2d701e7a5021",
    );
}
