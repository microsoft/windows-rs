use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn primitives() {
    let files = run_riddle("params");
    let reader = &Reader::new(&files);

    let def = reader
        .get(TypeName::new("Test", "IParams"))
        .next()
        .expect("Type missing");

    assert_eq!(reader.type_def_kind(def), TypeKind::Interface);

    assert!(reader.type_def_fields(def).next().is_none());
    let methods: Vec<MethodDef> = reader.type_def_methods(def).collect();
    assert_eq!(methods.len(), 2);

    assert_eq!(reader.method_def_name(methods[0]), "Nothing");
    assert_eq!(reader.method_def_name(methods[1]), "Bool");
}
