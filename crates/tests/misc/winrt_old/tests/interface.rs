use windows::core::Interface;
use windows::Foundation::IStringable;

#[test]
fn interface() -> windows::core::Result<()> {
    assert_eq!(
        IStringable::IID,
        windows::core::GUID::try_from("96369F54-8EB6-48F0-ABCE-C1B211E627C3")?
    );

    // TODO: Find an example where the default constructor is not exclusive.
    Ok(())
}
