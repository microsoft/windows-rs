use windows::{Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::System::Registry::*};

#[test]
fn handle() {
    let underlying: isize = 123;
    let handle: HANDLE = HANDLE(underlying);
    assert!(!handle.is_invalid());

    let copy = handle;
    assert!(copy == handle);

    let clone = handle.clone();
    assert!(clone == handle);

    let default = HANDLE::default();
    assert!(default.is_invalid());

    assert_eq!(format!("{:?}", handle), "HANDLE(123)");
}

#[test]
fn psid() {
    let underlying: *mut std::ffi::c_void = std::ptr::null_mut();
    let handle: PSID = PSID(underlying);
    assert!(handle.is_invalid());

    let copy = handle;
    assert!(copy == handle);

    let clone = handle.clone();
    assert!(clone == handle);

    let default = PSID::default();
    assert!(default.is_invalid());

    assert_eq!(format!("{:?}", handle), "PSID(0x0)");
}

#[test]
fn hfont() {
    unsafe {
        let underlying: isize = 123;
        let font: HFONT = HFONT(underlying);
        let object: HGDIOBJ = HGDIOBJ(font.0);

        assert!(!DeleteObject(font).as_bool());
        assert!(!DeleteObject(object).as_bool());
    }
}

#[test]
fn const_pattern() {
    match HKEY_CLASSES_ROOT {
        HKEY_CLASSES_ROOT => assert!(true),
        _ => assert!(false),
    }
}
