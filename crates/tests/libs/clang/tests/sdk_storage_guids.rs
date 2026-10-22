use windows_clang::*;
use windows_metadata::{HasAttributes, Value, reader};

const GUIDS: [(&str, u128); 4] = [
    (
        "FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE",
        0x0d0a64a1_38fc_4db8_9fe7_3f4352cd7c5c,
    ),
    (
        "FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE",
        0xb7624d64_b9a3_4cf8_8011_5b86c940e7b7,
    ),
    (
        "FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE",
        0x9d453eb7_d2a6_4dbd_a2e3_fbd0ed9109a9,
    ),
    (
        "GUID_DEVINTERFACE_SMARTCARD_READER",
        0x50dd5230_ba8a_11d1_bf5d_0000f805f530,
    ),
];
const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
#[ignore = "requires the pinned Windows SDK and native companion"]
fn storage_guid_definitions() {
    helpers::ensure_libclang();
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = "sdk_storage_guids.h".into();
    input.source = test_clang::fixture("sdk_storage_guids").source;
    input.roots = GUIDS.iter().map(|(name, _)| (*name).into()).collect();
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        for (name, expected) in GUIDS {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            assert!(value.observations.iter().any(|observation| matches!(
                observation.kind,
                ValueKind::Constant { value: Some(ConstantValue::Guid(value)), .. } if value == expected
            )));
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.constants, 4);
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang::input_dir().join("sdk_storage_guids.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let declaration = Input {
        name: "storage_guid_declarations.h".into(),
        source: input.source.replace("#define INITGUID", "#undef INITGUID"),
        arguments: input.arguments.clone(),
        roots: input.roots.clone(),
        ..Default::default()
    };
    let declarations = capture(std::slice::from_ref(&declaration), &TARGETS).unwrap();
    assert!(projection::project(&merge(&declarations).unwrap(), "Fixture").is_err());
    let mut inputs = [input, declaration];
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert_eq!(
            projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap(),
            output
        );
        inputs.reverse();
    }
    check_metadata(&text, &GUIDS, "storage-guids");
}

#[test]
#[ignore = "requires the pinned Windows SDK and native companion"]
fn guid_macro_collisions() {
    helpers::ensure_libclang();
    let guids = [
        (
            "CLSID_WICImagingFactory",
            0xcacaf262_9370_4615_a13b_9f5539da4c0a,
        ),
        (
            "CLSID_WICPngDecoder",
            0x389ea17b_5078_4cde_b6ef_25c15175c751,
        ),
        ("GUID_CLASS_INPUT", 0x4d1e55b2_f16f_11cf_88cb_001111000030),
    ];
    let mut declarations = test_clang::annotated_sdk_fixture();
    declarations.name = "sdk_guid_macro_collisions.h".into();
    declarations.source = test_clang::fixture("sdk_guid_macro_collisions").source;
    declarations.roots = guids.iter().map(|(name, _)| (*name).into()).collect();
    declarations.macros.clone_from(&declarations.roots);
    let definitions = Input {
        name: "sdk_guid_macro_definitions.h".into(),
        source: format!("#define INITGUID\n{}", declarations.source),
        roots: declarations.roots.clone(),
        arguments: declarations.arguments.clone(),
        ..Default::default()
    };
    let mut inputs = [declarations, definitions];
    let mut expected = None;
    for _ in 0..2 {
        let captured = capture_partial(&inputs, &TARGETS).unwrap();
        for snapshot in &captured.snapshots {
            for value in &snapshot.values {
                for observation in &value.observations {
                    assert!(matches!(
                        observation.kind,
                        ValueKind::Constant {
                            origin: ConstantOrigin::Declaration,
                            ..
                        }
                    ));
                }
            }
        }
        let result = projection::project_partial(captured, "Fixture").unwrap();
        assert_eq!(result.output.constants, guids.len());
        for (name, _) in guids {
            assert!(
                result
                    .omissions
                    .iter()
                    .any(|message| message.contains(&format!("Macro(\"{name}\")"))),
                "{:?}",
                result.omissions
            );
            assert!(
                !result
                    .omissions
                    .iter()
                    .any(|message| message.contains(&format!("Value(\"{name}\")"))),
                "{:?}",
                result.omissions
            );
        }
        let text: String = result
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        check_metadata(&text, &guids, "guid-macro-collisions");
        if let Some(expected) = &expected {
            assert_eq!(&text, expected);
        } else {
            expected = Some(text);
        }
        inputs.reverse();
    }
    let golden = test_clang::input_dir().join("sdk_guid_macro_collisions.rdl");
    let text = expected.unwrap();
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
#[ignore = "requires the pinned Windows SDK and native companion"]
fn guid_macros_beside_native_declarations() {
    helpers::ensure_libclang();
    let guids = [
        ("GUID_NULL", 0),
        ("IID_ITransaction", 0x0fb15084_af41_11ce_bd2b_204c4f4f5020),
    ];
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = "sdk_guid_macro_declarations.h".into();
    input.source = test_clang::fixture("sdk_guid_macro_declarations").source;
    input.roots = guids.iter().map(|(name, _)| (*name).into()).collect();
    input.macros.clone_from(&input.roots);
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty(), "{:?}", captured.omissions);
    for snapshot in &captured.snapshots {
        for (name, expected) in guids {
            let kinds: Vec<_> = snapshot
                .values
                .iter()
                .filter(|value| value.name == name)
                .flat_map(|value| &value.observations)
                .map(|observation| &observation.kind)
                .collect();
            assert!(kinds.iter().any(|kind| matches!(
                kind,
                ValueKind::Constant {
                    origin: ConstantOrigin::Declaration,
                    value: None,
                    ..
                }
            )));
            assert!(kinds.iter().any(|kind| matches!(
                kind,
                ValueKind::Constant {
                    origin: ConstantOrigin::Macro,
                    value: Some(ConstantValue::Guid(value)),
                    ..
                } if *value == expected
            )));
        }
    }
    let result = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!(result.output.constants, guids.len());
    for (name, _) in guids {
        assert!(
            result.omissions.iter().any(|message| {
                message.contains(&format!("Value(\"{name}\")"))
                    && message.contains("has no captured definition")
            }),
            "{:?}",
            result.omissions
        );
        assert!(
            !result
                .omissions
                .iter()
                .any(|message| message.contains(&format!("Macro(\"{name}\")"))),
            "{:?}",
            result.omissions
        );
    }
    let text: String = result
        .output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang::input_dir().join("sdk_guid_macro_declarations.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    check_metadata(&text, &guids, "guid-macro-declarations");
}

fn check_metadata(text: &str, guids: &[(&str, u128)], name: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target").join(format!("test-clang-{name}.winmd"));
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
        for architecture in [1, 2, 4] {
            let index = reader::Index::new_for_architecture(
                vec![reader::File::read(path).unwrap()],
                architecture,
            );
            for &(name, expected) in guids {
                let field = index
                    .expect("Fixture", "Apis")
                    .fields()
                    .find(|field| field.name() == name)
                    .unwrap();
                let actual = field
                    .find_attribute("GuidAttribute")
                    .unwrap()
                    .value()
                    .into_iter()
                    .fold(0_u128, |value, (_, part)| match part {
                        Value::U32(part) => (value << 32) | u128::from(part),
                        Value::U16(part) => (value << 16) | u128::from(part),
                        Value::U8(part) => (value << 8) | u128::from(part),
                        _ => panic!(),
                    });
                assert_eq!(actual, expected);
            }
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}
