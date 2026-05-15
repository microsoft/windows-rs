#![cfg(windows)]

use windows_core::*;
use windows_reference::*;

#[test]
fn primitive_i32() -> Result<()> {
    let r = IReference::<i32>::from(42);
    assert_eq!(r.Value()?, 42);
    Ok(())
}

#[test]
fn primitive_bool() -> Result<()> {
    let r = IReference::<bool>::from(true);
    assert!(r.Value()?);

    let r = IReference::<bool>::from(false);
    assert!(!r.Value()?);
    Ok(())
}

#[test]
fn primitive_f64() -> Result<()> {
    let r = IReference::<f64>::from(1.5);
    assert_eq!(r.Value()?, 1.5);
    Ok(())
}

#[test]
fn hstring_value() -> Result<()> {
    let r = IReference::<HSTRING>::from(HSTRING::from("hello"));
    assert_eq!(r.Value()?, HSTRING::from("hello"));
    Ok(())
}

#[test]
fn guid_value() -> Result<()> {
    let g = GUID::from_u128(0x11223344_5566_7788_99AA_BBCCDDEEFF00);
    let r = IReference::<GUID>::from(g);
    assert_eq!(r.Value()?, g);
    Ok(())
}

#[test]
fn query_property_value() -> Result<()> {
    // The stock IReference also implements IPropertyValue (because IReference derives from
    // it). The Type() accessor returns `OtherType` and the typed `Get*` accessors return
    // errors — callers should retrieve the value via `IReference::Value` instead.
    let r = IReference::<i32>::from(7);
    let pv: IPropertyValue = r.cast()?;
    assert_eq!(pv.Type()?, PropertyType::OtherType);
    assert!(!pv.IsNumericScalar()?);
    assert!(pv.GetInt32().is_err());

    // The original IReference is still reachable via QueryInterface.
    let r2: IReference<i32> = pv.cast()?;
    assert_eq!(r2.Value()?, 7);
    Ok(())
}

#[test]
fn use_as_inspectable() -> Result<()> {
    // A boxed IReference can be cast to an IInspectable since IReference derives from it.
    let r = IReference::<i32>::from(99);
    let _inspectable: IInspectable = r.cast()?;
    Ok(())
}
