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
    let attrs = reader.method_def_attributes(method);
    check_attr_arg_enum(
        reader,
        attrs,
        "SupportedArchitectureAttribute",
        "",
        "Windows.Win32.Foundation.Metadata.Architecture",
        4 | 2,
    );

    // The only attribute currently with a named enum argument is "GCPressure".
    let def = reader
        .get_type_def(TypeName::new("Windows.Graphics.Imaging", "BitmapBuffer"))
        .next()
        .unwrap();
    let attrs = reader.type_def_attributes(def);
    check_attr_arg_enum(
        reader,
        attrs,
        "GCPressureAttribute",
        "amount",
        "Windows.Foundation.Metadata.GCPressureAmount",
        2,
    );
}

fn check_attr_arg_enum(
    reader: &Reader,
    mut attrs: impl Iterator<Item = Attribute>,
    attr_name: &str,
    arg_name: &str,
    expected_type: &str,
    expected_value: i32,
) {
    let attr = attrs
        .find(|&attr| reader.attribute_name(attr) == attr_name)
        .unwrap();
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
