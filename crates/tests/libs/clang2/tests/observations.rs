use windows_clang2::{Input, capture};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

#[test]
fn buffer_optionality_conflicts_are_not_hidden_by_missing_annotations() {
    let source = include_str!("../input/annotation_context.h");
    for reversed in [false, true] {
        let mut inputs = [
            Input::new("a.hpp", source),
            Input::new("b.hpp", source.replace("_Out_writes_", "_Out_writes_opt_")),
            Input::new("c.hpp", "extern \"C\" void Fill(int size, int* buffer);"),
        ];
        if reversed {
            let [left, right, _] = &mut inputs;
            std::mem::swap(&mut left.source, &mut right.source);
        }
        let snapshot = capture(inputs, ARGS, &["Fill"]).unwrap();
        assert!(snapshot.resolve().is_err());
    }
}

#[test]
fn interface_uuids_must_agree() {
    let source = include_str!("../input/interface.h");
    for reversed in [false, true] {
        let mut inputs = [
            Input::new("a.hpp", source),
            Input::new(
                "b.hpp",
                source.replace(
                    "00000000-0000-0000-c000-000000000047",
                    "00000000-0000-0000-c000-000000000048",
                ),
            ),
        ];
        if reversed {
            let [left, right] = &mut inputs;
            std::mem::swap(&mut left.source, &mut right.source);
        }
        let snapshot = capture(inputs, ARGS, &["Use"]).unwrap();
        assert!(snapshot.resolve().is_err());
    }
}

#[test]
fn method_receiver_contracts_must_agree() {
    for (left, right) in [
        ("static void Method();", "void Method();"),
        ("void Method() const;", "void Method();"),
        ("void Method() &;", "void Method() &&;"),
        ("virtual void Method() = 0;", "virtual void Method();"),
    ] {
        let snapshot = capture(
            [
                Input::new("a.hpp", format!("struct Object {{ {left} }};")),
                Input::new("b.hpp", format!("struct Object {{ {right} }};")),
            ],
            ARGS,
            &["Object"],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err());
    }
}

#[test]
fn redundant_declarations_do_not_remove_dependencies() {
    for redundant in [false, true] {
        for conflict in [false, true] {
            let source = format!(
                "{}\n{}",
                if redundant { "#define REDECLARE" } else { "" },
                include_str!("../input/redeclaration.h")
            );
            let snapshot = capture(
                [
                    Input::new("a.hpp", source),
                    Input::new(
                        "b.hpp",
                        format!(
                            "typedef {} Required;",
                            if conflict { "float" } else { "int" }
                        ),
                    ),
                ],
                ARGS,
                &["Use"],
            )
            .unwrap();
            assert!(snapshot.dump().contains("] Required"));
            assert_eq!(snapshot.resolve().is_err(), conflict);
        }
    }
}

#[test]
fn observations_keep_annotation_parameter_contexts() {
    let source = include_str!("../input/annotation_context.h");
    let snapshot = capture(
        [
            Input::new("a.hpp", source),
            Input::new("b.hpp", format!("#define REVERSE\n{source}")),
        ],
        ARGS,
        &["Fill"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    assert_eq!(resolved.report().declarations, 2);
    assert_eq!(resolved.report().observations, 4);
    assert_eq!(resolved.group_count(), 1);
    let dump = snapshot.dump();
    assert!(dump.contains("name: \"count\""));
    assert!(dump.contains("name: \"renamed\""));
}

#[test]
fn annotation_bindings_must_agree_not_just_text() {
    let snapshot = capture([
        Input::new("a.hpp", "extern \"C\" void Fill(int count, int other, int* __attribute__((annotate(\"_Out_writes_(count)\"))) buffer);"),
        Input::new("b.hpp", "extern \"C\" void Fill(int other, int count, int* __attribute__((annotate(\"_Out_writes_(count)\"))) buffer);"),
    ], ARGS, &["Fill"]).unwrap();
    assert!(snapshot.resolve().is_err());
}

#[test]
fn missing_annotation_evidence_cannot_bridge_a_conflict() {
    let declarations = [
        "extern \"C\" void Use(int* first, int* second);",
        "extern \"C\" void Use(int* __attribute__((annotate(\"_In_\"))) first, int* second);",
        "extern \"C\" void Use(int* first, int* __attribute__((annotate(\"_Out_\"))) second);",
        "extern \"C\" void Use(int* __attribute__((annotate(\"_Out_\"))) first, int* second);",
    ];
    for reversed in [false, true] {
        let inputs = |count: usize| {
            let mut selected = declarations[..count].to_vec();
            if reversed {
                selected.reverse();
            }
            selected
                .into_iter()
                .enumerate()
                .map(|(i, source)| Input::new(format!("{i}.hpp"), source))
                .collect::<Vec<_>>()
        };
        assert!(
            capture(inputs(3), ARGS, &["Use"])
                .unwrap()
                .resolve()
                .is_ok()
        );
        assert!(
            capture(inputs(4), ARGS, &["Use"])
                .unwrap()
                .resolve()
                .is_err()
        );
    }
}

#[test]
fn alias_redeclarations_keep_dependencies_without_requiring_identical_spelling() {
    for conflict in [false, true] {
        let snapshot = capture(
            [
                Input::new(
                    "a.hpp",
                    "typedef unsigned long Base; typedef unsigned long Alias; typedef Base Alias;",
                ),
                Input::new(
                    "b.hpp",
                    if conflict {
                        "typedef float Base;"
                    } else {
                        "typedef unsigned long Base;"
                    },
                ),
            ],
            ARGS,
            &["Alias"],
        )
        .unwrap();
        assert_eq!(snapshot.resolve().is_err(), conflict);
    }
}

#[test]
fn forward_interface_uuid_conflicts_are_not_discarded() {
    let source = include_str!("../input/forward_uuid.h");
    for conflict in [false, true] {
        for definition_guid in [false, true] {
            for swapped in [false, true] {
                for reverse in [false, true] {
                    let guid = if conflict { 2 } else { 1 };
                    let mut inputs = vec![
                        Input::new(
                            if swapped { "z.hpp" } else { "a.hpp" },
                            format!(
                                "#define HAS_UUID\n{}",
                                source.replace(
                                    "GUID_VALUE",
                                    "\"00000001-0000-0000-c000-000000000046\""
                                )
                            ),
                        ),
                        Input::new(
                            if swapped { "a.hpp" } else { "z.hpp" },
                            format!(
                                "#define HAS_UUID\n{}{}",
                                if definition_guid {
                                    "#define DEFINITION\n"
                                } else {
                                    ""
                                },
                                source.replace(
                                    "GUID_VALUE",
                                    &format!("\"0000000{guid}-0000-0000-c000-000000000046\"")
                                )
                            ),
                        ),
                    ];
                    if !definition_guid {
                        inputs.push(Input::new(
                            "definition.hpp",
                            format!("#define DEFINITION\n{source}"),
                        ));
                    }
                    if reverse {
                        inputs.reverse();
                    }
                    let snapshot = capture(inputs, ARGS, &["IFoo"]).unwrap();
                    assert_eq!(snapshot.resolve().is_err(), conflict);
                }
            }
        }
    }
}
