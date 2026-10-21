use windows_clang2::*;
use windows_metadata::{HasAttributes, reader};
use windows_rdl::emit::SuccessCondition;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn predicate<'a>(owner: impl HasAttributes<'a>) -> Option<SuccessCondition> {
    let attributes: Vec<_> = owner
        .attributes()
        .filter(|attribute| attribute.name() == "SuccessConditionAttribute")
        .collect();
    assert!(attributes.len() <= 1);
    attributes.first().map(|attribute| {
        let values = attribute.value();
        assert_eq!(values.len(), 1);
        SuccessCondition::from_value(&values[0].1).unwrap()
    })
}

#[test]
fn success_contracts() {
    use SuccessCondition::*;
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang2::fixture("success")], &TARGETS).unwrap();
    let imports = snapshots[0]
        .values
        .iter()
        .map(|value| {
            (
                value.name.clone(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let text = &output.partitions["success"].rdl;
    let golden = test_clang2::input_dir().join("success.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        text,
        &std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-success.winmd");
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
            let index = reader::Index::new_for_architecture(
                vec![reader::File::read(path).unwrap()],
                architecture,
            );
            for (owner, name, expected) in [
                ("Apis", "SignedResult", Nonnegative),
                ("Apis", "ExactOne", EqualOne),
                ("Apis", "Truth", NotEqualZero),
                ("Apis", "BoolTruth", NotEqualZero),
                ("Apis", "FalseTruth", NotEqualZero),
                ("Apis", "NonNull", NotEqualZero),
                ("Apis", "NullSuccess", EqualZero),
                ("Apis", "Positive", Positive),
                ("Apis", "EnumResult", Nonnegative),
                ("Apis", "Override", EqualZero),
                ("Apis", "LegacyZero", EqualZero),
                ("Apis", "Duplicate", EqualZero),
                ("IStatus", "Read", EqualZero),
                ("IStatus", "Exact", EqualOne),
            ] {
                let method = index
                    .expect("Fixture", owner)
                    .methods()
                    .find(|method| method.name() == name)
                    .unwrap();
                assert_eq!(predicate(method), Some(expected), "{name}");
                assert!(
                    method.find_attribute("NativeSalAttribute").is_none(),
                    "{name}"
                );
            }
            for (name, expected) in [
                ("Status", Nonnegative),
                ("ZeroStatus", EqualZero),
                ("Callback", EqualZero),
                ("CallbackAlias", EqualZero),
            ] {
                let ty = index.expect("Fixture", name);
                assert_eq!(predicate(ty), Some(expected), "{name}");
                assert!(ty.find_attribute("NativeSalAttribute").is_none());
            }
            for name in ["CopyStatus", "CompetingStatus"] {
                assert_eq!(
                    index.expect("Fixture", name).underlying_type(),
                    Some(windows_metadata::Type::value_named("Fixture", "Status"))
                );
            }
            assert_eq!(predicate(index.expect("Fixture", "CopyStatus")), None);
            let pointer = index.expect("Fixture", "PointerCopy");
            assert_eq!(predicate(pointer), None);
            assert_eq!(
                pointer.underlying_type(),
                Some(windows_metadata::Type::PtrMut(
                    Box::new(windows_metadata::Type::I32),
                    1
                ))
            );
            for name in [
                "PStatus",
                "CompetingStatus",
                "UnsignedStatus",
                "FloatStatus",
                "NamedStatus",
                "ConflictingCallback",
            ] {
                let ty = index.expect("Fixture", name);
                assert_eq!(predicate(ty), None, "{name}");
                assert!(ty.find_attribute("NativeSalAttribute").is_some(), "{name}");
            }
            for name in [
                "UnsignedResult",
                "ScopedEnumResult",
                "Conflicting",
                "Always",
                "Bounded",
                "Partial",
                "InvalidPointer",
                "OnePointer",
                "Floating",
                "Scoped",
                "Expanded",
            ] {
                let method = index
                    .expect("Fixture", "Apis")
                    .methods()
                    .find(|method| method.name() == name)
                    .unwrap();
                assert_eq!(predicate(method), None, "{name}");
                assert!(
                    method.find_attribute("NativeSalAttribute").is_some(),
                    "{name}"
                );
            }
            for name in ["InheritedDefault", "UnresolvedDefault", "Parameter"] {
                let method = index
                    .expect("Fixture", "Apis")
                    .methods()
                    .find(|method| method.name() == name)
                    .unwrap();
                assert_eq!(predicate(method), None, "{name}");
                if name == "Parameter" {
                    let parameter = method.params().next().unwrap();
                    assert_eq!(predicate(parameter), None);
                    assert!(parameter.find_attribute("NativeSalAttribute").is_some());
                }
            }
        }
    }
    let partial = projection::project_partial_with_options(
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
    assert!(partial.omissions.is_empty());
    assert_eq!(partial.output, output);
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn default_alias_links_use_reconciled_annotations() {
    helpers::ensure_libclang();
    let mut plain = test_clang2::selected_fixture("success", &["CopyStatus"]);
    plain.name = "copy.h".into();
    plain.source = plain.source.replace(
        "typedef DEFAULT(return >= 0) long Status",
        "typedef long Status",
    );
    let mut annotated = test_clang2::selected_fixture("success", &["Status"]);
    annotated.name = "status.h".into();
    let mut inputs = [plain, annotated];
    let mut previous = None;
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(text.contains("type CopyStatus = Status;"), "{text}");
        assert!(text.contains("#[success(return >= 0)]"), "{text}");
        if let Some(previous) = &previous {
            assert_eq!(&output, previous);
        }
        previous = Some(output);
        inputs.reverse();
    }
}

#[test]
fn success_native_conflicts_remain_errors() {
    helpers::ensure_libclang();
    for (name, from, to) in [
        (
            "SignedResult",
            "SUCCESS(return >= 0) int SignedResult",
            "SUCCESS(return > 0) int SignedResult",
        ),
        (
            "Truth",
            "SUCCESS(return != 0) int Truth",
            "SUCCESS(return) int Truth",
        ),
        (
            "Status",
            "DEFAULT(return >= 0) long Status",
            "DEFAULT(return == 0) long Status",
        ),
        (
            "IStatus",
            "SUCCESS(return == 0) Status __stdcall Read",
            "SUCCESS(return != 0) Status __stdcall Read",
        ),
        (
            "Callback",
            "SUCCESS(return == 0) int (__stdcall *Callback)",
            "SUCCESS(return != 0) int (__stdcall *Callback)",
        ),
    ] {
        let first = test_clang2::selected_fixture("success", &[name]);
        let mut second = test_clang2::selected_fixture("success", &[name]);
        second.name = "other.h".into();
        second.source = second.source.replace(from, to);
        let mut inputs = [first, second];
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            let error = projection::project_partial_with_options(
                PartialCapture {
                    snapshots,
                    omissions: Vec::new(),
                },
                "Fixture",
                &projection::Options::default(),
            )
            .unwrap_err();
            assert!(error.to_string().contains(name), "{error}");
            inputs.reverse();
        }
    }
}
