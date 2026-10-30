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
    let expected = std::fs::read_to_string(test_clang::input_dir().join("com_result.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["com_result"].rdl, expected);
}
