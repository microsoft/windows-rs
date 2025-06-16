use windows_idl as idl;

#[test]
fn test() {
    let file = std::include_str!("interface.idl");
    let file = idl::parse(file).unwrap();

    assert_eq!(file.items.len(), 1);

    let idl::Item::Interface(ty) = &file.items[0] else {
        panic!()
    };

    assert_eq!(ty.methods.len(), 3);

    assert_eq!(ty.name, "ICoreWebView2Environment2");
    assert_eq!(ty.implements[0].name, "ICoreWebView2Environment");
}
