winrt::import!(
    dependencies
        os
    modules
        "windows.foundation"
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
    pv.get_uint32_array(&mut array)?;
    assert!(array.as_slice() == [1, 2, 3]);

    Ok(())
}
