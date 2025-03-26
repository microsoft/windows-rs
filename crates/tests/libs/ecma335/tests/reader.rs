use windows_ecma335::*;

#[test]
fn test() {
    let file = reader::File::read("../../../libs/bindgen/default/Windows.winmd").unwrap();

    let def = file.TypeDef().find(|def| def.name() == "Point").unwrap();
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
fn array() {
    let file = reader::File::read("../../../libs/bindgen/default/Windows.Win32.winmd").unwrap();
    let def = file
        .TypeDef()
        .find(|def| def.name() == "VDMCONTEXT")
        .unwrap();
    let field = def
        .fields()
        .find(|field| field.name() == "ExtendedRegisters")
        .unwrap();

    assert_eq!(field.ty(), Type::ArrayFixed(Box::new(Type::U8), 512));
}
