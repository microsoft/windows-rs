use windows_metadata::{reader::*, writer, *};

fn output(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .join("target")
        .join("test_metadata_nested_references");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(format!("{name}.winmd"))
}

fn fixture(deep_type: &Type) -> Vec<u8> {
    let mut file = writer::File::new("nested_references");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let flags = TypeAttributes::SequentialLayout | TypeAttributes::Sealed;
    let a = file.TypeDef("Test", "A", base, flags | TypeAttributes::Public);
    file.Field(
        "child",
        &Type::value_named("Test", "A/Shared"),
        FieldAttributes::Public,
    );
    file.Field(
        "deep",
        &Type::PtrConst(Box::new(Type::value_named("Test", "A/Shared/Deep")), 1),
        FieldAttributes::Public,
    );
    file.Field(
        "other",
        &Type::value_named("Test", "B"),
        FieldAttributes::Public,
    );
    let shared = file.TypeDef("", "Shared", base, flags | TypeAttributes::NestedPublic);
    file.NestedClass(shared, a);
    file.Field(
        "child",
        &Type::value_named("Test", "A/Shared/Deep"),
        FieldAttributes::Public,
    );
    let deep = file.TypeDef("", "Deep", base, flags | TypeAttributes::NestedPublic);
    file.NestedClass(deep, shared);
    file.Field("value", deep_type, FieldAttributes::Public);
    let b = file.TypeDef("Test", "B", base, flags | TypeAttributes::Public);
    file.Field(
        "child",
        &Type::value_named("Test", "B/Shared"),
        FieldAttributes::Public,
    );
    let shared = file.TypeDef("", "Shared", base, flags | TypeAttributes::NestedPublic);
    file.NestedClass(shared, b);
    file.Field("value", &Type::I64, FieldAttributes::Public);
    file.into_stream()
}

fn assert_scope(field: Field, namespace: &str, names: &[&str]) {
    let mut blob = field.blob(2);
    assert_eq!(blob.read_u8(), 0x06);
    assert_eq!(blob.read_u8(), 0x11);
    let TypeDefOrRef::TypeRef(mut reference) = blob.decode() else {
        panic!("expected TypeRef");
    };
    for name in names[1..].iter().rev() {
        assert_eq!(reference.name(), *name);
        assert_eq!(reference.namespace(), "");
        let ResolutionScope::TypeRef(parent) = reference.scope() else {
            panic!("nested reference must have an enclosing TypeRef");
        };
        reference = parent;
    }
    assert_eq!(reference.name(), names[0]);
    assert_eq!(reference.namespace(), namespace);
    assert!(matches!(reference.scope(), ResolutionScope::Module(_)));
}

fn assert_references(index: &Index, a_namespace: &str, b_namespace: &str, deep_type: &Type) {
    let a = index.expect(a_namespace, "A");
    let b = index.expect(b_namespace, "B");
    let mut fields = a.fields();
    let child = fields.next().unwrap();
    assert_scope(child, a_namespace, &["A", "Shared"]);
    assert_eq!(child.ty(), Type::value_named(a_namespace, "A/Shared"));
    assert_eq!(
        fields.next().unwrap().ty(),
        Type::PtrConst(Box::new(Type::value_named(a_namespace, "A/Shared/Deep")), 1)
    );
    let other = fields.next().unwrap();
    assert_scope(other, b_namespace, &["B"]);
    assert_eq!(other.ty(), Type::value_named(b_namespace, "B"));
    let shared = index.nested(a).next().unwrap();
    assert_scope(
        shared.fields().next().unwrap(),
        a_namespace,
        &["A", "Shared", "Deep"],
    );
    assert_scope(b.fields().next().unwrap(), b_namespace, &["B", "Shared"]);
    assert_eq!(
        b.fields().next().unwrap().ty(),
        Type::value_named(b_namespace, "B/Shared")
    );
    assert_eq!(
        index
            .nested(shared)
            .next()
            .unwrap()
            .fields()
            .next()
            .unwrap()
            .ty(),
        *deep_type
    );
    assert_eq!(
        index
            .nested(b)
            .next()
            .unwrap()
            .fields()
            .next()
            .unwrap()
            .ty(),
        Type::I64
    );
}

#[test]
fn nested_signature_identity() {
    let index = Index::new(vec![File::new(fixture(&Type::I32)).unwrap()]);
    assert_references(&index, "Test", "Test", &Type::I32);

    let a = index.expect("Test", "A");
    let shared = index.nested(a).next().unwrap();
    let deep = index.nested(shared).next().unwrap();
    for (def, name) in [(shared, "A/Shared"), (deep, "A/Shared/Deep")] {
        let code = TypeDefOrRef::TypeDef(def).encode();
        assert!(code < 0x80);
        let bytes = [0x11, code as u8];
        let mut blob = Blob::new(&index, 0, &bytes);
        assert_eq!(blob.read_type_code(&[]), Type::value_named("Test", name));
    }
}

#[test]
fn nested_references_survive_merge_and_remap() {
    let input = output("input");
    std::fs::write(&input, fixture(&Type::I32)).unwrap();
    let merged = output("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    assert_references(&Index::read(&merged).unwrap(), "Test", "Test", &Type::I32);

    let remapped = output("remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .route("A", "Routed.First")
        .route("B", "Routed.Second")
        .fallback("Wrong")
        .output(&remapped)
        .remap()
        .unwrap();
    assert_references(
        &Index::read(&remapped).unwrap(),
        "Routed.First",
        "Routed.Second",
        &Type::I32,
    );
}

#[test]
fn nested_references_survive_architecture_merge() {
    let x86 = output("x86");
    let x64 = output("x64");
    std::fs::write(&x86, fixture(&Type::I32)).unwrap();
    std::fs::write(&x64, fixture(&Type::I64)).unwrap();
    let merged = output("arch_merged");
    merge()
        .arch_input(&x86, 1)
        .arch_input(&x64, 2)
        .output(&merged)
        .merge()
        .unwrap();
    for (bits, ty) in [(1, Type::I32), (2, Type::I64)] {
        let index = Index::new_for_architecture(vec![File::read(&merged).unwrap()], bits);
        assert_references(&index, "Test", "Test", &ty);
        let a = index.expect("Test", "A");
        assert_eq!(a.arches(), bits);
        assert!(
            index
                .nested_recursive(a)
                .iter()
                .all(|child| child.arches() == bits)
        );
        assert_eq!(index.expect("Test", "B").arches(), 0);
    }
}

#[test]
#[should_panic(expected = "nested generic types are not supported")]
fn nested_generic_field_is_rejected() {
    let mut file = writer::File::new("nested_generic_field");
    let ty = Type::ClassName(TypeName {
        namespace: "Test".to_string(),
        name: "Generic`1/Child".to_string(),
        generics: vec![Type::I32],
    });
    file.Field("value", &ty, FieldAttributes::Public);
}

#[test]
#[should_panic(expected = "nested generic types are not supported")]
fn nested_generic_type_spec_is_rejected() {
    let mut file = writer::File::new("nested_generic_type_spec");
    file.TypeSpec("Test", "Generic`1/Child`1", &[Type::I32, Type::I64]);
}

#[test]
fn top_level_generic_signatures_preserve_arity() {
    for name in ["Generic", "Generic`1"] {
        let mut file = writer::File::new("top_level_generics");
        let generic = file.TypeDef(
            "Test",
            "Generic`1",
            writer::TypeDefOrRef::default(),
            TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
        );
        file.GenericParam(
            "T",
            writer::TypeOrMethodDef::TypeDef(generic),
            0,
            GenericParamAttributes::None,
        );
        let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "Object"));
        let holder = file.TypeDef("Test", "Holder", base, TypeAttributes::Public);
        let mut name = TypeName {
            namespace: "Test".to_string(),
            name: name.to_string(),
            generics: vec![Type::I32],
        };
        let ty = Type::ClassName(name.clone());
        file.Field("value", &ty, FieldAttributes::Public);
        file.InterfaceImpl(holder, &ty);
        let index = Index::new(vec![File::new(file.into_stream()).unwrap()]);
        let holder = index.expect("Test", "Holder");
        name.name = "Generic`1".to_string();
        let expected = Type::ClassName(name);
        assert_eq!(holder.fields().next().unwrap().ty(), expected);
        assert_eq!(
            holder.interface_impls().next().unwrap().interface(&[]),
            expected
        );
    }
}
