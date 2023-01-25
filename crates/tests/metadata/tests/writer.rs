#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test2.winmd");
    {
        use metadata::writer2::*;

        let mut items = vec![];

        items.push(Struct::new("test_component2", "Inner", vec![Field::new("Value32", Type::F32), Field::new("Value64", Type::F64)]));

        items.push(Struct::new("test_component2", "Outer", vec![Field::new("Value", Type::named("test_component2", "Inner"))]));

        items.push(Enum::new("test_component2", "Flags", vec![Constant::new("One", Value::I32(1)), Constant::new("Two", Value::I32(2))]));

        let buffer = write("test_component2", true, &items, &[]);
        std::fs::write(temp_file, buffer).unwrap();
    }
}
