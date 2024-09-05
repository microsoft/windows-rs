use windows_registry::*;
use windows_strings::*;

#[test]
fn bytes() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\bytes";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_bytes("bytes", Type::Bytes, &[1, 2, 3])?;
    assert_eq!(key.get_type("bytes")?, Type::Bytes);

    let value = key.get_value("bytes")?;
    assert_eq!(value.ty(), Type::Bytes);
    assert_eq!(*value, [1, 2, 3]);

    key.set_bytes("other", Type::Other(1234), &[1, 2, 3, 4])?;
    assert_eq!(key.get_type("other")?, Type::Other(1234));

    let value = key.get_value("other")?;
    assert_eq!(value.ty(), Type::Other(1234));
    assert_eq!(*value, [1, 2, 3, 4]);

    assert_eq!(
        unsafe { key.raw_get_info(w!("other"))? },
        (Type::Other(1234), 4)
    );

    Ok(())
}
