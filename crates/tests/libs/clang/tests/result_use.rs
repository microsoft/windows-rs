use windows_clang::*;
use windows_metadata::{HasAttributes, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn callable_result_use() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("result_use")], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for (name, annotation) in [
            ("Checked", "_Check_return_"),
            ("Inspected", "_Must_inspect_result_"),
        ] {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            assert!(value.observations[0].attributes.iter().any(|attribute| {
                matches!(&attribute.kind, AttributeKind::Sal(value) if value.name == annotation)
            }));
        }
    }
    let imports = [
        "Checked",
        "Inspected",
        "Combined",
        "Successful",
        "ReadStatus",
        "Outputs",
        "Scoped",
        "Conditional",
        "OldVersion",
        "UnexpectedArguments",
        "ExpandedArguments",
    ]
    .into_iter()
    .map(|name| {
        (
            name.into(),
            projection::Import {
                library: "fixture.dll".into(),
                name: None,
            },
        )
    })
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let text = &output.partitions["result_use"].rdl;
    assert_eq!(text.matches("#[must_use]").count(), 14);
    let golden = test_clang::input_dir().join("result_use.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        text,
        &std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-result-use.winmd");
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
            for owner in ["Apis", "IResults"] {
                for method in index.expect("Fixture", owner).methods() {
                    let expected = matches!(
                        method.name(),
                        "Checked" | "Inspected" | "Combined" | "Successful" | "ReadStatus"
                    );
                    assert_eq!(
                        method.find_attribute("MustUseAttribute").is_some(),
                        expected
                    );
                    if method.name() == "Outputs" {
                        let parameter = method.params().next().unwrap();
                        assert!(parameter.find_attribute("NativeSalAttribute").is_some());
                        assert!(parameter.find_attribute("MustUseAttribute").is_none());
                        assert_eq!(parameter.direction(), reader::ParamDirection::Output);
                    }
                }
            }
            for name in [
                "CheckedCallback",
                "InspectedCallback",
                "CheckedAlias",
                "CombinedAlias",
                "SuccessCallback",
                "SuccessAlias",
            ] {
                let callback = index.expect("Fixture", name);
                assert_eq!(
                    callback
                        .attributes()
                        .filter(|attribute| attribute.name() == "MustUseAttribute")
                        .count(),
                    1
                );
            }
            for name in ["Status", "InspectedValue"] {
                let ty = index.expect("Fixture", name);
                assert!(
                    ty.find_attribute(if name == "Status" {
                        "SuccessConditionAttribute"
                    } else {
                        "NativeSalAttribute"
                    })
                    .is_some()
                );
                assert!(ty.find_attribute("MustUseAttribute").is_none());
            }
            for field in index
                .expect("Fixture", "InspectedField")
                .fields()
                .chain(index.expect("Fixture", "Apis").fields())
            {
                assert!(field.find_attribute("NativeSalAttribute").is_some());
                assert!(field.find_attribute("MustUseAttribute").is_none());
            }
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn result_use_does_not_weaken_native_agreement() {
    helpers::ensure_libclang();
    for (name, from, to) in [
        ("Checked", "CHECK int Checked()", "INSPECT int Checked()"),
        (
            "CheckedCallback",
            "typedef CHECK int (__stdcall *CheckedCallback)",
            "typedef INSPECT int (__stdcall *CheckedCallback)",
        ),
        (
            "IResults",
            "virtual CHECK int __stdcall Checked()",
            "virtual INSPECT int __stdcall Checked()",
        ),
    ] {
        let first = test_clang::selected_fixture("result_use", &[name]);
        let mut second = test_clang::selected_fixture("result_use", &[name]);
        second.name = "other.h".into();
        second.source = second.source.replace(from, to);
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
        assert!(error.to_string().contains(name), "{error}");
    }
}
