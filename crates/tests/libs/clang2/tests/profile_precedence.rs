use windows_clang2::{Input, ProjectionOptions, capture};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

#[test]
fn precedence_preserves_winning_contracts_and_rejects_incompatible_dependencies() {
    for reversed in [false, true] {
        let mut inputs = [
            Input::new("primary.hpp", include_str!("../input/profile_primary.h")),
            Input::new(
                "secondary.hpp",
                include_str!("../input/profile_secondary.h"),
            ),
        ];
        if reversed {
            inputs.reverse();
        }
        let snapshot = capture(
            inputs,
            ARGS,
            &[
                "Shared",
                "ProfileValue",
                "UsePrimary",
                "UseSecondary",
                "UseStable",
            ],
        )
        .unwrap();
        assert!(snapshot.assess().is_err());
        let mut options = ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        for (priority, rejected, expected) in [
            (
                ["primary.hpp", "secondary.hpp"],
                "UseSecondary",
                include_str!("../expected/profile_primary.rdl"),
            ),
            (
                ["secondary.hpp", "primary.hpp"],
                "UsePrimary",
                include_str!("../expected/profile_secondary.rdl"),
            ),
        ] {
            let assessment = snapshot.assess_profiles(&priority).unwrap();
            assert_eq!(
                assessment
                    .rejected
                    .keys()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                [rejected]
            );
            let resolved = assessment.resolved.unwrap();
            let rdl = resolved.project(&options).unwrap().rdl();
            assert_eq!(rdl, expected.replace("\r\n", "\n"));
            let output =
                std::path::Path::new(env!("OUT_DIR")).join(format!("{}.winmd", priority[0]));
            windows_rdl::reader()
                .input_text(&rdl)
                .output(&output)
                .write()
                .unwrap();
            assert!(
                assessment.selections.iter().any(
                    |selection| selection.name == "Shared" && selection.selected == priority[0]
                )
            );
            assert!(
                snapshot
                    .assess_profiles(&["primary.hpp", "secondary.hpp", "unknown.hpp"])
                    .is_err()
            );
            assert!(snapshot.assess_profiles(&[]).is_err());
        }
        assert!(snapshot.assess().is_err());
        assert!(snapshot.assess_profiles(&["primary.hpp"]).is_err());
        assert!(
            snapshot
                .assess_profiles(&["primary.hpp", "primary.hpp"])
                .is_err()
        );
    }
}

#[test]
fn conflicting_annotations_within_the_authoritative_profile_remain_errors() {
    let snapshot = capture([
        Input::new("primary.hpp", "extern \"C\" void Use(int* __attribute__((annotate(\"_In_\"))) value);\nextern \"C\" void Use(int* __attribute__((annotate(\"_Out_\"))) value);"),
        Input::new("secondary.hpp", "extern \"C\" void Use(int* value);"),
    ], ARGS, &["Use"]).unwrap();
    assert!(
        snapshot
            .assess_profiles(&["primary.hpp", "secondary.hpp"])
            .is_err()
    );
}

#[test]
fn nested_callback_contracts_cannot_be_replaced_by_a_different_profile() {
    let source = include_str!("../input/profile_contracts.h");
    let primary = format!("#define PROFILE_PRIMARY\n{source}");
    let snapshot = capture(
        [
            Input::new("primary.hpp", primary),
            Input::new("secondary.hpp", source),
        ],
        ARGS,
        &["Callback", "UseContract"],
    )
    .unwrap();
    let assessment = snapshot
        .assess_profiles(&["primary.hpp", "secondary.hpp"])
        .unwrap();
    assert!(assessment.rejected.is_empty());
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let rdl = assessment
        .resolved
        .unwrap()
        .project(&options)
        .unwrap()
        .rdl();
    assert!(rdl.contains("_In_"), "{rdl}");
    assert!(!rdl.contains("_Out_"), "{rdl}");

    let snapshot = capture(
        [
            Input::new("primary.hpp", format!("#define PROFILE_PRIMARY\n{source}")),
            Input::new(
                "secondary.hpp",
                format!("{source}\nextern \"C\" void Secondary(Contract* value);"),
            ),
        ],
        ARGS,
        &["Callback", "Secondary"],
    )
    .unwrap();
    let assessment = snapshot
        .assess_profiles(&["primary.hpp", "secondary.hpp"])
        .unwrap();
    assert!(assessment.rejected["Secondary"].contains("annotation contracts differ"));
    assert!(
        assessment
            .resolved
            .unwrap()
            .project_roots(&options, &["Callback"])
            .is_ok()
    );
}

#[test]
fn unavailable_authoritative_evidence_never_falls_back() {
    let source = include_str!("../input/profile_unavailable.h");
    let snapshot = capture(
        [
            Input::new("primary.hpp", format!("#define PROFILE_PRIMARY\n{source}")),
            Input::new("secondary.hpp", source),
        ],
        ARGS,
        &["Shared", "Use"],
    )
    .unwrap();
    let assessment = snapshot
        .assess_profiles(&["primary.hpp", "secondary.hpp"])
        .unwrap();
    assert_eq!(
        assessment
            .rejected
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["Shared", "Use"]
    );
    assert!(assessment.resolved.is_none());
    let assessment = snapshot
        .assess_profiles(&["secondary.hpp", "primary.hpp"])
        .unwrap();
    assert!(assessment.rejected.is_empty());
    assert!(assessment.resolved.is_some());
}

#[test]
fn an_empty_profile_still_requires_an_exact_rank() {
    let snapshot = capture(
        [
            Input::new("empty.hpp", ""),
            Input::new("api.hpp", "struct Shared { int value; };"),
        ],
        ARGS,
        &["Shared"],
    )
    .unwrap();
    assert!(snapshot.assess_profiles(&["api.hpp"]).is_err());
    assert!(snapshot.assess_profiles(&["empty.hpp", "api.hpp"]).is_ok());
}

#[test]
fn roots_found_only_in_a_shadowed_profile_are_preserved() {
    let snapshot = capture(
        [
            Input::new(
                "primary.hpp",
                "struct Shared { int value; }; extern \"C\" void Primary(Shared*);",
            ),
            Input::new("secondary.hpp", "struct Shared { int value; };"),
        ],
        ARGS,
        &["Shared", "Primary"],
    )
    .unwrap();
    let assessment = snapshot
        .assess_profiles(&["secondary.hpp", "primary.hpp"])
        .unwrap();
    assert!(assessment.rejected.is_empty());
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    assert!(assessment.resolved.unwrap().project(&options).is_ok());
}

#[test]
fn available_dependencies_survive_when_their_profiles_roots_are_unavailable() {
    let snapshot = capture(
        [Input::new("primary.hpp", "struct Shared { int value; }; struct Bad { decltype(nullptr) bad; Shared value; };"),
         Input::new("secondary.hpp", "struct Shared { int value; }; extern \"C\" void Use(Shared* value);")],
        ARGS, &["Bad", "Use"],
    ).unwrap();
    let assessment = snapshot
        .assess_profiles(&["primary.hpp", "secondary.hpp"])
        .unwrap();
    assert_eq!(
        assessment
            .rejected
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["Bad"]
    );
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    assert!(
        assessment
            .resolved
            .unwrap()
            .project_roots(&options, &["Use"])
            .is_ok()
    );
}

#[test]
fn missing_canonical_annotations_do_not_erase_a_dependents_contract() {
    let source = include_str!("../input/profile_contracts.h");
    let snapshot = capture(
        [
            Input::new(
                "primary.hpp",
                format!("#define PROFILE_UNANNOTATED\n{source}"),
            ),
            Input::new(
                "secondary.hpp",
                format!("{source}\nextern \"C\" void Secondary(Contract* value);"),
            ),
        ],
        ARGS,
        &["Callback", "Secondary"],
    )
    .unwrap();
    let assessment = snapshot
        .assess_profiles(&["primary.hpp", "secondary.hpp"])
        .unwrap();
    assert!(assessment.rejected["Secondary"].contains("annotation contracts differ"));
}
