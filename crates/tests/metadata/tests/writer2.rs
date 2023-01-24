#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test2.winmd");
    {
        use metadata::writer2::*;

        let mut items = vec![];

        items.push(Struct::new("Test", "Inner", vec![Field::new("Value32", Type::F32), Field::new("Value64", Type::F64)]));

        items.push(Struct::new("Test", "Outer", vec![Field::new("Value", Type::new("Test", "Inner"))]));

        items.push(Enum::new("Test", "Things", vec![Constant::new("One", Value::U32(1)), Constant::new("Twoish", Value::F32(2.0))]));

        let buffer = write("module", true, &items, &[]);
        std::fs::write(temp_file, buffer).unwrap();
    }
}
