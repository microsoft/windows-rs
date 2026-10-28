use windows_clang::*;
use windows_metadata::{HasAttributes, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn direction_contracts() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("direction")], &TARGETS).unwrap();
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
    assert_eq!(output, partial.output);
    check_output(&output, "direction", "Fixture");
}

fn check_output(output: &projection::Output, partition: &str, namespace: &str) {
    let text = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect::<String>();
    let golden = test_clang::input_dir().join(format!("{partition}.rdl"));
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang-{partition}.winmd"));
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter(namespace)
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
            let check =
                |owner, method, parameter, direction, optional, conditions| {
                    let method = index
                        .expect(namespace, owner)
                        .methods()
                        .find(|value| value.name() == method)
                        .unwrap();
                    let parameter = method
                        .params()
                        .find(|value| value.name() == parameter)
                        .unwrap();
                    assert_eq!(
                        parameter.direction(),
                        direction,
                        "{}::{}",
                        method.name(),
                        parameter.name()
                    );
                    assert_eq!(parameter.is_optional(), optional, "{}", method.name());
                    assert_eq!(
                        parameter
                            .attributes()
                            .filter(|value| value.name() == "NativeSalAttribute")
                            .filter(|value| value.value().iter().any(|(_, value)| *value
                                == windows_metadata::Value::Utf8("_When_".into())))
                            .count(),
                        conditions
                    );
                };
            use reader::ParamDirection::*;
            for (owner, method) in [
                ("Apis", "Unknown"),
                ("UnknownCallback", "Invoke"),
                ("IDirections", "Unknown"),
            ] {
                for name in [
                    "scalar",
                    "pointer",
                    "constant",
                    "alias",
                    "reference",
                    "object",
                ] {
                    check(owner, method, name, Unspecified, false, 0);
                }
            }
            for (name, direction, optional) in [
                ("input", Input, false),
                ("output", Output, false),
                ("both", InputOutput, false),
                ("object", Input, true),
            ] {
                check("Apis", "Known", name, direction, optional, 0);
            }
            for (owner, method) in [
                ("Apis", "Query"),
                ("QueryCallback", "Invoke"),
                ("IDirections", "Query"),
                ("Apis", "Reversed"),
            ] {
                check(owner, method, "size", Output, true, 2);
            }
            for name in ["MixedOptional", "Required"] {
                check("Apis", name, "size", Output, false, 2);
            }
            check("Apis", "Disjoint", "size", Unspecified, true, 2);
            check("Apis", "Partial", "size", Unspecified, false, 1);
            for name in [
                "NonComplementary",
                "DifferentSubjects",
                "IntegerSubject",
                "Complex",
                "UnknownBranch",
                "Scoped",
            ] {
                check("Apis", name, "size", Unspecified, false, 2);
            }
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn conditional_evidence_still_conflicts() {
    helpers::ensure_libclang();
    for replacement in ["data == 0, _In_opt_", "data == 0, _Out_"] {
        let first = test_clang::selected_fixture("direction", &["Query"]);
        let mut second = test_clang::selected_fixture("direction", &["Query"]);
        second.name = "other.h".into();
        second.source = second.source.replace("data == 0, _Out_opt_", replacement);
        let snapshots = capture(&[first, second], &TARGETS).unwrap();
        assert!(
            projection::project_partial_with_options(
                PartialCapture {
                    snapshots,
                    omissions: Vec::new()
                },
                "Fixture",
                &projection::Options::default(),
            )
            .is_err()
        );
    }
}
