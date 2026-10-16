use std::collections::BTreeMap;
use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;
use windows_metadata::reader::HasAttributes;
use windows_metadata::{Type as MetadataType, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn imports() -> BTreeMap<String, projection::Import> {
    [
        ("ReadPayload", "example.dll", Some("ReadPayloadEntry")),
        ("NativeSum", "example.dll", None),
        ("GetCurrentProcessId", "kernel32.dll", None),
    ]
    .into_iter()
    .map(|(name, library, import)| {
        (
            name.into(),
            projection::Import {
                library: library.into(),
                name: import.map(String::from),
            },
        )
    })
    .collect()
}

fn project(inputs: &[Input]) -> projection::Output {
    let snapshots = capture(inputs, &TARGETS).unwrap();
    projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports()).unwrap()
}

fn roundtrip(name: &str, output: &projection::Output, check: impl Fn(&reader::Index, i32)) {
    let text: String = output
        .partitions
        .values()
        .map(|partition| partition.rdl.as_str())
        .collect();
    let expected = std::fs::read_to_string(input_dir().join(format!("{name}.rdl"))).unwrap();
    assert_eq!(text, expected.replace("\r\n", "\n"));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang2-breadth-{name}.winmd"));
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("roundtrip.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
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
            check(&index, bit);
        }
    }
    let second = rdl.with_extension("again.rdl");
    windows_rdl::writer()
        .input(&again)
        .filter("Fixture")
        .output(&second)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read(&rdl).unwrap(),
        std::fs::read(&second).unwrap()
    );
    for path in [winmd, again, rdl, second] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn global_overloads() {
    helpers::ensure_libclang();
    let input = selected_fixture("global_overloads", &[]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert_eq!(snapshot.values.len(), 2);
        assert_eq!(snapshot.inline_functions.len(), 3);
        for helper in &snapshot.inline_functions {
            assert!(!helper.usr.is_empty());
            assert!(snapshot.files[helper.location.file.0].ends_with("global_overloads.h"));
            let name = &input.source[helper.location.offset as usize..];
            assert!(name.starts_with(&helper.name));
            let exported = snapshot
                .values
                .iter()
                .find(|value| value.name == helper.name)
                .unwrap();
            assert_ne!(helper.usr, exported.usr);
            assert!(exported.observations.iter().all(|observation| matches!(
                observation.kind,
                ValueKind::Function {
                    language: LanguageLinkage::C,
                    ..
                }
            )));
        }
    }
    let output = project(std::slice::from_ref(&input));
    roundtrip("global_overloads", &output, |index, bit| {
        let methods: Vec<_> = index
            .expect("Fixture", "Apis")
            .methods()
            .filter(|method| method.arches() == 0 || method.arches() & bit != 0)
            .collect();
        assert_eq!(methods.len(), 2);
        let sum = methods
            .iter()
            .find(|method| method.name() == "NativeSum")
            .unwrap();
        assert_eq!(sum.signature(&[]).types, [MetadataType::I32]);
        assert_eq!(sum.signature(&[]).return_type, MetadataType::I32);
        let read = methods
            .iter()
            .find(|method| method.name() == "ReadPayload")
            .unwrap();
        assert_eq!(
            read.signature(&[]).types,
            [
                MetadataType::PtrConst(
                    Box::new(MetadataType::value_named("Fixture", "Payload")),
                    1
                ),
                MetadataType::PtrMut(Box::new(MetadataType::U32), 1),
            ]
        );
    });
    let partial = projection::project_partial_with_imports(
        capture_partial(std::slice::from_ref(&input), &TARGETS).unwrap(),
        "Fixture",
        &imports(),
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    let selected = project(&[selected_fixture(
        "global_overloads",
        &["NativeSum", "ReadPayload"],
    )]);
    assert_eq!(selected, output);
    let mut inputs = [input, selected_fixture("global_overloads", &[])];
    inputs[1].name = "other.h".into();
    inputs[1].source = inputs[1].source.replace(
        "return static_cast<int>(value);",
        "return static_cast<int>(value) + 1;",
    );
    let paired = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), paired);
    inputs[1].source = format!("#define NATIVE_TYPE long long\n{}", inputs[1].source);
    assert!(merge(&capture(&inputs, &TARGETS).unwrap()).is_err());
    let rejected = [
        "External",
        "Defined",
        "Only",
        "Missing",
        "Architecture",
        "Internal",
        "StaticOverload",
    ];
    for name in rejected {
        assert!(
            capture(
                &[selected_fixture("global_overload_rejections", &[name])],
                &TARGETS
            )
            .is_err(),
            "{name}"
        );
    }
    let partial = projection::generate_partial(
        &[selected_fixture("global_overload_rejections", &[])],
        "Fixture",
    )
    .unwrap();
    assert_eq!(partial.output.functions, 0);
    assert_eq!(partial.output.records, 1);
    for name in rejected {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
}

#[test]
fn value_inheritance_rejections() {
    helpers::ensure_libclang();
    for (name, reason) in [
        ("Multiple", "multiple value bases"),
        ("Private", "only public non-virtual bases"),
        ("Virtual", "only public non-virtual bases"),
        ("Collision", "base field name collides"),
        ("EmptyBase", "empty record"),
        ("PackedAligned", "contains an aligned base"),
        ("NativeSum", "by-value C++ inheritance"),
        ("Returns", "by-value C++ inheritance"),
        ("Takes", "by-value C++ inheritance"),
        ("TakesContainer", "by-value C++ inheritance"),
        ("CallbackHolder", "by-value C++ inheritance"),
    ] {
        let input = selected_fixture("value_inheritance_rejections", &[name]);
        let error = capture(&[input], &TARGETS)
            .and_then(|snapshots| {
                projection::project_with_imports(&merge(&snapshots)?, "Fixture", &imports())
            })
            .unwrap_err()
            .to_string();
        assert!(error.contains(reason), "{name}: {error}");
    }
    let output = projection::generate_partial(
        &[selected_fixture("value_inheritance_rejections", &[])],
        "Fixture",
    )
    .unwrap();
    assert!(
        output
            .output
            .partitions
            .values()
            .any(|part| part.rdl.contains("struct Derived"))
    );
    for name in [
        "Multiple",
        "Private",
        "Virtual",
        "Collision",
        "EmptyBase",
        "PackedAligned",
        "Returns",
        "Takes",
        "TakesContainer",
        "CallbackHolder",
    ] {
        assert!(
            output.omissions.iter().any(|error| error.contains(name)),
            "{name}"
        );
    }
}

#[test]
fn interface_overloads() {
    helpers::ensure_libclang();
    let input = selected_fixture("interface_overloads", &["IOverloaded"]);
    let output = project(std::slice::from_ref(&input));
    roundtrip("interface_overloads", &output, |index, _| {
        let methods: Vec<_> = index.expect("Fixture", "IOverloaded").methods().collect();
        assert_eq!(
            methods
                .iter()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["Read", "Read", "Read", "Tail"]
        );
        for (i, method) in methods.iter().enumerate() {
            assert_eq!(
                method.signature(&[]).types,
                match i {
                    0 | 2 => vec![MetadataType::I32],
                    1 => vec![MetadataType::F32],
                    _ => vec![],
                }
            );
            assert_eq!(
                method
                    .find_attribute("NativeConstMethodAttribute")
                    .is_some(),
                i == 0
            );
        }
    });
    let mut inputs = [
        selected_fixture("interface_overloads", &["IOverloaded"]),
        input,
    ];
    inputs[1].name = "other.h".into();
    let mut snapshots = capture(&inputs, &TARGETS).unwrap();
    for snapshot in &snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "IOverloaded")
            .unwrap();
        let definition = snapshot.observations[record.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        assert_eq!(
            definition
                .methods
                .iter()
                .map(|method| method.slot)
                .collect::<Vec<_>>(),
            [3, 4, 5, 6]
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    inputs.reverse();
    assert_eq!(project(&inputs), output);
    let record = snapshots[1]
        .records
        .iter()
        .find(|record| record.name == "IOverloaded")
        .unwrap();
    let id = record.observations[0];
    snapshots[1].observations[id.0]
        .definition
        .as_mut()
        .unwrap()
        .methods[0]
        .slot += 1;
    assert!(
        merge(&snapshots)
            .err()
            .unwrap()
            .to_string()
            .contains("method names, slots")
    );
    snapshots.remove(0);
    let error = projection::project(&merge(&snapshots).unwrap(), "Fixture")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("non-contiguous virtual method slots"),
        "{error}"
    );
    let error = projection::generate(
        &[selected_fixture(
            "interface_overload_collision",
            &["ICollision"],
        )],
        "Fixture",
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("interface method name collision"), "{error}");
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn interface_overload_bindings() {
    helpers::ensure_libclang();
    let output = project(&[selected_fixture("interface_overloads", &["IOverloaded"])]);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let directory = root.join("target\\test-clang2-interface-overloads");
    std::fs::create_dir_all(&directory).unwrap();
    let winmd = directory.join("fixture.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|part| &part.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    let bindings = directory.join("lib.rs");
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .flat()
        .implement_all()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(
        "\npub fn check_vtable(value: &IOverloaded_Vtbl) {\n\
         let _: unsafe extern \"system\" fn(*mut core::ffi::c_void, i32) -> i32 = value.Read;\n\
         let _: unsafe extern \"system\" fn(*mut core::ffi::c_void, f32) -> i32 = value.Read2;\n\
         let _: unsafe extern \"system\" fn(*mut core::ffi::c_void, i32) -> i32 = value.Read3;\n\
         }\n\
         const _: () = {\n\
         let pointer = core::mem::size_of::<usize>();\n\
         assert!(core::mem::offset_of!(IOverloaded_Vtbl, Read) == 3 * pointer);\n\
         assert!(core::mem::offset_of!(IOverloaded_Vtbl, Read2) == 4 * pointer);\n\
         assert!(core::mem::offset_of!(IOverloaded_Vtbl, Read3) == 5 * pointer);\n\
         assert!(core::mem::offset_of!(IOverloaded_Vtbl, Tail) == 6 * pointer);\n\
         assert!(core::mem::size_of::<IOverloaded_Vtbl>() == 7 * pointer);\n\
         };\n",
    );
    std::fs::write(&bindings, rust).unwrap();
    let manifest = directory.join("Cargo.toml");
    std::fs::write(&manifest, format!(
        "[package]\nname = \"test-clang2-interface-overloads\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [workspace]\n[lib]\npath = \"lib.rs\"\n[dependencies]\nwindows-core = {{ path = {:?} }}\n",
        root.join("crates\\libs\\core")
    )).unwrap();
    for target in TARGETS {
        let result = std::process::Command::new("cargo")
            .args([
                "check",
                "--offline",
                "--quiet",
                "--target",
                target.triple(),
                "--manifest-path",
            ])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(root.join("target\\test-clang2-binding-build"))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{target:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for path in [winmd, bindings, manifest, directory.join("Cargo.lock")] {
        std::fs::remove_file(path).unwrap();
    }
    std::fs::remove_dir(directory).unwrap();
}

#[test]
fn enum_constants() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("enum_constants", &[]);
    input.macros = vec!["SELECTED_MACRO".into(), "UNNAMED_VALUE".into()];
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 5);
    assert_eq!(output.enums, 2);
    let mut equal = [
        selected_fixture("enum_constants", &["SELECTED"]),
        selected_fixture("enum_constants", &["SELECTED"]),
    ];
    equal[1].name = "other.h".into();
    equal[1].source = format!("enum Unrelated {{ Extra }};\n{}", equal[1].source);
    equal[1].roots.push("Unrelated".into());
    let snapshots = capture(&equal, &TARGETS).unwrap();
    assert_ne!(
        snapshots[0]
            .enums
            .iter()
            .position(|value| value.name == "Mode"),
        snapshots[1]
            .enums
            .iter()
            .position(|value| value.name == "Mode")
    );
    assert_eq!(merge(&snapshots).unwrap().values.len(), 1);
    for _ in 0..2 {
        assert_eq!(project(&equal).constants, 1);
        equal.reverse();
    }
    let mut distinct = [
        selected_fixture("enum_constants", &["SELECTED"]),
        selected_fixture("enum_constants", &["SELECTED"]),
    ];
    distinct[1].name = "distinct.h".into();
    distinct[1].source = distinct[1].source.replace("Mode", "Other");
    for _ in 0..2 {
        let snapshots = capture(&distinct, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(projection::generate_partial(&distinct, "Fixture").is_err());
        distinct.reverse();
    }
    roundtrip("enum_constants", &output, |index, bit| {
        let apis = index.expect("Fixture", "Apis");
        for (name, ty, value) in [
            ("SELECTED", "ModeAlias", windows_metadata::Value::I8(-1)),
            ("SELECTED_MACRO", "Mode", windows_metadata::Value::I8(2)),
            ("UNNAMED_VALUE", "Mode", windows_metadata::Value::I8(3)),
            ("LARGE", "Wide", windows_metadata::Value::U64(u64::MAX)),
            (
                "NATIVE_WIDTH",
                "Wide",
                windows_metadata::Value::U64(if bit == 1 { 4 } else { 8 }),
            ),
        ] {
            let field = apis
                .fields()
                .find(|field| {
                    field.name() == name && (field.arches() == 0 || field.arches() & bit != 0)
                })
                .unwrap();
            assert_eq!(field.ty(), MetadataType::value_named("Fixture", ty));
            assert_eq!(field.constant().unwrap().value(), value);
        }
    });
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    for replacement in ["Mode::On", "static_cast<Mode>(3)"] {
        let mut inputs = [
            selected_fixture("enum_constants", &["SELECTED"]),
            selected_fixture("enum_constants", &["SELECTED"]),
        ];
        inputs[1].name = "other.h".into();
        inputs[1].source = inputs[1].source.replace("Mode::Off", replacement);
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
}

#[test]
fn aggregate_constants() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("aggregate_constants", &[]);
    input.macros = ["NARROW_MACRO", "WIDE_MACRO", "GUID_MACRO"]
        .map(String::from)
        .into();
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 12);
    let partial = projection::generate_partial(std::slice::from_ref(&input), "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    roundtrip("aggregate_constants", &output, |index, bit| {
        let apis = index.expect("Fixture", "Apis");
        let field = |name| {
            apis.fields()
                .find(|field| {
                    field.name() == name && (field.arches() == 0 || field.arches() & bit != 0)
                })
                .unwrap()
        };
        for (name, value, encoding) in [
            ("ASCII_TEXT", "a\0\0\0\0\0\0", "ansi"),
            ("UTF16_TEXT", "wide", "utf-16"),
            ("WIDE_TEXT", "A\u{1f600}\0B", "utf-16"),
            ("NARROW_MACRO", "macro", "ansi"),
            ("WIDE_MACRO", "wide", "utf-16"),
        ] {
            let field = field(name);
            assert_eq!(field.ty(), MetadataType::String);
            assert_eq!(
                field.constant().unwrap().value(),
                windows_metadata::Value::Utf16(value.into())
            );
            assert_eq!(
                field
                    .find_attribute("NativeEncodingAttribute")
                    .unwrap()
                    .value()[0]
                    .1,
                windows_metadata::Value::Utf8(encoding.into())
            );
        }
        for (name, guid) in [
            ("FORWARD_GUID", 0xffffffffabcd123480010203040506ff_u128),
            ("COPIED_GUID", 0),
            ("GUID_MACRO", 0x12345678000000000000000000000000),
            ("ZERO_GUID", 0),
            ("SELECTED_GUID", 0x00000010000500060700000000000000),
            (
                "ARCH_GUID",
                u128::from(if bit == 1 { 4_u32 } else { 8 }) << 96,
            ),
            ("KEY", 0x123456789abcdef00102030405060708),
        ] {
            let field = field(name);
            let mut expected = vec![
                windows_metadata::Value::U32((guid >> 96) as u32),
                windows_metadata::Value::U16((guid >> 80) as u16),
                windows_metadata::Value::U16((guid >> 64) as u16),
            ];
            expected.extend(
                guid.to_be_bytes()[8..]
                    .iter()
                    .map(|value| windows_metadata::Value::U8(*value)),
            );
            assert_eq!(
                field
                    .find_attribute("GuidAttribute")
                    .unwrap()
                    .value()
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect::<Vec<_>>(),
                expected
            );
        }
        assert_eq!(
            field("KEY").constant().unwrap().value(),
            windows_metadata::Value::U32(42)
        );
        for (name, message) in [("KEY", "use another key"), ("SELECTED_GUID", "use next")] {
            assert_eq!(
                field(name)
                    .find_attribute("NativeDeprecatedAttribute")
                    .unwrap()
                    .value()[0]
                    .1,
                windows_metadata::Value::Utf8(message.into())
            );
        }
    });
    let mut equal = [
        selected_fixture("aggregate_constants", &["FORWARD_GUID", "WIDE_TEXT"]),
        selected_fixture(
            "aggregate_constants",
            &["FORWARD_GUID", "WIDE_TEXT", "Unrelated"],
        ),
    ];
    equal[1].name = "other.h".into();
    equal[1].source = format!("struct Unrelated {{ int value; }};\n{}", equal[1].source);
    for _ in 0..2 {
        assert_eq!(project(&equal).constants, 2);
        equal.reverse();
    }
    for (name, old, new) in [
        ("FORWARD_GUID", "0xffffffff", "0xfffffffe"),
        ("KEY", "}, 42}", "}, 43}"),
        ("WIDE_TEXT", r#"L"A\U0001F600\0B""#, r#"L"C\U0001F600\0B""#),
    ] {
        let mut conflict = [
            selected_fixture("aggregate_constants", &[name]),
            selected_fixture("aggregate_constants", &[name]),
        ];
        conflict[1].name = "other.h".into();
        assert!(conflict[1].source.contains(old));
        conflict[1].source = conflict[1].source.replace(old, new);
        assert!(projection::generate(&conflict, "Fixture").is_err());
        assert!(projection::generate_partial(&conflict, "Fixture").is_err());
    }
    for name in [
        "SIDE_EFFECT",
        "UNDEFINED",
        "INVALID_UTF16",
        "UNTERMINATED",
        "NON_ASCII",
        "WRONG_SHAPE",
    ] {
        let input = selected_fixture("aggregate_constant_rejections", &[name]);
        assert!(
            projection::generate(std::slice::from_ref(&input), "Fixture").is_err(),
            "{name}"
        );
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.constants, 0, "{name}");
        assert!(!partial.omissions.is_empty(), "{name}");
    }
}

#[test]
fn incomplete_array_parameters() {
    helpers::ensure_libclang();
    let input = selected_fixture("incomplete_arrays", &["Users"]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert!(
            snapshot
                .types
                .iter()
                .any(|ty| matches!(ty.kind, TypeKind::Array { length: None, .. })
                    && ty.layout.is_none())
        );
    }
    let output = project(std::slice::from_ref(&input));
    roundtrip("incomplete_arrays", &output, |index, _| {
        for (name, expected) in [
            (
                "ArrayInput",
                MetadataType::PtrConst(Box::new(MetadataType::value_named("Fixture", "Item")), 1),
            ),
            (
                "AliasInput",
                MetadataType::PtrConst(Box::new(MetadataType::value_named("Fixture", "Item")), 1),
            ),
            (
                "ArrayOutput",
                MetadataType::PtrMut(Box::new(MetadataType::value_named("Fixture", "Item")), 1),
            ),
            (
                "ArrayRows",
                MetadataType::PtrMut(
                    Box::new(MetadataType::ArrayFixed(
                        Box::new(MetadataType::value_named("Fixture", "Count")),
                        3,
                    )),
                    1,
                ),
            ),
        ] {
            let callback = index.expect("Fixture", name);
            let method = callback
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap();
            assert_eq!(method.signature(&[]).types, [expected], "{name}");
        }
    });
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    let mut inputs = [
        selected_fixture("incomplete_arrays", &["ArrayInput"]),
        selected_fixture("incomplete_arrays", &["ArrayInput"]),
    ];
    inputs[1].name = "bounded.h".into();
    inputs[1].source = inputs[1].source.replace("values[]", "values[3]");
    for _ in 0..2 {
        assert_eq!(project(&inputs).aliases, 2);
        inputs.reverse();
    }
    let input = selected_fixture("incomplete_array_fields", &[]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert!(snapshot.types.iter().any(|ty| matches!(
            ty.kind,
            TypeKind::Array {
                length: Some(0),
                ..
            }
        )));
    }
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("Flexible") && error.contains("incomplete arrays"))
    );
    assert!(
        partial.output.partitions["incomplete_array_fields"]
            .rdl
            .contains("struct Safe")
    );
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_constants() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_constants.h".into();
    input.source = test_clang2::fixture("sdk_constants").source;
    input.roots = [
        "FOLDERID_Documents",
        "FOLDERID_Desktop",
        "PKEY_Title",
        "PKEY_ItemNameDisplay",
        "RuntimeClass_Windows_Foundation_Uri",
        "InterfaceName_Windows_Foundation_IStringable",
    ]
    .map(String::from)
    .into();
    input.macros = [
        "ERROR_ACCESS_DENIED",
        "GENERIC_READ",
        "INVALID_FILE_ATTRIBUTES",
        "WAIT_TIMEOUT",
        "S_OK",
        "E_FAIL",
        "FILE_ATTRIBUTE_NORMAL",
    ]
    .map(String::from)
    .into();
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 13);
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    roundtrip("sdk_constants", &output, |index, _| {
        use windows_metadata::Value::*;
        let apis = index.expect("Fixture", "Apis");
        let field = |name| apis.fields().find(|field| field.name() == name).unwrap();
        for (name, expected) in [
            ("ERROR_ACCESS_DENIED", I32(5)),
            ("GENERIC_READ", U32(0x80000000)),
            ("INVALID_FILE_ATTRIBUTES", U32(u32::MAX)),
            ("WAIT_TIMEOUT", I32(258)),
            ("S_OK", I32(0)),
            ("E_FAIL", I32(-2147467259)),
            ("FILE_ATTRIBUTE_NORMAL", I32(128)),
            ("PKEY_Title", U32(2)),
            ("PKEY_ItemNameDisplay", U32(10)),
            (
                "RuntimeClass_Windows_Foundation_Uri",
                Utf16("Windows.Foundation.Uri".into()),
            ),
            (
                "InterfaceName_Windows_Foundation_IStringable",
                Utf16("Windows.Foundation.IStringable".into()),
            ),
        ] {
            assert_eq!(field(name).constant().unwrap().value(), expected, "{name}");
        }
        for (name, expected) in [
            ("FOLDERID_Documents", 0xfdd39ad0238f46afadb46c85480369c7),
            ("FOLDERID_Desktop", 0xb4bfcc3adb2c424cb0297fe99a87c641),
            ("PKEY_Title", 0xf29f85e04ff91068ab9108002b27b3d9),
            ("PKEY_ItemNameDisplay", 0xb725f13047ef101aa5f102608c9eebac),
        ] {
            let args = field(name).find_attribute("GuidAttribute").unwrap().value();
            assert_eq!(args.len(), 11);
            let guid = args
                .into_iter()
                .fold(0_u128, |guid, (_, value)| match value {
                    U32(value) => (guid << 32) | u128::from(value),
                    U16(value) => (guid << 16) | u128::from(value),
                    U8(value) => (guid << 8) | u128::from(value),
                    _ => panic!("unexpected GUID argument"),
                });
            assert_eq!(guid, expected, "{name}");
        }
    });
}

#[test]
fn interface_qualifiers() {
    helpers::ensure_libclang();
    let input = selected_fixture("interface_qualifiers", &["Qualified", "QualifiedCallback"]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "Qualified")
            .unwrap();
        let definition = record
            .observations
            .iter()
            .find_map(|id| snapshot.observations[id.0].definition.as_ref())
            .unwrap();
        let TypeKind::Pointer(object) = snapshot.types[definition.fields[0].canonical.0].kind
        else {
            panic!()
        };
        assert!(snapshot.types[object.0].qualifiers.constant);
    }
    let output = project(std::slice::from_ref(&input));
    roundtrip("interface_qualifiers", &output, |index, _| {
        let object = MetadataType::class_named("Fixture", "IBase");
        let slots = MetadataType::PtrConst(Box::new(object.clone()), 1);
        assert_eq!(
            index
                .expect("Fixture", "Qualified")
                .fields()
                .map(|field| field.ty())
                .collect::<Vec<_>>(),
            [
                object.clone(),
                slots.clone(),
                slots.clone(),
                MetadataType::PtrMut(Box::new(object.clone()), 1),
                object.clone(),
                MetadataType::PtrConst(Box::new(MetadataType::value_named("Fixture", "Plain")), 1),
                object.clone(),
            ]
        );
        assert_eq!(
            index.expect("Fixture", "ObjectAlias").underlying_type(),
            Some(object.clone())
        );
        let callback = index.expect("Fixture", "QualifiedCallback");
        let method = callback
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap();
        assert_eq!(method.signature(&[]).return_type, object);
        assert_eq!(method.signature(&[]).types, [object, slots]);
    });
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    let mut inputs = [
        selected_fixture("interface_qualifiers", &["Qualified"]),
        selected_fixture("interface_qualifiers", &["Qualified"]),
    ];
    inputs[1].name = "mutable.h".into();
    inputs[1].source = inputs[1]
        .source
        .replace("const IBase* object", "IBase* object");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
    let input = selected_fixture(
        "interface_qualifier_rejections",
        &[
            "VolatileObject",
            "ConstVolatileObject",
            "RestrictedStorage",
            "Safe",
        ],
    );
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    for name in ["VolatileObject", "ConstVolatileObject", "RestrictedStorage"] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|error| error.contains(name) && error.contains("volatile/restrict")),
            "{name}"
        );
    }
    assert!(
        partial.output.partitions["interface_qualifier_rejections"]
            .rdl
            .contains("struct Safe")
    );
}

#[test]
fn free_functions() {
    helpers::ensure_libclang();
    let input = selected_fixture("free_functions", &[]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.values.len(), 2);
    assert!(merged.values.iter().all(|value| value.groups.len() == 3));
    for snapshot in &snapshots {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "ReadPayload")
            .unwrap();
        let observation = &value.observations[0];
        let ValueKind::Function {
            language,
            mangling,
            parameters,
        } = &observation.kind
        else {
            panic!()
        };
        assert_eq!(*language, LanguageLinkage::C);
        assert_eq!(
            parameters
                .iter()
                .map(|parameter| parameter.name.as_str())
                .collect::<Vec<_>>(),
            ["payload", "written"]
        );
        assert!(!mangling.is_empty());
    }
    let output = projection::project_with_imports(&merged, "Fixture", &imports()).unwrap();
    assert_eq!(
        (output.functions, output.records, output.aliases),
        (2, 1, 1)
    );
    assert_eq!(
        projection::project_partial_with_imports(
            capture_partial(std::slice::from_ref(&input), &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap()
        .output,
        output
    );
    roundtrip("free_functions", &output, |index, bit| {
        let apis = index.expect("Fixture", "Apis");
        let method = apis
            .methods()
            .find(|method| {
                method.name() == "ReadPayload"
                    && (method.arches() == 0 || method.arches() & bit != 0)
            })
            .unwrap();
        let map = method.impl_map().unwrap();
        assert_eq!(map.import_scope().name(), "example.dll");
        assert_eq!(map.import_name(), "ReadPayloadEntry");
        assert_eq!(
            method.calling_convention(),
            if bit == 1 { "system" } else { "C" }
        );
        assert_eq!(
            method.signature(&[]).return_type,
            MetadataType::value_named("Fixture", "Count")
        );
        assert_eq!(
            method.signature(&[]).types,
            [
                MetadataType::PtrConst(
                    Box::new(MetadataType::value_named("Fixture", "Payload")),
                    1
                ),
                MetadataType::PtrMut(Box::new(MetadataType::value_named("Fixture", "Count")), 1),
            ]
        );
        assert_eq!(
            method
                .params()
                .map(|parameter| parameter.name().to_string())
                .collect::<Vec<_>>(),
            ["payload", "written"]
        );
    });
}

#[test]
fn exception_specifications() {
    helpers::ensure_libclang();
    let input = selected_fixture(
        "exception_specifications",
        &["CallbackUsers", "INoThrow", "NativeSum", "ReadPayload"],
    );
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        for (name, expected) in [
            ("NativeSum", ExceptionSpecification::Noexcept),
            ("ReadPayload", ExceptionSpecification::DynamicNone),
        ] {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            let observation = &value.observations[0];
            let mut id = observation.written;
            let written = loop {
                match &snapshot.types[id.0].kind {
                    TypeKind::Attributed { modified, .. } => id = *modified,
                    TypeKind::Function(function) => break function,
                    _ => panic!(),
                }
            };
            assert_eq!(written.exceptions, expected);
            let TypeKind::Function(canonical) = &snapshot.types[observation.canonical.0].kind
            else {
                panic!()
            };
            assert_eq!(canonical.exceptions, ExceptionSpecification::Noexcept);
        }
    }
    let output = project(std::slice::from_ref(&input));
    roundtrip("exception_specifications", &output, |index, _| {
        let contract = |attributes: Vec<(String, windows_metadata::Value)>| {
            attributes
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>()
        };
        for (owner, method, expected) in [
            ("Apis", "NativeSum", "noexcept"),
            ("Apis", "ReadPayload", "throw()"),
            ("INoThrow", "Safe", "noexcept"),
            ("INoThrow", "Legacy", "nothrow"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|value| value.name() == method)
                .unwrap();
            assert_eq!(
                contract(
                    method
                        .find_attribute("NativeExceptionSpecificationAttribute")
                        .unwrap()
                        .value()
                ),
                [windows_metadata::Value::Utf8(expected.into())]
            );
        }
        let callback = index.expect("Fixture", "NoThrowCallback");
        assert_eq!(
            contract(
                callback
                    .find_attribute("NativeExceptionSpecificationAttribute")
                    .unwrap()
                    .value()
            ),
            [windows_metadata::Value::Utf8("noexcept".into())]
        );
        let ordinary = index.expect("Fixture", "OrdinaryCallback");
        assert!(
            ordinary
                .find_attribute("NativeExceptionSpecificationAttribute")
                .is_none()
        );
        let users = index.expect("Fixture", "CallbackUsers");
        assert_eq!(
            users.fields().map(|field| field.ty()).collect::<Vec<_>>(),
            [
                "NoThrowCallback",
                "OrdinaryCallback",
                "NoThrowCallback",
                "OrdinaryCallback"
            ]
            .map(|name| MetadataType::class_named("Fixture", name))
        );
    });
    let partial = projection::project_partial_with_imports(
        capture_partial(std::slice::from_ref(&input), &TARGETS).unwrap(),
        "Fixture",
        &imports(),
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    for root in ["NoThrowCallback", "INoThrow", "NativeSum"] {
        let mut inputs = [
            selected_fixture("exception_specifications", &[root]),
            selected_fixture("exception_specifications", &[root]),
        ];
        inputs[1].name = "throwing.h".into();
        inputs[1].source = inputs[1].source.replace(" noexcept", "");
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            match merge(&snapshots) {
                Ok(merged) => {
                    assert!(
                        projection::project_with_imports(&merged, "Fixture", &imports()).is_err(),
                        "{root}"
                    );
                }
                Err(_) => assert_ne!(root, "NoThrowCallback"),
            }
            assert!(
                projection::generate_partial(&inputs, "Fixture").is_err(),
                "{root}"
            );
            inputs.reverse();
        }
    }
    for root in ["ComputedTrue", "ComputedFalse"] {
        let input = selected_fixture("exception_specification_rejections", &[root]);
        let error = capture(&[input], &TARGETS).unwrap_err().to_string();
        assert!(
            error.contains("unsupported function exception specification"),
            "{error}"
        );
    }
    let input = selected_fixture(
        "exception_specification_rejections",
        &["BareNoexcept", "DynamicNone", "Safe"],
    );
    let error = projection::generate(std::slice::from_ref(&input), "Fixture")
        .unwrap_err()
        .to_string();
    assert!(error.contains("requires a named callback"), "{error}");
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("BareNoexcept"))
    );
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("DynamicNone"))
    );
    assert!(
        partial.output.partitions["exception_specification_rejections"]
            .rdl
            .contains("struct Safe")
    );
}

#[test]
fn cpp_members() {
    helpers::ensure_libclang();
    let input = selected_fixture("cpp_members", &["ICppMembers"]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "ICppMembers")
            .unwrap();
        let definition = snapshot.observations[record.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        assert_eq!(definition.methods.len(), 1);
        assert_eq!(
            definition
                .non_vtable_members
                .iter()
                .map(|member| (&*member.name, member.kind))
                .collect::<Vec<_>>(),
            [
                ("Read", NonVTableMemberKind::Using),
                ("Read", NonVTableMemberKind::Method),
                ("Helper", NonVTableMemberKind::Method),
            ]
        );
        let ValueKind::Function { parameters, .. } = &definition.methods[0].observation.kind else {
            panic!()
        };
        assert_eq!(
            parameters
                .iter()
                .map(|parameter| parameter.default.as_ref().unwrap().expression.as_str())
                .collect::<Vec<_>>(),
            ["7", "nullptr", "nullptr", "RuntimeDefault()"]
        );
    }
    let output = project(std::slice::from_ref(&input));
    roundtrip("cpp_members", &output, |index, _| {
        let interface = index.expect("Fixture", "ICppMembers");
        let methods = interface.methods().collect::<Vec<_>>();
        assert_eq!(methods.len(), 1);
        let method = methods[0];
        assert_eq!(method.name(), "Inspect");
        assert!(
            method
                .find_attribute("NativeConstMethodAttribute")
                .is_some()
        );
        assert_eq!(
            method
                .params()
                .map(|parameter| parameter
                    .find_attribute("NativeDefaultArgumentAttribute")
                    .unwrap()
                    .value()[0]
                    .1
                    .clone())
                .collect::<Vec<_>>(),
            ["7", "nullptr", "nullptr", "RuntimeDefault()"]
                .map(|value| windows_metadata::Value::Utf8(value.into()))
        );
    });
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    for (original, replacement) in [
        (" const noexcept", " noexcept"),
        ("#define DEFAULT_COUNT 7", "#define DEFAULT_COUNT 8"),
    ] {
        let mut inputs = [
            selected_fixture("cpp_members", &["ICppMembers"]),
            selected_fixture("cpp_members", &["ICppMembers"]),
        ];
        inputs[1].name = "changed.h".into();
        inputs[1].source = inputs[1].source.replace(original, replacement);
        for _ in 0..2 {
            assert!(projection::generate_partial(&inputs, "Fixture").is_err());
            inputs.reverse();
        }
    }
    for (root, reason) in [
        ("IVolatile", "volatile/restrict method"),
        ("IRefQualified", "unsupported function properties"),
        ("UsingType", "method using declaration"),
    ] {
        let error = projection::generate(
            &[selected_fixture("cpp_member_rejections", &[root])],
            "Fixture",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains(reason), "{error}");
    }
}

#[test]
fn deprecations() {
    helpers::ensure_libclang();
    let input = selected_fixture("deprecations", &[]);
    let output = project(std::slice::from_ref(&input));
    roundtrip("deprecations", &output, |index, _| {
        let check = |attributes: Vec<(String, windows_metadata::Value)>,
                     message: &str,
                     replacement: &str| {
            assert_eq!(
                attributes
                    .into_iter()
                    .take(2)
                    .map(|(_, value)| value)
                    .collect::<Vec<_>>(),
                [
                    windows_metadata::Value::Utf8(message.into()),
                    windows_metadata::Value::Utf8(replacement.into())
                ]
            );
        };
        for (name, message, replacement) in [
            ("LegacyRecord", "old record", ""),
            ("OldCount", "old count", "NewCount"),
            ("LegacyCallback", "old callback", ""),
            ("ILegacy", "old interface", ""),
        ] {
            check(
                index
                    .expect("Fixture", name)
                    .find_attribute("NativeDeprecatedAttribute")
                    .unwrap()
                    .value(),
                message,
                replacement,
            );
        }
        let owned = index
            .expect("Fixture", "Owned")
            .find_attribute("NativeDeprecatedAttribute")
            .unwrap()
            .value();
        assert_eq!(
            owned[2],
            (
                "TypedefName".into(),
                windows_metadata::Value::Utf8("Owned".into())
            )
        );
        check(owned, "old typedef", "");
        for (owner, field, message) in [
            ("Apis", "LegacyValue", "old anonymous"),
            ("Apis", "LegacyConstant", ""),
            ("Uses", "value", "use \"new\"\nnext"),
            ("Mode", "Old", "old mode"),
        ] {
            let field = index
                .expect("Fixture", owner)
                .fields()
                .find(|value| value.name() == field)
                .unwrap();
            check(
                field
                    .find_attribute("NativeDeprecatedAttribute")
                    .unwrap()
                    .value(),
                message,
                "",
            );
        }
        for (owner, method, message) in [
            ("Apis", "NativeSum", "old function"),
            ("ILegacy", "Use", "old method"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|value| value.name() == method)
                .unwrap();
            check(
                method
                    .find_attribute("NativeDeprecatedAttribute")
                    .unwrap()
                    .value(),
                message,
                "",
            );
        }
    });
    let partial = projection::project_partial_with_imports(
        capture_partial(&[input], &TARGETS).unwrap(),
        "Fixture",
        &imports(),
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    for root in ["LegacyRecord", "OldCount", "ILegacy", "NativeSum", "Mode"] {
        let mut inputs = [
            selected_fixture("deprecations", &[root]),
            selected_fixture("deprecations", &[root]),
        ];
        inputs[1].name = "changed.h".into();
        inputs[1].source = inputs[1].source.replace("old ", "different ");
        for _ in 0..2 {
            assert!(
                projection::generate_partial(&inputs, "Fixture").is_err(),
                "{root}"
            );
            inputs.reverse();
        }
    }
}

#[test]
fn interfaces() {
    helpers::ensure_libclang();
    let input = selected_fixture("interfaces", &["IDerived"]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let base = snapshot
            .records
            .iter()
            .find(|record| record.name == "IBase")
            .unwrap();
        let definition = snapshot.observations[base.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        assert_eq!(definition.non_vtable_members.len(), 1);
        assert_eq!(definition.non_vtable_members[0].name, "QueryInterface");
        for method in &definition.methods {
            let TypeKind::Function(function) = &snapshot.types[method.observation.canonical.0].kind
            else {
                panic!()
            };
            assert_eq!(
                function.convention,
                if snapshot.target == Target::X86 {
                    CallingConvention::Stdcall
                } else {
                    CallingConvention::C
                }
            );
        }
    }
    let output = project(std::slice::from_ref(&input));
    assert_eq!((output.interfaces, output.records), (2, 1));
    roundtrip("interfaces", &output, |index, _| {
        let derived = index.expect("Fixture", "IDerived");
        assert_eq!(
            derived
                .find_attribute("GuidAttribute")
                .unwrap()
                .value()
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>(),
            [
                windows_metadata::Value::U32(0x01234567),
                windows_metadata::Value::U16(0x89ab),
                windows_metadata::Value::U16(0xcdef),
                windows_metadata::Value::U8(0x01),
                windows_metadata::Value::U8(0x23),
                windows_metadata::Value::U8(0x45),
                windows_metadata::Value::U8(0x67),
                windows_metadata::Value::U8(0x89),
                windows_metadata::Value::U8(0xab),
                windows_metadata::Value::U8(0xcd),
                windows_metadata::Value::U8(0xef),
            ]
        );
        assert!(
            derived
                .flags()
                .contains(windows_metadata::TypeAttributes::Interface)
        );
        assert_eq!(
            derived
                .interface_impls()
                .map(|base| base.interface(&[]))
                .collect::<Vec<_>>(),
            [MetadataType::class_named("Fixture", "IBase")]
        );
        assert_eq!(
            derived
                .methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["Read", "Count"]
        );
        let read = derived.methods().next().unwrap();
        assert_eq!(
            read.signature(&[]).types[1],
            MetadataType::PtrMut(Box::new(MetadataType::class_named("Fixture", "IBase")), 1)
        );
        let base = index.expect("Fixture", "IBase");
        assert_eq!(
            base.methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["QueryInterface", "AddRef", "Release"]
        );
        assert_eq!(
            base.methods().next().unwrap().signature(&[]).types[0],
            MetadataType::RefConst(Box::new(MetadataType::value_named("Fixture", "GUID")))
        );
        assert!(base.methods().all(|method| {
            method
                .signature(&[])
                .flags
                .contains(windows_metadata::MethodCallAttributes::HASTHIS)
        }));
    });
}

#[test]
fn constants() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("constants", &[]);
    input.macros = vec!["SHIFTED".into(), "NATIVE_BYTES".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    assert_eq!(merge(&snapshots).unwrap().values.len(), 6);
    for snapshot in &snapshots {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "SHIFTED")
            .unwrap();
        let observation = &value.observations[0];
        assert!(matches!(
            observation.kind,
            ValueKind::Constant {
                value: Some(ConstantValue::Integer(EnumValue::Unsigned(1099511627779))),
                origin: ConstantOrigin::Macro,
                ..
            }
        ));
        assert_eq!(observation.location.line, 7);
        assert!(snapshot.files[observation.location.file.0].ends_with("constants.h"));
    }
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 6);
    roundtrip("constants", &output, |index, bit| {
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
        assert_eq!(value("NEGATIVE"), windows_metadata::Value::I32(-9));
        assert_eq!(value("WIDE"), windows_metadata::Value::U64(u64::MAX));
        assert_eq!(
            value("SHIFTED"),
            windows_metadata::Value::U64(1099511627779)
        );
        assert_eq!(
            value("POINTER_BYTES"),
            windows_metadata::Value::U32(if bit == 1 { 4 } else { 8 })
        );
    });
}
#[test]
fn function_redeclarations() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture(
            "function_redeclarations_a",
            &["NativeSum", "ReadPayload", "STABLE"],
        ),
        selected_fixture(
            "function_redeclarations_b",
            &["NativeSum", "ReadPayload", "STABLE"],
        ),
    ];
    let mut expected = None;
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        assert!(
            projection::project_with_imports(&merged, "Fixture", &imports())
                .unwrap_err()
                .to_string()
                .contains("conflicting public names or contracts")
        );
        let partial = projection::project_partial_with_imports(
            capture_partial(&inputs, &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        assert_eq!(partial.output.functions, 0);
        assert_eq!(partial.output.constants, 1);
        for name in ["NativeSum", "ReadPayload"] {
            assert!(
                partial.omissions.iter().any(|error| error.contains(name)
                    && error.contains("conflicting public names or contracts")),
                "{name}"
            );
        }
        if let Some(expected) = &expected {
            assert_eq!(&partial, expected);
        } else {
            expected = Some(partial);
        }
        inputs.reverse();
    }
}

#[test]
fn conflicts_and_observation_order() {
    helpers::ensure_libclang();
    for (fixture, roots, from, to) in [
        ("free_functions", vec![], "int left", "unsigned int left"),
        ("interfaces", vec!["IDerived"], "ULONG index", "long index"),
        ("interfaces", vec!["IDerived"], "__stdcall", "__cdecl"),
        ("interfaces", vec!["IDerived"], "AddRef()", "OtherRef()"),
        (
            "interfaces",
            vec!["IDerived"],
            "01234567-89ab",
            "11234567-89ab",
        ),
        ("constants", vec![], "ANSWER = 42", "ANSWER = 43"),
        ("constants", vec![], "(1ULL << 40)", "(1ULL << 39)"),
    ] {
        let mut first = selected_fixture(fixture, &roots);
        let mut second = selected_fixture(fixture, &roots);
        second.name = "second.h".into();
        if fixture == "constants" {
            first.macros = vec!["SHIFTED".into()];
            second.macros = first.macros.clone();
        }
        let mut inputs = [first, second];
        let forward = project(&inputs);
        inputs.reverse();
        assert_eq!(project(&inputs), forward);
        inputs[0].source = inputs[0].source.replace(from, to);
        for _ in 0..2 {
            let captured = capture(&inputs, &TARGETS).unwrap();
            let error = match merge(&captured) {
                Ok(_) => panic!("accepted conflicting {fixture}: {from} -> {to}"),
                Err(error) => error.to_string(),
            };
            assert!(error.contains("differ"), "{error}");
            assert_eq!(
                capture_partial(&inputs, &TARGETS).unwrap_err().to_string(),
                error
            );
            inputs.reverse();
        }
    }
}

#[test]
fn dependencies_and_unsupported_contracts() {
    helpers::ensure_libclang();
    for scalar in ["int", "long"] {
        let inputs = [
            Input {
                name: "enum.h".into(),
                source: "enum : int { SHARED = 7 };".into(),
                ..Default::default()
            },
            Input {
                name: "value.h".into(),
                source: format!("const {scalar} SHARED = 7;"),
                ..Default::default()
            },
        ];
        let captured = capture(&inputs, &TARGETS).unwrap();
        let result = projection::project(&merge(&captured).unwrap(), "Fixture");
        if scalar == "int" {
            assert_eq!(result.unwrap().constants, 1);
        } else {
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("conflicting native definitions for constant")
            );
        }
    }
    let input = Input {
                    name: "dependencies.h".into(),
                    source: r#"
                        struct Bad { volatile int value; };
                        extern "C" void UsesBad(Bad* value);
                        struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) IBase {
                            virtual void __stdcall Read(Bad* value) = 0;
                        };
                        struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IDerived : IBase {
                            virtual void __stdcall Other() = 0;
                        };
                        extern "C" void UsesInterface(IDerived* value);
                        const int GOOD = 7;
                    "#.into(),
                    ..Default::default()
                };
    let imports = ["UsesBad", "UsesInterface"]
        .into_iter()
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "example.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let partial = projection::project_partial_with_imports(
        capture_partial(&[input], &TARGETS).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    assert_eq!(
        (
            partial.output.records,
            partial.output.functions,
            partial.output.interfaces,
            partial.output.constants
        ),
        (0, 0, 0, 1)
    );
    for name in ["Bad", "UsesBad", "IBase", "IDerived", "UsesInterface"] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            partial.omissions
        );
    }
    let input = selected_fixture("free_functions", &[]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    assert!(
        projection::project(&merge(&snapshots).unwrap(), "Fixture")
            .unwrap_err()
            .to_string()
            .contains("DLL/export ownership")
    );
    for source in [
        r#"extern "C" void Variadic(int first, ...);"#,
        "int Cxx(int value);",
        "int MUTABLE = 3;",
        r#"extern "C" int Next(); const int SIDE_EFFECT = (Next(), 3);"#,
        r#"struct __declspec(uuid("11111111-1111-1111-1111-111111111111")) I {
                        virtual void __stdcall Read() = 0;
                    };
                    struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) J : I {
                        virtual void __stdcall Read() = 0;
                    };"#,
    ] {
        assert!(
            capture(
                &[Input {
                    name: "unsupported.h".into(),
                    source: source.into(),
                    ..Default::default()
                }],
                &TARGETS
            )
            .is_err(),
            "{source}"
        );
    }
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_breadth() {
    helpers::ensure_libclang();
    let mut input = test_clang2::sdk_fixture();
    input.source = test_clang2::fixture("sdk_breadth").source;
    input.roots = vec![
        "GetCurrentProcessId".into(),
        "IClassFactory".into(),
        "UIA_InvokePatternId".into(),
    ];
    input.macros = vec!["MAX_PATH".into()];
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
    assert_eq!(
        (output.functions, output.interfaces, output.constants),
        (1, 2, 2)
    );
    roundtrip("sdk_breadth", &output, |index, bit| {
        let factory = index.expect("Fixture", "IClassFactory");
        assert_eq!(
            factory.interface_impls().next().unwrap().interface(&[]),
            MetadataType::class_named("Fixture", "IUnknown")
        );
        assert_eq!(
            factory
                .methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["CreateInstance", "LockServer"]
        );
        let apis = index.expect("Fixture", "Apis");
        let method = apis
            .methods()
            .find(|method| {
                method.name() == "GetCurrentProcessId"
                    && (method.arches() == 0 || method.arches() & bit != 0)
            })
            .unwrap();
        assert_eq!(
            method.impl_map().unwrap().import_scope().name(),
            "kernel32.dll"
        );
        let constant = |name| {
            apis.fields()
                .find(|field| field.name() == name)
                .unwrap()
                .constant()
                .unwrap()
                .value()
        };
        assert_eq!(constant("MAX_PATH"), windows_metadata::Value::I32(260));
        assert_eq!(
            constant("UIA_InvokePatternId"),
            windows_metadata::Value::I32(10000)
        );
    });
}
