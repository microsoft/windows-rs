use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("params", "winrt", &[]);
    let reader = &Reader::new(&files);

    let def = reader
        .get_type_def(TypeName::new("Test", "IParams"))
        .next()
        .expect("Type missing");

    assert_eq!(reader.type_def_kind(def), TypeKind::Interface);
    let generics = &vec![];

    assert!(reader.type_def_fields(def).next().is_none());
    let methods: Vec<MethodDef> = reader.type_def_methods(def).collect();
    assert_eq!(methods.len(), 14);

    assert_eq!(reader.method_def_name(methods[0]), "Nothing");
    let sig = reader.method_def_signature(methods[0], generics);
    assert_eq!(sig.return_type, Type::Void);
    assert!(sig.params.is_empty());

    assert_eq!(reader.method_def_name(methods[1]), "Bool");
    assert_eq!(reader.method_def_name(methods[2]), "I8");
    assert_eq!(reader.method_def_name(methods[3]), "U8");
    assert_eq!(reader.method_def_name(methods[4]), "I16");
    assert_eq!(reader.method_def_name(methods[5]), "U16");
    assert_eq!(reader.method_def_name(methods[6]), "I32");
    assert_eq!(reader.method_def_name(methods[7]), "U32");
    assert_eq!(reader.method_def_name(methods[8]), "I64");
    assert_eq!(reader.method_def_name(methods[9]), "U64");
    assert_eq!(reader.method_def_name(methods[10]), "F32");
    assert_eq!(reader.method_def_name(methods[11]), "F64");
    assert_eq!(reader.method_def_name(methods[12]), "ISize");
    assert_eq!(reader.method_def_name(methods[13]), "USize");

    method(reader, generics, methods[1], Type::Bool);
    method(reader, generics, methods[2], Type::I8);
    method(reader, generics, methods[3], Type::U8);
    method(reader, generics, methods[4], Type::I16);
    method(reader, generics, methods[5], Type::U16);
    method(reader, generics, methods[6], Type::I32);
    method(reader, generics, methods[7], Type::U32);
    method(reader, generics, methods[8], Type::I64);
    method(reader, generics, methods[9], Type::U64);
    method(reader, generics, methods[10], Type::F32);
    method(reader, generics, methods[11], Type::F64);
    method(reader, generics, methods[12], Type::ISize);
    method(reader, generics, methods[13], Type::USize);
}

fn method(reader: &Reader, generics: &[Type], method: MethodDef, expected: Type) {
    let sig = reader.method_def_signature(method, generics);
    assert_eq!(sig.return_type, expected);
    assert_eq!(sig.params.len(), 2);
    assert_eq!(sig.params[0], expected);
    assert_eq!(sig.params[1], expected);
}
