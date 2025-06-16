use windows_idl as idl;

#[test]
fn test() {
    let file = std::include_str!("struct.idl");
    let file = idl::parse(file).unwrap();

    assert_eq!(file.items.len(), 1);
}
