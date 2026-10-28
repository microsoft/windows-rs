use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn native_initialization_policy() {
    helpers::ensure_libclang();
    let input = selected_fixture("initialization", &["Initialized", "Uninitialized"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let expected = match snapshot.target {
            Target::X86 => Layout {
                size: 8,
                alignment: 4,
            },
            _ => Layout {
                size: 16,
                alignment: 8,
            },
        };
        for record in &snapshot.records {
            let observation = &snapshot.observations[record.observations[0].0];
            assert_eq!(observation.definition.as_ref().unwrap().layout, expected);
            assert_eq!(
                observation
                    .attributes
                    .iter()
                    .any(|attribute| attribute.kind == AttributeKind::NoTrivialAutoVarInit),
                record.name == "Uninitialized"
            );
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.records, 2);
    let rdl = &output.partitions["initialization"].rdl;
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(input_dir().join("initialization.rdl"), rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("initialization.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn native_initialization_conflict() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("initialization", &["Uninitialized"]),
        selected_fixture("initialization", &["Uninitialized"]),
    ];
    inputs[0].source = "#include \"initialization.h\"".into();
    inputs[1].name = "other.h".into();
    inputs[1].source = "#define NO_INIT\n#include \"initialization.h\"".into();
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let error = merge(&snapshots).err().unwrap().to_string();
        assert!(error.contains("differs"), "{error}");
        inputs.reverse();
    }
}

#[test]
fn public_initialization_conflict() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("initialization", &["PublicInitialization"]),
        selected_fixture("initialization", &["PublicInitialization"]),
    ];
    inputs[0].source = "#include \"initialization.h\"".into();
    inputs[1].name = "other.h".into();
    inputs[1].source =
        "#define RECORD_NAME Other\n#define NO_INIT\n#include \"initialization.h\"".into();
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let error = projection::project(&merge(&snapshots).unwrap(), "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("incompatible native definitions"), "{error}");
        inputs.reverse();
    }
}

#[test]
fn forward_initialization_conflict() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("initialization", &["Uninitialized"]),
        selected_fixture("initialization", &["Uninitialized"]),
    ];
    inputs[0].name = "forward.h".into();
    inputs[0].source = "struct __declspec(no_init_all) Uninitialized;".into();
    inputs[1].source = "#define NO_INIT\n#include \"initialization.h\"".into();
    let error = projection::generate(&inputs, "Fixture")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("forward auto-initialization policy differs"),
        "{error}"
    );
    inputs[1].source = "#include \"initialization.h\"".into();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap().records, 1);
}
