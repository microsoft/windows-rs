#![cfg(windows)]
use windows::{combaseapi::*, core::*, objidlbase::*, xmllite::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(Default::default(), true)?;

        let writer: IXmlWriter = CreateXmlWriter(None)?;
        writer.SetOutput(&stream).ok()?;

        writer.WriteStartDocument(XmlStandalone_Omit).ok()?;
        writer.WriteStartElement(None, w!("html"), None).ok()?;
        writer
            .WriteElementString(
                None,
                w!("head"),
                None,
                w!("The quick brown fox jumps over the lazy dog"),
            )
            .ok()?;
        writer.WriteStartElement(None, w!("body"), None).ok()?;
        writer.WriteChars(None).ok()?;
        writer.WriteChars(Some(&[0x52, 0x75, 0x73, 0x74])).ok()?;
        writer.WriteEndDocument().ok()?;
        writer.Flush().ok()?;

        let mut pos = 0;
        stream
            .Seek(0, STREAM_SEEK_SET as u32, Some(&mut pos))
            .ok()?;
        assert_eq!(pos, 0);

        let reader: IXmlReader = CreateXmlReader(None)?;
        reader.SetInput(&stream).ok()?;

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_XmlDeclaration);

        let mut name = PCWSTR::null();
        let mut name_len = 0;

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            "html"
        );

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            "head"
        );

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Text);

        let mut message = Vec::new();
        let mut chunk: [u16; 10] = std::mem::zeroed();
        let mut chars_read = 0;
        let mut read_count = 0;

        while reader.ReadValueChunk(&mut chunk, &mut chars_read).is_ok() && chars_read > 0 {
            message.extend_from_slice(&chunk[0..chars_read as usize]);
            read_count += 1;
        }

        assert_eq!(read_count, 5);
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(message.as_ptr(), message.len())),
            "The quick brown fox jumps over the lazy dog"
        );

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_EndElement);

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Text);

        reader.ReadValueChunk(&mut chunk, &mut chars_read).ok()?;
        assert_eq!(chars_read, 4);
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(
                chunk.as_ptr(),
                chars_read as usize
            )),
            "Rust"
        );

        Ok(())
    }
}

#[test]
fn lite() -> Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(Default::default(), true)?;

        let writer: IXmlWriterLite = CreateXmlWriter(None)?;
        writer.SetOutput(&stream).ok()?;

        writer.WriteStartElement(&HSTRING::from("html")).ok()?;
        writer
            .WriteAttributeString(&HSTRING::from("no-value"), None)
            .ok()?;
        writer
            .WriteAttributeString(&HSTRING::from("with-value"), Some(&HSTRING::from("value")))
            .ok()?;
        writer.WriteEndElement(&HSTRING::from("html")).ok()?;
        writer.Flush().ok()?;

        let mut pos = 0;
        stream
            .Seek(0, STREAM_SEEK_SET as u32, Some(&mut pos))
            .ok()?;
        assert_eq!(pos, 0);

        let reader: IXmlReader = CreateXmlReader(None)?;
        reader.SetInput(&stream).ok()?;

        let mut name = PCWSTR::null();
        let mut name_len = 0;

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            "html"
        );

        assert_eq!(reader.GetAttributeCount()?, 2);
        reader.MoveToFirstAttribute().ok()?;

        reader.GetLocalName(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            "no-value"
        );

        reader.GetValue(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            ""
        );

        reader.MoveToNextAttribute().ok()?;

        reader.GetLocalName(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            "with-value"
        );

        reader.GetValue(&mut name, Some(&mut name_len)).ok()?;
        assert_eq!(
            String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as usize)),
            "value"
        );

        Ok(())
    }
}
