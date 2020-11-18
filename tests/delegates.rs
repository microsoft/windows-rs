use std::convert::*;
use winrt::foundation::collections::{
    CollectionChange, IObservableMap, MapChangedEventHandler, PropertySet,
};
use winrt::foundation::{AsyncActionCompletedHandler, AsyncStatus, TypedEventHandler, Uri};
use winrt::{Abi, Interface};

#[test]
fn non_generic() -> winrt::Result<()> {
    type Handler = AsyncActionCompletedHandler;

    assert_eq!(
        Handler::IID,
        winrt::Guid::from("A4ED5C81-76C9-40BD-8BE6-B1D90FB20AE7")
    );

    let (tx, rx) = std::sync::mpsc::channel();

    let d = Handler::new(move |info, status| {
        tx.send(true).unwrap();
        assert!(info.is_none());
        assert!(status == AsyncStatus::Completed);
        Ok(())
    });

    // TODO: delegates are function objects (logically) and we should be able
    // to call them without an explicit `invoke` method e.g. `d(args);`
    d.invoke(None, AsyncStatus::Completed)?;

    assert!(rx.recv().unwrap());

    Ok(())
}

#[test]
fn generic() -> winrt::Result<()> {
    type Handler = TypedEventHandler<Uri, i32>;

    assert_eq!(
        Handler::IID,
        winrt::Guid::from("DAE18EA9-FCF3-5ACF-BCDD-8C354CBA6D23")
    );

    let uri = Uri::create_uri("http://kennykerr.ca")?;
    let (tx, rx) = std::sync::mpsc::channel();

    let uri_clone = uri.clone();
    let d = Handler::new(move |sender, port| {
        tx.send(true).unwrap();
        assert!(uri_clone.abi() == sender.abi());

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
    let set = PropertySet::new()?;
    let (tx, rx) = std::sync::mpsc::channel();

    let set_clone = set.clone();
    // TODO: Should be able to elide the delegate construction and simply say:
    // set.map_changed(|sender, args| {...})?;
    set.map_changed(
        MapChangedEventHandler::<winrt::HString, winrt::Object>::new(move |sender, args| {
            let args = args.as_ref().unwrap();
            tx.send(true).unwrap();
            let set = set_clone.clone();
            let map: IObservableMap<winrt::HString, winrt::Object> = set.into();
            assert!(map.abi() == sender.abi());
            assert!(args.key()? == "A");
            assert!(args.collection_change()? == CollectionChange::ItemInserted);
            Ok(())
        }),
    )?;

    set.insert("A", winrt::Object::try_from(1_u32)?)?;

    assert!(rx.recv().unwrap());

    Ok(())
}
