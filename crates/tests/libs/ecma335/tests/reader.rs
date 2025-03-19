use windows_ecma335::{*, reader::*};

#[test]
fn test() {
    let file = File::read("../../../libs/bindgen/default/Windows.winmd").unwrap();

    let def = file
        .table::<TypeDef>()
        .find(|def| def.name() == "Point")
        .unwrap();

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
