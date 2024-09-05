use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("win32_struct", "win32", &[]);
    let reader = Reader::new(files);

    let def = reader
        .get_type_def("Test", "Type")
        .next()
        .expect("Type missing");

    let flags = def.flags();
    assert!(!flags.contains(TypeAttributes::WindowsRuntime));

    assert_eq!(def.kind(), TypeKind::Struct);
    let fields: Vec<Field> = def.fields().collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name(), "field");
    assert!(matches!(fields[0].ty(None), Type::I32));
}
