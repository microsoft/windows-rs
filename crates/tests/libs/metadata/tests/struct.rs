use windows_metadata::*;

#[test]
fn test() {
    let mut file = writer::File::new("test");
    let value_type = file.TypeRef("System", "ValueType");

    file.TypeDef(
        "Namespace",
        "Name",
        writer::TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public
            | TypeAttributes::SequentialLayout
            | TypeAttributes::Sealed
            | TypeAttributes::WindowsRuntime,
    );

    file.Field(
        "SomeGuid",
        &Type::named("System", "Guid"),
        FieldAttributes::Public,
    );

    file.Field("SomeNum", &Type::I32, FieldAttributes::Public);

    let bytes = file.into_stream();
    std::fs::write("tests/struct.winmd", bytes).unwrap();

    let index = reader::TypeIndex::read("tests/struct.winmd").unwrap();
    let ty = index.expect("Namespace", "Name");

    let fields: Vec<_> = ty.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "SomeGuid");
    assert_eq!(fields[0].ty(), Type::named("System", "Guid"));
    assert_eq!(fields[1].name(), "SomeNum");
    assert_eq!(fields[1].ty(), Type::I32);
}
