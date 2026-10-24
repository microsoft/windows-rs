use windows_metadata::{HasAttributes, MethodAttributes, Value, reader};

#[test]
fn structural_short_forms_preserve_metadata() {
    let source = r#"
        #[win32] mod Test {
            #[align(16)] struct Aligned {
                #[Windows::Win32::Metadata::Alignment(16)]
                value: u32,
            }
            interface Base {
                fn Call(&self, #[in] value: u32);
            }
            interface Derived: Base {
                #[Windows::Win32::Metadata::NativeOverrideSlot(0u32)]
                fn Call(&self, #[in] value: u32);
            }
        }
    "#;
    let compact = source
        .replace("Windows::Win32::Metadata::Alignment", "align")
        .replace(
            "Windows::Win32::Metadata::NativeOverrideSlot",
            "override_slot",
        );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let output = std::path::Path::new(env!("OUT_DIR"));
    let mut expected = None;
    for (index, source) in [source, &compact].iter().enumerate() {
        let winmd = output.join(format!("structural-attributes-{index}.winmd"));
        let rdl = winmd.with_extension("rdl");
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(source)
            .output(&winmd)
            .write()
            .unwrap();
        let metadata = reader::Index::read(&winmd).unwrap();
        let aligned = metadata.expect("Test", "Aligned");
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
        let derived = metadata.expect("Test", "Derived");
        let method = derived.methods().next().unwrap();
        assert!(!method.flags().contains(MethodAttributes::NewSlot));
        assert_eq!(
            method
                .find_attribute("NativeOverrideSlotAttribute")
                .unwrap()
                .value(),
            vec![(String::new(), Value::U32(0))]
        );
        windows_rdl::writer()
            .input(&winmd)
            .filter("Test")
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(rdl).unwrap();
        assert_eq!(text.matches("#[align(16)]").count(), 2);
        assert!(text.contains("#[override_slot(0)]"));
        if let Some(expected) = &expected {
            assert_eq!(&text, expected);
        }
        expected = Some(text);
    }
}
