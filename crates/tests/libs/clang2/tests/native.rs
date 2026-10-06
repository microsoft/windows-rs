use windows_clang2::{Input, Snapshot, capture};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc19.0.0"];
const NATIVE: &str = include_str!("../input/native.h");
const DEPENDENCIES: &str = include_str!("../input/dependencies.h");
const FORWARD: &str = include_str!("../input/forward.h");
const DEFINITION: &str = include_str!("../input/definition.h");
const CYCLE: &str = include_str!("../input/cycle.h");

fn pair(source: &str, left: &str, right: &str, roots: &[&str]) -> Snapshot {
    capture(
        [
            Input::new("a.hpp", format!("#define VALUE {left}\n{source}")),
            Input::new("b.hpp", format!("#define VALUE {right}\n{source}")),
        ],
        ARGS,
        roots,
    )
    .unwrap()
}

#[test]
fn native_evidence_golden() {
    let snapshot = capture(
        [Input::new("native.h", NATIVE)],
        ARGS,
        &["Use", "incomplete_array", "zero_array"],
    )
    .unwrap();
    assert_eq!(snapshot.validate().unwrap().incomplete, ["Forward"]);
    let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/native.txt");
    let actual = snapshot.dump();
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, &actual).unwrap();
    }
    assert_eq!(actual, std::fs::read_to_string(expected).unwrap());
}

#[test]
fn legacy_and_native_capture_run_side_by_side() {
    let legacy =
        windows_clang::extract([windows_clang::Input::new("native.h", NATIVE)], ARGS).unwrap();
    let record = legacy
        .facts()
        .iter()
        .find(|fact| fact.name == "Native")
        .unwrap();
    let windows_clang::FactData::Record { fields, .. } = &record.data else {
        panic!("legacy capture did not produce the native record");
    };
    assert_eq!(fields[0].ty, fields[1].ty);
    assert_eq!(fields[3].ty, fields[4].ty);
    let native = capture([Input::new("native.h", NATIVE)], ARGS, &["Native"]).unwrap();
    assert!(native.validate().unwrap().incomplete.is_empty());
    for (left, right) in [("int", "long"), ("int&", "int&&")] {
        let snapshot = pair("typedef VALUE Different;", left, right, &["Different"]);
        assert!(snapshot.validate().is_err());
    }
}

#[test]
fn every_root_kind_validates_native_dependencies() {
    for root in ["Packet", "Callback", "Use", "ITEM"] {
        pair(DEPENDENCIES, "int", "int", &[root])
            .validate()
            .unwrap();
        let error = pair(DEPENDENCIES, "int", "float", &[root])
            .validate()
            .unwrap_err();
        assert!(error.to_string().contains("`Data`"), "{root}: {error}");
    }
}

#[test]
fn discovers_nonroot_completions_and_checks_every_definition() {
    for reverse in [false, true] {
        for swapped in [false, true] {
            for conflicting in [false, true] {
                let mut inputs = vec![
                    Input::new(if swapped { "z.hpp" } else { "a.hpp" }, FORWARD),
                    Input::new("b.hpp", format!("#define VALUE int\n{DEFINITION}")),
                    Input::new(
                        if swapped { "a.hpp" } else { "z.hpp" },
                        format!(
                            "#define VALUE {}\n{DEFINITION}",
                            if conflicting { "float" } else { "int" }
                        ),
                    ),
                ];
                if reverse {
                    inputs.reverse();
                }
                let snapshot = capture(inputs, ARGS, &["Packet"]).unwrap();
                if conflicting {
                    assert!(
                        snapshot
                            .validate()
                            .unwrap_err()
                            .to_string()
                            .contains("`Data`")
                    );
                } else {
                    assert!(snapshot.validate().unwrap().incomplete.is_empty());
                }
            }
        }
    }
}

#[test]
fn cycles_agree_independently_of_entry_and_order() {
    for root in ["First", "Second", "Raw", "Alias", "Use"] {
        pair(CYCLE, "int", "int", &[root]).validate().unwrap();
        let bad = pair(CYCLE, "int", "float", &[root]);
        for _ in 0..2 {
            assert!(bad.validate().unwrap_err().to_string().contains("`Second`"));
        }
        pair(CYCLE, "float", "int", &[root]).validate().unwrap_err();
    }
}

#[test]
fn shared_graph_work_is_bounded() {
    for depth in [32, 64, 128] {
        for cyclic in [false, true] {
            let mut source = format!("struct N{depth};\nstruct N{};\n", depth + 1);
            for index in (0..depth).rev() {
                source.push_str(&format!(
                    "struct N{index} {{ N{}* left; N{}* right; }};\n",
                    index + 1,
                    index + 2
                ));
            }
            if cyclic {
                source.push_str(&format!(
                    "struct N{depth} {{ N0* back; }};\nstruct N{} {{ N0* back; }};\n",
                    depth + 1
                ));
            }
            let report = pair(&source, "int", "int", &["N0"]).validate().unwrap();
            assert_eq!(report.declaration_pairs, depth + if cyclic { 6 } else { 2 });
            assert!(report.type_pairs <= 4 * (depth + 2), "{report:?}");
            assert_eq!(report.incomplete.len(), if cyclic { 0 } else { 2 });
        }
    }
}

#[test]
fn distinguishes_native_qualifiers_arrays_and_annotations() {
    for (source, left, right, root) in [
        ("typedef VALUE int* Pointer;", "volatile", "", "Pointer"),
        (
            "typedef VALUE struct Data* Pointer; struct Data;",
            "const",
            "",
            "Pointer",
        ),
        ("typedef int Array[VALUE];", "", "0", "Array"),
        (
            "extern \"C\" void Use(int* __attribute__((annotate(VALUE))) output);",
            "\"_In_\"",
            "\"_Out_\"",
            "Use",
        ),
        ("enum E : VALUE;", "int", "unsigned", "E"),
    ] {
        pair(source, left, right, &[root]).validate().unwrap_err();
    }
}

#[test]
fn forward_enum_preserves_underlying_type_evidence() {
    let snapshot = capture(
        [
            Input::new("a.hpp", "enum E : unsigned;"),
            Input::new("b.hpp", "enum E : int { Value };"),
        ],
        ARGS,
        &["E"],
    )
    .unwrap();
    snapshot.validate().unwrap_err();
}

#[test]
fn native_enum_and_callable_distinctions_are_retained() {
    for (source, left, right, root) in [
        ("enum VALUE E : int { A };", "", "class", "E"),
        ("extern \"C\" void Use() VALUE;", "", "noexcept", "Use"),
        (
            "typedef void (*Callback)(int* __attribute__((annotate(VALUE))) output);",
            "\"_In_\"",
            "\"_Out_\"",
            "Callback",
        ),
    ] {
        pair(source, left, right, &[root]).validate().unwrap_err();
    }
}

#[test]
fn qualified_aliases_and_internal_entities_do_not_collide() {
    let source = "namespace A { typedef VALUE Item; } namespace B { typedef int Item; }";
    pair(source, "int", "float", &["B::Item"])
        .validate()
        .unwrap();
    pair(source, "int", "float", &["A::Item"])
        .validate()
        .unwrap_err();
    pair(
        "namespace { typedef VALUE Item; }",
        "int",
        "float",
        &["(anonymous namespace)::Item"],
    )
    .validate()
    .unwrap();
}

#[test]
fn same_tu_redeclarations_have_one_native_identity() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            "struct Data; struct Data { int value; }; struct Data;",
        )],
        ARGS,
        &["Data"],
    )
    .unwrap();
    let report = snapshot.validate().unwrap();
    assert_eq!(report.declarations, 1);
    assert!(report.incomplete.is_empty());
}

#[test]
fn owned_evidence_can_move_to_a_thread_without_libclang() {
    let snapshot = pair(CYCLE, "int", "int", &["Use"]);
    let report = std::thread::spawn(move || {
        assert!(!clang_sys::is_loaded());
        assert!(!snapshot.dump().is_empty());
        snapshot.validate()
    })
    .join()
    .unwrap()
    .unwrap();
    assert!(report.incomplete.is_empty());
}

#[test]
fn wide_constant_values_are_not_truncated_into_equivalence() {
    for (source, root) in [
        (
            "enum E : unsigned __int128 { A = (unsigned __int128)1 << 100 };",
            "E",
        ),
        (
            "extern const unsigned __int128 V = (unsigned __int128)1 << 100;",
            "V",
        ),
    ] {
        let snapshot = capture(
            [Input::new("wide.hpp", source)],
            &["-x", "c++", "--target=x86_64-unknown-linux-gnu"],
            &[root],
        )
        .unwrap();
        assert!(
            snapshot
                .validate()
                .unwrap_err()
                .to_string()
                .contains("64 bits")
        );
    }
}

#[test]
fn initializer_observations_do_not_disappear_behind_a_declaration() {
    for value in [42, 43] {
        let snapshot = capture(
            [
                Input::new("a.hpp", "extern const int Value;"),
                Input::new("b.hpp", "extern const int Value = 42;"),
                Input::new("c.hpp", format!("extern const int Value = {value};")),
            ],
            ARGS,
            &["Value"],
        )
        .unwrap();
        assert_eq!(snapshot.validate().is_ok(), value == 42);
    }
}

#[test]
fn function_parameter_names_are_not_native_type_identity() {
    pair(
        "extern \"C\" void Use(int VALUE);",
        "first",
        "second",
        &["Use"],
    )
    .validate()
    .unwrap();
}

#[test]
fn retains_target_and_native_calling_convention() {
    for target in ["x86_64", "i686", "aarch64"] {
        let argument = format!("--target={target}-pc-windows-msvc");
        let snapshot = capture(
            [Input::new("native.h", NATIVE)],
            &["-x", "c++", &argument],
            &["Use"],
        )
        .unwrap();
        snapshot.validate().unwrap();
        assert!(
            snapshot.target().starts_with(target),
            "{}",
            snapshot.target()
        );
    }
    let snapshot = capture(
        [
            Input::new("a.hpp", "extern \"C\" void __stdcall Use(int value);"),
            Input::new("b.hpp", "extern \"C\" void __cdecl Use(int value);"),
        ],
        &["-x", "c++", "--target=i686-pc-windows-msvc"],
        &["Use"],
    )
    .unwrap();
    snapshot.validate().unwrap_err();
}

#[test]
fn unsupported_evidence_is_not_equivalence() {
    let source = include_str!("../input/unsupported.h");
    capture([Input::new("api.hpp", source)], ARGS, &["Good"])
        .unwrap()
        .validate()
        .unwrap();
    let snapshot = capture([Input::new("api.hpp", source)], ARGS, &["Bad"]).unwrap();
    assert!(snapshot.dump().contains("MemberPointer"));
    assert!(
        snapshot
            .validate()
            .unwrap_err()
            .to_string()
            .contains("unsupported native evidence")
    );
}

#[test]
fn ignores_unselected_conflicts() {
    pair(
        "struct Unselected { VALUE value; }; struct Selected { int value; };",
        "int",
        "float",
        &["Selected"],
    )
    .validate()
    .unwrap();
}

#[test]
fn reports_bad_inputs_and_preserves_existing_loader_registration() {
    clang_sys::load().unwrap();
    let previous = clang_sys::get_library().unwrap();
    capture([Input::new("good.hpp", "struct Good {};")], ARGS, &["Good"])
        .unwrap()
        .validate()
        .unwrap();
    assert!(std::sync::Arc::ptr_eq(
        &previous,
        &clang_sys::get_library().unwrap()
    ));
    assert!(capture([Input::new("bad.hpp", "struct Bad {")], ARGS, &["Bad"]).is_err());
    assert!(
        capture(
            [Input::new("good.hpp", "struct Good {};")],
            ARGS,
            &["Missing"]
        )
        .is_err()
    );
    assert!(
        capture(
            [Input::new("a.hpp", ""), Input::new("a.hpp", "")],
            ARGS,
            &["A"]
        )
        .is_err()
    );
    assert!(std::sync::Arc::ptr_eq(
        &previous,
        &clang_sys::get_library().unwrap()
    ));
    clang_sys::unload().unwrap();
}
