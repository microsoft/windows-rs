use windows::{Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::System::Registry::*};

#[test]
fn boolean() {
    let underlying: u8 = 123;
    let handle: BOOLEAN = BOOLEAN(underlying);
    assert!(!handle.is_invalid());
    assert!(handle.ok().unwrap() == handle);

    let copy = handle;
    assert!(copy == handle);

    let clone = handle.clone();
    assert!(clone == handle);

    let default = BOOLEAN::default();
    assert!(default.is_invalid());

    assert!(format!("{:?}", handle) == "BOOLEAN(123)");
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
