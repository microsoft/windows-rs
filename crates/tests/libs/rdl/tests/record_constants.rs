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
            assert!(field.constant().is_some());
        }
        assert_eq!(
            fields[0].constant().unwrap().value(),
            Value::U32(4293982463)
        );
        assert_eq!(fields[1].ty(), Type::value_named("Test", "Color/Channel"));
        let nested: Vec<_> = index.nested(color).collect();
        assert_eq!(nested.len(), 1);
        assert_eq!(nested[0].category(), TypeCategory::Enum);
        assert_eq!(
            nested[0].fields().nth(1).unwrap().ty(),
            Type::value_named("Test", "Color/Channel")
        );
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
fn architecture_specific_enum_constants() {
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let path = dir.join("record_constants_arch.winmd");
    let rdl = dir.join("record_constants_arch.rdl");
    windows_rdl::reader()
        .input_text(include_str!("../input/record_constants_arch.rdl"))
        .output(&path)
        .write()
        .unwrap();
    for _ in 0..2 {
        for (arch, expected, backing, name) in [
            (1, Value::I32(-1), Type::I32, "Signed"),
            (2, Value::U32(u32::MAX), Type::U32, "Unsigned"),
            (4, Value::U32(u32::MAX), Type::U32, "Unsigned"),
        ] {
            let index = Index::new_for_architecture(vec![File::read(&path).unwrap()], arch);
            let holder = index.expect("Test", "Holder");
            assert_eq!(
                holder.fields().next().unwrap().constant().unwrap().value(),
                expected
            );
            let enumeration = index.nested(holder).find(|ty| ty.name() == "E").unwrap();
            assert_eq!(enumeration.underlying_type().unwrap(), backing);
            let nested = index
                .nested(holder)
                .find(|ty| ty.name() == "Nested")
                .unwrap();
            assert_eq!(
                nested.fields().next().unwrap().constant().unwrap().value(),
                expected
            );
            for owner in ["Shared", "Apis"] {
                let field = index
                    .expect("Test", owner)
                    .fields()
                    .find(|field| field.name() == name)
                    .unwrap();
                assert_eq!(field.ty(), Type::value_named("Test", "Holder/E"));
                assert_eq!(field.constant().unwrap().value(), expected);
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
fn ambiguous_enum_backing_requires_architecture() {
    let source = format!(
        "{}\n#[win32] mod Test {{ const Ambiguous: Holder::E = 1; }}",
        include_str!("../input/record_constants_arch.rdl")
    );
    let error = windows_rdl::reader()
        .input_text(&source)
        .output(std::path::Path::new(env!("OUT_DIR")).join("ambiguous_enum_backing.winmd"))
        .write()
        .unwrap_err()
        .to_string();
    assert!(error.contains("architecture-dependent backing"), "{error}");
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
            "#[win32] mod Test { struct S { struct E {} #[repr(u32)] enum E { A = 0, } } }",
            "duplicate nested type",
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
