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
        .input("../../../libs/bindgen/default/Windows.winmd")
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

#[test]
pub fn exclusive_to_with_use() {
    reader()
        .input("tests/exclusive-to-use.rdl")
        .input("../../../libs/bindgen/default/Windows.winmd")
        .output("tests/exclusive-to-use.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/exclusive-to-use.winmd")
        .output("tests/exclusive-to-use-out.rdl")
        .filter("Test")
        .write()
        .unwrap();

    let expected = std::fs::read_to_string("tests/exclusive-to-use-out.rdl").unwrap();
    assert!(
        expected.contains("ExclusiveTo"),
        "expected ExclusiveTo attribute in output"
    );
}
