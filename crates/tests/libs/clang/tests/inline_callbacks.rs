use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

#[test]
fn inline_callbacks() {
    helpers::ensure_libclang();
    let input = selected_fixture("inline_callbacks", &[]);
    let output = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (2, 2));
    let expected_path = input_dir().join("inline_callbacks.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected_path, &output.partitions["inline_callbacks"].rdl).unwrap();
    }
    let expected = std::fs::read_to_string(expected_path)
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["inline_callbacks"].rdl, expected);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-inline-callbacks.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&expected)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join("target\\test-clang-inline-callbacks.rdl");
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
    let snapshots = capture(&[input], &[Target::Arm64, Target::X64, Target::X86]).unwrap();
    assert_eq!(
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap(),
        output
    );
}
