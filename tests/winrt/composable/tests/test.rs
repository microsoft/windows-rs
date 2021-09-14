use test_winrt_composable::*;
use windows::*;
use Component::Composable::*;

#[test]
fn base() -> Result<()> {
    let base = Base::new()?;
    assert!(base.Value()? == 0);
    base.SetValue(123)?;
    assert!(base.Value()? == 123);

    let base = Base::CreateWithValue(456)?;
    assert!(base.Value()? == 456);
    base.SetValue(123)?;
    assert!(base.Value()? == 123);

    assert!(base.BaseRequired()? == "BaseRequired");

    Ok(())
}

#[test]
fn derived() -> Result<()> {
    let base = Derived::new()?;
    assert!(base.Value()? == 0);
    base.SetValue(123)?;
    assert!(base.Value()? == 123);

    let base = Derived::CreateWithValue(456)?;
    assert!(base.Value()? == 456);
    base.SetValue(123)?;
    assert!(base.Value()? == 123);

    assert!(base.BaseRequired()? == "BaseRequired");
    assert!(base.DerivedRequired()? == "DerivedRequired");

    Ok(())
}
