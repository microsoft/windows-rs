use windows_registry::*;

#[test]
fn value() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\value";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_value("u32", &Value::try_from(123u32)?)?;
    assert_eq!(key.get_type("u32")?, Type::U32);
    assert_eq!(key.get_value("u32")?, Value::try_from(123u32)?);
    assert_eq!(key.get_u32("u32")?, 123u32);
    assert_eq!(key.get_u64("u32")?, 123u64);

    key.set_value("u64", &Value::try_from(123u64)?)?;
    assert_eq!(key.get_type("u64")?, Type::U64);
    assert_eq!(key.get_value("u64")?, Value::try_from(123u64)?);
    assert_eq!(key.get_u32("u64")?, 123u32);
    assert_eq!(key.get_u64("u64")?, 123u64);

    key.set_value("string", &Value::try_from("string")?)?;
    assert_eq!(key.get_type("string")?, Type::String);
    assert_eq!(key.get_value("string")?, Value::try_from("string")?);
    assert_eq!(key.get_string("string")?, "string");

    let mut value = Value::try_from("expand")?;
    value.set_ty(Type::ExpandString);
    assert_eq!(value.ty(), Type::ExpandString);
    key.set_value("expand", &value)?;
    assert_eq!(key.get_type("expand")?, Type::ExpandString);
    assert_eq!(key.get_value("expand")?, value);
    assert_eq!(key.get_string("expand")?, "expand");

    key.set_value("bytes", &Value::try_from([1u8, 2u8, 3u8])?)?;
    assert_eq!(key.get_type("bytes")?, Type::Bytes);
    assert_eq!(key.get_value("bytes")?, Value::try_from([1, 2, 3])?);

    let mut value = Value::try_from([1u8, 2u8, 3u8, 4u8].as_slice())?;
    value.set_ty(Type::Other(1234));
    key.set_value("slice", &value)?;
    assert_eq!(key.get_type("slice")?, Type::Other(1234));
    assert_eq!(key.get_value("slice")?, value);

    Ok(())
}
