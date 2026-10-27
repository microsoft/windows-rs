use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input() -> Input {
    let mut input = test_clang::fixture("function_selection");
    input.macro_headers.push("function_selection.h".into());
    input.alias_variants = vec![vec!["-DUNICODE".into()]];
    input
}

fn imports() -> projection::Imports {
    let mut imports: projection::Imports = ["NativeA", "NativeW", "Shadow"]
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
    for target in [Target::X64, Target::Arm64] {
        for name in ["Variant", "Wide"] {
            imports.symbols.entry(target).or_default().insert(
                name.into(),
                projection::Import {
                    library: "native.dll".into(),
                    name: Some(name.into()),
                },
            );
        }
    }
    imports
}

fn alias<'a>(snapshot: &'a Snapshot, name: &str) -> &'a FunctionAlias {
    snapshot
        .values
        .iter()
        .filter(|value| value.name == name)
        .flat_map(|value| &value.observations)
        .find_map(|observation| match &observation.kind {
            ValueKind::Function {
                alias: Some(alias), ..
            } => Some(&**alias),
            _ => None,
        })
        .unwrap()
}

#[test]
fn variant_aliases_are_not_published() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let switched = alias(snapshot, "Switched");
        assert_eq!(switched.name, "NativeA");
        assert_eq!(switched.variants, [Some("NativeW".to_string())]);
        let invariant = alias(snapshot, "Invariant");
        assert_eq!(invariant.variants, [Some("NativeW".to_string())]);
    }
    let merged = merge(&snapshots).unwrap();
    let imports = imports();
    let options = projection::Options {
        imports: Some(&imports),
        ..Default::default()
    };
    let output = projection::project_with_options(&merged, "Fixture", &options).unwrap();
    let actual = &output.partitions["function_selection"].rdl;
    let golden = test_clang::input_dir().join("function_selection.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, actual).unwrap();
    }
    assert_eq!(
        actual,
        &std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let partial = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Fixture",
        &options,
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(output, partial.output);
}

#[test]
fn aliases_without_variants_are_published() {
    helpers::ensure_libclang();
    let mut input = input();
    input.alias_variants.clear();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    assert!(alias(&snapshots[0], "Switched").variants.is_empty());
    let imports = imports();
    let output = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Fixture",
        &projection::Options {
            imports: Some(&imports),
            ..Default::default()
        },
    )
    .unwrap();
    assert!(
        output
            .omissions
            .iter()
            .all(|omission| omission.contains("function alias `Shadow` collides")),
        "{:?}",
        output.omissions
    );
    let actual = &output.output.partitions["function_selection"].rdl;
    assert!(actual.contains("fn Switched("), "{actual}");
    assert!(actual.contains("#[arch(X86)]"), "{actual}");
}

#[test]
fn excluded_aliases_do_not_hide_native_conflicts() {
    helpers::ensure_libclang();
    let first = input();
    let mut second = input();
    second.name = "other_selection.h".into();
    second.macro_headers = vec![second.name.clone()];
    second.source = second
        .source
        .replace("#define Switched NativeA", "#define Switched NativeW");
    let snapshots = capture(&[first, second], &TARGETS).unwrap();
    let error = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Fixture",
        &projection::Options::default(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("merge"), "{error}");
    assert!(error.to_string().contains("Switched"), "{error}");
}
