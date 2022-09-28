use windows::core::*;

#[test]
fn test() -> Result<()> {
    // `w!` returns a `&HSTRING` so `into()` is needed to convert it to a `PCWSTR`
    let p: PCWSTR = w!("hello").into();
    let s: String = unsafe { p.to_string()? };
    assert_eq!("hello", s);
    assert_eq!("hello", format!("{}", unsafe { p.display() }));
    Ok(())
}
