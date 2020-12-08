use winrt::*;

#[test]
fn named_arguments() -> Result<()> {
    let reader = winmd::TypeReader::from_build();
    let type_def = reader.resolve_type_def(("TestComponent", "TestRunner"));

    // TestRunner should have a custom attribute on it
    let mut some_string = 0;
    let mut some_int = 0;
    let mut some_bool = 0;
    for attribute in type_def.attributes() {
        match attribute.name() {
            ("TestComponent", "CustomTestAttribute") => {
                for (name, arg) in attribute.args() {
                    match (&name as &str, &arg) {
                        ("SomeString", winmd::AttributeArg::String(value)) => {
                            assert_eq!(value, "Hello, World!");
                            some_string += 1;
                        }
                        ("SomeInt", winmd::AttributeArg::I32(value)) => {
                            assert_eq!(*value, 1975);
                            some_int += 1;
                        }
                        ("SomeBool", winmd::AttributeArg::Bool(value)) => {
                            assert_eq!(*value, true);
                            some_bool += 1;
                        }
                        _ => panic!(
                            "Unexpected named argument! Name: '{}'  Arg: {:?}",
                            name, arg
                        ),
                    }
                }
            }
            _ => {}
        }
    }
    assert_eq!(some_string, 1);
    assert_eq!(some_int, 1);
    assert_eq!(some_bool, 1);

    Ok(())
}
