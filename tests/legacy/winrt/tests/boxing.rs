use std::convert::{TryFrom, TryInto};
use test_winrt::Windows::Foundation::{IPropertyValue, PropertyValue};
use windows::runtime::{IInspectable, Interface, HSTRING};

macro_rules! primitive_try_into_test {
    ($(($t:ty, $v:literal)),+) => {
        $(
            let o: IInspectable = $v.try_into()?;
            let t: $t = (&o).try_into()?;
            assert_eq!($v, t);
            let t: $t = o.try_into()?;
            assert_eq!($v, t);
        )*
    };
}

macro_rules! primitive_try_from_test {
    ($(($t:ty, $v:literal)),+) => {
        $(
            let o = IInspectable::try_from($v)?;
            assert_eq!($v, <$t>::try_from(&o)?);
            assert_eq!($v, <$t>::try_from(o)?);
        )*
    };
}

#[test]
fn boxing_into() -> windows::runtime::Result<()> {
    primitive_try_into_test! {
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

    let o: IInspectable = "hello".try_into()?;
    let v: HSTRING = (&o).try_into()?;
    assert!("hello" == v);
    let v: HSTRING = o.try_into()?;
    assert!("hello" == v);

    let v = HSTRING::from("hello");
    let o: IInspectable = (&v).try_into()?;
    let v: HSTRING = (&o).try_into()?;
    assert!("hello" == v);
    let v: HSTRING = o.try_into()?;
    assert!("hello" == v);

    let v = HSTRING::from("hello");
    let o: IInspectable = v.try_into()?;
    let v: HSTRING = o.try_into()?;
    assert!("hello" == v);

    let o = IInspectable::try_from("hello")?;
    assert_eq!("hello", HSTRING::try_from(&o)?);
    assert_eq!("hello", HSTRING::try_from(o)?);

    Ok(())
}

#[test]
fn explicit_boxing() -> windows::runtime::Result<()> {
    let object = PropertyValue::CreateString("hello")?;
    let pv: IPropertyValue = object.cast()?;
    assert!(pv.GetString()? == "hello");

    let object = PropertyValue::CreateUInt32Array(&[1, 2, 3])?;
    let pv: IPropertyValue = object.cast()?;
    let mut array = windows::runtime::Array::new();
    assert!(array.is_empty());
    assert!(array.is_empty());

    pv.GetUInt32Array(&mut array)?;
    assert!(array[..] == [1, 2, 3]);
    assert!(!array.is_empty());
    assert!(array.len() == 3);

    let object =
        PropertyValue::CreateStringArray(&["Hello".into(), "Rust".into(), "WinRT".into()])?;
    let pv: IPropertyValue = object.cast()?;
    let mut array = windows::runtime::Array::new();
    assert!(array.is_empty());
    assert!(array.is_empty());

    pv.GetStringArray(&mut array)?;
    assert!(array[..] == ["Hello", "Rust", "WinRT"]);
    assert!(!array.is_empty());
    assert!(array.len() == 3);
    array.clear();

    Ok(())
}
