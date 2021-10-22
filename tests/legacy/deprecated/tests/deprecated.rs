use test_deprecated::Windows::ApplicationModel::Contacts::KnownContactField;

#[test]
fn test() -> windows::runtime::Result<()> {
    assert_eq!(KnownContactField::Email()?, "email");

    Ok(())
}
