use windows::{core::*, Win32::Foundation::*};

#[test]
fn test() -> Result<()> {
    helpers::set_thread_ui_language();

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

#[test]
fn can_display() {
    // 💖 followed by an invalid byte sequence and then an incomplete one
    let s = [240, 159, 146, 150, 255, 240, 159, 0];
    let s = PCSTR::from_raw(s.as_ptr());
    assert_eq!("💖�", format!("{}", unsafe { s.display() }));
}
