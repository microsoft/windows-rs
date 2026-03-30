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
        .filter("Test.C")
        .write()
        .unwrap();

    writer()
        .input("tests/mod-recursive.winmd")
        .output("tests/mod-recursive.rdl")
        .filter("Test")
        .write()
        .unwrap();

    writer()
        .input("tests/mod-recursive.winmd")
        .output("tests/mod-recursive-subset.rdl")
        .filter("Test.C")
        .write()
        .unwrap();

    writer()
        .input("tests/mod-recursive.winmd")
        .output("tests/mod-recursive-exclude.rdl")
        .filter("Test")
        .filter("!Test.C")
        .write()
        .unwrap();
}
