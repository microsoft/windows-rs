use windows_metadata::{reader::*, *};

#[test]
fn literal_fields_do_not_become_storage() {
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let path = dir.join("record_constants.winmd");
    let rdl = dir.join("record_constants.rdl");
    windows_rdl::reader()
        .input_text(include_str!("../input/record_constants.rdl"))
        .output(&path)
        .write()
        .unwrap();
    for _ in 0..2 {
        let index = Index::read(&path).unwrap();
        let color = index.expect("Test", "Color");
        let fields: Vec<_> = color.fields().collect();
        assert_eq!(fields.len(), 2);
        for field in &fields {
            assert!(field.flags().contains(FieldAttributes::Literal));
            assert!(field.flags().contains(FieldAttributes::Static));
        }
        assert_eq!(
            fields[0].constant().unwrap().value(),
            Value::U32(4293982463)
        );
        assert_eq!(fields[1].constant().unwrap().value(), Value::U8(255));
        let palette = index.expect("Test", "Palette");
        assert_eq!(
            palette
                .fields()
                .filter(|field| !field.flags().contains(FieldAttributes::Static))
                .count(),
            2
        );
        windows_rdl::writer()
            .input(&path)
            .output(&rdl)
            .write()
            .unwrap();
        windows_rdl::reader()
            .input(&rdl)
            .output(&path)
            .write()
            .unwrap();
    }
}

#[test]
fn invalid_record_constants() {
    for (source, message) in [
        (
            "#[win32] mod Test { struct S { const A: u32; } }",
            "require a value",
        ),
        (
            "#[winrt] mod Test { struct S { A: u32, const B: u32 = 1; } }",
            "require Win32 metadata",
        ),
        (
            "#[win32] mod Test { struct S { A: u32, const A: u32 = 1; } }",
            "duplicate record member",
        ),
        (
            "#[win32] mod Test { struct S { const A: u32 = 1; const A: u32 = 2; } }",
            "duplicate record member",
        ),
        (
            "#[win32] mod Test { struct S { #[arch(X86)] const A: u32 = 1; } }",
            "inherit the record's architecture",
        ),
        (
            "#[win32] mod Test { struct S { #[repr(u32)] enum E { A = 0, } } }",
            "nested enums are not supported",
        ),
    ] {
        let error = windows_rdl::reader()
            .input_text(source)
            .output(std::path::Path::new(env!("OUT_DIR")).join("invalid_record_constant.winmd"))
            .write()
            .unwrap_err()
            .to_string();
        assert!(error.contains(message), "{error}");
    }
}
