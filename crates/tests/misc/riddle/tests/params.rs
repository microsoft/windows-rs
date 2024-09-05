use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("params", "winrt", &[]);
    let reader = Reader::new(files);

    let def = reader
        .get_type_def("Test", "IParams")
        .next()
        .expect("Type missing");

    assert_eq!(def.kind(), TypeKind::Interface);
    let generics = &vec![];

    assert!(def.fields().next().is_none());
    let methods: Vec<MethodDef> = def.methods().collect();
    assert_eq!(methods.len(), 14);

    assert_eq!(methods[0].name(), "Nothing");
    let sig = methods[0].signature(generics);
    assert_eq!(sig.return_type, Type::Void);
    assert!(sig.params.is_empty());

    assert_eq!(methods[1].name(), "Bool");
    assert_eq!(methods[2].name(), "I8");
    assert_eq!(methods[3].name(), "U8");
    assert_eq!(methods[4].name(), "I16");
    assert_eq!(methods[5].name(), "U16");
    assert_eq!(methods[6].name(), "I32");
    assert_eq!(methods[7].name(), "U32");
    assert_eq!(methods[8].name(), "I64");
    assert_eq!(methods[9].name(), "U64");
    assert_eq!(methods[10].name(), "F32");
    assert_eq!(methods[11].name(), "F64");
    assert_eq!(methods[12].name(), "ISize");
    assert_eq!(methods[13].name(), "USize");

    method(generics, methods[1], Type::Bool);
    method(generics, methods[2], Type::I8);
    method(generics, methods[3], Type::U8);
    method(generics, methods[4], Type::I16);
    method(generics, methods[5], Type::U16);
    method(generics, methods[6], Type::I32);
    method(generics, methods[7], Type::U32);
    method(generics, methods[8], Type::I64);
    method(generics, methods[9], Type::U64);
    method(generics, methods[10], Type::F32);
    method(generics, methods[11], Type::F64);
    method(generics, methods[12], Type::ISize);
    method(generics, methods[13], Type::USize);
}

fn method(generics: &[Type], method: MethodDef, expected: Type) {
    let sig = method.signature(generics);
    assert_eq!(sig.return_type, expected);
    assert_eq!(sig.params.len(), 2);
    assert_eq!(sig.params[0], expected);
    assert_eq!(sig.params[1], expected);
}
