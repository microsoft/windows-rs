use windows::core::*;

#[test]
fn test() -> Result<()> {
    let p: PCSTR = s!("hello");
    let s: String = unsafe { p.to_string()? };
    assert_eq!("hello", s);
    assert_eq!("hello", format!("{}", unsafe { p.display() }));
    Ok(())
}
