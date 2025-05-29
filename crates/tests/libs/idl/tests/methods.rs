use windows_idl as idl;

#[test]
fn test() {
    let file = std::fs::read_to_string("tests/methods.idl").unwrap();
    let file = idl::parse(&file).unwrap();

    assert_eq!(file.items.len(), 1);

    let idl::Item::Interface(ty) = &file.items[0] else {
        panic!()
    };

    assert_eq!(ty.methods.len(), 4);
}
