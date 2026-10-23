use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

#[test]
fn referenced_interface_typedefs_are_nominal() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-native-reference-aliases");
    std::fs::create_dir_all(&scratch).unwrap();
    let path = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input(input_dir().join("native_reference_metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&path).unwrap();
    let references = projection::References {
        metadata: &metadata,
        native_prefixes: &[],
        delegate_prefix: None,
    };
    let imports = [(
        "Create".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let options = projection::Options {
        imports: Some(&imports),
        references: Some(&references),
        ..Default::default()
    };
    for (define, error) in [
        ("-DCASE=0", None),
        (
            "-DBY_VALUE",
            Some("by-value C++ inheritance requires a calling-ABI contract"),
        ),
        (
            "-DBY_VALUE=2",
            Some("interface values require a native pointer"),
        ),
        (
            "-DANNOTATED_ALIAS",
            Some("interface values require a native pointer"),
        ),
        ("-DCASE=1", Some("UUID differs")),
        ("-DCASE=2", Some("base differs")),
        ("-DCASE=3", Some("non-contiguous virtual method slots")),
    ] {
        let mut input = selected_fixture(
            "native_reference_aliases",
            &["Local", "IForeign", "Holder", "Create"],
        );
        input.arguments.push(define.into());
        input.external_interfaces = references.native_interfaces().unwrap();
        let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
        for snapshot in &snapshots {
            assert!(snapshot.aliases.iter().any(|alias| {
                alias.name == "IForeign"
                    && alias
                        .observations
                        .iter()
                        .any(|observation| !observation.implicit)
            }));
        }
        let strict =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options);
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
            &options,
        );
        if let Some(expected) = error {
            let error = strict.unwrap_err().to_string();
            assert!(error.contains(expected), "{define}: {error}");
            match partial {
                Err(error) => assert!(error.to_string().contains(expected), "{error}"),
                Ok(partial) => {
                    assert!(
                        partial
                            .omissions
                            .iter()
                            .any(|error| error.contains(expected))
                    );
                    if define.starts_with("-DBY_VALUE") {
                        assert_eq!(partial.output.functions, 0);
                        assert!(
                            partial
                                .omissions
                                .iter()
                                .all(|error| !error.starts_with("Alias("))
                        );
                    } else if define == "-DANNOTATED_ALIAS" {
                        assert_eq!(partial.output.functions, 0);
                        assert!(
                            partial
                                .omissions
                                .iter()
                                .any(|error| error.starts_with("Alias("))
                        );
                    } else {
                        assert_eq!(partial.output.interfaces, 0);
                    }
                }
            }
            continue;
        }
        let output = strict.unwrap();
        let partial = partial.unwrap();
        assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
        assert_eq!(output, partial.output);
        assert_eq!(
            (output.interfaces, output.records, output.functions),
            (1, 1, 1)
        );
        let actual: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        let golden = input_dir().join("native_reference_aliases.rdl");
        if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&golden, &actual).unwrap();
        }
        assert_eq!(
            actual,
            std::fs::read_to_string(golden)
                .unwrap()
                .replace("\r\n", "\n")
        );
        let first = scratch.join("first.winmd");
        let rdl = scratch.join("roundtrip.rdl");
        let second = scratch.join("second.winmd");
        windows_rdl::reader()
            .input_text(&actual)
            .reference(&path)
            .output(&first)
            .write()
            .unwrap();
        windows_rdl::writer()
            .input(&first)
            .reference(&path)
            .output(&rdl)
            .write()
            .unwrap();
        windows_rdl::reader()
            .input(&rdl)
            .reference(&path)
            .output(&second)
            .write()
            .unwrap();
        for output in [first, second] {
            let metadata = windows_metadata::reader::Index::read(output).unwrap();
            let expected = windows_metadata::Type::class_named("External.Native", "IForeign");
            assert_eq!(
                metadata
                    .expect("Fixture", "Holder")
                    .fields()
                    .next()
                    .unwrap()
                    .ty(),
                expected
            );
            assert_eq!(
                metadata
                    .expect("Fixture", "Local")
                    .methods()
                    .next()
                    .unwrap()
                    .signature(&[])
                    .types,
                [expected]
            );
            assert!(!metadata.contains("External.Native", "IForeign"));
        }
    }
}

#[test]
fn native_definition_ownership() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-native-reference-ownership");
    std::fs::create_dir_all(&scratch).unwrap();
    let path = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input(input_dir().join("native_reference_metadata.rdl"))
        .output(&path)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&path).unwrap();
    let references = projection::References {
        metadata: &metadata,
        native_prefixes: &[],
        delegate_prefix: None,
    };
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    for case in 0..5 {
        let mut input = selected_fixture("native_reference", &["Local"]);
        input.arguments.push(format!("-DCASE={case}"));
        if case != 4 {
            input.external_interfaces = references.native_interfaces().unwrap();
        }
        let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
        let output =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options);
        if case == 0 {
            let output = output.unwrap();
            assert_eq!(output.interfaces, 1);
            assert_eq!(output.partitions.len(), 1);
            let actual = &output.partitions["native_reference"].rdl;
            let expected = input_dir().join("native_reference.rdl");
            if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
                std::fs::write(&expected, actual).unwrap();
            }
            assert_eq!(
                *actual,
                std::fs::read_to_string(expected)
                    .unwrap()
                    .replace("\r\n", "\n")
            );
            let generated = scratch.join("generated.rdl");
            std::fs::write(&generated, actual).unwrap();
            windows_rdl::reader()
                .input(&generated)
                .reference(&path)
                .output(scratch.join("generated.winmd"))
                .write()
                .unwrap();
        } else {
            let expected = match case {
                1 => "UUID differs",
                2 => "base differs",
                3 => "non-contiguous virtual method slots",
                4 => "unsupported external metadata contract",
                _ => unreachable!(),
            };
            let error = output.unwrap_err().to_string();
            assert!(error.contains(expected), "{case}: {error}");
            let partial = projection::project_partial_with_options(
                PartialCapture {
                    snapshots,
                    omissions: Vec::new(),
                },
                "Fixture",
                &options,
            );
            if case <= 2 {
                assert!(partial.unwrap_err().to_string().contains(expected));
            } else {
                let partial = partial.unwrap();
                assert!(partial.output.partitions.is_empty());
                assert!(
                    partial
                        .omissions
                        .iter()
                        .any(|error| error.contains(expected))
                );
            }
        }
    }
    let ambiguous = scratch.join("ambiguous.winmd");
    windows_rdl::reader()
        .input(input_dir().join("native_reference_ambiguous.rdl"))
        .output(&ambiguous)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::new(vec![
        windows_metadata::reader::File::read(&path).unwrap(),
        windows_metadata::reader::File::read(&ambiguous).unwrap(),
    ]);
    let references = projection::References {
        metadata: &metadata,
        ..references
    };
    let mut input = selected_fixture("native_reference", &["Local"]);
    input.external_interfaces = references.native_interfaces().unwrap();
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let error = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &projection::Options {
            references: Some(&references),
            ..Default::default()
        },
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("ambiguous external metadata ownership"),
        "{error}"
    );
}
