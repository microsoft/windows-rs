#[test]
fn writer() {
    // ildasm %temp%\test_metadata.winmd
    let temp_file = std::env::temp_dir().join("test_metadata.winmd");
    {
        use metadata::writer::*;

        let mut items = vec![];

        items.push(Struct::item("test_metadata", "A", vec![Field::new("A1", Type::F32), Field::new("A2", Type::F64)]));
        items.push(Struct::item("test_metadata", "B", vec![Field::new("B1", Type::F32), Field::new("B2", Type::named("test_metadata", "A"))]));

        items.push(Enum::item("test_metadata", "C", vec![Constant::new("C1", Value::I32(1)), Constant::new("C2", Value::I32(2))]));
        items.push(Enum::item("test_metadata", "D", vec![Constant::new("D1", Value::I32(3)), Constant::new("D2", Value::I32(4))]));

        items.push(Interface::item("test_metadata", "E", vec![Method::new("E1", Type::Void, vec![]), Method::new("E2", Type::I32, vec![Param::new("p1", Type::I32, ParamFlags::INPUT), Param::new("p2", Type::F32, ParamFlags::OUTPUT), Param::new("p3", Type::F32, ParamFlags::INPUT | ParamFlags::OUTPUT)])]));
        items.push(Interface::item("test_metadata", "F", vec![Method::new("F1", Type::Void, vec![]), Method::new("F2", Type::Void, vec![])]));

        let buffer = write("test_metadata", true, &items, &[]);
        std::fs::write(temp_file, buffer).unwrap();
    }
}
