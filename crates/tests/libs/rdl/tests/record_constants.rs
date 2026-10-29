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
            assert!(field.flags().contains(FieldAttributes::HasDefault));
        }
        assert_eq!(
            fields[0].constant().unwrap().value(),
            Value::U32(4293982463)
        );
        assert_eq!(fields[1].constant().unwrap().value(), Value::U8(255));
        let palette = index.expect("Test", "Palette");
        for field in palette.fields() {
            assert_eq!(
                field.flags().contains(FieldAttributes::HasDefault),
                field.constant().is_some()
            );
        }
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
fn constant_rows_have_default_flags() {
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let path = dir.join("constant_defaults.winmd");
    let rdl = dir.join("constant_defaults.rdl");
    windows_rdl::reader()
        .input_text(
            r#"
#[winrt]
mod Runtime {
    #[repr(i32)]
    enum Signed { Negative = -1, }
    #[repr(u32)]
    enum Unsigned { High = 0x80000000, }
}
#[win32]
mod Native {
    #[repr(i32)]
    enum Number { One = 1, }
    const Present: i32 = 42;
    const Missing: i32;
    const Id: GUID = 0x00000001_0002_0003_0004_000000000005;
    struct PROPERTYKEY {
        fmtid: GUID,
        pid: u32,
    }
    #[guid(0x00000001_0002_0003_0004_000000000005)]
    const Key: PROPERTYKEY = 7;
}
"#,
        )
        .output(&path)
        .write()
        .unwrap();
    for _ in 0..2 {
        let index = Index::read(&path).unwrap();
        for (namespace, name) in [
            ("Runtime", "Signed"),
            ("Runtime", "Unsigned"),
            ("Native", "Number"),
            ("Native", "Apis"),
            ("Native", "PROPERTYKEY"),
        ] {
            for field in index.expect(namespace, name).fields() {
                assert_eq!(
                    field.flags().contains(FieldAttributes::HasDefault),
                    field.constant().is_some(),
                    "{namespace}.{name}.{}",
                    field.name()
                );
            }
        }
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
