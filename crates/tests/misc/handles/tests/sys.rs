use windows_sys::Win32::Graphics::Gdi::*;

#[test]
fn hfont() {
    unsafe {
        let underlying: isize = 123;
        let font: HFONT = underlying as _;
        let object: HGDIOBJ = font;

        assert!(DeleteObject(font) == 0);
        assert!(DeleteObject(object) == 0);
    }
}
