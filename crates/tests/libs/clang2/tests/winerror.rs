use windows_clang2::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "winerror_discovery.h".into();
    input.source = test_clang2::fixture("winerror_discovery").source;
    input.roots = vec!["MacroDiscoveryAnchor".into()];
    input.macro_headers = vec!["winerror.h".into(), "urlmon.h".into()];
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    // Sample distinct native value/type families without duplicating the whole SDK header.
    for header in ["winerror", "urlmon"] {
        let actual = &output.output.partitions[header].rdl;
        let expected = std::fs::read_to_string(
            test_clang2::input_dir().join(format!("{header}_discovery.rdl")),
        )
        .unwrap();
        for declaration in expected
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("const "))
        {
            assert!(
                actual.lines().any(|line| line.trim() == declaration),
                "{declaration}"
            );
        }
    }
}
