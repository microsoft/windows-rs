use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{HasAttributes, Type, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn enum_projection() {
    helpers::ensure_libclang();
    values();
    conflicts();
    partial();
}

fn values() {
    let mut inputs = [
        selected_fixture("enums", &[]),
        selected_fixture("enums", &[]),
    ];
    inputs[1].name = "second.h".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!((output.records, output.enums, output.aliases), (1, 11, 2));
    let rdl = &output.partitions["enum_types"].rdl;
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join("enum_types.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    inputs.reverse();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    assert_eq!(
        output,
        projection::generate_partial(&inputs, "Fixture")
            .unwrap()
            .output
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-enums.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for snapshot in &snapshots {
        let bit = match snapshot.target {
            Target::X86 => 1,
            Target::X64 => 2,
            Target::Arm64 => 4,
        };
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        for enumeration in &snapshot.enums {
            let name = enumeration.name.trim_start_matches('_');
            let definition = index.expect("Fixture", name);
            assert_eq!(definition.category(), reader::TypeCategory::Enum);
            assert!(
                !definition
                    .attributes()
                    .any(|attribute| attribute.name() == "FlagsAttribute")
            );
            let repr = definition.underlying_type().unwrap();
            let size = match repr {
                Type::I8 | Type::U8 => 1,
                Type::I16 | Type::U16 => 2,
                Type::I32 | Type::U32 => 4,
                Type::I64 | Type::U64 => 8,
                _ => panic!("unexpected backing type: {repr:?}"),
            };
            for observation in &enumeration.observations {
                assert_eq!(
                    definition
                        .attributes()
                        .any(|attribute| attribute.name() == "ScopedEnumAttribute"),
                    observation.scoped,
                );
                assert_eq!(
                    observation.layout,
                    Layout {
                        size,
                        alignment: size
                    }
                );
                let Some(members) = &observation.members else {
                    continue;
                };
                let metadata: Vec<_> = definition
                    .fields()
                    .filter_map(|field| {
                        field
                            .constant()
                            .map(|constant| (field.name(), constant.value()))
                    })
                    .collect();
                assert_eq!(members.len(), metadata.len());
                for (member, (name, value)) in members.iter().zip(metadata) {
                    assert_eq!(member.name, name);
                    let value = match value {
                        Value::I8(v) => EnumValue::Signed(v.into()),
                        Value::I16(v) => EnumValue::Signed(v.into()),
                        Value::I32(v) => EnumValue::Signed(v.into()),
                        Value::I64(v) => EnumValue::Signed(v),
                        Value::U8(v) => EnumValue::Unsigned(v.into()),
                        Value::U16(v) => EnumValue::Unsigned(v.into()),
                        Value::U32(v) => EnumValue::Unsigned(v.into()),
                        Value::U64(v) => EnumValue::Unsigned(v),
                        _ => panic!("unexpected enum value"),
                    };
                    assert_eq!(member.value, value);
                }
            }
        }
        assert_eq!(
            index.expect("Fixture", "StateAlias").underlying_type(),
            Some(Type::value_named("Fixture", "State"))
        );
        assert_eq!(
            index
                .expect("Fixture", "EnumUser")
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::value_named("Fixture", "State")
        );
    }
    let roundtrip = root.join("target\\test-clang-enums.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), *rdl);
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

fn conflicts() {
    for (prefix, expected) in [
        ("#define BASE long\n", "enum backing type"),
        ("#define BASE unsigned int\n", "enum backing type"),
        ("#define VALUE 2\n", "enum members differ"),
        ("#define SCOPE class\n", "enum backing type"),
        (
            "#if defined(_M_X64)\n#define VALUE 2\n#endif\n",
            "enum members differ",
        ),
    ] {
        let mut inputs = [
            selected_fixture("enum_conflict", &[]),
            selected_fixture("enum_conflict", &[]),
        ];
        inputs[1].name = "second.h".into();
        inputs[1].source.insert_str(0, prefix);
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let error = merge(&snapshots).err().unwrap().to_string();
        assert!(error.contains(expected), "{error}");
        assert!(
            projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string()
                .contains(expected)
        );
    }
    let mut inputs = [
        selected_fixture("enum_conflict", &[]),
        selected_fixture("enum_conflict", &[]),
    ];
    inputs[1].name = "second.h".into();
    inputs[1].source.insert_str(0, "#define FORWARD_ONLY\n");
    projection::generate(&inputs, "Fixture").unwrap();
    inputs[1]
        .source
        .insert_str(0, "#define BASE unsigned int\n");
    assert!(merge(&capture(&inputs, &TARGETS).unwrap()).is_err());
    inputs[1].source = "#define TAG Other\n#include \"enum_conflict_types.h\"\n".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(merged.enums.len(), 2);
    assert!(
        projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string()
            .contains("incompatible native bindings")
    );
}

fn partial() {
    let captured = capture_partial(&[selected_fixture("enum_partial", &[])], &TARGETS).unwrap();
    for snapshot in &captured.snapshots {
        assert_eq!(
            snapshot
                .records
                .iter()
                .map(|value| value.name.as_str())
                .collect::<Vec<_>>(),
            ["Safe", "SecondSafe"]
        );
        assert_eq!(
            snapshot
                .enums
                .iter()
                .map(|value| value.name.as_str())
                .collect::<Vec<_>>(),
            ["SafeEnum", "Wide"]
        );
    }
    let output = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!((output.output.records, output.output.enums), (2, 2));
    for name in [
        "Changing",
        "Dependent",
        "BadMember",
        "BadDependent",
        "BadPointer",
        "BackingEnum",
        "BackingUser",
    ] {
        assert!(
            output.omissions.iter().any(|value| value.contains(name)),
            "{name}"
        );
    }
    let captured = capture_partial(
        &[selected_fixture("enum_partial", &["Wide", "BadPointer"])],
        &TARGETS,
    )
    .unwrap();
    assert!(
        captured
            .snapshots
            .iter()
            .all(|snapshot| snapshot.records.is_empty())
    );
    let output = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!(
        (
            output.output.records,
            output.output.enums,
            output.output.aliases
        ),
        (0, 1, 0)
    );
}
