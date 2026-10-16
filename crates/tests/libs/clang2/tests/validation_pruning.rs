use test_clang2::selected_fixture;
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn conflicts_before_pruning() {
    helpers::ensure_libclang();
    for change in [
        "#define FIELD_TYPE float\n",
        "#define RESULT_TYPE float\n",
        "#if defined(_M_IX86)\n#define FIELD_TYPE float\n#endif\n",
    ] {
        for dependency in ["", "#define COMPLETE_DEPENDENCY\n"] {
            let mut inputs = [
                selected_fixture("validation_pruning", &[]),
                selected_fixture("validation_pruning", &[]),
            ];
            inputs[0].source.insert_str(0, dependency);
            inputs[1].name = "other.h".into();
            inputs[1]
                .source
                .insert_str(0, &format!("{dependency}{change}"));
            for _ in 0..2 {
                let result = capture_partial(&inputs, &TARGETS);
                assert!(
                    result.is_err(),
                    "capture exclusions hid a native conflict: {dependency}{change}"
                );
                let error = result.unwrap_err().to_string();
                assert!(error.contains("native field types differ"), "{error}");
                assert!(error.contains("Shared"), "{error}");
                inputs.reverse();
            }
        }
    }
}

#[test]
fn compatible_observations_still_prune() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("validation_pruning", &[]),
        selected_fixture("validation_pruning", &[]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1]
        .source
        .insert_str(0, "#if defined(_M_IX86)\n#define COMPLETE_ENUM\n#endif\n");
    let result = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert_eq!(result.output.records, 1);
    assert_eq!(result.output.functions, 0);
    for name in [
        "Broken",
        "BrokenEnum",
        "BrokenAlias",
        "EnumDependent",
        "AliasDependent",
        "Shared",
        "InspectShared",
    ] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
    inputs.reverse();
    assert_eq!(
        projection::project_partial(
            capture_partial(&inputs, &[Target::Arm64, Target::X64, Target::X86]).unwrap(),
            "Fixture",
        )
        .unwrap(),
        result
    );
}
