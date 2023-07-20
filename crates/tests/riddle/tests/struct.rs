use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("struct");
    let reader = &Reader::new(&files);

    let def = reader
        .get_type_def(TypeName::new("Test", "Primitives"))
        .next()
        .expect("Type missing");

    assert_eq!(reader.type_def_kind(def), TypeKind::Struct);

    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 13);

    assert_eq!(reader.field_name(fields[0]), "field_bool");
    assert_eq!(reader.field_name(fields[1]), "field_i8");
    assert_eq!(reader.field_name(fields[2]), "field_u8");
    assert_eq!(reader.field_name(fields[3]), "field_i16");
    assert_eq!(reader.field_name(fields[4]), "field_u16");
    assert_eq!(reader.field_name(fields[5]), "field_i32");
    assert_eq!(reader.field_name(fields[6]), "field_u32");
    assert_eq!(reader.field_name(fields[7]), "field_i64");
    assert_eq!(reader.field_name(fields[8]), "field_u64");
    assert_eq!(reader.field_name(fields[9]), "field_f32");
    assert_eq!(reader.field_name(fields[10]), "field_f64");
    assert_eq!(reader.field_name(fields[11]), "field_isize");
    assert_eq!(reader.field_name(fields[12]), "field_usize");

    assert!(matches!(reader.field_type(fields[0], None), Type::Bool));
    assert!(matches!(reader.field_type(fields[1], None), Type::I8));
    assert!(matches!(reader.field_type(fields[2], None), Type::U8));
    assert!(matches!(reader.field_type(fields[3], None), Type::I16));
    assert!(matches!(reader.field_type(fields[4], None), Type::U16));
    assert!(matches!(reader.field_type(fields[5], None), Type::I32));
    assert!(matches!(reader.field_type(fields[6], None), Type::U32));
    assert!(matches!(reader.field_type(fields[7], None), Type::I64));
    assert!(matches!(reader.field_type(fields[8], None), Type::U64));
    assert!(matches!(reader.field_type(fields[9], None), Type::F32));
    assert!(matches!(reader.field_type(fields[10], None), Type::F64));
    assert!(matches!(reader.field_type(fields[11], None), Type::ISize));
    assert!(matches!(reader.field_type(fields[12], None), Type::USize));
}
