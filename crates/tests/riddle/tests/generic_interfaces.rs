use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("generic_interfaces", "winrt", &[]);
    let reader = &Reader::new(&files);

    let types: Vec<Item> = reader
        .namespace_items("Test", &Default::default())
        .collect();

    assert_eq!(types.len(), 4);

    let def = reader
        .get_type_def(TypeName::new("Test", "IIterable"))
        .next()
        .unwrap();

    assert_eq!(reader.type_def_interface_impls(def).count(), 0);
    let generics: Vec<GenericParam> = reader.type_def_generics(def).collect();
    assert_eq!(generics.len(), 1);
    assert_eq!(reader.generic_param_name(generics[0]), "T");

    let def = reader
        .get_type_def(TypeName::new("Test", "IMapView"))
        .next()
        .unwrap();

    let impls: Vec<InterfaceImpl> = reader.type_def_interface_impls(def).collect();
    assert_eq!(impls.len(), 1);
    let generics: Vec<GenericParam> = reader.type_def_generics(def).collect();
    assert_eq!(generics.len(), 2);
    assert_eq!(reader.generic_param_name(generics[0]), "K");
    assert_eq!(reader.generic_param_name(generics[1]), "V");
}
