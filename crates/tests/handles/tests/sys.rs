use windows_sys::{Win32::Foundation::*, Win32::Graphics::Gdi::*};

#[test]
fn boolean() {
    let underlying: u8 = 123;
    let handle: BOOLEAN = underlying;
    assert!(handle == underlying);
}

#[test]
fn hfont() {
    unsafe {
        let underlying: isize = 123;
        let font: HFONT = underlying;
        let object: HGDIOBJ = font;

        assert!(DeleteObject(font) == 0);
        assert!(DeleteObject(object) == 0);
    }
}
