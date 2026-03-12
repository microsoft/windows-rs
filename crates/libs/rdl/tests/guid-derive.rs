use windows_metadata::reader::TypeIndex;
use windows_metadata::HasAttributes;
use windows_rdl::*;

/// Encode the RDL to winmd, then read back the GuidAttribute for the named type and
/// return it as a lower-case hyphenated GUID string "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".
fn guid_for_type(winmd: &str, namespace: &str, name: &str) -> String {
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

    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]
    )
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
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IEmpty"),
        "93f72119-d661-5bef-a41c-d6bf160de60a",
        "IEmpty GUID mismatch"
    );

    // ISimple: fn get_Value(&self) -> i32  → "Test.ISimple:HRESULT get_Value(Int32*);"
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "ISimple"),
        "edb47bff-51f8-5d11-8cee-758fd914e40a",
        "ISimple GUID mismatch"
    );

    // IWithParam: fn Add(&self, a: i32, b: i32) -> i32  → "Test.IWithParam:HRESULT Add(Int32,Int32,Int32*);"
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IWithParam"),
        "f973f502-0793-56e3-ab53-1fd25ec517d0",
        "IWithParam GUID mismatch"
    );

    // IWithString: fn get_Name(&self) -> String  → "Test.IWithString:HRESULT get_Name(String*);"
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IWithString"),
        "fd7b1ce0-5024-52b3-9551-75e3ba22f958",
        "IWithString GUID mismatch"
    );

    // IHandler (delegate): fn Invoke(arg: i32) -> u32  → "Test.IHandler:HRESULT Invoke(Int32,UInt32*);"
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IHandler"),
        "0121059b-6e73-5866-878e-2bc9151d46bc",
        "IHandler delegate GUID mismatch"
    );

    // IExplicitDelegate: WinRT delegate with explicit Guid — derivation must be suppressed.
    // Explicit values: data1=1, data2=2, data3=3, b0..b7 = 4..11
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IExplicitDelegate"),
        "00000001-0002-0003-0405-060708090a0b",
        "IExplicitDelegate explicit GUID mismatch"
    );

    // IExplicitInterface: WinRT interface with explicit Guid — derivation must be suppressed.
    // Explicit values: data1=17, data2=18, data3=19, b0..b7 = 20..27
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IExplicitInterface"),
        "00000011-0012-0013-1415-161718191a1b",
        "IExplicitInterface explicit GUID mismatch"
    );

    // IWin32: Win32 interface with a derived Guid → "Test.IWin32:"
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IWin32"),
        "9a1e6fa8-6f1f-5234-b2d0-40f90d191be6",
        "IWin32 derived GUID mismatch"
    );

    // IWin32Explicit: Win32 interface with explicit Guid — derivation must be suppressed.
    // Explicit values: data1=33, data2=34, data3=35, b0..b7 = 36..43
    assert_eq!(
        guid_for_type("tests/guid-derive.winmd", "Test", "IWin32Explicit"),
        "00000021-0022-0023-2425-262728292a2b",
        "IWin32Explicit explicit GUID mismatch"
    );
}
