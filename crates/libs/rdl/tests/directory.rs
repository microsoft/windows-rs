use windows_rdl::*;

#[test]
pub fn parse() {
    // Reader::reference with a directory of .winmd files
    reader()
        .input("tests/path.rdl")
        .reference("../bindgen/default")
        .output("tests/directory.winmd")
        .write()
        .unwrap();

    // Writer::input and Writer::reference with a directory of .winmd files
    writer()
        .input("tests/directory.winmd")
        .output("tests/directory-output.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
