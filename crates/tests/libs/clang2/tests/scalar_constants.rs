use windows_clang2::*;
use windows_metadata::{Value, reader, reader::HasAttributes};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input() -> Input {
    let mut input = test_clang2::selected_fixture(
        "scalar_constants",
        &[
            "ENABLED",
            "DISABLED",
            "FRACTION",
            "PRECISE",
            "NEGATIVE_ZERO",
            "NEGATIVE_ZERO64",
            "ROUNDED",
            "SMALLEST",
            "LARGEST",
            "FOLDED",
            "NATIVE_WIDTH",
        ],
    );
    input.macros = vec!["MACRO_SCALE".into(), "MACRO_FLAG".into()];
    input
}

#[test]
fn scalar_constants() {
    helpers::ensure_libclang();
    let input = input();
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let negative = snapshot
            .values
            .iter()
            .find(|value| value.name == "NEGATIVE_ZERO")
            .unwrap();
        assert!(matches!(
            negative.observations[0].kind,
            ValueKind::Constant {
                value: ConstantValue::Float32(0x80000000),
                ..
            }
        ));
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.constants, 13);
    let text = &output.partitions["scalar_constants"].rdl;
    assert_eq!(
        *text,
        std::fs::read_to_string(test_clang2::input_dir().join("scalar_constants.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-scalar-constants.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for bit in [1, 2, 4] {
            let index =
                reader::Index::new_for_architecture(vec![reader::File::read(path).unwrap()], bit);
            let apis = index.expect("Fixture", "Apis");
            let value = |name| {
                apis.fields()
                    .find(|field| {
                        field.name() == name && (field.arches() == 0 || field.arches() & bit != 0)
                    })
                    .unwrap()
                    .constant()
                    .unwrap()
                    .value()
            };
            assert_eq!(value("ENABLED"), Value::Bool(true));
            assert_eq!(value("DISABLED"), Value::Bool(false));
            assert_eq!(value("MACRO_FLAG"), Value::Bool(bit != 1));
            for (name, expected) in [
                ("FRACTION", 0.1f32),
                ("NEGATIVE_ZERO", -0.0),
                ("ROUNDED", 16777216.0),
                ("SMALLEST", f32::from_bits(1)),
                ("FOLDED", 0.25),
                ("MACRO_SCALE", 0.75),
            ] {
                let Value::F32(actual) = value(name) else {
                    panic!("{name}: expected f32")
                };
                assert_eq!(actual.to_bits(), expected.to_bits(), "{name}");
            }
            for (name, expected) in [
                ("PRECISE", 0.1f64),
                ("NEGATIVE_ZERO64", -0.0),
                ("LARGEST", f64::MAX),
                ("NATIVE_WIDTH", if bit == 1 { 4.0 } else { 8.0 }),
            ] {
                let Value::F64(actual) = value(name) else {
                    panic!("{name}: expected f64")
                };
                assert_eq!(actual.to_bits(), expected.to_bits(), "{name}");
            }
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
}

#[test]
fn scalar_conflicts_and_rejections() {
    helpers::ensure_libclang();
    for (old, new) in [("0.1f", "0.2f"), ("-0.0f", "0.0f"), ("true", "false")] {
        let mut inputs = [input(), input()];
        inputs[1].name = "other.h".into();
        inputs[1].source = inputs[1].source.replace(old, new);
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            let error = merge(&snapshots).err().unwrap().to_string();
            assert!(
                error.contains("native value declarations differ"),
                "{error}"
            );
            inputs.reverse();
        }
    }
    let input = test_clang2::fixture("scalar_constants_rejections");
    for (name, diagnostic) in [
        ("SIDE_EFFECT", "initializer is not a constant expression"),
        ("INFINITE", "not a supported finite scalar constant"),
        ("NOT_A_NUMBER", "not a supported finite scalar constant"),
        ("EXTENDED", "not a supported finite scalar constant"),
        ("MUTABLE", "requires a const-qualified type"),
    ] {
        let mut selected = test_clang2::fixture("scalar_constants_rejections");
        selected.roots = vec![name.into()];
        let error = capture(&[selected], &TARGETS).unwrap_err().to_string();
        assert!(error.contains(diagnostic), "{name}: {error}");
    }
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 1);
    for name in [
        "SIDE_EFFECT",
        "INFINITE",
        "NOT_A_NUMBER",
        "EXTENDED",
        "MUTABLE",
    ] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
}
