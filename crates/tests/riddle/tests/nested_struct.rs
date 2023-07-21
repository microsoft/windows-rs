use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("nested_struct", &[]);
    let reader = &Reader::new(&files);

    let def = reader
        .get_type_def(TypeName::new("Test", "Inner"))
        .next()
        .expect("Type missing");

    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(reader.field_name(fields[0]), "field_i32");
    assert!(matches!(reader.field_type(fields[0], None), Type::I32));

    let def = reader
        .get_type_def(TypeName::new("Test", "Outer"))
        .next()
        .expect("Type missing");

    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 3);
    assert_eq!(reader.field_name(fields[0]), "field_bool");
    assert_eq!(reader.field_name(fields[1]), "field_inner");
    assert_eq!(reader.field_name(fields[2]), "field_usize");
    assert!(matches!(reader.field_type(fields[0], None), Type::Bool));
    assert!(matches!(reader.field_type(fields[2], None), Type::USize));

    let Type::TypeDef(def, generics) = reader.field_type(fields[1], None) else {
        panic!("wrong type")
    };

    assert_eq!(reader.type_def_namespace(def), "Test");
    assert_eq!(reader.type_def_name(def), "Inner");
    assert!(generics.is_empty());
}
