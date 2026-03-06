use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/use-declarations.rdl")
        .output("tests/use-declarations.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/use-declarations.winmd")
        .output("tests/use-declarations-out.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}

#[test]
pub fn parse_with_reference() {
    Reader::new()
        .input("tests/use-declarations-ref.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/use-declarations-ref.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input("tests/use-declarations-ref.winmd")
        .output("tests/use-declarations-ref-out.rdl")
        .namespace("Test")
        .write()
        .unwrap();
}
