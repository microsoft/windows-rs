use test_riddle::run_riddle;
use windows_metadata::reader::*;

#[test]
fn riddle_basic() {
    let output = run_riddle("tests/basic.idl");
    let files = File::with_default(&[&output]).expect("Failed to open winmd files");
    let reader = &Reader::new(&files);

    let root = reader.tree("Root", &[]).expect("Root namespace not found");
    assert_eq!(root.namespace, "Root");
    assert_eq!(root.nested.len(), 1);
    let nested = &root.nested["Nested"];
    assert_eq!(nested.namespace, "Root.Nested");
    assert_eq!(nested.nested.len(), 0);

    let types: Vec<TypeDef> = reader.namespace_types("Root").collect();
    assert_eq!(types.len(), 1);

    assert_eq!(reader.type_def_namespace(types[0]), "Root");
    assert_eq!(reader.type_def_name(types[0]), "IRoot");
    assert_eq!(reader.type_def_kind(types[0]), TypeKind::Interface);

    let types: Vec<TypeDef> = reader.namespace_types("Root.Nested").collect();
    assert_eq!(types.len(), 1);

    assert_eq!(reader.type_def_namespace(types[0]), "Root.Nested");
    assert_eq!(reader.type_def_name(types[0]), "INested");
    assert_eq!(reader.type_def_kind(types[0]), TypeKind::Interface);
}
