use windows_metadata::{HasAttributes, Value, reader};

#[test]
fn align_short_form_preserves_metadata() {
    let source = r#"
        #[win32] mod Test {
            #[packed(4)] #[align(16)] struct Aligned {
                #[Windows::Win32::Metadata::Alignment(16)]
                value: u32,
            }
        }
    "#;
    let compact = source.replace("Windows::Win32::Metadata::Alignment", "align");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let output = std::path::Path::new(env!("OUT_DIR"));
    let mut expected = None;
    for (index, source) in [source, &compact].iter().enumerate() {
        let winmd = output.join(format!("align-short-form-{index}.winmd"));
        let rdl = winmd.with_extension("rdl");
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(source)
            .output(&winmd)
            .write()
            .unwrap();
        let metadata = reader::Index::read(&winmd).unwrap();
        let aligned = metadata.expect("Test", "Aligned");
        assert_eq!(aligned.class_layout().unwrap().packing_size(), 4);
        for attr in [
            aligned.find_attribute("AlignmentAttribute").unwrap(),
            aligned
                .fields()
                .next()
                .unwrap()
                .find_attribute("AlignmentAttribute")
                .unwrap(),
        ] {
            assert_eq!(attr.value(), vec![(String::new(), Value::I32(16))]);
        }
        windows_rdl::writer()
            .input(&winmd)
            .filter("Test")
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(rdl).unwrap();
        assert_eq!(text.matches("#[align(16)]").count(), 2);
        assert_eq!(text.matches("#[packed(4)]").count(), 1);
        if let Some(expected) = &expected {
            assert_eq!(&text, expected);
        }
        expected = Some(text);
    }
}
