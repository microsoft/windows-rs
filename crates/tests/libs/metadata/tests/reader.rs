use windows_metadata::*;

#[test]
fn type_index() {
    let index = reader::Index::read("../../../libs/bindgen/default/Windows.winmd").unwrap();

    let def = index.expect("Windows.Foundation", "Point");
    assert_eq!(def.namespace(), "Windows.Foundation");
    assert_eq!(def.name(), "Point");

    let extends = def.extends().unwrap();
    assert_eq!(extends.namespace(), "System");
    assert_eq!(extends.name(), "ValueType");

    let fields: Vec<_> = def.fields().collect();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name(), "X");
    assert_eq!(fields[1].name(), "Y");
    assert_eq!(fields[0].ty(), Type::F32);
    assert_eq!(fields[1].ty(), Type::F32);
}

#[test]
fn item_index() {
    let index = reader::Index::new(vec![
        reader::File::read("../../../libs/bindgen/default/Windows.winmd").unwrap(),
        reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap(),
        reader::File::read("../../../libs/bindgen/default/Windows.Wdk.winmd").unwrap(),
    ]);

    let reader::Item::Type(ty) = index.expect_item("Windows.Foundation", "Point") else {
        panic!()
    };
    assert_eq!(ty.namespace(), "Windows.Foundation");
    assert_eq!(ty.name(), "Point");

    let reader::Item::Fn(function) = index.expect_item("Windows.Win32", "ReadFileEx") else {
        panic!()
    };
    assert_eq!(function.name(), "ReadFileEx");

    let reader::Item::Const(constant) = index.expect_item("Windows.Win32", "CONTROL_C_EXIT")
    else {
        panic!()
    };
    assert_eq!(constant.name(), "CONTROL_C_EXIT");
}

#[test]
fn array() {
    let index =
        reader::Index::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();
    let def = index
        .types()
        .find(|def| def.name() == "SID_IDENTIFIER_AUTHORITY")
        .unwrap();

    let field = def.fields().find(|field| field.name() == "Value").unwrap();

    assert_eq!(field.ty(), Type::ArrayFixed(Box::new(Type::U8), 6));
}

#[test]
fn nested() {
    let index =
        reader::Index::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();

    let def = index
        .types()
        .find(|def| def.name() == "D3D10_BUFFER_RTV")
        .unwrap();

    let fields: Vec<reader::Field> = def.fields().collect();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "Anonymous");
    assert_eq!(fields[1].name(), "Anonymous2");

    assert_eq!(fields[0].ty(), Type::value_named("", "D3D10_BUFFER_RTV_0"));
    assert_eq!(fields[1].ty(), Type::value_named("", "D3D10_BUFFER_RTV_1"));

    let types: Vec<reader::TypeDef> = index.nested(def).collect();
    assert_eq!(types.len(), 2);

    assert_eq!(types[0].namespace(), "");
    assert_eq!(types[1].namespace(), "");

    assert_eq!(types[0].name(), "D3D10_BUFFER_RTV_0");
    assert_eq!(types[1].name(), "D3D10_BUFFER_RTV_1");

    let fields: Vec<reader::Field> = types[0].fields().collect();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "FirstElement");
    assert_eq!(fields[1].name(), "ElementOffset");

    let fields: Vec<reader::Field> = types[1].fields().collect();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name(), "NumElements");
    assert_eq!(fields[1].name(), "ElementWidth");
}
