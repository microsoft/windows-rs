use windows_ecma335::*;

#[test]
fn test() {
    let mut file = writer::File::new("test");

    let interface_def = file.TypeDef(
        "Namespace",
        "IName",
        Default::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let guid_value = vec![
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

    let guid_signature = Signature {
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

    let guid_attribute = writer::MemberRefParent::TypeRef(
        file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"),
    );

    let guid_ctor = file.MemberRef(".ctor", &guid_signature, guid_attribute);

    let object_name = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "Object"));

    let class_def = file.TypeDef(
        "Namespace",
        "Name",
        object_name,
        TypeAttributes::Public | TypeAttributes::Sealed | TypeAttributes::WindowsRuntime,
    );

    let interface_impl = file.InterfaceImpl(class_def, &Type::named("Namespace", "IName"));

    let default_attribute = writer::MemberRefParent::TypeRef(
        file.TypeRef("Windows.Foundation.Metadata", "DefaultAttribute"),
    );

    let default_ctor = file.MemberRef(".ctor", &Signature::default(), default_attribute);

    file.Attribute(
        writer::HasAttribute::InterfaceImpl(interface_impl),
        writer::AttributeType::MemberRef(default_ctor),
        &[],
    );

    file.Attribute(
        writer::HasAttribute::TypeDef(interface_def),
        writer::AttributeType::MemberRef(guid_ctor),
        &guid_value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/class.winmd", bytes).unwrap();

    let reader = reader::File::read("tests/class.winmd").unwrap();

    let _ty = reader.TypeDef().find(|def| def.name() == "Name").unwrap();

    // let required_interfaces = ty.required_interfaces();
    // assert_eq!(required_interfaces.len(), 1);
    // assert_eq!(required_interfaces[0].def.name(), "IName");
    // assert_eq!(required_interfaces[0].kind, reader::InterfaceKind::Default);

    // let at: Vec<_> = required_interfaces[0].def.attributes().collect();
    // assert_eq!(at.len(), 1);

    // let guid = required_interfaces[0].def.guid_attribute().unwrap();
    // assert_eq!(format!("{guid}"), "d095a8ca-1103-4ef5-998c-62821510ef8f");
}
