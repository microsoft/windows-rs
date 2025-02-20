use windows_registry::*;
use windows_strings::*;

#[test]
fn value() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\value";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_value("u32", &Value::from(123u32))?;
    assert_eq!(key.get_type("u32")?, Type::U32);
    assert_eq!(key.get_value("u32")?, Value::from(123u32));
    assert_eq!(key.get_u32("u32")?, 123u32);
    assert_eq!(key.get_u64("u32")?, 123u64);
    assert_eq!(u32::try_from(key.get_value("u32")?)?, 123u32);

    assert_eq!(unsafe { key.raw_get_info(w!("u32"))? }, (Type::U32, 4));

    key.set_value("u64", &Value::from(123u64))?;
    assert_eq!(key.get_type("u64")?, Type::U64);
    assert_eq!(key.get_value("u64")?, Value::from(123u64));
    assert_eq!(key.get_u32("u64")?, 123u32);
    assert_eq!(key.get_u64("u64")?, 123u64);
    assert_eq!(u64::try_from(key.get_value("u64")?)?, 123u64);

    assert_eq!(unsafe { key.raw_get_info(w!("u64"))? }, (Type::U64, 8));

    key.set_value("string", &Value::from("string"))?;
    assert_eq!(key.get_type("string")?, Type::String);
    assert_eq!(key.get_value("string")?, Value::from("string"));
    assert_eq!(key.get_string("string")?, "string");
    assert_eq!(String::try_from(key.get_value("string")?)?, "string");

    assert_eq!(
        unsafe { key.raw_get_info(w!("string"))? },
        (Type::String, 14)
    );

    let mut value = Value::from("expand");
    value.set_ty(Type::ExpandString);
    assert_eq!(value.ty(), Type::ExpandString);
    key.set_value("expand", &value)?;
    assert_eq!(key.get_type("expand")?, Type::ExpandString);
    assert_eq!(key.get_value("expand")?, value);
    assert_eq!(key.get_string("expand")?, "expand");

    assert_eq!(
        unsafe { key.raw_get_info(w!("expand"))? },
        (Type::ExpandString, 14)
    );

    key.set_value("bytes", &Value::from([1u8, 2u8, 3u8]))?;
    assert_eq!(key.get_type("bytes")?, Type::Bytes);
    assert_eq!(key.get_value("bytes")?, Value::from([1, 2, 3]));

    assert_eq!(unsafe { key.raw_get_info(w!("bytes"))? }, (Type::Bytes, 3));

    let mut value = Value::from([1u8, 2u8, 3u8, 4u8].as_slice());
    value.set_ty(Type::Other(1234));
    key.set_value("slice", &value)?;
    assert_eq!(key.get_type("slice")?, Type::Other(1234));
    assert_eq!(key.get_value("slice")?, value);

    assert_eq!(
        unsafe { key.raw_get_info(w!("slice"))? },
        (Type::Other(1234), 4)
    );

    key.set_value("hstring", &Value::from(h!("HSTRING")))?;
    assert_eq!(key.get_type("hstring")?, Type::String);
    assert_eq!(key.get_value("hstring")?, Value::from(h!("HSTRING")));
    assert_eq!(key.get_string("hstring")?, "HSTRING");
    assert_eq!(HSTRING::try_from(key.get_value("hstring")?)?, "HSTRING");

    assert_eq!(
        unsafe { key.raw_get_info(w!("hstring"))? },
        (Type::String, 16)
    );

    let abc = Value::from("abc");
    assert_eq!(abc.as_wide(), &[97, 98, 99, 0]);
    let abc = Value::from(h!("abcd"));
    assert_eq!(abc.as_wide(), &[97, 98, 99, 100, 0]);

    Ok(())
}
