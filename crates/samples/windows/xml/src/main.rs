#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() -> windows::core::Result<()> {
    use windows::Data::Xml::Dom::XmlDocument;
    use windows::core::*;

    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    println!("{:?}", root.InnerText()?);

    Ok(())
}
