use windows_riddle as riddle;

#[test]
fn test() {
    let file = std::fs::read_to_string("tests/file.rdl").unwrap();
    let file = riddle::parse(&file).unwrap();

    assert_eq!(file.items.len(), 3);
}
