use windows_metadata::*;

#[test]
fn system_type_names() {
    for ty in [
        Type::Void,
        Type::Bool,
        Type::Char,
        Type::I8,
        Type::U8,
        Type::I16,
        Type::U16,
        Type::I32,
        Type::U32,
        Type::I64,
        Type::U64,
        Type::F32,
        Type::F64,
        Type::ISize,
        Type::USize,
        Type::String,
        Type::Object,
    ] {
        assert_eq!(ty.type_name().unwrap().primitive(), Some(ty));
    }

    assert_eq!(
        Type::I32.type_name(),
        Some(TypeName::named("System", "Int32"))
    );
    assert_eq!(TypeName::named("Other", "Int32").primitive(), None);
    assert_eq!(TypeName::named("System", "Unknown").primitive(), None);
    assert_eq!(
        Type::value_named("Test", "Mode").type_name(),
        Some(TypeName::named("Test", "Mode"))
    );
    assert_eq!(Type::PtrMut(Box::new(Type::I32), 1).type_name(), None);
    let generic = TypeName {
        namespace: "System".into(),
        name: "Int32".into(),
        generics: vec![Type::I32],
    };
    assert_eq!(generic.primitive(), None);
    assert_eq!(Type::ClassName(generic).type_name(), None);
}

#[test]
fn named_system_type_encoding() {
    let mut file = writer::File::new("types");
    let def = file.TypeDef("Test", "Bits", Default::default(), TypeAttributes::Public);
    let attribute = writer::MemberRefParent::TypeRef(file.TypeRef("Test", "BitfieldAttribute"));
    let ctor = file.MemberRef(".ctor", &Signature::default(), attribute);
    let values = [(
        "MemberType".into(),
        Value::TypeName(TypeName::named("System", "Int32")),
    )];
    file.Attribute(
        writer::HasAttribute::TypeDef(def),
        writer::AttributeType::MemberRef(ctor),
        &values,
    );
    let bytes = file.into_stream();
    let expected = b"\x01\x00\x01\x00\x53\x50\x0aMemberType\x0cSystem.Int32";
    assert!(
        bytes
            .windows(expected.len())
            .any(|window| window == expected)
    );
    let index = reader::Index::new(vec![reader::File::new(bytes).unwrap()]);
    assert_eq!(
        index
            .expect("Test", "Bits")
            .attributes()
            .next()
            .unwrap()
            .value(),
        values
    );
}
