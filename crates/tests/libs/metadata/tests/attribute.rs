use windows_metadata::*;

#[test]
fn test() {
    let mut file = writer::File::new("test");

    let def = file.TypeDef(
        "Namespace",
        "Name",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let attribute = writer::MemberRefParent::TypeRef(
        file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"),
    );

    let signature = Signature {
        types: vec![
            Type::U32,
            Type::U16,
            Type::U16,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
        ],
        ..Default::default()
    };

    let ctor = file.MemberRef(".ctor", &signature, attribute);

    let value = vec![
        (String::new(), Value::U32(0xd095a8ca)),
        (String::new(), Value::U16(0x1103)),
        (String::new(), Value::U16(0x4ef5)),
        (String::new(), Value::U8(0x99)),
        (String::new(), Value::U8(0x8c)),
        (String::new(), Value::U8(0x62)),
        (String::new(), Value::U8(0x82)),
        (String::new(), Value::U8(0x15)),
        (String::new(), Value::U8(0x10)),
        (String::new(), Value::U8(0xef)),
        (String::new(), Value::U8(0x8f)),
    ];

    file.Attribute(
        writer::HasAttribute::TypeDef(def),
        writer::AttributeType::MemberRef(ctor),
        &value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/attribute.winmd", bytes).unwrap();

    let index = reader::TypeIndex::read("tests/attribute.winmd").unwrap();
    let ty = index.expect("Namespace", "Name");

    let attributes: Vec<_> = ty.attributes().collect();
    assert_eq!(attributes.len(), 1);
}
