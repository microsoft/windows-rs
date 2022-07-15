use windows::core::*;

#[test]
fn display() {
    let s = HSTRING::from_wide(&[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0xDD1E, 0x0069, 0x0063, 0xD834]);
    let d = format!("{}", s);
    assert_eq!(d, "𝄞mus�ic�");
}
