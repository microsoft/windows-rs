winrt::import!(
    dependencies
        os
    types
        windows::data::xml::dom::XmlDocument
        windows::foundation::Uri
);

use windows::data::xml::dom::XmlDocument;
use windows::foundation::Uri;

// Simple test to validate debugging works
#[test]
fn debugging() -> winrt::Result<()> {
    let doc = XmlDocument::new()?;
    let url = Uri::create_uri("http://kennykerr.ca")?;

    assert!(format!("{:?}", doc).starts_with("XmlDocument("));
    assert_eq!("http://kennykerr.ca/", &format!("{:?}", url));

    Ok(())
}
