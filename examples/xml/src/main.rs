use bindings::{windows::data::xml::dom::XmlDocument, windows::Result};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    println!("{}", root.inner_text()?);

    Ok(())
}
