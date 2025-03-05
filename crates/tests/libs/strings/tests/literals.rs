use windows::core::Result;
use windows_strings::*;

#[test]
fn literals() -> Result<()> {
    const A: PCSTR = s!("ansi");
    assert_eq!(unsafe { A.to_string()? }, "ansi");

    const W: PCWSTR = w!("wide");
    assert_eq!(unsafe { W.to_string()? }, "wide");

    let h: &'static HSTRING = h!("hstring");
    assert_eq!(h, "hstring");

    Ok(())
}

#[test]
fn temporary() {
    expect_pcstr(s!("ansi"));
    expect_pcwstr(w!("wide"));
    expect_hstring(h!("hstring"));
}

fn expect_hstring(_: &'static HSTRING) {}
fn expect_pcwstr(_: PCWSTR) {}
fn expect_pcstr(_: PCSTR) {}
