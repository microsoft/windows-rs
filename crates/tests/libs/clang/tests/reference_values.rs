use test_clang::selected_fixture;
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn shared_values_have_one_checked_owner() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-reference-owned-values");
    std::fs::create_dir_all(&scratch).unwrap();
    let imports = projection::Imports::from_iter([(
        "SharedFunction".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]);
    let snapshots = capture(
        &[selected_fixture(
            "reference_owned_values",
            &["SharedConstant", "SharedFunction"],
        )],
        &TARGETS,
    )
    .unwrap();
    let producer =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let path = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_texts(producer.partitions.values().map(|partition| &partition.rdl))
        .output(&path)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&path).unwrap();
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        imports: Some(&imports),
        ..Default::default()
    };
    for prefix in [
        "",
        "#define SHARED_CONSTANT 8\n",
        "#define RESULT_TYPE unsigned int\n",
    ] {
        let mut input = selected_fixture("reference_owned_values", &[]);
        input.source.insert_str(0, prefix);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let strict =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options);
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
            &options,
        )
        .unwrap();
        assert_eq!(strict.is_ok(), prefix.is_empty());
        assert_eq!(partial.omissions.is_empty(), prefix.is_empty());
        assert_eq!(partial.output.records, 1);
        assert_eq!(partial.output.functions, 0);
        assert_eq!(partial.output.constants, 0);
    }
}

#[test]
fn shared_enum_extensions_preserve_existing_values() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-reference-enums");
    std::fs::create_dir_all(&scratch).unwrap();
    let producer = projection::generate(
        &[selected_fixture("reference_enums", &["SharedKind"])],
        "Fixture",
    )
    .unwrap();
    let reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_texts(producer.partitions.values().map(|partition| &partition.rdl))
        .output(&reference)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&reference).unwrap();
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    for changed in [false, true] {
        let mut input = selected_fixture("reference_enums", &["EnumConsumer", "EnumIndependent"]);
        input.source.insert_str(0, "#define EXTRA_MEMBER\n");
        if changed {
            input.source.insert_str(0, "#define SHARED_VALUE 3\n");
        }
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let strict =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options);
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
            &options,
        )
        .unwrap();
        if changed {
            assert!(strict.is_err());
            assert_eq!(partial.output.records, 1);
            assert_eq!(partial.output.enums, 0);
            assert!(
                partial
                    .omissions
                    .iter()
                    .any(|reason| reason.contains("SharedFirst"))
            );
        } else {
            assert_eq!(strict.unwrap(), partial.output);
            assert_eq!(partial.output.enums, 1);
            assert!(partial.omissions.is_empty());
            let extension = scratch.join("extension.winmd");
            windows_rdl::reader()
                .input_texts(
                    partial
                        .output
                        .partitions
                        .values()
                        .map(|partition| &partition.rdl),
                )
                .reference(&reference)
                .output(&extension)
                .write()
                .unwrap();
            let merged = scratch.join("merged.winmd");
            windows_metadata::merge()
                .input(&reference)
                .input(&extension)
                .union_enums()
                .output(&merged)
                .merge()
                .unwrap();
            let index = windows_metadata::reader::Index::read(&merged).unwrap();
            let members: Vec<_> = index
                .expect("Fixture", "SharedKind")
                .fields()
                .filter(|field| field.constant().is_some())
                .map(|field| field.name())
                .collect();
            assert_eq!(members, ["SharedFirst", "SharedSecond"]);
        }
    }
}

#[test]
fn incompatible_shared_records_are_not_reused() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-reference-values");
    std::fs::create_dir_all(&scratch).unwrap();
    let producer = projection::generate(
        &[selected_fixture(
            "reference_values",
            &[
                "SharedValue",
                "AliasRecord",
                "PointerAlias",
                "DoublePointer",
            ],
        )],
        "External",
    )
    .unwrap();
    let reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_texts(producer.partitions.values().map(|partition| &partition.rdl))
        .output(&reference)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&reference).unwrap();
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    let snapshots = capture(
        &[selected_fixture("reference_values", &["PointerConsumer"])],
        &TARGETS,
    )
    .unwrap();
    let pointers =
        projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options)
            .unwrap();
    assert_eq!(pointers.records, 1);
    assert_eq!(pointers.aliases, 0);
    for prefix in [
        "",
        "#define FIELD_TYPE unsigned int\n",
        "#define FIELD_NAME other\n",
        "#define EXTRA_FIELD\n",
        "#define PACKING 1\n",
        "#define NESTED_FIELD\n",
        "#define SHARED_ALIAS\n",
    ] {
        let mut input =
            selected_fixture("reference_values", &["ValueConsumer", "IndependentValue"]);
        input.source.insert_str(0, prefix);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let strict =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options);
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Consumer",
            &options,
        )
        .unwrap();
        if prefix.is_empty() {
            assert_eq!(strict.unwrap(), partial.output);
            assert_eq!(partial.output.records, 2);
            assert!(partial.omissions.is_empty());
        } else {
            assert!(strict.is_err(), "{prefix}");
            assert_eq!(partial.output.records, 1, "{prefix}");
            assert!(
                partial
                    .omissions
                    .iter()
                    .any(|reason| reason.contains("SharedValue")),
                "{prefix}: {:?}",
                partial.omissions,
            );
            assert!(
                partial
                    .output
                    .partitions
                    .values()
                    .all(|partition| !partition.rdl.contains("ValueConsumer")),
                "{prefix}",
            );
        }
    }
}

#[test]
fn records_cannot_reuse_metadata_enums() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang-reference-kinds");
    std::fs::create_dir_all(&scratch).unwrap();
    let reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_text("#[win32] mod External { #[repr(i32)] enum SharedValue { First = 0, } }")
        .output(&reference)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&reference).unwrap();
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    let snapshots = capture(
        &[selected_fixture(
            "reference_values",
            &["ValueConsumer", "IndependentValue"],
        )],
        &TARGETS,
    )
    .unwrap();
    assert!(
        projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options)
            .unwrap_err()
            .to_string()
            .contains("does not reference a record")
    );
    let partial = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Consumer",
        &options,
    )
    .unwrap();
    assert_eq!(partial.output.records, 1);
    assert!(
        partial
            .omissions
            .iter()
            .any(|reason| reason.contains("SharedValue"))
    );
}
