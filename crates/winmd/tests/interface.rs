#[test]
fn interface() {
    use winmd::*;

    let reader = &Reader::from_os();
    let def = reader.resolve(("Windows.Foundation", "IStringable"));
    let t = def.into_type(reader);
    let name = t.name();
    assert!(name.namespace == "Windows.Foundation");
    assert!(name.name == "IStringable");
    assert!(name.generics.is_empty());
    assert!(name.def == def);

    // TODO: add tests for complete structure of the interface
}
