use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn anonymous_enums() {
    helpers::ensure_libclang();
    roundtrip("anonymous-enums");
    shared_constants();
    rejections();
}

fn roundtrip(name: &str) {
    let snapshots = capture(&[selected_fixture("anonymous_enums", &[])], &TARGETS).unwrap();
    assert!(snapshots.iter().all(|snapshot| {
        snapshot
            .enums
            .iter()
            .flat_map(|enumeration| &enumeration.observations)
            .any(|observation| observation.anonymous)
    }));
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(
        (
            output.records,
            output.enums,
            output.aliases,
            output.constants
        ),
        (2, 1, 0, 9)
    );
    let rdl = &output.partitions["anonymous_enums"].rdl;
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("anonymous_enums.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join(format!("target\\test-clang-{name}.rdl"));
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    let rebuilt = root.join(format!("target\\test-clang-{name}-rebuilt.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rewritten)
        .output(&rebuilt)
        .write()
        .unwrap();
    for path in [winmd, rewritten, rebuilt] {
        std::fs::remove_file(path).unwrap();
    }
}

fn shared_constants() {
    let mut inputs = [
        selected_fixture("anonymous_enum_a", &[]),
        selected_fixture("anonymous_enum_b", &[]),
    ];
    for input in &mut inputs {
        input.source = format!("#include \"{}\"", input.name);
    }
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!(output.constants, 3);
    assert_eq!(
        output.partitions["shared"].rdl,
        "#[win32]\nmod Fixture {\n    const Common: i32 = 4;\n}\n"
    );
    assert_eq!(output.partitions["shared"].headers.len(), 2);
    assert!(
        output.partitions["anonymous_enum_a"]
            .rdl
            .contains("const OnlyA: i32 = 10;")
    );
    assert!(
        output.partitions["anonymous_enum_b"]
            .rdl
            .contains("const OnlyB: i32 = 20;")
    );
    inputs.reverse();
    inputs[0].name = "renamed.h".into();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    for prefix in ["#define VALUE 5\n", "#define BACKING long\n"] {
        let mut inputs = [
            selected_fixture("anonymous_enum_a", &[]),
            selected_fixture("anonymous_enum_b", &[]),
        ];
        inputs[1].source.insert_str(0, prefix);
        let error = projection::generate_partial(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("conflicting native definitions for constant `Common`"),
            "{error}"
        );
    }
}

fn rejections() {
    let output = projection::generate_partial(
        &[selected_fixture("anonymous_enum_rejections", &[])],
        "Fixture",
    )
    .unwrap();
    assert_eq!(
        (
            output.output.records,
            output.output.enums,
            output.output.aliases
        ),
        (1, 1, 2)
    );
    let rdl = &output.output.partitions["anonymous_enum_rejections"].rdl;
    assert!(rdl.contains("const Survives: i32 = 5;"), "{rdl}");
    for name in ["Aligned", "Attributed", "Neighbor"] {
        assert!(!rdl.contains(name), "{rdl}");
    }
    assert!(rdl.contains("const PointerValue: i32 = 4;"), "{rdl}");
    assert!(
        rdl.contains("enum AliasA {\n        AmbiguousValue = 6,"),
        "{rdl}"
    );
    assert!(rdl.contains("type AliasB = AliasA;"), "{rdl}");
    assert_eq!(output.output.constants, 2);
    assert!(rdl.contains("type EnumPointer = *mut i32;"), "{rdl}");
    assert!(rdl.contains("value: EnumPointer,"), "{rdl}");
}
