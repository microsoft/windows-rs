winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

use winrt::ComInterface;

#[test]
fn non_generic() -> winrt::Result<()> {
    use windows::foundation::AsyncStatus;
    use windows::foundation::IAsyncAction;
    type Handler = windows::foundation::AsyncActionCompletedHandler;

    assert_eq!(
        Handler::IID,
        winrt::Guid::from("A4ED5C81-76C9-40BD-8BE6-B1D90FB20AE7")
    );

    let d = Handler::default();
    assert!(d.is_null());

    let mut invoked = false;

    let d = Handler::new(|info, status| {
        invoked = true;
        assert!(info.is_null());
        assert!(status == AsyncStatus::Completed);
        Ok(())
    });

    // TODO: delegates are function objects (logically) ans we should be able
    // to call them without an explicit `invoke` method e.g. `d(args);`
    d.invoke(IAsyncAction::default(), AsyncStatus::Completed)?;

    assert!(invoked);

    Ok(())
}

#[test]
fn generic() -> winrt::Result<()> {
    use windows::foundation::Uri;
    type Handler = windows::foundation::TypedEventHandler<Uri, i32>;

    // TODO: Handler::IID is not correct for generic types

    let d = Handler::default();
    assert!(d.is_null());

    let uri = &Uri::create_uri("http://kennykerr.ca")?;
    let mut invoked = false;

    let d = Handler::new(|sender, port| {
        invoked = true;
        assert!(uri.as_raw() == sender.as_raw());

        // TODO: ideally primitives would be passed by value
        assert!(*port == 80);
        Ok(())
    });

    d.invoke(uri, uri.port()?)?;

    assert!(invoked);

    Ok(())
}
