use std::convert::*;
use winrt::AbiTransferable;
use winrt::ComInterface;

#[test]
fn non_generic() -> winrt::Result<()> {
    use winrt::{AsyncStatus, IAsyncAction};
    type Handler = winrt::AsyncActionCompletedHandler;

    assert_eq!(
        Handler::iid(),
        winrt::Guid::from("A4ED5C81-76C9-40BD-8BE6-B1D90FB20AE7")
    );

    let d = Handler::default();
    assert!(d.is_null());

    let (tx, rx) = std::sync::mpsc::channel();

    let d = Handler::new(move |info, status| {
        tx.send(true).unwrap();
        assert!(info.is_null());
        assert!(status == AsyncStatus::Completed);
        Ok(())
    });

    // TODO: delegates are function objects (logically) ans we should be able
    // to call them without an explicit `invoke` method e.g. `d(args);`
    d.invoke(IAsyncAction::default(), AsyncStatus::Completed)?;

    assert!(rx.recv().unwrap());

    Ok(())
}

#[test]
fn generic() -> winrt::Result<()> {
    use winrt::Uri;

    type Handler = winrt::TypedEventHandler<Uri, i32>;

    // TODO: Handler::IID is not correct for generic types

    let d = Handler::default();
    assert!(d.is_null());

    let uri = Uri::create_uri("http://kennykerr.ca")?;
    let (tx, rx) = std::sync::mpsc::channel();

    let uri_clone = uri.clone();
    let d = Handler::new(move |sender, port| {
        tx.send(true).unwrap();
        assert!(uri_clone.get_abi() == sender.get_abi());

        // TODO: ideally primitives would be passed by value
        assert!(*port == 80);
        Ok(())
    });

    let port = uri.port()?;
    d.invoke(uri, port)?;

    assert!(rx.recv().unwrap());

    Ok(())
}

#[test]
fn event() -> winrt::Result<()> {
    use winrt::collections::*;
    use winrt::*;

    let set = PropertySet::new()?;
    let (tx, rx) = std::sync::mpsc::channel();

    let set_clone = set.clone();
    // TODO: Should be able to elide the delegate construction and simply say:
    // set.map_changed(|sender, args| {...})?;
    set.map_changed(
        MapChangedEventHandler::<winrt::HString, winrt::Object>::new(move |sender, args| {
            tx.send(true).unwrap();
            let set = set_clone.clone();
            let map: IObservableMap<winrt::HString, winrt::Object> = set.into();
            assert!(map.get_abi() == sender.get_abi());
            assert!(args.key()? == "A");
            assert!(args.collection_change()? == CollectionChange::ItemInserted);
            Ok(())
        }),
    )?;

    set.insert("A", Object::try_from(1_u32)?)?;

    assert!(rx.recv().unwrap());

    Ok(())
}
