use super::*;
use windows_metadata::reader::{HasAttributes, Item, TypeCategory};

const VERSION: &str = "10.0.28000.1839";
const KERNEL_SOURCE: &str = "
#if defined(_M_AMD64)
#define _AMD64_
#define AMD64
#elif defined(_M_ARM64)
#define _ARM64_
#define ARM64
#elif defined(_M_IX86)
#define _X86_
#define i386 1
#else
#error Unsupported WDK target
#endif
#define INITGUID
#include <ntifs.h>
#include <wdm.h>
";

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let scratch = Path::new("target\\win32\\wdk");
    std::fs::create_dir_all(scratch)?;
    let winrt = Path::new("crates\\libs\\default\\Windows.winmd");
    let um = scratch.join("Windows.Win32.um.winmd");
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input("metadata\\win32")
        .reference(winrt)
        .output(&um)
        .write()?;
    let metadata = windows_metadata::reader::Index::new(vec![
        windows_metadata::reader::File::read(&um).ok_or("invalid SDK metadata")?,
        windows_metadata::reader::File::read(winrt).ok_or("invalid WinRT metadata")?,
    ]);
    let references = projection::References::new(&metadata);
    let external_interfaces = references.native_interfaces()?;
    let sdk = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
        .join("c\\Include")
        .join(helpers::marketing_dir(SDK_VERSION));
    let wdk = helpers::nuget_package("microsoft.windows.wdk.x64", VERSION)
        .join("c\\Include")
        .join(helpers::marketing_dir(VERSION));
    let resource = std::path::PathBuf::from(helpers::clang_resource_dir());
    let mut kernel_arguments = vec!["-DNTDDI_VERSION=0x0A000010".into()];
    for directory in ["km", "shared"] {
        kernel_arguments.extend(["-isystem".into(), wdk.join(directory).display().to_string()]);
    }
    kernel_arguments.extend(sdk_arguments(&sdk, &resource)?);
    let mut inputs = vec![
        Input {
            name: "wdk-kernel.h".into(),
            source: KERNEL_SOURCE.into(),
            arguments: kernel_arguments.clone(),
            macro_headers: ["ntifs.h", "wdm.h", "ntddk.h", "mce.h", "ntnls.h"]
                .map(String::from)
                .into(),
            root_paths: Some(vec![wdk.join("km")]),
            ..Default::default()
        },
        Input {
            name: "wdk-offreg.h".into(),
            source: format!("{SUPPORT_PRELUDE}{PRELUDE}#include <offreg.h>\n"),
            arguments: kernel_arguments,
            macro_headers: vec!["offreg.h".into()],
            root_paths: Some(vec![wdk.join("km\\offreg.h")]),
            ..Default::default()
        },
    ];
    let mut evidence = String::new();
    let mut discovered = BTreeMap::new();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        discovered.insert(target, read_imports(target, &mut evidence)?);
    }
    let function_symbols = discovered
        .iter()
        .map(|(target, imports)| {
            (
                *target,
                imports
                    .values()
                    .filter(|import| import.kind == windows_rdl::implib::ImportKind::Code)
                    .map(|import| import.symbol.clone())
                    .collect(),
            )
        })
        .collect::<BTreeMap<_, BTreeSet<_>>>();
    let mut configuration = format!(
        "WDK: {VERSION}\nSDK: {SDK_VERSION}\nLLVM: {}\nScope: ntdll and offreg user-mode imports\nSDK reference: {}\n",
        helpers::LIBCLANG_VERSION,
        um.display(),
    );
    for input in &mut inputs {
        input.external_interfaces.clone_from(&external_interfaces);
        input.function_symbols = Some(function_symbols.clone());
        configuration.push_str(&format!(
            "\nInput: {}\nArguments: {:?}\nMacro headers: {:?}\nRoot paths: {:?}\nSource:\n{}\n",
            input.name, input.arguments, input.macro_headers, input.root_paths, input.source,
        ));
    }
    std::fs::write(scratch.join("capture-configuration.txt"), configuration)?;
    println!("Capturing WDK headers for x86, x64, and ARM64");
    let captured =
        windows_clang::capture_partial(&inputs, &[Target::X86, Target::X64, Target::Arm64])?;
    let mut imports = projection::Imports::default();
    let named = named_exports::Evidence::load()?;
    for (target, discovered) in discovered {
        evidence.push_str(&add_sdk_imports(
            &mut imports,
            target,
            discovered,
            &named,
            &captured.snapshots,
        )?);
    }
    std::fs::write(scratch.join("import-omissions.txt"), evidence)?;
    let aliases = BTreeMap::new();
    let result = projection::project_partial_with_options(
        captured,
        "Windows.Win32",
        &projection::Options {
            imports: Some(&imports),
            references: Some(&references),
            function_aliases: Some(&aliases),
        },
    )?;
    std::fs::write(
        scratch.join("omissions.txt"),
        result.omissions.join("\n") + "\n",
    )?;
    let output = result.output;
    if output.partitions.is_empty() {
        return Err("no WDK declarations could be projected; output is unchanged".into());
    }
    windows_rdl::reader()
        .reference(&um)
        .reference(winrt)
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(scratch.join("Windows.Win32.km.winmd"))
        .write()?;
    write_partitions(Path::new("metadata\\wdk"), &output.partitions)?;
    println!(
        "WDK projection: {} records, {} enums, {} aliases, {} constants, {} functions, {} interfaces, {} partitions; {} omissions",
        output.records,
        output.enums,
        output.aliases,
        output.constants,
        output.functions,
        output.interfaces,
        output.partitions.len(),
        result.omissions.len(),
    );
    Ok(())
}

fn read_imports(
    target: Target,
    evidence: &mut String,
) -> Result<BTreeMap<String, windows_rdl::implib::Import>, Box<dyn std::error::Error>> {
    let (arch, machine) = import_target(target);
    let sdk = helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION);
    let wdk = helpers::nuget_package(&format!("microsoft.windows.wdk.{arch}"), VERSION);
    let libraries = [
        sdk.join("c").join("um").join(arch).join("ntdll.lib"),
        wdk.join("c\\Lib")
            .join(helpers::marketing_dir(VERSION))
            .join("km")
            .join(arch)
            .join("offreg.lib"),
    ];
    let mut discovered = BTreeMap::new();
    for library in libraries {
        if target == Target::X86 && library.file_name().unwrap() == "offreg.lib" {
            if library.exists() {
                return Err("x86 offreg.lib is now available; update WDK import policy".into());
            }
            evidence.push_str("X86: pinned WDK package does not provide offreg.lib\n");
            continue;
        }
        let bytes =
            std::fs::read(&library).map_err(|error| format!("{}: {error}", library.display()))?;
        let members = windows_rdl::implib::read(&bytes)
            .map_err(|error| format!("{}: {error}", library.display()))?;
        let selected = members
            .iter()
            .filter(|import| import.machine == machine)
            .count();
        if selected == 0 {
            return Err(format!("{} has no {target:?} import records", library.display()).into());
        }
        evidence.push_str(&format!(
            "{target:?} {}: {selected} target-native import records\n",
            library.display(),
        ));
        for import in members {
            if import.machine == machine {
                discovered.entry(import.symbol.clone()).or_insert(import);
            } else {
                evidence.push_str(&format!(
                    "{target:?} {}: skipped machine {:#x}\n",
                    library.display(),
                    import.machine,
                ));
            }
        }
    }
    Ok(discovered)
}

fn item_arches(item: Item<'_>) -> i32 {
    let bits = match item {
        Item::Type(value) => value.arches(),
        Item::Fn(value) => value.arches(),
        Item::Const(value) => value.arches(),
    };
    if bits == 0 { 7 } else { bits }
}

pub fn validate_additions(um: &Path, km: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let um = windows_metadata::reader::Index::read(um).ok_or("invalid SDK metadata")?;
    let km = windows_metadata::reader::Index::read(km).ok_or("invalid WDK metadata")?;
    for (namespace, name, addition) in km.iter_items() {
        for existing in um.get_item(namespace, name) {
            if item_arches(existing) & item_arches(addition) == 0
                || matches!(existing, Item::Type(_)) != matches!(addition, Item::Type(_))
            {
                continue;
            }
            let (Item::Type(existing), Item::Type(addition)) = (existing, addition) else {
                return Err(
                    format!("WDK value `{namespace}.{name}` duplicates SDK ownership").into(),
                );
            };
            if existing.category() != TypeCategory::Enum
                || addition.category() != TypeCategory::Enum
                || existing.underlying_type() != addition.underlying_type()
            {
                return Err(
                    format!("WDK type `{namespace}.{name}` duplicates SDK ownership").into(),
                );
            }
            let members: BTreeMap<_, _> = existing
                .fields()
                .filter_map(|field| {
                    field
                        .constant()
                        .map(|constant| (field.name(), constant.value()))
                })
                .collect();
            for field in addition.fields() {
                if let Some(expected) = members.get(field.name())
                    && field
                        .constant()
                        .is_none_or(|constant| constant.value() != *expected)
                {
                    return Err(format!(
                        "WDK enum `{namespace}.{name}` member `{}` conflicts with SDK metadata",
                        field.name(),
                    )
                    .into());
                }
            }
        }
    }
    Ok(())
}

pub fn validate_unique_items(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = windows_metadata::reader::Index::read(path).ok_or("invalid merged metadata")?;
    let mut items = BTreeSet::new();
    for (namespace, name, item) in metadata.iter_items() {
        for bit in [1, 2, 4] {
            if item_arches(item) & bit != 0
                && !items.insert((namespace, name, matches!(item, Item::Type(_)), bit))
            {
                return Err(format!(
                    "duplicate merged metadata item `{namespace}.{name}` on architecture {bit}"
                )
                .into());
            }
        }
    }
    Ok(())
}

#[test]
fn target_native_imports() {
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let mut evidence = String::new();
        let imports = read_imports(target, &mut evidence).unwrap();
        assert!(
            imports
                .values()
                .all(|import| import.machine == import_target(target).1)
        );
        let close = if target == Target::X86 {
            "_NtClose@4"
        } else {
            "NtClose"
        };
        assert!(imports.contains_key(close));
        let offreg = imports
            .values()
            .filter(|import| import.dll.eq_ignore_ascii_case("offreg.dll"))
            .count();
        assert_eq!(offreg == 0, target == Target::X86);
        if target != Target::X86 {
            assert!(imports.contains_key("OROpenHive"));
            assert!(imports.contains_key("ORCloseHive"));
        }
        assert_eq!(
            evidence.contains("does not provide offreg.lib"),
            target == Target::X86
        );
    }
}
