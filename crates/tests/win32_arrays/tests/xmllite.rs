use windows::{core::*, Win32::Data::Xml::XmlLite::*, Win32::System::Com::StructuredStorage::*, Win32::System::Com::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(0, true)?;

        let mut writer: Option<IXmlWriter> = None;
        CreateXmlWriter(&IXmlWriter::IID, &mut writer as *mut _ as _, None)?;
        let writer = writer.unwrap();
        writer.SetOutput(&stream)?;

        writer.WriteStartDocument(XmlStandalone_Omit)?;
        writer.WriteStartElement(None, "html", None)?;
        writer.WriteElementString(None, "body", None, "The quick brown fox jumps over the lazy dog")?;
        writer.WriteEndDocument()?;
        writer.Flush()?;

        let pos = stream.Seek(0, STREAM_SEEK_SET)?;
        assert_eq!(pos, 0);

        let mut reader: Option<IXmlReader> = None;
        CreateXmlReader(&IXmlReader::IID, &mut reader as *mut _ as _, None)?;
        let reader = reader.unwrap();
        reader.SetInput(&stream)?;

        let node_type = reader.Read()?;
        assert_eq!(node_type, XmlNodeType_XmlDeclaration);

        let mut name = PWSTR::default();
        let mut name_len = 0;

        let node_type = reader.Read()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, &mut name_len)?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "html");

        let node_type = reader.Read()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, &mut name_len)?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "body");

        let node_type = reader.Read()?;
        assert_eq!(node_type, XmlNodeType_Text);

        let mut message = Vec::new();
        let mut chunk: [u16; 10] = std::mem::zeroed();
        let mut chars_read = 0;
        let mut read_count = 0;

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/816
        while reader.ReadValueChunk(&mut chunk, &mut chars_read).is_ok() && chars_read > 0 {
            message.extend_from_slice(&chunk[0..chars_read as _]);
            read_count += 1;
        }

        assert_eq!(read_count, 5);
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(message.as_ptr(), message.len())), "The quick brown fox jumps over the lazy dog");
        Ok(())
    }
}
