use test_riddle::run_riddle;
use windows_metadata::*;

#[test]
fn test() {
    let files = run_riddle("generic_interfaces", "winrt", &[]);
    let reader = Reader::new(files);

    let types: Vec<Item> = reader
        .namespace_items("Test", &Default::default())
        .collect();

    assert_eq!(types.len(), 4);

    let def = reader
        .get_type_def("Test", "IIterable")
        .next()
        .unwrap();

    assert_eq!(def.interface_impls().count(), 0);
    let generics: Vec<GenericParam> = def.generics().collect();
    assert_eq!(generics.len(), 1);
    assert_eq!(generics[0].name(), "T");

    let def = reader
        .get_type_def("Test", "IMapView")
        .next()
        .unwrap();

    let impls: Vec<InterfaceImpl> = def.interface_impls().collect();
    assert_eq!(impls.len(), 1);
    let generics: Vec<GenericParam> = def.generics().collect();
    assert_eq!(generics.len(), 2);
    assert_eq!(generics[0].name(), "K");
    assert_eq!(generics[1].name(), "V");
}
