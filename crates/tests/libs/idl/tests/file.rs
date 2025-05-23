use windows_idl as idl;

#[test]
fn test() {
    let file = std::fs::read_to_string("tests/file.idl").unwrap();
    let file = idl::parse(&file).unwrap();

    assert_eq!(file.1.items.len(), 4);
}
