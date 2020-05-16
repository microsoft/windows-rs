winrt::import!(
    dependencies
        os
    modules
        "windows.foundation.collections"
);

use winrt::ComInterface;

#[test]
fn non_generic() -> winrt::Result<()> {
    use windows::foundation::AsyncStatus;
    use windows::foundation::IAsyncAction;
    type Handler = windows::foundation::AsyncActionCompletedHandler;

    assert_eq!(
        Handler::iid(),
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

#[test]
fn event() -> winrt::Result<()> {
    use windows::foundation::collections::*;
    use windows::foundation::*;

    let set = &PropertySet::new()?;
    let mut invoked = false;

    // TODO: Should be able to elide the delegate construction and simply say:
    // set.map_changed(|sender, args| {...})?;
    set.map_changed(
        MapChangedEventHandler::<winrt::HString, winrt::Object>::new(|sender, args| {
            invoked = true;
            let map: IObservableMap<winrt::HString, winrt::Object> = set.into();
            assert!(map.as_raw() == sender.as_raw());
            assert!(args.key()? == "A");
            assert!(args.collection_change()? == CollectionChange::ItemInserted);
            Ok(())
        }),
    )?;

    set.insert("A", PropertyValue::create_uint32(1)?)?;

    assert!(invoked);

    Ok(())
}
