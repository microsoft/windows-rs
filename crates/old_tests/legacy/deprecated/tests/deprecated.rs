use test_deprecated::Windows::ApplicationModel::Contacts::KnownContactField;

#[test]
fn test() -> windows::core::Result<()> {
    assert_eq!(KnownContactField::Email()?, "email");

    Ok(())
}
