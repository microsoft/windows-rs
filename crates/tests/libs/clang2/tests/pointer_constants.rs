use windows_clang2::*;

#[test]
fn numeric_pointer_rdl() {
    helpers::ensure_libclang();
    let mut input = test_clang2::fixture("pointer_constants");
    input.macros = vec!["INVALID".into()];
    let output = projection::generate(&[input], "Fixture").unwrap();
    let rdl = &output.partitions["pointer_constants"].rdl;
    let expected = std::fs::read_to_string(test_clang2::input_dir().join("pointer_constants.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(*rdl, expected);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join("target\\test-clang2-pointer-constants.winmd");
    let roundtrip = path.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&path)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&path)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), *rdl);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

#[test]
fn pointer_rejections() {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture(
        "pointer_constants_invalid",
        &[
            "Relocated",
            "SideEffects",
            "Undefined",
            "Mutable",
            "CodeAddress",
            "ObjectAddress",
            "Safe",
        ],
    );
    assert!(projection::generate(std::slice::from_ref(&input), "Fixture").is_err());
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 1);
    for name in [
        "Relocated",
        "SideEffects",
        "Undefined",
        "Mutable",
        "CodeAddress",
        "ObjectAddress",
    ] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            partial.omissions
        );
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_pointer_sentinel() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.source = test_clang2::fixture("winerror_discovery").source;
    input.roots = vec!["MacroDiscoveryAnchor".into()];
    input.macros = vec!["INVALID_P_ROOT_SECURITY_ID".into()];
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "INVALID_P_ROOT_SECURITY_ID")
            .unwrap();
        assert!(matches!(
            &value.observations[0].kind,
            ValueKind::Constant {
                value: Some(ConstantValue::Pointer(-1)),
                ..
            }
        ));
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert!(
        output.partitions["urlmon"]
            .rdl
            .contains("const INVALID_P_ROOT_SECURITY_ID: *mut BYTE = -1;")
    );
}
