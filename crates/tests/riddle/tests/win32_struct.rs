use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("win32_struct","win32", &[]);
    let reader = &Reader::new(&files);

    let def = reader
        .get_type_def(TypeName::new("Test", "Type"))
        .next()
        .expect("Type missing");

    let flags = reader.type_def_flags(def);
    assert!(!flags.contains(TypeAttributes::WindowsRuntime));

    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);
    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(reader.field_name(fields[0]), "field");
    assert!(matches!(reader.field_type(fields[0], None), Type::I32));
}
