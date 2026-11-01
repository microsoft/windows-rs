use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{Type, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn referenced_types_keep_their_metadata_identity() {
    helpers::ensure_libclang();
    for namespace in ["Arbitrary.Types", "Windows.Win32"] {
        roundtrip(namespace);
    }
}

fn roundtrip(namespace: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root
        .join("target\\test-clang-reference-types")
        .join(namespace);
    std::fs::create_dir_all(&scratch).unwrap();
    let producer =
        projection::generate(&[selected_fixture("reference_types", &[])], namespace).unwrap();
    let reference = scratch.join("producer.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(producer.partitions.values().map(|part| &part.rdl))
        .output(&reference)
        .write()
        .unwrap();
    let metadata = reader::Index::read(&reference).unwrap();
    let references = projection::References::new(&metadata);
    let snapshots = capture(
        &[selected_fixture(
            "reference_consumer",
            &[
                "Consumer",
                "Consume",
                "LocalResult",
                "LocalCompletion",
                "LOCAL_FAILURE",
            ],
        )],
        &TARGETS,
    )
    .unwrap();
    let imports = [(
        "Consume".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let options = projection::Options {
        references: Some(&references),
        imports: Some(&imports),
    };
    let output =
        projection::project_with_options(&merge(&snapshots).unwrap(), "Component", &options)
            .unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    if namespace == "Arbitrary.Types" {
        let expected = input_dir().join("reference_consumer.rdl");
        if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&expected, &text).unwrap();
        }
        assert_eq!(
            text,
            std::fs::read_to_string(expected)
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    let consumer = scratch.join("consumer.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .reference(&reference)
        .output(&consumer)
        .write()
        .unwrap();
    for architecture in [1, 2, 4] {
        let index = reader::Index::new_for_architecture(
            vec![reader::File::read(&consumer).unwrap()],
            architecture,
        );
        for name in [
            "HRESULT",
            "Count",
            "CountAlias",
            "Text",
            "NativeSize",
            "State",
            "Payload",
            "VARIANT",
            "Completion",
            "Envelope",
        ] {
            assert!(!index.contains("Component", name), "{name}");
        }
        let fields: Vec<_> = index
            .expect("Component", "Consumer")
            .fields()
            .map(|field| field.ty())
            .collect();
        for (actual, name) in fields.iter().zip([
            "HRESULT",
            "CountAlias",
            "Text",
            "NativeSize",
            "State",
            "Payload",
            "VARIANT",
        ]) {
            assert_eq!(*actual, Type::value_named(namespace, name));
        }
        assert_eq!(fields[7], Type::class_named(namespace, "Completion"));
        assert_eq!(fields[10], Type::value_named(namespace, "Envelope/Inner"));
        for (local, external) in [("LocalResult", "HRESULT"), ("LocalCount", "CountAlias")] {
            assert_eq!(
                index.expect("Component", local).underlying_type(),
                Some(Type::value_named(namespace, external))
            );
        }
    }
    let partial = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Component",
        &options,
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    let local = projection::generate(
        &[selected_fixture("reference_consumer", &["Consumer"])],
        "Component",
    )
    .unwrap();
    assert!(
        local
            .partitions
            .values()
            .any(|part| part.rdl.contains("type HRESULT = i32;"))
    );

    let ambiguous = scratch.join("ambiguous.winmd");
    windows_rdl::reader()
        .input_text("#[win32] mod Other { type Count = u32; }")
        .output(&ambiguous)
        .write()
        .unwrap();
    for paths in [[&reference, &ambiguous], [&ambiguous, &reference]] {
        let metadata =
            reader::Index::new(paths.map(|path| reader::File::read(path).unwrap()).into());
        let references = projection::References::new(&metadata);
        // Unused duplicate names must not prevent other references from being used.
        let input = selected_fixture("reference_types", &["HRESULT"]);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        projection::project_with_options(
            &merge(&snapshots).unwrap(),
            "Component",
            &projection::Options {
                references: Some(&references),
                ..Default::default()
            },
        )
        .unwrap();
        let input = selected_fixture("reference_types", &["Count"]);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let error = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Component",
            &projection::Options {
                references: Some(&references),
                ..Default::default()
            },
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("ambiguous external metadata ownership for `Count`"),
            "{error}"
        );
    }
    let mut conflicting = selected_fixture("reference_types", &["Count"]);
    conflicting.name = "conflicting.h".into();
    conflicting.source = conflicting.source.replace(
        "typedef unsigned int Count;",
        "typedef unsigned long Count;",
    );
    let snapshots = capture(
        &[selected_fixture("reference_types", &["Count"]), conflicting],
        &TARGETS,
    )
    .unwrap();
    let error = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Component",
        &options,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("Count"), "{error}");
}
