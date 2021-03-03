use bindings::{windows::data::xml::dom::XmlDocument};

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    println!("{}", root.inner_text()?);

    Ok(())
}
