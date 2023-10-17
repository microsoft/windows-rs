use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("nested_struct", "winrt", &[]);
    let reader = Reader::new(files);

    let def = reader
        .get_type_def("Test", "Inner")
        .next()
        .expect("Type missing");

    assert_eq!(def.kind(), TypeKind::Struct);
    let fields: Vec<Field> = def.fields().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name(), "field_i32");
    assert!(matches!(fields[0].ty(None), Type::I32));

    let def = reader
        .get_type_def("Test", "Outer")
        .next()
        .expect("Type missing");

    assert_eq!(def.kind(), TypeKind::Struct);
    let fields: Vec<Field> = def.fields().collect();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].name(), "field_bool");
    assert_eq!(fields[1].name(), "field_inner");
    assert_eq!(fields[2].name(), "field_usize");
    assert!(matches!(fields[0].ty(None), Type::Bool));
    assert!(matches!(fields[2].ty(None), Type::USize));

    let Type::TypeDef(def, generics) = fields[1].ty(None) else {
        panic!("wrong type")
    };

    assert_eq!(def.namespace(), "Test");
    assert_eq!(def.name(), "Inner");
    assert!(generics.is_empty());
}
