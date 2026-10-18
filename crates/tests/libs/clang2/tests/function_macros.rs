use windows_clang2::*;

#[test]
fn function_macros() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("function_macros", &["Native"]);
    input.macros = ["Public", "Nested", "Parenthesized"]
        .map(String::from)
        .into();
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        for name in ["Public", "Nested", "Parenthesized"] {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            let ValueKind::Function {
                alias: Some(alias), ..
            } = &value.observations[0].kind
            else {
                panic!("missing function alias evidence");
            };
            assert_eq!(alias.name, "Native");
            assert!(!alias.usr.is_empty());
            assert!(snapshot.files[alias.location.file.0].ends_with("function_macros.h"));
        }
    }
    let imports = [(
        "Native".into(),
        projection::Import {
            library: "native.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 4);
    let actual: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang2::input_dir().join("function_macros.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn function_macro_expressions_are_not_aliases() {
    helpers::ensure_libclang();
    for (name, message) in [
        ("Comma", "function macro must directly reference a function"),
        ("Pointer", "numeric pointer initializer unavailable"),
        (
            "ScopedAlias",
            "function alias target is not a captured global declaration",
        ),
    ] {
        let mut input = test_clang2::selected_fixture("function_macros", &["Native"]);
        input.macros.push(name.into());
        let error = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
    }
}

#[test]
fn function_alias_conflicts() {
    helpers::ensure_libclang();
    let mut first = test_clang2::selected_fixture("function_macros", &["Native"]);
    first.macros.push("Public".into());
    let mut second = test_clang2::selected_fixture("function_macros", &["Native"]);
    second.name = "other_macros.h".into();
    second.macros.push("Public".into());
    second.source = second
        .source
        .replace("#define Public Native", "#define Public Other");
    let snapshots = capture(&[first, second], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    assert!(merge(&snapshots).is_err());
}

#[test]
fn function_alias_collision_does_not_hide_native_conflicts() {
    helpers::ensure_libclang();
    let mut first = test_clang2::selected_fixture("function_macros", &["Shadow"]);
    first.macros.push("Shadow".into());
    let mut second = test_clang2::selected_fixture("function_macros", &["Shadow"]);
    second.macros.push("Shadow".into());
    second.name = "conflicting_native.h".into();
    second.source = second.source.replace("Shadow(unsigned", "Shadow(int");
    let mut inputs = [first, second];
    for _ in 0..2 {
        let error =
            capture_partial(&inputs, &[Target::X86, Target::X64, Target::Arm64]).unwrap_err();
        assert!(error.to_string().contains("native"), "{error}");
        assert!(error.to_string().contains("Shadow"), "{error}");
        inputs.reverse();
    }
}

#[test]
fn function_alias_pruning() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("function_macros", &["Native"]);
    input.macros = ["UnsupportedAlias", "Self"].map(String::from).into();
    let captured = capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &captured.snapshots {
        assert!(
            snapshot
                .values
                .iter()
                .all(|value| value.name != "UnsupportedAlias")
        );
        assert!(
            snapshot
                .values
                .iter()
                .all(|value| value.name != "Unsupported")
        );
        assert!(snapshot.values.iter().any(|value| value.name == "Self"));
    }
    let mut input = test_clang2::selected_fixture("function_macros", &["Native"]);
    input.source.push_str("\n#define Native (0, Other)\n");
    input.macros.push("Native".into());
    let captured = capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &captured.snapshots {
        let values: Vec<_> = snapshot
            .values
            .iter()
            .filter(|value| value.name == "Native")
            .collect();
        assert_eq!(values.len(), 1);
        assert!(matches!(
            values[0].observations[0].kind,
            ValueKind::Function { alias: None, .. }
        ));
    }
}

#[test]
fn function_alias_import_ownership() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("function_macros", &["Native"]);
    input.macros.push("Public".into());
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let merged = merge(&snapshots).unwrap();
    let mut imports: projection::Imports = [(
        "Native".into(),
        projection::Import {
            library: "native.dll".into(),
            name: Some("Export".into()),
        },
    )]
    .into_iter()
    .collect();
    imports.functions.insert(
        "Public".into(),
        projection::Import {
            library: "other.dll".into(),
            name: Some("Export".into()),
        },
    );
    let error = projection::project_with_imports(&merged, "Fixture", &imports).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("changes native import ownership"),
        "{error}"
    );
    imports.functions.get_mut("Public").unwrap().library = "NATIVE.DLL".into();
    let output = projection::project_with_imports(&merged, "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 2);
    assert!(
        output
            .partitions
            .values()
            .all(|part| { !part.rdl.contains("library(\"other.dll\"") })
    );
}

#[test]
fn function_alias_native_collision() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("function_macros", &["Shadow"]);
    input.macros.push("Shadow".into());
    let captured = capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    assert!(captured.omissions.is_empty());
    let imports = ["Shadow", "Narrow"]
        .into_iter()
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "native.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let merged = merge(&captured.snapshots).unwrap();
    assert_eq!(merged.values.len(), 3);
    assert_eq!(
        merged
            .values
            .iter()
            .filter(|value| value.name == "Shadow")
            .count(),
        2
    );
    for value in &merged.values {
        assert_eq!(value.groups.len(), 3);
        let mut kinds = std::collections::BTreeSet::new();
        for (target, group) in &value.groups {
            let resolution = &merged.targets[target];
            let reference = resolution.values[*group].observations[0];
            let observation = &resolution.snapshots[reference.snapshot].values[reference.value.0]
                .observations[reference.observation];
            kinds.insert(matches!(
                observation.kind,
                ValueKind::Function { alias: Some(_), .. }
            ));
        }
        assert_eq!(kinds.len(), 1);
    }
    let error = projection::project_with_imports(&merged, "Fixture", &imports).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("collides with a native declaration"),
        "{error}"
    );
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 2);
    assert!(
        result
            .omissions
            .iter()
            .any(|message| message.contains("collides with a native declaration"))
    );
    let actual: String = result
        .output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang2::input_dir().join("function_macro_collision.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
}
