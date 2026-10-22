use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

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
