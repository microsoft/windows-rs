use bindings::Windows::Data::Xml::Dom::XmlDocument;

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    println!("{}", root.InnerText()?);

    Ok(())
}
