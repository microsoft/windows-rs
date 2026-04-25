#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/merge_a.rdl")
        .output("tests/merge_a.winmd")
        .write()
        .unwrap();

    windows_rdl::reader()
        .input("tests/merge_b.rdl")
        .output("tests/merge_b.winmd")
        .write()
        .unwrap();

    windows_metadata::merge()
        .input("tests/merge_a.winmd")
        .input("tests/merge_b.winmd")
        .output("tests/merged.winmd")
        .merge()
        .unwrap();

    windows_rdl::writer()
        .input("tests/merged.winmd")
        .output("tests/merge_out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
