use std::convert::TryFrom;
use windows::core::*;

#[test]
fn hstring_works() {
    let empty = HSTRING::new();
    assert!(empty.is_empty());
    assert!(empty.is_empty());

    let mut hello = HSTRING::from("Hello");
    assert!(!hello.is_empty());
    assert!(hello.len() == 5);

    let rust = hello.to_string();
    assert!(rust == "Hello");
    assert!(rust.len() == 5);

    let hello2 = hello.clone();
    hello.clear();
    assert!(hello.is_empty());
    hello.clear();
    assert!(hello.is_empty());
    assert!(!hello2.is_empty());
    assert!(hello2.len() == 5);

    assert!(HSTRING::from("Hello") == HSTRING::from("Hello"));
    assert!(HSTRING::from("Hello") != HSTRING::from("World"));

    assert!(HSTRING::from("Hello") == "Hello");
    assert!(HSTRING::from("Hello") != "Hello ");
    assert!(HSTRING::from("Hello") != "Hell");
    assert!(HSTRING::from("Hello") != "World");

    assert!(HSTRING::from("Hello").to_string() == String::from("Hello"));
}

#[test]
fn display_format() {
    let value = HSTRING::from("Hello world");
    assert!(format!("{}", value) == "Hello world");
}

#[test]
fn debug_format() {
    let value = HSTRING::from("Hello world");
    assert!(format!("{:?}", value) == "Hello world");
}

#[test]
fn from_empty_string() {
    let h = HSTRING::from("");
    assert!(format!("{}", h).is_empty());
}

#[test]
fn from_os_string_string() {
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    use std::os::windows::prelude::OsStringExt;
    let o = std::ffi::OsString::from_wide(wide_data);
    let h = HSTRING::from(o);
    let d = HSTRING::from_wide(wide_data);
    assert_eq!(h, d);
}

#[test]
fn hstring_to_string() {
    let h = HSTRING::from("test");
    let s = String::try_from(h).unwrap();
    assert!(s == "test");
}

#[test]
fn hstring_to_string_err() {
    // 𝄞mu<invalid>ic
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let h = HSTRING::from_wide(wide_data);
    let err = String::try_from(h);
    assert!(err.is_err());
}

#[test]
fn hstring_to_string_lossy() {
    // 𝄞mu<invalid>ic
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let h = HSTRING::from_wide(wide_data);
    let s = h.to_string_lossy();
    assert_eq!(s, "𝄞mu�ic");
}

#[test]
fn hstring_to_os_string() {
    // 𝄞mu<invalid>ic
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let h = HSTRING::from_wide(wide_data);
    let s = h.to_os_string();
    use std::os::windows::prelude::OsStringExt;
    assert_eq!(s, std::ffi::OsString::from_wide(wide_data));
}

#[test]
fn hstring_equality_combinations() {
    let h = HSTRING::from("test");
    let s = String::from("test");
    let ss: &str = "test";

    assert_eq!(h, s);
    assert_eq!(&h, s);
    assert_eq!(h, &s);
    assert_eq!(&h, &s);

    assert_eq!(s, h);
    assert_eq!(s, &h);
    assert_eq!(&s, h);
    assert_eq!(&s, &h);

    assert_eq!(h, *ss);
    assert_eq!(&h, *ss);
    assert_eq!(h, ss);
    assert_eq!(&h, ss);

    assert_eq!(*ss, h);
    assert_eq!(*ss, &h);
    assert_eq!(ss, h);
    assert_eq!(ss, &h);
}

#[test]
fn hstring_osstring_equality_combinations() {
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let h = HSTRING::from_wide(wide_data);
    use std::os::windows::prelude::OsStringExt;
    let s = std::ffi::OsString::from_wide(wide_data);
    let ss = s.as_os_str();

    assert_eq!(h, s);
    assert_eq!(&h, s);
    assert_eq!(h, &s);
    assert_eq!(&h, &s);

    assert_eq!(s, h);
    assert_eq!(s, &h);
    assert_eq!(&s, h);
    assert_eq!(&s, &h);

    assert_eq!(h, *ss);
    assert_eq!(&h, *ss);
    assert_eq!(h, ss);
    assert_eq!(&h, ss);

    assert_eq!(*ss, h);
    assert_eq!(*ss, &h);
    assert_eq!(ss, h);
    assert_eq!(ss, &h);
}

#[test]
fn hstring_compat() -> Result<()> {
    unsafe {
        use windows::Win32::System::WinRT::*;
        let hey = HSTRING::from("Hey");
        let world = HSTRING::from("World");
        assert_eq!(WindowsCompareStringOrdinal(&hey, &world)?, -1);
        assert_eq!(WindowsCompareStringOrdinal("Hey", "World")?, -1);

        assert_eq!(WindowsConcatString(&hey, &world)?, "HeyWorld");
        assert_eq!(WindowsConcatString("Hey", "World")?, "HeyWorld");

        assert_eq!(WindowsCreateString(hey.as_wide())?, "Hey");

        assert_eq!(WindowsDuplicateString(&hey)?, "Hey");
        assert_eq!(WindowsDuplicateString("Hey")?, "Hey");

        assert_eq!(WindowsGetStringLen(&hey), 3);
        assert_eq!(WindowsGetStringLen("Hey"), 3);
        assert_eq!(WindowsGetStringLen(&world), 5);
        assert_eq!(WindowsGetStringLen("World"), 5);

        assert_eq!(WindowsIsStringEmpty(HSTRING::new()), true);
        assert_eq!(WindowsIsStringEmpty(""), true);
        assert_eq!(WindowsIsStringEmpty(&world), false);
        assert_eq!(WindowsIsStringEmpty("World"), false);

        let mut len = 0;
        let buffer = WindowsGetStringRawBuffer(&world, &mut len);
        assert_eq!(len, 5);
        // Adding +1 to the length of the slice to validate that it is null terminated.
        assert_eq!(std::slice::from_raw_parts(buffer.0, 6), [87, 111, 114, 108, 100, 0]);

        // We need to drop to windows-sys to call the raw WindowsDeleteString function to avoid double-freeing the HSTRING,
        // but this test is important as it ensures that the allocators match.
        let hresult = windows_sys::Win32::System::WinRT::WindowsDeleteString(std::mem::transmute_copy(&*std::mem::ManuallyDrop::new(hey)));
        assert_eq!(hresult, 0);

        // An HSTRING reference a.k.a. "fast pass" string is a kind of stack-based HSTRING used by C++ callers
        // to avoid the heap allocation in some cases. It's not used in Rust since it assumes a wide character
        // string literal, which is inconvenient to create in Rust. Here we again use windows-sys to make one
        // and thereby excercise the windows::core::HSTRING support for HSTRING reference duplication.
        let mut header: windows_sys::Win32::System::WinRT::HSTRING_HEADER = std::mem::zeroed();
        let mut stack_hstring: windows_sys::core::HSTRING = std::mem::zeroed();
        let hresult = windows_sys::Win32::System::WinRT::WindowsCreateStringReference([87, 111, 114, 108, 100, 0].as_ptr(), 5, &mut header, &mut stack_hstring);
        assert_eq!(hresult, 0);
        assert_eq!(header.length, 5);
        let stack_hstring: std::mem::ManuallyDrop<HSTRING> = std::mem::transmute(stack_hstring);
        let duplicate: HSTRING = (*stack_hstring).clone();
        assert_eq!(&duplicate, &*stack_hstring);
        assert_eq!(duplicate, "World");

        let mut len = 0;
        let buffer = WindowsGetStringRawBuffer(&duplicate, &mut len);
        assert_eq!(len, 5);
        // Adding +1 to the length of the slice to validate that it is null terminated.
        assert_eq!(std::slice::from_raw_parts(buffer.0, 6), [87, 111, 114, 108, 100, 0]);

        Ok(())
    }
}
