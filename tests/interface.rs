use winrt::foundation::{IStringable, Uri};
use winrt::ComInterface;

#[test]
fn non_generic() -> winrt::Result<()> {
    assert_eq!(
        IStringable::IID,
        winrt::Guid::from("96369F54-8EB6-48F0-ABCE-C1B211E627C3")
    );

    let s = IStringable::default();
    assert!(s.is_null());

    let s: IStringable = Uri::create_uri("http://kennykerr.ca")?.into();

    let s = s.to_string()?;

    assert!(s == "http://kennykerr.ca/");

    Ok(())
}
