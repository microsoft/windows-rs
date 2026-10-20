use windows_clang2::*;
use windows_metadata::{Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const MEMBER_RECORDS: [&str; 13] = [
    "DeviceIoControl",
    "QuerySecurity",
    "Request",
    "Survivor",
    "AlignedValue",
    "AlignedArray",
    "AlignedUnion",
    "PaddingName",
    "FloatGap",
    "ArrayContainer",
    "AlignedPointer",
    "PointerBefore",
    "AlignmentConflict",
];

fn golden(name: &str, output: &projection::Output) -> String {
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang2::input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&path, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    rdl
}

#[test]
fn member_alignment_preserves_native_layout() {
    member_alignment("member-alignment", &[None]);
}

#[test]
#[ignore = "requires x86, x64, and ARM64 MSVC Rust targets"]
fn member_alignment_cross_targets() {
    member_alignment("member-alignment-cross", &TARGETS.map(Some));
}

fn member_alignment(name: &str, targets: &[Option<Target>]) {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture("member_alignment", &MEMBER_RECORDS);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let wide = snapshot.target != Target::X86;
        for (name, size, offsets) in [
            (
                "DeviceIoControl",
                if wide { 32 } else { 16 },
                if wide {
                    vec![0, 8, 16, 24]
                } else {
                    vec![0, 4, 8, 12]
                },
            ),
            (
                "QuerySecurity",
                if wide { 16 } else { 8 },
                if wide { vec![0, 8] } else { vec![0, 4] },
            ),
        ] {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let definition = record
                .observations
                .iter()
                .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                .unwrap();
            assert_eq!(
                definition.layout,
                Layout {
                    size,
                    alignment: if wide { 8 } else { 4 }
                }
            );
            assert_eq!(
                definition
                    .fields
                    .iter()
                    .map(|field| field.offset_bits / 8)
                    .collect::<Vec<_>>(),
                offsets
            );
            assert_eq!(
                definition.fields[1]
                    .attributes
                    .iter()
                    .any(|attribute| matches!(
                        attribute.kind,
                        AttributeKind::Aligned { bits: 64, .. }
                    )),
                wide
            );
        }
    }
    let result = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(result.output.records, MEMBER_RECORDS.len());
    assert!(result.omissions.is_empty(), "{:?}", result.omissions);
    let rdl = golden("member_alignment", &result.output);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang2-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join(format!("target\\test-clang2-{name}.rdl"));
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), rdl);
    for sys in [true, false] {
        let bindings = root.join(format!("target\\test-clang2-{name}-{sys}.rs"));
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Fixture")
            .flat()
            .output(&bindings);
        if sys {
            builder.sys()
        } else {
            builder.minimal()
        }
        .write();
        let mut rust = std::fs::read_to_string(&bindings).unwrap();
        for snapshot in &snapshots {
            let arch = match snapshot.target {
                Target::X86 => "x86",
                Target::X64 => "x86_64",
                Target::Arm64 => "aarch64",
            };
            rust.push_str(&format!(
                "\n#[cfg(target_arch = \"{arch}\")] const _: () = {{\n"
            ));
            for record in &snapshot.records {
                let definition = record
                    .observations
                    .iter()
                    .find_map(|id| snapshot.observations[id.0].definition.as_ref())
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
        rust.push_str(
            r#"
fn main() {
    let mut storage = core::mem::MaybeUninit::<DeviceIoControl>::uninit();
    let value = unsafe {
        let pointer = storage.as_mut_ptr();
        core::ptr::addr_of_mut!((*pointer).OutputBufferLength).write(11);
        core::ptr::addr_of_mut!((*pointer).InputBufferLength).write(22);
        core::ptr::addr_of_mut!((*pointer).IoControlCode).write(33);
        core::ptr::addr_of_mut!((*pointer).Type3InputBuffer).write(core::ptr::null_mut());
        storage.assume_init()
    };
    let copied = value;
    assert_eq!(copied.InputBufferLength, 22);
    assert_eq!(copied.IoControlCode, 33);
    let _: u32 = copied.InputBufferLength;
    let mut single = AlignedValue::default();
    let field: &mut u32 = &mut single.Value;
    *field = 44;
    assert_eq!(single.Value, 44);
}
"#,
        );
        std::fs::write(&bindings, rust).unwrap();
        let executable = root.join(format!("target\\test-clang2-{name}-{sys}.exe"));
        for target in targets {
            let mut command = std::process::Command::new("rustc");
            command.args(["--edition=2024", "--crate-name=test_member_alignment"]);
            if let Some(target) = target {
                command.args(["--target", target.triple()]);
            }
            let result = command
                .arg(&bindings)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{target:?} sys={sys}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            if *target != Some(Target::Arm64) {
                let result = std::process::Command::new(&executable).output().unwrap();
                assert!(
                    result.status.success(),
                    "{}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
        for path in [&bindings, &executable, &executable.with_extension("pdb")] {
            if path.is_file() {
                std::fs::remove_file(path).unwrap();
            }
        }
    }
    for path in [winmd, rewritten] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn member_alignment_rejects_unqualified_abi_and_packing() {
    helpers::ensure_libclang();
    let input = test_clang2::selected_fixture(
        "member_alignment_calls",
        &[
            "ByPointer",
            "ArrayParameter",
            "AliasArrayParameter",
            "ByValue",
            "ByReturn",
            "ByContainingArray",
            "AlignedCallback",
            "IAligned",
            "PackedMember",
            "PackedContainer",
            "AlignedBitfield",
        ],
    );
    let imports = [
        "ByPointer",
        "ArrayParameter",
        "AliasArrayParameter",
        "ByValue",
        "ByReturn",
        "ByContainingArray",
    ]
    .into_iter()
    .map(|name| {
        (
            name.into(),
            projection::Import {
                library: "fixture.dll".into(),
                name: None,
            },
        )
    })
    .collect();
    let result = projection::project_partial_with_imports(
        capture_partial(&[input], &TARGETS).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    for name in [
        "ByValue",
        "ByReturn",
        "ByContainingArray",
        "AlignedCallback",
        "IAligned",
    ] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name) && message.contains("calling-ABI contract")),
            "{name}: {:?}",
            result.omissions
        );
    }
    for name in ["PackedMember", "PackedContainer", "AlignedBitfield"] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            result.omissions
        );
    }
    let rdl = golden("member_alignment_calls", &result.output);
    for name in ["ByPointer", "ArrayParameter", "AliasArrayParameter"] {
        assert!(rdl.contains(&format!("fn {name}(")), "{name}: {rdl}");
    }
}

#[test]
fn member_alignment_requests_conflict_even_with_identical_layout() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::selected_fixture("member_alignment", &["AlignmentConflict"]),
        test_clang2::selected_fixture("member_alignment", &["AlignmentConflict"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(0, "#define MEMBER_REQUEST 4\n");
    for input in &inputs {
        for snapshot in capture(std::slice::from_ref(input), &TARGETS).unwrap() {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == "AlignmentConflict")
                .unwrap();
            for definition in record
                .observations
                .iter()
                .filter_map(|id| snapshot.observations[id.0].definition.as_ref())
            {
                assert_eq!(
                    definition.layout,
                    Layout {
                        size: 16,
                        alignment: 16
                    }
                );
                assert_eq!(definition.fields[0].offset_bits, 0);
            }
        }
    }
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS)
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("AlignmentConflict") && error.contains("differ"),
            "{error}"
        );
        inputs.reverse();
    }
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_constant_and_callable_types_preserve_native_signedness() {
    helpers::ensure_libclang();
    let fixture = test_clang2::fixture("sdk_constant_types");
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = [
        "GetLastError",
        "SetLastError",
        "SetWinEventHook",
        "PHANDLER_ROUTINE",
    ]
    .map(String::from)
    .into();
    input.macros = [
        "CTRL_C_EVENT",
        "ERROR_INVALID_DATA",
        "WINEVENT_OUTOFCONTEXT",
    ]
    .map(String::from)
    .into();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for (name, scalar) in [
            ("CTRL_C_EVENT", Scalar::Int),
            ("ERROR_INVALID_DATA", Scalar::Long),
            ("WINEVENT_OUTOFCONTEXT", Scalar::Int),
        ] {
            let expected = TypeKind::Scalar(scalar);
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            assert!(
                value
                    .observations
                    .iter()
                    .all(|observation| snapshot.types[observation.canonical.0].kind == expected)
            );
        }
        for (name, parameter) in [
            ("GetLastError", None),
            ("SetLastError", Some(0)),
            ("SetWinEventHook", Some(6)),
        ] {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            for observation in &value.observations {
                let TypeKind::Function(function) = &snapshot.types[observation.canonical.0].kind
                else {
                    panic!()
                };
                let ty = parameter.map_or(function.result, |index| function.parameters[index]);
                assert_eq!(
                    snapshot.types[ty.0].kind,
                    TypeKind::Scalar(Scalar::UnsignedLong)
                );
            }
        }
    }
    let imports = [
        ("GetLastError", "kernel32.dll"),
        ("SetLastError", "kernel32.dll"),
        ("SetWinEventHook", "user32.dll"),
    ]
    .into_iter()
    .map(|(name, library)| {
        (
            name.into(),
            projection::Import {
                library: library.into(),
                name: None,
            },
        )
    })
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let rdl = golden("sdk_constant_types", &output);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-constant-types.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for architecture in [1, 2, 4] {
        let index = reader::Index::new_for_architecture(
            vec![reader::File::read(&winmd).unwrap()],
            architecture,
        );
        let apis = index.expect("Fixture", "Apis");
        for (name, value) in [
            ("CTRL_C_EVENT", 0),
            ("ERROR_INVALID_DATA", 13),
            ("WINEVENT_OUTOFCONTEXT", 0),
        ] {
            let field = apis.fields().find(|field| field.name() == name).unwrap();
            assert_eq!(field.constant().unwrap().value(), Value::I32(value));
        }
    }
    std::fs::remove_file(winmd).unwrap();
}
