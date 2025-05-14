use core::convert::*;

use windows::{core::Interface, Foundation::Collections::PropertySet, Foundation::*};

use windows_collections::*;

#[test]
fn uri() -> windows::core::Result<()> {
    let uri = Uri::CreateUri(&windows::core::HSTRING::from(
        "http://kennykerr.ca?A=1&B=2&C=3",
    ))?;
    let query = uri.QueryParsed()?;

    assert!(query.Size()? == 3);

    assert!(query.GetAt(0)?.Name()? == "A");
    assert!(query.GetAt(0)?.Value()? == "1");

    assert!(query.GetAt(1)?.Name()? == "B");
    assert!(query.GetAt(1)?.Value()? == "2");

    assert!(query.GetAt(2)?.Name()? == "C");
    assert!(query.GetAt(2)?.Value()? == "3");

    // This tests the IntoIterator support on the WwwFormUrlDecoder returned by QueryParsed.
    // It should be using the fast IVectorView iterator.

    let mut result = String::new();

    for entry in query {
        result.push_str(&entry.Value()?.to_string());
    }

    assert!(result == "123");

    // This tests the ability to treat the WwwFormUrlDecoder as an IIterable<T> and use
    // the slower IIterator<T> iterator.

    let iterable: IIterable<IWwwFormUrlDecoderEntry> = uri.QueryParsed()?.cast()?;

    let mut result = String::new();

    for entry in iterable {
        result.push_str(&entry.Name()?.to_string());
    }

    assert!(result == "ABC");

    // This tests the ability to treat the WwwFormUrlDecoder as an IVectorView<T> and use
    // the fast IVectorView iterator directly.

    let iterable: IVectorView<IWwwFormUrlDecoderEntry> = uri.QueryParsed()?.cast()?;

    let mut result = String::new();

    for entry in iterable {
        result.push_str(&entry.Value()?.to_string());
    }

    assert!(result == "123");

    Ok(())
}

#[test]
fn property_set() -> windows::core::Result<()> {
    // The PropertySet class implements IIterable<IKeyValuePair<HSTRING, IInspectable>> so the following
    // for loop will exercise the IIterator<T> iterator implicitly.

    let set = PropertySet::new()?;

    set.Insert(
        &windows::core::HSTRING::from("A"),
        &PropertyValue::CreateInt32(1)?,
    )?;
    set.Insert(
        &windows::core::HSTRING::from("B"),
        &PropertyValue::CreateInt32(2)?,
    )?;
    set.Insert(
        &windows::core::HSTRING::from("C"),
        &PropertyValue::CreateInt32(3)?,
    )?;

    assert!(set.Size()? == 3);

    let mut keys = Vec::new();
    let mut values = 0;

    for pair in &set {
        keys.push(pair.Key()?.to_string());
        values += pair.Value()?.cast::<IReference<i32>>()?.Value()?;
    }
    assert!(set.Size()? == 3);

    keys.sort();
    assert!(String::from_iter(keys) == "ABC");
    assert!(values == 6);

    Ok(())
}
