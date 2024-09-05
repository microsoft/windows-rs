use windows::{core::*, Foundation::*};

#[test]
fn test() -> Result<()> {
    let a = Uri::CreateUri(&HSTRING::from("http://kennykerr.ca"))?;

    let w = a.downgrade()?;
    let b = w.upgrade().unwrap();
    assert_eq!(a, b);
    drop(a);
    assert!(w.upgrade().is_some());
    drop(b);
    assert!(w.upgrade().is_none());

    let w = Weak::<Uri>::new();
    assert!(w.upgrade().is_none());

    Ok(())
}
