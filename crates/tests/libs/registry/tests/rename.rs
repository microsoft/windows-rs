use windows_registry::*;

#[test]
fn string() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\rename";
    _ = CURRENT_USER.remove_tree(test_key);
    let test_key = CURRENT_USER.create(test_key)?;

    let from = test_key.create("from")?;
    from.set_string("key", "renamed value")?;
    drop(from);

    test_key.rename("from", "to")?;

    let renamed = test_key.open("to")?;
    assert_eq!(renamed.get_string("key")?, "renamed value");

    CURRENT_USER.rename("software\\windows-rs\\tests\\rename\\to", "too")?;

    Ok(())
}
