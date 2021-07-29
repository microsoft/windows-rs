#[test]
fn named_arguments() {
    let reader = gen::TypeReader::get();
    let type_def = reader.resolve_type_def(gen::TypeName::new("TestComponent", "TestRunner"));

    // TestRunner should have a custom attribute on it
    let mut some_string = 0;
    let mut some_int = 0;
    let mut some_bool = 0;
    for attribute in type_def.attributes() {
        match attribute.name() {
            "CustomTestAttribute" => {
                for (name, arg) in attribute.args() {
                    match (&name as &str, &arg) {
                        ("SomeString", gen::ConstantValue::String(value)) => {
                            assert_eq!(value, "Hello, World!");
                            some_string += 1;
                        }
                        ("SomeInt", gen::ConstantValue::I32(value)) => {
                            assert_eq!(*value, 1975);
                            some_int += 1;
                        }
                        ("SomeBool", gen::ConstantValue::Bool(value)) => {
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
