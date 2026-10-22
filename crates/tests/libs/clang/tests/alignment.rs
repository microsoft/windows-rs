use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{HasAttributes, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn alignment() {
    helpers::ensure_libclang();
    roundtrip("alignment", &[None]);
    conflicts();
    rejections();
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn alignment_cross_targets() {
    helpers::ensure_libclang();
    roundtrip("alignment-cross-targets", &TARGETS.map(Some));
}

fn roundtrip(name: &str, targets: &[Option<Target>]) {
    let input = selected_fixture("alignment_types", &[]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.records, 11);
    let rdl = &output.partitions["alignment_types"].rdl;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        for (name, expected) in [
            ("Standard", 16),
            ("Declspec", 32),
            ("Gnu", 64),
            ("TypeAligned", 16),
            ("Multiple", 16),
            ("Unchanged", 4),
            ("AlignedUnion", 32),
            ("Recursive", 16),
            ("Platform", if bit == 1 { 16 } else { 32 }),
        ] {
            let attribute = index
                .expect("Fixture", name)
                .find_attribute("AlignmentAttribute")
                .unwrap();
            assert_eq!(attribute.value()[0].1, Value::I32(expected));
        }
        for name in ["Zero", "Container"] {
            assert!(
                index
                    .expect("Fixture", name)
                    .find_attribute("AlignmentAttribute")
                    .is_none()
            );
        }
    }
    let rewritten = root.join(format!("target\\test-clang-{name}.rdl"));
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(*rdl, std::fs::read_to_string(&rewritten).unwrap());
    let bindings = root.join(format!("target\\test-clang-{name}.rs"));
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    for snapshot in &snapshots {
        let target = match snapshot.target {
            Target::X86 => "x86",
            Target::X64 => "x86_64",
            Target::Arm64 => "aarch64",
        };
        rust.push_str(&format!(
            "\n#[cfg(target_arch = \"{target}\")] const _: () = {{\n"
        ));
        for record in &snapshot.records {
            let definition = snapshot.observations[record.observations[0].0]
                .definition
                .as_ref()
                .unwrap();
            rust.push_str(&format!("assert!(core::mem::size_of::<{}>() == {});\nassert!(core::mem::align_of::<{}>() == {});\n",
                record.name, definition.layout.size, record.name, definition.layout.alignment));
            for field in &definition.fields {
                rust.push_str(&format!(
                    "assert!(core::mem::offset_of!({}, {}) == {});\n",
                    record.name,
                    field.name,
                    field.offset_bits / 8
                ));
            }
        }
        rust.push_str("};\n");
    }
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join(format!("target\\test-clang-{name}.rmeta"));
    for target in targets {
        let mut command = std::process::Command::new("rustc");
        command.args([
            "--crate-name",
            "test_clang_alignment",
            "--crate-type=lib",
            "--emit=metadata",
            "--edition=2024",
        ]);
        if let Some(target) = target {
            command.args(["--target", target.triple()]);
        }
        let result = command
            .arg(&bindings)
            .arg("-o")
            .arg(&compiled)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{target:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for path in [winmd, rewritten, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("alignment_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
}

fn conflicts() {
    for (root, prefix, native) in [
        ("Probe", "#define REQUEST 8\n", true),
        ("Probe", "#define ALIGNMENT\n", true),
        (
            "Probe",
            "#define ALIGNMENT __attribute__((aligned(4)))\n",
            true,
        ),
        (
            "PublicProbe",
            "#define TAG Other\n#define REQUEST 8\n",
            false,
        ),
    ] {
        let mut inputs = [
            selected_fixture("alignment_conflict", &[root]),
            selected_fixture("alignment_conflict", &[root]),
        ];
        inputs[0].source = "#include \"alignment_conflict.h\"".into();
        inputs[1].name = "other.h".into();
        inputs[1].source = format!("{prefix}#include \"alignment_conflict.h\"");
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        for pair in snapshots.as_chunks::<2>().0 {
            assert_eq!(
                pair[0].observations[0].definition.as_ref().unwrap().layout,
                pair[1].observations[0].definition.as_ref().unwrap().layout
            );
        }
        if native {
            assert!(
                merge(&snapshots)
                    .err()
                    .unwrap()
                    .to_string()
                    .contains("differs")
            );
        } else {
            assert!(
                projection::project(&merge(&snapshots).unwrap(), "Fixture")
                    .unwrap_err()
                    .to_string()
                    .contains("incompatible native definitions")
            );
        }
    }
    let mut inputs = [
        selected_fixture("alignment_forward", &["Forward"]),
        selected_fixture("alignment_forward", &["Forward"]),
    ];
    inputs[0].source =
        "#define FORWARD\n#define REQUEST 4\n#include \"alignment_forward.h\"".into();
    inputs[1].name = "definition.h".into();
    inputs[1].source = "#include \"alignment_forward.h\"".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    assert!(
        merge(&snapshots)
            .err()
            .unwrap()
            .to_string()
            .contains("forward alignment")
    );
    inputs[0].source = "#define FORWARD\n#include \"alignment_forward.h\"".into();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap().records, 1);

    let mut inputs = [
        selected_fixture("alignment_conflict", &["Probe"]),
        selected_fixture("alignment_conflict", &["Probe"]),
    ];
    inputs[0].source = "#define ALIGNMENT __declspec(align(4)) __declspec(align(8))\n#include \"alignment_conflict.h\"".into();
    inputs[1].name = "reordered.h".into();
    inputs[1].source = "#define ALIGNMENT __declspec(align(8)) __declspec(align(4)) __declspec(align(8))\n#include \"alignment_conflict.h\"".into();
    let output = projection::generate(&inputs, "Fixture").unwrap();
    inputs.reverse();
    assert_eq!(
        output.partitions,
        projection::generate(&inputs, "Fixture").unwrap().partitions
    );
    assert_eq!(output.records, 1);
}

fn rejections() {
    let result =
        projection::generate_partial(&[selected_fixture("alignment_rejections", &[])], "Fixture")
            .unwrap();
    assert_eq!(result.output.records, 14);
    for name in [
        "PackedAligned",
        "PackedContainer",
        "PackedArray",
        "AlignedAlias",
        "AliasUser",
        "ConstAlignedAlias",
        "ConstAliasUser",
        "AlignedRecordAlias",
        "RecordAliasUser",
        "AlignedEnum",
        "EnumUser",
        "Dependent",
    ] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            result.omissions
        );
    }
}
