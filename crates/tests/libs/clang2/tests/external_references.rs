use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn external_definition_ownership() {
    helpers::ensure_libclang();
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang2-external-references");
    std::fs::create_dir_all(&scratch).unwrap();
    let reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input(input_dir().join("namespace_reference_metadata.rdl"))
        .output(&reference)
        .write()
        .unwrap();
    let metadata = windows_metadata::reader::Index::read(&reference).unwrap();
    let references = projection::References {
        metadata: &metadata,
        native_prefixes: &["", "ABI"],
        delegate_prefix: Some("I"),
    };
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    let input = |case| {
        let mut input = selected_fixture(
            "namespace_reference",
            &["ReferenceConsumer", "ABI::External::BufferType"],
        );
        input.source = format!("#define CASE {case}\n{}", input.source);
        input.external_interfaces = references.native_interfaces().unwrap();
        input
    };
    let mut expected = std::fs::read_to_string(input_dir().join("namespace_reference.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    for case in 0..11 {
        let snapshots = capture(&[input(case)], &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        let result = projection::project_with_options(&merged, "Fixture", &options);
        if matches!(case, 1 | 7 | 10) {
            let expected = if case == 10 {
                "requires IUnknown"
            } else {
                "UUID differs"
            };
            let error = result.unwrap_err().to_string();
            assert!(error.contains(expected), "{case}: {error}");
            let error = projection::project_partial_with_options(
                PartialCapture {
                    snapshots,
                    omissions: Vec::new(),
                },
                "Fixture",
                &options,
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains(expected), "{case}: {error}");
        } else {
            let result = result.unwrap();
            if case == 0 && std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
                expected = result.partitions["namespace_reference"].rdl.clone();
                std::fs::write(input_dir().join("namespace_reference.rdl"), &expected).unwrap();
            }
            assert_eq!(result.partitions["namespace_reference"].rdl, expected);
        }
    }
    let snapshots = capture(&[input(0)], &TARGETS).unwrap();
    let error = projection::project(&merge(&snapshots).unwrap(), "Fixture")
        .unwrap_err()
        .to_string();
    assert!(error.contains("requires authoritative metadata"), "{error}");

    let mut other = input(2);
    other.name = "other.h".into();
    let snapshots = capture(&[input(0), other], &TARGETS).unwrap();
    let output =
        projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options).unwrap();
    assert_eq!(output.partitions["foundation"].rdl, expected);

    for case in 0..2 {
        let mut input = selected_fixture("external_forward", &["ForwardConsumer"]);
        input.arguments.push(format!("-DCASE={case}"));
        input.external_interfaces = references.native_interfaces().unwrap();
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let expected = if case == 0 {
            "has no native definition"
        } else {
            "UUID differs"
        };
        let error =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options)
                .unwrap_err()
                .to_string();
        assert!(error.contains(expected), "{error}");
        let result = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
            &options,
        );
        if case == 0 {
            let result = result.unwrap();
            assert!(result.output.partitions.is_empty());
            assert!(
                result
                    .omissions
                    .iter()
                    .any(|message| message.contains(expected))
            );
        } else {
            assert!(result.unwrap_err().to_string().contains(expected));
        }
    }

    let mut attributed = input(0);
    attributed.source = attributed.source.replace(
        "typedef IBuffer IBuffer;",
        "typedef IBuffer IBuffer __attribute__((deprecated(\"external alias\")));",
    );
    let snapshots = capture(&[attributed], &TARGETS).unwrap();
    let error = projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options)
        .unwrap_err()
        .to_string();
    assert!(error.contains("attributed external alias"), "{error}");
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
            .any(|message| message.contains("attributed external alias"))
    );

    for conflict in 0..3 {
        let mut other = input(0);
        other.name = "other.h".into();
        let expected = match conflict {
            0 => {
                other.external_interfaces.clear();
                "ownership differs"
            }
            1 => {
                other.source = other.source.replace(
                    "905a0fef-bc53-11df-8c49-001e4fc686da",
                    "00000001-0000-0000-c000-000000000046",
                );
                "UUID differs"
            }
            _ => {
                other.source = other
                    .source
                    .replace("Access(unsigned char**", "Access(int**");
                "differ"
            }
        };
        let snapshots = capture(&[input(0), other], &TARGETS).unwrap();
        let error = match merge(&snapshots) {
            Ok(_) => panic!("conflicting evidence accepted"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "{conflict}: {error}");
    }

    for case in 0..3 {
        let mut input = input(0);
        let expected = match case {
            0 => {
                input.source = input
                    .source
                    .replace("= 0;", ";")
                    .replace("IBuffer* buffer", "IBuffer buffer");
                "interface values require a native pointer"
            }
            1 => {
                input.source = input.source.replace(
                    "ABI::External::IBuffer* buffer",
                    "volatile ABI::External::IBuffer* buffer",
                );
                "volatile"
            }
            _ => {
                input
                    .source
                    .push_str("\nstruct LocalDerived : ABI::External::IBuffer {};\n");
                input.roots.push("LocalDerived".into());
                "inheritance from externally owned interface"
            }
        };
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let error =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Fixture", &options)
                .unwrap_err()
                .to_string();
        assert!(error.contains(expected), "{case}: {error}");
    }
}
