use windows_bindgen as reader;
use windows_bindgen::HasAttributes;
use windows_ecma335::{writer::*, *};

#[test]
fn test() {
    let mut file = File::new("test");

    let interface_def = file.TypeDef(
        "Namespace",
        "IName",
        TypeDefOrRef::zeroed(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let guid_value = file.AttributeValue(
        &[
            Value::U32(0xd095a8ca),
            Value::U16(0x1103),
            Value::U16(0x4ef5),
            Value::U8(0x99),
            Value::U8(0x8c),
            Value::U8(0x62),
            Value::U8(0x82),
            Value::U8(0x15),
            Value::U8(0x10),
            Value::U8(0xef),
            Value::U8(0x8f),
        ],
        &[],
    );

    let guid_signature = file.MethodDefSig(
        &[
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
        &Type::Void,
        MethodCallAttributes::HASTHIS,
    );

    let guid_attribute =
        MemberRefParent::TypeRef(file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"));

    let guid_ctor = file.MemberRef(".ctor", guid_signature, guid_attribute);

    let object_name = TypeDefOrRef::TypeRef(file.TypeRef("System", "Object"));
    let interface_name = TypeDefOrRef::TypeRef(file.TypeRef("Namespace", "IName"));

    let class_def = file.TypeDef(
        "Namespace",
        "Name",
        object_name,
        TypeAttributes::Public | TypeAttributes::Sealed | TypeAttributes::WindowsRuntime,
    );

    let interface_impl = file.InterfaceImpl(class_def, interface_name);

    let default_attribute =
        MemberRefParent::TypeRef(file.TypeRef("Windows.Foundation.Metadata", "DefaultAttribute"));

    let default_signature = file.MethodDefSig(&[], &Type::Void, MethodCallAttributes::HASTHIS);
    let default_ctor = file.MemberRef(".ctor", default_signature, default_attribute);

    let default_value = file.AttributeValue(&[], &[]);

    file.Attribute(
        HasAttribute::InterfaceImpl(interface_impl),
        AttributeType::MemberRef(default_ctor),
        default_value,
    );

    file.Attribute(
        HasAttribute::TypeDef(interface_def),
        AttributeType::MemberRef(guid_ctor),
        guid_value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/class.winmd", bytes).unwrap();

    let reader = reader::Reader::new(vec![reader::File::new(
        std::fs::read("tests/class.winmd").unwrap(),
    )
    .unwrap()]);

    let types: Vec<reader::Type> = reader.with_full_name("Namespace", "Name").collect();
    assert_eq!(types.len(), 1);

    let reader::Type::Class(ref ty) = types[0] else {
        panic!()
    };

    let required_interfaces = ty.required_interfaces();
    assert_eq!(required_interfaces.len(), 1);
    assert_eq!(required_interfaces[0].def.name(), "IName");
    assert_eq!(required_interfaces[0].kind, reader::InterfaceKind::Default);

    let at: Vec<_> = required_interfaces[0].def.attributes().collect();
    assert_eq!(at.len(), 1);

    let guid = required_interfaces[0].def.guid_attribute().unwrap();
    assert_eq!(format!("{guid}"), "d095a8ca-1103-4ef5-998c-62821510ef8f");
}
