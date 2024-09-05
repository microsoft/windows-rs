use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("struct", "winrt", &[]);
    let reader = Reader::new(files);

    let def = reader
        .get_type_def("Test", "Primitives")
        .next()
        .expect("Type missing");

    assert_eq!(def.kind(), TypeKind::Struct);

    let fields: Vec<Field> = def.fields().collect();
    assert_eq!(fields.len(), 13);

    assert_eq!(fields[0].name(), "field_bool");
    assert_eq!(fields[1].name(), "field_i8");
    assert_eq!(fields[2].name(), "field_u8");
    assert_eq!(fields[3].name(), "field_i16");
    assert_eq!(fields[4].name(), "field_u16");
    assert_eq!(fields[5].name(), "field_i32");
    assert_eq!(fields[6].name(), "field_u32");
    assert_eq!(fields[7].name(), "field_i64");
    assert_eq!(fields[8].name(), "field_u64");
    assert_eq!(fields[9].name(), "field_f32");
    assert_eq!(fields[10].name(), "field_f64");
    assert_eq!(fields[11].name(), "field_isize");
    assert_eq!(fields[12].name(), "field_usize");

    assert!(matches!(fields[0].ty(None), Type::Bool));
    assert!(matches!(fields[1].ty(None), Type::I8));
    assert!(matches!(fields[2].ty(None), Type::U8));
    assert!(matches!(fields[3].ty(None), Type::I16));
    assert!(matches!(fields[4].ty(None), Type::U16));
    assert!(matches!(fields[5].ty(None), Type::I32));
    assert!(matches!(fields[6].ty(None), Type::U32));
    assert!(matches!(fields[7].ty(None), Type::I64));
    assert!(matches!(fields[8].ty(None), Type::U64));
    assert!(matches!(fields[9].ty(None), Type::F32));
    assert!(matches!(fields[10].ty(None), Type::F64));
    assert!(matches!(fields[11].ty(None), Type::ISize));
    assert!(matches!(fields[12].ty(None), Type::USize));
}
