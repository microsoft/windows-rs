winrt::import!(
    dependencies
        os
    types
        windows::foundation::{IPropertyValue, PropertyValue}
);

use windows::foundation::{IPropertyValue, PropertyValue};
use winrt::TryInto;

#[test]
fn boxing() -> winrt::Result<()> {
    // TODO: this is explicit boxing - still need to wrap this up neatly.

    let object = PropertyValue::create_string("hello")?;
    let pv: IPropertyValue = object.try_into()?;
    assert!(pv.get_string()? == "hello");

    let object = PropertyValue::create_uint32_array(&[1, 2, 3])?;
    let pv: IPropertyValue = object.try_into()?;
    let mut array = winrt::Array::new();
    assert!(array.is_empty());
    assert!(array.len() == 0);

    pv.get_uint32_array(&mut array)?;
    assert!(array[..] == [1, 2, 3]);
    assert!(!array.is_empty());
    assert!(array.len() == 3);

    let object =
        PropertyValue::create_string_array(&["Hello".into(), "Rust".into(), "WinRT".into()])?;
    let pv: IPropertyValue = object.try_into()?;
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
