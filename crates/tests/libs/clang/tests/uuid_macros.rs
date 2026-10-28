use windows_clang::*;

#[test]
fn probe_declarations_are_not_roots() {
    helpers::ensure_libclang();
    for authored in [false, true] {
        let mut input = test_clang::fixture("probe_declarations");
        input.macros.push("PROBE_IDENTITY".into());
        if authored {
            input.arguments.push("-DAUTHORED_EXPANSION".into());
        }
        let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
        for snapshot in &snapshots {
            assert_eq!(
                snapshot
                    .records
                    .iter()
                    .any(|record| record.name == "ProbeIdentity"),
                authored
            );
        }
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        assert_eq!(output.constants, if authored { 3 } else { 1 });
        let actual = &output.partitions["probe_declarations"].rdl;
        assert!(
            actual.contains("const PROBE_IDENTITY: GUID = 0x12345678_1234_5678_90ab_cdef01234567;")
        );
        assert_eq!(actual.contains("const ProbeIdentity:"), authored);
    }
}

#[test]
fn explicit_and_discovered_uuid() {
    helpers::ensure_libclang();
    let mut input = test_clang::selected_fixture("uuid_macros", &["GUID"]);
    input.macros = vec!["IDENTITY_VALUE".into()];
    let explicit = projection::generate_partial(std::slice::from_ref(&input), "Fixture").unwrap();
    assert!(explicit.omissions.is_empty(), "{:?}", explicit.omissions);
    assert_eq!(explicit.output.constants, 1);
    input.macros.clear();
    input.macro_headers = vec!["uuid_macros.h".into()];
    let discovered = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(
        discovered.omissions.is_empty(),
        "{:?}",
        discovered.omissions
    );
    assert_eq!(discovered.output, explicit.output);
}
