use std::convert::*;
use std::iter::FromIterator;

use test_winrt::{
    Windows::Foundation::Collections::{IIterable, IVectorView, PropertySet},
    Windows::Foundation::{IWwwFormUrlDecoderEntry, Uri},
};

#[test]
fn uri() -> windows::Result<()> {
    let uri = Uri::CreateUri("http://kennykerr.ca?A=1&B=2&C=3")?;
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

    let iterable: IIterable<IWwwFormUrlDecoderEntry> = uri.QueryParsed()?.into();

    let mut result = String::new();

    for entry in iterable {
        result.push_str(&entry.Name()?.to_string());
    }

    assert!(result == "ABC");

    // This tests the ability to treat the WwwFormUrlDecoder as an IVectorView<T> and use
    // the fast IVectorView iterator directly.

    let iterable: IVectorView<IWwwFormUrlDecoderEntry> = uri.QueryParsed()?.into();

    let mut result = String::new();

    for entry in iterable {
        result.push_str(&entry.Value()?.to_string());
    }

    assert!(result == "123");

    Ok(())
}

#[test]
fn property_set() -> windows::Result<()> {
    // The PropertySet class implements IIterable<IKeyValuePair<HString, IInspectable>> so the following
    // for loop will excercise the IIterator<T> iterator implicitly.

    let set = PropertySet::new()?;

    set.Insert("A", windows::IInspectable::try_from(1)?)?;
    set.Insert("B", windows::IInspectable::try_from(2)?)?;
    set.Insert("C", windows::IInspectable::try_from(3)?)?;

    assert!(set.Size()? == 3);

    let mut keys = Vec::new();
    let mut values = 0;

    for pair in &set {
        keys.push(pair.Key()?.to_string());
        values += i32::try_from(pair.Value()?)?;
    }
    assert!(set.Size()? == 3);

    keys.sort();
    assert!(String::from_iter(keys) == "ABC");
    assert!(values == 6);

    Ok(())
}
