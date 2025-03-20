use windows_bindgen as reader;
use windows_ecma335::{writer::*, *};

#[test]
fn test() {
    let mut file = File::new("test");

    file.TypeDef(
        "Namespace",
        "Name",
        TypeDefOrRef::zeroed(),
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

    let signature = file.MethodDefSig(
        &[Type::I8, Type::I16],
        &Type::I32,
        MethodCallAttributes::HASTHIS,
    );
    file.MethodDef("One", signature, flags, MethodImplAttributes(0));
    file.Param("i8", 1, ParamAttributes::In);
    file.Param("i16", 2, ParamAttributes::In);

    let signature = file.MethodDefSig(&[], &Type::String, MethodCallAttributes::HASTHIS);
    file.MethodDef("Two", signature, flags, MethodImplAttributes(0));

    let bytes = file.into_stream();
    std::fs::write("tests/interface.winmd", bytes).unwrap();

    let reader = reader::Reader::new(vec![reader::File::new(
        std::fs::read("tests/interface.winmd").unwrap(),
    )
    .unwrap()]);

    let types: Vec<reader::Type> = reader.with_full_name("Namespace", "Name").collect();
    assert_eq!(types.len(), 1);

    let reader::Type::Interface(ref ty) = types[0] else {
        panic!()
    };

    let methods: Vec<_> = ty.def.methods().collect();
    assert_eq!(methods.len(), 2);

    assert_eq!(methods[0].name(), "One");
    let sig = methods[0].signature("Namespace", &[]);
    assert_eq!(sig.call_flags, reader::MethodCallAttributes::HASTHIS);
    assert_eq!(sig.return_type, reader::Type::I32);
    assert_eq!(sig.params.len(), 2);
    assert_eq!(sig.params[0].def.name(), "i8");
    assert_eq!(sig.params[0].ty, reader::Type::I8);
    assert_eq!(sig.params[1].def.name(), "i16");
    assert_eq!(sig.params[1].ty, reader::Type::I16);

    assert_eq!(methods[1].name(), "Two");
    let sig = methods[1].signature("Namespace", &[]);
    assert_eq!(sig.call_flags, reader::MethodCallAttributes::HASTHIS);
    assert_eq!(sig.return_type, reader::Type::String);
    assert_eq!(sig.params.len(), 0);
}
