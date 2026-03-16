use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/mod-recursive.rdl")
        .output("tests/mod-recursive.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/mod-recursive.winmd")
        .output("tests/mod-recursive-not.rdl")
        .namespace("Test.C")
        .write()
        .unwrap();

    writer()
        .input("tests/mod-recursive.winmd")
        .output("tests/mod-recursive.rdl")
        .namespace("Test")
        .recursive()
        .write()
        .unwrap();

    writer()
        .input("tests/mod-recursive.winmd")
        .output("tests/mod-recursive-subset.rdl")
        .namespace("Test.C")
        .recursive()
        .write()
        .unwrap();
}
