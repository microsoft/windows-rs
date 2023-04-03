use windows::{core::*, Win32::Foundation::*};

#[test]
fn test() -> Result<()> {
    let p: PCWSTR = w!("hello");
    let s: String = unsafe { p.to_string()? };
    assert_eq!("hello", s);
    assert_eq!("hello", format!("{}", unsafe { p.display() }));

    let invalid = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let p = PCWSTR::from_raw(invalid.as_ptr());
    let e: Error = unsafe { p.to_string().unwrap_err().into() };
    assert_eq!(e.code(), ERROR_NO_UNICODE_TRANSLATION.into());
    assert_eq!(
        e.message(),
        "No mapping for the Unicode character exists in the target multi-byte code page."
    );

    let p: PCWSTR = w!("world");
    let s: HSTRING = unsafe { p.to_hstring()? };
    assert_eq!("world", s);

    Ok(())
}
