#![cfg(windows)]
use windows::Data::Xml::Dom::XmlDocument;
use windows::core::HSTRING;

#[test]
fn xml() -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;

    doc.LoadXml(&HSTRING::from("<html>hello world</html>"))?;
    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    Ok(())
}
