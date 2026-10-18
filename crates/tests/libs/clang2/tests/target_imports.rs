use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn imports(snapshots: &[Snapshot]) -> projection::Imports {
    let mut imports = projection::Imports::default();
    for snapshot in snapshots
        .iter()
        .filter(|snapshot| snapshot.target != Target::X86)
    {
        for value in &snapshot.values {
            for observation in &value.observations {
                if let ValueKind::Function {
                    alias: None,
                    mangling,
                    ..
                } = &observation.kind
                {
                    imports.symbols.entry(snapshot.target).or_default().insert(
                        mangling.clone(),
                        projection::Import {
                            library: "fixture.dll".into(),
                            name: Some(value.name.clone()),
                        },
                    );
                }
            }
        }
    }
    imports
}

#[test]
fn unresolved_imports_are_target_local() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("target_imports", &["Available"]);
    input.macros.push("PublicAvailable".into());
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty());
    let imports = imports(&captured.snapshots);
    let error =
        projection::project_with_imports(&merge(&captured.snapshots).unwrap(), "Fixture", &imports)
            .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("no configured DLL/export ownership"),
        "{error}"
    );
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 2);
    assert_eq!(result.omissions.len(), 2);
    assert!(
        result.omissions.iter().all(
            |line| line.contains("X86:") && line.contains("no configured DLL/export ownership")
        )
    );
    let actual: String = result
        .output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang2::input_dir().join("target_imports.rdl");
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
fn all_unresolved_imports_remain_explicit() {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture("target_imports", &["Available"]);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    let result = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!(result.output.functions, 0);
    assert_eq!(result.omissions.len(), 3);
    for target in TARGETS {
        assert!(result.omissions.iter().any(|line| {
            line.contains(&format!("{target:?}:"))
                && line.contains("no configured DLL/export ownership")
        }));
    }
}

#[test]
fn missing_imports_do_not_hide_contract_failures() {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture("target_imports", &["Unsupported"]);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty());
    let imports = imports(&captured.snapshots);
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 0);
    assert!(
        result
            .omissions
            .iter()
            .any(|line| line.contains("volatile")),
        "{:?}",
        result.omissions
    );
}

#[test]
fn invalid_imports_remain_global_failures() {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture("target_imports", &["Available"]);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    let mut imports = imports(&captured.snapshots);
    imports
        .symbols
        .get_mut(&Target::X64)
        .unwrap()
        .values_mut()
        .next()
        .unwrap()
        .library
        .clear();
    let result = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(result.output.functions, 0);
    assert!(
        result
            .omissions
            .iter()
            .any(|line| line.contains("empty DLL/export ownership"))
    );
}
