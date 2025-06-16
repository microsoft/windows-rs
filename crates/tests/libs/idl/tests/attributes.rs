use windows_idl as idl;

#[test]
fn test() {
    let file = std::include_str!("attributes.idl");
    let file = idl::parse(file).unwrap();

    assert_eq!(file.items.len(), 7);
}
