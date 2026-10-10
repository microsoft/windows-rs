use windows_clang2::{
    Input, ProjectionOptions, capture, capture_report, capture_report_with_progress,
};
use windows_metadata::{Type, Value, reader::*};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

#[test]
fn macros_do_not_replace_same_named_native_declarations() {
    let source = include_str!("../input/macro_identity.h");
    for reverse in [false, true] {
        let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
        if reverse {
            inputs.reverse();
        }
        let report =
            capture_report(inputs, ARGS, &["Name", "External", "Initialized", "Use"]).unwrap();
        assert!(report.rejected.is_empty());
        let snapshot = report.snapshot.unwrap();
        assert!(snapshot.resolve().is_err());
        let assessed = snapshot.assess().unwrap();
        assert_eq!(
            assessed
                .rejected
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["External", "Name"]
        );
        let resolved = assessed.resolved.unwrap();
        let mut options = ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        let plan = resolved.project(&options).unwrap();
        let rdl = plan.rdl();
        let expected =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("expected\\macro_identity.rdl");
        if std::env::var_os("UPDATE_EXPECT").is_some() {
            std::fs::write(&expected, &rdl).unwrap();
        }
        assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
        let output = std::path::Path::new(env!("OUT_DIR")).join("macro_identity.winmd");
        windows_rdl::reader()
            .input_text(&rdl)
            .output(&output)
            .write()
            .unwrap();
        let index = Index::read(output).unwrap();
        assert_eq!(
            index.expect("Test", "Name").fields().next().unwrap().ty(),
            Type::I32
        );
        let Item::Const(value) = index.expect_item("Test", "Initialized") else {
            panic!()
        };
        assert_eq!(value.constant().unwrap().value(), Value::I32(17));
        assert!(resolved.project_roots(&options, &["Name"]).is_err());
        assert!(resolved.project_roots(&options, &["Use"]).is_ok());
    }
}

#[test]
fn macro_value_conflicts_are_not_hidden_by_internal_variable_identity() {
    let source = include_str!("../input/macro_identity.h");
    let snapshot = capture(
        [
            Input::new("a.hpp", source),
            Input::new("b.hpp", source.replace("= 17", "= 19")),
        ],
        ARGS,
        &["Initialized"],
    )
    .unwrap();
    assert!(snapshot.resolve().is_err());
    assert!(snapshot.assess().is_err());
}

#[test]
fn failed_probes_do_not_supply_error_bearing_native_evidence() {
    let source = include_str!("../input/probe_failures.h");
    let roots = [
        "Good", "Missing", "Type", "Broken", "Removed", "Function", "Helper", "Use", "Changed",
        "Use",
    ];
    assert!(capture([Input::new("probes.hpp", source)], ARGS, &roots).is_err());
    let report = capture_report([Input::new("probes.hpp", source)], ARGS, &roots).unwrap();
    assert_eq!(
        report
            .rejected
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["Broken", "Function", "Missing", "Removed", "Type"]
    );
    assert_eq!(report.parses, 4);
    assert!(report.rejected["Missing"].contains("absent_identifier"));
    assert!(report.rejected["Removed"].contains("undefined"));
    let snapshot = report.snapshot.unwrap();
    let resolved = snapshot.resolve().unwrap();
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let plan = resolved.project(&options).unwrap();
    assert!(plan.rdl().contains("const Good: i32 = 17"));
    assert!(plan.rdl().contains("const Helper: i32 = 17"));
    assert!(plan.rdl().contains("const Changed: i32 = 23"));
    assert!(resolved.project_roots(&options, &["Missing"]).is_err());
}

#[test]
fn probe_batches_do_not_reparse_once_per_bad_macro() {
    for count in [32, 64, 128] {
        let mut source = String::from("#define Good 19\n");
        let mut roots = vec!["Good".to_string()];
        for index in 0..count {
            let name = format!("Bad{index}");
            source.push_str(&format!("#define {name} missing_{index}\n"));
            roots.push(name);
        }
        let report = capture_report(
            [Input::new("many.hpp", source)],
            ARGS,
            &roots.iter().map(String::as_str).collect::<Vec<_>>(),
        )
        .unwrap();
        assert_eq!(report.rejected.len(), count);
        assert_eq!(report.parses, 3);
        report.snapshot.unwrap().resolve().unwrap();
    }
}

#[test]
fn syntax_recovery_reuses_the_tu_and_keeps_later_valid_probes() {
    for count in [32, 64, 128] {
        let mut source = String::new();
        let mut roots = Vec::new();
        for index in 0..count {
            let name = format!("Bad{index:03}");
            source.push_str(&format!("#define {name} {{\n"));
            roots.push(name);
        }
        source.push_str("#define ZGood 19\n");
        roots.push("ZGood".into());
        let mut progress = Vec::new();
        let report = capture_report_with_progress(
            [Input::new("syntax-recovery.hpp", source)],
            ARGS,
            &roots.iter().map(String::as_str).collect::<Vec<_>>(),
            |event| progress.push((event.parses, event.reparsed, event.errors)),
        )
        .unwrap();
        assert_eq!(report.rejected.len(), count);
        assert_eq!(report.parses, count + 2);
        assert_eq!(progress.len(), report.parses);
        assert!(progress[2..].iter().all(|(_, reparsed, _)| *reparsed));
        assert_eq!(progress.last().unwrap().2, 0);
        let rdl = report
            .snapshot
            .unwrap()
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap()
            .rdl();
        assert_eq!(rdl, "#[win32]\nmod Test {\n    const ZGood: i32 = 19;\n}\n");
    }
}

#[test]
fn recovery_reparses_preserve_preprocessing_state_and_finish_with_a_complete_ast() {
    let mut source = include_str!("../input/probe_counter.h").to_string();
    let mut roots = vec!["ACounter".to_string(), "ZGood".to_string()];
    for index in 0..512 {
        let name = format!("Bad{index:04}");
        source.push_str(&format!("\n#define {name} {{\n"));
        roots.push(name);
    }
    let mut sizes = Vec::new();
    let report = capture_report_with_progress(
        [Input::new("counter-recovery.hpp", &source)],
        ARGS,
        &roots.iter().map(String::as_str).collect::<Vec<_>>(),
        |event| sizes.push(event.probes),
    )
    .unwrap();
    assert_eq!(report.rejected.len(), 512);
    assert_eq!(sizes[1], 514);
    assert!(sizes[1..].windows(2).all(|pair| pair[1] < pair[0]));
    assert_eq!(*sizes.last().unwrap(), 2);
    let rdl = report
        .snapshot
        .unwrap()
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap()
        .rdl();
    let clean = capture(
        [Input::new("counter-recovery.hpp", source)],
        ARGS,
        &["ACounter", "ZGood"],
    )
    .unwrap();
    assert_eq!(
        rdl,
        clean
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap()
            .rdl()
    );
}

#[test]
fn included_macro_failures_keep_their_probe_ownership() {
    let header =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input\\probe_macro_locations.h");
    let report = capture_report(
        [Input::new(
            "macro-locations.hpp",
            format!("#include \"{}\"\n", header.display()),
        )],
        ARGS,
        &["Good", "MissingAlias", "MemberAlias", "StorageSpecifier"],
    )
    .unwrap();
    assert_eq!(
        report
            .rejected
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["MemberAlias", "MissingAlias", "StorageSpecifier"],
    );
    report.snapshot.unwrap().resolve().unwrap();
}

#[test]
fn untyped_braced_initializers_are_explicit_rejections_not_expression_syntax_errors() {
    let report = capture_report(
        [Input::new(
            "initializers.hpp",
            include_str!("../input/macro_initializers.h"),
        )],
        ARGS,
        &[
            "Initializer",
            "Alias",
            "EmptyInitializer",
            "Good",
            "String",
            "Typed",
            "ScalarCall",
        ],
    )
    .unwrap();
    assert_eq!(report.parses, 2);
    assert_eq!(
        report
            .rejected
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["Alias", "EmptyInitializer", "Initializer"]
    );
    assert!(
        report
            .rejected
            .values()
            .all(|reason| reason.contains("native initialization target"))
    );
    let assessment = report.snapshot.as_ref().unwrap().assess().unwrap();
    let resolved = assessment.resolved.unwrap();
    let options = ProjectionOptions::new("Test");
    let rdl = resolved
        .project_roots(&options, &["Good", "String", "ScalarCall"])
        .unwrap()
        .rdl();
    assert!(rdl.contains("const Good: i32 = 40"), "{rdl}");
    assert!(rdl.contains("const ScalarCall: i32 = 18"), "{rdl}");
    assert!(resolved.project_roots(&options, &["Typed"]).is_err());
}

#[test]
fn parser_recovery_rechecks_remaining_probes_until_the_ast_is_clean() {
    let report = capture_report(
        [Input::new(
            "recovery.hpp",
            include_str!("../input/probe_recovery.h"),
        )],
        ARGS,
        &["AGood", "BMalformed", "CMissing", "DMember", "ZGood"],
    )
    .unwrap();
    assert_eq!(
        report
            .rejected
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["BMalformed", "CMissing", "DMember"],
    );
    let snapshot = report.snapshot.unwrap();
    let rdl = snapshot
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap()
        .rdl();
    assert!(rdl.contains("const AGood: i32 = 17"), "{rdl}");
    assert!(rdl.contains("const ZGood: i32 = 23"), "{rdl}");
}

#[test]
fn all_rejected_probes_and_source_errors_remain_explicit() {
    let report = capture_report(
        [Input::new("all.hpp", "#define Bad missing\n")],
        ARGS,
        &["Bad"],
    )
    .unwrap();
    assert!(report.snapshot.is_none());
    assert_eq!(report.rejected.len(), 1);
    assert!(
        capture_report(
            [Input::new(
                "source.hpp",
                "struct Packet { Missing value; };\n#define Good 1"
            )],
            ARGS,
            &["Good"],
        )
        .is_err()
    );
}

#[test]
fn probe_instantiation_errors_outside_owned_ranges_remain_fatal() {
    let error = capture_report(
        [Input::new(
            "unowned.hpp",
            include_str!("../input/probe_unowned.h"),
        )],
        ARGS,
        &["Good", "Bad"],
    )
    .err()
    .unwrap();
    assert!(error.to_string().contains("unowned.hpp:1:"), "{error}");
}

#[test]
fn availability_follows_all_observations_and_cyclic_dependencies() {
    let source = include_str!("../input/unavailable_closure.h");
    for reverse in [false, true] {
        let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
        if reverse {
            inputs.reverse();
        }
        let snapshot = capture(inputs, ARGS, &["Invalid", "Node", "TooWide", "Use"]).unwrap();
        assert!(snapshot.resolve().is_err());
        let assessed = snapshot.assess().unwrap();
        assert_eq!(
            assessed
                .rejected
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["Invalid", "Node", "TooWide"]
        );
        let resolved = assessed.resolved.unwrap();
        assert_eq!(resolved.report().observations, 4);
        let mut options = ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        assert!(resolved.project_roots(&options, &["Use"]).is_ok());
        assert!(resolved.project_roots(&options, &["Invalid"]).is_err());
    }
}

#[test]
fn unsupported_roots_cannot_hide_available_native_conflicts() {
    let source = include_str!("../input/unavailable_closure.h");
    let snapshot = capture(
        [
            Input::new("a.hpp", source),
            Input::new("b.hpp", source.replace("int value", "float value")),
        ],
        ARGS,
        &["Invalid", "Use"],
    )
    .unwrap();
    let error = snapshot.assess().err().unwrap();
    assert!(
        error
            .to_string()
            .contains("conflicting native declarations"),
        "{error}"
    );
}

#[test]
fn rejected_probe_observations_cannot_be_replaced_by_a_successful_tu() {
    let report = capture_report(
        [
            Input::new(
                "a.hpp",
                "#define Value missing\nstruct Packet { int value; };",
            ),
            Input::new("b.hpp", "#define Value 17\nstruct Packet { int value; };"),
        ],
        ARGS,
        &["Value", "Packet"],
    )
    .unwrap();
    assert!(report.rejected.contains_key("Value"));
    let snapshot = report.snapshot.unwrap();
    let resolved = snapshot.resolve().unwrap();
    assert!(
        resolved
            .project_roots(&ProjectionOptions::new("Test"), &["Value"])
            .is_err()
    );
}

#[test]
fn probe_rejection_does_not_hide_valid_native_conflicts() {
    let report = capture_report(
        [
            Input::new(
                "a.hpp",
                "#define Bad missing\nstruct Packet { int value; };",
            ),
            Input::new(
                "b.hpp",
                "#define Bad missing\nstruct Packet { float value; };",
            ),
        ],
        ARGS,
        &["Bad", "Packet"],
    )
    .unwrap();
    assert!(report.snapshot.unwrap().assess().is_err());
}

#[test]
fn unavailability_propagation_visits_unique_group_edges() {
    for depth in [32, 64, 128] {
        let mut source =
            String::from("enum Bad : unsigned __int128 { Large = (unsigned __int128)1 << 100 };\n");
        source.push_str(&format!("struct N{depth} {{ N0* back; Bad* value; }};\n"));
        source.insert_str(0, "struct N0;\n");
        for index in (0..depth).rev() {
            source.push_str(&format!(
                "struct N{index} {{ N{}* first; N{}* second; }};\n",
                index + 1,
                index + 1
            ));
        }

        let snapshot = capture([Input::new("edges.hpp", source)], ARGS, &["N0"]).unwrap();
        let assessment = snapshot.assess().unwrap();
        assert_eq!(assessment.dependency_edges, depth + 2);
        assert_eq!(assessment.unavailable_groups, depth + 2);
        assert!(assessment.resolved.is_none());
        assert_eq!(assessment.rejected.len(), 1);
    }
}

#[test]
fn unavailable_overloads_reject_the_whole_named_root() {
    let snapshot = capture(
        [Input::new(
            "overloads.hpp",
            include_str!("../input/unavailable_closure.h"),
        )],
        ARGS,
        &["Overloaded", "Use"],
    )
    .unwrap();
    let assessed = snapshot.assess().unwrap();
    assert!(assessed.rejected.contains_key("Overloaded"));
    let resolved = assessed.resolved.unwrap();
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    assert!(resolved.project_roots(&options, &["Overloaded"]).is_err());
    assert!(
        !resolved
            .project(&options)
            .unwrap()
            .rdl()
            .contains("Overloaded")
    );
}
