use test_clang::{annotated_sdk_fixture, fixture, input_dir};
use windows_clang::*;
use windows_metadata::reader;

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_string_references_own_contracts_without_changing_native_evidence() {
    helpers::ensure_libclang();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let targets = [Target::X86, Target::X64, Target::Arm64];
    let names = [
        "sdk_unicode_string",
        "sdk_counted_unicode_string",
        "sdk_lsa_unicode_string",
    ];
    let cases = names.map(|name| {
        let source = fixture(name);
        let mut input = annotated_sdk_fixture();
        input.name = source.name;
        input.source = source.source;
        input.roots = vec!["UNICODE_STRING".into()];
        let snapshots = capture(std::slice::from_ref(&input), &targets).unwrap();
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        let text: String = output
            .partitions
            .values()
            .map(|partition| partition.rdl.as_str())
            .collect();
        let expected = input_dir().join(format!("{name}.rdl"));
        if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&expected, &text).unwrap();
        }
        assert_eq!(
            text,
            std::fs::read_to_string(expected)
                .unwrap()
                .replace("\r\n", "\n")
        );
        let path = root.join(format!("target\\test-clang-{name}.winmd"));
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&text)
            .output(&path)
            .write()
            .unwrap();
        input
            .source
            .push_str("\nstruct StringConsumer { UNICODE_STRING value; };\n");
        input.roots = vec!["StringConsumer".into()];
        (
            capture(&[input], &targets).unwrap(),
            reader::Index::read(path).unwrap(),
        )
    });

    for (left, right) in cases[0].0.iter().zip(&cases[1].0) {
        let layout = |snapshot: &Snapshot| {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == "_UNICODE_STRING")
                .unwrap();
            let definition = record
                .observations
                .iter()
                .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                .unwrap();
            (
                definition.layout,
                definition
                    .fields
                    .iter()
                    .map(|field| (field.name.clone(), field.offset_bits))
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(left.target, right.target);
        assert_eq!(layout(left), layout(right));
    }

    for (native_case, metadata_case) in [(0, 1), (1, 0), (1, 2)] {
        let native = &cases[native_case].0;
        let metadata = &cases[metadata_case].1;
        let references = projection::References::new(metadata);
        let output = projection::project_with_options(
            &merge(native).unwrap(),
            "Consumer",
            &projection::Options {
                references: Some(&references),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(output.records, 1);
        let path = root.join(format!(
            "target\\test-clang-string-consumer-{native_case}-{metadata_case}.winmd"
        ));
        windows_rdl::reader()
            .input_texts(output.partitions.values().map(|part| &part.rdl))
            .reference(root.join(format!("target\\test-clang-{}.winmd", names[metadata_case])))
            .output(&path)
            .write()
            .unwrap();
        let consumer = reader::Index::read(path).unwrap();
        assert!(!consumer.contains("Consumer", "UNICODE_STRING"));
        assert_eq!(
            consumer
                .expect("Consumer", "StringConsumer")
                .fields()
                .next()
                .unwrap()
                .ty(),
            windows_metadata::Type::value_named("Fixture", "UNICODE_STRING")
        );
    }

    let combined: Vec<_> = cases
        .into_iter()
        .take(2)
        .flat_map(|(snapshots, _)| snapshots)
        .collect();
    let error = merge(&combined).err().unwrap();
    assert!(error.to_string().contains("_UNICODE_STRING"), "{error}");
    assert!(error.to_string().contains("attributes differ"), "{error}");
}
