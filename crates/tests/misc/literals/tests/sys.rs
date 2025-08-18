use windows_sys::core::*;

#[test]
fn test() {
    const A: PCSTR = s!("");
    assert_utf8(A, &[0]);

    const B: PCWSTR = w!("");
    assert_utf16(B, &[0]);

    assert_utf8(s!("hello"), &[b'h', b'e', b'l', b'l', b'o', 0]);
    assert_utf16(w!("hello!"), &[0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x21, 0]);
    assert_utf16(
        w!("α & ω ❤"),
        &[0x03B1, 0x20, 0x26, 0x20, 0x03C9, 0x20, 0x2764, 0],
    );

    assert_utf16(w!("\u{007f}"), &[0x007f, 0]); // Largest one byte UTF8 sequence
    assert_utf16(w!("\u{0080}"), &[0x0080, 0]); // Smallest two byte UTF8 sequence
    assert_utf16(w!("\u{07ff}"), &[0x07ff, 0]); // Largest two byte UTF8 sequence
    assert_utf16(w!("\u{0800}"), &[0x0800, 0]); // Smallest three byte UTF8 sequence
    assert_utf16(w!("\u{d7ff}"), &[0xd7ff, 0]); // Highest code point below surrogate range
    assert_utf16(w!("\u{e000}"), &[0xe000, 0]); // Smallest code point above surrogate range
    assert_utf16(w!("\u{010000}"), &[0xD800, 0xDC00, 0]); // Smallest four byte UTF8 sequence
    assert_utf16(w!("\u{10ffff}"), &[0xDBFF, 0xDFFF, 0]); // Largest Unicode code point
}

fn assert_utf8(left: PCSTR, right: &[u8]) {
    let len = unsafe { strlen(left) };
    assert_eq!(len, right.len() - 1);
    let left = unsafe { std::slice::from_raw_parts(left, right.len()) };
    assert_eq!(left, right);
}

fn assert_utf16(left: PCWSTR, right: &[u16]) {
    let len = unsafe { wcslen(left) };
    assert_eq!(len, right.len() - 1);
    let left = unsafe { std::slice::from_raw_parts(left, right.len()) };
    assert_eq!(left, right);
}

unsafe extern "C" {
    pub fn strlen(s: PCSTR) -> usize;
    pub fn wcslen(s: PCWSTR) -> usize;
}
