winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation.collections"
);

use windows::foundation::*;
use windows::foundation::collections::*;

#[test]
fn uri() -> winrt::Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca?A=1&B=2&C=3")?;
    let query = uri.query_parsed()?;

    assert!(query.size()? == 3);

    assert!(query.get_at(0)?.name()? == "A");
    assert!(query.get_at(0)?.value()? == "1");

    assert!(query.get_at(1)?.name()? == "B");
    assert!(query.get_at(1)?.value()? == "2");

    assert!(query.get_at(2)?.name()? == "C");
    assert!(query.get_at(2)?.value()? == "3");

    // TODO: still need to figure out how to get this not to consume the `query` value.

    // This tests the IntoIterator support on the WwwFormUrlDecoder returned by query_parsed.
    // It should be using the fast IVectorView iterator.

    let mut result = String::new();

    for entry in query { 
        result.push_str(&entry.value()?.to_string());
    }

    assert!(result == "123");

    // This tests the ability to treat the WwwFormUrlDecoder as an IIterable<T> and use
    // the slower IIterator<T> iterator.

    let iterable: IIterable<IWwwFormUrlDecoderEntry> = uri.query_parsed()?.into();

    let mut result = String::new();

    for entry in iterable { 
        result.push_str(&entry.name()?.to_string());
    }

    assert!(result == "ABC");

    // This tests the ability to treat the WwwFormUrlDecoder as an IVectorView<T> and use
    // the fast IVectorView iterator directly.

    let iterable: IVectorView<IWwwFormUrlDecoderEntry> = uri.query_parsed()?.into();

    let mut result = String::new();

    for entry in iterable { 
        result.push_str(&entry.value()?.to_string());
    }

    assert!(result == "123");

    Ok(())
}

#[test]
fn property_set() -> winrt::Result<()> {
    let set = PropertySet::new()?;

    set.insert("A", PropertyValue::create_uint32(1)?)?;
    set.insert("B", PropertyValue::create_uint32(2)?)?;
    set.insert("C", PropertyValue::create_uint32(3)?)?;

    assert!(set.size()? == 3);

    let mut result = String::new();

    for pair in set { 
        result.push_str(&pair.key()?.to_string());
    }

    println!("result {}", result);
    assert!(result == "ABC");

    Ok(())
}