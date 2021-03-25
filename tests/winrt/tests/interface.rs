use test_winrt::Windows::Foundation::{IStringable, Uri};
use windows::{Abi, Interface};

#[test]
fn interface() -> windows::Result<()> {
    assert_eq!(
        IStringable::IID,
        windows::Guid::from("96369F54-8EB6-48F0-ABCE-C1B211E627C3")
    );

    let uri = &Uri::CreateUri("http://kennykerr.ca")?;

    // TODO: Find an example where the default constructor is not exclusive.

    // The class and the non-default interface have different vtable types, which
    // means we need to cast in order to compare their pointers (which won't match).
    let s: IStringable = uri.into();
    assert!(s.ToString()? == "http://kennykerr.ca/");

    assert!(s.abi() != uri.abi());

    // Here two different values of the same class won't share the same value as
    // they are unique instances even though they have the same vtable layout.
    let other = &Uri::CreateUri("http://microsoft.com")?;
    assert!(uri.abi() != other.abi());

    Ok(())
}
