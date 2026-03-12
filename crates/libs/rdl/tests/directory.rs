use windows_rdl::*;

#[test]
pub fn parse() {
    // Reader::reference with a directory of .winmd files
    Reader::new()
        .input("tests/path.rdl")
        .reference("../bindgen/default")
        .output("tests/directory.winmd")
        .write()
        .unwrap();

    // Writer::input and Writer::reference with a directory of .winmd files
    Writer::new()
        .input("tests/directory.winmd")
        .output("tests/directory-output.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
