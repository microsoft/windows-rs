#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test2.winmd");
    {
        use metadata::writer2::*;

        let mut items = vec![];

        items.push(Item::Struct(Struct { namespace: "Test".to_string(), name: "Inner".to_string(), fields: vec![Field::new("Value32", Type::F32), Field::new("Value64", Type::F64)] }));

        items.push(Item::Struct(Struct { namespace: "Test".to_string(), name: "Outer".to_string(), fields: vec![Field::new("Value", Type::new("Test", "Inner"))] }));

        items.push(Item::Enum(Enum {
            namespace: "Test".to_string(),
            name: "Things".to_string(),
            constants: vec![Constant::new("One", Value::U32(1)), Constant::new("Twoish", Value::F32(2.0))],
        }));

        let buffer = write("module", true, &items, &[]);
        std::fs::write(temp_file, buffer).unwrap();
    }
}
