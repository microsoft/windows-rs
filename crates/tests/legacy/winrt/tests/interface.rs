use test_winrt::Windows::Foundation::IStringable;
use windows::runtime::Interface;

#[test]
fn interface() -> windows::runtime::Result<()> {
    assert_eq!(IStringable::IID, windows::runtime::GUID::from("96369F54-8EB6-48F0-ABCE-C1B211E627C3"));

    // TODO: Find an example where the default constructor is not exclusive.
    Ok(())
}
