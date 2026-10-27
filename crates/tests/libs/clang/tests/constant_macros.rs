use windows_clang::*;
use windows_metadata::HasAttributes;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn enum_members_keep_native_identity() {
    helpers::ensure_libclang();
    let mut input = test_clang::selected_fixture("enum_macro_collisions", &[]);
    input.macro_headers.push(input.name.clone());
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    for snapshot in &captured.snapshots {
        assert!(
            snapshot
                .enums
                .iter()
                .any(|enumeration| enumeration.name == "Other::Named")
        );
    }
    let strict = projection::project(&merge(&captured.snapshots).unwrap(), "Fixture").unwrap();
    let result = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!(strict, result.output);
    assert_eq!(result.omissions.len(), 1);
    assert!(
        result
            .omissions
            .iter()
            .all(|message| message.contains("Macro(\"FailedMember\")"))
    );
    let text = &result.output.partitions["enum_macro_collisions"].rdl;
    let expected = test_clang::input_dir().join("enum_macro_collisions.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, text).unwrap();
    }
    assert_eq!(
        *text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-enum-macros.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for architecture in [1, 2, 4] {
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(path).unwrap()],
                architecture,
            );
            let field = |namespace, owner, name| {
                index
                    .expect(namespace, owner)
                    .fields()
                    .find(|field| field.name() == name)
                    .unwrap()
            };
            for (name, value) in [
                ("PoolDefault", 0),
                ("PoolAligned", 4),
                ("PoolNx", 512),
                ("PoolNxAligned", 516),
                ("MatchingMember", 7),
                ("FailedMember", 9),
            ] {
                assert_eq!(
                    field("Fixture", "Pool", name).constant().unwrap().value(),
                    windows_metadata::Value::I32(value)
                );
            }
            for (namespace, owner, name, value) in [
                ("Fixture", "Apis", "MatchingMember", 7),
                ("Fixture", "Apis", "ScopedMember", 12),
                ("Fixture", "Apis", "NamespacedMember", 14),
                ("Fixture", "Scoped", "ScopedMember", 11),
                ("Fixture", "Named", "NamespacedMember", 13),
            ] {
                assert_eq!(
                    field(namespace, owner, name).constant().unwrap().value(),
                    windows_metadata::Value::I32(value)
                );
            }
            let apis = index.expect("Fixture", "Apis");
            for (name, value) in [("PoolDefault", 512), ("PoolAligned", 516)] {
                let constant = apis.fields().find(|field| {
                    field.name() == name
                        && (field.arches() == 0 || field.arches() & architecture != 0)
                });
                if architecture == 4 {
                    assert_eq!(
                        constant.unwrap().constant().unwrap().value(),
                        windows_metadata::Value::I32(value)
                    );
                } else {
                    assert!(constant.is_none());
                }
            }
            assert_eq!(
                field("Fixture", "PoolConsumer", "value").ty(),
                windows_metadata::Type::value_named("Fixture", "Pool")
            );
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn enum_macro_collisions_do_not_hide_conflicts() {
    helpers::ensure_libclang();
    for (before, after, expected) in [
        ("PoolDefault = 0", "PoolDefault = 1", "enum members differ"),
        (
            "#define MatchingMember 7",
            "#define MatchingMember 8",
            "native value declarations differ",
        ),
    ] {
        let mut first = test_clang::selected_fixture("enum_macro_collisions", &[]);
        first.macro_headers.push(first.name.clone());
        let mut second = test_clang::selected_fixture("enum_macro_collisions", &[]);
        second.name = "enum_macro_conflict.h".into();
        second.source = second.source.replace(before, after);
        second.macro_headers = vec![second.name.clone()];
        let mut inputs = [first, second];
        for _ in 0..2 {
            let error = capture_partial(&inputs, &TARGETS).unwrap_err();
            assert!(error.to_string().contains(expected), "{error}");
            inputs.reverse();
        }
    }
}

fn inputs() -> [Input; 2] {
    let mut input = test_clang::selected_fixture(
        "constant_macro_collisions",
        &[
            "Changed",
            "Matching",
            "Failed",
            "Linked",
            "DeclarationOnly",
            "Invalid",
            "Unpublishable",
        ],
    );
    input.macros.clone_from(&input.roots);
    input.macros.push("MacroOnly".into());
    [
        input,
        test_clang::selected_fixture("constant_macro_definitions", &["Linked", "Invalid"]),
    ]
}

#[test]
fn failed_and_colliding_macros_preserve_native_constants() {
    helpers::ensure_libclang();
    let mut inputs = inputs();
    let mut expected = None;
    for _ in 0..2 {
        let captured = capture_partial(&inputs, &TARGETS).unwrap();
        let merged = merge(&captured.snapshots).unwrap();
        for name in ["Changed", "Matching"] {
            let groups: Vec<_> = merged
                .values
                .iter()
                .filter(|group| group.name == name)
                .collect();
            assert_eq!(groups.len(), 2, "{name}");
            for group in groups {
                assert_eq!(group.groups.len(), 3);
                let mut origins = std::collections::BTreeSet::new();
                for (target, id) in &group.groups {
                    let resolution = &merged.targets[target];
                    for reference in &resolution.values[*id].observations {
                        let observation = &resolution.snapshots[reference.snapshot].values
                            [reference.value.0]
                            .observations[reference.observation];
                        let ValueKind::Constant { origin, .. } = &observation.kind else {
                            panic!()
                        };
                        origins.insert(*origin == ConstantOrigin::Macro);
                    }
                }
                assert_eq!(origins.len(), 1);
            }
        }
        let error = projection::project(&merged, "Fixture").unwrap_err();
        assert!(
            error.to_string().contains("has no captured definition"),
            "{error}"
        );
        let result = projection::project_partial(captured, "Fixture").unwrap();
        assert_eq!(result.output.constants, 6);
        for name in ["Changed", "Matching", "Failed", "Linked"] {
            assert!(
                result
                    .omissions
                    .iter()
                    .any(|message| message.contains(&format!("Macro(\"{name}\")"))),
                "{:?}",
                result.omissions
            );
            assert!(
                !result
                    .omissions
                    .iter()
                    .any(|message| message.contains(&format!("Value(\"{name}\")"))),
                "{:?}",
                result.omissions
            );
        }
        for name in ["DeclarationOnly", "Invalid", "Unpublishable"] {
            assert!(
                result
                    .omissions
                    .iter()
                    .any(|message| message.contains(&format!("Value(\"{name}\")"))),
                "{:?}",
                result.omissions
            );
        }
        assert!(
            !result
                .omissions
                .iter()
                .any(|message| message.contains("Macro(\"DeclarationOnly\")")),
            "{:?}",
            result.omissions
        );
        let text: String = result
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        for name in ["Invalid", "Unpublishable"] {
            assert!(!text.contains(&format!("const {name}:")), "{text}");
        }
        if let Some(expected) = &expected {
            assert_eq!(&text, expected);
        } else {
            expected = Some(text.clone());
        }
        check_metadata(&text);
        inputs.reverse();
    }
    let golden = test_clang::input_dir().join("constant_macro_collisions.rdl");
    let text = expected.unwrap();
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

fn check_metadata(text: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-constant-macros.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for architecture in [1, 2, 4] {
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(path).unwrap()],
                architecture,
            );
            let apis = index.expect("Fixture", "Apis");
            for (name, expected) in [
                ("Changed", 7),
                ("Matching", 9),
                ("Failed", 11),
                ("Linked", 17),
                ("MacroOnly", 13),
                ("DeclarationOnly", 23),
            ] {
                let field = apis.fields().find(|field| field.name() == name).unwrap();
                assert_eq!(
                    field.constant().unwrap().value(),
                    windows_metadata::Value::I32(expected)
                );
            }
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn collisions_do_not_hide_native_or_macro_conflicts() {
    helpers::ensure_libclang();
    for (before, after) in [
        ("Changed = 7", "Changed = 6"),
        ("#define Changed 8", "#define Changed 10"),
        ("Matching = 9", "Matching = 10"),
        ("Failed = 11", "Failed = 12"),
    ] {
        let [mut first, _] = inputs();
        first.roots = vec!["Changed".into(), "Matching".into(), "Failed".into()];
        first.macros = first.roots.clone();
        let [mut second, _] = inputs();
        second.name = "conflicting_constants.h".into();
        second.source = first.source.replace(before, after);
        second.roots.clone_from(&first.roots);
        second.macros.clone_from(&first.macros);
        let mut inputs = [first, second];
        for _ in 0..2 {
            let error = capture_partial(&inputs, &TARGETS).unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("native value declarations differ"),
                "{error}"
            );
            inputs.reverse();
        }
    }
}
