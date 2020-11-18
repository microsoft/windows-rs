use winrt::foundation::{IStringable, IUriRuntimeClass, Uri};
use winrt::{Abi, Interface};

#[test]
fn interface() -> winrt::Result<()> {
    assert_eq!(
        IStringable::IID,
        winrt::Guid::from("96369F54-8EB6-48F0-ABCE-C1B211E627C3")
    );

    let uri = &Uri::create_uri("http://kennykerr.ca")?;

    // The class and the default interface have the same vtable types, which
    // means you can compare them directly.
    let u: IUriRuntimeClass = uri.into();
    assert!(u.abi() == uri.abi());
    assert!(u.domain()? == "kennykerr.ca");

    // The class and the non-default interface have different vtable types, which
    // means we need to cast in order to compare their pointers (which won't match).
    let s: IStringable = uri.into();
    assert!(s.to_string()? == "http://kennykerr.ca/");

    assert!(s.abi() != uri.abi());

    // Here two different values of the same class won't share the same value as
    // they are unique instances even though they have the same vtable layout.
    let other = &Uri::create_uri("http://microsoft.com")?;
    assert!(uri.abi() != other.abi());

    Ok(())
}
