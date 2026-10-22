use windows_clang::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    for unicode in [false, true] {
        let mut input = test_clang::annotated_sdk_fixture();
        input.name = "winuser_discovery.h".into();
        input.source = test_clang::fixture("winuser_discovery").source;
        input.roots = vec!["MacroDiscoveryAnchor".into()];
        input.macro_headers = vec!["winuser.h".into(), "imm.h".into()];
        if unicode {
            input.arguments.push("-DUNICODE".into());
        }
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        for header in ["winuser", "imm"] {
            let actual = &output.output.partitions[header].rdl;
            let expected = std::fs::read_to_string(
                test_clang::input_dir().join(format!("{header}_discovery.rdl")),
            )
            .unwrap();
            let expected = if unicode {
                expected.replace(": LPSTR =", ": LPWSTR =")
            } else {
                expected
            };
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
        assert!(
            !output.output.partitions["winuser"]
                .rdl
                .contains("const MOD_ALT:")
        );
    }
}
