winrt::import!(
    dependencies
        "os"
    modules
        "windows.data.xml.dom"
);

// Simple test to validate that default constructors are projected as static `new` methods.
#[test]
fn xml() -> winrt::Result<()> {
    use windows::data::xml::dom::*;

    let doc = XmlDocument::new()?;

    doc.load_xml("<html>hello world</html>")?;
    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");

    Ok(())
}
