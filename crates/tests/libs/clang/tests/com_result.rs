use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn com_result() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("com_result")], &TARGETS).unwrap();
    let imports = [(
        "CreateObject".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let expected_path = test_clang::input_dir().join("com_result.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected_path, &output.partitions["com_result"].rdl).unwrap();
    }
    let expected = std::fs::read_to_string(expected_path)
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["com_result"].rdl, expected);
}
