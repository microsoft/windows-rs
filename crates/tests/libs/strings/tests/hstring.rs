use windows::core::Result;
use windows_strings::*;

#[test]
#[expect(clippy::cmp_owned)] // intentionally testing how operations work
fn hstring_works() {
    assert_eq!(size_of::<HSTRING>(), size_of::<usize>());
    let empty = HSTRING::new();
    assert!(empty.is_empty());
    assert!(empty.is_empty());

    let hello = HSTRING::from("Hello");
    assert!(!hello.is_empty());
    assert!(hello.len() == 5);

    let rust = hello.to_string();
    assert!(rust == "Hello");
    assert!(rust.len() == 5);

    let hello2 = hello.clone();
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
fn display_invalid_format() {
    let s = HSTRING::from_wide(&[
        0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0xDD1E, 0x0069, 0x0063, 0xD834,
    ]);
    let d = format!("{}", s);
    assert_eq!(d, "𝄞mus�ic�");
}

#[test]
fn debug_format() {
    let value = HSTRING::from("Hello world");
    assert!(format!("{:?}", value) == r#""Hello world""#);
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
fn from_os_str_string() {
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    use std::os::windows::prelude::OsStringExt;
    let o = std::ffi::OsString::from_wide(wide_data);
    let o = o.as_os_str();
    let h = HSTRING::from(o);
    let d = HSTRING::from_wide(wide_data);
    assert_eq!(h, d);
}

#[test]
fn from_path() {
    let p = std::path::Path::new("/foo/bar");
    let h = HSTRING::from(p);
    assert_eq!(h, "/foo/bar");
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
fn hstring_hashing_equal_strings() {
    // Checks if two strings of identical contents have the same hash
    use std::hash::{DefaultHasher, Hash, Hasher};

    let hstring_1 = HSTRING::from("Hello World");
    let hstring_2 = HSTRING::from("Hello World");

    assert_eq!(hstring_1, hstring_2);

    let mut hasher_1 = DefaultHasher::new();
    let mut hasher_2 = DefaultHasher::new();

    hstring_1.hash(&mut hasher_1);
    hstring_2.hash(&mut hasher_2);

    let h1_hash = hasher_1.finish();
    let h2_hash = hasher_2.finish();

    assert_eq!(h1_hash, h2_hash);
}

#[test]
fn hstring_hashing_different_strings() {
    // Checks if two strings of different contents have the same hash
    use std::hash::{DefaultHasher, Hash, Hasher};

    let hstring_1 = HSTRING::from("Hello World");
    let hstring_2 = HSTRING::from("Hello World 2");

    assert_ne!(hstring_1, hstring_2);

    let mut hasher_1 = DefaultHasher::new();
    let mut hasher_2 = DefaultHasher::new();

    hstring_1.hash(&mut hasher_1);
    hstring_2.hash(&mut hasher_2);

    let h1_hash = hasher_1.finish();
    let h2_hash = hasher_2.finish();

    assert_ne!(h1_hash, h2_hash);
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

        let result = WindowsConcatString(&hey, &world)?;
        assert_eq!(result, "HeyWorld");

        let result = WindowsCreateString(Some(&hey))?;
        assert_eq!(result, "Hey");

        let result = WindowsDuplicateString(&hey)?;
        assert_eq!(result, "Hey");

        assert_eq!(WindowsGetStringLen(&hey), 3);
        assert_eq!(WindowsGetStringLen(&world), 5);

        assert_eq!(WindowsIsStringEmpty(&HSTRING::new()), true);
        assert_eq!(WindowsIsStringEmpty(&HSTRING::default()), true);
        assert_eq!(WindowsIsStringEmpty(&world), false);

        let mut len = 0;
        let buffer = WindowsGetStringRawBuffer(&world, Some(&mut len));
        assert_eq!(len, 5);
        // Adding +1 to the length of the slice to validate that it is null terminated.
        assert_eq!(
            std::slice::from_raw_parts(buffer.0, 6),
            [87, 111, 114, 108, 100, 0]
        );

        // We need to drop to raw bindings to call the raw WindowsDeleteString function to avoid double-freeing the HSTRING,
        // but this test is important as it ensures that the allocators match.
        let hresult =
            sys::WindowsDeleteString(std::mem::transmute_copy(&*std::mem::ManuallyDrop::new(hey)));
        assert_eq!(hresult, 0);

        // An HSTRING reference a.k.a. "fast pass" string is a kind of stack-based HSTRING used by C++ callers
        // to avoid the heap allocation in some cases. It's not used in Rust since it assumes a wide character
        // string literal, which is inconvenient to create in Rust. Here we again use raw bindings to make one
        // and thereby exercise the windows::core::HSTRING support for HSTRING reference duplication.
        let mut header: sys::HSTRING_HEADER = std::mem::zeroed();
        let mut stack_hstring: sys::HSTRING = std::mem::zeroed();
        let hresult = sys::WindowsCreateStringReference(
            [87, 111, 114, 108, 100, 0].as_ptr(),
            5,
            &mut header,
            &mut stack_hstring,
        );
        assert_eq!(hresult, 0);
        assert_eq!(header.length, 5);
        let stack_hstring: std::mem::ManuallyDrop<HSTRING> = std::mem::transmute(stack_hstring);
        let duplicate: HSTRING = (*stack_hstring).clone();
        assert_eq!(&duplicate, &*stack_hstring);
        assert_eq!(duplicate, "World");

        let mut len = 0;
        let buffer = WindowsGetStringRawBuffer(&duplicate, Some(&mut len));
        assert_eq!(len, 5);
        // Adding +1 to the length of the slice to validate that it is null terminated.
        assert_eq!(
            std::slice::from_raw_parts(buffer.0, 6),
            [87, 111, 114, 108, 100, 0]
        );

        Ok(())
    }
}

#[test]
fn deref_as_slice() {
    let deref = HSTRING::from("0123456789");
    assert!(!deref.is_empty());
    assert_eq!(deref.len(), 10);
    assert_eq!(HSTRING::from_wide(&deref[..=3]), "0123");
    assert!(deref.ends_with(&deref[7..=9]));
    assert_eq!(deref.get(5), Some(b'5' as u16).as_ref());
    let ptr = PCWSTR(deref.as_ptr());
    assert_eq!(deref.cmp(&deref), std::cmp::Ordering::Equal);

    unsafe {
        assert_eq!(*ptr.as_wide(), *deref);
    }

    let empty = HSTRING::new();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(*empty, []);

    unsafe {
        assert_eq!(wcslen(empty.as_ptr()), 0);
    }
}

unsafe extern "C" {
    pub fn wcslen(s: *const u16) -> usize;
}

#[expect(clippy::upper_case_acronyms)]
mod sys {
    windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsCreateStringReference(sourcestring: PCWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut HSTRING) -> HRESULT);
    windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsDeleteString(string: HSTRING) -> HRESULT);

    pub type HRESULT = i32;
    pub type HSTRING = *mut core::ffi::c_void;
    pub type PCWSTR = *const u16;

    #[repr(C)]
    pub struct HSTRING_HEADER {
        pub flags: u32,
        pub length: u32,
        pub padding1: u32,
        pub padding2: u32,
        pub data: isize,
    }
}
