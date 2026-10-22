use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{Type, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn type_visibility() {
    helpers::ensure_libclang();
    roundtrip();
    conflicts();
    rejections();
}

fn roundtrip() {
    let mut inputs = [
        selected_fixture("visibility", &[]),
        selected_fixture("visibility", &[]),
    ];
    for input in &mut inputs {
        input.source = "#include \"visibility.h\"".into();
    }
    inputs[1].name = "second.h".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    for snapshot in &snapshots {
        for record in &snapshot.records {
            let definition = record
                .observations
                .iter()
                .map(|id| &snapshot.observations[id.0])
                .find(|observation| observation.definition.is_some())
                .unwrap();
            let visible = definition
                .attributes
                .iter()
                .any(|attribute| attribute.kind == AttributeKind::DefaultTypeVisibility);
            assert_eq!(visible, record.name != "Container", "{}", record.name);
            if record.name == "_GUID" {
                assert_eq!(
                    definition.definition.as_ref().unwrap().layout,
                    Layout {
                        size: 16,
                        alignment: 4
                    }
                );
            }
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!((output.records, output.enums), (5, 1));
    inputs.reverse();
    inputs[0].name = "renamed.h".into();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    let rdl = &output.partitions["visibility"].rdl;
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("visibility.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-visibility.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        assert_eq!(
            index
                .expect("Fixture", "Guid")
                .fields()
                .map(|field| field.ty())
                .collect::<Vec<_>>(),
            [
                Type::U32,
                Type::U16,
                Type::U16,
                Type::ArrayFixed(Box::new(Type::U8), 8)
            ]
        );
        assert_eq!(
            index
                .expect("Fixture", "Container")
                .fields()
                .last()
                .unwrap()
                .ty(),
            Type::value_named("Fixture", "Guid")
        );
    }
    let rewritten = root.join("target\\test-clang-visibility.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(*rdl, std::fs::read_to_string(&rewritten).unwrap());
    let bindings = root.join("target\\test-clang-visibility.rs");
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(
        "\nconst _: () = {
            assert!(core::mem::size_of::<Guid>() == 16);
            assert!(core::mem::align_of::<Guid>() == 4);
            assert!(core::mem::offset_of!(Guid, Data4) == 8);
            assert!(core::mem::size_of::<Visible>() == 4);
            assert!(core::mem::size_of::<VisibleUnion>() == 4);
            assert!(VisibleValue == 7);
        };\n",
    );
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join("target\\test-clang-visibility.rmeta");
    let result = std::process::Command::new("rustc")
        .args([
            "--crate-type=lib",
            "--emit=metadata",
            "--edition=2024",
            "--crate-name",
            "test_clang_visibility",
        ])
        .arg(&bindings)
        .arg("-o")
        .arg(&compiled)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for path in [winmd, rewritten, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
}

fn conflicts() {
    for (root, prefix, native) in [
        ("Probe", "#define VISIBILITY\n", true),
        ("Enumeration", "#define VISIBILITY\n", true),
        (
            "PublicProbe",
            "#define TAG Other\n#define VISIBILITY\n",
            false,
        ),
    ] {
        let mut inputs = [
            selected_fixture("visibility_conflict", &[root]),
            selected_fixture("visibility_conflict", &[root]),
        ];
        inputs[0].source = "#include \"visibility_conflict.h\"".into();
        inputs[1].name = "other.h".into();
        inputs[1].source = format!("{prefix}#include \"visibility_conflict.h\"");
        let snapshots = capture(&inputs, &TARGETS).unwrap();
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
        selected_fixture("visibility_conflict", &["Probe"]),
        selected_fixture("visibility_conflict", &["Probe"]),
    ];
    inputs[0].source = "#define FORWARD\n#include \"visibility_conflict.h\"".into();
    inputs[1].name = "definition.h".into();
    inputs[1].source = "#define VISIBILITY\n#include \"visibility_conflict.h\"".into();
    assert!(
        projection::generate(&inputs, "Fixture")
            .unwrap_err()
            .to_string()
            .contains("forward type visibility request differs")
    );
    inputs[1].source = "#include \"visibility_conflict.h\"".into();
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap().records, 1);
}

fn rejections() {
    for name in ["Hidden", "Protected", "HiddenEnum"] {
        let error = capture(
            &[selected_fixture("visibility_rejections", &[name])],
            &TARGETS,
        )
        .err()
        .unwrap()
        .to_string();
        assert!(error.contains("unsupported type visibility"), "{error}");
    }
    let result =
        projection::generate_partial(&[selected_fixture("visibility_rejections", &[])], "Fixture")
            .unwrap();
    assert_eq!(result.output.records, 1);
    for name in [
        "Hidden",
        "Protected",
        "HiddenEnum",
        "HiddenAlias",
        "Dependent",
        "SymbolVisible",
        "Mixed",
    ] {
        assert!(
            result.omissions.iter().any(|error| error.contains(name)),
            "{name}"
        );
    }
    assert!(
        result
            .omissions
            .iter()
            .any(|error| error.contains("Unresolved(\"Visibility\")"))
    );
}
