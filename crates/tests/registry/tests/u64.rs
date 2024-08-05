use windows_registry::*;
use windows_strings::*;

#[test]
fn u64() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\u64";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_u64("u64", 123u64)?;
    assert_eq!(key.get_type("u64")?, Type::U64);
    assert_eq!(key.get_u32("u64")?, 123u32);
    assert_eq!(key.get_u64("u64")?, 123u64);

    assert_eq!(unsafe { key.raw_get_info(w!("u64"))? }, (Type::U64, 8));

    Ok(())
}
