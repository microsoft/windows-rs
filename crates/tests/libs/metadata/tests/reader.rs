use windows_metadata::*;

#[test]
fn type_index() {
    let index = reader::TypeIndex::read("../../../libs/bindgen/default/Windows.winmd").unwrap();

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
    let index = reader::TypeIndex::new(vec![
        reader::File::read("../../../libs/bindgen/default/Windows.winmd").unwrap(),
        reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap(),
        reader::File::read("../../../libs/bindgen/default/Windows.Wdk.winmd").unwrap(),
    ]);

    let index = reader::ItemIndex::new(&index);

    let reader::Item::Type(ty) = index.expect("Windows.Foundation", "Point") else {
        panic!()
    };
    assert_eq!(ty.namespace(), "Windows.Foundation");
    assert_eq!(ty.name(), "Point");

    let reader::Item::Fn(function) = index.expect("Windows.Win32.Storage.FileSystem", "ReadFileEx")
    else {
        panic!()
    };
    assert_eq!(function.name(), "ReadFileEx");

    let reader::Item::Const(constant) = index.expect("Windows.Win32.Foundation", "CONTROL_C_EXIT")
    else {
        panic!()
    };
    assert_eq!(constant.name(), "CONTROL_C_EXIT");

    let reader::Item::Const(constant) =
        index.expect("Windows.Win32.Foundation", "FACILITY_DEBUGGER")
    else {
        panic!()
    };
    assert_eq!(constant.name(), "FACILITY_DEBUGGER");
}

#[test]
fn array() {
    let index =
        reader::TypeIndex::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();
    let def = index
        .types()
        .find(|def| def.name() == "VDMCONTEXT")
        .unwrap();

    let field = def
        .fields()
        .find(|field| field.name() == "ExtendedRegisters")
        .unwrap();

    assert_eq!(field.ty(), Type::ArrayFixed(Box::new(Type::U8), 512));
}
