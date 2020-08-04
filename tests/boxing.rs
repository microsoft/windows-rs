use std::convert::{TryFrom, TryInto};
use winrt::foundation::{IPropertyValue, PropertyValue};
use winrt::ComInterface;
use winrt::HString;
use winrt::Object;

macro_rules! primitive_try_into_test {
    ($($v:literal),+) => {
        $(
            let o: Object = $v.try_into()?;
            assert_eq!($v, (&o).try_into()?);
            assert_eq!($v, o.try_into()?);
        )*
    };
}

macro_rules! primitive_try_from_test {
    ($(($t:ty, $v:literal)),+) => {
        $(
            let o = Object::try_from($v)?;
            assert_eq!($v, <$t>::try_from(&o)?);
            assert_eq!($v, <$t>::try_from(o)?);
        )*
    };
}

#[test]
fn boxing_into() -> winrt::Result<()> {
    primitive_try_into_test! {
        true,
        false,
        123_u8,
        123_i16,
        123_u16,
        123_i32,
        123_u32,
        123_i64,
        123_u64,
        123_f32,
        123_f64
    }

    primitive_try_from_test! {
        (bool, true),
        (bool, false),
        (u8, 123_u8),
        (i16, 123_i16),
        (u16, 123_u16),
        (i32, 123_i32),
        (u32, 123_u32),
        (i64, 123_i64),
        (u64, 123_u64),
        (f32, 123_f32),
        (f64, 123_f64)
    }

    let o: Object = "hello".try_into()?;
    let v: HString = (&o).try_into()?;
    assert!("hello" == v);
    let v: HString = o.try_into()?;
    assert!("hello" == v);

    let v = HString::from("hello");
    let o: Object = (&v).try_into()?;
    let v: HString = (&o).try_into()?;
    assert!("hello" == v);
    let v: HString = o.try_into()?;
    assert!("hello" == v);

    let v = HString::from("hello");
    let o: Object = v.try_into()?;
    let v: HString = o.try_into()?;
    assert!("hello" == v);

    let o = Object::try_from("hello")?;
    assert_eq!("hello", HString::try_from(&o)?);
    assert_eq!("hello", HString::try_from(o)?);

    Ok(())
}

#[test]
fn explicit_boxing() -> winrt::Result<()> {
    let object = PropertyValue::create_string("hello")?;
    let pv: IPropertyValue = object.query();
    assert!(pv.get_string()? == "hello");

    let object = PropertyValue::create_uint32_array(&[1, 2, 3])?;
    let pv: IPropertyValue = object.query();
    let mut array = winrt::Array::new();
    assert!(array.is_empty());
    assert!(array.len() == 0);

    pv.get_uint32_array(&mut array)?;
    assert!(array[..] == [1, 2, 3]);
    assert!(!array.is_empty());
    assert!(array.len() == 3);

    let object =
        PropertyValue::create_string_array(&["Hello".into(), "Rust".into(), "WinRT".into()])?;
    let pv: IPropertyValue = object.query();
    let mut array = winrt::Array::new();
    assert!(array.is_empty());
    assert!(array.len() == 0);

    pv.get_string_array(&mut array)?;
    assert!(array[..] == ["Hello", "Rust", "WinRT"]);
    assert!(!array.is_empty());
    assert!(array.len() == 3);
    array.clear();

    Ok(())
}
