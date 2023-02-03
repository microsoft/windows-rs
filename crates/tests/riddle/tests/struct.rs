use test_riddle::run_riddle;
use windows_metadata::reader::*;

#[test]
fn riddle_struct() {
    let output = run_riddle("tests/struct.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("TypeNamespace", "TypeName")).next().expect("Type missing");
    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);

    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 4);

    assert_eq!(reader.field_name(fields[0]), "a");
    assert_eq!(reader.field_name(fields[1]), "b");
    assert_eq!(reader.field_name(fields[2]), "c");
    assert_eq!(reader.field_name(fields[3]), "d");

    assert!(matches!(reader.field_type(fields[0], None), Type::I32));
    assert!(matches!(reader.field_type(fields[1], None), Type::F32));
    assert!(matches!(reader.field_type(fields[2], None), Type::U64));
    assert!(matches!(reader.field_type(fields[3], None), Type::F64));
}
