use test_winrt::Windows::Foundation::IStringable;
use windows::Interface;

#[test]
fn interface() -> windows::Result<()> {
    assert_eq!(
        IStringable::IID,
        windows::Guid::from("96369F54-8EB6-48F0-ABCE-C1B211E627C3")
    );

    // TODO: Find an example where the default constructor is not exclusive.
    Ok(())
}
