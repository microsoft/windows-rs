use test_riddle::run_riddle;
use windows_metadata::reader::*;

#[test]
fn riddle_enum() {
    let output = run_riddle("tests/enum.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let def = reader.get(TypeName::new("TypeNamespace", "TypeName")).next().expect("Type missing");
    assert_eq!(reader.type_def_kind(def), TypeKind::Enum);

    let fields: Vec<Field> = reader.type_def_fields(def).collect();
    assert_eq!(fields.len(), 5);
    assert_eq!(reader.field_name(fields[0]), "value__");

    assert_eq!(reader.field_name(fields[1]), "A");
    assert_eq!(reader.field_name(fields[2]), "B");
    assert_eq!(reader.field_name(fields[3]), "C");
    assert_eq!(reader.field_name(fields[4]), "D");

    assert!(matches!(reader.constant_value(reader.field_constant(fields[1]).expect("missing constant")), Value::I32(value) if value == 0));
    assert!(matches!(reader.constant_value(reader.field_constant(fields[2]).expect("missing constant")), Value::I32(value) if value == 1));
    assert!(matches!(reader.constant_value(reader.field_constant(fields[3]).expect("missing constant")), Value::I32(value) if value == 42));
    assert!(matches!(reader.constant_value(reader.field_constant(fields[4]).expect("missing constant")), Value::I32(value) if value == 43));
}
