use windows::core::*;

#[test]
fn test() -> Result<()> {
    let p = PWSTR::from_raw(w!("hello").as_ptr() as *mut _);
    let s: String = unsafe { p.to_string()? };
    assert_eq!("hello", s);
    assert_eq!("hello", format!("{}", unsafe { p.display() }));
    Ok(())
}
