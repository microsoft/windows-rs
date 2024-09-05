use windows_registry::*;
use windows_strings::*;

#[test]
fn u32() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\u32";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_u32("u32", 123u32)?;
    assert_eq!(key.get_type("u32")?, Type::U32);
    assert_eq!(key.get_u32("u32")?, 123u32);
    assert_eq!(key.get_u64("u32")?, 123u64);

    assert_eq!(unsafe { key.raw_get_info(w!("u32"))? }, (Type::U32, 4));

    Ok(())
}
