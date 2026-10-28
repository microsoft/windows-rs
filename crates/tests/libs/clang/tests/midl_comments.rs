use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn com_output_owns_returned_pointer_storage() {
    helpers::ensure_libclang();
    let input = test_clang::fixture("sal_midl_direction");
    let output = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    let expected = std::fs::read_to_string(test_clang::input_dir().join("sal_midl_direction.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["sal_midl_direction"].rdl, expected);

    let mut other = test_clang::fixture("sal_midl_direction");
    other.name = "other.h".into();
    other.source = other.source.replace(
        "/* [annotation][in][out] */ COM_OUT",
        "/* [annotation][out] */ COM_OUT",
    );
    let error = projection::generate_partial(&[input, other], "Fixture").unwrap_err();
    assert!(error.to_string().contains("differ"), "{error}");
}

#[test]
fn parameter_comment_ownership() {
    helpers::ensure_libclang();
    let input = test_clang::fixture("midl_comments");
    for snapshot in capture(&[input], &TARGETS).unwrap() {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "BufferFunction")
            .unwrap();
        let ValueKind::Function { parameters, .. } = &value.observations[0].kind else {
            panic!()
        };
        assert!(parameters[0].comments.is_empty());
        assert_eq!(parameters[1].comments.len(), 1);
        assert_eq!(
            parameters[1].comments[0].text,
            "/* [annotation][iid_is][retval][out] */"
        );
        assert_eq!(parameters[1].comments[0].location.line, 6);
        assert!(parameters[2].comments.is_empty());
        assert_eq!(parameters[3].comments.len(), 1);
        assert_eq!(parameters[4].comments.len(), 1);
        let alias = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "Buffer")
            .unwrap();
        let parameters = alias.observations[0].parameters.as_ref().unwrap();
        assert_eq!(parameters[0].comments[0].text, "/* [iid_is][out] */");
        let alias = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "MacroBuffer")
            .unwrap();
        assert!(
            alias.observations[0]
                .parameters
                .as_ref()
                .unwrap()
                .iter()
                .all(|p| p.comments.is_empty())
        );
    }
}

#[test]
fn result_contract_agreement() {
    helpers::ensure_libclang();
    let mut other = test_clang::fixture("midl_comments");
    other.name = "other.h".into();
    other.source = other.source.replace(
        "[annotation][iid_is][retval][out]",
        "[ out ] [ retval ][ iid_is ] [annotation]",
    );
    let mut inputs = [test_clang::fixture("midl_comments"), other];
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        merge(&snapshots).unwrap();
        inputs.reverse();
    }
    for root in ["BufferFunction", "Buffer", "IBuffer"] {
        let mut other = test_clang::fixture("midl_comments");
        other.name = "other.h".into();
        other.roots = vec![root.into()];
        other.source = other
            .source
            .replace("iid_is", "unclassified")
            .replace("retval", "unclassified");
        let mut original = test_clang::fixture("midl_comments");
        original.roots = vec![root.into()];
        let mut inputs = [original, other];
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains("differ"), "{error}");
            inputs.reverse();
        }
    }
}

#[test]
fn direction_contract_agreement() {
    helpers::ensure_libclang();
    for (replacement, agrees) in [
        ("[ in ][out][unique]", true),
        ("[unique][out]", false),
        ("[unique]", false),
    ] {
        let mut original = test_clang::fixture("midl_comments");
        original.roots = vec!["Directions".into()];
        let mut other = test_clang::fixture("midl_comments");
        other.roots = original.roots.clone();
        other.name = "other.h".into();
        other.source = other.source.replace("[unique][out][in]", replacement);
        let mut inputs = [original, other];
        for _ in 0..2 {
            let result = projection::generate_partial(&inputs, "Fixture");
            if agrees {
                result.unwrap();
            } else {
                let error = result.unwrap_err();
                assert!(error.to_string().contains("differ"), "{error}");
            }
            inputs.reverse();
        }
    }
}

#[test]
fn invalid_result_storage() {
    helpers::ensure_libclang();
    for (marker, ty, expected) in [
        ("[retval][out]", "int", "requires a pointer"),
        (
            "[iid_is][out]",
            "int*",
            "requires pointer-to-pointer storage",
        ),
        ("[retval]", "void**", "requires output storage"),
    ] {
        let input = Input {
            name: "invalid_result.h".into(),
            source: format!("typedef void (*Callback)(/* {marker} */ {ty} result);"),
            ..Default::default()
        };
        let error = projection::generate(&[input], "Fixture").unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn input_iid_is_not_a_com_output() {
    helpers::ensure_libclang();
    for marker in ["[iid_is]", "[iid_is][in]", "[iid_is][in][out]"] {
        let input = Input {
            name: "input_iid.h".into(),
            source: format!("typedef void (*Callback)(/* {marker} */ void** value);"),
            ..Default::default()
        };
        let output = projection::generate(&[input], "Fixture").unwrap();
        for partition in output.partitions.values() {
            assert!(!partition.rdl.contains("#[iid_is]"));
            assert!(!partition.rdl.contains("#[retval]"));
        }
    }
}

#[test]
fn indirect_callback_keeps_its_contract_owner() {
    helpers::ensure_libclang();
    let input = Input {
        name: "indirect_midl.h".into(),
        source: "typedef void (**Callback)(/* [iid_is][out] */ void** result);".into(),
        ..Default::default()
    };
    let error = projection::generate(&[input], "Fixture").unwrap_err();
    assert!(
        error.to_string().contains("has no metadata owner"),
        "{error}"
    );
}
