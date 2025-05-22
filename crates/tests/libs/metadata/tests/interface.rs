use windows_metadata::*;

#[test]
fn test() {
    let mut file = writer::File::new("test");

    file.TypeDef(
        "Namespace",
        "Name",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let flags = MethodAttributes::Public
        | MethodAttributes::HideBySig
        | MethodAttributes::Abstract
        | MethodAttributes::NewSlot
        | MethodAttributes::Virtual;

    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::I32,
        types: vec![Type::I8, Type::I16],
    };

    file.MethodDef("One", &signature, flags, Default::default());
    file.Param("i8", 1, ParamAttributes::In);
    file.Param("i16", 2, ParamAttributes::In);

    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::String,
        types: vec![],
    };

    file.MethodDef("Two", &signature, flags, Default::default());

    let bytes = file.into_stream();
    std::fs::write("tests/interface.winmd", bytes).unwrap();

    let index = reader::TypeIndex::read("tests/interface.winmd").unwrap();
    let ty = index.expect("Namespace", "Name");

    let methods: Vec<_> = ty.methods().collect();
    assert_eq!(methods.len(), 2);

    assert_eq!(methods[0].name(), "One");
    let sig = methods[0].signature(&[]);
    assert_eq!(sig.flags, MethodCallAttributes::HASTHIS);
    assert_eq!(sig.return_type, Type::I32);
    assert_eq!(sig.types.len(), 2);
    assert_eq!(sig.types[0], Type::I8);
    assert_eq!(sig.types[1], Type::I16);

    assert_eq!(methods[1].name(), "Two");
    let sig = methods[1].signature(&[]);
    assert_eq!(sig.flags, MethodCallAttributes::HASTHIS);
    assert_eq!(sig.return_type, Type::String);
    assert_eq!(sig.types.len(), 0);
}
