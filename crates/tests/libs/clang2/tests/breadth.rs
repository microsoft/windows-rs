use std::collections::BTreeMap;
use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;
use windows_metadata::reader::HasAttributes;
use windows_metadata::{Type as MetadataType, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

const BUFFER_CONTRACTS: [(&str, Option<&str>); 12] = [
    ("_In_reads_", Some("__in_ecount")),
    ("_In_reads_bytes_", Some("__in_bcount")),
    ("_In_reads_opt_", Some("__in_ecount_opt")),
    ("_In_reads_bytes_opt_", Some("__in_bcount_opt")),
    ("_Out_writes_", Some("__out_ecount")),
    ("_Out_writes_bytes_", Some("__out_bcount")),
    ("_Out_writes_opt_", Some("__out_ecount_opt")),
    ("_Out_writes_bytes_opt_", Some("__out_bcount_opt")),
    ("_Inout_updates_", Some("__inout_ecount")),
    ("_Inout_updates_bytes_", Some("__inout_bcount")),
    ("_Inout_updates_opt_", None),
    ("_Inout_updates_bytes_opt_", None),
];

const BUFFER_POSTCONDITIONS: [(&str, Option<&str>); 14] = [
    ("_In_z_", Some("__in_z")),
    ("_In_opt_z_", Some("__in_z_opt")),
    ("_Out_writes_z_", None),
    ("_Out_writes_opt_z_", None),
    ("_Inout_updates_z_", Some("__inout_ecount_z")),
    ("_Inout_updates_opt_z_", None),
    ("_Out_writes_to_", Some("__out_ecount_part")),
    ("_Out_writes_to_opt_", None),
    ("_Out_writes_bytes_to_", Some("__out_bcount_part")),
    ("_Out_writes_bytes_to_opt_", None),
    ("_Inout_updates_to_", Some("__inout_ecount_part")),
    ("_Inout_updates_to_opt_", None),
    ("_Inout_updates_bytes_to_", Some("__inout_bcount_part")),
    ("_Inout_updates_bytes_to_opt_", None),
];

const RPC_CONTRACTS: [(&str, &str, &str); 7] = [
    ("__RPC__out", "_Out_", ""),
    ("__RPC__inout", "_Inout_", ""),
    ("__RPC__inout_opt", "_Inout_opt_", ""),
    ("__RPC__inout_ecount", "_Inout_updates_", "capacity"),
    ("__RPC__inout_ecount_opt", "_Inout_updates_opt_", "capacity"),
    (
        "__RPC__inout_ecount_part",
        "_Inout_updates_to_",
        "capacity, written",
    ),
    (
        "__RPC__inout_ecount_part_opt",
        "_Inout_updates_to_opt_",
        "capacity, written",
    ),
];

const POINTER_CONTRACTS: [(&str, &str, reader::ParamDirection, bool); 24] = {
    use reader::ParamDirection::*;
    [
        ("1.2", "__RPC__deref_in", Input, false),
        ("1.2", "__RPC__deref_in_opt", Input, false),
        ("1.2", "__RPC__deref_opt_in", Input, true),
        ("1.2", "__RPC__deref_opt_in_opt", Input, true),
        ("1.2", "__RPC__deref_out", Output, false),
        ("1.2", "__RPC__deref_out_opt", Output, false),
        ("1.2", "__RPC__deref_inout", InputOutput, false),
        ("1.2", "__RPC__deref_inout_opt", InputOutput, false),
        ("1.2", "__RPC__deref_opt_inout", InputOutput, true),
        ("1.2", "__RPC__deref_opt_inout_opt", InputOutput, true),
        ("2", "_Outptr_", Output, false),
        ("2", "_Outptr_opt_", Output, true),
        ("2", "_Outptr_result_maybenull_", Output, false),
        ("2", "_Outptr_opt_result_maybenull_", Output, true),
        ("2", "_COM_Outptr_", Output, false),
        ("2", "_COM_Outptr_opt_", Output, true),
        ("2", "_COM_Outptr_result_maybenull_", Output, false),
        ("2", "_COM_Outptr_opt_result_maybenull_", Output, true),
        ("1", "__deref_out", Output, false),
        ("1", "__deref_out_opt", Output, false),
        ("1", "__deref_opt_out", Output, true),
        ("1", "__deref_opt_out_opt", Output, true),
        ("2", "_Outptr_result_nullonfailure_", Output, false),
        ("2", "_Outptr_opt_result_nullonfailure_", Output, true),
    ]
};

#[test]
fn uuid_tags() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("uuid_tags");
    let output = project(&[input]);
    assert_eq!(output.constants, 3);
    assert_eq!(output.records, 0);
    assert_eq!(output.aliases, 0);
    roundtrip("uuid_tags", &output, |index, bit| {
        let api = index.expect("Fixture", "Apis");
        for (name, guid) in [
            ("ClassTag", 0x123456789abcdef00102030405060708_u128),
            ("StructTag", 0xffffffffabcd123480010203040506ff),
            (
                "ArchTag",
                u128::from(if bit == 1 { 4_u32 } else { 8 }) << 96,
            ),
        ] {
            let field = api
                .fields()
                .find(|field| {
                    field.name() == name && (field.arches() == 0 || field.arches() & bit != 0)
                })
                .unwrap();
            let mut expected = vec![
                windows_metadata::Value::U32((guid >> 96) as u32),
                windows_metadata::Value::U16((guid >> 80) as u16),
                windows_metadata::Value::U16((guid >> 64) as u16),
            ];
            expected.extend(
                guid.to_be_bytes()[8..]
                    .iter()
                    .map(|byte| windows_metadata::Value::U8(*byte)),
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
    });
}

#[test]
fn uuid_tag_contracts() {
    helpers::ensure_libclang();
    let selected = selected_fixture("uuid_tags", &["ClassTag"]);
    assert_eq!(project(&[selected]).constants, 1);
    let mut inputs = [
        test_clang2::fixture("uuid_tags"),
        Input {
            name: "uuid_declarations.h".into(),
            source: "class ClassTag; struct StructTag; class ArchTag;".into(),
            ..Default::default()
        },
    ];
    let expected = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), expected);
    inputs[0].source = "struct ClassTag; class StructTag; class ArchTag;".into();
    assert_eq!(project(&inputs), expected);
    inputs[0].source =
        "class __declspec(uuid(\"00000000-0000-0000-0000-000000000000\")) ClassTag; \
        struct Bad { ClassTag* tag; const int invalid; };"
            .into();
    for _ in 0..2 {
        assert!(
            projection::generate(&inputs, "Fixture")
                .unwrap_err()
                .to_string()
                .contains("UUID differs")
        );
        assert!(
            projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string()
                .contains("UUID differs")
        );
        inputs.reverse();
    }
    let mut input = test_clang2::fixture("uuid_tags");
    input.source.push_str(
        "\ntypedef struct _GUID { unsigned long Data1; unsigned short Data2, Data3; unsigned char Data4[8]; } GUID;\n\
         #define StructTag __uuidof(struct StructTag)\n",
    );
    input.macros.push("StructTag".into());
    let output = project(&[input]);
    assert_eq!(output.constants, 3);
    assert_eq!(output.records, 1);
}

#[test]
fn uuid_tags_are_not_types_or_object_definitions() {
    helpers::ensure_libclang();
    for declaration in [
        "typedef ClassTag Renamed;",
        "struct Consumer { ClassTag* value; };",
        "class Plain;",
        "class ClassTag {};",
        "struct __declspec(align(16)) StructTag;",
        "const int ClassTag = 7;",
    ] {
        let mut input = test_clang2::fixture("uuid_tags");
        input.source = input.source.replace("typedef class ClassTag ClassTag;", "");
        input.source.push_str(declaration);
        assert!(
            projection::generate(&[input], "Fixture").is_err(),
            "{declaration}"
        );
    }
    let mut input = test_clang2::fixture("uuid_tags");
    input.source.push_str(
        "\nstruct GUID { unsigned long Data1; unsigned short Data2, Data3; unsigned char Data4[8]; };\
         extern const GUID CLSID_ClassTag;\
         typedef ClassTag Renamed;\
         struct Consumer { ClassTag* value; };",
    );
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 3);
    for name in ["CLSID_ClassTag", "Renamed", "Consumer"] {
        assert!(
            partial.omissions.iter().any(|error| error.contains(name)),
            "{name}"
        );
        assert!(
            !partial
                .output
                .partitions
                .values()
                .any(|partition| partition.rdl.contains(name))
        );
    }
    let mut input = test_clang2::fixture("uuid_tags");
    input.source = input.source.replace("typedef class ClassTag ClassTag;", "");
    input.source.push_str("\nextern const int ClassTag;");
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 2);
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("unpublishable native value"))
    );
    let mut input = test_clang2::fixture("uuid_tags");
    input.source.push_str("\nclass ClassTag {};");
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 2);
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("class definitions are unsupported"))
    );
}

fn imports() -> projection::Imports {
    [
        ("ReadPayload", "example.dll", Some("ReadPayloadEntry")),
        ("NativeSum", "example.dll", None),
        ("NormalizeFunction", "example.dll", None),
        ("BufferFunction", "example.dll", None),
        ("GetCurrentProcessId", "kernel32.dll", None),
        ("MetaDataGetDispenser", "RoMetadata.dll", None),
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

#[test]
fn annotation_normalization() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::fixture("annotation_normalization"),
        test_clang2::fixture("annotation_normalization"),
    ];
    inputs[1].name = "legacy.h".into();
    inputs[1].source.insert_str(0, "#define LEGACY\n");
    let expected = project(&inputs);
    for (input, version, name) in [(&inputs[0], "2", "_In_"), (&inputs[1], "1", "__in")] {
        let single = project(std::slice::from_ref(input));
        assert_eq!(single.partitions.len(), 1);
        assert_eq!(
            single.partitions.values().next().unwrap().rdl,
            expected.partitions.values().next().unwrap().rdl
        );
        for snapshot in capture(std::slice::from_ref(input), &TARGETS).unwrap() {
            let ValueKind::Function { parameters, .. } = &snapshot.values[0].observations[0].kind
            else {
                panic!()
            };
            let AttributeKind::Sal(value) = &parameters[0].attributes[0].kind else {
                panic!()
            };
            assert_eq!(value.version, version);
            assert_eq!(value.name, name);
        }
    }
    let mut repeated = test_clang2::fixture("annotation_normalization");
    repeated.source = repeated
        .source
        .replace("READ int* input", "READ READ int* input");
    assert_eq!(
        project(&[repeated]),
        project(std::slice::from_ref(&inputs[0]))
    );
    inputs.reverse();
    assert_eq!(project(&inputs), expected);
    assert_eq!(
        projection::project_partial_with_imports(
            capture_partial(&inputs, &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap()
        .output,
        expected
    );
    roundtrip("annotation_normalization", &expected, |index, _| {
        let apis = index.expect("Fixture", "Apis");
        let interface = index.expect("Fixture", "INormalized");
        let callback = index.expect("Fixture", "Normalizer");
        for method in [
            apis.methods().next().unwrap(),
            interface.methods().next().unwrap(),
            callback
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap(),
        ] {
            let parameters: Vec<_> = method.params().collect();
            assert_eq!(parameters.len(), 6);
            for (index, parameter) in parameters.iter().enumerate() {
                assert_eq!(parameter.is_optional(), index % 2 == 1);
                assert_eq!(
                    parameter.direction(),
                    match index / 2 {
                        0 => reader::ParamDirection::Input,
                        1 => reader::ParamDirection::Output,
                        _ => reader::ParamDirection::InputOutput,
                    }
                );
                assert!(parameter.find_attribute("NativeSalAttribute").is_none());
            }
        }
    });
}

#[test]
fn annotation_normalization_conflicts() {
    helpers::ensure_libclang();
    for root in ["Normalizer", "NormalizeFunction", "INormalized"] {
        for replacement in [
            "CONTRACT(\"_Inout_\", \"__inout\")",
            "CONTRACT(\"_In_opt_\", \"__in_opt\")",
        ] {
            let mut inputs = [
                selected_fixture("annotation_normalization", &[root]),
                selected_fixture("annotation_normalization", &[root]),
            ];
            inputs[1].name = "other.h".into();
            inputs[1].source = inputs[1]
                .source
                .replace("CONTRACT(\"_In_\", \"__in\")", replacement);
            for _ in 0..2 {
                let expected = match root {
                    "Normalizer" => "callback parameter annotations differ",
                    "NormalizeFunction" => "native value declarations differ",
                    _ => "native method names, slots, or attributes differ",
                };
                let error = projection::generate(&inputs, "Fixture").unwrap_err();
                assert!(error.to_string().contains(expected), "{error}");
                let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
                assert!(error.to_string().contains(expected), "{error}");
                inputs.reverse();
            }
        }
    }
    for qualifier in ["", "volatile "] {
        let mut inputs = [
            selected_fixture("annotation_normalization", &["Normalizer"]),
            selected_fixture("annotation_normalization", &["Normalizer"]),
        ];
        inputs[1].name = "other.h".into();
        for input in &mut inputs {
            input.source = input.source.replace(
                "READ int* input",
                &format!("READ CONTRACT(\"_Outptr_\", \"__out\") {qualifier}int* input"),
            );
        }
        inputs[1].source = inputs[1].source.replace(
            "CONTRACT(\"_In_\", \"__in\")",
            "CONTRACT(\"_Inout_\", \"__inout\")",
        );
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("callback parameter annotations differ"),
                "{error}"
            );
            inputs.reverse();
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn annotation_normalization_sdk() {
    helpers::ensure_libclang();
    for legacy in [false, true] {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "annotation_normalization.h".into();
        input.source = test_clang2::fixture("annotation_normalization").source;
        input.roots = ["NormalizeFunction", "Normalizer", "INormalized"]
            .map(String::from)
            .into();
        for (current, old) in [
            ("_In_", "__in"),
            ("_In_opt_", "__in_opt"),
            ("_Out_", "__out"),
            ("_Out_opt_", "__out_opt"),
            ("_Inout_", "__inout"),
            ("_Inout_opt_", "__inout_opt"),
        ] {
            input.source = input.source.replace(
                &format!("CONTRACT(\"{current}\", \"{old}\")"),
                if legacy { old } else { current },
            );
        }
        let output = project(&[input]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert_eq!(
            text,
            &std::fs::read_to_string(input_dir().join("annotation_normalization.rdl"))
                .unwrap()
                .replace("\r\n", "\n")
        );
        roundtrip_metadata(
            &format!("annotation_normalization_sdk_{legacy}"),
            text,
            |_, _| {},
        );
    }
}

#[test]
fn annotation_normalization_residuals() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("annotation_residuals");
    let output = projection::generate(&[input], "Fixture").unwrap();
    roundtrip("annotation_residuals", &output, |index, _| {
        // RDL defaults mutable pointers to output when no direction is emitted.
        for (name, count, direction) in [
            ("Conditional", 1, reader::ParamDirection::Output),
            ("Mixed", 2, reader::ParamDirection::InputOutput),
            ("NullableResult", 0, reader::ParamDirection::Output),
            ("ObjectInput", 0, reader::ParamDirection::Input),
            ("ObjectOutput", 1, reader::ParamDirection::Input),
            ("ObjectUpdate", 1, reader::ParamDirection::Input),
            ("Terminated", 0, reader::ParamDirection::Input),
            ("UnexpectedArguments", 1, reader::ParamDirection::Output),
        ] {
            let method = index
                .expect("Fixture", name)
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap();
            let parameter = method.params().next().unwrap();
            assert_eq!(parameter.direction(), direction, "{name}");
            assert!(!parameter.is_optional(), "{name}");
            assert_eq!(
                parameter
                    .attributes()
                    .filter(|attribute| attribute.name() == "NativeSalAttribute")
                    .count(),
                count,
                "{name}"
            );
            assert_eq!(
                parameter
                    .find_attribute("NativeReturnedPointerAttribute")
                    .is_some(),
                name == "NullableResult"
            );
        }
        let field = index
            .expect("Fixture", "FieldOwner")
            .fields()
            .next()
            .unwrap();
        assert!(field.find_attribute("NativeSalAttribute").is_some());
    });
    let mut inputs = [
        selected_fixture("annotation_residuals", &["Mixed"]),
        selected_fixture("annotation_residuals", &["Mixed"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source = inputs[1].source.replace(
        "SAL(\"_In_\", \"\") SAL(\"_Out_\", \"\")",
        "SAL(\"_Inout_\", \"\")",
    );
    for _ in 0..2 {
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
}

#[test]
fn buffer_normalization() {
    helpers::ensure_libclang();
    for (case, (current, legacy)) in BUFFER_CONTRACTS.into_iter().enumerate() {
        let mut input = test_clang2::fixture("buffer_normalization");
        input
            .source
            .insert_str(0, &format!("#define ANNOTATION \"{current}\"\n"));
        let output = project(std::slice::from_ref(&input));
        let text = &output.partitions.values().next().unwrap().rdl;
        if case == 0 {
            if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
                std::fs::write(input_dir().join("buffer_normalization.rdl"), text).unwrap();
            }
            assert_eq!(
                text,
                &std::fs::read_to_string(input_dir().join("buffer_normalization.rdl"))
                    .unwrap()
                    .replace("\r\n", "\n")
            );
        }
        if let Some(legacy) = legacy {
            let mut other = test_clang2::fixture("buffer_normalization");
            other.name = "legacy.h".into();
            other.source.insert_str(
                0,
                &format!("#define VERSION \"1\"\n#define ANNOTATION \"{legacy}\"\n"),
            );
            assert_eq!(
                &project(std::slice::from_ref(&other))
                    .partitions
                    .values()
                    .next()
                    .unwrap()
                    .rdl,
                text
            );
            let mut inputs = [input, other];
            for _ in 0..2 {
                assert_eq!(
                    &project(&inputs).partitions.values().next().unwrap().rdl,
                    text
                );
                inputs.reverse();
            }
        }
        roundtrip_metadata(&format!("buffer_normalization_{case}"), text, |index, _| {
            for (owner, name) in [
                ("Apis", "BufferFunction"),
                ("Buffer", "Invoke"),
                ("IBuffer", "Read"),
            ] {
                let method = index
                    .expect("Fixture", owner)
                    .methods()
                    .find(|method| method.name() == name)
                    .unwrap();
                let parameter = method.params().next().unwrap();
                assert_eq!(parameter.is_optional(), case % 4 >= 2);
                assert_eq!(
                    parameter.direction(),
                    match case / 4 {
                        0 => reader::ParamDirection::Input,
                        1 => reader::ParamDirection::Output,
                        _ => reader::ParamDirection::InputOutput,
                    }
                );
                assert_eq!(
                    parameter.buffer_relationship(),
                    Some(if case % 2 == 0 {
                        reader::BufferRelationship::ElementsParam(1)
                    } else {
                        reader::BufferRelationship::BytesParam(1)
                    })
                );
                assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                let entry = parameter.find_attribute("NativeInputAttribute");
                assert_eq!(entry.is_some(), case < 4);
                if let Some(entry) = entry {
                    assert_eq!(
                        entry.value().into_iter().collect::<BTreeMap<_, _>>(),
                        [("ReadOnly".into(), windows_metadata::Value::Bool(true))].into()
                    );
                }
            }
        });
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn buffer_normalization_sdk() {
    helpers::ensure_libclang();
    for (current, legacy) in BUFFER_CONTRACTS {
        let mut expected = test_clang2::fixture("buffer_normalization");
        expected
            .source
            .insert_str(0, &format!("#define ANNOTATION \"{current}\"\n"));
        let expected = project(&[expected]);
        for name in Some(current).into_iter().chain(legacy) {
            let mut input = test_clang2::annotated_sdk_fixture();
            input.name = "buffer_normalization.h".into();
            input.source = test_clang2::fixture("buffer_normalization").source.replace(
                "BUFFER unsigned short*",
                &format!("{name}(BOUND) unsigned short*"),
            );
            input.roots = ["Buffer", "BufferFunction", "IBuffer"]
                .map(String::from)
                .into();
            let output = project(&[input]);
            assert_eq!(
                output.partitions.values().next().unwrap().rdl,
                expected.partitions.values().next().unwrap().rdl,
                "{name}"
            );
        }
    }
    for name in ["__inout_ecount_opt", "__inout_bcount_opt"] {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "buffer_normalization.h".into();
        input.source = test_clang2::fixture("buffer_normalization").source.replace(
            "BUFFER unsigned short*",
            &format!("{name}(BOUND) unsigned short*"),
        );
        input.roots = vec!["Buffer".into()];
        let output = project(&[input]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert!(
            text.contains(&format!("NativeSal(\"1\", \"{name}\"")),
            "{text}"
        );
        assert!(
            !text.contains("len_param") && !text.contains("size_param"),
            "{text}"
        );
    }
}

#[test]
fn buffer_normalization_conflicts() {
    helpers::ensure_libclang();
    for root in ["Buffer", "BufferFunction", "IBuffer"] {
        for change in [
            "#define BOUND other\n",
            "#define ANNOTATION \"_In_reads_bytes_\"\n",
            "#define ANNOTATION \"_In_reads_opt_\"\n",
        ] {
            let mut inputs = [
                selected_fixture("buffer_normalization", &[root]),
                selected_fixture("buffer_normalization", &[root]),
            ];
            inputs[1].name = "other.h".into();
            inputs[1].source.insert_str(0, change);
            for _ in 0..2 {
                let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
                let expected = match root {
                    "Buffer" => "callback parameter annotations differ",
                    "BufferFunction" => "native value declarations differ",
                    _ => "native method names, slots, or attributes differ",
                };
                assert!(error.to_string().contains(expected), "{error}");
                inputs.reverse();
            }
        }
    }
    let mut inputs = [
        selected_fixture("buffer_normalization", &["BufferFunction"]),
        selected_fixture("buffer_normalization", &["BufferFunction"]),
    ];
    inputs[1].name = "renamed.h".into();
    inputs[1].source.insert_str(0, "#define COUNT size\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        merge(&snapshots).unwrap();
        let output = projection::project_partial_with_imports(
            capture_partial(&inputs, &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        assert_eq!(output.output.functions, 1);
        assert!(output.omissions.is_empty());
        inputs.reverse();
    }
    for (version, annotation) in [("2", "_In_reads_"), ("1", "__in_ecount")] {
        let mut input = selected_fixture("buffer_normalization", &["BufferFunction"]);
        input.source.insert_str(0, &format!(
            "#define COUNT_TYPE float\n#define VERSION \"{version}\"\n#define ANNOTATION \"{annotation}\"\n"
        ));
        let output = projection::project_partial_with_imports(
            capture_partial(&[input], &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        assert_eq!(output.output.functions, 0);
        assert!(
            output
                .omissions
                .iter()
                .any(|error| error.contains("requires a by-value integer"))
        );
    }
}

fn postcondition_input(name: &str, arguments: &str, indirect: bool) -> Input {
    let mut input = test_clang2::fixture("buffer_postconditions");
    input.source.insert_str(
        0,
        &format!(
            "#define ANNOTATION \"{name}\"\n#define ARGUMENTS \"{arguments}\"\n{}",
            if indirect {
                "#define LENGTH_TYPE unsigned int*\n"
            } else {
                ""
            }
        ),
    );
    input
}

const FULL_BUFFERS: [(&str, &str); 8] = [
    ("_Out_writes_all_", "_Out_writes_to_"),
    ("_Out_writes_all_opt_", "_Out_writes_to_opt_"),
    ("_Out_writes_bytes_all_", "_Out_writes_bytes_to_"),
    ("_Out_writes_bytes_all_opt_", "_Out_writes_bytes_to_opt_"),
    ("_Inout_updates_all_", "_Inout_updates_to_"),
    ("_Inout_updates_all_opt_", "_Inout_updates_to_opt_"),
    ("_Inout_updates_bytes_all_", "_Inout_updates_bytes_to_"),
    (
        "_Inout_updates_bytes_all_opt_",
        "_Inout_updates_bytes_to_opt_",
    ),
];

#[test]
fn buffer_full() {
    helpers::ensure_libclang();
    for (full, partial) in FULL_BUFFERS {
        let input = postcondition_input(full, "capacity", false);
        let mut expected = postcondition_input(partial, "capacity, capacity", false);
        let output = project(std::slice::from_ref(&input));
        assert_eq!(
            output.partitions["buffer_postconditions"].rdl,
            project(std::slice::from_ref(&expected)).partitions["buffer_postconditions"].rdl,
            "{full}"
        );
        if full == "_Out_writes_bytes_all_" {
            assert_eq!(
                output.partitions["buffer_postconditions"].rdl,
                std::fs::read_to_string(input_dir().join("buffer_full.rdl"))
                    .unwrap()
                    .replace("\r\n", "\n")
            );
        }
        expected.name = "other.h".into();
        let mut inputs = [input, expected];
        for _ in 0..2 {
            assert_eq!(
                project(&inputs).partitions.values().next().unwrap().rdl,
                output.partitions["buffer_postconditions"].rdl
            );
            inputs.reverse();
        }
    }
    for count in ["capacity + 1", "*written", "16"] {
        let output = project(&[postcondition_input("_Out_writes_bytes_all_", count, false)]);
        let rdl = &output.partitions["buffer_postconditions"].rdl;
        assert!(rdl.contains("NativeSal(") && rdl.contains("#[out]"));
        assert!(!rdl.contains("size_param") && !rdl.contains("NativeInitializedLength"));
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn buffer_full_sdk() {
    helpers::ensure_libclang();
    for (full, partial) in FULL_BUFFERS {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "buffer_postconditions.h".into();
        input.source = test_clang2::fixture("buffer_postconditions")
            .source
            .replace(
                "BUFFER unsigned short*",
                &format!("{full}(capacity) unsigned short*"),
            );
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        assert_eq!(project(&[input]).partitions["buffer_postconditions"].rdl,
            project(&[postcondition_input(partial, "capacity, capacity", false)])
                .partitions["buffer_postconditions"].rdl, "{full}");
    }
}

fn returned_buffer_contracts() -> Vec<(String, String, bool)> {
    let mut contracts = Vec::new();
    for units in ["buffer", "bytebuffer"] {
        for suffix in [
            "",
            "_to",
            "_all",
            "_maybenull",
            "_to_maybenull",
            "_all_maybenull",
        ] {
            for optional in [false, true] {
                contracts.push((
                    format!(
                        "_Outptr{}_result_{units}{suffix}_",
                        if optional { "_opt" } else { "" }
                    ),
                    if suffix.starts_with("_to") {
                        "capacity, *written"
                    } else {
                        "*written"
                    }
                    .into(),
                    optional,
                ));
            }
        }
    }
    contracts
}

#[test]
fn returned_buffer_flags() {
    helpers::ensure_libclang();
    for (name, arguments, optional) in returned_buffer_contracts() {
        let mut input = postcondition_input(&name, &arguments, true);
        input.source = input
            .source
            .replace("BUFFER unsigned short*", "BUFFER unsigned short**");
        let output = project(&[input]);
        let rdl = &output.partitions["buffer_postconditions"].rdl;
        assert_eq!(rdl.matches("#[out]").count(), 3, "{name}");
        assert_eq!(
            rdl.matches("#[opt]").count(),
            if optional { 3 } else { 0 },
            "{name}"
        );
        assert_eq!(rdl.matches("NativeSal(").count(), 3, "{name}");
        assert!(
            !rdl.contains("size_param")
                && !rdl.contains("len_param")
                && !rdl.contains("NativeInitializedLength"),
            "{rdl}"
        );
        if name == "_Outptr_opt_result_bytebuffer_all_maybenull_" {
            roundtrip("returned_buffer", &output, |index, _| {
                let method = index.expect("Fixture", "Apis").methods().next().unwrap();
                let parameter = method.params().next().unwrap();
                assert_eq!(parameter.direction(), reader::ParamDirection::Output);
                assert!(parameter.is_optional());
                assert!(parameter.buffer_relationship().is_none());
            });
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn returned_buffer_flags_sdk() {
    helpers::ensure_libclang();
    for (name, arguments, _) in returned_buffer_contracts() {
        let mut expected = postcondition_input(&name, &arguments, true);
        expected.source = expected
            .source
            .replace("BUFFER unsigned short*", "BUFFER unsigned short**");
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = expected.name.clone();
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        input.source = expected.source.replace(
            "BUFFER unsigned short**",
            &format!("{name}({arguments}) unsigned short**"),
        );
        assert_eq!(
            project(&[input]).partitions["buffer_postconditions"].rdl,
            project(&[expected]).partitions["buffer_postconditions"].rdl,
            "{name}"
        );
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_bcrypt() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_bcrypt.h".into();
    input.source = test_clang2::fixture("sdk_bcrypt").source;
    input.roots = [
        "BCryptDeleteContext",
        "BCryptFinishHash",
        "BCryptEnumContexts",
    ]
    .map(String::from)
    .into();
    let imports = input
        .roots
        .iter()
        .map(|name| {
            (
                name.clone(),
                projection::Import {
                    library: "bcrypt.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert_eq!(rdl.matches("fn BCryptDeleteContext(").count(), 1);
    assert_eq!(rdl.matches("fn BCryptFinishHash(").count(), 1);
    assert!(rdl.contains("extern fn BCryptDeleteContext("));
    assert!(rdl.contains("#[out] #[opt] ppBuffer: *mut PCRYPT_CONTEXTS"));
    assert!(rdl.contains("#[out] #[size_param(2)] #[Windows::Win32::Metadata::NativeInitializedLength(CountParamIndex = 2, IsByteCount = true, Dereference = false)] pbOutput: PUCHAR"));
    roundtrip("sdk_bcrypt", &output, |index, _| {
        let method = index
            .expect("Fixture", "Apis")
            .methods()
            .find(|method| method.name() == "BCryptFinishHash")
            .unwrap();
        let buffer = method.params().nth(1).unwrap();
        assert_eq!(buffer.direction(), reader::ParamDirection::Output);
        assert_eq!(
            buffer.buffer_relationship(),
            Some(reader::BufferRelationship::BytesParam(2))
        );
    });
}

#[test]
fn pointer_parameter_flags() {
    helpers::ensure_libclang();
    for (case, (version, name, direction, optional)) in POINTER_CONTRACTS.into_iter().enumerate() {
        let input = pointer_contract_input(version, name, "");
        let output = project(&[input]);
        let text = &output.partitions.values().next().unwrap().rdl;
        roundtrip_metadata(&format!("pointer_flags_{case}"), text, |index, _| {
            for (owner, method) in [
                ("Apis", "BufferFunction"),
                ("Buffer", "Invoke"),
                ("IBuffer", "Read"),
            ] {
                let method = index
                    .expect("Fixture", owner)
                    .methods()
                    .find(|value| value.name() == method)
                    .unwrap();
                let parameter = method.params().next().unwrap();
                assert_eq!(parameter.direction(), direction, "{name}");
                assert_eq!(parameter.is_optional(), optional, "{name}");
                if direction == reader::ParamDirection::Output {
                    assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                    assert_eq!(
                        parameter
                            .find_attribute("NativeReturnedPointerAttribute")
                            .unwrap()
                            .value(),
                        [
                            (
                                "MayBeNull".into(),
                                windows_metadata::Value::Bool(
                                    name.contains("maybenull")
                                        || (version == "1" && name.ends_with("_opt")),
                                ),
                            ),
                            (
                                "NullOnFailure".into(),
                                windows_metadata::Value::Bool(
                                    name.starts_with("_COM_") || name.contains("nullonfailure"),
                                ),
                            ),
                        ]
                    );
                } else {
                    let evidence = parameter
                        .find_attribute("NativeSalAttribute")
                        .unwrap()
                        .value();
                    assert_eq!(evidence[0].1, windows_metadata::Value::Utf8(version.into()));
                    assert_eq!(evidence[1].1, windows_metadata::Value::Utf8(name.into()));
                }
            }
        });
        let wrong_version = if version == "1" { "2" } else { "1" };
        for (version, arguments) in [(wrong_version, ""), (version, "unexpected")] {
            let text = project(&[pointer_contract_input(version, name, arguments)])
                .partitions
                .into_values()
                .next()
                .unwrap()
                .rdl;
            assert!(text.contains("NativeSal("), "{text}");
            for flag in ["#[in]", "#[out]", "#[opt]"] {
                assert!(!text.contains(flag), "{text}");
            }
        }
    }
    for (left, right) in [
        ("__RPC__deref_in", "__RPC__deref_in_opt"),
        ("__RPC__deref_inout", "__RPC__deref_inout_opt"),
        ("_Outptr_", "_Outptr_result_maybenull_"),
        ("_Outptr_", "_COM_Outptr_"),
        ("_Outptr_", "_Outptr_result_nullonfailure_"),
        ("_COM_Outptr_", "_Outptr_result_nullonfailure_"),
    ] {
        let version = if left.starts_with("__RPC") {
            "1.2"
        } else {
            "2"
        };
        let mut inputs = [
            pointer_contract_input(version, left, ""),
            pointer_contract_input(version, right, ""),
        ];
        inputs[1].name = "other.h".into();
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains("differ"), "{error}");
            inputs.reverse();
        }
    }
}

#[test]
fn sal1_returned_pointer_agreement() {
    helpers::ensure_libclang();
    for (legacy, current) in [
        ("__deref_out", "_Outptr_"),
        ("__deref_out_opt", "_Outptr_result_maybenull_"),
        ("__deref_opt_out", "_Outptr_opt_"),
        ("__deref_opt_out_opt", "_Outptr_opt_result_maybenull_"),
    ] {
        let mut inputs = [
            pointer_contract_input("1", legacy, ""),
            pointer_contract_input("2", current, ""),
        ];
        inputs[1].name = "other.h".into();
        let expected = project(&inputs);
        for _ in 0..2 {
            assert_eq!(project(&inputs), expected);
            inputs.reverse();
        }
    }
    for name in ["__deref_out_opt", "__deref_opt_out"] {
        let mut inputs = [
            pointer_contract_input("1", "__deref_out", ""),
            pointer_contract_input("1", name, ""),
        ];
        inputs[1].name = "other.h".into();
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains("differ"), "{error}");
            inputs.reverse();
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_rometadata() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_rometadata.h".into();
    input.source = test_clang2::fixture("sdk_rometadata").source;
    input.roots = vec!["MetaDataGetDispenser".into()];
    let output = project(&[input]);
    roundtrip("sdk_rometadata", &output, |index, _| {
        let method = index
            .expect("Fixture", "Apis")
            .methods()
            .find(|method| method.name() == "MetaDataGetDispenser")
            .unwrap();
        let result = method.params().last().unwrap();
        assert_eq!(result.direction(), reader::ParamDirection::Output);
        assert!(!result.is_optional());
        assert!(result.find_attribute("NativeSalAttribute").is_none());
        assert!(result.find_attribute("ComOutPtrAttribute").is_none());
        assert_eq!(
            result
                .find_attribute("NativeReturnedPointerAttribute")
                .unwrap()
                .value(),
            [
                ("MayBeNull".into(), windows_metadata::Value::Bool(false)),
                ("NullOnFailure".into(), windows_metadata::Value::Bool(false)),
            ]
        );
    });
}

#[test]
fn com_output() {
    helpers::ensure_libclang();
    for (version, annotation, com) in [
        ("2", "_COM_Outptr_", true),
        ("2", "_COM_Outptr_opt_", true),
        ("2", "_COM_Outptr_result_maybenull_", true),
        ("2", "_COM_Outptr_opt_result_maybenull_", true),
        ("2", "_Outptr_", false),
        ("2", "_Outptr_opt_", false),
        ("2", "_Outptr_result_nullonfailure_", false),
        ("2", "_Outptr_opt_result_nullonfailure_", false),
        ("1.2", "__RPC__deref_out_opt", false),
        ("1", "_COM_Outptr_", false),
    ] {
        let mut input = test_clang2::fixture("com_output");
        input.source.insert_str(
            0,
            &format!("#define VERSION \"{version}\"\n#define ANNOTATION \"{annotation}\"\n"),
        );
        let output = project(&[input]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert_eq!(text.matches("#[iid_is]").count(), if com { 6 } else { 0 });
        if version == "2" && annotation == "_COM_Outptr_" {
            roundtrip("com_output", &output, |index, _| {
                for (owner, name) in [
                    ("Apis", "BufferFunction"),
                    ("Buffer", "Invoke"),
                    ("IBuffer", "Read"),
                ] {
                    let method = index
                        .expect("Fixture", owner)
                        .methods()
                        .find(|m| m.name() == name)
                        .unwrap();
                    for parameter in method.params() {
                        assert_eq!(
                            parameter.find_attribute("ComOutPtrAttribute").is_some(),
                            matches!(parameter.name(), "raw" | "alias")
                        );
                    }
                }
            });
        }
    }
}

#[test]
fn input_contracts() {
    helpers::ensure_libclang();
    for (case, (version, name, read_only, optional)) in [
        ("1.2", "__RPC__in", false, false),
        ("1.2", "__RPC__in_opt", false, true),
        ("2", "_In_", true, false),
        ("2", "_In_opt_", true, true),
        ("1", "__in", true, false),
        ("1", "__in_opt", true, true),
    ]
    .into_iter()
    .enumerate()
    {
        let mut input = test_clang2::fixture("input_contracts");
        input.source.insert_str(
            0,
            &format!(
                "#define INPUT __attribute__((annotate(\"windows_clang:sal:{version}:{name}\\n()\\n()\")))\n"
            ),
        );
        let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
        for snapshot in &snapshots {
            let ValueKind::Function { parameters, .. } = &snapshot.values[0].observations[0].kind
            else {
                panic!()
            };
            for parameter in parameters {
                let AttributeKind::Sal(annotation) = &parameter.attributes[0].kind else {
                    panic!()
                };
                assert_eq!(annotation.version, version);
                assert_eq!(annotation.name, name);
            }
        }
        let output =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
                .unwrap();
        let text = &output.partitions.values().next().unwrap().rdl;
        assert!(!text.contains("NativeSal("), "{text}");
        let check = |index: &reader::Index, _| {
            for (owner, method) in [
                ("Apis", "BufferFunction"),
                ("Buffer", "Invoke"),
                ("IBuffer", "Read"),
            ] {
                let method = index
                    .expect("Fixture", owner)
                    .methods()
                    .find(|m| m.name() == method)
                    .unwrap();
                assert_eq!(method.params().count(), 3);
                for parameter in method.params() {
                    assert_eq!(parameter.direction(), reader::ParamDirection::Input);
                    assert_eq!(parameter.is_optional(), optional);
                    assert_eq!(
                        parameter
                            .find_attribute("NativeInputAttribute")
                            .unwrap()
                            .value(),
                        [("ReadOnly".into(), windows_metadata::Value::Bool(read_only))]
                    );
                    assert!(
                        parameter
                            .find_attribute("NativeArrayInfoAttribute")
                            .is_none()
                    );
                }
            }
        };
        if case == 0 {
            roundtrip("input_contracts", &output, check);
            input.source = input.source.replace("INPUT void*", "INPUT INPUT void*");
            assert_eq!(project(&[input]), output);
        } else {
            roundtrip_metadata(&format!("input_contracts_{case}"), text, check);
        }
    }
    let mut input = test_clang2::fixture("input_contracts");
    input.source = input.source.replace(
        "INPUT void*",
        "INPUT __attribute__((annotate(\"windows_clang:sal:2:_In_\\n()\\n()\"))) void*",
    );
    let text = project(&[input])
        .partitions
        .into_values()
        .next()
        .unwrap()
        .rdl;
    assert_eq!(text.matches("NativeSal(").count(), 6);
    assert_eq!(text.matches("NativeInput(").count(), 6);
}

#[test]
fn midl_comments() {
    helpers::ensure_libclang();
    let output = project(&[test_clang2::fixture("midl_comments")]);
    roundtrip("midl_comments", &output, |index, _| {
        let structural = index
            .expect("Fixture", "CommentCallbackStorage")
            .fields()
            .find(|field| field.name() == "structural")
            .unwrap();
        assert!(matches!(structural.ty(), MetadataType::FnPtr(_)));
        for (owner, method, position, iid, retval) in [
            ("Apis", "BufferFunction", 1, true, true),
            ("Buffer", "Invoke", 0, true, false),
            ("IBuffer", "Read", 0, false, true),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|m| m.name() == method)
                .unwrap();
            for (i, parameter) in method.params().enumerate() {
                assert_eq!(
                    parameter.find_attribute("ComOutPtrAttribute").is_some(),
                    i == position && iid
                );
                assert_eq!(
                    parameter.find_attribute("RetValAttribute").is_some(),
                    i == position && retval
                );
                if i == position {
                    assert_eq!(parameter.direction(), reader::ParamDirection::Output);
                }
            }
        }
    });
}

#[test]
#[ignore = "requires the pinned SDK"]
fn input_contracts_sdk() {
    helpers::ensure_libclang();
    for (version, name) in [
        ("1.2", "__RPC__in"),
        ("1.2", "__RPC__in_opt"),
        ("2", "_In_"),
        ("2", "_In_opt_"),
        ("1", "__in"),
        ("1", "__in_opt"),
    ] {
        let mut expected = test_clang2::fixture("input_contracts");
        expected.source.insert_str(0, &format!(
            "#define INPUT __attribute__((annotate(\"windows_clang:sal:{version}:{name}\\n()\\n()\")))\n"
        ));
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = expected.name.clone();
        input.source = format!(
            "#include <rpcsal.h>\n#define INPUT {name}\n{}",
            test_clang2::fixture("input_contracts").source
        );
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        assert_eq!(project(&[input]), project(&[expected]));
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_accounts_settings() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_accounts_settings.h".into();
    input.source = test_clang2::fixture("sdk_accounts_settings").source;
    input.roots = vec!["IAccountsSettingsPaneInterop".into()];
    let output = project(&[input]);
    roundtrip("sdk_accounts_settings", &output, |index, _| {
        let interface = index.expect("Fixture", "IAccountsSettingsPaneInterop");
        assert_eq!(interface.methods().count(), 3);
        for method in interface.methods() {
            for parameter in method.params().take(2) {
                assert_eq!(parameter.direction(), reader::ParamDirection::Input);
                assert!(!parameter.is_optional());
                assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                assert_eq!(
                    parameter
                        .find_attribute("NativeInputAttribute")
                        .unwrap()
                        .value(),
                    [("ReadOnly".into(), windows_metadata::Value::Bool(false))]
                );
            }
            let result = method.params().last().unwrap();
            assert!(result.find_attribute("ComOutPtrAttribute").is_some());
            assert!(result.find_attribute("RetValAttribute").is_some());
        }
    });
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_audio_client() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_audio_client.h".into();
    input.source = test_clang2::fixture("sdk_audio_client").source;
    input.roots = vec!["IAudioClient".into()];
    let output = project(&[input]);
    roundtrip("sdk_audio_client", &output, |index, _| {
        let method = index
            .expect("Fixture", "IAudioClient")
            .methods()
            .find(|m| m.name() == "GetService")
            .unwrap();
        let result = method.params().last().unwrap();
        assert!(result.find_attribute("ComOutPtrAttribute").is_some());
        assert!(result.find_attribute("RetValAttribute").is_none());
        assert_eq!(result.direction(), reader::ParamDirection::Output);
    });
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_midl_inputs() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_midl_inputs.h".into();
    input.source = test_clang2::fixture("sdk_midl_inputs").source;
    input.roots = [
        "IBindStatusCallback",
        "ITfSource",
        "ITfSourceSingle",
        "ICommandStream",
    ]
    .map(String::from)
    .into();
    let output = project(&[input]);
    roundtrip("sdk_midl_inputs", &output, |index, _| {
        for (owner, method, name) in [
            ("IBindStatusCallback", "OnObjectAvailable", "punk"),
            ("ITfSource", "AdviseSink", "punk"),
            ("ITfSourceSingle", "AdviseSingleSink", "punk"),
            ("ICommandStream", "SetCommandStream", "pCommandStream"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|m| m.name() == method)
                .unwrap();
            let parameter = method.params().find(|p| p.name() == name).unwrap();
            assert_eq!(parameter.direction(), reader::ParamDirection::Input);
            assert!(parameter.find_attribute("ComOutPtrAttribute").is_none());
            assert!(parameter.find_attribute("RetValAttribute").is_none());
        }
    });
}

#[test]
#[ignore = "requires the pinned SDK"]
fn com_output_sdk() {
    helpers::ensure_libclang();
    for annotation in [
        "_COM_Outptr_",
        "_COM_Outptr_opt_",
        "_COM_Outptr_result_maybenull_",
        "_COM_Outptr_opt_result_maybenull_",
    ] {
        let mut expected = test_clang2::fixture("com_output");
        expected
            .source
            .insert_str(0, &format!("#define ANNOTATION \"{annotation}\"\n"));
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = expected.name.clone();
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        input.source = format!(
            "#include <sal.h>\n{}",
            expected
                .source
                .replace(
                    "#define PARAMETERS OUTPUT",
                    &format!("#define PARAMETERS {annotation}")
                )
                .replace(", OUTPUT ", &format!(", {annotation} "))
        );
        assert_eq!(project(&[input]), project(&[expected]));
    }
}

fn pointer_contract_input(version: &str, name: &str, arguments: &str) -> Input {
    let mut input = postcondition_input(name, arguments, false);
    input
        .source
        .insert_str(0, &format!("#define VERSION \"{version}\"\n"));
    input.source = input
        .source
        .replace("unsigned short* buffer", "unsigned short** buffer");
    input
}

#[test]
fn returned_pointer_contracts() {
    helpers::ensure_libclang();
    let input = pointer_contract_input("1.2", "__RPC__deref_out_opt", "");
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let observation = &snapshot
            .values
            .iter()
            .find(|value| value.name == "BufferFunction")
            .unwrap()
            .observations[0];
        let ValueKind::Function { parameters, .. } = &observation.kind else {
            panic!();
        };
        assert!(parameters[0].attributes.iter().any(|attribute| {
            matches!(&attribute.kind, AttributeKind::Sal(value)
                if value.version == "1.2" && value.name == "__RPC__deref_out_opt")
        }));
    }
    let output = project(std::slice::from_ref(&input));
    roundtrip("returned_pointer", &output, |_, _| {});
    for (version, name) in [("1.2", "__RPC__deref_out"), ("2", "_Outptr_")] {
        let mut other = pointer_contract_input(version, name, "");
        assert_eq!(project(std::slice::from_ref(&other)), output);
        other.name = "other.h".into();
        let mut inputs = [
            pointer_contract_input("1.2", "__RPC__deref_out_opt", ""),
            other,
        ];
        for _ in 0..2 {
            assert_eq!(
                project(&inputs).partitions.values().next().unwrap().rdl,
                output.partitions.values().next().unwrap().rdl
            );
            inputs.reverse();
        }
    }
    // SAL1's optional returned value is not the RPC alias's non-null result.
    let optional = project(&[pointer_contract_input("1", "__deref_out_opt", "")]);
    assert_eq!(
        optional,
        project(&[pointer_contract_input("2", "_Outptr_result_maybenull_", "")])
    );
    assert_ne!(optional, output);
    let mut mixed = pointer_contract_input("2", "_Outptr_opt_", "");
    mixed.source = mixed.source.replace(
        "BUFFER unsigned short**",
        "BUFFER __attribute__((annotate(\"windows_clang:sal:2:_Inout_\\n()\\n()\"))) unsigned short**",
    );
    let output = project(&[mixed]);
    let text = &output.partitions.values().next().unwrap().rdl;
    assert_eq!(text.matches("NativeSal(").count(), 6);
    assert_eq!(text.matches("\"_Inout_\"").count(), 3);
    assert_eq!(text.matches("\"_Outptr_opt_\"").count(), 3);
    assert!(!text.contains("NativeReturnedPointer("));
    roundtrip("returned_pointer_mixed", &output, |_, _| {});
    for ty in ["unsigned short", "unsigned short*", "unsigned short&"] {
        let mut invalid = pointer_contract_input("1.2", "__RPC__deref_out_opt", "");
        invalid.source = invalid
            .source
            .replace("unsigned short** buffer", &format!("{ty} buffer"));
        let snapshots = capture(&[invalid], &TARGETS).unwrap();
        let error =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
                .unwrap_err();
        assert!(
            error.to_string().contains("pointer-to-pointer storage"),
            "{error}"
        );
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_background_copy_error() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_background_copy_error.h".into();
    input.source = test_clang2::fixture("sdk_background_copy_error").source;
    input.roots = vec!["IBackgroundCopyError".into()];
    let output = project(&[input]);
    roundtrip("sdk_background_copy_error", &output, |index, _| {
        for method in index.expect("Fixture", "IBackgroundCopyError").methods() {
            for parameter in method.params() {
                assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                if method.name() == "GetError" || parameter.name() == "LanguageId" {
                    assert!(
                        parameter
                            .find_attribute("NativeReturnedPointerAttribute")
                            .is_none()
                    );
                } else {
                    assert_eq!(parameter.direction(), reader::ParamDirection::Output);
                    assert!(!parameter.is_optional());
                    assert_eq!(
                        parameter
                            .find_attribute("NativeReturnedPointerAttribute")
                            .unwrap()
                            .value(),
                        [
                            ("MayBeNull".into(), windows_metadata::Value::Bool(false)),
                            ("NullOnFailure".into(), windows_metadata::Value::Bool(false)),
                        ]
                    );
                }
            }
        }
    });
}

#[test]
#[ignore = "requires the pinned SDK"]
fn pointer_parameter_flags_sdk() {
    helpers::ensure_libclang();
    for (version, name, _, _) in POINTER_CONTRACTS {
        let source = pointer_contract_input(version, name, "");
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = source.name.clone();
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        input.source = format!(
            "#include <rpcsal.h>\n{}",
            source.source.replace(
                "BUFFER unsigned short**",
                &format!("{name} unsigned short**")
            )
        );
        assert_eq!(project(&[input]), project(&[source]), "{name}");
    }
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "buffer_postconditions.h".into();
    input.roots = ["Buffer", "BufferFunction", "IBuffer"]
        .map(String::from)
        .into();
    input.source = pointer_contract_input("2", "_Outptr_opt_", "")
        .source
        .replace(
            "BUFFER unsigned short**",
            "_Outptr_opt_ _Inout_ unsigned short**",
        );
    input.source.insert_str(0, "#include <sal.h>\n");
    let output = project(&[input]);
    assert_eq!(
        output.partitions["buffer_postconditions"].rdl,
        std::fs::read_to_string(input_dir().join("returned_pointer_mixed.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn rpc_strings() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("rpc_strings");
    let output = project(std::slice::from_ref(&input));
    let text = &output.partitions.values().next().unwrap().rdl;
    assert!(text.contains("NativeNullTerminated("), "{text}");
    roundtrip("rpc_strings", &output, |index, _| {
        for (owner, name) in [
            ("Apis", "BufferFunction"),
            ("Buffer", "Invoke"),
            ("IBuffer", "Read"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|method| method.name() == name)
                .unwrap();
            for (position, parameter) in method.params().enumerate() {
                assert_eq!(
                    parameter.direction(),
                    if position < 2 {
                        reader::ParamDirection::Input
                    } else {
                        reader::ParamDirection::InputOutput
                    }
                );
                assert_eq!(parameter.is_optional(), position % 2 == 1);
                assert_eq!(
                    parameter.find_attribute("NativeSalAttribute").is_some(),
                    position >= 2
                );
                let input = parameter.find_attribute("NativeInputAttribute");
                assert_eq!(input.is_some(), position < 2);
                if let Some(input) = input {
                    assert_eq!(
                        input.value().into_iter().collect::<BTreeMap<_, _>>(),
                        [("ReadOnly".into(), windows_metadata::Value::Bool(false))].into()
                    );
                }
                assert_eq!(parameter.buffer_relationship(), None);
                assert_eq!(
                    parameter
                        .find_attribute("NativeNullTerminatedAttribute")
                        .unwrap()
                        .value()
                        .into_iter()
                        .collect::<BTreeMap<_, _>>(),
                    [
                        ("OnEntry".into(), windows_metadata::Value::Bool(true)),
                        (
                            "OnSuccess".into(),
                            windows_metadata::Value::Bool(position == 2)
                        ),
                    ]
                    .into()
                );
            }
        }
    });
    let mut other = test_clang2::fixture("rpc_strings");
    other.name = "other.h".into();
    let mut inputs = [input, other];
    let expected = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), expected);
}

#[test]
fn rpc_strings_residuals() {
    helpers::ensure_libclang();
    for change in [
        "#define VERSION \"2\"\n",
        "#define ARGUMENTS \"unexpected\"\n",
    ] {
        let mut input = test_clang2::fixture("rpc_strings");
        input.source.insert_str(0, change);
        let output = project(&[input]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert!(text.contains("NativeSal("), "{text}");
        for flag in ["NativeNullTerminated(", "#[in]", "#[out]", "#[opt]"] {
            assert!(!text.contains(flag), "{text}");
        }
    }
    for root in ["Buffer", "BufferFunction", "IBuffer"] {
        for replacement in [
            r#"__attribute__((annotate("windows_clang:sal:1.2:__RPC__in\n()\n()")))"#,
            r#"__attribute__((annotate("windows_clang:sal:2:_In_z_\n()\n()")))"#,
        ] {
            let mut inputs = [
                selected_fixture("rpc_strings", &[root]),
                selected_fixture("rpc_strings", &[root]),
            ];
            inputs[1].name = "other.h".into();
            inputs[1].source = inputs[1]
                .source
                .replace("CONTRACT(\"__RPC__in_string\")", replacement);
            for _ in 0..2 {
                let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
                assert!(error.to_string().contains("differ"), "{error}");
                inputs.reverse();
            }
        }
        let mut input = selected_fixture("rpc_strings", &[root]);
        input.source = input.source.replace("unsigned short*", "unsigned short");
        let output = projection::project_partial_with_imports(
            capture_partial(&[input], &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        assert_eq!(
            output.output.functions + output.output.aliases + output.output.interfaces,
            0
        );
        assert!(
            output
                .omissions
                .iter()
                .any(|error| error.contains("requires a buffer pointer")),
            "{:?}",
            output.omissions
        );
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn rpc_strings_sdk() {
    helpers::ensure_libclang();
    let source = test_clang2::fixture("rpc_strings");
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = source.name.clone();
    input.roots = ["Buffer", "BufferFunction", "IBuffer"]
        .map(String::from)
        .into();
    input.source = format!("#include <rpcsal.h>\n{}", source.source);
    for name in [
        "__RPC__in_string",
        "__RPC__in_opt_string",
        "__RPC__inout_string",
        "__RPC__inout_opt_string",
    ] {
        input.source = input.source.replace(&format!("CONTRACT(\"{name}\")"), name);
    }
    assert_eq!(project(&[input]), project(&[source]));
}

fn rpc_buffer_input(name: &str, arguments: &str, indirect: bool) -> Input {
    let mut input = postcondition_input(name, arguments, indirect);
    input.source.insert_str(0, "#define VERSION \"1.2\"\n");
    input
}

#[test]
fn conditional_length() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("conditional_length");
    let output = project(&[input]);
    roundtrip("conditional_length", &output, |index, _| {
        for (owner, name) in [
            ("Apis", "BufferFunction"),
            ("Buffer", "Invoke"),
            ("IBuffer", "Read"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|m| m.name() == name)
                .unwrap();
            let parameter = method.params().nth(1).unwrap();
            assert_eq!(parameter.direction(), reader::ParamDirection::Output);
            assert_eq!(
                parameter.buffer_relationship(),
                Some(reader::BufferRelationship::ElementsParam(0))
            );
            assert!(parameter.find_attribute("NativeSalAttribute").is_none());
            assert_eq!(
                parameter
                    .find_attribute("NativeArrayInfoAttribute")
                    .unwrap()
                    .value(),
                [
                    ("CountParamIndex".into(), windows_metadata::Value::I16(0)),
                    ("PostWritable".into(), windows_metadata::Value::Bool(true)),
                ]
            );
            assert_eq!(
                parameter
                    .find_attribute("NativeInitializedLengthAttribute")
                    .unwrap()
                    .value(),
                [
                    ("CountParamIndex".into(), windows_metadata::Value::I16(2)),
                    ("IsByteCount".into(), windows_metadata::Value::Bool(false)),
                    ("Dereference".into(), windows_metadata::Value::Bool(true)),
                    (
                        "NullFallbackParamIndex".into(),
                        windows_metadata::Value::I16(0)
                    ),
                ]
            );
        }
    });
    for expression in [
        "written?*written:capacity",
        " written ? * written : capacity ",
    ] {
        let mut input = test_clang2::fixture("conditional_length");
        input
            .source
            .insert_str(0, &format!("#define LENGTH \"{expression}\"\n"));
        assert_eq!(project(&[input]), output);
    }
    for expression in [
        "other ? *written : capacity",
        "written ? *written : 1",
        "written ? *written + 1 : capacity",
        "!written ? capacity : *written",
        "written ? *written : other ? other : capacity",
    ] {
        let mut input = test_clang2::fixture("conditional_length");
        input
            .source
            .insert_str(0, &format!("#define LENGTH \"{expression}\"\n"));
        let output = project(&[input]);
        let text = &output.partitions["conditional_length"].rdl;
        assert!(
            text.contains("NativeSal(")
                && text.contains("NativeArrayInfo(CountParamIndex = 0, PostWritable = true)"),
            "{text}"
        );
        assert!(!text.contains("NativeInitializedLength("), "{text}");
    }
    for definition in [
        "#define COUNT_TYPE unsigned int\n",
        "#define COUNT_TYPE float*\n",
        "#define FALLBACK_TYPE unsigned int*\n",
    ] {
        let mut input = test_clang2::fixture("conditional_length");
        input.source.insert_str(0, definition);
        let output = projection::project_partial_with_imports(
            capture_partial(&[input], &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        assert_eq!(
            output.output.functions + output.output.aliases + output.output.interfaces,
            0
        );
        assert!(
            output
                .omissions
                .iter()
                .any(|error| error.contains("requires") && error.contains("integer")),
            "{:?}",
            output.omissions
        );
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn conditional_length_sdk() {
    helpers::ensure_libclang();
    for (version, name) in [("1.2", "__RPC__out_ecount_part"), ("2", "_Out_writes_to_")] {
        let mut expected = test_clang2::fixture("conditional_length");
        expected.source.insert_str(
            0,
            &format!("#define VERSION \"{version}\"\n#define ANNOTATION \"{name}\"\n"),
        );
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = expected.name.clone();
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        input.source = format!(
            "#include <rpcsal.h>\n{}",
            expected.source.replace(
                "BUFFER unsigned short*",
                &format!("{name}(capacity, written ? *written : capacity) unsigned short*")
            )
        );
        assert_eq!(project(&[input]), project(&[expected]));
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_background_copy_enumerator() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_background_copy_enumerator.h".into();
    input.source = test_clang2::fixture("sdk_background_copy_enumerator").source;
    input.roots = vec!["IEnumBackgroundCopyFiles".into()];
    let output = project(&[input]);
    roundtrip("sdk_background_copy_enumerator", &output, |index, _| {
        let next = index
            .expect("Fixture", "IEnumBackgroundCopyFiles")
            .methods()
            .find(|m| m.name() == "Next")
            .unwrap();
        let array = next.params().nth(1).unwrap();
        assert!(array.find_attribute("NativeSalAttribute").is_none());
        assert_eq!(
            array.buffer_relationship(),
            Some(reader::BufferRelationship::ElementsParam(0))
        );
        assert_eq!(
            array
                .find_attribute("NativeInitializedLengthAttribute")
                .unwrap()
                .value(),
            [
                ("CountParamIndex".into(), windows_metadata::Value::I16(2)),
                ("IsByteCount".into(), windows_metadata::Value::Bool(false)),
                ("Dereference".into(), windows_metadata::Value::Bool(true)),
                (
                    "NullFallbackParamIndex".into(),
                    windows_metadata::Value::I16(0)
                ),
            ]
        );
        assert!(next.params().nth(2).unwrap().is_optional());
    });
}

#[test]
fn rpc_buffers() {
    helpers::ensure_libclang();
    let output = project(&[rpc_buffer_input(
        "__RPC__out_ecount_part",
        "capacity, written",
        false,
    )]);
    let text = &output.partitions.values().next().unwrap().rdl;
    assert!(text.contains("NativeInitializedLength("), "{text}");
    roundtrip("rpc_buffers", &output, |index, _| {
        for (owner, name) in [
            ("Apis", "BufferFunction"),
            ("Buffer", "Invoke"),
            ("IBuffer", "Read"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|method| method.name() == name)
                .unwrap();
            let parameter = method.params().next().unwrap();
            assert_eq!(parameter.direction(), reader::ParamDirection::Output);
            assert_eq!(
                parameter.buffer_relationship(),
                Some(reader::BufferRelationship::ElementsParam(1))
            );
            assert!(parameter.find_attribute("NativeSalAttribute").is_none());
            assert_eq!(
                parameter
                    .find_attribute("NativeInitializedLengthAttribute")
                    .unwrap()
                    .value()
                    .into_iter()
                    .collect::<BTreeMap<_, _>>(),
                [
                    ("CountParamIndex".into(), windows_metadata::Value::I16(2)),
                    ("IsByteCount".into(), windows_metadata::Value::Bool(false)),
                    ("Dereference".into(), windows_metadata::Value::Bool(false)),
                ]
                .into()
            );
        }
    });
}

#[test]
fn rpc_buffer_contracts() {
    check_rpc_buffers(false);
}

#[test]
fn rpc_input_buffer_agreement() {
    helpers::ensure_libclang();
    for (first, second) in [
        ("__RPC__in_ecount", "__RPC__in_ecount_full"),
        ("__RPC__in_ecount", "__RPC__in_xcount_full"),
        ("__RPC__in_ecount_opt", "__RPC__in_xcount_full_opt"),
        (
            "__RPC__in_ecount_full_string",
            "__RPC__in_xcount_full_string",
        ),
        (
            "__RPC__in_ecount_opt_string",
            "__RPC__in_ecount_full_opt_string",
        ),
    ] {
        let mut inputs = [
            rpc_buffer_input(first, "capacity", false),
            rpc_buffer_input(second, "capacity", false),
        ];
        inputs[1].name = "other.h".into();
        let expected = project(&inputs);
        for _ in 0..2 {
            assert_eq!(project(&inputs), expected);
            inputs.reverse();
        }
    }
    let mut inputs = [
        rpc_buffer_input("__RPC__in_ecount_full", "capacity", false),
        postcondition_input("_In_reads_", "capacity", false),
    ];
    inputs[1].name = "other.h".into();
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
        assert!(error.to_string().contains("differ"), "{error}");
        inputs.reverse();
    }
    for name in ["__RPC__in_ecount_full", "__RPC__in_ecount_full_string"] {
        for arguments in ["*written", "capacity + 1", "42"] {
            let output = project(&[rpc_buffer_input(name, arguments, true)]);
            let text = &output.partitions.values().next().unwrap().rdl;
            assert!(text.contains("NativeSal("), "{text}");
            assert!(text.contains("#[in]"), "{text}");
            for attribute in ["NativeInput(", "len_param(", "NativeInitializedLength("] {
                assert!(!text.contains(attribute), "{text}");
            }
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn rpc_buffer_contracts_sdk() {
    check_rpc_buffers(true);
    let input = rpc_buffer_input("__RPC__out_ecount_full", "*written", true);
    let mut native = test_clang2::annotated_sdk_fixture();
    native.name.clone_from(&input.name);
    native.roots = ["Buffer", "BufferFunction", "IBuffer"]
        .map(String::from)
        .into();
    native.source = format!(
        "#include <rpcsal.h>\n{}",
        input.source.replace(
            "BUFFER unsigned short*",
            "__RPC__out_ecount_full(*written) unsigned short*"
        )
    );
    assert_eq!(project(&[native]), project(&[input]));
}

fn check_rpc_buffers(sdk: bool) {
    use reader::ParamDirection::*;
    use windows_metadata::Value;
    helpers::ensure_libclang();
    for (names, direction, optional, (capacity, initialized), terminated) in [
        (
            &[
                "__RPC__in_ecount",
                "__RPC__in_ecount_full",
                "__RPC__in_xcount",
                "__RPC__in_xcount_full",
            ][..],
            Input,
            false,
            (true, None),
            false,
        ),
        (
            &[
                "__RPC__in_ecount_full_string",
                "__RPC__in_xcount_full_string",
            ][..],
            Input,
            false,
            (true, None),
            true,
        ),
        (
            &[
                "__RPC__in_ecount_opt",
                "__RPC__in_ecount_full_opt",
                "__RPC__in_xcount_opt",
                "__RPC__in_xcount_full_opt",
            ][..],
            Input,
            true,
            (true, None),
            false,
        ),
        (
            &[
                "__RPC__in_ecount_opt_string",
                "__RPC__in_ecount_full_opt_string",
                "__RPC__in_xcount_opt_string",
                "__RPC__in_xcount_full_opt_string",
            ][..],
            Input,
            true,
            (true, None),
            true,
        ),
        (
            &["__RPC__in_ecount_part", "__RPC__in_xcount_part"][..],
            Input,
            false,
            (true, None),
            false,
        ),
        (
            &["__RPC__in_ecount_part_opt", "__RPC__in_xcount_part_opt"][..],
            Input,
            true,
            (true, None),
            false,
        ),
        (
            &["__RPC__out_ecount"][..],
            Output,
            false,
            (true, None),
            false,
        ),
        (
            &["__RPC__out_ecount_string"][..],
            Output,
            false,
            (true, None),
            true,
        ),
        (
            &["__RPC__out_ecount_part"][..],
            Output,
            false,
            (true, Some(2)),
            false,
        ),
        (
            &["__RPC__out_ecount_full"][..],
            Output,
            false,
            (true, Some(1)),
            false,
        ),
        (
            &["__RPC__out_ecount_full_string"][..],
            Output,
            false,
            (true, Some(1)),
            true,
        ),
        (
            &["__RPC__inout_ecount_full"][..],
            InputOutput,
            false,
            (true, Some(1)),
            false,
        ),
        (
            &["__RPC__inout_ecount_full_string"][..],
            InputOutput,
            false,
            (true, Some(1)),
            true,
        ),
        (
            &["__RPC__inout_ecount_full_opt"][..],
            InputOutput,
            true,
            (true, Some(1)),
            false,
        ),
        (
            &["__RPC__inout_ecount_full_opt_string"][..],
            InputOutput,
            true,
            (true, Some(1)),
            true,
        ),
        (
            &[
                "__RPC__out_xcount",
                "__RPC__out_xcount_part",
                "__RPC__out_xcount_full",
            ][..],
            Output,
            false,
            (false, None),
            false,
        ),
        (
            &["__RPC__out_xcount_string", "__RPC__out_xcount_full_string"][..],
            Output,
            false,
            (false, None),
            true,
        ),
        (
            &[
                "__RPC__inout_xcount",
                "__RPC__inout_xcount_part",
                "__RPC__inout_xcount_full",
            ][..],
            InputOutput,
            false,
            (false, None),
            false,
        ),
        (
            &["__RPC__inout_xcount_full_string"][..],
            InputOutput,
            false,
            (false, None),
            true,
        ),
        (
            &[
                "__RPC__inout_xcount_opt",
                "__RPC__inout_xcount_part_opt",
                "__RPC__inout_xcount_full_opt",
            ][..],
            InputOutput,
            true,
            (false, None),
            false,
        ),
        (
            &["__RPC__inout_xcount_full_opt_string"][..],
            InputOutput,
            true,
            (false, None),
            true,
        ),
    ] {
        for name in names {
            let arguments = if name.contains("_part") {
                "capacity,written"
            } else {
                "capacity"
            };
            let input = rpc_buffer_input(name, arguments, false);
            let output = project(std::slice::from_ref(&input));
            if sdk {
                let mut native = test_clang2::annotated_sdk_fixture();
                native.name.clone_from(&input.name);
                native.roots = ["Buffer", "BufferFunction", "IBuffer"]
                    .map(String::from)
                    .into();
                native.source = format!(
                    "#include <rpcsal.h>\n{}",
                    input.source.replace(
                        "BUFFER unsigned short*",
                        &format!("{name}({arguments}) unsigned short*")
                    )
                );
                assert_eq!(project(&[native]), output, "{name}");
            }
            let text = &output.partitions.values().next().unwrap().rdl;
            roundtrip_metadata(&format!("rpc_buffer_{name}_{sdk}"), text, |index, _| {
                for (owner, method) in [
                    ("Apis", "BufferFunction"),
                    ("Buffer", "Invoke"),
                    ("IBuffer", "Read"),
                ] {
                    let method = index
                        .expect("Fixture", owner)
                        .methods()
                        .find(|value| value.name() == method)
                        .unwrap();
                    let parameter = method.params().next().unwrap();
                    assert_eq!(parameter.direction(), direction, "{name}");
                    assert_eq!(parameter.is_optional(), optional, "{name}");
                    let input = direction == Input && !name.contains("_part");
                    let output_buffer = direction != Input && name.contains("_ecount");
                    assert_eq!(
                        parameter.find_attribute("NativeSalAttribute").is_some(),
                        !input && !output_buffer,
                        "{name}"
                    );
                    let entry = parameter.find_attribute("NativeInputAttribute");
                    assert_eq!(entry.is_some(), input, "{name}");
                    if let Some(entry) = entry {
                        assert_eq!(
                            entry.value().into_iter().collect::<BTreeMap<_, _>>(),
                            [("ReadOnly".into(), Value::Bool(false))].into(),
                            "{name}"
                        );
                    }
                    assert_eq!(
                        parameter.buffer_relationship(),
                        capacity.then_some(reader::BufferRelationship::ElementsParam(1)),
                        "{name}"
                    );
                    let post_writable = parameter
                        .find_attribute("NativeArrayInfoAttribute")
                        .is_some_and(|attribute| {
                            attribute
                                .value()
                                .contains(&("PostWritable".into(), Value::Bool(true)))
                        });
                    assert_eq!(
                        post_writable,
                        output_buffer && direction == Output,
                        "{name}"
                    );
                    let length = parameter.find_attribute("NativeInitializedLengthAttribute");
                    assert_eq!(length.is_some(), initialized.is_some(), "{name}");
                    if let Some(length) = length {
                        assert_eq!(
                            length.value().into_iter().collect::<BTreeMap<_, _>>(),
                            [
                                ("CountParamIndex".into(), Value::I16(initialized.unwrap())),
                                ("IsByteCount".into(), Value::Bool(false)),
                                ("Dereference".into(), Value::Bool(false)),
                            ]
                            .into(),
                            "{name}"
                        );
                    }
                    let termination = parameter.find_attribute("NativeNullTerminatedAttribute");
                    assert_eq!(termination.is_some(), terminated, "{name}");
                    if let Some(termination) = termination {
                        assert_eq!(
                            termination.value().into_iter().collect::<BTreeMap<_, _>>(),
                            [
                                ("OnEntry".into(), Value::Bool(direction != Output)),
                                ("OnSuccess".into(), Value::Bool(direction != Input)),
                            ]
                            .into(),
                            "{name}"
                        );
                    }
                }
            });
        }
    }
}

#[test]
fn rpc_output_buffer_agreement() {
    helpers::ensure_libclang();
    for (first, arguments, second, second_arguments, version, agrees) in [
        (
            "__RPC__out_ecount_full",
            "capacity",
            "__RPC__out_ecount_part",
            "capacity, capacity",
            "1.2",
            true,
        ),
        (
            "__RPC__inout_ecount_full",
            "capacity",
            "_Inout_updates_all_",
            "capacity",
            "2",
            true,
        ),
        (
            "__RPC__out_ecount_full",
            "capacity",
            "_Out_writes_all_",
            "capacity",
            "2",
            false,
        ),
        (
            "__RPC__out_ecount",
            "capacity",
            "_Out_writes_",
            "capacity",
            "2",
            false,
        ),
    ] {
        let mut other = postcondition_input(second, second_arguments, false);
        other.name = "other.h".into();
        other
            .source
            .insert_str(0, &format!("#define VERSION \"{version}\"\n"));
        let mut inputs = [rpc_buffer_input(first, arguments, false), other];
        for _ in 0..2 {
            if agrees {
                project(&inputs);
            } else {
                let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
                assert!(error.to_string().contains("differ"), "{error}");
            }
            inputs.reverse();
        }
    }
    for arguments in ["42", "*written", "capacity + 1"] {
        let output = project(&[rpc_buffer_input(
            "__RPC__out_ecount_full_string",
            arguments,
            true,
        )]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert!(text.contains("NativeSal("), "{text}");
        assert!(!text.contains("PostWritable = true"), "{text}");
        assert!(text.contains("OnSuccess = true"), "{text}");
    }
}

#[test]
fn rpc_full_buffers() {
    helpers::ensure_libclang();
    let output = project(&[test_clang2::fixture("rpc_full_buffers")]);
    roundtrip("rpc_full_buffers", &output, |_, _| {});
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_shell_item() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_shell_item.h".into();
    input.source = test_clang2::fixture("sdk_shell_item").source;
    input.roots = vec!["IShellItem".into()];
    let output = project(&[input]);
    roundtrip("sdk_shell_item", &output, |index, _| {
        let method = index
            .expect("Fixture", "IShellItem")
            .methods()
            .find(|method| method.name() == "GetDisplayName")
            .unwrap();
        let result = method.params().last().unwrap();
        assert_eq!(result.direction(), reader::ParamDirection::Output);
        assert!(!result.is_optional());
        assert!(result.find_attribute("NativeSalAttribute").is_none());
        assert!(result.find_attribute("ComOutPtrAttribute").is_none());
        assert_eq!(
            result
                .find_attribute("NativeReturnedPointerAttribute")
                .unwrap()
                .value(),
            [
                ("MayBeNull".into(), windows_metadata::Value::Bool(false)),
                ("NullOnFailure".into(), windows_metadata::Value::Bool(true)),
            ]
        );
    });
}

#[test]
fn rpc_buffer_residuals() {
    helpers::ensure_libclang();
    let output = project(&[rpc_buffer_input("__RPC__out_ecount_full", "*written", true)]);
    let text = &output.partitions.values().next().unwrap().rdl;
    assert!(!text.contains("len_param("), "{text}");
    assert!(
        text.contains(
            "NativeInitializedLength(CountParamIndex = 2, IsByteCount = false, Dereference = true)"
        ),
        "{text}"
    );
    roundtrip_metadata("rpc_buffer_indirect_full", text, |index, _| {
        let parameter = index
            .expect("Fixture", "Buffer")
            .methods()
            .next()
            .unwrap()
            .params()
            .next()
            .unwrap();
        assert_eq!(parameter.buffer_relationship(), None);
        assert_eq!(
            parameter
                .find_attribute("NativeInitializedLengthAttribute")
                .unwrap()
                .value()
                .into_iter()
                .collect::<BTreeMap<_, _>>(),
            [
                ("CountParamIndex".into(), windows_metadata::Value::I16(2)),
                ("IsByteCount".into(), windows_metadata::Value::Bool(false)),
                ("Dereference".into(), windows_metadata::Value::Bool(true)),
            ]
            .into()
        );
    });
    for (arguments, indirect, capacity, length) in [
        ("capacity, *written", true, true, true),
        ("capacity + other, *written", true, false, true),
        ("capacity, written + other", false, true, false),
        ("capacity + other, written + other", false, false, false),
    ] {
        let output = project(&[rpc_buffer_input(
            "__RPC__out_ecount_part",
            arguments,
            indirect,
        )]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert_eq!(text.contains("NativeSal("), !(capacity && length), "{text}");
        assert_eq!(
            text.contains("len_param(1)") || text.contains("PostWritable = true"),
            capacity,
            "{text}"
        );
        assert_eq!(text.contains("NativeInitializedLength("), length, "{text}");
        if length {
            assert!(text.contains("Dereference = true"), "{text}");
        }
    }
    for name in [
        "__RPC__in_ecount_full",
        "__RPC__out_ecount_part",
        "__RPC__inout_xcount",
    ] {
        for (version, arguments) in [("2", "capacity"), ("1.2", "")] {
            let mut input = rpc_buffer_input(name, arguments, false);
            input.source = input.source.replace(
                "#define VERSION \"1.2\"",
                &format!("#define VERSION \"{version}\""),
            );
            let output = project(&[input]);
            let text = &output.partitions.values().next().unwrap().rdl;
            assert!(text.contains("NativeSal("), "{text}");
            for flag in [
                "#[in]",
                "#[out]",
                "#[opt]",
                "len_param(",
                "NativeInitializedLength(",
            ] {
                assert!(!text.contains(flag), "{text}");
            }
        }
    }
    for root in ["Buffer", "BufferFunction", "IBuffer"] {
        for (change, expected) in [
            (
                ("unsigned short*", "unsigned short"),
                "requires a buffer pointer",
            ),
            (
                ("unsigned int capacity", "float capacity"),
                "requires a by-value integer",
            ),
            (
                ("LENGTH_TYPE unsigned int", "LENGTH_TYPE float"),
                "requires an integer count",
            ),
        ] {
            let mut input = rpc_buffer_input("__RPC__out_ecount_part", "capacity, written", false);
            input.roots = vec![root.into()];
            input.source = input.source.replace(change.0, change.1);
            let output = projection::project_partial_with_imports(
                capture_partial(&[input], &TARGETS).unwrap(),
                "Fixture",
                &imports(),
            )
            .unwrap();
            assert_eq!(
                output.output.functions + output.output.aliases + output.output.interfaces,
                0
            );
            assert!(
                output
                    .omissions
                    .iter()
                    .any(|error| error.contains(expected)),
                "{:?}",
                output.omissions
            );
        }
        let mut inputs = [
            rpc_buffer_input("__RPC__out_ecount_part", "capacity,written", false),
            rpc_buffer_input("__RPC__out_xcount_part", "capacity,written", false),
        ];
        inputs[1].name = "other.h".into();
        for input in &mut inputs {
            input.roots = vec![root.into()];
        }
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains("differ"), "{error}");
            inputs.reverse();
        }
    }
    let mut inputs = [
        rpc_buffer_input("__RPC__out_ecount_part", "capacity, written", false),
        rpc_buffer_input("__RPC__out_ecount_part", "capacity, written", false),
    ];
    inputs[1].name = "other.h".into();
    let expected = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), expected);
}

#[test]
fn reserved_parameters() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("reserved_parameters");
    let output = project(std::slice::from_ref(&input));
    let text = &output.partitions.values().next().unwrap().rdl;
    assert!(text.contains("#[reserved]"), "{text}");
    roundtrip("reserved_parameters", &output, |index, _| {
        for (owner, name) in [
            ("Apis", "BufferFunction"),
            ("Reserved", "Invoke"),
            ("IReserved", "Read"),
        ] {
            let method = index
                .expect("Fixture", owner)
                .methods()
                .find(|method| method.name() == name)
                .unwrap();
            let parameters: Vec<_> = method.params().collect();
            assert_eq!(parameters.len(), 4);
            for (index, parameter) in parameters.iter().enumerate() {
                assert_eq!(parameter.is_reserved(), index < 3);
                assert!(!parameter.is_optional());
                assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                assert_eq!(
                    parameter.direction(),
                    if index == 1 {
                        reader::ParamDirection::Output
                    } else {
                        reader::ParamDirection::Input
                    }
                );
            }
        }
    });
    let mut legacy = test_clang2::fixture("reserved_parameters");
    legacy.source.insert_str(
        0,
        "#define VERSION \"1\"\n#define ANNOTATION \"__reserved\"\n",
    );
    assert_eq!(project(std::slice::from_ref(&legacy)), output);
    legacy.name = "legacy.h".into();
    let mut inputs = [input, legacy];
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    for snapshot in &snapshots {
        let ValueKind::Function { parameters, .. } = &snapshot.values[0].observations[0].kind
        else {
            panic!()
        };
        let AttributeKind::Sal(value) = &parameters[0].attributes[0].kind else {
            panic!()
        };
        assert!(matches!(
            (value.version.as_str(), value.name.as_str()),
            ("2", "_Reserved_") | ("1", "__reserved")
        ));
    }
    let expected = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), expected);
    assert_eq!(
        projection::project_partial_with_imports(
            capture_partial(&inputs, &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap()
        .output,
        expected
    );
    let mut snapshots = capture(&inputs, &TARGETS).unwrap();
    snapshots.reverse();
    assert_eq!(
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap(),
        expected
    );
    let mut repeated = test_clang2::fixture("reserved_parameters");
    repeated.source = repeated
        .source
        .replace("RESERVED void*", "RESERVED RESERVED void*");
    assert_eq!(project(&[repeated]), output);
}

#[test]
fn reserved_parameter_residuals() {
    helpers::ensure_libclang();
    for (case, (version, name, arguments)) in [
        ("1", "_Reserved_", ""),
        ("2", "__reserved", ""),
        ("2", "_Reserved_", "unexpected"),
        ("2", "_When_", "ordinary, _Reserved_"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut input = test_clang2::fixture("reserved_parameters");
        input.source.insert_str(
            0,
            &format!(
                "#define VERSION \"{version}\"\n#define ANNOTATION \"{name}\"\n#define ARGUMENTS \"{arguments}\"\n"
            ),
        );
        let output = project(&[input]);
        let text = &output.partitions.values().next().unwrap().rdl;
        assert!(!text.contains("#[reserved]"), "{text}");
        roundtrip_metadata(&format!("reserved_residual_{case}"), text, |index, _| {
            let parameter = index
                .expect("Fixture", "Apis")
                .methods()
                .next()
                .unwrap()
                .params()
                .next()
                .unwrap();
            assert!(!parameter.is_reserved());
            assert!(parameter.find_attribute("NativeSalAttribute").is_some());
        });
    }
    for root in ["BufferFunction", "Reserved", "IReserved"] {
        let current = selected_fixture("reserved_parameters", &[root]);
        let mut other = selected_fixture("reserved_parameters", &[root]);
        other.name = "other.h".into();
        other.source.insert_str(0, "#define RESERVED\n");
        let mut inputs = [current, other];
        for _ in 0..2 {
            let expected = match root {
                "Reserved" => "callback parameter annotations differ",
                "BufferFunction" => "native value declarations differ",
                _ => "native method names, slots, or attributes differ",
            };
            let error = projection::generate(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains(expected), "{error}");
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains(expected), "{error}");
            inputs.reverse();
        }
    }
    let mut input = test_clang2::fixture("reserved_parameters");
    input
        .source
        .push_str("\ntypedef RESERVED void* ReservedPointer;\n");
    let text = project(&[input])
        .partitions
        .into_values()
        .next()
        .unwrap()
        .rdl;
    roundtrip_metadata("reserved_typedef", &text, |index, _| {
        let alias = index.expect("Fixture", "ReservedPointer");
        assert!(alias.find_attribute("ReservedAttribute").is_none());
        let annotation = alias.find_attribute("NativeSalAttribute").unwrap();
        assert_eq!(
            annotation.value()[1].1,
            windows_metadata::Value::Utf8("_Reserved_".into())
        );
    });
}

#[test]
fn conditional_length_agreement() {
    helpers::ensure_libclang();
    let expected = project(&[test_clang2::fixture("conditional_length")]);
    for expression in ["written?*written:capacity", "written ? *written : other"] {
        let mut changed = test_clang2::fixture("conditional_length");
        changed.name = "other.h".into();
        changed
            .source
            .insert_str(0, &format!("#define LENGTH \"{expression}\"\n"));
        let mut inputs = [test_clang2::fixture("conditional_length"), changed];
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            if expression.ends_with("capacity") {
                let actual = projection::project_with_imports(
                    &merge(&snapshots).unwrap(),
                    "Fixture",
                    &imports(),
                )
                .unwrap();
                assert_eq!(
                    actual.partitions.values().next().unwrap().rdl,
                    expected.partitions.values().next().unwrap().rdl
                );
            } else {
                let error = merge(&snapshots).err().unwrap();
                assert!(error.to_string().contains("differ"), "{error}");
            }
            inputs.reverse();
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn reserved_parameters_sdk() {
    helpers::ensure_libclang();
    let expected = project(&[test_clang2::fixture("reserved_parameters")]);
    for name in ["_Reserved_", "__reserved"] {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "reserved_parameters.h".into();
        input.source = format!(
            "#define RESERVED {name}\n{}",
            test_clang2::fixture("reserved_parameters").source
        );
        input.roots = ["Reserved", "BufferFunction", "IReserved"]
            .map(String::from)
            .into();
        assert_eq!(project(&[input]), expected);
    }
}

#[test]
fn rpc_annotation_normalization() {
    helpers::ensure_libclang();
    for (case, (rpc, current, arguments)) in RPC_CONTRACTS.into_iter().enumerate() {
        for indirect in [false, true] {
            let arguments = if indirect {
                arguments.replace("written", "*written")
            } else {
                arguments.into()
            };
            let current = postcondition_input(current, &arguments, indirect);
            let mut rpc_input = postcondition_input(rpc, &arguments, indirect);
            rpc_input.source.insert_str(0, "#define VERSION \"1.2\"\n");
            let output = project(std::slice::from_ref(&current));
            let text = &output.partitions.values().next().unwrap().rdl;
            assert_eq!(
                project(std::slice::from_ref(&rpc_input))
                    .partitions
                    .values()
                    .next()
                    .unwrap()
                    .rdl,
                *text,
                "{rpc}"
            );
            for snapshot in capture(std::slice::from_ref(&rpc_input), &TARGETS).unwrap() {
                let ValueKind::Function { parameters, .. } =
                    &snapshot.values[0].observations[0].kind
                else {
                    panic!()
                };
                let AttributeKind::Sal(annotation) = &parameters[0].attributes[0].kind else {
                    panic!()
                };
                assert_eq!(annotation.version, "1.2");
                assert_eq!(annotation.name, rpc);
            }
            rpc_input.name = "rpc.h".into();
            let mut inputs = [current, rpc_input];
            for _ in 0..2 {
                assert_eq!(
                    &project(&inputs).partitions.values().next().unwrap().rdl,
                    text
                );
                inputs.reverse();
            }
            roundtrip_metadata(
                &format!("rpc_annotation_{case}_{indirect}"),
                text,
                |index, _| {
                    for (owner, name) in [
                        ("Apis", "BufferFunction"),
                        ("Buffer", "Invoke"),
                        ("IBuffer", "Read"),
                    ] {
                        let method = index
                            .expect("Fixture", owner)
                            .methods()
                            .find(|method| method.name() == name)
                            .unwrap();
                        assert!(
                            method
                                .params()
                                .next()
                                .unwrap()
                                .find_attribute("NativeSalAttribute")
                                .is_none()
                        );
                    }
                },
            );
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn rpc_annotation_normalization_sdk() {
    helpers::ensure_libclang();
    for (rpc, current, arguments) in RPC_CONTRACTS {
        for indirect in [false, true] {
            let arguments = if indirect {
                arguments.replace("written", "*written")
            } else {
                arguments.into()
            };
            let expected = project(&[postcondition_input(current, &arguments, indirect)]);
            let mut input = test_clang2::annotated_sdk_fixture();
            input.name = "buffer_postconditions.h".into();
            let annotation = if arguments.is_empty() {
                rpc.to_string()
            } else {
                format!("{rpc}({arguments})")
            };
            input.source = format!(
                "#include <rpcsal.h>\n{}",
                postcondition_input(current, &arguments, indirect)
                    .source
                    .replace(
                        "BUFFER unsigned short*",
                        &format!("{annotation} unsigned short*")
                    )
            );
            input.roots = ["Buffer", "BufferFunction", "IBuffer"]
                .map(String::from)
                .into();
            let output = project(&[input]);
            assert_eq!(
                output.partitions.values().next().unwrap().rdl,
                expected.partitions.values().next().unwrap().rdl,
                "{rpc}"
            );
        }
    }
    for rpc in ["__RPC__in", "__RPC__in_opt"] {
        let mut expected = postcondition_input(rpc, "", false);
        expected.source.insert_str(0, "#define VERSION \"1.2\"\n");
        let expected = project(&[expected]);
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "buffer_postconditions.h".into();
        input.source = format!(
            "#include <rpcsal.h>\n{}",
            postcondition_input(rpc, "", false)
                .source
                .replace("BUFFER unsigned short*", &format!("{rpc} unsigned short*"))
        );
        input.roots = ["Buffer", "BufferFunction", "IBuffer"]
            .map(String::from)
            .into();
        assert_eq!(
            project(&[input]).partitions.values().next().unwrap().rdl,
            expected.partitions.values().next().unwrap().rdl
        );
    }
}

#[test]
fn rpc_annotation_residuals() {
    helpers::ensure_libclang();
    for (rpc, arguments, current, current_arguments) in [
        ("__RPC__in", "", "_In_", ""),
        ("__RPC__in_opt", "", "_In_opt_", ""),
        ("__RPC__out_ecount", "capacity", "_Out_writes_", "capacity"),
        (
            "__RPC__inout_opt_string",
            "",
            "_Inout_updates_opt_z_",
            "capacity",
        ),
        (
            "__RPC__inout_xcount",
            "capacity",
            "_Inout_updates_",
            "capacity",
        ),
    ] {
        let mut rpc_input = postcondition_input(rpc, arguments, false);
        rpc_input.source.insert_str(0, "#define VERSION \"1.2\"\n");
        let text = project(std::slice::from_ref(&rpc_input))
            .partitions
            .into_values()
            .next()
            .unwrap()
            .rdl;
        let typed_input = rpc == "__RPC__in" || rpc == "__RPC__in_opt";
        assert_eq!(
            text.contains(&format!("NativeSal(\"1.2\", \"{rpc}\"")),
            !typed_input && rpc != "__RPC__out_ecount",
            "{text}"
        );
        assert_eq!(
            text.contains("NativeArrayInfo(CountParamIndex = 1, PostWritable = true)"),
            rpc == "__RPC__out_ecount",
            "{text}"
        );
        assert_eq!(
            text.contains("NativeNullTerminated"),
            rpc == "__RPC__inout_opt_string",
            "{text}"
        );
        if typed_input {
            roundtrip_metadata(&format!("rpc_residual_{rpc}"), &text, |index, _| {
                for (owner, name) in [
                    ("Apis", "BufferFunction"),
                    ("Buffer", "Invoke"),
                    ("IBuffer", "Read"),
                ] {
                    let method = index
                        .expect("Fixture", owner)
                        .methods()
                        .find(|method| method.name() == name)
                        .unwrap();
                    let parameter = method.params().next().unwrap();
                    assert_eq!(parameter.direction(), reader::ParamDirection::Input);
                    assert_eq!(parameter.is_optional(), rpc == "__RPC__in_opt");
                    assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                    assert_eq!(
                        parameter
                            .find_attribute("NativeInputAttribute")
                            .unwrap()
                            .value(),
                        [("ReadOnly".into(), windows_metadata::Value::Bool(false))]
                    );
                }
            });
        }
        let mut current = postcondition_input(current, current_arguments, false);
        current.name = "current.h".into();
        let mut inputs = [rpc_input, current];
        for _ in 0..2 {
            assert!(
                projection::generate_partial(&inputs, "Fixture").is_err(),
                "{rpc}"
            );
            inputs.reverse();
        }
    }
    for (version, arguments) in [("1", ""), ("2", ""), ("1.2", "unexpected")] {
        for name in ["__RPC__out", "__RPC__in", "__RPC__in_opt"] {
            let mut input = postcondition_input(name, arguments, false);
            input
                .source
                .insert_str(0, &format!("#define VERSION \"{version}\"\n"));
            let output = project(&[input]);
            let text = &output.partitions.values().next().unwrap().rdl;
            assert!(text.contains("NativeSal("), "{text}");
            assert!(
                !text.contains("#[in]") && !text.contains("#[opt]"),
                "{text}"
            );
        }
    }
}

#[test]
fn buffer_postconditions() {
    use windows_metadata::Value;
    helpers::ensure_libclang();
    for (case, (current, legacy)) in BUFFER_POSTCONDITIONS.into_iter().enumerate() {
        for indirect in [false, true] {
            let arguments = match case {
                0..2 => "",
                2..6 => "capacity",
                _ if indirect => "capacity, *written",
                _ => "capacity, written",
            };
            let input = postcondition_input(current, arguments, indirect);
            let output = project(std::slice::from_ref(&input));
            let text = &output.partitions.values().next().unwrap().rdl;
            if case == 0 && !indirect {
                if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
                    std::fs::write(input_dir().join("buffer_postconditions.rdl"), text).unwrap();
                }
                assert_eq!(
                    text,
                    &std::fs::read_to_string(input_dir().join("buffer_postconditions.rdl"))
                        .unwrap()
                        .replace("\r\n", "\n")
                );
            }
            let mut other = postcondition_input(legacy.unwrap_or(current), arguments, indirect);
            other.name = "other.h".into();
            if legacy.is_some() {
                other.source.insert_str(0, "#define VERSION \"1\"\n");
            }
            assert_eq!(
                &project(std::slice::from_ref(&other))
                    .partitions
                    .values()
                    .next()
                    .unwrap()
                    .rdl,
                text
            );
            let mut inputs = [input, other];
            for _ in 0..2 {
                assert_eq!(
                    &project(&inputs).partitions.values().next().unwrap().rdl,
                    text
                );
                inputs.reverse();
            }
            roundtrip_metadata(
                &format!("buffer_postconditions_{case}_{indirect}"),
                text,
                |index, _| {
                    for (owner, name) in [
                        ("Apis", "BufferFunction"),
                        ("Buffer", "Invoke"),
                        ("IBuffer", "Read"),
                    ] {
                        let method = index
                            .expect("Fixture", owner)
                            .methods()
                            .find(|method| method.name() == name)
                            .unwrap();
                        let parameter = method.params().next().unwrap();
                        let input = current.starts_with("_In");
                        let output = !current.starts_with("_In_");
                        assert_eq!(
                            parameter.direction(),
                            match (input, output) {
                                (true, false) => reader::ParamDirection::Input,
                                (false, true) => reader::ParamDirection::Output,
                                _ => reader::ParamDirection::InputOutput,
                            }
                        );
                        assert_eq!(parameter.is_optional(), current.contains("_opt_"));
                        assert!(parameter.find_attribute("NativeSalAttribute").is_none());
                        let entry = parameter.find_attribute("NativeInputAttribute");
                        assert_eq!(entry.is_some(), case < 2);
                        if let Some(entry) = entry {
                            assert_eq!(
                                entry.value().into_iter().collect::<BTreeMap<_, _>>(),
                                [("ReadOnly".into(), Value::Bool(true))].into()
                            );
                        }
                        let terminated = parameter.find_attribute("NativeNullTerminatedAttribute");
                        assert_eq!(terminated.is_some(), case < 6);
                        if let Some(terminated) = terminated {
                            assert_eq!(
                                terminated.value().into_iter().collect::<BTreeMap<_, _>>(),
                                [
                                    ("OnEntry".into(), Value::Bool(input)),
                                    ("OnSuccess".into(), Value::Bool(output))
                                ]
                                .into()
                            );
                        }
                        let length = parameter.find_attribute("NativeInitializedLengthAttribute");
                        assert_eq!(length.is_some(), case >= 6);
                        let bytes = current.contains("_bytes_");
                        if let Some(length) = length {
                            assert_eq!(
                                length.value().into_iter().collect::<BTreeMap<_, _>>(),
                                [
                                    ("CountParamIndex".into(), Value::I16(2)),
                                    ("IsByteCount".into(), Value::Bool(bytes)),
                                    ("Dereference".into(), Value::Bool(indirect)),
                                ]
                                .into()
                            );
                        }
                        assert_eq!(
                            parameter.buffer_relationship(),
                            if case < 2 {
                                None
                            } else {
                                Some(if bytes {
                                    reader::BufferRelationship::BytesParam(1)
                                } else {
                                    reader::BufferRelationship::ElementsParam(1)
                                })
                            }
                        );
                    }
                },
            );
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn buffer_postconditions_sdk() {
    helpers::ensure_libclang();
    for (case, (current, legacy)) in BUFFER_POSTCONDITIONS.into_iter().enumerate() {
        for indirect in [false, true] {
            let arguments = match case {
                0..2 => "",
                2..6 => "capacity",
                _ if indirect => "capacity, *written",
                _ => "capacity, written",
            };
            let expected = project(&[postcondition_input(current, arguments, indirect)]);
            for name in Some(current).into_iter().chain(legacy) {
                let mut input = test_clang2::annotated_sdk_fixture();
                input.name = "buffer_postconditions.h".into();
                let annotation = if arguments.is_empty() {
                    name.to_string()
                } else {
                    format!("{name}({arguments})")
                };
                input.source = postcondition_input(current, arguments, indirect)
                    .source
                    .replace(
                        "BUFFER unsigned short*",
                        &format!("{annotation} unsigned short*"),
                    );
                input.roots = ["Buffer", "BufferFunction", "IBuffer"]
                    .map(String::from)
                    .into();
                let output = project(&[input]);
                assert_eq!(
                    output.partitions.values().next().unwrap().rdl,
                    expected.partitions.values().next().unwrap().rdl,
                    "{name}"
                );
            }
        }
    }
}

#[test]
fn buffer_postconditions_conflicts() {
    helpers::ensure_libclang();
    for root in ["Buffer", "BufferFunction", "IBuffer"] {
        for (first, second) in [
            (
                postcondition_input("_Out_writes_bytes_all_", "capacity", false),
                postcondition_input("_Out_writes_bytes_", "capacity", false),
            ),
            (
                postcondition_input("_In_z_", "", false),
                postcondition_input("_In_", "", false),
            ),
            (
                postcondition_input("_Inout_updates_z_", "capacity", false),
                postcondition_input("_Out_writes_z_", "capacity", false),
            ),
            (
                postcondition_input("_Out_writes_to_", "capacity, written", false),
                postcondition_input("_Out_writes_to_", "capacity, other", false),
            ),
            (
                postcondition_input("_Out_writes_to_", "capacity, written", true),
                postcondition_input("_Out_writes_to_", "capacity, *written", true),
            ),
            (
                postcondition_input("_Out_writes_to_", "capacity, written", false),
                postcondition_input("_Out_writes_bytes_to_", "capacity, written", false),
            ),
        ] {
            let mut inputs = [first, second];
            inputs[1].name = "other.h".into();
            for input in &mut inputs {
                input.roots = vec![root.into()];
            }
            for _ in 0..2 {
                let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
                let expected = match root {
                    "Buffer" => "callback parameter annotations differ",
                    "BufferFunction" => "native value declarations differ",
                    _ => "native method names, slots, or attributes differ",
                };
                assert!(error.to_string().contains(expected), "{error}");
                inputs.reverse();
            }
        }
    }
    for (arguments, indirect, change, message) in [
        (
            "capacity, *written",
            false,
            "",
            "requires a pointer to an integer",
        ),
        ("capacity, written", true, "", "requires an integer count"),
        (
            "capacity, *written",
            true,
            "float*",
            "requires an integer count",
        ),
        (
            "capacity, *written",
            true,
            "unsigned int**",
            "requires an integer count",
        ),
    ] {
        let mut input = postcondition_input("_Out_writes_to_", arguments, indirect);
        if !change.is_empty() {
            input.source = input.source.replace("unsigned int*", change);
        }
        input.roots = vec!["Buffer".into()];
        let error = projection::generate(&[input], "Fixture").unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
    }
    let mut input = postcondition_input("_In_z_", "", false);
    input.source = input.source.replace("unsigned short*", "unsigned short");
    input.roots = vec!["Buffer".into()];
    let error = projection::generate(&[input], "Fixture").unwrap_err();
    assert!(
        error.to_string().contains("requires a buffer pointer"),
        "{error}"
    );
    let mut input = postcondition_input("_Out_writes_to_", "capacity, written", false);
    input.roots = vec!["Buffer".into()];
    input.source = input.source.replace(
        "BUFFER unsigned short*",
        "BUFFER __attribute__((annotate(\"windows_clang:sal:2:_Out_writes_to_\\n(capacity, other)\\n(capacity, other)\"))) unsigned short*",
    );
    let error = projection::generate(&[input], "Fixture").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("conflicting SAL initialized length"),
        "{error}"
    );
}

#[test]
fn buffer_postcondition_residuals() {
    use windows_metadata::Value;
    helpers::ensure_libclang();
    let input = test_clang2::fixture("buffer_postcondition_residuals");
    let output = project(&[input]);
    roundtrip("buffer_postcondition_residuals", &output, |index, _| {
        for (name, raw, capacity, entry, success, length) in [
            ("Expression", 1, true, false, false, false),
            ("ReturnLength", 1, true, false, false, false),
            ("IndirectCapacity", 1, false, false, false, false),
            ("ConstantCapacity", 1, false, false, false, false),
            ("Conditional", 1, false, false, false, false),
            ("PointerDifference", 1, false, false, false, false),
            ("ExtraArguments", 1, true, false, false, false),
            ("UnexpectedArguments", 1, false, false, false, false),
            ("EntryOnly", 2, false, true, false, false),
            ("EntryAndSuccess", 2, true, true, true, false),
            ("OptionalCount", 0, true, false, false, true),
            ("Successful", 0, true, false, false, true),
        ] {
            let owner = index.expect("Fixture", name);
            let method = owner
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap();
            let parameter = method.params().next().unwrap();
            assert_eq!(
                parameter
                    .attributes()
                    .filter(|attribute| attribute.name() == "NativeSalAttribute")
                    .count(),
                raw,
                "{name}"
            );
            assert_eq!(
                parameter.buffer_relationship(),
                capacity.then_some(reader::BufferRelationship::ElementsParam(1)),
                "{name}"
            );
            assert_eq!(
                parameter
                    .find_attribute("NativeInitializedLengthAttribute")
                    .is_some(),
                length,
                "{name}"
            );
            let termination = parameter.find_attribute("NativeNullTerminatedAttribute");
            assert_eq!(termination.is_some(), entry || success, "{name}");
            if let Some(termination) = termination {
                assert_eq!(
                    termination.value().into_iter().collect::<BTreeMap<_, _>>(),
                    [
                        ("OnEntry".into(), Value::Bool(entry)),
                        ("OnSuccess".into(), Value::Bool(success)),
                    ]
                    .into()
                );
                assert_eq!(parameter.direction(), reader::ParamDirection::InputOutput);
            }
            if name == "OptionalCount" {
                assert!(method.params().nth(2).unwrap().is_optional());
            }
            if name == "Successful" {
                let sal = owner.find_attribute("NativeSalAttribute").unwrap();
                assert!(
                    sal.value()
                        .iter()
                        .any(|(_, value)| value == &Value::Utf8("_Success_".into()))
                );
                assert!(
                    sal.value()
                        .iter()
                        .any(|(_, value)| value == &Value::Utf8("return != 0".into()))
                );
            }
        }
    });
    let mut inputs = [
        postcondition_input("_Out_writes_to_", "capacity, written", false),
        postcondition_input("_Out_writes_to_", "capacity, written", false),
    ];
    inputs[1].name = "renamed.h".into();
    inputs[1].source = inputs[1].source.replace("written", "received");
    for _ in 0..2 {
        merge(&capture(&inputs, &TARGETS).unwrap()).unwrap();
        inputs.reverse();
    }
}

#[test]
fn function_typedefs() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("function_typedefs", &[]),
        selected_fixture("function_typedefs", &[]),
    ];
    inputs[1].name = "other.h".into();
    for snapshot in capture(&inputs, &TARGETS).unwrap() {
        for (name, function) in [("Bare", true), ("Pointer", false)] {
            let alias = snapshot
                .aliases
                .iter()
                .find(|alias| alias.name == name)
                .unwrap();
            let kind = &snapshot.types[alias.observations[0].canonical.0].kind;
            assert_eq!(matches!(kind, TypeKind::Function(_)), function);
            assert_eq!(matches!(kind, TypeKind::Pointer(_)), !function);
        }
    }
    let output = project(&inputs);
    inputs.reverse();
    assert_eq!(output, project(&inputs));
    roundtrip("function_typedefs", &output, |index, _| {
        let bare = MetadataType::class_named("Fixture", "Bare");
        let again = MetadataType::class_named("Fixture", "BareAgain");
        let pointer = MetadataType::class_named("Fixture", "Pointer");
        let indirect = MetadataType::PtrMut(Box::new(bare.clone()), 1);
        let constant = MetadataType::PtrConst(Box::new(bare.clone()), 1);
        let fields: Vec<_> = index
            .expect("Fixture", "FunctionTypes")
            .fields()
            .map(|field| field.ty())
            .collect();
        assert_eq!(
            &fields[..10],
            &[
                bare.clone(),
                again.clone(),
                pointer.clone(),
                indirect.clone(),
                constant.clone(),
                MetadataType::ArrayFixed(Box::new(bare.clone()), 2),
                MetadataType::class_named("Fixture", "Dispatcher"),
                MetadataType::class_named("Fixture", "Factory"),
                MetadataType::class_named("Fixture", "NoThrow"),
                MetadataType::class_named("Fixture", "StdcallBare"),
            ]
        );
        assert!(matches!(fields[10], MetadataType::FnPtr(_)));
        let no_throw = MetadataType::class_named("Fixture", "NoThrow");
        let no_throw_address = MetadataType::PtrMut(Box::new(no_throw.clone()), 1);
        assert_eq!(
            &fields[11..],
            &[
                no_throw_address.clone(),
                MetadataType::ArrayFixed(Box::new(no_throw), 2),
                MetadataType::value_named("Fixture", "NoThrowIndirect"),
            ]
        );
        assert_eq!(
            index.expect("Fixture", "NoThrowIndirect").underlying_type(),
            Some(no_throw_address.clone())
        );
        assert_eq!(
            index.expect("Fixture", "Indirect").underlying_type(),
            Some(indirect.clone())
        );
        let invoke = |name| {
            index
                .expect("Fixture", name)
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap()
        };
        for name in ["Bare", "BareAgain", "Pointer", "OtherBare", "NoThrow"] {
            let parameter = invoke(name).params().next().unwrap();
            assert_eq!(parameter.name(), "value");
            assert_eq!(parameter.direction(), reader::ParamDirection::Input);
            assert!(parameter.find_attribute("NativeSalAttribute").is_none());
        }
        assert_eq!(invoke("Factory").signature(&[]).return_type, bare);
        assert_eq!(
            invoke("Dispatcher").signature(&[]).types,
            [
                bare.clone(),
                bare,
                again,
                pointer,
                indirect,
                constant,
                MetadataType::class_named("Fixture", "OtherBare"),
                MetadataType::PtrMut(Box::new(MetadataType::I32), 1),
                no_throw_address,
            ]
        );
        let convention = index
            .expect("Fixture", "StdcallBare")
            .find_attribute("UnmanagedFunctionPointerAttribute")
            .unwrap()
            .value()
            .into_iter()
            .next()
            .unwrap()
            .1;
        let windows_metadata::Value::EnumValue(_, convention) = convention else {
            panic!()
        };
        assert_eq!(*convention, windows_metadata::Value::I32(1));
        assert!(
            index
                .expect("Fixture", "NoThrow")
                .find_attribute("NativeExceptionSpecificationAttribute")
                .is_some()
        );
    });
    let mut plain = selected_fixture("function_typedefs", &[]);
    plain.source.insert_str(0, "#define CONTRACT\n");
    let plain = project(&[plain]);
    let text = &plain.partitions["function_typedefs"].rdl;
    assert!(text.contains("type Indirect = *mut Bare;"), "{text}");
    assert!(
        text.contains("type NoThrowIndirect = *mut NoThrow;"),
        "{text}"
    );
    assert!(
        text.contains("inline_pointer: extern \"C\" fn(*mut i32) -> Count"),
        "{text}"
    );
}

#[test]
fn function_typedefs_conflicts() {
    helpers::ensure_libclang();
    for (original, replacement, message) in [
        (
            "typedef Count Bare(CONTRACT int* value);",
            "typedef Count (*Bare)(CONTRACT int* value);",
            "incompatible native bindings",
        ),
        ("_In_", "_Out_", "callback parameter annotations differ"),
        (
            "int* value",
            "int* renamed",
            "conflicting public names or contracts",
        ),
    ] {
        let mut inputs = [
            selected_fixture("function_typedefs", &["Bare"]),
            selected_fixture("function_typedefs", &["Bare"]),
        ];
        inputs[1].name = "other.h".into();
        inputs[1].source = inputs[1].source.replace(original, replacement);
        for _ in 0..2 {
            let error = projection::generate(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains(message), "{error}");
            let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
            assert!(error.to_string().contains(message), "{error}");
            inputs.reverse();
        }
    }
    let mut inputs = [
        selected_fixture("function_typedefs", &["Bare"]),
        selected_fixture("function_typedefs", &["Bare"]),
    ];
    inputs[1].name = "other.h".into();
    for input in &mut inputs {
        input.source = input.source.replace("int* value", "volatile int* value");
    }
    inputs[1].source = inputs[1].source.replace("_In_", "_Out_");
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("callback parameter annotations differ"),
            "{error}"
        );
        inputs.reverse();
    }
    let output = projection::generate_partial(
        &[selected_fixture("function_typedef_rejections", &[])],
        "Fixture",
    )
    .unwrap();
    assert_eq!((output.output.records, output.output.aliases), (1, 0));
    for name in ["Volatile", "Varargs", "Method", "RejectedFunction"] {
        assert!(
            output.omissions.iter().any(|error| error.contains(name)),
            "{name}: {:?}",
            output.omissions
        );
    }
}

#[test]
fn implicit_aliases() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("implicit_aliases", &[]),
        selected_fixture("implicit_aliases", &[]),
    ];
    inputs[1].name = "other.h".into();
    for snapshot in capture(&inputs, &TARGETS).unwrap() {
        let alias = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "__builtin_va_list")
            .unwrap();
        assert_eq!(alias.usr, "c:@T@__builtin_va_list");
        assert_eq!(alias.observations.len(), 1);
        let observation = &alias.observations[0];
        assert!(observation.implicit);
        assert!(snapshot.files[observation.location.file.0].is_empty());
        assert!(matches!(
            snapshot.types[observation.canonical.0].kind,
            TypeKind::Pointer(_)
        ));
    }
    let output = project(&inputs);
    assert_eq!(
        (output.records, output.aliases, output.functions),
        (1, 3, 1)
    );
    inputs.reverse();
    assert_eq!(output, project(&inputs));
    roundtrip("implicit_aliases", &output, |index, _| {
        assert_eq!(
            index.expect("Fixture", "Arguments").underlying_type(),
            Some(MetadataType::PtrMut(Box::new(MetadataType::I8), 1))
        );
        for (owner, name) in [("Apis", "NativeSum"), ("ArgumentCallback", "Invoke")] {
            let signature = index
                .expect("Fixture", owner)
                .methods()
                .find(|method| method.name() == name)
                .unwrap()
                .signature(&[]);
            assert_eq!(
                signature.types,
                [MetadataType::value_named("Fixture", "Arguments")]
            );
        }
    });
    assert!(
        output
            .partitions
            .values()
            .all(|partition| !partition.rdl.contains("__builtin"))
    );
    let mut input = selected_fixture("implicit_aliases", &[]);
    input
        .source
        .insert_str(0, "typedef __builtin_va_list __builtin_va_list;\n");
    for snapshot in capture(std::slice::from_ref(&input), &TARGETS).unwrap() {
        let alias = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "__builtin_va_list")
            .unwrap();
        assert_eq!(alias.observations.len(), 2);
        assert!(alias.observations[0].implicit);
        assert!(!alias.observations[1].implicit);
    }
    let mut inputs = [input, selected_fixture("implicit_aliases", &[])];
    inputs[1].name = "other.h".into();
    let output = project(&inputs);
    assert_eq!(output.aliases, 4);
    assert!(
        output
            .partitions
            .values()
            .any(|partition| partition.rdl.contains("type __builtin_va_list = *mut i8;"))
    );
    inputs.reverse();
    assert_eq!(output, project(&inputs));
    let error = capture(
        &[selected_fixture("implicit_aliases", &["__builtin_va_list"])],
        &TARGETS,
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("requested declaration"),
        "{error}"
    );
}

#[test]
fn implicit_aliases_conflicts() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("implicit_aliases", &["Arguments"]),
        selected_fixture("implicit_aliases", &["Arguments"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source = inputs[1].source.replace("__builtin_va_list", "void*");
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
        assert!(error.to_string().contains("incompatible"), "{error}");
        inputs.reverse();
    }
    let input = Input {
        name: "local_alias.h".into(),
        source: "struct Owner { typedef __builtin_va_list Arguments; }; \
            typedef Owner::Arguments Hidden;"
            .into(),
        roots: vec!["Hidden".into()],
        ..Default::default()
    };
    let error = capture(&[input], &TARGETS).unwrap_err();
    assert!(
        error.to_string().contains("not a global type alias"),
        "{error}"
    );
}

fn roundtrip(name: &str, output: &projection::Output, check: impl Fn(&reader::Index, i32)) {
    let text: String = output
        .partitions
        .values()
        .map(|partition| partition.rdl.as_str())
        .collect();
    let path = input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&path, &text).unwrap();
    }
    let expected = std::fs::read_to_string(path).unwrap();
    assert_eq!(text, expected.replace("\r\n", "\n"));
    roundtrip_metadata(name, &text, check);
}

fn roundtrip_metadata(name: &str, text: &str, check: impl Fn(&reader::Index, i32)) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang2-breadth-{name}.winmd"));
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("roundtrip.winmd");
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
    input.macros = [
        "SELECTED_MACRO",
        "UNNAMED_VALUE",
        "ANONYMOUS_MACRO",
        "MIDL_MACRO",
    ]
    .map(String::from)
    .into();
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 9);
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
            (
                "ANONYMOUS_MACRO",
                MetadataType::I16,
                windows_metadata::Value::I16(-2),
            ),
            (
                "MIDL_MACRO",
                MetadataType::U16,
                windows_metadata::Value::U16(u16::MAX),
            ),
        ] {
            let field = apis.fields().find(|field| field.name() == name).unwrap();
            assert_eq!(field.ty(), ty);
            assert_eq!(field.constant().unwrap().value(), value);
        }
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
    for attribute in ["align(8)", "deprecated(\"legacy\")"] {
        let mut input = selected_fixture("enum_constants", &["ANONYMOUS_MACRO"]);
        input.macros = vec!["ANONYMOUS_MACRO".into()];
        input.source = input.source.replace(
            "enum : short",
            &format!("enum __declspec({attribute}) : short"),
        );
        assert!(projection::generate(std::slice::from_ref(&input), "Fixture").is_err());
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.constants, 0);
        assert!(!partial.omissions.is_empty());
    }
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
fn native_enum_types() {
    helpers::ensure_libclang();
    let input = selected_fixture("native_enum_types", &[]);
    let output = project(&[input]);
    assert_eq!(
        (output.enums, output.constants, output.functions),
        (1, 2, 1)
    );
    roundtrip("native_enum_types", &output, |index, _| {
        let native = index.expect("Fixture", "_Options");
        let fields: Vec<_> = native
            .fields()
            .filter(|field| field.constant().is_some())
            .collect();
        assert_eq!(
            fields
                .iter()
                .map(|field| (field.name(), field.constant().unwrap().value()))
                .collect::<Vec<_>>(),
            [
                ("OPTION_NONE", windows_metadata::Value::I32(0)),
                ("OPTION_HIGH", windows_metadata::Value::I32(i32::MIN)),
                ("OPTION_ALL", windows_metadata::Value::I32(-1)),
            ]
        );
        for field in fields {
            assert_eq!(field.ty(), MetadataType::value_named("Fixture", "_Options"));
        }
        let options = index.expect("Fixture", "Options");
        assert_eq!(options.fields().next().unwrap().ty(), MetadataType::U32);
        let apis = index.expect("Fixture", "Apis");
        for (name, ty, value) in [
            ("ENUM_VALUE", "_Options", windows_metadata::Value::I32(-1)),
            (
                "INTEGER_VALUE",
                "Options",
                windows_metadata::Value::U32(u32::MAX),
            ),
        ] {
            let field = apis.fields().find(|field| field.name() == name).unwrap();
            assert_eq!(field.ty(), MetadataType::value_named("Fixture", ty));
            assert_eq!(field.constant().unwrap().value(), value);
        }
    });
}

#[test]
fn byte_aligned_qualifiers() {
    helpers::ensure_libclang();
    let input = selected_fixture("byte_aligned_qualifiers", &[]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let qualified: Vec<_> = snapshot
            .types
            .iter()
            .filter(|ty| ty.qualifiers.unaligned)
            .collect();
        assert!(!qualified.is_empty());
        assert!(qualified.iter().all(|ty| ty.layout.unwrap().alignment == 1));
        let alias = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "PackedPointer")
            .unwrap();
        let TypeKind::Pointer(pointee) = snapshot.types[alias.observations[0].canonical.0].kind
        else {
            panic!()
        };
        assert!(snapshot.types[pointee.0].qualifiers.unaligned);
    }
    let output = project(std::slice::from_ref(&input));
    let mut plain = selected_fixture("byte_aligned_qualifiers", &[]);
    plain.source.insert_str(0, "#define PLAIN\n");
    assert_eq!(project(&[plain]), output);
    roundtrip("byte_aligned_qualifiers", &output, |index, _| {
        let uses = index.expect("Fixture", "Uses");
        assert_eq!(uses.fields().count(), 7);
        for (name, ty) in [
            (
                "PackedPointer",
                MetadataType::PtrMut(Box::new(MetadataType::value_named("Fixture", "Packed")), 1),
            ),
            (
                "ConstPackedPointer",
                MetadataType::PtrConst(Box::new(MetadataType::value_named("Fixture", "Packed")), 1),
            ),
        ] {
            assert_eq!(
                index.expect("Fixture", name).fields().next().unwrap().ty(),
                ty
            );
        }
    });
}

#[test]
fn noreturn() {
    helpers::ensure_libclang();
    let input = selected_fixture(
        "noreturn",
        &[
            "NativeSum",
            "ReadPayload",
            "BufferFunction",
            "NormalizeFunction",
            "StoppingCallbacks",
            "IStopping",
        ],
    );
    for snapshot in capture(std::slice::from_ref(&input), &TARGETS).unwrap() {
        for value in &snapshot.values {
            let observation = &value.observations[0];
            let TypeKind::Function(function) = &snapshot.types[observation.canonical.0].kind else {
                panic!()
            };
            assert_eq!(function.noreturn, value.name != "BufferFunction");
            assert_eq!(
                observation
                    .attributes
                    .iter()
                    .any(|attribute| attribute.kind == AttributeKind::Cxx11NoReturn),
                matches!(value.name.as_str(), "BufferFunction" | "NormalizeFunction")
            );
        }
    }
    let output = project(&[input]);
    roundtrip("noreturn", &output, |index, _| {
        for method in index.expect("Fixture", "Apis").methods() {
            assert_eq!(
                method
                    .attributes()
                    .filter(|attribute| attribute.name() == "DoesNotReturnAttribute")
                    .count(),
                1
            );
        }
        for method in index.expect("Fixture", "IStopping").methods() {
            assert_eq!(
                method.has_attribute("DoesNotReturnAttribute"),
                method.name() != "Ordinary"
            );
        }
        assert!(
            index
                .expect("Fixture", "StoppingCallback")
                .has_attribute("DoesNotReturnAttribute")
        );
        assert!(
            !index
                .expect("Fixture", "OrdinaryCallback")
                .has_attribute("DoesNotReturnAttribute")
        );
        let callbacks = index.expect("Fixture", "StoppingCallbacks");
        let inferred = callbacks
            .fields()
            .find(|field| field.name() == "inferred")
            .unwrap();
        assert_eq!(
            inferred.ty(),
            MetadataType::class_named("Fixture", "StoppingCallback")
        );
    });
}

#[test]
fn noreturn_conflicts() {
    helpers::ensure_libclang();
    for root in [
        "NativeSum",
        "BufferFunction",
        "StoppingCallbacks",
        "IStopping",
    ] {
        let mut inputs = [
            selected_fixture("noreturn", &[root]),
            selected_fixture("noreturn", &[root]),
        ];
        inputs[1].name = "returning.h".into();
        inputs[1].source = inputs[1]
            .source
            .replace("__declspec(noreturn)", "")
            .replace("__attribute__((noreturn))", "")
            .replace("[[noreturn]]", "");
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            assert!(merge(&snapshots).is_err(), "{root}");
            assert!(
                projection::generate_partial(&inputs, "Fixture").is_err(),
                "{root}"
            );
            inputs.reverse();
        }
    }
    let input = selected_fixture("noreturn_unnamed", &[]);
    let error = projection::generate(std::slice::from_ref(&input), "Fixture")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("noreturn function pointer requires a named callback"),
        "{error}"
    );
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!((partial.output.records, partial.output.aliases), (0, 1));
    for name in ["UnnamedStopping", "Dependent"] {
        assert!(
            partial.omissions.iter().any(|error| error.contains(name)),
            "{name}"
        );
    }
}

#[test]
fn import_targets() {
    helpers::ensure_libclang();
    let input = selected_fixture("import_targets", &["Entry"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let mut imports = projection::Imports::default();
    for snapshot in &snapshots {
        let ValueKind::Function { mangling, .. } = &snapshot.values[0].observations[0].kind else {
            panic!()
        };
        let (symbol, ordinal, library) = match snapshot.target {
            Target::X86 => ("_Entry@4", 17, "x86.dll"),
            Target::X64 => ("Entry", 29, "x64.dll"),
            Target::Arm64 => ("Entry", 41, "arm64.dll"),
        };
        assert_eq!(mangling, symbol);
        imports.symbols.entry(snapshot.target).or_default().insert(
            symbol.into(),
            projection::Import {
                library: library.into(),
                name: Some(format!("#{ordinal}")),
            },
        );
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    roundtrip("import_targets", &output, |index, bit| {
        let method = index
            .expect("Fixture", "Apis")
            .methods()
            .find(|method| method.arches() == 0 || method.arches() & bit != 0)
            .unwrap();
        assert_eq!(
            method.impl_map().unwrap().import_name(),
            match bit {
                1 => "#17",
                2 => "#29",
                4 => "#41",
                _ => unreachable!(),
            }
        );
        assert_eq!(
            method.impl_map().unwrap().import_scope().name(),
            match bit {
                1 => "x86.dll",
                2 => "x64.dll",
                4 => "arm64.dll",
                _ => unreachable!(),
            }
        );
    });
    let x86 = imports.symbols.get_mut(&Target::X86).unwrap();
    let entry = x86.remove("_Entry@4").unwrap();
    x86.insert("Entry".into(), entry);
    assert!(
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).is_err()
    );
    let partial = projection::project_partial_with_imports(
        capture_partial(&[selected_fixture("import_targets", &["Entry"])], &TARGETS).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    assert_eq!(partial.output.functions, 1);
    assert!(partial.omissions.iter().all(|line| line.contains("X86:")));
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("Entry"))
    );
    imports.functions.insert(
        "Entry".into(),
        projection::Import {
            library: "override.dll".into(),
            name: Some("NamedEntry".into()),
        },
    );
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    roundtrip_metadata("import_override", &text, |index, _| {
        for method in index.expect("Fixture", "Apis").methods() {
            let import = method.impl_map().unwrap();
            assert_eq!(import.import_name(), "NamedEntry");
            assert_eq!(import.import_scope().name(), "override.dll");
        }
    });
    for name in ["", "#", "#-1", "#65536"] {
        imports.functions.get_mut("Entry").unwrap().name = Some(name.into());
        assert!(
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports)
                .is_err()
        );
    }
    let mut changed = selected_fixture("import_targets", &["Entry"]);
    changed.name = "changed.h".into();
    changed.source = changed
        .source
        .replace("int __stdcall Entry", "long __stdcall Entry");
    assert!(
        projection::generate_partial(
            &[selected_fixture("import_targets", &["Entry"]), changed],
            "Fixture",
        )
        .is_err()
    );
}

#[test]
#[ignore = "requires the pinned Windows SDK and import libraries"]
fn sdk_import_targets() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_import_targets.h".into();
    input.source = test_clang2::fixture("sdk_import_targets").source;
    input.roots = ["ILClone", "ILFree", "WSAStartup"].map(String::from).into();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let mut imports = projection::Imports::default();
    for snapshot in &snapshots {
        let (arch, machine) = match snapshot.target {
            Target::X86 => ("x86", 0x14c),
            Target::X64 => ("x64", 0x8664),
            Target::Arm64 => ("arm64", 0xaa64),
        };
        let root = helpers::nuget_package(
            &format!("microsoft.windows.sdk.cpp.{arch}"),
            "10.0.28000.2270",
        )
        .join("c")
        .join("um")
        .join(arch);
        let mut symbols = BTreeMap::new();
        for library in ["shell32.lib", "ws2_32.lib"] {
            for import in
                windows_rdl::implib::read(&std::fs::read(root.join(library)).unwrap()).unwrap()
            {
                if import.machine == machine && import.kind == windows_rdl::implib::ImportKind::Code
                {
                    symbols.insert(import.symbol.clone(), import);
                }
            }
        }
        for value in &snapshot.values {
            let ValueKind::Function { mangling, .. } = &value.observations[0].kind else {
                panic!()
            };
            let import = &symbols[mangling];
            let expected = match value.name.as_str() {
                "ILClone" => 18,
                "ILFree" => 155,
                "WSAStartup" => 115,
                _ => unreachable!(),
            };
            assert_eq!(
                import.name,
                windows_rdl::implib::ImportName::Ordinal(expected)
            );
            imports.symbols.entry(snapshot.target).or_default().insert(
                mangling.clone(),
                projection::Import {
                    library: import.dll.clone(),
                    name: Some(format!("#{expected}")),
                },
            );
        }
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 3);
    roundtrip("sdk_import_targets", &output, |index, bit| {
        for method in index
            .expect("Fixture", "Apis")
            .methods()
            .filter(|method| method.arches() == 0 || method.arches() & bit != 0)
        {
            let (ordinal, library) = match method.name() {
                "ILClone" => ("#18", "SHELL32.dll"),
                "ILFree" => ("#155", "SHELL32.dll"),
                "WSAStartup" => ("#115", "WS2_32.dll"),
                _ => unreachable!(),
            };
            let import = method.impl_map().unwrap();
            assert_eq!(import.import_name(), ordinal);
            assert_eq!(import.import_scope().name(), library);
        }
    });
}

#[test]
#[ignore = "requires the pinned Windows SDK and import libraries"]
fn sdk_import_gaps() {
    helpers::ensure_libclang();
    let input = || {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "sdk_import_gaps.h".into();
        input.source = test_clang2::fixture("sdk_import_gaps").source;
        input.roots = ["DhcpAddServer", "DnsValidateName_W", "BSTR_UserSize64"]
            .map(String::from)
            .into();
        input
    };
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    let mut imports = projection::Imports::default();
    for snapshot in &snapshots {
        let (arch, machine) = match snapshot.target {
            Target::X86 => ("x86", 0x14c),
            Target::X64 => ("x64", 0x8664),
            Target::Arm64 => ("arm64", 0xaa64),
        };
        let root = helpers::nuget_package(
            &format!("microsoft.windows.sdk.cpp.{arch}"),
            "10.0.28000.2270",
        )
        .join("c")
        .join("um")
        .join(arch);
        let symbols = imports.symbols.entry(snapshot.target).or_default();
        for library in ["dhcpsapi.lib", "dnsapi.lib", "oleaut32.lib"] {
            for import in
                windows_rdl::implib::read(&std::fs::read(root.join(library)).unwrap()).unwrap()
            {
                if import.machine != machine || import.kind != windows_rdl::implib::ImportKind::Code
                {
                    continue;
                }
                let name = match import.name {
                    windows_rdl::implib::ImportName::Name(name) => name,
                    windows_rdl::implib::ImportName::Ordinal(ordinal) => format!("#{ordinal}"),
                };
                symbols.insert(
                    import.symbol,
                    projection::Import {
                        library: import.dll,
                        name: Some(name),
                    },
                );
            }
        }
        for value in &snapshot.values {
            let ValueKind::Function { mangling, .. } = &value.observations[0].kind else {
                panic!()
            };
            assert_eq!(
                symbols.contains_key(mangling),
                snapshot.target != Target::X86
            );
            if snapshot.target == Target::X86 {
                match value.name.as_str() {
                    "DhcpAddServer" => {
                        assert_eq!(mangling, "_DhcpAddServer");
                        assert!(symbols.contains_key("_DhcpAddServer@20"));
                    }
                    "DnsValidateName_W" => {
                        assert_eq!(mangling, "_DnsValidateName_W");
                        assert!(symbols.contains_key("_DnsValidateName_W@8"));
                    }
                    "BSTR_UserSize64" => assert!(
                        symbols
                            .values()
                            .all(|import| import.name.as_deref() != Some("BSTR_UserSize64"))
                    ),
                    _ => unreachable!(),
                }
            }
        }
    }
    assert!(
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).is_err()
    );
    let partial = projection::project_partial_with_imports(
        capture_partial(&[input()], &TARGETS).unwrap(),
        "Fixture",
        &imports,
    )
    .unwrap();
    assert_eq!(partial.output.functions, 3);
    assert!(partial.omissions.iter().all(|line| line.contains("X86:")));
    for name in ["DhcpAddServer", "DnsValidateName_W", "BSTR_UserSize64"] {
        assert!(partial.omissions.iter().any(|error| error.contains(name)));
    }
    roundtrip("sdk_unresolved_imports", &partial.output, |index, bit| {
        let methods: Vec<_> = index
            .expect("Fixture", "Apis")
            .methods()
            .filter(|method| method.arches() == 0 || method.arches() & bit != 0)
            .collect();
        assert_eq!(methods.len(), if bit == 1 { 0 } else { 3 });
        for method in methods {
            let library = match method.name() {
                "DhcpAddServer" => "dhcpsapi.dll",
                "DnsValidateName_W" => "dnsapi.dll",
                "BSTR_UserSize64" => "oleaut32.dll",
                _ => unreachable!(),
            };
            let import = method.impl_map().unwrap();
            assert_eq!(import.import_name(), method.name());
            assert!(import.import_scope().name().eq_ignore_ascii_case(library));
        }
    });
}

#[test]
fn import_conventions() {
    helpers::ensure_libclang();
    let snapshots = capture(&[selected_fixture("import_conventions", &[])], &TARGETS).unwrap();
    let imports = ["Cdecl", "Shared", "Varying", "Only64", "Changing"]
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "example.dll".into(),
                    name: None,
                },
            )
        })
        .into_iter()
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    roundtrip("import_conventions", &output, |index, _| {
        for (name, abi) in [
            ("CdeclCallback", 2),
            ("SharedCallback", 1),
            ("VaryingCallback", 1),
        ] {
            let callback = index.expect("Fixture", name);
            let attribute = callback
                .find_attribute("UnmanagedFunctionPointerAttribute")
                .unwrap();
            let windows_metadata::Value::EnumValue(_, value) = attribute.value().remove(0).1 else {
                panic!()
            };
            assert_eq!(*value, windows_metadata::Value::I32(abi));
        }
    });
    for snapshot in &snapshots {
        for alias in &snapshot.aliases {
            let TypeKind::Pointer(pointee) = snapshot.types[alias.observations[0].canonical.0].kind
            else {
                panic!()
            };
            let TypeKind::Function(function) = &snapshot.types[pointee.0].kind else {
                panic!()
            };
            assert_eq!(
                function.convention,
                if snapshot.target == Target::X86 && alias.name != "CdeclCallback" {
                    CallingConvention::Stdcall
                } else {
                    CallingConvention::C
                }
            );
        }
        let error = projection::project_with_imports(
            &merge(std::slice::from_ref(snapshot)).unwrap(),
            "Fixture",
            &imports,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "projection requires x86, x64, and ARM64 observations",
        );
        for value in &snapshot.values {
            let TypeKind::Function(function) =
                &snapshot.types[value.observations[0].canonical.0].kind
            else {
                panic!()
            };
            assert_eq!(
                function.convention,
                if snapshot.target == Target::X86 && value.name != "Cdecl" {
                    CallingConvention::Stdcall
                } else {
                    CallingConvention::C
                }
            );
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_resource_callbacks() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_resource_callbacks.h".into();
    input.source = test_clang2::fixture("sdk_resource_callbacks").source;
    input.roots = ["ENUMRESNAMEPROCA", "ENUMRESTYPEPROCW"]
        .map(String::from)
        .into();
    let output = project(&[input]);
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    for name in ["ENUMRESNAMEPROCA", "ENUMRESTYPEPROCW"] {
        assert_eq!(rdl.matches(&format!("fn {name}(")).count(), 1);
        assert!(rdl.contains(&format!("extern fn {name}(")));
    }
    assert!(rdl.contains("lpName: LPSTR"));
    assert!(rdl.contains("lpType: LPWSTR"));
    roundtrip("sdk_resource_callbacks", &output, |index, _| {
        for name in ["ENUMRESNAMEPROCA", "ENUMRESTYPEPROCW"] {
            let callback = index.expect("Fixture", name);
            for parameter in callback
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap()
                .params()
                .filter(|parameter| matches!(parameter.name(), "lpName" | "lpType"))
            {
                assert_eq!(parameter.direction(), reader::ParamDirection::Input);
                assert_eq!(
                    parameter
                        .find_attribute("NativeInputAttribute")
                        .unwrap()
                        .value(),
                    [("ReadOnly".into(), windows_metadata::Value::Bool(true))]
                );
            }
            let attribute = callback
                .find_attribute("UnmanagedFunctionPointerAttribute")
                .unwrap();
            let windows_metadata::Value::EnumValue(_, value) = attribute.value().remove(0).1 else {
                panic!()
            };
            assert_eq!(*value, windows_metadata::Value::I32(1));
        }
    });
}

#[test]
fn calling_defaults() {
    helpers::ensure_libclang();
    for stdcall in [false, true] {
        let mut input = selected_fixture("calling_defaults", &[]);
        if stdcall {
            input
                .arguments
                .extend(["-Xarch_i686".into(), "-mrtd".into()]);
        }
        let snapshots = capture(&[input], &TARGETS).unwrap();
        for snapshot in &snapshots {
            let expected = |name: &str| {
                if snapshot.target == Target::X86
                    && (name.starts_with("Stdcall") || stdcall && name.starts_with("Plain"))
                {
                    CallingConvention::Stdcall
                } else {
                    CallingConvention::C
                }
            };
            for value in &snapshot.values {
                let TypeKind::Function(function) =
                    &snapshot.types[value.observations[0].canonical.0].kind
                else {
                    panic!()
                };
                assert_eq!(function.convention, expected(&value.name));
                let ValueKind::Function { mangling, .. } = &value.observations[0].kind else {
                    panic!()
                };
                let name = &value.name;
                assert_eq!(
                    mangling,
                    &if snapshot.target != Target::X86 {
                        name.clone()
                    } else if expected(name) == CallingConvention::Stdcall {
                        format!("_{name}@4")
                    } else {
                        format!("_{name}")
                    }
                );
            }
            for alias in &snapshot.aliases {
                let TypeKind::Pointer(pointee) =
                    &snapshot.types[alias.observations[0].canonical.0].kind
                else {
                    panic!()
                };
                let TypeKind::Function(function) = &snapshot.types[pointee.0].kind else {
                    panic!()
                };
                assert_eq!(function.convention, expected(&alias.name));
            }
        }
        let imports = ["Plain", "Cdecl", "Stdcall"]
            .map(|name| {
                (
                    name.into(),
                    projection::Import {
                        library: "example.dll".into(),
                        name: None,
                    },
                )
            })
            .into_iter()
            .collect();
        let output =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports)
                .unwrap();
        roundtrip(
            if stdcall {
                "calling_defaults_stdcall"
            } else {
                "calling_defaults_cdecl"
            },
            &output,
            |index, bit| {
                for name in ["Plain", "Cdecl", "Stdcall"] {
                    let expected = if name == "Stdcall" || stdcall && name == "Plain" && bit == 1 {
                        "system"
                    } else {
                        "C"
                    };
                    let method = index
                        .expect("Fixture", "Apis")
                        .methods()
                        .find(|method| {
                            method.name() == name
                                && (method.arches() == 0 || method.arches() & bit != 0)
                        })
                        .unwrap();
                    assert_eq!(method.calling_convention(), expected);
                }
            },
        );
    }
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_noreturn() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_noreturn.h".into();
    input.source = test_clang2::fixture("sdk_noreturn").source;
    input.roots = [
        "ExitProcess",
        "ExitThread",
        "FreeLibraryAndExitThread",
        "RpcRaiseException",
    ]
    .map(String::from)
    .into();
    let imports = input
        .roots
        .iter()
        .map(|name| {
            (
                name.clone(),
                projection::Import {
                    library: if name == "RpcRaiseException" {
                        "rpcrt4.dll"
                    } else {
                        "kernel32.dll"
                    }
                    .into(),
                    name: None,
                },
            )
        })
        .collect();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 4);
    roundtrip("sdk_noreturn", &output, |index, bit| {
        let apis = index.expect("Fixture", "Apis");
        let methods: Vec<_> = apis
            .methods()
            .filter(|method| method.arches() == 0 || method.arches() & bit != 0)
            .collect();
        assert_eq!(methods.len(), 4);
        for method in methods {
            assert!(method.has_attribute("DoesNotReturnAttribute"));
            assert_eq!(method.signature(&[]).return_type, MetadataType::Void);
        }
    });
}

#[test]
fn byte_aligned_qualifier_conflicts() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("byte_aligned_qualifiers", &["Uses"]),
        selected_fixture("byte_aligned_qualifiers", &["Uses"]),
    ];
    inputs[1].name = "plain.h".into();
    inputs[1].source.insert_str(0, "#define PLAIN\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
    for name in [
        "UnalignedForward",
        "UnalignedVoid",
        "UnalignedInt",
        "UnalignedPointer",
        "VolatileByte",
    ] {
        let input = selected_fixture("byte_aligned_rejections", &[name]);
        assert!(
            projection::generate(std::slice::from_ref(&input), "Fixture").is_err(),
            "{name}"
        );
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.aliases, 0, "{name}");
        assert!(!partial.omissions.is_empty(), "{name}");
    }
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_byte_aligned_qualifiers() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_byte_aligned_qualifiers.h".into();
    input.source = test_clang2::fixture("sdk_byte_aligned_qualifiers").source;
    input.roots = ["LPCITEMIDLIST", "LPSHITEMID", "LPITEMIDLIST"]
        .map(String::from)
        .into();
    let output = project(&[input]);
    roundtrip("sdk_byte_aligned_qualifiers", &output, |index, _| {
        for (name, constant) in [
            ("LPCITEMIDLIST", true),
            ("LPSHITEMID", false),
            ("LPITEMIDLIST", false),
        ] {
            let ty = index.expect("Fixture", name).fields().next().unwrap().ty();
            assert_eq!(matches!(ty, MetadataType::PtrConst(..)), constant, "{name}");
            assert!(
                matches!(ty, MetadataType::PtrMut(..) | MetadataType::PtrConst(..)),
                "{name}"
            );
        }
    });
}

#[test]
fn scalar_constant_aliases() {
    helpers::ensure_libclang();
    let roots = ["SHARED", "UNIQUE", "BOOLEAN", "FLOATING"];
    let mut inputs = [
        selected_fixture("scalar_constant_aliases", &roots),
        selected_fixture("scalar_constant_aliases", &roots),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(0, "#define OTHER\n");
    for input in &mut inputs {
        input.macros = ["SHARED_MACRO", "BYTE_MACRO"].map(String::from).into();
    }
    let output = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), output);
    roundtrip("scalar_constant_aliases", &output, |index, _| {
        let apis = index.expect("Fixture", "Apis");
        for (name, ty, value) in [
            ("SHARED", MetadataType::U32, windows_metadata::Value::U32(7)),
            (
                "UNIQUE",
                MetadataType::value_named("Fixture", "ULongA"),
                windows_metadata::Value::U32(9),
            ),
            (
                "BOOLEAN",
                MetadataType::Bool,
                windows_metadata::Value::Bool(true),
            ),
            (
                "FLOATING",
                MetadataType::F64,
                windows_metadata::Value::F64(1.25),
            ),
            (
                "SHARED_MACRO",
                MetadataType::U32,
                windows_metadata::Value::U32(7),
            ),
            (
                "BYTE_MACRO",
                MetadataType::U8,
                windows_metadata::Value::U8(255),
            ),
        ] {
            let field = apis.fields().find(|field| field.name() == name).unwrap();
            assert_eq!(field.ty(), ty);
            assert_eq!(field.constant().unwrap().value(), value);
        }
    });
    for input in &mut inputs {
        input.roots = vec!["SHARED".into()];
        input.macros.clear();
    }
    inputs[0].source.insert_str(0, "#define DECLARATION_ONLY\n");
    let declared = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), declared);
    let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, declared);
    for input in &mut inputs {
        input.source.insert_str(0, "#define VALUE_ANNOTATED\n");
    }
    let annotated = project(&inputs);
    let text: String = annotated
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    roundtrip_metadata("scalar_constant_annotations", &text, |index, _| {
        let field = index
            .expect("Fixture", "Apis")
            .fields()
            .find(|field| field.name() == "SHARED")
            .unwrap();
        assert_eq!(field.ty(), MetadataType::U32);
        assert!(field.find_attribute("NativeDeprecatedAttribute").is_some());
    });
}

#[test]
fn scalar_constant_alias_conflicts() {
    helpers::ensure_libclang();
    for (root, define, native) in [
        ("SHARED", "NATIVE_MISMATCH", true),
        ("SHARED", "VALUE_MISMATCH", true),
        ("SHARED", "ANNOTATION_MISMATCH", true),
        ("FLOATING", "FLOAT_MISMATCH", true),
        ("SHARED", "ANNOTATED", false),
        ("POINTER", "", false),
        ("ENUM_VALUE", "", false),
    ] {
        let mut inputs = [
            selected_fixture("scalar_constant_aliases", &[root]),
            selected_fixture("scalar_constant_aliases", &[root]),
        ];
        inputs[1].name = "other.h".into();
        inputs[1].source.insert_str(0, "#define OTHER\n");
        if !define.is_empty() {
            for input in &mut inputs {
                input.source.insert_str(0, &format!("#define {define}\n"));
            }
        }
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            assert_eq!(merge(&snapshots).is_err(), native, "{root}: {define}");
            assert!(projection::generate(&inputs, "Fixture").is_err());
            let partial = projection::generate_partial(&inputs, "Fixture");
            if native {
                assert!(partial.is_err(), "{root}: {define}");
            } else {
                let partial = partial.unwrap();
                assert_eq!(partial.output.constants, 0, "{root}: {define}");
                assert!(
                    partial
                        .omissions
                        .iter()
                        .any(|omission| omission.contains("conflicting public names or contracts")),
                    "{root}: {define}: {:?}",
                    partial.omissions
                );
            }
            inputs.reverse();
        }
    }
}

#[test]
fn scalar_constant_alias_resolved_annotations() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("scalar_constant_aliases", &["SHARED"]),
        selected_fixture("scalar_constant_aliases", &["SHARED"]),
        selected_fixture("scalar_constant_alias_annotations", &["ULongA"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(0, "#define OTHER\n");
    for _ in 0..inputs.len() {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        for snapshot in &snapshots {
            let annotated = snapshot.input == "scalar_constant_alias_annotations.h";
            assert_eq!(
                snapshot
                    .aliases
                    .iter()
                    .flat_map(|alias| &alias.observations)
                    .any(|observation| !observation.attributes.is_empty()),
                annotated
            );
            if annotated {
                assert!(snapshot.values.is_empty());
            }
        }
        let merged = merge(&snapshots).unwrap();
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("conflicting public names or contracts"),
            "{error}"
        );
        let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
        assert_eq!(partial.output.constants, 0);
        assert!(
            partial
                .omissions
                .iter()
                .any(|omission| omission.contains("conflicting public names or contracts"))
        );
        inputs.rotate_left(1);
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
    assert_eq!(output.constants, 14);
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
            ("DEVICE_KEY", 0x123456789abcdef00102030405060708),
            (
                "ARCH_DEVICE_KEY",
                u128::from(if bit == 1 { 4_u32 } else { 8 }) << 96,
            ),
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
        for (name, id) in [
            ("DEVICE_KEY", u32::MAX),
            ("ARCH_DEVICE_KEY", if bit == 1 { 4 } else { 8 }),
        ] {
            assert_eq!(
                field(name).ty(),
                MetadataType::ValueName(windows_metadata::TypeName::named("Fixture", "DEVPROPKEY"))
            );
            assert_eq!(
                field(name).constant().unwrap().value(),
                windows_metadata::Value::U32(id)
            );
        }
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
    for (old, new) in [
        ("DEVPROPID pid", "long pid"),
        ("DEVPROPID pid", "unsigned short pid"),
        ("DEVPROPKEY", "OtherKey"),
    ] {
        let mut input = selected_fixture("aggregate_constants", &["DEVICE_KEY"]);
        input.source = input
            .source
            .replace("0xffffffff};", "42};")
            .replace(old, new);
        assert!(projection::generate(std::slice::from_ref(&input), "Fixture").is_err());
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.constants, 0);
        assert!(!partial.omissions.is_empty());
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
fn sdk_guid_context() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_guid_context.h".into();
    input.source = test_clang2::fixture("sdk_guid_context").source;
    input.roots = ["CLSID_CFontPropPage", "GUID_HIMETRIC", "GUID_HANDLE"]
        .map(String::from)
        .into();
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 3);
    roundtrip("sdk_guid_context", &output, |index, _| {
        let apis = index.expect("Fixture", "Apis");
        for (name, expected) in [
            ("CLSID_CFontPropPage", 0x0be352008f9111ce9de300aa004bb851),
            ("GUID_HIMETRIC", 0x66504300be0f101a8bbb00aa00300cab),
            ("GUID_HANDLE", 0x66504313be0f101a8bbb00aa00300cab),
        ] {
            let field = apis.fields().find(|field| field.name() == name).unwrap();
            assert_eq!(field.ty(), MetadataType::value_named("Fixture", "GUID"));
            let actual = field
                .find_attribute("GuidAttribute")
                .unwrap()
                .value()
                .into_iter()
                .fold(0_u128, |value, (_, part)| match part {
                    windows_metadata::Value::U32(part) => (value << 32) | u128::from(part),
                    windows_metadata::Value::U16(part) => (value << 16) | u128::from(part),
                    windows_metadata::Value::U8(part) => (value << 8) | u128::from(part),
                    _ => panic!(),
                });
            assert_eq!(actual, expected);
        }
    });
    let declaration = Input {
        name: "guid_declarations.h".into(),
        source: input.source.replace("#define INITGUID", "#undef INITGUID"),
        arguments: input.arguments.clone(),
        roots: input.roots.clone(),
        ..Default::default()
    };
    assert!(projection::generate(std::slice::from_ref(&declaration), "Fixture").is_err());
    let mut inputs = [input, declaration];
    let merged = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), merged);
    assert_eq!(merged, output);
    let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
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
        "MAX_ERROR_MESSAGE_CHARS",
        "EXCEPTION_RO_ORIGINATEERROR",
        "EXCEPTION_RO_TRANSFORMERROR",
        "CLSCTX_VALID_MASK",
    ]
    .map(String::from)
    .into();
    let output = project(std::slice::from_ref(&input));
    assert_eq!(output.constants, 17);
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
    roundtrip("sdk_constants", &output, |index, _| {
        use windows_metadata::Value::*;
        let apis = index.expect("Fixture", "Apis");
        let field = |name| apis.fields().find(|field| field.name() == name).unwrap();
        assert_eq!(
            field("CLSCTX_VALID_MASK").ty(),
            MetadataType::value_named("Fixture", "CLSCTX")
        );
        for name in ["EXCEPTION_RO_ORIGINATEERROR", "EXCEPTION_RO_TRANSFORMERROR"] {
            assert_eq!(
                field(name).ty(),
                MetadataType::value_named("Fixture", "DWORD")
            );
        }
        for (name, expected) in [
            ("ERROR_ACCESS_DENIED", I32(5)),
            ("GENERIC_READ", U32(0x80000000)),
            ("INVALID_FILE_ATTRIBUTES", U32(u32::MAX)),
            ("WAIT_TIMEOUT", I32(258)),
            ("S_OK", I32(0)),
            ("E_FAIL", I32(-2147467259)),
            ("FILE_ATTRIBUTE_NORMAL", I32(128)),
            ("MAX_ERROR_MESSAGE_CHARS", I32(512)),
            ("EXCEPTION_RO_ORIGINATEERROR", U32(0x40080201)),
            ("EXCEPTION_RO_TRANSFORMERROR", U32(0x40080202)),
            ("CLSCTX_VALID_MASK", I32(-1612712929)),
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
        for (name, kind) in [
            ("ReadPayload", AttributeKind::DllImport),
            ("NativeSum", AttributeKind::DllExport),
        ] {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            assert!(value.observations.iter().all(|observation| {
                observation
                    .attributes
                    .iter()
                    .any(|attribute| attribute.kind == kind)
            }));
        }
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
            ..
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
        let exported = apis
            .methods()
            .find(|method| method.name() == "NativeSum")
            .unwrap();
        let map = exported.impl_map().unwrap();
        assert_eq!(map.import_scope().name(), "example.dll");
        assert_eq!(map.import_name(), "NativeSum");
        assert_eq!(exported.calling_convention(), "C");
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
        assert_eq!(method.calling_convention(), "system");
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
fn dll_export_contracts() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("free_functions", &[]),
        selected_fixture("free_functions", &[]),
    ];
    inputs[1].name = "other.h".into();
    let expected = project(&inputs);
    inputs.reverse();
    assert_eq!(project(&inputs), expected);
    for replacement in ["", "__declspec(dllimport)"] {
        let original = inputs[0].source.clone();
        inputs[0].source = original.replace("__declspec(dllexport)", replacement);
        for _ in 0..2 {
            assert!(capture_partial(&inputs, &TARGETS).is_err());
            inputs.reverse();
        }
        inputs[0].source = original;
    }
    let mut ownership = imports();
    ownership.functions.remove("NativeSum");
    let output = projection::project_partial_with_imports(
        capture_partial(&inputs, &TARGETS).unwrap(),
        "Fixture",
        &ownership,
    )
    .unwrap();
    assert_eq!(output.output.functions, 1);
    assert!(output.omissions.iter().any(|error| {
        error.contains("NativeSum") && error.contains("no configured DLL/export ownership")
    }));
    for source in [
        "extern \"C\" __declspec(dllexport) const int Exported = 42;",
        "struct __declspec(dllexport) Exported { int field; };",
    ] {
        let input = Input {
            name: "exported_storage.h".into(),
            source: source.into(),
            ..Default::default()
        };
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(output.output.constants + output.output.records, 0);
        assert!(!output.omissions.is_empty());
    }
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
fn packed_interfaces() {
    helpers::ensure_libclang();
    let mut expected = None;
    for packing in [1, 2, 8, 16] {
        let mut input = selected_fixture("packed_interfaces", &["IPackedChild", "IPackedRoot"]);
        input
            .source
            .insert_str(0, &format!("#define PACK {packing}\n"));
        let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
        for snapshot in &snapshots {
            let size = if snapshot.target == Target::X86 { 4 } else { 8 };
            for (name, bases, methods, first_slot) in [
                ("IPacked", 1, 1, 3),
                ("IPackedChild", 1, 1, 4),
                ("IPackedRoot", 0, 3, 0),
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
                        alignment: size.min(packing)
                    }
                );
                assert!(definition.fields.is_empty());
                assert_eq!(definition.bases.len(), bases);
                assert!(definition.bases.iter().all(|base| base.offset_bits == 0));
                assert_eq!(definition.methods.len(), methods);
                for (index, method) in definition.methods.iter().enumerate() {
                    assert_eq!(method.slot, first_slot + index as u64);
                }
            }
        }
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        assert_eq!(output.interfaces, 4);
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        } else {
            expected = Some(output);
        }
        let mut invalid = snapshots;
        let record = invalid[0]
            .records
            .iter()
            .find(|record| record.name == "IPacked")
            .unwrap();
        let id = record.observations[0].0;
        let layout = invalid[0].observations[id]
            .definition
            .as_ref()
            .unwrap()
            .layout;
        for invalid_size in [false, true] {
            let definition = invalid[0].observations[id].definition.as_mut().unwrap();
            definition.layout = layout;
            if invalid_size {
                definition.layout.size += 1;
            } else {
                definition.layout.alignment = 3;
            }
            assert!(
                projection::project(&merge(&invalid).unwrap(), "Fixture")
                    .unwrap_err()
                    .to_string()
                    .contains("requires one vptr")
            );
        }
    }
    roundtrip("packed_interfaces", &expected.unwrap(), |index, _| {
        assert_eq!(
            index
                .expect("Fixture", "IPacked")
                .methods()
                .next()
                .unwrap()
                .name(),
            "Read"
        );
        let child = index.expect("Fixture", "IPackedChild");
        assert_eq!(child.methods().next().unwrap().name(), "Count");
        assert!(child.fields().next().is_none());
        assert!(child.class_layout().is_none());
    });
    for (first, second) in [(1, 2), (8, 16)] {
        let mut inputs = [first, second].map(|packing| {
            let mut input = selected_fixture("packed_interfaces", &["IPackedChild"]);
            input
                .source
                .insert_str(0, &format!("#define PACK {packing}\n"));
            input
        });
        inputs[1].name = "other.h".into();
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("record layout or field count differs"),
                "{error}"
            );
            inputs.reverse();
        }
    }
    for name in ["IData", "IMultiple", "IAligned"] {
        let input = selected_fixture("packed_interfaces_invalid", &[name]);
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert!(
            partial.omissions.iter().any(|error| error.contains(name)),
            "{name}"
        );
        assert!(
            !partial
                .output
                .partitions
                .values()
                .any(|part| part.rdl.contains(name)),
            "{name}"
        );
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
        let output = projection::project_with_imports(&merged, "Fixture", &imports()).unwrap();
        let partial = projection::project_partial_with_imports(
            capture_partial(&inputs, &TARGETS).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        assert_eq!(partial.output, output);
        assert_eq!(partial.output.functions, 2);
        assert_eq!(partial.output.constants, 1);
        assert!(partial.omissions.is_empty());
        if let Some(expected) = &expected {
            assert_eq!(&partial, expected);
        } else {
            expected = Some(partial);
        }
        inputs.reverse();
    }
}

#[test]
fn function_parameter_names() {
    helpers::ensure_libclang();
    let mut inputs: Vec<_> = (0..4)
        .map(|variant| {
            let mut input = selected_fixture("function_names", &[]);
            input.name = format!("names_{variant}.h");
            input
                .source
                .insert_str(0, &format!("#define VARIANT {variant}\n"));
            input
        })
        .collect();
    let mut targets = TARGETS;
    let mut expected = None;
    for _ in 0..4 {
        let snapshots = capture(&inputs, &targets).unwrap();
        let output =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
                .unwrap();
        assert_eq!(output.functions, 1);
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        } else {
            roundtrip("function_names", &output, |index, _| {
                let method = index
                    .expect("Fixture", "Apis")
                    .methods()
                    .find(|method| method.name() == "NativeSum")
                    .unwrap();
                let names: Vec<_> = method
                    .params()
                    .map(|parameter| parameter.name().to_owned())
                    .collect();
                assert_eq!(names, ["alpha", "zeta"]);
            });
            expected = Some(output);
        }
        inputs.rotate_left(1);
        targets.reverse();
    }
    let mut inputs = [
        selected_fixture("function_redeclarations_a", &["ReadPayload"]),
        selected_fixture("function_redeclarations_b", &["ReadPayload"]),
    ];
    inputs[1].source = inputs[1]
        .source
        .replace("typedef int INT;", "typedef int* POINTER;")
        .replace("INT* value", "POINTER value");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let output =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
                .unwrap();
        assert_eq!(output.functions, 1);
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
        "int Cxx(int value);",
        "int MUTABLE = 3;",
        r#"extern "C" int Next(); const int SIDE_EFFECT = (Next(), 3);"#,
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

const FIELD_CONTRACTS: [(&str, bool, bool, bool, bool); 12] = [
    ("_Field_size_", false, false, false, false),
    ("_Field_size_opt_", false, true, false, false),
    ("_Field_size_full_", false, false, true, false),
    ("_Field_size_full_opt_", false, true, true, false),
    ("_Field_size_part_", false, false, false, true),
    ("_Field_size_part_opt_", false, true, false, true),
    ("_Field_size_bytes_", true, false, false, false),
    ("_Field_size_bytes_opt_", true, true, false, false),
    ("_Field_size_bytes_full_", true, false, true, false),
    ("_Field_size_bytes_full_opt_", true, true, true, false),
    ("_Field_size_bytes_part_", true, false, false, true),
    ("_Field_size_bytes_part_opt_", true, true, false, true),
];

fn field_buffer_input(name: &str, arguments: &str) -> Input {
    let mut input = test_clang2::fixture("field_buffers");
    input.source.insert_str(
        0,
        &format!("#define ANNOTATION \"{name}\"\n#define ARGUMENTS \"{arguments}\"\n"),
    );
    input
}

#[test]
fn field_buffers() {
    use windows_metadata::Value;
    helpers::ensure_libclang();
    for (case, (name, bytes, optional, full, partial)) in FIELD_CONTRACTS.into_iter().enumerate() {
        let arguments = if partial {
            "capacity, readable"
        } else {
            "capacity"
        };
        let input = field_buffer_input(name, arguments);
        let output = project(&[input]);
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(!text.contains("NativeSal("), "{text}");
        let check = |index: &reader::Index, _| {
            let field = index
                .expect("Fixture", "FieldBuffers")
                .fields()
                .find(|field| field.name() == "values")
                .unwrap();
            let mut expected: BTreeMap<_, _> = [
                ("CapacityFieldName".into(), Value::Utf8("capacity".into())),
                ("IsByteCount".into(), Value::Bool(bytes)),
                ("MayBeNull".into(), Value::Bool(optional)),
            ]
            .into();
            if full || partial {
                expected.insert(
                    "ReadableFieldName".into(),
                    Value::Utf8(if full { "capacity" } else { "readable" }.into()),
                );
            }
            assert_eq!(
                field
                    .find_attribute("NativeFieldBufferAttribute")
                    .unwrap()
                    .value()
                    .into_iter()
                    .collect::<BTreeMap<_, _>>(),
                expected
            );
        };
        if case == 0 {
            roundtrip("field_buffers", &output, check);
        } else {
            roundtrip_metadata(&format!("field_buffers_{case}"), &text, check);
        }
    }
    let mut inline = field_buffer_input("_Field_size_", "capacity");
    inline
        .source
        .insert_str(0, "#define BUFFER_FIELD unsigned short values[8]\n");
    let output = project(&[inline]);
    roundtrip("field_buffers_inline", &output, |_, _| {});
}

#[test]
#[ignore = "requires the pinned SDK"]
fn field_buffers_sdk() {
    helpers::ensure_libclang();
    for (name, _, _, _, partial) in FIELD_CONTRACTS {
        let arguments = if partial {
            "capacity, readable"
        } else {
            "capacity"
        };
        let expected = field_buffer_input(name, arguments);
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name.clone_from(&expected.name);
        input.roots = ["Count", "FieldBuffers", "FieldAlias", "OuterFields"]
            .map(String::from)
            .into();
        input.source = format!(
            "#include <sal.h>\n{}",
            expected.source.replace(
                "CONTRACT BUFFER_FIELD",
                &format!("{name}({arguments}) BUFFER_FIELD")
            )
        );
        assert_eq!(project(&[input]), project(&[expected]), "{name}");
    }
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_field_buffers.h".into();
    input.source = test_clang2::fixture("sdk_field_buffers").source;
    input.roots = ["AUTHZ_ACCESS_REPLY", "BCRYPT_MULTI_HASH_OPERATION"]
        .map(String::from)
        .into();
    let output = project(&[input]);
    roundtrip("sdk_field_buffers", &output, |index, _| {
        for (record, field, count, optional) in [
            (
                "AUTHZ_ACCESS_REPLY",
                "GrantedAccessMask",
                "ResultListLength",
                false,
            ),
            (
                "AUTHZ_ACCESS_REPLY",
                "SaclEvaluationResults",
                "ResultListLength",
                true,
            ),
            ("AUTHZ_ACCESS_REPLY", "Error", "ResultListLength", false),
            ("BCRYPT_MULTI_HASH_OPERATION", "pbBuffer", "cbBuffer", false),
        ] {
            let field = index
                .expect("Fixture", record)
                .fields()
                .find(|f| f.name() == field)
                .unwrap();
            assert!(field.find_attribute("NativeSalAttribute").is_none());
            assert_eq!(
                field
                    .find_attribute("NativeFieldBufferAttribute")
                    .unwrap()
                    .value(),
                [
                    (
                        "CapacityFieldName".into(),
                        windows_metadata::Value::Utf8(count.into())
                    ),
                    ("IsByteCount".into(), windows_metadata::Value::Bool(false)),
                    ("MayBeNull".into(), windows_metadata::Value::Bool(optional)),
                ]
            );
        }
    });
}

#[test]
fn field_buffer_agreement() {
    helpers::ensure_libclang();
    for public in [false, true] {
        for (name, arguments, agrees) in [
            ("_Field_size_part_", "capacity, capacity", true),
            ("_Field_size_full_", " capacity ", true),
            ("_Field_size_", "capacity", false),
            ("_Field_size_full_opt_", "capacity", false),
            ("_Field_size_bytes_full_", "capacity", false),
            ("_Field_size_part_", "capacity, readable", false),
        ] {
            let mut inputs = [
                field_buffer_input("_Field_size_full_", "capacity"),
                field_buffer_input(name, arguments),
            ];
            inputs[1].name = "other.h".into();
            for input in &mut inputs {
                input.roots = vec!["FieldAlias".into()];
            }
            if public {
                inputs[1].source.insert_str(0, "#define TAG OtherFields\n");
            }
            for _ in 0..2 {
                if agrees {
                    project(&inputs);
                } else {
                    let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
                    assert!(
                        error
                            .to_string()
                            .contains(if public { "incompatible" } else { "differ" }),
                        "{error}"
                    );
                }
                inputs.reverse();
            }
        }
    }
}

#[test]
fn field_buffer_residuals() {
    helpers::ensure_libclang();
    for (name, arguments) in [
        ("_Field_size_", "42"),
        ("_Field_size_", "capacity + 1"),
        ("_Field_size_", "inner.capacity"),
        ("_Field_size_", "*capacity"),
        ("_Field_size_", ""),
        ("_Field_size_part_", "capacity, 1"),
        ("_Field_size_part_", "capacity, missing"),
    ] {
        let output = project(&[field_buffer_input(name, arguments)]);
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(text.contains("NativeSal("), "{text}");
        assert!(!text.contains("NativeFieldBuffer("), "{text}");
    }
    for (change, reason) in [
        ("#define COUNT_TYPE float\n", "requires an integer field"),
        (
            "#define COUNT_TYPE unsigned int*\n",
            "requires an integer field",
        ),
        (
            "#define BUFFER_FIELD unsigned short values\n",
            "requires data pointer or array storage",
        ),
        (
            "#define BUFFER_FIELD void (*values)()\n",
            "requires data pointer or array storage",
        ),
    ] {
        let mut input = field_buffer_input("_Field_size_", "capacity");
        input.source.insert_str(0, change);
        input.roots = vec!["FieldAlias".into()];
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(output.output.records, 0);
        assert!(
            output.omissions.iter().any(|error| error.contains(reason)),
            "{:?}",
            output.omissions
        );
    }
    let mut input = field_buffer_input("_Field_size_", "capacity");
    input.source.insert_str(0, "#define VERSION \"1\"\n");
    let output = project(&[input]);
    assert!(
        !output
            .partitions
            .values()
            .any(|p| p.rdl.contains("NativeFieldBuffer("))
    );
    let mut input = field_buffer_input("_Field_size_", "capacity");
    input.source = input
        .source
        .replace("COUNT_TYPE capacity;", "unsigned int capacity : 8;");
    let output = project(&[input]);
    assert!(
        !output
            .partitions
            .values()
            .any(|p| p.rdl.contains("NativeFieldBuffer("))
    );
    let mut input = field_buffer_input("_Field_size_", "capacity");
    input.source = input.source.replace(
        "CONTRACT BUFFER_FIELD",
        "CONTRACT __attribute__((annotate(\"windows_clang:sal:2:_Field_size_opt_\\n(capacity)\\n(capacity)\"))) BUFFER_FIELD",
    );
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        let field = snapshot
            .observations
            .iter()
            .filter_map(|observation| observation.definition.as_ref())
            .flat_map(|definition| &definition.fields)
            .find(|field| field.name == "values")
            .unwrap();
        assert_eq!(
            field
                .attributes
                .iter()
                .filter(|attribute| matches!(attribute.kind, AttributeKind::Sal(_)))
                .count(),
            2
        );
    }
    let output = project(&[input]);
    let text: String = output.partitions.values().map(|p| p.rdl.as_str()).collect();
    assert!(text.contains("NativeSal("), "{text}");
    assert!(!text.contains("NativeFieldBuffer("), "{text}");
    let mut input = field_buffer_input("_Field_size_", "capacity");
    input.source = input.source.replace("        COUNT_TYPE capacity;", "");
    let output = project(&[input]);
    let text: String = output.partitions.values().map(|p| p.rdl.as_str()).collect();
    roundtrip_metadata("field_buffers_outer_count", &text, |index, _| {
        let inner = index
            .nested(index.expect("Fixture", "OuterFields"))
            .next()
            .unwrap();
        let values = inner
            .fields()
            .find(|field| field.name() == "values")
            .unwrap();
        assert!(values.find_attribute("NativeSalAttribute").is_some());
        assert!(
            values
                .find_attribute("NativeFieldBufferAttribute")
                .is_none()
        );
    });
}
