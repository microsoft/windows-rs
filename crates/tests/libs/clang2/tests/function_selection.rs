use std::collections::{BTreeMap, BTreeSet};
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input() -> Input {
    let mut input = test_clang2::fixture("function_selection");
    input.macro_headers.push("function_selection.h".into());
    input
}

fn imports() -> projection::Imports {
    let mut imports: projection::Imports = ["NativeA", "NativeW"]
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
        imports.symbols.entry(target).or_default().insert(
            "Variant".into(),
            projection::Import {
                library: "native.dll".into(),
                name: Some("Variant".into()),
            },
        );
    }
    imports
}

#[test]
fn selected_aliases_preserve_native_declarations() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    let imports = imports();
    let mut selected: BTreeMap<_, _> = TARGETS
        .into_iter()
        .map(|target| (target, BTreeSet::from(["Required".into()])))
        .collect();
    selected
        .get_mut(&Target::X86)
        .unwrap()
        .insert("Variant".into());
    let options = projection::Options {
        imports: Some(&imports),
        function_aliases: Some(&selected),
        ..Default::default()
    };
    let output = projection::project_with_options(&merged, "Fixture", &options).unwrap();
    let actual = &output.partitions["function_selection"].rdl;
    let golden = test_clang2::input_dir().join("function_selection.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, actual).unwrap();
    }
    assert_eq!(
        actual,
        &std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    assert_eq!(output.functions, 4);
    for snapshot in &snapshots {
        for name in ["Neutral", "Compatibility"] {
            assert!(snapshot.values.iter().any(|value| {
                value.name == name
                    && matches!(
                        value.observations[0].kind,
                        ValueKind::Function { alias: Some(_), .. }
                    )
            }));
        }
    }
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
fn empty_selection_keeps_wide_native_variant() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    let imports = imports();
    let selected = BTreeMap::new();
    let output = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &projection::Options {
            imports: Some(&imports),
            function_aliases: Some(&selected),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(output.functions, 3);
    let actual = &output.partitions["function_selection"].rdl;
    assert!(actual.contains("#[arch(X64 | Arm64)]"), "{actual}");
    assert!(actual.contains("fn Variant("), "{actual}");
    assert!(!actual.contains("fn Required("), "{actual}");
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
        .replace("#define Neutral NativeA", "#define Neutral NativeW");
    let snapshots = capture(&[first, second], &TARGETS).unwrap();
    let error = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Fixture",
        &projection::Options {
            function_aliases: Some(&BTreeMap::new()),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(error.to_string().contains("merge"), "{error}");
    assert!(error.to_string().contains("Neutral"), "{error}");
}

#[test]
fn unselected_alias_does_not_claim_a_native_public_name() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("function_macros", &["Shadow"]);
    input.macros.push("Shadow".into());
    let snapshots = capture(&[input], &TARGETS).unwrap();
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
    let output = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &projection::Options {
            imports: Some(&imports),
            function_aliases: Some(&BTreeMap::new()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(output.functions, 2);
    assert!(
        output.partitions["function_macros"]
            .rdl
            .contains("fn Shadow(")
    );
}
