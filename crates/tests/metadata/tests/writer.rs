#[test]
fn writer() {
    // ildasm %temp%\test_metadata.winmd
    let temp_file = std::env::temp_dir().join("test_metadata.winmd");
    {
        use metadata::writer::*;

        let mut items = vec![];

        items.push(Struct::item("test_metadata", "Inner", vec![Field::new("Value32", Type::F32), Field::new("Value64", Type::F64)]));

        items.push(Struct::item("test_metadata", "Outer", vec![Field::new("Value", Type::named("test_metadata", "Inner"))]));

        items.push(Enum::item("test_metadata", "Flags", vec![Constant::new("One", Value::I32(1)), Constant::new("Two", Value::I32(2))]));

        let buffer = write("test_metadata", true, &items, &[]);
        std::fs::write(temp_file, buffer).unwrap();
    }
}
