use test_clang::{fixture, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn namespace_binary_roots() {
    helpers::ensure_libclang();
    for roots in [NamespaceRoots::All, NamespaceRoots::Binary] {
        let mut input = selected_fixture("namespace_selection", &[]);
        input.namespace_roots = roots;
        input.root_paths = Some(Vec::new());
        let captured = capture_partial(&[input], &TARGETS).unwrap();
        let omissions = &captured.omissions;
        if roots == NamespaceRoots::All {
            assert_eq!(omissions.len(), 1);
            assert!(omissions[0].contains("Selected::Helper"), "{omissions:?}");
        } else {
            assert!(omissions.is_empty(), "{omissions:?}");
        }
        for snapshot in captured.snapshots {
            let mut names: Vec<_> = snapshot
                .roots
                .iter()
                .map(|root| match *root {
                    DeclarationId::Record(id) => snapshot.records[id.0].name.as_str(),
                    DeclarationId::Enum(id) => snapshot.enums[id.0].name.as_str(),
                    DeclarationId::Value(id) => snapshot.values[id.0].name.as_str(),
                    DeclarationId::Alias(id) => snapshot.aliases[id.0].name.as_str(),
                })
                .collect();
            names.sort_unstable();
            if roots == NamespaceRoots::All {
                assert_eq!(
                    names,
                    [
                        "Selected::Flat",
                        "Selected::IRoot",
                        "Selected::Mode",
                        "Selected::Root"
                    ]
                );
            } else {
                assert_eq!(names, ["Selected::Flat", "Selected::IRoot"]);
                assert!(snapshot.enums.is_empty());
            }
            let mut records: Vec<_> = snapshot
                .records
                .iter()
                .map(|record| record.name.as_str())
                .collect();
            records.sort_unstable();
            assert_eq!(
                records,
                [
                    "Right::Dependent",
                    "Right::Item",
                    "Selected::IRoot",
                    "Selected::Root"
                ]
            );
        }
    }
}

#[test]
fn namespace_native_identity() {
    helpers::ensure_libclang();
    let snapshots = capture(&[fixture("namespace_identity")], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(
        merged
            .aliases
            .iter()
            .map(|alias| alias.name.as_str())
            .collect::<Vec<_>>(),
        ["Left::Callback", "Left::Number", "Right::Number"]
    );
    assert_eq!(
        merged
            .values
            .iter()
            .map(|value| value.name.as_str())
            .collect::<Vec<_>>(),
        ["Left::Value", "Right::Value"]
    );
    for snapshot in &snapshots {
        let child = snapshot
            .records
            .iter()
            .find(|record| record.name == "Child")
            .unwrap();
        let owner = child.owner.unwrap();
        assert_eq!(snapshot.records[owner.record.0].name, "Left::Container");
        assert_eq!(
            snapshot
                .enums
                .iter()
                .map(|value| value.name.as_str())
                .collect::<Vec<_>>(),
            ["Left::Kind", "Right::Kind", "Left::__MIDL_fixture_0001"]
        );
    }
    let selected = capture(
        &[selected_fixture("namespace_identity", &["Left::Item"])],
        &TARGETS,
    )
    .unwrap();
    for snapshot in selected {
        assert_eq!(snapshot.records.len(), 1);
        assert_eq!(snapshot.records[0].name, "Left::Item");
        assert_eq!(snapshot.aliases[0].name, "Left::Number");
    }
}

#[test]
fn namespace_partial_identity() {
    helpers::ensure_libclang();
    let partial = capture_partial(&[fixture("namespace_partial")], &TARGETS).unwrap();
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("Left::Item"))
    );
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("Left::Dependent"))
    );
    for snapshot in &partial.snapshots {
        let names: std::collections::BTreeSet<_> = snapshot
            .records
            .iter()
            .map(|record| record.name.as_str())
            .collect();
        assert_eq!(names, ["Right::Dependent", "Right::Item"].into());
    }
    merge(&partial.snapshots).unwrap();
}

#[test]
fn namespace_public_names() {
    helpers::ensure_libclang();
    let input = selected_fixture(
        "namespace_identity",
        &[
            "Left::Container",
            "Left::Value",
            "Left::Kind",
            "Left::Callbacks",
            "Left::__MIDL_fixture_0001",
        ],
    );
    let output = projection::generate(&[input], "Fixture").unwrap();
    let actual = &output.partitions["namespace_identity"].rdl;
    let expected = test_clang::input_dir().join("namespace_identity.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-namespaces");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input_text(actual)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("output.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(*actual, std::fs::read_to_string(rewritten).unwrap());
}

#[test]
fn namespace_public_collisions() {
    helpers::ensure_libclang();
    for roots in [
        ["Left::Item", "Right::Item"],
        ["Left::Number", "Right::Number"],
        ["Left::Kind", "Right::Kind"],
        ["Left::Value", "Right::Value"],
    ] {
        let input = selected_fixture("namespace_identity", &roots);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("collides between native"),
            "{roots:?}: {error}"
        );
        let error = projection::project_partial(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("collides between native"),
            "{roots:?}: {error}"
        );
    }
    for name in ["namespace_anonymous", "namespace_targets"] {
        let error = projection::generate_partial(&[fixture(name)], "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("collides between native"), "{name}: {error}");
    }
}

#[test]
fn namespace_native_conflicts() {
    helpers::ensure_libclang();
    let mut inputs = ["int", "unsigned int"].map(|field| {
        let mut input = selected_fixture("namespace_conflict", &["Scoped::Item"]);
        input
            .source
            .insert_str(0, &format!("#define FIELD {field}\n"));
        input
    });
    inputs[1].name = "other.h".into();
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("native field types differ"), "{error}");
        inputs.reverse();
    }
}
