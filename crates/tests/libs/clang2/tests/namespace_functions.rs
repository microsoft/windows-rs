use windows_clang2::*;

#[test]
fn scoped_import_aliases() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("namespace_import_aliases");
    let captured = capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let imports = [(
        "Supported::Equivalent".into(),
        projection::Import {
            library: "native.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let partial = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(partial.output.functions, 1);
    assert!(
        partial
            .omissions
            .iter()
            .all(|message| message.contains("Unsupported::Number")),
        "{:?}",
        partial.omissions
    );
    assert!(!partial.omissions.is_empty());
    let actual = &partial.output.partitions["namespace_import_aliases"].rdl;
    let expected = test_clang2::input_dir().join("namespace_import_aliases.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn scoped_function_alias() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("function_macros", &["Scoped::Hidden"]);
    input.macros.push("ScopedAlias".into());
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        let alias = snapshot
            .values
            .iter()
            .find(|value| value.name == "ScopedAlias")
            .unwrap();
        let ValueKind::Function {
            alias: Some(alias), ..
        } = &alias.observations[0].kind
        else {
            panic!()
        };
        assert_eq!(alias.name, "Scoped::Hidden");
    }
    let imports = [(
        "Scoped::Hidden".into(),
        projection::Import {
            library: "native.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 2);
    let actual = &output.partitions["function_macros"].rdl;
    let expected = test_clang2::input_dir().join("namespace_functions.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}
