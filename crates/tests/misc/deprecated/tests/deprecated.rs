use windows::ApplicationModel::Contacts::KnownContactField;

#[test]
fn test() {
    assert_eq!(KnownContactField::Email().unwrap(), "email");
}
