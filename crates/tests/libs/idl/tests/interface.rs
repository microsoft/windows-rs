use windows_idl as idl;

#[test]
fn test() {
    let file = std::fs::read_to_string("tests/interface.idl").unwrap();
    let file = idl::parse(&file).unwrap();

    assert_eq!(file.0.trim(), "");
    assert_eq!(file.1.items.len(), 1);

    let idl::Item::Interface(ty) = &file.1.items[0] else {
        panic!()
    };

    assert_eq!(ty.methods.len(), 3);

    assert_eq!(ty.name, "ICoreWebView2Environment2");
    assert_eq!(ty.implements[0].name, "ICoreWebView2Environment");
}
