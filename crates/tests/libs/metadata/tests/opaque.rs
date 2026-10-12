use windows_metadata::{reader::*, writer, *};

fn mark(file: &mut writer::File, target: writer::HasAttribute, name: &str) {
    let attribute = file.TypeRef("Windows.Win32.Metadata", name);
    let ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![],
        },
        writer::MemberRefParent::TypeRef(attribute),
    );
    file.Attribute(target, writer::AttributeType::MemberRef(ctor), &[]);
}

fn record(file: &mut writer::File, namespace: &str, name: &str) -> writer::TypeDef {
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    file.TypeDef(
        namespace,
        name,
        base,
        TypeAttributes::Public | TypeAttributes::Sealed | TypeAttributes::SequentialLayout,
    )
}

#[test]
fn malformed_opaque_metadata_is_rejected_by_checked_emission_and_readback() {
    for case in [
        "fields",
        "methods",
        "layout",
        "alias",
        "field_marker",
        "value",
        "base",
    ] {
        let mut file = writer::File::new(case);
        let opaque = record(&mut file, "Test", "Opaque");
        if case != "field_marker" {
            mark(
                &mut file,
                writer::HasAttribute::TypeDef(opaque),
                "NativeOpaqueAttribute",
            );
        }
        match case {
            "fields" => {
                file.Field("value", &Type::I32, FieldAttributes::Public);
            }
            "methods" => {
                file.MethodDef(
                    "Method",
                    &Signature::default(),
                    MethodAttributes::Public,
                    Default::default(),
                );
            }
            "layout" => {
                file.ClassLayout(opaque, 0, 0);
            }
            "alias" => {
                mark(
                    &mut file,
                    writer::HasAttribute::TypeDef(opaque),
                    "NativeTypedefAttribute",
                );
            }
            "field_marker" => {
                let field = file.Field("value", &Type::I32, FieldAttributes::Public);
                mark(
                    &mut file,
                    writer::HasAttribute::Field(field),
                    "NativeOpaqueAttribute",
                );
            }
            "value" => {
                record(&mut file, "Test", "Holder");
                file.Field(
                    "value",
                    &Type::value_named("Test", "Opaque"),
                    FieldAttributes::Public,
                );
            }
            "base" => {
                let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("Test", "Opaque"));
                file.TypeDef("Test", "Derived", base, TypeAttributes::Public);
            }
            _ => unreachable!(),
        }
        let expected = match case {
            "field_marker" => "NativeOpaqueAttribute requires a native record",
            "value" | "base" => "requires storage for opaque type `Test.Opaque`",
            _ => "must be a native record without",
        };
        let bytes = file.into_stream();
        let index = Index::new(vec![File::new(bytes.clone()).unwrap()]);
        let error = index.validate_opaque_types(None).unwrap_err();
        assert!(error.to_string().contains(expected), "{case}: {error}");
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../..")
            .join("target")
            .join("test_metadata_opaque");
        std::fs::create_dir_all(&path).unwrap();
        let input = path.join(format!("{case}.winmd"));
        let output = path.join(format!("{case}_merged.winmd"));
        std::fs::write(&input, bytes).unwrap();
        std::fs::write(&output, b"last-good-output").unwrap();
        let error = merge().input(&input).output(&output).merge().unwrap_err();
        assert!(error.to_string().contains(expected), "{case}: {error}");
        assert_eq!(std::fs::read(&output).unwrap(), b"last-good-output");
        let error = merge::Remapper::new()
            .input(&input)
            .source("Test")
            .fallback("Routed")
            .output(&output)
            .remap()
            .unwrap_err();
        assert!(error.to_string().contains(expected), "{case}: {error}");
        assert_eq!(std::fs::read(&output).unwrap(), b"last-good-output");
        let rdl = path.join(format!("{case}.rdl"));
        std::fs::write(&rdl, b"last-good-rdl").unwrap();
        let error = windows_rdl::writer()
            .input(&input)
            .output(&rdl)
            .write()
            .unwrap_err();
        assert!(error.message.contains(expected), "{case}: {error}");
        assert_eq!(std::fs::read(&rdl).unwrap(), b"last-good-rdl");
        assert!(
            index
                .validate_opaque_storage(&Type::value_named("Test", "Opaque"))
                .is_err()
        );
    }
}

#[test]
fn nominally_distinct_opaque_records_and_nested_identities_are_preserved() {
    let mut file = writer::File::new("nominal");
    let a = record(&mut file, "Test", "A");
    mark(
        &mut file,
        writer::HasAttribute::TypeDef(a),
        "NativeOpaqueAttribute",
    );
    let b = record(&mut file, "Test", "B");
    mark(
        &mut file,
        writer::HasAttribute::TypeDef(b),
        "NativeOpaqueAttribute",
    );
    let owner = record(&mut file, "Test", "Owner");
    file.Field(
        "a",
        &Type::PtrMut(Box::new(Type::value_named("Test", "A")), 1),
        FieldAttributes::Public,
    );
    file.Field(
        "b",
        &Type::PtrMut(Box::new(Type::value_named("Test", "B")), 1),
        FieldAttributes::Public,
    );
    file.Field(
        "nested",
        &Type::PtrConst(Box::new(Type::value_named("Test", "Owner/Hidden")), 1),
        FieldAttributes::Public,
    );
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let hidden = file.TypeDef(
        "",
        "Hidden",
        base,
        TypeAttributes::NestedPublic | TypeAttributes::SequentialLayout,
    );
    file.NestedClass(hidden, owner);
    mark(
        &mut file,
        writer::HasAttribute::TypeDef(hidden),
        "NativeOpaqueAttribute",
    );
    let index = Index::new(vec![File::new(file.try_into_stream().unwrap()).unwrap()]);
    assert!(index.expect("Test", "A").is_opaque());
    assert!(index.expect("Test", "B").is_opaque());
    let fields: Vec<_> = index
        .expect("Test", "Owner")
        .fields()
        .map(|field| field.ty())
        .collect();
    assert_ne!(fields[0], fields[1]);
    assert!(
        index
            .nested(index.expect("Test", "Owner"))
            .next()
            .unwrap()
            .is_opaque()
    );
    assert!(
        index
            .validate_opaque_storage(&Type::value_named("Test", "Owner/Hidden"))
            .is_err()
    );
    for ty in fields {
        index.validate_opaque_storage(&ty).unwrap();
    }
}

#[test]
fn opaque_and_complete_variants_check_member_architecture() {
    let mut file = writer::File::new("variants");
    let opaque = record(&mut file, "Test", "Tag");
    mark(
        &mut file,
        writer::HasAttribute::TypeDef(opaque),
        "NativeOpaqueAttribute",
    );
    arch(&mut file, opaque, 1);
    let complete = record(&mut file, "Test", "Tag");
    arch(&mut file, complete, 2);
    let holder = record(&mut file, "Test", "Holder");
    arch(&mut file, holder, 2);
    file.Field(
        "value",
        &Type::value_named("Test", "Tag"),
        FieldAttributes::Public,
    );
    assert!(file.try_into_stream().is_ok());
}

#[test]
fn checked_emission_rejects_unknown_storage() {
    let mut file = writer::File::new("checked");
    let opaque = record(&mut file, "Test", "Opaque");
    mark(
        &mut file,
        writer::HasAttribute::TypeDef(opaque),
        "NativeOpaqueAttribute",
    );
    record(&mut file, "Test", "Holder");
    file.Field(
        "value",
        &Type::value_named("Test", "Opaque"),
        FieldAttributes::Public,
    );
    let error = file.try_into_stream().unwrap_err();
    assert!(
        error
            .to_string()
            .contains("requires storage for opaque type `Test.Opaque`")
    );
}

fn arch(file: &mut writer::File, ty: writer::TypeDef, bits: i32) {
    let attribute = file.TypeRef("Windows.Win32.Metadata", "SupportedArchitectureAttribute");
    let ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![Type::I32],
        },
        writer::MemberRefParent::TypeRef(attribute),
    );
    file.Attribute(
        writer::HasAttribute::TypeDef(ty),
        writer::AttributeType::MemberRef(ctor),
        &[(String::new(), Value::I32(bits))],
    );
}
