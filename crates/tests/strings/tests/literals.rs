use windows_strings::*;

#[test]
fn literals() -> Result<()> {
    const A: PCSTR = s!("ansi");
    assert_eq!(unsafe { A.to_string()? }, "ansi");

    const W: PCWSTR = w!("wide");
    assert_eq!(unsafe { W.to_string()? }, "wide");

    const H: &HSTRING = h!("hstring");
    assert_eq!(H, "hstring");

    Ok(())
}
