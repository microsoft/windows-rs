use windows::core::HSTRING;
use windows::Data::Xml::Dom::XmlDocument;

fn main() -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    println!("{}", root.InnerText()?);

    Ok(())
}
