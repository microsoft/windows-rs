#![cfg(target_env = "msvc")]
#![cfg(test)]

mod bindings;
use bindings::*;
use windows_core::*;

#[test]
fn a() -> Result<(), HRESULT> {
    let test = A::new()?;

    assert_eq!(test.Method()?, 1);
    assert_eq!(test.Method2(0)?, 2);

    Ok(())
}

#[test]
fn b() -> Result<(), HRESULT> {
    let test = B::new()?;

    assert_eq!(test.MethodOne()?, 3);
    assert_eq!(test.MethodTwo(0)?, 4);

    Ok(())
}

#[test]
fn c() -> Result<(), HRESULT> {
    let test = C::new()?;

    assert_eq!(test.Method()?, 5);
    assert_eq!(test.Method2(0)?, 6);

    Ok(())
}

#[test]
fn d() -> Result<(), HRESULT> {
    let test = D::new()?;

    assert_eq!(test.Method()?, 7);
    assert_eq!(test.Method2(0)?, 8);
    assert_eq!(test.Method3(0, 0)?, 9);
    assert_eq!(test.Method4(0, 0, 0)?, 10);

    Ok(())
}

#[test]
fn e() -> Result<(), HRESULT> {
    let test = E::new()?;

    assert_eq!(test.MethodOne()?, 11);
    assert_eq!(test.MethodTwo(0)?, 12);
    assert_eq!(test.MethodThree(0, 0)?, 13);
    assert_eq!(test.MethodFour(0, 0, 0)?, 14);

    Ok(())
}
