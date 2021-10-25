#[test]
fn named_arguments() {
    let reader = reader::TypeReader::get();
    let type_def = reader.expect_type_def(reader::TypeName::new("Component.Attributes", "Test"));

    let mut some_string = 0;
    let mut some_int = 0;
    let mut some_bool = 0;

    for attribute in type_def.attributes() {
        match attribute.name() {
            "CustomTestAttribute" => {
                for (name, arg) in attribute.args() {
                    match (&name as &str, &arg) {
                        ("SomeString", reader::ConstantValue::String(value)) => {
                            assert_eq!(value, "Hello, World!");
                            some_string += 1;
                        }
                        ("SomeInt", reader::ConstantValue::I32(value)) => {
                            assert_eq!(*value, 1975);
                            some_int += 1;
                        }
                        ("SomeBool", reader::ConstantValue::Bool(value)) => {
                            assert_eq!(*value, true);
                            some_bool += 1;
                        }
                        _ => panic!("Unexpected named argument! Name: '{}'", name),
                    }
                }
            }
            _ => {}
        }
    }

    assert_eq!(some_string, 1);
    assert_eq!(some_int, 1);
    assert_eq!(some_bool, 1);
}
