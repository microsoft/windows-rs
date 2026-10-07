use windows_metadata::{reader::*, *};

fn assert_record(index: &Index, record: TypeDef, namespace: &str, parents: &[String]) {
    let children: Vec<_> = index.nested(record).collect();
    let mut found = 0;
    for (position, field) in record.fields().enumerate() {
        let name = format!("{}_{position}", record.name());
        let Some(child) = children.iter().find(|child| child.name() == name) else {
            continue;
        };
        found += 1;
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
fn unscoped_nested_fields_still_render_inline() {
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
        "#[win32]\nmod Test {\n    struct Outer {\n        Anonymous: struct {\n            value: i32,\n        },\n    }\n}\n"
    );
}
