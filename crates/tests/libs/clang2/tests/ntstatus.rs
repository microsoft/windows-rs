use windows_clang2::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "ntstatus_discovery.h".into();
    input.source = test_clang2::fixture("ntstatus_discovery").source;
    input.roots = vec!["MacroDiscoveryAnchor".into()];
    input.macro_headers = vec!["ntstatus.h".into()];
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(output.omissions.len(), 1, "{:?}", output.omissions);
    assert!(
        output.omissions[0]
            .contains("macro `_NTSTATUS_`: empty macro replacement is not an expression")
    );
    let actual = &output.output.partitions["ntstatus"].rdl;
    let expected =
        std::fs::read_to_string(test_clang2::input_dir().join("ntstatus_discovery.rdl")).unwrap();
    for declaration in expected
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("const "))
    {
        assert!(
            actual.lines().any(|line| line.trim() == declaration),
            "{declaration}: {:?}",
            output.omissions.iter().take(5).collect::<Vec<_>>()
        );
    }
}
