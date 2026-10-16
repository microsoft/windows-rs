use windows_clang2::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn guid_definition_input() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "winnt_guid_definitions.h".into();
    input.source = test_clang2::fixture("winnt_guid_definitions").source;
    input.roots = vec!["GUID_MAX_POWER_SAVINGS".into(), "KCRM_PROTOCOL_BLOB".into()];
    let output = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    let rdl = &output.partitions["winnt"].rdl;
    assert!(
        rdl.contains("const GUID_MAX_POWER_SAVINGS: GUID = 0xa184130835414fabbc81f71556f20b4a;"),
        "{rdl}"
    );
    let mut declaration = test_clang2::annotated_sdk_fixture();
    declaration.name = "winnt_discovery.h".into();
    declaration.source = test_clang2::fixture("winnt_discovery").source;
    declaration.roots = vec!["GUID_MAX_POWER_SAVINGS".into(), "KCRM_PROTOCOL_BLOB".into()];
    let mut inputs = [declaration, input];
    for _ in 0..2 {
        let combined = projection::generate(&inputs, "Fixture").unwrap();
        assert_eq!(combined.partitions["winnt"].rdl, *rdl);
        let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
        assert_eq!(partial.output, combined);
        assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
        inputs.reverse();
    }
    inputs[1].source = inputs[1].source.replace("#include <intsafe.h>", "");
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("_KCRM_PROTOCOL_BLOB"), "{error}");
        assert!(error.contains("attributes differ"), "{error}");
        inputs.reverse();
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "winnt_discovery.h".into();
    input.source = test_clang2::fixture("winnt_discovery").source;
    input.roots = vec!["MacroDiscoveryAnchor".into()];
    input.macro_headers = vec!["winnt.h".into(), "winerror.h".into()];
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    let actual = &output.output.partitions["winnt"].rdl;
    let expected =
        std::fs::read_to_string(test_clang2::input_dir().join("winnt_discovery.rdl")).unwrap();
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
    assert!(
        output.output.partitions["winerror"]
            .rdl
            .contains("const S_OK: HRESULT = 0;"),
        "support headers must not displace the public S_OK definition"
    );
}
