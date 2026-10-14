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
        assert_eq!(definition.member_templates.len(), 1);
        assert_eq!(definition.member_templates[0].name, "QueryInterface");
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
                value: EnumValue::Unsigned(1099511627779),
                origin: ConstantOrigin::Macro,
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
            let partial = capture_partial(&inputs, &TARGETS).unwrap();
            assert!(merge(&partial.snapshots).is_err());
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
