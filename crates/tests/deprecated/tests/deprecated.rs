use windows::ApplicationModel::Contacts::KnownContactField;

#[test]
fn test() -> windows::core::Result<()> {
    assert_eq!(KnownContactField::Email()?, "email");

    Ok(())
}
