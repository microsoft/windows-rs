use windows_clang2::*;
use windows_metadata::{HasAttributes, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_input_parameter_kinds() {
    helpers::ensure_libclang();
    for read_only in [true, false] {
        check_input(read_only);
    }
}

fn check_input(read_only: bool) {
    let mut input = test_clang2::fixture("native_input");
    if !read_only {
        input.arguments.push("-DRPC_INPUT".into());
    }
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let observation = &snapshot
            .values
            .iter()
            .find(|value| value.name == "Inputs")
            .unwrap()
            .observations[0];
        let ValueKind::Function { parameters, .. } = &observation.kind else {
            panic!();
        };
        let TypeKind::Function(function) = &snapshot.types[observation.canonical.0].kind else {
            panic!();
        };
        assert_eq!(parameters.len(), 19);
        for (position, (parameter, ty)) in parameters.iter().zip(&function.parameters).enumerate() {
            assert!(
                parameter
                    .attributes
                    .iter()
                    .any(|attribute| matches!(attribute.kind, AttributeKind::Sal(_)))
            );
            assert_eq!(
                matches!(
                    snapshot.types[ty.0].kind,
                    TypeKind::Scalar(_) | TypeKind::Enum(_)
                ),
                position < 12,
                "{}",
                parameter.name
            );
        }
    }
    let imports = ["Inputs", "ScalarConflict", "EnumConflict"]
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
    let text = &output.partitions["native_input"].rdl;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-native-input.winmd");
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
            for (owner, name) in [
                ("Apis", "Inputs"),
                ("InputCallback", "Invoke"),
                ("IInputs", "Apply"),
            ] {
                let method = index
                    .expect("Fixture", owner)
                    .methods()
                    .find(|method| method.name() == name)
                    .unwrap();
                let parameters: Vec<_> = method.params().collect();
                assert_eq!(parameters.len(), 19);
                for (position, parameter) in parameters.iter().enumerate() {
                    assert_eq!(parameter.direction(), reader::ParamDirection::Input);
                    assert_eq!(parameter.is_optional(), position == 13);
                    let attribute = parameter.find_attribute("NativeInputAttribute");
                    assert_eq!(
                        attribute.is_some(),
                        position >= 12,
                        "{owner}::{name} parameter {position}"
                    );
                    if let Some(attribute) = attribute {
                        assert_eq!(
                            attribute.value(),
                            vec![("ReadOnly".into(), Value::Bool(read_only))]
                        );
                    }
                    assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                }
            }
        }
    }
    let golden = test_clang2::input_dir().join(if read_only {
        "native_input.rdl"
    } else {
        "native_input_rpc.rdl"
    });
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        text,
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
fn omitted_input_metadata_does_not_hide_native_conflicts() {
    helpers::ensure_libclang();
    for name in ["ScalarConflict", "EnumConflict"] {
        for annotation in ["WRITE", "RPC_READ"] {
            let first = test_clang2::selected_fixture("native_input", &[name]);
            let mut second = test_clang2::selected_fixture("native_input", &[name]);
            second.name = "other.h".into();
            second.source = second
                .source
                .replace(&format!("{name}(READ "), &format!("{name}({annotation} "));
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
            assert!(error.to_string().contains("merge"), "{error}");
        }
    }
}
