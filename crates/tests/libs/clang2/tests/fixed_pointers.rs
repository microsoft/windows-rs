use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn fixed_pointer_rejections() {
    helpers::ensure_libclang();
    for case in 0..9 {
        let mut input = selected_fixture("fixed_pointers_invalid", &["Bad", "Dependent", "Safe"]);
        input.arguments.push(format!("-DCASE={case}"));
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        let rdl = &output.output.partitions["fixed_pointers_invalid"].rdl;
        assert!(!rdl.contains("Bad"), "{case}: {rdl}");
        assert!(!rdl.contains("Dependent"), "{case}: {rdl}");
        assert!(rdl.contains("struct Safe"), "{case}: {rdl}");
        assert!(!output.omissions.is_empty(), "{case}");
    }
}

#[test]
fn fixed_pointer_conflicts() {
    helpers::ensure_libclang();
    for other in ["void*", "unsigned long long", "const void* __ptr64"] {
        let mut inputs = ["void* __ptr64", other].map(|pointer| {
            let mut input = selected_fixture("fixed_pointers_conflict", &["Conflict"]);
            input
                .source
                .insert_str(0, &format!("#define POINTER {pointer}\n"));
            input
        });
        inputs[1].name = "other.h".into();
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string();
            let expected = if other == "void*" {
                "record layout or field count differs"
            } else {
                "native field types differ"
            };
            assert!(error.contains(expected), "{other}: {error}");
            inputs.reverse();
        }
    }
}

#[test]
fn fixed_pointers() {
    roundtrip(&[Target::X64]);
}

#[test]
#[ignore = "requires the pinned SDK, clang resource headers, and three MSVC Rust targets"]
fn sdk_file_segment() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    let fixture = test_clang2::fixture("sdk_file_segment");
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = [
        "FILE_SEGMENT_ELEMENT",
        "PFILE_SEGMENT_ELEMENT",
        "HANDLE64",
        "PHANDLE64",
        "SegmentConsumer",
    ]
    .map(String::from)
    .into();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let actual: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = input_dir().join("sdk_file_segment.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang2-sdk-file-segment");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&actual)
        .output(&winmd)
        .write()
        .unwrap();
    for sys in [false, true] {
        let bindings = scratch.join(format!("bindings-{sys}.rs"));
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Fixture.FILE_SEGMENT_ELEMENT")
            .filter("Fixture.SegmentConsumer")
            .flat()
            .output(&bindings);
        if sys {
            builder.sys();
        }
        builder.write();
        let mut rust = std::fs::read_to_string(&bindings).unwrap();
        rust.push_str(include_str!("../input/sdk_file_segment.rs"));
        std::fs::write(&bindings, rust).unwrap();
        for target in TARGETS {
            compile(&bindings, &scratch, target, sys);
        }
    }
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn fixed_pointers_cross_targets() {
    roundtrip(&TARGETS);
}

fn roundtrip(targets: &[Target]) {
    helpers::ensure_libclang();
    let snapshots = capture(
        &[selected_fixture(
            "fixed_pointers",
            &["Addresses", "Segment", "PackedAddresses"],
        )],
        &TARGETS,
    )
    .unwrap();
    for snapshot in &snapshots {
        let field = snapshot
            .observations
            .iter()
            .filter_map(|observation| observation.definition.as_ref())
            .flat_map(|definition| &definition.fields)
            .find(|field| field.name == "constant_storage")
            .unwrap();
        let TypeKind::Pointer(inner) = snapshot.types[field.canonical.0].kind else {
            panic!()
        };
        assert!(
            snapshot.types[inner.0].qualifiers.constant,
            "{:?}: {:?}",
            snapshot.target, snapshot.types[inner.0]
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let rdl = &output.partitions["fixed_pointers"].rdl;
    let expected = input_dir().join("fixed_pointers.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join(format!(
        "target\\test-clang2-fixed-pointers-{}",
        targets.len()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("rewritten.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(rewritten).unwrap(), *rdl);
    for sys in [false, true] {
        let bindings = scratch.join(format!("bindings-{sys}.rs"));
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Fixture")
            .flat()
            .output(&bindings);
        if sys {
            builder.sys().write();
        } else {
            builder.write();
        }
        let mut rust = std::fs::read_to_string(&bindings).unwrap();
        rust.push_str(include_str!("../input/fixed_pointers.rs"));
        for snapshot in &snapshots {
            let target = match snapshot.target {
                Target::X86 => "x86",
                Target::X64 => "x86_64",
                Target::Arm64 => "aarch64",
            };
            for record in &snapshot.records {
                if record.owner.is_some() {
                    continue;
                }
                let Some(definition) = &snapshot.observations[record.observations[0].0].definition
                else {
                    continue;
                };
                let name = &record.name;
                let layout = definition.layout;
                rust.push_str(&format!(
                    "\n#[cfg(target_arch = {target:?})] const _: () = {{\
                    assert!(core::mem::size_of::<{name}>() == {});\
                    assert!(core::mem::align_of::<{name}>() == {});",
                    layout.size, layout.alignment,
                ));
                for field in &definition.fields {
                    rust.push_str(&format!(
                        "assert!(core::mem::offset_of!({name}, {}) == {});",
                        field.name,
                        field.offset_bits / 8,
                    ));
                }
                rust.push_str("};\n");
            }
        }
        std::fs::write(&bindings, rust).unwrap();
        for target in targets {
            compile(&bindings, &scratch, *target, sys);
        }
    }
}

fn compile(bindings: &std::path::Path, scratch: &std::path::Path, target: Target, sys: bool) {
    let result = std::process::Command::new("rustc")
        .args([
            "--edition=2024",
            "--emit=metadata",
            "--target",
            target.triple(),
        ])
        .arg(bindings)
        .arg("-o")
        .arg(scratch.join("output.rmeta"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    if target != Target::Arm64 {
        let executable = scratch.join(format!("{target:?}-{sys}.exe"));
        let result = std::process::Command::new("rustc")
            .args(["--edition=2024", "--target", target.triple()])
            .arg(bindings)
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(
            std::process::Command::new(executable)
                .status()
                .unwrap()
                .success()
        );
    }
}
