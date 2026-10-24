use test_clang::selected_fixture;
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn callback_references_check_abi_and_keep_metadata_contracts() {
    helpers::ensure_libclang();
    let input = "#define INPUT __attribute__((annotate(\"windows_clang:sal:2:_In_\\n()\\n()\")))\n";
    let output =
        "#define INPUT __attribute__((annotate(\"windows_clang:sal:2:_Out_\\n()\\n()\")))\n";
    let optional =
        "#define INPUT __attribute__((annotate(\"windows_clang:sal:2:_In_opt_\\n()\\n()\")))\n";
    for (case, producer_prefix, consumer_prefix, compatible) in [
        ("direction", "", input, true),
        (
            "renamed",
            input,
            "#define PARAMETER other\n#define INPUT __attribute__((annotate(\"windows_clang:sal:2:_In_\\n()\\n()\")))\n",
            true,
        ),
        ("opposite", input, output, true),
        (
            "inout",
            input,
            "#define INPUT __attribute__((annotate(\"windows_clang:sal:2:_Inout_\\n()\\n()\")))\n",
            true,
        ),
        (
            "complex",
            "",
            "#define INPUT __attribute__((annotate(\"windows_clang:sal:2:_Out_writes_\\n(2)\\n(2)\")))\n",
            true,
        ),
        ("missing-direction", input, "", true),
        ("optional", "", optional, true),
        ("required", optional, input, true),
        ("convention", "", "#define CALLBACK_ABI __cdecl\n", false),
        ("pointee", "", "#define PARAMETER_TYPE const int*\n", false),
        ("arity", "", "#define EXTRA_PARAMETER , int second\n", false),
        (
            "result",
            "",
            "#define CALLBACK_RESULT unsigned int\n",
            false,
        ),
    ] {
        let mut producer = selected_fixture("reference_callback", &["SharedCallback"]);
        producer.source.insert_str(0, producer_prefix);
        let metadata = reference(&format!("callback-{case}"), producer);
        let references = projection::References::new(&metadata);
        let options = projection::Options {
            references: Some(&references),
            ..Default::default()
        };
        let mut consumer = selected_fixture(
            "reference_callback",
            &["CallbackConsumer", "CallbackIndependent"],
        );
        consumer.source.insert_str(0, consumer_prefix);
        let snapshots = capture(&[consumer], &TARGETS).unwrap();
        let strict =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options);
        assert_eq!(strict.is_ok(), compatible, "{case}: {strict:?}");
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Consumer",
            &options,
        )
        .unwrap();
        assert_eq!(
            partial.output.records,
            if compatible { 2 } else { 1 },
            "{case}"
        );
        assert_eq!(partial.omissions.is_empty(), compatible, "{case}");
        if compatible {
            let rdl: String = partial
                .output
                .partitions
                .values()
                .map(|part| part.rdl.as_str())
                .collect();
            assert!(rdl.contains("External::SharedCallback"), "{case}");
            assert!(!rdl.contains("extern fn SharedCallback"), "{case}");
        }
    }
}

fn reference(name: &str, input: Input) -> windows_metadata::reader::Index {
    let output = projection::generate(&[input], "External").unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join(format!("target\\test-clang-reference-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(&path)
        .write()
        .unwrap();
    windows_metadata::reader::Index::read(path).unwrap()
}

#[test]
fn secondary_aliases_do_not_replace_canonical_ownership() {
    helpers::ensure_libclang();
    for incompatible in [false, true] {
        let mut producer = selected_fixture(
            "reference_alias_owners",
            &["Canonical", "Alternate", "PAlternate"],
        );
        producer.source.insert_str(0, "#define DISTINCT_ALIAS\n");
        if incompatible {
            producer
                .source
                .insert_str(0, "#define ALTERNATE_TYPE unsigned int\n");
        }
        let metadata = reference(&format!("alias-owner-{incompatible}"), producer);
        let references = projection::References::new(&metadata);
        let options = projection::Options {
            references: Some(&references),
            ..Default::default()
        };
        let snapshots = capture(
            &[selected_fixture(
                "reference_alias_owners",
                &["CanonicalConsumer", "AlternateConsumer"],
            )],
            &TARGETS,
        )
        .unwrap();
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
        assert_eq!(strict.is_ok(), !incompatible);
        assert_eq!(partial.output.records, if incompatible { 1 } else { 2 });
        let rdl = partial
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect::<String>();
        assert!(rdl.contains("struct CanonicalConsumer"));
        assert!(rdl.contains("External::Canonical"));
        assert_eq!(rdl.contains("struct AlternateConsumer"), !incompatible);
        if incompatible {
            assert!(
                partial
                    .omissions
                    .iter()
                    .any(|error| error.starts_with("Alias(\"Alternate\")"))
            );
            assert!(
                !partial
                    .omissions
                    .iter()
                    .any(|error| error.starts_with("Tag(\"Canonical\")"))
            );
        } else {
            assert!(partial.omissions.is_empty());
        }
    }
}

#[test]
fn ambiguous_aliases_do_not_choose_an_owner() {
    helpers::ensure_libclang();
    let mut producer = selected_fixture(
        "reference_alias_owners",
        &["Canonical", "Alternate", "PAlternate"],
    );
    producer.source.insert_str(0, "#define DISTINCT_ALIAS\n");
    let metadata = reference("ambiguous-owner", producer);
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    let mut consumer = selected_fixture(
        "reference_alias_owners",
        &["CanonicalConsumer", "AlternateConsumer"],
    );
    consumer.source.insert_str(0, "#define UNOWNED_CANONICAL\n");
    let captured = capture_partial(&[consumer], &TARGETS).unwrap();
    let partial = projection::project_partial_with_options(captured, "Consumer", &options).unwrap();
    assert_eq!(partial.output.records, 0);
    assert!(
        partial
            .omissions
            .iter()
            .any(|reason| reason.contains("ambiguous alias ownership"))
    );
}

#[test]
fn referenced_records_keep_their_owners_contracts() {
    helpers::ensure_libclang();
    let capacity = "#define BUFFER_CONTRACT __attribute__((annotate(\"windows_clang:sal:2:_Field_size_\\n(capacity)\\n(capacity)\")))\n";
    let length = "#define BUFFER_CONTRACT __attribute__((annotate(\"windows_clang:sal:2:_Field_size_\\n(length)\\n(length)\")))\n";
    for (case, producer_prefix, consumer_prefix) in [
        ("additional", "", capacity),
        ("matching", capacity, capacity),
        ("conflicting", capacity, length),
        ("missing", capacity, ""),
        ("typedef", "#define BUFFER_TYPE Terminated\n", ""),
        ("native-typedef", "", "#define BUFFER_TYPE Terminated\n"),
        (
            "typedef-matching",
            "#define BUFFER_TYPE Terminated\n",
            "#define BUFFER_TYPE Terminated\n",
        ),
        (
            "typedef-missing-contract",
            "#define BUFFER_TYPE Terminated\n",
            "#define BUFFER_TYPE Terminated\n#define TERMINATION\n",
        ),
    ] {
        let mut producer = selected_fixture("reference_contracts", &["Counted"]);
        producer.source.insert_str(0, producer_prefix);
        let metadata = reference(case, producer);
        let references = projection::References::new(&metadata);
        let options = projection::Options {
            references: Some(&references),
            ..Default::default()
        };
        let mut consumer = selected_fixture(
            "reference_contracts",
            &["ContractConsumer", "ContractIndependent"],
        );
        consumer.source.insert_str(0, consumer_prefix);
        let snapshots = capture(&[consumer], &TARGETS).unwrap();
        let strict =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options);
        let strict = strict.unwrap();
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Consumer",
            &options,
        )
        .unwrap();
        assert_eq!(partial.output.records, 2, "{case}");
        assert_eq!(partial.output, strict, "{case}");
        assert!(
            partial.omissions.is_empty(),
            "{case}: {:?}",
            partial.omissions
        );
        let rdl: String = partial
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(!rdl.contains("struct Counted"), "{case}");
        assert!(rdl.contains("External::Counted"), "{case}");
    }
}
