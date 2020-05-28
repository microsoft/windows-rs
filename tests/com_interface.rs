winrt::import!(
    dependencies
        os
    modules
        "windows.foundation"
);

#[test]
fn com_interface() -> winrt::Result<()> {
    use winrt::ComInterface;
    use winrt::RuntimeType;

    use windows::foundation::IStringable;
    use windows::foundation::IUriRuntimeClass;
    use windows::foundation::Uri;

    let uri = &Uri::create_uri("http://kennykerr.ca")?;

    // The class and the default interface have the same vtable types, which
    // means you can compare them directly.
    let u: IUriRuntimeClass = uri.into();
    assert!(u.abi() == uri.abi());

    // The class and the non-default interface have different vtable types, which
    // means we need to cast in order to compare their pointers (which won't match).
    let s: IStringable = uri.into();
    unsafe {
        assert!(
            s.abi().unwrap().cast::<winrt::IUnknown>()
                != uri.abi().unwrap().cast::<winrt::IUnknown>()
        );
    }
    // Here two different values of the same class won't share the same value as
    // they are unique instances even though they have the same vtable layout.
    let other = &Uri::create_uri("http://microsoft.com")?;
    assert!(uri.abi() != other.abi());

    // A default constructor class will always have a null vtable pointer.
    let uri = Uri::default();
    assert!(uri.is_null());

    Ok(())
}
