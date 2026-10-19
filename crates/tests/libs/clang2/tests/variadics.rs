use test_clang2::{fixture, input_dir, selected_fixture};
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const FUNCTIONS: &[&str] = &[
    "Sum",
    "Explicit",
    "NoFixed",
    "Fixed",
    "Variadic",
    "Requested",
];

fn imports(names: &[&str]) -> projection::Imports {
    names
        .iter()
        .map(|name| {
            (
                (*name).into(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect()
}

#[test]
fn native_variadic_imports() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("variadic_imports", FUNCTIONS);
    input.macros.push("SumAlias".into());
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for value in &snapshot.values {
            let observation = &value.observations[0];
            let TypeKind::Function(function) = &snapshot.types[observation.canonical.0].kind else {
                panic!();
            };
            assert_eq!(function.variadic, value.name != "Fixed");
            assert_eq!(function.convention, CallingConvention::C);
            if function.variadic {
                let ValueKind::Function {
                    stdcall_candidate, ..
                } = &observation.kind
                else {
                    panic!();
                };
                assert!(stdcall_candidate.is_none());
            }
        }
    }
    let output = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports(FUNCTIONS),
    )
    .unwrap();
    assert_eq!(output.functions, FUNCTIONS.len() + 1);
    let text = &output.partitions["variadic_imports"].rdl;
    let path = input_dir().join("variadic_imports.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&path, text).unwrap();
    }
    assert_eq!(
        text,
        &std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-variadics.winmd");
    let roundtrip = winmd.with_extension("rdl");
    let mut source = text.clone();
    for _ in 0..2 {
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(&winmd)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::read(&winmd).unwrap();
        for method in index.expect("Fixture", "Apis").methods() {
            let signature = method.signature(&[]);
            assert_eq!(
                signature
                    .flags
                    .contains(windows_metadata::MethodCallAttributes::VARARG),
                method.name() != "Fixed"
            );
            assert_eq!(
                signature.types.len(),
                usize::from(method.name() != "NoFixed")
            );
        }
        windows_rdl::writer()
            .input(&winmd)
            .filter("Fixture")
            .output(&roundtrip)
            .write()
            .unwrap();
        source = std::fs::read_to_string(&roundtrip).unwrap();
        assert_eq!(&source, text);
    }
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

#[test]
fn callbacks_and_methods_remain_rejected() {
    helpers::ensure_libclang();
    let output = projection::project_partial_with_imports(
        capture_partial(&[fixture("variadic_callbacks")], &TARGETS).unwrap(),
        "Fixture",
        &imports(&["UseCallback", "UsePointer", "Fixed", "UseFixed"]),
    )
    .unwrap();
    assert_eq!(output.output.functions, 2);
    assert_eq!(output.output.interfaces, 0);
    for name in [
        "VariadicCallback",
        "VariadicInterface",
        "UseCallback",
        "UsePointer",
    ] {
        assert!(
            output.omissions.iter().any(|reason| reason.contains(name)),
            "{name}"
        );
    }
    assert!(
        output
            .omissions
            .iter()
            .any(|reason| reason.contains("variadic callbacks"))
    );
    assert!(
        output
            .omissions
            .iter()
            .any(|reason| reason.contains("variadic interface"))
    );
}

#[test]
fn variadic_disagreement_is_native_conflict() {
    helpers::ensure_libclang();
    let mut inputs = [
        Input {
            name: "fixed.h".into(),
            source: "extern \"C\" int Clash(int first);".into(),
            ..Default::default()
        },
        Input {
            name: "variadic.h".into(),
            source: "extern \"C\" int Clash(int first, ...);".into(),
            ..Default::default()
        },
    ];
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
        assert!(
            error.contains("Clash") && error.contains("native field types differ"),
            "{error}"
        );
        inputs.reverse();
    }
}
