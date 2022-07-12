use windows::Data::Xml::Dom::XmlDocument;
use windows::*;

fn main() -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(w!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    println!("{}", root.InnerText()?);

    Ok(())
}
