#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
mod imp {
    use windows::core::*;
    use windows::Data::Xml::Dom::XmlDocument;

    pub fn main() -> windows::core::Result<()> {
        let doc = XmlDocument::new()?;
        doc.LoadXml(h!("<html>hello world</html>"))?;

        let root = doc.DocumentElement()?;
        assert!(root.NodeName()? == "html");
        println!("{:?}", root.InnerText()?);

        Ok(())
    }
}

#[cfg(windows)]
fn main() -> impl std::process::Termination {
    imp::main()
}
