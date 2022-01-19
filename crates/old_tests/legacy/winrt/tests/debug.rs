use test_winrt::{Windows::Data::Xml::Dom::XmlDocument, Windows::Foundation::Uri};

// Simple test to validate debugging works
#[test]
fn debugging() -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;
    let url = Uri::CreateUri("http://kennykerr.ca")?;

    assert!(format!("{:?}", doc).ends_with(" Windows.Data.Xml.Dom.XmlDocument)"));
    assert!(&format!("{:?}", url).ends_with(" http://kennykerr.ca/)"));

    Ok(())
}
