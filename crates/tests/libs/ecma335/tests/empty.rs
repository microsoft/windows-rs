use windows_ecma335::*;

#[test]
fn test() {
    let file = writer::File::new("test");
    let bytes = file.into_stream();
    std::fs::write("tests/empty.winmd", bytes).unwrap();

    let _reader = reader::File::read("tests/empty.winmd").unwrap();
}
