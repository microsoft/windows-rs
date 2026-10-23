use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_function_symbols_select_roots_without_hiding_selected_conflicts() {
    helpers::ensure_libclang();
    let input = || {
        let mut input = test_clang::selected_fixture("function_scope", &[]);
        input.macros = ["SelectedAlias", "IgnoredAlias", "ScopeConstant"]
            .map(String::from)
            .into();
        input.function_symbols = Some(
            [
                (Target::X86, "_Selected@4"),
                (Target::X64, "Selected"),
                (Target::Arm64, "Selected"),
            ]
            .into_iter()
            .map(|(target, symbol)| (target, [symbol.to_string()].into()))
            .collect(),
        );
        input
    };
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let names: std::collections::BTreeSet<_> = snapshot
            .values
            .iter()
            .map(|value| value.name.as_str())
            .collect();
        assert_eq!(names, ["Selected", "SelectedAlias", "ScopeConstant"].into());
        assert_eq!(snapshot.records[0].name, "Dependency");
    }
    merge(&snapshots).unwrap();
    let mut explicit = input();
    explicit.roots.push("Ignored".into());
    assert!(capture_partial(&[explicit], &TARGETS).is_err());
    let mut unfiltered = input();
    unfiltered.function_symbols = None;
    assert!(capture_partial(&[unfiltered], &TARGETS).is_err());
    let mut selected = input();
    for (target, symbol) in [
        (Target::X86, "_Ignored@4"),
        (Target::X64, "Ignored"),
        (Target::Arm64, "Ignored"),
    ] {
        selected
            .function_symbols
            .as_mut()
            .unwrap()
            .get_mut(&target)
            .unwrap()
            .insert(symbol.into());
    }
    assert!(capture_partial(&[selected], &TARGETS).is_err());
}

fn input() -> Input {
    let mut input = test_clang::selected_fixture("root_scope", &[]);
    input.root_paths = Some(Vec::new());
    input.macros.push("SUPPORT_CONSTANT".into());
    input
}

#[test]
fn excluded_roots_keep_dependencies_and_observations() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert_eq!(snapshot.roots.len(), 3);
        assert_eq!(snapshot.records.len(), 2);
        assert!(
            snapshot
                .records
                .iter()
                .any(|record| record.name == "SupportRecord")
        );
        assert!(
            !snapshot
                .records
                .iter()
                .any(|record| record.name == "UnusedSupport")
        );
        let function = snapshot
            .values
            .iter()
            .find(|value| value.name == "SharedFunction")
            .unwrap();
        assert_eq!(function.observations.len(), 2);
    }
    let imports = projection::Imports::from_iter([(
        "SharedFunction".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]);
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.records, 2);
    assert_eq!(output.functions, 1);
    assert_eq!(output.constants, 1);
    assert!(
        output.partitions["declarations"]
            .rdl
            .contains("struct SupportRecord")
    );

    let mut other = input();
    other.name = "other.h".into();
    other.source = format!("#define CONFLICT\n{}", other.source);
    let error = capture_partial(&[input(), other], &TARGETS)
        .unwrap_err()
        .to_string();
    assert!(error.contains("differ"), "{error}");
}

#[test]
fn explicit_roots_override_directory_exclusion() {
    helpers::ensure_libclang();
    let mut input = input();
    input.roots.push("UnusedSupport".into());
    input.macros.clear();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in snapshots {
        assert_eq!(snapshot.records.len(), 1);
        assert_eq!(snapshot.records[0].name, "UnusedSupport");
    }
}

#[test]
fn included_paths_select_header_roots() {
    helpers::ensure_libclang();
    for path in ["root_support", "root_support\\declarations.h"] {
        let mut input = input();
        input.root_paths = Some(vec![test_clang::input_dir().join(path)]);
        let captured = capture_partial(&[input], &TARGETS).unwrap();
        for snapshot in captured.snapshots {
            for name in ["UnusedSupport", "OtherSupport"] {
                let rooted = snapshot.roots.iter().any(|id| match id {
                    DeclarationId::Record(id) => snapshot.records[id.0].name == name,
                    _ => false,
                });
                assert_eq!(rooted, name == "UnusedSupport" || path == "root_support");
            }
            for name in ["InterfaceName_Public", "InterfaceName_Support"] {
                let rooted = snapshot.roots.iter().any(|id| match id {
                    DeclarationId::Value(id) => snapshot.values[id.0].name == name,
                    _ => false,
                });
                assert_eq!(
                    rooted,
                    name == "InterfaceName_Public" || path == "root_support"
                );
            }
        }
    }
}

#[test]
fn explicit_support_constant_remains_eligible() {
    helpers::ensure_libclang();
    let mut input = input();
    input.roots = vec!["InterfaceName_Support".into()];
    input.macros.clear();
    for snapshot in capture(&[input], &TARGETS).unwrap() {
        assert_eq!(snapshot.values.len(), 1);
        assert_eq!(snapshot.values[0].name, "InterfaceName_Support");
    }
}

#[test]
fn missing_root_path_is_an_error() {
    helpers::ensure_libclang();
    let mut input = input();
    input.root_paths = Some(vec![test_clang::input_dir().join("missing-root-scope.h")]);
    let error = capture_partial(&[input], &TARGETS).unwrap_err().to_string();
    assert!(
        error.contains("root path") && error.contains("missing-root-scope.h"),
        "{error}"
    );
}

#[test]
fn rejected_roots_do_not_publish_unused_support_dependencies() {
    helpers::ensure_libclang();
    let mut input = input();
    input.source = format!("#define REJECTED_ROOT\n{}", input.source);
    let imports = projection::Imports::from_iter([(
        "SharedFunction".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]);
    let partial = projection::project_partial_with_imports(
        capture_partial(&[input], &TARGETS).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("RejectedRoot"))
    );
    assert!(
        partial
            .output
            .partitions
            .values()
            .all(|partition| !partition.rdl.contains("UnusedSupport"))
    );
    assert_eq!(partial.output.records, 2);
}
