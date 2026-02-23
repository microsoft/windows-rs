use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/generic.rdl")
        .output("tests/generic.winmd")
        .write()
        .unwrap();

    // Writer::new()
    //     .input("tests/generic.winmd")
    //     .output("tests/generic.rdl")
    //     .namespace("Test")
    //     .write()
    //     .unwrap();
}
