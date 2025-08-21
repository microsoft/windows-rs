use windows::{
    Win32::Devices::Bluetooth::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::Security::*, Win32::System::ApplicationInstallationAndServicing::*,
    Win32::System::Registry::*,
};

#[test]
fn handle() {
    let underlying: isize = 123;
    let handle: HANDLE = HANDLE(underlying as _);
    assert!(!handle.is_invalid());

    let copy = handle;
    assert!(copy == handle);

    let clone = handle;
    assert!(clone == handle);

    let default = HANDLE::default();
    assert!(default.is_invalid());

    assert_eq!(format!("{:?}", handle), "HANDLE(0x7b)");
}

#[test]
fn psid() {
    let underlying: *mut std::ffi::c_void = std::ptr::null_mut();
    let handle: PSID = PSID(underlying);
    assert!(handle.is_invalid());

    let copy = handle;
    assert!(copy == handle);

    let clone = handle;
    assert!(clone == handle);

    let default = PSID::default();
    assert!(default.is_invalid());

    assert_eq!(format!("{:?}", handle), "PSID(0x0)");
}

#[test]
fn hfont() {
    unsafe {
        let underlying: isize = 123;
        let font: HFONT = HFONT(underlying as _);
        let object: HGDIOBJ = HGDIOBJ(font.0);

        assert!(!DeleteObject(font.into()).as_bool());
        assert!(!DeleteObject(object).as_bool());
    }
}

#[test]
#[expect(clippy::assertions_on_constants)] // intentionally testing constant
fn const_pattern() {
    match HKEY_CLASSES_ROOT {
        HKEY_CLASSES_ROOT => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn unsigned_negative_invalid() {
    assert!(MSIHANDLE(u32::MAX).is_invalid());
    assert!(MSIHANDLE(0).is_invalid());
    assert!(!MSIHANDLE(1).is_invalid());

    assert!(HANDLE_SDP_TYPE(u64::MAX).is_invalid());
    assert!(HANDLE_SDP_TYPE(0).is_invalid());
    assert!(!HANDLE_SDP_TYPE(1).is_invalid());
}
