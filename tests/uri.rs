winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

#[test]
fn uri() -> winrt::Result<()> {
    use windows::foundation::Uri;

    assert_eq!(<Uri as winrt::RuntimeName>::NAME, "Windows.Foundation.Uri");

    assert_eq!(
        <Uri as winrt::ComInterface>::GUID,
        winrt::Guid::from("9E365E57-48B2-4160-956F-C7385120BBFC") // IUriRuntimeClass
    );

    let _null_uri = Uri::default();
    // TODO: how to signal/test empty value?

    let uri = &Uri::create_uri("http://kennykerr.ca?first=101&second=102&third=103")?;

    assert!(uri.domain()? == "kennykerr.ca");
    assert!(uri.port()? == 80);

    // Calls QueryInterface followed by IStringable::ToString under the hood
    assert!(uri.to_string()? == "http://kennykerr.ca/?first=101&second=102&third=103");

    let default: windows::foundation::IUriRuntimeClass = uri.into();
    assert!(default.domain()? == uri.domain()?);

    let stringable: windows::foundation::IStringable = uri.into();
    assert!(stringable.to_string()? == uri.to_string()?);

    // query_parsed returns WwwFormUrlDecoder, which implements IVector<IWwwFormUrlDecoderEntry> so
    // the following code needs to QI for that generic interface in order to inspect the query string.
    let query = uri.query_parsed()?;
    assert!(query.size()? == 3);

    assert!(query.get_at(0)?.name()? == "first");
    assert!(query.get_at(0)?.value()? == "101");

    assert!(query.get_at(1)?.name()? == "second");
    assert!(query.get_at(1)?.value()? == "102");

    assert!(query.get_at(2)?.name()? == "third");
    assert!(query.get_at(2)?.value()? == "103");

    Ok(())
}
