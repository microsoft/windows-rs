use windows_bindgen as reader;
use windows_ecma335::{writer::*, *};

#[test]
fn test() {
    let mut file = File::new("test");
    let value_type = file.TypeRef("System", "ValueType");

    file.TypeDef(
        "Namespace",
        "Name",
        TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public
            | TypeAttributes::SequentialLayout
            | TypeAttributes::Sealed
            | TypeAttributes::WindowsRuntime,
    );

    let signature = file.FieldSig(&Type::named("System", "Guid"));
    file.Field("SomeGuid", signature, FieldAttributes::Public);

    let signature = file.FieldSig(&Type::I32);
    file.Field("SomeNum", signature, FieldAttributes::Public);

    let bytes = file.into_stream();
    std::fs::write("tests/struct.winmd", bytes).unwrap();

    let reader = reader::Reader::new(vec![reader::File::new(
        std::fs::read("tests/struct.winmd").unwrap(),
    )
    .unwrap()]);

    let types: Vec<reader::Type> = reader.with_full_name("Namespace", "Name").collect();
    assert_eq!(types.len(), 1);

    let reader::Type::Struct(ref ty) = types[0] else {
        panic!()
    };

    let fields: Vec<_> = ty.def.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "SomeGuid");
    assert_eq!(fields[0].ty(None), reader::Type::GUID);
    assert_eq!(fields[1].name(), "SomeNum");
    assert_eq!(fields[1].ty(None), reader::Type::I32);
}
