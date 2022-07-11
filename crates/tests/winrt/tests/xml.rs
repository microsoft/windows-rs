use windows::core::HSTRING;
use windows::Data::Xml::Dom::XmlDocument;

// Simple test to validate that default constructors are projected as static `new` methods.
#[test]
fn xml() -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;

    doc.LoadXml(&HSTRING::from("<html>hello world</html>"))?;
    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    Ok(())
}
