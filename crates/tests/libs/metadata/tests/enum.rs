use windows_metadata::*;

#[test]
fn test() {
    let mut file = writer::File::new("test");
    let value_type = file.TypeRef("System", "Enum");

    file.TypeDef(
        "Namespace",
        "Name",
        writer::TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public | TypeAttributes::Sealed | TypeAttributes::WindowsRuntime,
    );

    file.Field(
        "value__",
        &Type::I32,
        FieldAttributes::Private | FieldAttributes::SpecialName | FieldAttributes::RTSpecialName,
    );

    let field = file.Field(
        "Constant",
        &Type::named("Namespace", "Name"),
        FieldAttributes::Public | FieldAttributes::Static | FieldAttributes::Literal,
    );

    file.Constant(writer::HasConstant::Field(field), &Value::I32(123));

    let bytes = file.into_stream();
    std::fs::write("tests/enum.winmd", bytes).unwrap();

    let index = reader::TypeIndex::read("tests/enum.winmd").unwrap();
    let _ty = index.expect("Namespace", "Name");
}
