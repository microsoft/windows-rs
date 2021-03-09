use test_winrt::{windows::data::xml::dom::XmlDocument, windows::foundation::Uri};

// Simple test to validate debugging works
#[test]
fn debugging() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    let url = Uri::create_uri("http://kennykerr.ca")?;

    assert!(format!("{:?}", doc).ends_with(" Windows.Data.Xml.Dom.XmlDocument)"));
    assert!(&format!("{:?}", url).ends_with(" http://kennykerr.ca/)"));

    Ok(())
}
