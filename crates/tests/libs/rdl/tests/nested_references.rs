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
fn owner_qualified_lookup_and_forward_references() {
    let source = r#"
        #[win32] mod Test {
            struct Outer {
                first: Outer::Inner,
                second: Outer::Inner,
                struct Inner {
                    value: Outer::Inner::Leaf,
                    next: *mut Outer::Inner,
                    struct Leaf { value: i32, }
                }
            }
            struct Other {
                struct Inner { value: u16, }
                local: Other::Inner,
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
    let rdl = reference_path.with_extension("rdl");
    windows_rdl::writer()
        .input(&reference_path)
        .reference(&path)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        "#[win32]\nmod Consumer {\n    struct Record {\n        value: Test::Outer::Inner,\n    }\n}\n"
    );
    windows_rdl::writer()
        .input(&reference_path)
        .references([&path])
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rdl).unwrap(), "");
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
        ("struct Inner {} value: Inner,", "type not found"),
        (
            "struct Inner {} value: Outer::Inner<i32>,",
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

fn roundtrip_nested(name: &str, source: &str) -> Index {
    let path = std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.winmd"));
    windows_rdl::reader()
        .input_text(source)
        .output(&path)
        .write()
        .unwrap();
    let rdl = path.with_extension("rdl");
    windows_rdl::writer()
        .input(&path)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rdl).unwrap(), source);
    Index::read(&path).unwrap()
}

#[test]
fn nested_paths_do_not_shadow_namespace_types() {
    let index = roundtrip_nested(
        "nested-namespace-paths",
        "#[win32]\nmod Test {\n    mod A {\n        struct B {\n            value: i32,\n        }\n    }\n    struct Outer {\n        struct A {\n            struct B {\n                value: u8,\n            }\n            value: Outer::A::B,\n        }\n        nested: Outer::A,\n        global: A::B,\n        deep: Outer::A::B,\n    }\n}\n",
    );
    assert_eq!(
        index
            .expect("Test", "Outer")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        [
            Type::value_named("Test", "Outer/A"),
            Type::value_named("Test.A", "B"),
            Type::value_named("Test", "Outer/A/B"),
        ]
    );
}

#[test]
fn nested_paths_start_at_the_namespace() {
    let index = roundtrip_nested(
        "nested-owner-paths",
        "#[win32]\nmod Test {\n    struct Outer {\n        struct Inner {\n            value: i32,\n        }\n        struct Outer {\n            struct Inner {\n                value: u8,\n            }\n            value: Outer::Outer::Inner,\n            other: Outer::Inner,\n        }\n        first: Outer::Inner,\n        second: Outer::Outer::Inner,\n        third: Outer::Outer,\n    }\n}\n",
    );
    let outer = Type::value_named("Test", "Outer/Inner");
    let inner = Type::value_named("Test", "Outer/Outer/Inner");
    assert_eq!(
        index
            .expect("Test", "Outer")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        [
            outer.clone(),
            inner.clone(),
            Type::value_named("Test", "Outer/Outer")
        ]
    );
    let nested = index
        .nested(index.expect("Test", "Outer"))
        .find(|child| child.name() == "Outer")
        .unwrap();
    assert_eq!(
        nested.fields().map(|field| field.ty()).collect::<Vec<_>>(),
        [inner, outer]
    );
}

#[test]
fn referenced_nested_enums() {
    let dependency = std::path::Path::new(env!("OUT_DIR")).join("nested-enum-dependency.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Dep { attribute ModeAttribute { fn(mode: Options::Mode); } struct Options { #[flags] #[repr(u32)] enum Mode { A = 1, B = 2, } value: u32, } }",
        )
        .output(&dependency)
        .write()
        .unwrap();
    let source = "#[win32]\nmod Consumer {\n    const Both: Dep::Options::Mode = 3;\n    struct Record {\n        #[Dep::Mode(B | A)]\n        value: u32,\n    }\n}\n";
    let consumer = dependency.with_file_name("nested-enum-consumer.winmd");
    windows_rdl::reader()
        .reference(&dependency)
        .input_text(source)
        .output(&consumer)
        .write()
        .unwrap();
    let index = Index::read(&consumer).unwrap();
    let mode = TypeName::named("Dep", "Options/Mode");
    let field = index.expect("Consumer", "Record").fields().next().unwrap();
    assert_eq!(
        field.attributes().next().unwrap().value(),
        [(
            String::new(),
            Value::EnumValue(mode, Box::new(Value::I32(3)))
        )]
    );
    let rdl = consumer.with_extension("rdl");
    windows_rdl::writer()
        .input(&consumer)
        .reference(&dependency)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rdl).unwrap(), source);
}

#[test]
fn inline_positions_skip_record_constants() {
    let mut file = writer::File::new("inline_positions");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let flags = TypeAttributes::SequentialLayout | TypeAttributes::Sealed;
    let outer = file.TypeDef("Test", "Outer", base, flags | TypeAttributes::Public);
    let constant = file.Field(
        "Count",
        &Type::U32,
        FieldAttributes::Public
            | FieldAttributes::Static
            | FieldAttributes::Literal
            | FieldAttributes::HasDefault,
    );
    file.Constant(writer::HasConstant::Field(constant), &Value::U32(1));
    let inner = Type::value_named("Test", "Outer/Outer_1");
    let pointer = Type::PtrMut(Box::new(inner.clone()), 1);
    file.Field("value", &inner, FieldAttributes::Public);
    file.Field("pointer", &pointer, FieldAttributes::Public);
    let child = file.TypeDef("", "Outer_1", base, flags | TypeAttributes::NestedPublic);
    file.NestedClass(child, outer);
    file.Field("x", &Type::I32, FieldAttributes::Public);
    let rdl = std::path::Path::new(env!("OUT_DIR")).join("inline-positions.rdl");
    windows_rdl::writer()
        .input_bytes(&file.into_stream())
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(&rdl)
        .output(rdl.with_extension("winmd"))
        .write()
        .unwrap();
    let index = Index::read(rdl.with_extension("winmd")).unwrap();
    let outer = index.expect("Test", "Outer");
    assert_eq!(index.nested(outer).next().unwrap().name(), "Outer_1");
    let fields: Vec<_> = outer
        .fields()
        .map(|field| (field.name().to_string(), field.ty()))
        .collect();
    assert!(fields.contains(&("value".to_string(), inner)));
    assert!(fields.contains(&("pointer".to_string(), pointer)));
    assert!(fields.contains(&("Count".to_string(), Type::U32)));
}

#[test]
fn referenced_constants_use_their_architecture_backing() {
    let dependency = std::path::Path::new(env!("OUT_DIR")).join("arch-backing-dependency.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../input/record_constants_arch.rdl"))
        .output(&dependency)
        .write()
        .unwrap();
    let consumer = dependency.with_file_name("arch-backing-consumer.winmd");
    windows_rdl::reader()
        .reference(&dependency)
        .input_text(
            "#[win32] mod Consumer { #[arch(X86)] const Signed: Test::Holder::E = -1; #[arch(X64 | Arm64)] const Unsigned: Test::Holder::E = 4294967295; struct Holder { #[arch(X86)] const Nested: Test::Holder::E = -1; } }",
        )
        .output(&consumer)
        .write()
        .unwrap();
    let index = Index::read(&consumer).unwrap();
    let values: Vec<_> = index
        .types()
        .flat_map(|ty| ty.fields())
        .map(|field| (field.name().to_string(), field.constant().unwrap().value()))
        .collect();
    assert!(values.contains(&("Signed".to_string(), Value::I32(-1))));
    assert!(values.contains(&("Unsigned".to_string(), Value::U32(u32::MAX))));
    assert!(values.contains(&("Nested".to_string(), Value::I32(-1))));

    let error = windows_rdl::reader()
        .reference(&dependency)
        .input_text("#[win32] mod Consumer { const Any: Test::Holder::E = 1; }")
        .output(consumer.with_file_name("arch-backing-conflict.winmd"))
        .write()
        .unwrap_err();
    assert!(
        error.message.contains("architecture-dependent backing"),
        "{error}"
    );
}

#[test]
fn local_attributes_resolve_nested_types_in_their_namespace() {
    let path = std::path::Path::new(env!("OUT_DIR")).join("attribute-nested-namespace.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Def { attribute ModeAttribute { fn(mode: Options::Mode); kind: Options::Mode, } struct Options { #[repr(i32)] enum Mode { A = 1, B = 2, } value: u32, } }",
        )
        .input_text("#[win32] mod Consumer { struct Record { #[Def::Mode(A, kind = B)] value: u32, } }")
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    let mode = TypeName::named("Def", "Options/Mode");
    let field = index.expect("Consumer", "Record").fields().next().unwrap();
    assert_eq!(
        field.attributes().next().unwrap().value(),
        [
            (
                String::new(),
                Value::EnumValue(mode.clone(), Box::new(Value::I32(1)))
            ),
            (
                "kind".to_string(),
                Value::EnumValue(mode, Box::new(Value::I32(2)))
            ),
        ]
    );
}

#[test]
fn glob_imports_keep_qualified_prefixes() {
    let path = std::path::Path::new(env!("OUT_DIR")).join("glob-qualified.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Dep { struct Inner { value: i32, } struct Outer { struct Inner { value: u8, } value: u32, } mod Child { struct Leaf { value: u16, } } }",
        )
        .input_text(
            "use Dep::*;\n#[win32] mod Consumer { struct Record { nested: Outer::Inner, plain: Inner, child: Child::Leaf, } }",
        )
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    assert_eq!(
        index
            .expect("Consumer", "Record")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        [
            Type::value_named("Dep", "Outer/Inner"),
            Type::value_named("Dep", "Inner"),
            Type::value_named("Dep.Child", "Leaf"),
        ]
    );
}

#[test]
fn record_constants_with_disjoint_architectures() {
    let index = roundtrip_nested(
        "record-constant-variants",
        "#[win32]\nmod Test {\n    struct Shared {\n        value: i32,\n        #[arch(X86)]\n        const Value: u32 = 1;\n        #[arch(X64 | Arm64)]\n        const Value: u32 = 2;\n    }\n}\n",
    );
    let values: Vec<_> = index
        .expect("Test", "Shared")
        .fields()
        .filter_map(|field| field.constant().map(|c| (field.arches(), c.value())))
        .collect();
    assert_eq!(values, [(1, Value::U32(1)), (6, Value::U32(2))]);

    for body in [
        "#[arch(X86)] const Value: u32 = 1; #[arch(X86 | X64)] const Value: u32 = 2;",
        "const Value: u32 = 1; #[arch(X64)] const Value: u32 = 2;",
        "#[arch(X86)] const value: u32 = 1; value: i32,",
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ struct Shared {{ {body} }} }}"
            ))
            .output(std::path::Path::new(env!("OUT_DIR")).join("record-constant-conflict.winmd"))
            .write()
            .unwrap_err();
        assert!(
            error.message.contains("duplicate record member name"),
            "{error}"
        );
    }
}
