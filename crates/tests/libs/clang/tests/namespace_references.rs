use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn authoritative_namespace_references() {
    helpers::ensure_libclang();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang-namespace-references");
    std::fs::create_dir_all(&scratch).unwrap();
    let reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input(input_dir().join("namespace_reference_metadata.rdl"))
        .output(&reference)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&reference).unwrap();
    let references = projection::References {
        global_namespace: None,
        metadata: &metadata,
        native_prefixes: &["", "ABI"],
        delegate_prefix: Some("I"),
    };
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    for case in 0..11 {
        let mut input = selected_fixture(
            "namespace_reference",
            &["ReferenceConsumer", "ABI::External::BufferType"],
        );
        input.arguments.push(format!("-DCASE={case}"));
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        let result = projection::project_with_options(&merged, "Fixture", &options);
        if case != 0 {
            let error = result.unwrap_err().to_string();
            let expected = match case {
                1 | 7 => "UUID differs",
                2 | 8 => "parameter ABI differs",
                3 => "qualified storage",
                4 => "method mapping",
                5 => "requires a native pointer",
                6 => "ABI differs",
                9 => "method count differs",
                10 => "requires IUnknown",
                _ => unreachable!(),
            };
            assert!(error.contains(expected), "{case}: {error}");
            let partial = projection::project_partial_with_options(
                PartialCapture {
                    snapshots,
                    omissions: Vec::new(),
                },
                "Fixture",
                &options,
            )
            .unwrap_err()
            .to_string();
            assert!(partial.contains(expected), "{case}: {partial}");
            continue;
        }
        let output = result.unwrap();
        let actual = &output.partitions["namespace_reference"].rdl;
        let expected = input_dir().join("namespace_reference.rdl");
        if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&expected, actual).unwrap();
        }
        assert_eq!(
            *actual,
            std::fs::read_to_string(expected)
                .unwrap()
                .replace("\r\n", "\n")
        );
        let winmd = scratch.join("input.winmd");
        windows_rdl::reader()
            .input_text(actual)
            .reference(&reference)
            .output(&winmd)
            .write()
            .unwrap();
        let generated = windows_metadata::reader::Index::read(&winmd).unwrap();
        assert_eq!(generated.get("Fixture", "IBuffer").count(), 0);
        assert_eq!(generated.get("Fixture", "IHandler").count(), 0);
        assert_eq!(generated.get("Fixture", "BufferType").count(), 1);
        assert_eq!(generated.get("Fixture", "LocalHelper").count(), 1);
        let consumer = generated
            .get("Fixture", "ReferenceConsumer")
            .next()
            .unwrap();
        assert_eq!(
            consumer.fields().next().unwrap().ty(),
            windows_metadata::Type::class_named("External", "IBuffer")
        );
        assert_eq!(
            consumer.fields().last().unwrap().ty(),
            windows_metadata::Type::class_named("External", "Handler")
        );
        let rewritten = scratch.join("rewritten.rdl");
        windows_rdl::writer()
            .input(&winmd)
            .filter("Fixture")
            .output(&rewritten)
            .write()
            .unwrap();
        assert_eq!(*actual, std::fs::read_to_string(rewritten).unwrap());
        let local = projection::project(&merged, "Fixture").unwrap();
        assert!(
            local.partitions["namespace_reference"]
                .rdl
                .contains("interface BufferType")
        );
        let mut collision =
            selected_fixture("namespace_reference", &["ReferenceConsumer", "IBuffer"]);
        collision
            .source
            .push_str("\nstruct IBuffer { int value; };\n");
        let collisions = capture(&[collision], &TARGETS).unwrap();
        let error =
            projection::project_with_options(&merge(&collisions).unwrap(), "External", &options)
                .unwrap_err()
                .to_string();
        assert!(error.contains("collides with external metadata"), "{error}");
        let unsupported_path = scratch.join("unsupported.winmd");
        windows_rdl::reader()
            .input(input_dir().join("namespace_reference_unsupported.rdl"))
            .output(&unsupported_path)
            .write()
            .unwrap();
        let unsupported = windows_metadata::reader::Index::read(&unsupported_path).unwrap();
        let references = projection::References {
            global_namespace: None,
            metadata: &unsupported,
            native_prefixes: &["ABI"],
            delegate_prefix: Some("I"),
        };
        let options = projection::Options {
            references: Some(&references),
            ..Default::default()
        };
        let error = projection::project_with_options(&merged, "Fixture", &options)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("unsupported external metadata contract"),
            "{error}"
        );
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
            &options,
        )
        .unwrap();
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains("unsupported external metadata contract"))
        );
        for partition in partial.output.partitions.values() {
            assert!(!partition.rdl.contains("IBuffer"));
            assert!(!partition.rdl.contains("ReferenceConsumer"));
        }
    }
}
