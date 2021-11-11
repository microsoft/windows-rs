use test_winrt_classes::*;
use windows::core::*;
use Component::Classes::*;
use Component::Interfaces::*;

#[test]
fn static_class() -> Result<()> {
    assert!(Static::Method()? == 0);
    assert!(Static::Property()? == 0);
    assert!(Static::ReadOnly()? == 0);
    Static::SetProperty(123)?;
    assert!(Static::Method()? == 123);
    assert!(Static::Property()? == 123);
    assert!(Static::ReadOnly()? == 123);

    Ok(())
}

#[test]
fn activatable() -> Result<()> {
    let c = Activatable::new()?;
    assert!(c.Property()? == 0);

    let c = Activatable::CreateInstance(123)?;
    assert!(c.Property()? == 123);

    Ok(())
}

#[test]
fn creator() -> Result<()> {
    let c = Creator::Create(123)?;
    assert!(c.Property()? == 123);

    Ok(())
}

#[test]
fn required() -> Result<()> {
    let c = Required::new()?;
    assert!(c.Property()? == 0);
    c.SetProperty(123)?;
    assert!(c.Property()? == 123);

    let r: IProperty = c.cast()?;
    assert!(r.Property()? == 123);
    r.SetProperty(456)?;
    assert!(c.Property()? == 456);

    let r: IProperty = c.into();
    assert!(r.Property()? == 456);

    Ok(())
}
