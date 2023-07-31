use metadata::*;

#[test]
fn attribute_enum() {
    let files = tool_lib::default_metadata();
    let reader = &Reader::new(&files);

    let (method, _) = reader
        .get_method_def(TypeName::new(
            "Windows.Win32.UI.WindowsAndMessaging",
            "SetWindowLongPtrA",
        ))
        .next()
        .unwrap();

    check_attr_arg_enum(
        reader,
        reader
            .find_attribute(method, "SupportedArchitectureAttribute")
            .unwrap(),
        "",
        "Windows.Win32.Foundation.Metadata.Architecture",
        4 | 2,
    );

    // The only attribute currently with a named enum argument is "GCPressure".
    let def = reader
        .get_type_def(TypeName::new("Windows.Graphics.Imaging", "BitmapBuffer"))
        .next()
        .unwrap();

    check_attr_arg_enum(
        reader,
        reader.find_attribute(def, "GCPressureAttribute").unwrap(),
        "amount",
        "Windows.Foundation.Metadata.GCPressureAmount",
        2,
    );
}

fn check_attr_arg_enum(
    reader: &Reader,
    attr: Attribute,
    arg_name: &str,
    expected_type: &str,
    expected_value: i32,
) {
    let (_, value) = reader
        .attribute_args(attr)
        .drain(..)
        .find(|(name, _)| name == arg_name)
        .unwrap();

    if let Value::EnumDef(ty, value) = value {
        if let Value::I32(value) = *value {
            assert_eq!(expected_type, reader.type_def_type_name(ty).to_string());
            assert_eq!(expected_value, value);
            return;
        }
    }

    panic!("Value not found");
}
