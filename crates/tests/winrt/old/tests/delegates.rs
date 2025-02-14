use core::convert::*;

use windows::core::Interface;
use windows::{Foundation::Collections::*, Foundation::*};
use windows_future::*;

#[test]
fn non_generic() -> windows::core::Result<()> {
    type Handler = AsyncActionCompletedHandler;

    assert_eq!(
        Handler::IID,
        windows::core::GUID::try_from("A4ED5C81-76C9-40BD-8BE6-B1D90FB20AE7")?
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
    d.Invoke(None, AsyncStatus::Completed)?;

    assert!(rx.recv().unwrap());

    Ok(())
}

#[test]
fn generic() -> windows::core::Result<()> {
    type Handler = TypedEventHandler<Uri, i32>;

    assert_eq!(
        Handler::IID,
        windows::core::GUID::try_from("DAE18EA9-FCF3-5ACF-BCDD-8C354CBA6D23")?
    );

    let uri = Uri::CreateUri(&windows::core::HSTRING::from("http://kennykerr.ca"))?;
    let (tx, rx) = std::sync::mpsc::channel();

    let d = Handler::new(move |_, port| {
        tx.send(true).unwrap();

        // TODO: ideally primitives would be passed by value
        assert!(*port == 80);
        Ok(())
    });

    let port = uri.Port()?;
    d.Invoke(&uri, port)?;

    assert!(rx.recv().unwrap());

    Ok(())
}

#[test]
fn event() -> windows::core::Result<()> {
    let set = PropertySet::new()?;
    let (tx, rx) = std::sync::mpsc::channel();

    let set_clone = set.clone();
    // TODO: Should be able to elide the delegate construction and simply say:
    // set.MapChanged(|sender, args| {...})?;
    set.MapChanged(&MapChangedEventHandler::<
        windows::core::HSTRING,
        windows::core::IInspectable,
    >::new(move |_, args| {
        let args = args.as_ref().unwrap();
        tx.send(true).unwrap();
        let set = set_clone.clone();
        let _: IObservableMap<windows::core::HSTRING, windows::core::IInspectable> = set.cast()?;
        assert!(args.Key()? == "A");
        assert!(args.CollectionChange()? == CollectionChange::ItemInserted);
        Ok(())
    }))?;

    set.Insert(
        &windows::core::HSTRING::from("A"),
        &PropertyValue::CreateUInt32(1_u32)?,
    )?;

    assert!(rx.recv().unwrap());

    Ok(())
}
