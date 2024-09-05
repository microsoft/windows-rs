use windows_registry::*;

#[test]
fn values() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\values";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER.create(test_key)?;
    key.set_u32("u32", 123)?;
    key.set_u64("u64", 456)?;
    key.set_string("string", "hello world")?;

    let names: Vec<(String, Value)> = key.values()?.collect();
    assert_eq!(names.len(), 3);

    assert_eq!(
        names,
        [
            ("u32".to_string(), Value::from(123u32)),
            ("u64".to_string(), Value::from(456u64)),
            ("string".to_string(), Value::from("hello world")),
        ]
    );

    Ok(())
}
