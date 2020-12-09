#[test]
fn stringable() {
    let reader = winmd::TypeReader::from_build();

    let def = reader.resolve_type_def(("Windows.Foundation", "IStringable"));
    assert!(def.name() == ("Windows.Foundation", "IStringable"));

    let methods: Vec<winmd::parsed::MethodDef> = def.methods().collect();
    assert!(methods.len() == 1);

    let method = methods[0];
    assert!(method.name() == "ToString");
}
