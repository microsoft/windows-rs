use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("nested_module", "winrt", &[]);
    let reader = Reader::new(files);

    let types: Vec<Item> = reader.namespace_items("Test").collect();

    assert_eq!(types.len(), 1);
    let Item::Type(def) = types[0] else {
        panic!("type expected")
    };

    assert_eq!(def.name(), "TestType");
    assert_eq!(def.kind(), TypeKind::Struct);
    let fields: Vec<Field> = def.fields().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name(), "field");
    assert!(matches!(fields[0].ty(None), Type::I32));

    let types: Vec<Item> = reader.namespace_items("Test.NestedModule").collect();

    assert_eq!(types.len(), 1);
    let Item::Type(def) = types[0] else {
        panic!("type expected")
    };

    assert_eq!(def.name(), "NestedType");
    assert_eq!(def.kind(), TypeKind::Struct);
    let fields: Vec<Field> = def.fields().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name(), "field");
    assert!(matches!(fields[0].ty(None), Type::F32));
}
