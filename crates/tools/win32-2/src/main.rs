mod coverage;
#[path = "../../win32/src/headers.rs"]
mod headers;
#[path = "../../win32/src/imports.rs"]
mod import_policy;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::time::Instant;
use windows_clang2::{Input, Target, projection};

const SDK_VERSION: &str = "10.0.28000.2270";
const SUPPORT_PRELUDE: &str = "#include <intsafe.h>\n";
const PRELUDE: &str = "#define SECURITY_WIN32
#define WIN32_NO_STATUS
#include <winsock2.h>
#include <windows.h>
#undef WIN32_NO_STATUS
#include <ntstatus.h>
";
const GUID_RESET: &str = "#undef INITGUID\n#include <guiddef.h>\n";

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("metadata\\metadata.rdl").is_file() {
        return Err("run tool-win32-2 from the repository root".into());
    }
    if std::env::args().nth(1).as_deref() == Some("--compare-constants") {
        if std::env::args().len() != 2 {
            return Err("--compare-constants cannot be combined with generation arguments".into());
        }
        return coverage::run();
    }
    let mut headers = Vec::new();
    let mut partial = false;
    let mut sdk_imports = false;
    let mut sdk_macros = false;
    let mut x86_stdcall = false;
    let mut dry_run = false;
    let mut first = None;
    let mut imports = projection::Imports::default();
    let mut macros = Vec::new();
    let mut macro_headers = Vec::new();
    for argument in std::env::args().skip(1) {
        if argument == "--partial" {
            partial = true;
        } else if argument == "--sdk-imports" {
            sdk_imports = true;
        } else if argument == "--sdk-macros" {
            sdk_macros = true;
        } else if argument == "--x86-stdcall" {
            x86_stdcall = true;
        } else if argument == "--dry-run" {
            dry_run = true;
        } else if let Some(value) = argument.strip_prefix("--import=") {
            let (name, library) = value
                .split_once('=')
                .ok_or("use --import=Function=library.dll")?;
            if name.is_empty()
                || library.is_empty()
                || imports
                    .functions
                    .insert(
                        name.to_string(),
                        projection::Import {
                            library: library.into(),
                            name: None,
                        },
                    )
                    .is_some()
            {
                return Err(format!("invalid or duplicate import `{value}`").into());
            }
        } else if let Some(name) = argument.strip_prefix("--macro=") {
            macros.push(name.to_string());
        } else if let Some(header) = argument.strip_prefix("--macro-header=") {
            if header.is_empty() {
                return Err("--macro-header requires a header filename".into());
            }
            macro_headers.push(header.to_string());
        } else if let Some(count) = argument.strip_prefix("--first=") {
            if first.replace(count.parse::<usize>()?).is_some() {
                return Err("--first may only be specified once".into());
            }
        } else if argument.starts_with('-') {
            return Err(format!(
                "unknown option `{argument}`; use --compare-constants alone, or --partial, --sdk-imports, --sdk-macros, --x86-stdcall, --dry-run, --first=N, --import=Function=library.dll, --macro=NAME, --macro-header=header.h, or header names"
            )
            .into());
        } else {
            headers.push(argument);
        }
    }
    if first.is_some() && !headers.is_empty() {
        return Err("--first cannot be combined with explicit header names".into());
    }
    if headers.is_empty() {
        headers.extend(
            headers::HEADERS
                .iter()
                .chain(headers::SATELLITE_HEADERS)
                .map(|header| (*header).to_string()),
        );
    }
    if let Some(count) = first {
        if count == 0 || count > headers.len() {
            return Err(format!("--first must be between 1 and {}", headers.len()).into());
        }
        headers.truncate(count);
    }
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
        .join("c")
        .join("Include")
        .join(helpers::marketing_dir(SDK_VERSION));
    if sdk_macros {
        macro_headers.extend(sdk_macro_headers(&include, &headers)?);
        macro_headers.sort();
        macro_headers.dedup();
    }
    let arguments = sdk_arguments(
        &include,
        Path::new(&helpers::clang_resource_dir()),
        x86_stdcall,
    )?;
    let mut inputs = inputs(&headers, &arguments);
    for input in &mut inputs {
        input.macros.clone_from(&macros);
        input.macro_headers.clone_from(&macro_headers);
    }
    inputs.extend(guid_inputs(&headers, &arguments));
    let started = Instant::now();
    let scratch = Path::new("target\\win32-2");
    std::fs::create_dir_all(scratch)?;
    let mut import_evidence = String::new();
    if sdk_imports {
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let (arch, _) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let discovered = read_sdk_imports(&directory, target, &mut import_evidence)?;
            import_evidence.push_str(&add_sdk_imports(&mut imports, target, discovered));
        }
        for entry in import_policy::LIBRARY_OVERRIDES
            .iter()
            .filter(|entry| entry.sdk_library.is_none())
        {
            imports
                .functions
                .entry(entry.symbol.into())
                .or_insert(projection::Import {
                    library: entry.corrected_library.into(),
                    name: None,
                });
        }
        std::fs::write(scratch.join("import-omissions.txt"), &import_evidence)?;
        let symbols: usize = imports.symbols.values().map(BTreeMap::len).sum();
        let ordinals = imports
            .symbols
            .values()
            .flat_map(BTreeMap::values)
            .filter(|import| {
                import
                    .name
                    .as_deref()
                    .is_some_and(|name| name.starts_with('#'))
            })
            .count();
        println!(
            "SDK ownership: {} named and {ordinals} ordinal target imports; {} explicit public mappings",
            symbols - ordinals,
            imports.functions.len(),
        );
    }
    let mut configuration = format!(
        "SDK: {SDK_VERSION}\nLLVM: {}\nTargets: x86, x64, ARM64\nPartial: {partial}\nCommand arguments: {:?}\n",
        helpers::LIBCLANG_VERSION,
        std::env::args().skip(1).collect::<Vec<_>>(),
    );
    for input in &inputs {
        configuration.push_str(&format!(
            "\nInput: {}\nArguments: {:?}\nMacros: {:?}\nMacro headers: {:?}\nSource:\n{}\n",
            input.name, input.arguments, input.macros, input.macro_headers, input.source,
        ));
    }
    configuration.push_str("\nExplicit imports:\n");
    for (symbol, import) in &imports.functions {
        configuration.push_str(&format!(
            "{symbol}={}!{}\n",
            import.library,
            import.name.as_deref().unwrap_or(symbol),
        ));
    }
    for (target, symbols) in &imports.symbols {
        configuration.push_str(&format!("\n{target:?} linker imports:\n"));
        for (symbol, import) in symbols {
            configuration.push_str(&format!(
                "{symbol}={}!{}\n",
                import.library,
                import.name.as_deref().unwrap_or(symbol),
            ));
        }
    }
    if !import_evidence.is_empty() {
        configuration.push_str("\nSDK import evidence:\n");
        configuration.push_str(&import_evidence);
    }
    std::fs::write(scratch.join("capture-configuration.txt"), configuration)?;
    if dry_run {
        println!("Configuration only; see target\\win32-2\\capture-configuration.txt");
        return Ok(());
    }
    println!(
        "Capturing {} headers in {} translation units for x86, x64, and ARM64",
        headers.len(),
        inputs.len()
    );
    let output = if partial {
        let captured =
            windows_clang2::capture_partial(&inputs, &[Target::X86, Target::X64, Target::Arm64])?;
        let projection_started = Instant::now();
        let result = projection::project_partial_with_imports(captured, "Windows.Win32", &imports)?;
        if std::env::var_os("WINDOWS_CLANG_PROFILE").is_some() {
            eprintln!(
                "clang-profile\tall-targets\tprojection\twall\t1\t{:.6}",
                projection_started.elapsed().as_secs_f64(),
            );
        }
        std::fs::write(
            scratch.join("omissions.txt"),
            result.omissions.join("\n") + "\n",
        )?;
        println!(
            "PARTIAL: {} omission diagnostics; see target\\win32-2\\omissions.txt",
            result.omissions.len()
        );
        result.output
    } else {
        let snapshots =
            windows_clang2::capture(&inputs, &[Target::X86, Target::X64, Target::Arm64])?;
        projection::project_with_imports(
            &windows_clang2::merge(&snapshots)?,
            "Windows.Win32",
            &imports,
        )?
    };
    let generated = started.elapsed();
    if output.partitions.is_empty() {
        return Err("no declarations could be projected; output is unchanged".into());
    }
    println!(
        "Compiling {} records, {} enums, {} aliases, {} constants, {} functions, and {} interfaces from {} partitions",
        output.records,
        output.enums,
        output.aliases,
        output.constants,
        output.functions,
        output.interfaces,
        output.partitions.len()
    );
    let started = Instant::now();
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(scratch.join("Windows.Win32.winmd"))
        .write()?;
    let compiled = started.elapsed();

    let destination = Path::new("metadata\\win32");
    std::fs::create_dir_all(destination)?;
    let previous = std::fs::read_dir(destination)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|extension| extension == "rdl"))
        .collect::<Vec<_>>();
    for path in &previous {
        if !path.is_file() {
            return Err(format!(
                "unexpected entry in generated directory: {}",
                path.display()
            )
            .into());
        }
    }
    for path in previous {
        std::fs::remove_file(path)?;
    }
    for (stem, partition) in &output.partitions {
        std::fs::write(destination.join(format!("{stem}.rdl")), &partition.rdl)?;
    }
    println!(
        "SDK projection: {} records, {} enums, {} aliases, {} constants, {} functions, {} interfaces, {} partitions; capture/merge/project {:.3}s, compile {:.3}s",
        output.records,
        output.enums,
        output.aliases,
        output.constants,
        output.functions,
        output.interfaces,
        output.partitions.len(),
        generated.as_secs_f64(),
        compiled.as_secs_f64()
    );
    println!(
        "Rebuilt {}. Validation WinMD: target\\win32-2\\Windows.Win32.winmd",
        destination.display()
    );
    Ok(())
}

fn sdk_macro_headers(include: &Path, selected: &[String]) -> Result<Vec<String>, std::io::Error> {
    let mut pending = vec![include.join("um"), include.join("shared")];
    let mut headers = BTreeSet::new();
    for header in selected {
        let name = Path::new(header).file_name().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("selected header has no filename: {header}"),
            )
        })?;
        headers.insert(name.to_string_lossy().to_ascii_lowercase());
    }
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            let kind = entry.file_type()?;
            if kind.is_dir() {
                pending.push(path);
            } else if kind.is_file()
                && path
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("h"))
            {
                let name = entry.file_name().into_string().map_err(|name| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("SDK header filename is not UTF-8: {name:?}"),
                    )
                })?;
                headers.insert(name.to_ascii_lowercase());
            }
        }
    }
    Ok(headers.into_iter().collect())
}

fn add_sdk_imports(
    imports: &mut projection::Imports,
    target: Target,
    discovered: BTreeMap<String, windows_rdl::implib::Import>,
) -> String {
    let mut omitted = String::new();
    let symbols = imports.symbols.entry(target).or_default();
    for (symbol, import) in discovered {
        if import.kind != windows_rdl::implib::ImportKind::Code {
            omitted.push_str(&format!(
                "{target:?} {symbol}: {} {:?} import is not a function\n",
                import.dll, import.kind,
            ));
            continue;
        }
        let name = match import.name {
            windows_rdl::implib::ImportName::Name(name) => name,
            windows_rdl::implib::ImportName::Ordinal(ordinal) => format!("#{ordinal}"),
        };
        symbols.insert(
            symbol,
            projection::Import {
                library: import.dll,
                name: Some(name),
            },
        );
    }
    omitted
}

fn import_target(target: Target) -> (&'static str, u16) {
    match target {
        Target::X86 => ("x86", 0x14c),
        Target::X64 => ("x64", 0x8664),
        Target::Arm64 => ("arm64", 0xaa64),
    }
}

fn read_sdk_imports(
    directory: &Path,
    target: Target,
    evidence: &mut String,
) -> Result<BTreeMap<String, windows_rdl::implib::Import>, Box<dyn std::error::Error>> {
    let (arch, machine) = import_target(target);
    let mut imports = BTreeMap::new();
    for library in import_policy::IMPORT_LIBS {
        let path = ["um", "ucrt"]
            .into_iter()
            .map(|part| directory.join(part).join(arch).join(library))
            .find(|path| path.is_file());
        if target == Target::X86 && matches!(*library, "winhvplatform.lib" | "vertdll.lib") {
            if path.is_some() {
                return Err(format!("SDK library `{library}` is now available for x86; update its availability policy").into());
            }
            evidence.push_str(&format!("{target:?}: SDK does not provide {library}\n"));
            continue;
        }
        let path = path.ok_or_else(|| {
            format!(
                "SDK import library `{library}` is missing from {}",
                directory.display()
            )
        })?;
        let bytes = std::fs::read(&path)?;
        let members = windows_rdl::implib::read(&bytes)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let mut other_machines = BTreeMap::<u16, usize>::new();
        for import in members {
            if import.machine != machine {
                *other_machines.entry(import.machine).or_default() += 1;
                continue;
            }
            imports.entry(import.symbol.clone()).or_insert(import);
        }
        for (other, count) in other_machines {
            evidence.push_str(&format!(
                "{target:?} {library}: {count} members for unselected machine {other:#x}\n",
            ));
        }
    }
    for entry in import_policy::LIBRARY_OVERRIDES {
        let mut matches: Vec<_> = imports.values_mut().filter(|import| {
            import.symbol == entry.symbol
                || matches!(&import.name, windows_rdl::implib::ImportName::Name(name) if name == entry.symbol)
        }).collect();
        match (entry.sdk_library, matches.as_mut_slice()) {
            (None, []) => {}
            (Some(expected), [import])
                if import.dll == expected
                    && import.kind == windows_rdl::implib::ImportKind::Code =>
            {
                import.dll = entry.corrected_library.into();
            }
            _ => return Err(format!(
                "{target:?} SDK library mapping for `{}` changed; remove or update the override",
                entry.symbol,
            )
            .into()),
        }
    }
    Ok(imports)
}

fn sdk_arguments(
    include: &Path,
    resource: &Path,
    x86_stdcall: bool,
) -> Result<Vec<String>, std::io::Error> {
    let mut arguments = vec![
        "-fms-extensions".into(),
        "-include".into(),
        std::fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("src\\sal.h"))?
            .display()
            .to_string(),
        "-resource-dir".into(),
        std::fs::canonicalize(resource)?.display().to_string(),
    ];
    for directory in ["ucrt", "um", "shared", "winrt", "cppwinrt"] {
        arguments.extend([
            "-isystem".into(),
            include.join(directory).display().to_string(),
        ]);
    }
    if x86_stdcall {
        arguments.extend(["-Xarch_i686".into(), "-mrtd".into()]);
    }
    Ok(arguments)
}

fn guid_inputs(headers: &[String], arguments: &[String]) -> Vec<Input> {
    let mut inputs = vec![Input {
        name: "win32-2-guid-definitions.h".into(),
        source: format!(
            "#define WIN32_LEAN_AND_MEAN\n#define WIN32_NO_STATUS\n#define INITGUID\n{SUPPORT_PRELUDE}#include <windows.h>\n"
        ),
        arguments: arguments.to_vec(),
        ..Default::default()
    }];
    let mut groups: [Vec<&str>; 3] = Default::default();
    for header in headers {
        let (group, header) = match header.as_str() {
            // These storage headers include ntddstor's unguarded GUID block.
            "ntddcdrm.h" | "ntddtape.h" => (2, "ntddstor.h"),
            // Unguarded definitions also reached through other selected headers.
            "devpkey.h" | "ntddstor.h" | "poclass.h" => (2, header.as_str()),
            header if headers::SATELLITE_HEADERS.contains(&header) => (1, header),
            header => (0, header),
        };
        if !groups[group].contains(&header) {
            groups[group].push(header);
        }
        // shlobj_core suppresses this include when INITGUID is defined.
        if matches!(header, "shlobj.h" | "shlobj_core.h") && !groups[0].contains(&"shlguid.h") {
            groups[0].push("shlguid.h");
        }
    }
    let topology = headers.iter().any(|header| header == "devicetopology.h");
    for (name, group) in [
        "win32-2-main-guid-definitions.h",
        "win32-2-satellite-guid-definitions.h",
        "win32-2-device-guid-definitions.h",
    ]
    .into_iter()
    .zip(groups)
    {
        if group.is_empty() {
            continue;
        }
        // The Windows prelude supplies prerequisites in declaration mode.
        let mut source =
            format!("{SUPPORT_PRELUDE}{PRELUDE}#define INITGUID\n#include <guiddef.h>\n");
        if topology {
            source.push_str("#include <ks.h>\n");
        }
        for header in group {
            if header == "devicetopology.h" {
                source.push_str("#define _KS_\n");
            }
            source.push_str(&format!("#include <{header}>\n"));
        }
        inputs.push(Input {
            name: name.into(),
            source,
            arguments: arguments.to_vec(),
            ..Default::default()
        });
    }
    if headers
        .iter()
        .any(|header| matches!(header.as_str(), "olectl.h" | "dshow.h"))
    {
        // The manifest reaches Olectl through dshow.h.
        // Olectl hides GUID declarations under INITGUID; preload dependencies before changing modes.
        inputs.push(Input {
            name: "win32-2-olectl-guid-definitions.h".into(),
            source: format!(
                "{SUPPORT_PRELUDE}{PRELUDE}#include <ole2.h>\n#include <ocidl.h>\n\
                 #define INITGUID\n#include <guiddef.h>\n#undef INITGUID\n#include <olectl.h>\n"
            ),
            arguments: arguments.to_vec(),
            ..Default::default()
        });
    }
    inputs
}

fn inputs(headers: &[String], arguments: &[String]) -> Vec<Input> {
    let (satellite, main): (Vec<_>, Vec<_>) = headers
        .iter()
        .map(String::as_str)
        .partition(|header| headers::SATELLITE_HEADERS.contains(header));
    let topology = satellite.contains(&"devicetopology.h");
    let mut inputs = Vec::new();
    for (name, group, reset) in [
        ("win32-2-main.h", main, false),
        ("win32-2-satellite.h", satellite, true),
    ] {
        if group.is_empty() {
            continue;
        }
        let mut source = format!("{SUPPORT_PRELUDE}{PRELUDE}");
        if reset {
            source.push_str(GUID_RESET);
        } else if topology {
            // Load KS before any main header can select devicetopology's fallback types.
            source.push_str("#include <ks.h>\n");
        }
        for header in group {
            if reset && header == "devicetopology.h" {
                source.push_str("#include <ks.h>\n#define _KS_\n");
            }
            source.push_str(&format!("#include <{header}>\n"));
            if reset {
                source.push_str(GUID_RESET);
            }
        }
        inputs.push(Input {
            name: name.into(),
            source,
            arguments: arguments.to_vec(),
            ..Default::default()
        });
    }
    inputs
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_rdl::implib::{Import, ImportKind, ImportName};

    #[test]
    fn import_names_and_explicit_overrides() {
        let discovered = [
            ("Original", ImportName::Name("Original".into())),
            ("Local", ImportName::Name("Export".into())),
            ("Ordinal", ImportName::Ordinal(123)),
            ("Override", ImportName::Ordinal(456)),
            ("NamedOverride", ImportName::Name("SDK".into())),
        ]
        .into_iter()
        .map(|(symbol, name)| {
            (
                symbol.into(),
                Import {
                    machine: 0x8664,
                    kind: ImportKind::Code,
                    symbol: symbol.into(),
                    dll: "sdk.dll".into(),
                    name,
                },
            )
        })
        .collect();
        let mut imports = ["Override", "NamedOverride"]
            .into_iter()
            .map(|name| {
                (
                    name.into(),
                    projection::Import {
                        library: "explicit.dll".into(),
                        name: None,
                    },
                )
            })
            .collect();
        assert_eq!(add_sdk_imports(&mut imports, Target::X64, discovered), "");
        let symbols = &imports.symbols[&Target::X64];
        assert_eq!(symbols.len(), 5);
        assert_eq!(symbols["Original"].name.as_deref(), Some("Original"));
        assert_eq!(symbols["Local"].name.as_deref(), Some("Export"));
        assert_eq!(symbols["Ordinal"].name.as_deref(), Some("#123"));
        assert_eq!(symbols["Original"].library, "sdk.dll");
        for name in ["Override", "NamedOverride"] {
            assert_eq!(imports.functions[name].library, "explicit.dll");
            assert_eq!(imports.functions[name].name, None);
        }
        let data = Import {
            machine: 0x8664,
            kind: ImportKind::Data,
            symbol: "Data".into(),
            dll: "sdk.dll".into(),
            name: ImportName::Name("Data".into()),
        };
        let omitted = add_sdk_imports(
            &mut imports,
            Target::X64,
            [("Data".into(), data)].into_iter().collect(),
        );
        assert!(omitted.contains("Data import is not a function"));
        assert!(!imports.symbols[&Target::X64].contains_key("Data"));
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK import libraries"]
    fn sdk_library_policy() {
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let (arch, machine) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let mut evidence = String::new();
            let imports = read_sdk_imports(&directory, target, &mut evidence).unwrap();
            assert!(imports.len() > 20_000);
            assert!(imports.values().all(|import| import.machine == machine));
            let named = |name: &str| {
                imports
                    .values()
                    .find(|import| import.name == ImportName::Name(name.into()))
                    .unwrap()
            };
            for (symbol, library) in [
                ("CreateFileW", "kernel32.dll"),
                ("MessageBoxW", "user32.dll"),
                ("DWriteCreateFactory", "dwrite.dll"),
                ("DecodeRemotePointer", "kernelbase.dll"),
                (
                    "OpenTraceFromFile",
                    "api-ms-win-eventing-consumer-l1-1-2.dll",
                ),
            ] {
                assert!(named(symbol).dll.eq_ignore_ascii_case(library), "{symbol}");
            }
            assert!(named("WaitOnAddress").dll.starts_with("api-ms-win-"));
            for (symbol, ordinal) in if target == Target::X86 {
                [
                    ("_ILClone@4", 18),
                    ("_ILFree@4", 155),
                    ("_WSAStartup@8", 115),
                ]
            } else {
                [("ILClone", 18), ("ILFree", 155), ("WSAStartup", 115)]
            } {
                assert_eq!(imports[symbol].name, ImportName::Ordinal(ordinal));
            }
            if target == Target::Arm64 {
                assert!(evidence.contains("unselected machine 0xa641"));
            } else if target == Target::X86 {
                assert!(evidence.contains("SDK does not provide winhvplatform.lib"));
                assert!(evidence.contains("SDK does not provide vertdll.lib"));
            }
        }
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK headers"]
    fn sdk_macro_scope() {
        let include = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
            .join("c")
            .join("Include")
            .join(helpers::marketing_dir(SDK_VERSION));
        let headers = sdk_macro_headers(&include, &[]).unwrap();
        assert!(headers.windows(2).all(|pair| pair[0] < pair[1]));
        for name in [
            "windows.h",
            "winbase.h",
            "nserror.h",
            "bthdef.h",
            "wiadef.h",
            "gl.h",
        ] {
            assert!(headers.iter().any(|header| header == name), "{name}");
        }
        assert!(!headers.iter().any(|header| header == "stdio.h"));
        assert!(
            headers
                .iter()
                .all(|header| Path::new(header).file_name().unwrap() == header.as_str())
        );
        let selected = sdk_macro_headers(
            &include,
            &["roerrorapi.h".into(), "winrt\\RoErrorApi.h".into()],
        )
        .unwrap();
        assert_eq!(
            selected
                .into_iter()
                .filter(|header| headers.binary_search(header).is_err())
                .collect::<Vec<_>>(),
            ["roerrorapi.h"]
        );
        assert!(sdk_macro_headers(&include, &[String::new()]).is_err());
        assert!(sdk_macro_headers(&include.join("missing"), &[]).is_err());
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK and import libraries"]
    fn sdk_stdcall() {
        helpers::ensure_libclang();
        let include = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
            .join("c")
            .join("Include")
            .join(helpers::marketing_dir(SDK_VERSION));
        let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
            || {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..\\..\\..\\target\\tool-clang\\clang-resource")
                    .join(helpers::LIBCLANG_VERSION)
            },
            std::path::PathBuf::from,
        );
        let targets = [Target::X86, Target::X64, Target::Arm64];
        let mut imports = projection::Imports::default();
        for target in targets {
            let (arch, _) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let discovered = read_sdk_imports(&directory, target, &mut String::new()).unwrap();
            add_sdk_imports(&mut imports, target, discovered);
        }
        for stdcall in [false, true] {
            let input = Input {
                name: "sdk_import_gaps.h".into(),
                source: std::fs::read_to_string(
                    Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("..\\..\\tests\\libs\\clang2\\input\\sdk_import_gaps.h"),
                )
                .unwrap(),
                arguments: sdk_arguments(&include, &resource, stdcall).unwrap(),
                roots: ["DhcpAddServer", "DnsValidateName_W"]
                    .map(String::from)
                    .into(),
                ..Default::default()
            };
            let snapshots = windows_clang2::capture(&[input], &targets).unwrap();
            let output = projection::project_with_imports(
                &windows_clang2::merge(&snapshots).unwrap(),
                "Fixture",
                &imports,
            );
            if stdcall {
                assert_eq!(output.unwrap().functions, 2);
            } else {
                assert!(output.is_err());
            }
        }
    }

    #[test]
    fn guid_definition_scope() {
        let headers = ["ntddtape.h", "ntddcdrm.h", "ntddstor.h", "devpkey.h"].map(String::from);
        let inputs = guid_inputs(&headers, &[]);
        assert_eq!(inputs.len(), 2);
        let source = &inputs[1].source;
        assert_eq!(source.matches("#include <ntddstor.h>").count(), 1);
        assert_eq!(source.matches("#include <devpkey.h>").count(), 1);
        assert!(!source.contains("#include <ntddtape.h>"));
        assert!(!source.contains("#include <ntddcdrm.h>"));
        assert!(!source.contains("#include <propkey.h>"));
        assert!(
            inputs
                .iter()
                .all(|input| input.macros.is_empty() && input.macro_headers.is_empty())
        );
        let inputs = guid_inputs(&["shlobj.h".into(), "shlguid.h".into()], &[]);
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[1].source.matches("#include <shlguid.h>").count(), 1);
        assert!(inputs.iter().all(|input| !input.name.contains("olectl")));
        for header in ["olectl.h", "dshow.h"] {
            let inputs = guid_inputs(&[header.into()], &["-fms-extensions".into()]);
            assert_eq!(inputs.len(), 3);
            let input = inputs.last().unwrap();
            assert_eq!(input.name, "win32-2-olectl-guid-definitions.h");
            assert_eq!(input.arguments, ["-fms-extensions"]);
            assert!(input.source.ends_with(
                "#include <ole2.h>\n#include <ocidl.h>\n#define INITGUID\n\
                 #include <guiddef.h>\n#undef INITGUID\n#include <olectl.h>\n"
            ));
            assert!(input.macros.is_empty() && input.macro_headers.is_empty());
        }
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK headers"]
    fn sdk_guid_definitions() {
        helpers::ensure_libclang();
        let include = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
            .join("c")
            .join("Include")
            .join(helpers::marketing_dir(SDK_VERSION));
        let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
            || {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..\\..\\..\\target\\tool-clang\\clang-resource")
                    .join(helpers::LIBCLANG_VERSION)
            },
            std::path::PathBuf::from,
        );
        let arguments = sdk_arguments(&include, &resource, false).unwrap();
        let headers: Vec<_> = headers::HEADERS
            .iter()
            .chain(headers::SATELLITE_HEADERS)
            .map(|header| (*header).to_string())
            .collect();
        let mut inputs = inputs(&headers, &arguments);
        inputs.extend(guid_inputs(&headers, &arguments));
        let targets = [Target::X86, Target::X64, Target::Arm64];
        let captured = windows_clang2::capture_partial(&inputs, &targets).unwrap();
        for target in targets {
            for name in [
                "PKEY_Title",
                "DEVPKEY_Device_DeviceDesc",
                "MF_MT_MAJOR_TYPE",
                "FOLDERID_Desktop",
                "GUID_DEVINTERFACE_DISK",
                "GUID_DEVINTERFACE_HID",
                "CLSID_MyComputer",
                "CLSID_CFontPropPage",
                "GUID_HIMETRIC",
                "GUID_HANDLE",
            ] {
                assert!(
                    captured
                        .snapshots
                        .iter()
                        .filter(|snapshot| snapshot.target == target)
                        .flat_map(|snapshot| &snapshot.values)
                        .filter(|value| value.name == name)
                        .flat_map(|value| &value.observations)
                        .any(|observation| matches!(
                            observation.kind,
                            windows_clang2::ValueKind::Constant { value: Some(_), .. }
                        )),
                    "{target:?}: {name}"
                );
            }
        }
    }
}
