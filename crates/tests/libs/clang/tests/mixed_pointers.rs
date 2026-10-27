use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

#[test]
fn mixed_pointers() {
    helpers::ensure_libclang();
    let output =
        projection::generate(&[selected_fixture("mixed_pointers", &[])], "Fixture").unwrap();
    let rdl = &output.partitions["mixed_pointers"].rdl;
    let expected = input_dir().join("mixed_pointers.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, rdl).unwrap();
    }
    let expected = std::fs::read_to_string(expected)
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(*rdl, expected);

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-mixed-pointers.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join("target\\test-clang-mixed-pointers.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), expected);

    for path in [winmd, rewritten] {
        std::fs::remove_file(path).unwrap();
    }
}
