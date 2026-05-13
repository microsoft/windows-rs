#![cfg(windows)]
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

    // make sure there's some zeros in the middle in case someone adds some
    // null-checks that shouldn't be happening for this Type.
    let test_bytes = &[
        0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9A, 0xBC,
    ];
    key.set_bytes("binary", Type::Bytes, test_bytes)?;
    assert_eq!(key.get_type("binary")?, Type::Bytes);
    assert_eq!(key.get_bytes("binary")?, test_bytes.to_vec());

    // make sure you can't get a non-bytes value as bytes
    key.set_string("string_value", "not bytes")?;
    assert!(key.get_bytes("string_value").is_err());

    Ok(())
}
