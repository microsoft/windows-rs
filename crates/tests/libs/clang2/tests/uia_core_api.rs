use std::collections::BTreeMap;
use windows_clang2::*;
use windows_metadata::reader;

const FUNCTIONS: [&str; 9] = [
    "UiaHostProviderFromHwnd",
    "UiaReturnRawElementProvider",
    "UiaGetReservedNotSupportedValue",
    "UiaGetReservedMixedAttributeValue",
    "UiaRaiseAutomationEvent",
    "UiaRaiseAutomationPropertyChangedEvent",
    "UiaClientsAreListening",
    "UiaDisconnectProvider",
    "UiaLookupId",
];
const IDS: [(&str, i64); 5] = [
    ("UiaRootObjectId", -25),
    ("UIA_NamePropertyId", 30005),
    ("UIA_InvokePatternId", 10000),
    ("UIA_ButtonControlTypeId", 50000),
    ("UIA_Invoke_InvokedEventId", 20009),
];

#[test]
#[ignore = "requires the pinned Windows SDK, import libraries and native companion"]
fn sdk_provider_apis_and_ids() {
    helpers::ensure_libclang();
    let fixture = test_clang2::fixture("sdk_uia_core_api");
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = FUNCTIONS
        .iter()
        .copied()
        .chain(IDS[1..].iter().map(|(name, _)| *name))
        .map(String::from)
        .collect();
    input.macros = vec!["UiaRootObjectId".into()];
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let mut imports = projection::Imports::default();
    for snapshot in &snapshots {
        for (name, expected) in IDS {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            for observation in &value.observations {
                if name != "UiaRootObjectId" {
                    assert!(
                        std::path::Path::new(&snapshot.files[observation.location.file.0])
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .eq_ignore_ascii_case("uiautomationclient.h")
                    );
                }
                assert!(matches!(
                    observation.kind,
                    ValueKind::Constant {
                        value: Some(ConstantValue::Integer(EnumValue::Signed(value))), ..
                    } if value == expected
                ));
                assert_eq!(
                    snapshot.types[observation.canonical.0].kind,
                    TypeKind::Scalar(if name == "UiaRootObjectId" {
                        Scalar::Int
                    } else {
                        Scalar::Long
                    })
                );
            }
        }
        let (arch, machine) = match snapshot.target {
            Target::X86 => ("x86", 0x14c),
            Target::X64 => ("x64", 0x8664),
            Target::Arm64 => ("arm64", 0xaa64),
        };
        let library = helpers::nuget_package(
            &format!("microsoft.windows.sdk.cpp.{arch}"),
            "10.0.28000.2270",
        )
        .join("c")
        .join("um")
        .join(arch)
        .join("uiautomationcore.lib");
        let symbols: BTreeMap<_, _> = windows_rdl::implib::read(&std::fs::read(library).unwrap())
            .unwrap()
            .into_iter()
            .filter(|import| {
                import.machine == machine && import.kind == windows_rdl::implib::ImportKind::Code
            })
            .map(|import| (import.symbol.clone(), import))
            .collect();
        for name in FUNCTIONS {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            for observation in &value.observations {
                let ValueKind::Function {
                    mangling, alias, ..
                } = &observation.kind
                else {
                    panic!()
                };
                assert!(alias.is_none());
                let import = &symbols[mangling];
                assert_eq!(
                    import.name,
                    windows_rdl::implib::ImportName::Name(name.into())
                );
                assert!(import.dll.eq_ignore_ascii_case("uiautomationcore.dll"));
                imports.symbols.entry(snapshot.target).or_default().insert(
                    mangling.clone(),
                    projection::Import {
                        library: import.dll.clone(),
                        name: Some(name.into()),
                    },
                );
            }
        }
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, FUNCTIONS.len());
    assert_eq!(output.constants, IDS.len());
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let golden = test_clang2::input_dir().join("sdk_uia_core_api.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-uia-core-api.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
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
        for architecture in [1, 2, 4] {
            let index = reader::Index::new_for_architecture(
                vec![reader::File::read(path).unwrap()],
                architecture,
            );
            let apis = index.expect("Fixture", "Apis");
            for name in FUNCTIONS {
                let method = apis.methods().find(|method| method.name() == name).unwrap();
                let import = method.impl_map().unwrap();
                assert_eq!(import.import_name(), name);
                assert!(
                    import
                        .import_scope()
                        .name()
                        .eq_ignore_ascii_case("uiautomationcore.dll")
                );
            }
            for (name, value) in IDS {
                let field = apis.fields().find(|field| field.name() == name).unwrap();
                assert_eq!(field.ty(), windows_metadata::Type::I32);
                assert_eq!(
                    field.constant().unwrap().value(),
                    windows_metadata::Value::I32(i32::try_from(value).unwrap())
                );
            }
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
}
