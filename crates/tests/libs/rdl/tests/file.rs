use windows_rdl as rdl;

#[test]
fn test() {
    let file = std::fs::read_to_string("tests/file.rdl").unwrap();
    let file = rdl::parse(&file).unwrap();

    assert_eq!(file.items.len(), 3);
}
