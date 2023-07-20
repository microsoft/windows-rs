use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("nested_module");
    let reader = &Reader::new(&files);

    let types: Vec<Item> = reader
        .namespace_items("Test", &Default::default())
        .collect();

    assert_eq!(types.len(), 1);
    let Item::Type(def) = types[0] else { panic!("type expected") };

    assert_eq!(reader.type_def_name(def), "TestType");
    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(reader.field_name(fields[0]), "field");
    assert!(matches!(reader.field_type(fields[0], None), Type::I32));

    let types: Vec<Item> = reader
        .namespace_items("Test.NestedModule", &Default::default())
        .collect();

    assert_eq!(types.len(), 1);
    let Item::Type(def) = types[0] else { panic!("type expected") };

    assert_eq!(reader.type_def_name(def), "NestedType");
    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(reader.field_name(fields[0]), "field");
    assert!(matches!(reader.field_type(fields[0], None), Type::F32));
}
