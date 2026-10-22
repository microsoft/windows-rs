use windows_clang::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = "hidusage_discovery.h".into();
    input.source = test_clang::fixture("hidusage_discovery").source;
    input.roots = vec!["MacroDiscoveryAnchor".into()];
    input.macro_headers = vec!["hidusage.h".into()];
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    let actual = &output.output.partitions["hidusage"].rdl;
    assert_eq!(output.output.constants, 2833);
    assert!(!actual.contains("#[arch("), "{actual}");
    let expected =
        std::fs::read_to_string(test_clang::input_dir().join("hidusage_discovery.rdl")).unwrap();
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
    assert_eq!(output.omissions.len(), 1, "{:?}", output.omissions);
    assert!(
        output.omissions[0]
            .contains("macro `__HIDUSAGE_H__`: empty macro replacement is not an expression")
    );
}
