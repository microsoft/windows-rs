use std::process::Command;
use windows_clang2::*;

#[test]
fn callback_interop_rdl() {
    let output = fixture();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang2::input_dir().join("callback_interop.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn callback_convention_conflicts() {
    helpers::ensure_libclang();
    let imports = [(
        "ConventionFunction".into(),
        projection::Import {
            library: "example.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    for name in [
        "ConventionCallback",
        "ConventionRecord",
        "ConventionFunction",
    ] {
        let mut inputs = [
            test_clang2::selected_fixture("callback_convention_conflicts", &[name]),
            test_clang2::selected_fixture("callback_convention_conflicts", &[name]),
        ];
        inputs[1].name = "other.h".into();
        inputs[1].source.insert_str(0, "#define OTHER\n");
        for _ in 0..2 {
            let snapshots = capture(&inputs, &[Target::X86, Target::X64, Target::Arm64]).unwrap();
            let merged = merge(&snapshots).unwrap();
            let error = projection::project_with_imports(&merged, "Fixture", &imports).unwrap_err();
            assert!(error.to_string().contains(name), "{error}");
            inputs.reverse();
        }
    }
}

fn fixture() -> projection::Output {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture(
        "callback_interop",
        &[
            "SystemCallbacks",
            "CdeclCallbacks",
            "CallbackSelection",
            "NamedSystem",
            "NamedCdecl",
            "NestedSystem",
            "NestedCdecl",
        ],
    );
    let mut snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        for (name, expected) in [
            ("SystemCallbacks", CallingConvention::Stdcall),
            ("CdeclCallbacks", CallingConvention::C),
        ] {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let definition = snapshot.observations[record.observations[0].0]
                .definition
                .as_ref()
                .unwrap();
            for field in definition.fields.iter().take(2) {
                assert_eq!(
                    written_convention(snapshot, field.written),
                    expected,
                    "{:?} {name}.{}",
                    snapshot.target,
                    field.name
                );
            }
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap()
    );
    output
}

#[test]
#[ignore = "requires three-target Rust/MSVC link toolchains"]
fn callback_interop() {
    let output = fixture();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang2-callback-interop");
    std::fs::create_dir_all(&scratch).unwrap();
    let rdl = scratch.join("input.rdl");
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    std::fs::write(&rdl, text).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("roundtrip.rdl");
    let again = scratch.join("roundtrip.winmd");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rewritten)
        .output(&again)
        .write()
        .unwrap();
    std::fs::copy(
        test_clang2::input_dir().join("callback_interop_consumer.rs"),
        scratch.join("main.rs"),
    )
    .unwrap();
    for metadata in [winmd, again] {
        for sys in [false, true] {
            consume(&scratch, &metadata, sys);
        }
    }
    std::fs::remove_dir_all(scratch).unwrap();
}

fn consume(scratch: &std::path::Path, metadata: &std::path::Path, sys: bool) {
    let mut builder = windows_bindgen::builder();
    builder.input(metadata).filter("Fixture").flat();
    if sys {
        builder.sys();
    }
    builder.output(scratch.join("bindings.rs")).write();
    let mut failures = Vec::new();
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        let executable = scratch.join(format!("{target}.exe"));
        let result = Command::new("rustc")
            .args([
                "--edition=2024",
                "--crate-name=callback_interop",
                "--target",
                target,
            ])
            .arg(scratch.join("main.rs"))
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        if !result.status.success() {
            failures.push(format!(
                "{target}:\n{}",
                String::from_utf8_lossy(&result.stderr)
            ));
            continue;
        }

        if target == "i686-pc-windows-msvc"
            && cfg!(any(target_arch = "x86", target_arch = "x86_64"))
            || target == "x86_64-pc-windows-msvc" && cfg!(target_arch = "x86_64")
            || target == "aarch64-pc-windows-msvc" && cfg!(target_arch = "aarch64")
        {
            let result = Command::new(&executable).output().unwrap();
            assert!(
                result.status.success(),
                "{target}:\n{}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
    assert!(
        failures.is_empty(),
        "{} sys={sys}:\n{}",
        metadata.display(),
        failures.join("\n")
    );
}

fn written_convention(snapshot: &Snapshot, mut id: TypeId) -> CallingConvention {
    loop {
        match &snapshot.types[id.0].kind {
            TypeKind::Alias(alias) => id = snapshot.aliases[alias.0].observations[0].underlying,
            TypeKind::Pointer(inner) => id = *inner,
            TypeKind::Attributed { convention, .. } => return *convention,
            TypeKind::Function(function) => return function.convention,
            other => panic!("{other:?}"),
        }
    }
}
