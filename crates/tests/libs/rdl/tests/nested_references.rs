use windows_metadata::{reader::*, *};

#[test]
fn anonymous_provenance_validation() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("anonymous_invalid.winmd");
    for body in [
        "#[anonymous(0)] struct Root { value: i32, }",
        "struct Root { #[anonymous(-1)] struct Child { value: i32, } }",
        "struct Root { #[anonymous(0)] #[anonymous(0)] struct Child { value: i32, } }",
        "struct Root { value: #[anonymous(1)] struct { value: i32, }, }",
        "struct Root { #[anonymous(0)] value: i32, }",
        "#[anonymous(0)] extern \"C\" fn Callback();",
        "extern \"C\" fn Callback(#[anonymous(0)] value: i32);",
    ] {
        let result = windows_rdl::reader()
            .input_text(&format!("#[win32] mod Test {{ {body} }}"))
            .output(&output)
            .write();
        assert!(result.is_err(), "accepted {body}");
        let error = result.unwrap_err();
        assert!(error.to_string().contains("anonymous"), "{error}");
    }
}

fn assert_record(index: &Index, record: TypeDef, namespace: &str, parents: &[String]) {
    let children: Vec<_> = index.nested(record).collect();
    let mut found = 0;
    for (position, field) in record.fields().enumerate() {
        let name = format!("{}_{position}", record.name());
        let Some(child) = children.iter().find(|child| child.name() == name) else {
            continue;
        };
        found += 1;
        assert_eq!(child.anonymous_index().unwrap(), Some(position as u32));
        let mut names = parents.to_vec();
        names.push(name);
        assert_eq!(field.ty(), Type::value_named(namespace, &names.join("/")));

        let mut blob = field.blob(2);
        assert_eq!(blob.read_u8(), 0x06);
        assert_eq!(blob.read_u8(), 0x11);
        let TypeDefOrRef::TypeRef(mut reference) = blob.decode() else {
            panic!("expected TypeRef");
        };
        for name in names[1..].iter().rev() {
            assert_eq!(reference.name(), name);
            assert_eq!(reference.namespace(), "");
            let ResolutionScope::TypeRef(parent) = reference.scope() else {
                panic!("nested field reference must identify its enclosing type");
            };
            reference = parent;
        }
        assert_eq!(reference.namespace(), namespace);
        assert_eq!(reference.name(), parents[0]);
        assert!(matches!(reference.scope(), ResolutionScope::Module(_)));
        assert_eq!(child.arches(), record.arches());
        assert_record(index, *child, namespace, &names);
    }
    assert_eq!(found, children.len());
}

fn assert_metadata(path: &std::path::Path) {
    let index = Index::read(path).unwrap();
    let mut count = 0;
    for record in index.types() {
        if record.namespace() == "Test" {
            assert_record(&index, record, "Test", &[record.name().to_string()]);
            count += 1;
        }
    }
    assert_eq!(count, 5);
    let outer = index.expect("Test", "Outer");
    assert_eq!(
        outer.fields().map(|field| field.name()).collect::<Vec<_>>(),
        ["header", "Anonymous", "tail"]
    );
}

#[test]
fn inline_records_have_scoped_references() {
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let original = include_str!("../input/struct_nested_anon.rdl");
    let winmd = dir.join("nested_references.winmd");
    windows_rdl::reader()
        .input_text(original)
        .output(&winmd)
        .write()
        .unwrap();
    assert_metadata(&winmd);

    let rdl = dir.join("nested_references.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rdl).unwrap(), original);
    let roundtrip = dir.join("nested_references_roundtrip.winmd");
    windows_rdl::reader()
        .input(&rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_metadata(&roundtrip);
}

#[test]
fn unscoped_nested_names_do_not_imply_anonymity() {
    let mut file = writer::File::new("legacy_nested");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let flags = TypeAttributes::SequentialLayout | TypeAttributes::Sealed;
    let outer = file.TypeDef("Test", "Outer", base, flags | TypeAttributes::Public);
    file.Field(
        "Anonymous",
        &Type::value_named("", "Outer_0"),
        FieldAttributes::Public,
    );
    let child = file.TypeDef("", "Outer_0", base, flags | TypeAttributes::NestedPublic);
    file.NestedClass(child, outer);
    file.Field("value", &Type::I32, FieldAttributes::Public);
    let rdl = std::path::Path::new(env!("OUT_DIR")).join("legacy_nested.rdl");
    windows_rdl::writer()
        .input_bytes(&file.into_stream())
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(rdl).unwrap(),
        "#[win32]\nmod Test {\n    struct Outer {\n        struct Outer_0 {\n            value: i32,\n        }\n        Anonymous: Outer::Outer_0,\n    }\n}\n"
    );
}

#[test]
fn shared_nested_type_identity() {
    for name in ["Inner", "Outer_0"] {
        let mut file = writer::File::new("shared_nested");
        let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
        let flags = TypeAttributes::SequentialLayout | TypeAttributes::Sealed;
        let outer = file.TypeDef("Test", "Outer", base, flags | TypeAttributes::Public);
        let shared = Type::value_named("Test", &format!("Outer/{name}"));
        for field in ["first", "second"] {
            file.Field(field, &shared, FieldAttributes::Public);
        }
        let child = file.TypeDef("", name, base, flags | TypeAttributes::NestedPublic);
        file.NestedClass(child, outer);
        file.Field("value", &Type::I32, FieldAttributes::Public);
        let path = std::path::Path::new(env!("OUT_DIR")).join(format!("shared-{name}.rdl"));
        windows_rdl::writer()
            .input_bytes(&file.into_stream())
            .output(&path)
            .write()
            .unwrap();
        windows_rdl::reader()
            .input(&path)
            .output(path.with_extension("winmd"))
            .write()
            .unwrap();
        let index = Index::read(path.with_extension("winmd")).unwrap();
        let outer = index.expect("Test", "Outer");
        let children: Vec<_> = index.nested(outer).collect();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name(), name);
        assert_eq!(
            outer.fields().map(|field| field.ty()).collect::<Vec<_>>(),
            [shared.clone(), shared]
        );
    }
}

#[test]
fn scoped_lookup_and_forward_references() {
    let source = r#"
        #[win32] mod Test {
            struct Outer {
                first: Inner,
                second: Inner,
                struct Inner {
                    value: Leaf,
                    next: *mut Inner,
                    struct Leaf { value: i32, }
                }
            }
            struct Other {
                struct Inner { value: u16, }
                local: Inner,
                foreign: Outer::Inner,
            }
            mod Child { struct Consumer { value: super::Outer::Inner, } }
        }
    "#;
    let path = std::path::Path::new(env!("OUT_DIR")).join("scoped-lookup.winmd");
    windows_rdl::reader()
        .input_text(source)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    let outer = index.expect("Test", "Outer");
    let shared = Type::value_named("Test", "Outer/Inner");
    assert!(outer.fields().all(|field| field.ty() == shared));
    let inner = index.nested(outer).next().unwrap();
    assert_eq!(
        inner.fields().map(|field| field.ty()).collect::<Vec<_>>(),
        [
            Type::value_named("Test", "Outer/Inner/Leaf"),
            Type::PtrMut(Box::new(shared.clone()), 1),
        ]
    );
    assert_eq!(
        index
            .expect("Test", "Other")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        [Type::value_named("Test", "Other/Inner"), shared.clone()]
    );
    assert_eq!(
        index
            .expect("Test.Child", "Consumer")
            .fields()
            .next()
            .unwrap()
            .ty(),
        shared,
    );
    let reference_path = path.with_file_name("nested-reference-consumer.winmd");
    windows_rdl::reader()
        .reference(&path)
        .input_text("#[win32] mod Consumer { struct Record { value: Test::Outer::Inner, } }")
        .output(&reference_path)
        .write()
        .unwrap();
    let reference = Index::read(&reference_path).unwrap();
    assert_eq!(
        reference
            .expect("Consumer", "Record")
            .fields()
            .next()
            .unwrap()
            .ty(),
        Type::value_named("Test", "Outer/Inner")
    );
}

#[test]
fn nested_name_conflicts_are_errors() {
    for (body, message) in [
        (
            "struct Inner {} union Inner {}",
            "duplicate nested type name",
        ),
        (
            "struct Outer_0 {} value: struct { x: i32, },",
            "inline nested type name collides",
        ),
        ("value: Outer::Missing,", "type not found"),
        (
            "struct Inner {} value: Inner<i32>,",
            "nested types cannot have generic arguments",
        ),
        (
            "struct Inner {} value: Outer<i32>::Inner,",
            "nested types cannot have generic arguments",
        ),
        (
            "struct Inner {} value: <Outer as Outer>::Inner,",
            "qualified associated types are not supported",
        ),
    ] {
        let path = std::path::Path::new(env!("OUT_DIR")).join("nested-name-errors.winmd");
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ struct Outer {{ {body} }} }}"
            ))
            .output(path)
            .write()
            .unwrap_err();
        assert!(error.message.contains(message), "{error}");
    }
}
