use windows_rdl as rdl;

#[test]
fn test() {
    let file = std::include_str!("file.rdl");
    let file = rdl::parse(&file).unwrap();

    assert_eq!(file.items.len(), 3);
}
