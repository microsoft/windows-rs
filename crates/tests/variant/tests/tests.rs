use windows_core::*;

const EMPTY_VARIANT: VARIANT = VARIANT::new();
const EMPTY_PROPVARIANT: PROPVARIANT = PROPVARIANT::new();

#[test]
fn test_variant() -> Result<()> {
    assert!(EMPTY_VARIANT.is_empty());

    let v = VARIANT::default();
    assert!(v.is_empty());

    let v = VARIANT::from(true);
    assert!(!v.is_empty());
    assert_eq!(bool::try_from(&v)?, true);
    let v = VARIANT::from(false);
    assert_eq!(bool::try_from(&v)?, false);

    let v = VARIANT::from(123u8);
    assert_eq!(u16::try_from(&v)?, 123u16);

    let v = VARIANT::from(-123i8);
    assert_eq!(i16::try_from(&v)?, -123i16);

    let v = VARIANT::from(345u16);
    assert_eq!(u16::try_from(&v)?, 345u16);

    let v = VARIANT::from(-345i16);
    assert_eq!(i16::try_from(&v)?, -345i16);

    let v = VARIANT::from(67890u32);
    assert_eq!(u32::try_from(&v)?, 67890u32);

    let v = VARIANT::from(-67890i32);
    assert_eq!(i32::try_from(&v)?, -67890i32);

    let v = VARIANT::from(5294967295u64);
    assert_eq!(u64::try_from(&v)?, 5294967295u64);

    let v = VARIANT::from(-5294967295i64);
    assert_eq!(i64::try_from(&v)?, -5294967295i64);

    let v = VARIANT::from(3.5f32);
    assert_eq!(f64::try_from(&v)?, 3.5f64);

    let v = VARIANT::from(3.5f64);
    assert_eq!(f64::try_from(&v)?, 3.5f64);

    let v = VARIANT::from(BSTR::from("hello"));
    assert_eq!(BSTR::try_from(&v)?, "hello");

    let v = VARIANT::from(3.5f64);
    assert_eq!(BSTR::try_from(&v)?, "3.5");

    Ok(())
}

#[test]
fn test_propvariant() -> Result<()> {
    assert!(EMPTY_PROPVARIANT.is_empty());

    let v = PROPVARIANT::default();
    assert!(v.is_empty());

    let v = PROPVARIANT::from(true);
    assert!(!v.is_empty());
    assert_eq!(bool::try_from(&v)?, true);
    let v = PROPVARIANT::from(false);
    assert_eq!(bool::try_from(&v)?, false);

    let v = PROPVARIANT::from(123u8);
    assert_eq!(u16::try_from(&v)?, 123u16);

    let v = PROPVARIANT::from(-123i8);
    assert_eq!(i16::try_from(&v)?, -123i16);

    let v = PROPVARIANT::from(345u16);
    assert_eq!(u16::try_from(&v)?, 345u16);

    let v = PROPVARIANT::from(-345i16);
    assert_eq!(i16::try_from(&v)?, -345i16);

    let v = PROPVARIANT::from(67890u32);
    assert_eq!(u32::try_from(&v)?, 67890u32);

    let v = PROPVARIANT::from(-67890i32);
    assert_eq!(i32::try_from(&v)?, -67890i32);

    let v = PROPVARIANT::from(5294967295u64);
    assert_eq!(u64::try_from(&v)?, 5294967295u64);

    let v = PROPVARIANT::from(-5294967295i64);
    assert_eq!(i64::try_from(&v)?, -5294967295i64);

    let v = PROPVARIANT::from(3.5f32);
    assert_eq!(f64::try_from(&v)?, 3.5f64);

    let v = PROPVARIANT::from(3.5f64);
    assert_eq!(f64::try_from(&v)?, 3.5f64);

    let v = PROPVARIANT::from(BSTR::from("hello"));
    assert_eq!(BSTR::try_from(&v)?, "hello");

    let v = PROPVARIANT::from(3.5f64);
    assert_eq!(BSTR::try_from(&v)?, "3.5");

    Ok(())
}
