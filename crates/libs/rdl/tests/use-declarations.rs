use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/use-declarations.rdl")
        .output("tests/use-declarations.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/use-declarations.winmd")
        .output("tests/use-declarations-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}

#[test]
pub fn parse_with_reference() {
    reader()
        .input("tests/use-declarations-ref.rdl")
        .reference("../bindgen/default/Windows.winmd")
        .output("tests/use-declarations-ref.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/use-declarations-ref.winmd")
        .output("tests/use-declarations-ref-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
