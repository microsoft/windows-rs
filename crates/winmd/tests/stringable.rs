extern crate winrt_winmd as winmd;

#[test]
fn stringable() {
    let reader = winmd::TypeReader::from_build();

    let def = reader.resolve_type_def(("Windows.Foundation", "IStringable"));
    assert!(def.name(reader) == ("Windows.Foundation", "IStringable"));

    let methods: Vec<winmd::parsed::MethodDef> = def.methods(reader).collect();
    assert!(methods.len() == 1);
    assert!(methods[0].name(reader) == "ToString");
}
