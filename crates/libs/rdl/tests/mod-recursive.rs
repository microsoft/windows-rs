use windows_rdl::*;

#[test]
pub fn parse() {
    Reader::new()
        .input("tests/mod-recursive.rdl")
        .output("tests/mod-recursive.winmd")
        .write()
        .unwrap();

            Writer::new()
        .input(r#"tests/mod-recursive.winmd"#)
        .output("tests/mod-recursive-not.rdl")
        .namespace("Test.C")
        .write()
        .unwrap();

    Writer::new()
        .input(r#"tests/mod-recursive.winmd"#)
        .output("tests/mod-recursive.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();
}
