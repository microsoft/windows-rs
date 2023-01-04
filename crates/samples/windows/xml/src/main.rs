use windows::core::*;
use windows::Data::Xml::Dom::XmlDocument;

fn main() -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    println!("{}", root.InnerText()?);

    Ok(())
}
