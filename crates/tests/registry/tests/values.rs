use windows_registry::*;
use windows_result::*;

#[test]
fn values() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\values";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER.create(test_key)?;
    key.set_u32("u32", 123)?;
    key.set_u64("u64", 456)?;
    key.set_string("string", "hello")?;
    key.set_bytes("bytes", &[1u8, 2u8, 2u8])?;
    key.set_multi_string("multi", &["hello", "world"])?;

    assert_eq!(key.get_u32("u32")?, 123u32);
    assert_eq!(key.get_u64("u64")?, 456u64);
    assert_eq!(key.get_string("string")?, "hello".to_string());
    assert_eq!(key.get_bytes("bytes")?, vec![1u8, 2u8, 2u8]);
    assert_eq!(
        key.get_multi_string("multi")?,
        vec!["hello".to_string(), "world".to_string()]
    );

    let err = key.get_u32("string").unwrap_err();
    assert_eq!(err.code(), HRESULT(0x8007000Du32 as i32)); // HRESULT_FROM_WIN32(ERROR_INVALID_DATA)
    assert_eq!(err.message(), "The data is invalid.");

    assert_eq!(key.get_value("u32")?, Value::U32(123));
    assert_eq!(key.get_value("u64")?, Value::U64(456));
    assert_eq!(key.get_value("string")?, Value::String("hello".to_string()));
    assert_eq!(key.get_value("bytes")?, Value::Bytes(vec![1u8, 2u8, 2u8]));
    assert_eq!(
        key.get_value("multi")?,
        Value::MultiString(vec!["hello".to_string(), "world".to_string()])
    );

    let names: Vec<(String, Value)> = key.values()?.collect();

    assert_eq!(
        names,
        [
            ("u32".to_string(), Value::U32(123)),
            ("u64".to_string(), Value::U64(456)),
            ("string".to_string(), Value::String("hello".to_string())),
            ("bytes".to_string(), Value::Bytes(vec![1u8, 2u8, 2u8])),
            (
                "multi".to_string(),
                Value::MultiString(vec!["hello".to_string(), "world".to_string()])
            ),
        ]
    );

    key.remove_value("string")?;
    key.remove_value("multi")?;
    let names: Vec<_> = key.values()?.collect();

    assert_eq!(
        names,
        [
            ("u32".to_string(), Value::U32(123)),
            ("u64".to_string(), Value::U64(456)),
            ("bytes".to_string(), Value::Bytes(vec![1u8, 2u8, 2u8])),
        ]
    );

    Ok(())
}
