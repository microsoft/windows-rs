use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/path.rdl")
        .reference("../bindgen/default/windows.winmd")
        .output("tests/path.winmd")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/path.winmd"#)
        .output("tests/path-output.rdl")
        .namespace("ModForB")
        .write()
        .unwrap();
}
