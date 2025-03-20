use windows_bindgen as reader;
use windows_ecma335::writer::*;

#[test]
fn test() {
    let file = File::new("test");
    let bytes = file.into_stream();
    std::fs::write("tests/empty.winmd", bytes).unwrap();

    let _reader = reader::Reader::new(vec![reader::File::new(
        std::fs::read("tests/empty.winmd").unwrap(),
    )
    .unwrap()]);
}
