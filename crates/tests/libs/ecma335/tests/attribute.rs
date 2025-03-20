use reader::HasAttributes;
use windows_bindgen as reader;
use windows_ecma335::{writer::*, *};

#[test]
fn test() {
    let mut file = File::new("test");

    let def = file.TypeDef(
        "Namespace",
        "Name",
        TypeDefOrRef::zeroed(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let attribute =
        MemberRefParent::TypeRef(file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"));

    let signature = file.MethodDefSig(
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

    let ctor = file.MemberRef(".ctor", signature, attribute);

    let value = file.AttributeValue(
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

    file.Attribute(
        HasAttribute::TypeDef(def),
        AttributeType::MemberRef(ctor),
        value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/attribute.winmd", bytes).unwrap();

    let reader = reader::Reader::new(vec![reader::File::new(
        std::fs::read("tests/attribute.winmd").unwrap(),
    )
    .unwrap()]);

    let types: Vec<reader::Type> = reader.with_full_name("Namespace", "Name").collect();
    assert_eq!(types.len(), 1);

    let reader::Type::Interface(ref ty) = types[0] else {
        panic!()
    };

    let attributes: Vec<_> = ty.def.attributes().collect();
    assert_eq!(attributes.len(), 1);
    let guid = ty.def.guid_attribute().unwrap();
    assert_eq!(format!("{guid}"), "d095a8ca-1103-4ef5-998c-62821510ef8f");
}
