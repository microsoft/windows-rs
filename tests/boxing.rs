// import!(
//     dependencies
//         "os"
//     modules
//         "windows.foundation"
// );

// use windows::foundation::*;
// use winrt::*;

// #[test]
// fn boxing() -> Result<()> {
//     // TODO: this is explicit boxing - still need to wrap this up neatly.
//     let object = PropertyValue::create_string("hello")?;
//     let pv: IPropertyValue = object.query();
//     assert!(pv.get_string()? == "hello");

//     // TODO: see how badly CreateUInt32Array is converted to camel_case.
//     let object = PropertyValue::create_u_int32_array(&[1, 2, 3])?;
//     let pv: IPropertyValue = object.query();
//     let mut array = Array::new();
//     pv.get_u_int32_array(&mut array)?;
//     assert!(array.as_slice() == [1, 2, 3]);

//     Ok(())
// }
