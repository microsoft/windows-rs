use windows::{core::*, Win32::Foundation::*};

#[test]
fn test() -> Result<()> {
    let p: PCSTR = s!("hello");
    let s: String = unsafe { p.to_string()? };
    assert_eq!("hello", s);
    assert_eq!("hello", format!("{}", unsafe { p.display() }));

    let invalid = &[0xc0, 0x80];
    let p = PCSTR::from_raw(invalid.as_ptr());
    let e: Error = unsafe { p.to_string().unwrap_err().into() };
    assert_eq!(e.code(), ERROR_NO_UNICODE_TRANSLATION.into());
    assert_eq!(
        e.message(),
        "No mapping for the Unicode character exists in the target multi-byte code page."
    );

    Ok(())
}
