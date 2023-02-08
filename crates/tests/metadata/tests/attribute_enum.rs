use metadata::reader::{Attribute, File, Integer, Reader, TypeName, Value};

#[test]
fn attribute_enum() {
    let files = File::with_default(&[]).unwrap();
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("Windows.Win32.UI.WindowsAndMessaging", "Apis")).next().unwrap();
    let method = reader.type_def_methods(def).find(|&m| reader.method_def_name(m) == "SetWindowLongPtrA").unwrap();
    let attrs = reader.method_def_attributes(method);
    check_attr_arg_enum(reader, attrs, "SupportedArchitectureAttribute", "", "Windows.Win32.Interop.Architecture", 4 | 2);

    // The only attribute currently with a named enum argument is "GCPressure".
    let def = reader.get(TypeName::new("Windows.Graphics.Imaging", "BitmapBuffer")).next().unwrap();
    let attrs = reader.type_def_attributes(def);
    check_attr_arg_enum(reader, attrs, "GCPressureAttribute", "amount", "Windows.Foundation.Metadata.GCPressureAmount", 2);
}

fn check_attr_arg_enum(reader: &Reader, mut attrs: impl Iterator<Item = Attribute>, attr_name: &str, arg_name: &str, expected_type: &str, expected_value: i32) {
    let attr = attrs.find(|&attr| reader.attribute_name(attr) == attr_name).unwrap();
    let (_, value) = reader.attribute_args(attr).drain(..).find(|(name, _)| name == arg_name).unwrap();

    if let Value::Enum(ty, Integer::I32(value)) = value {
        assert_eq!(expected_type, reader.type_def_type_name(ty).to_string());
        assert_eq!(expected_value, value);
    } else {
        panic!("Value not found");
    }
}
