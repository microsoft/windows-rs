mod coverage;
#[path = "../../win32/src/headers.rs"]
mod headers;
#[path = "../../win32/src/imports.rs"]
mod import_policy;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::time::Instant;
use windows_clang2::{Input, projection};

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

// Named exports covered by sdk_functions whose SDK import records use ordinals.
const NAMED_SDK_IMPORTS: &[(&str, &str, u16)] = &[
    ("WSAStartup", "ws2_32.dll", 115),
    ("WSACleanup", "ws2_32.dll", 116),
    ("socket", "ws2_32.dll", 23),
    ("bind", "ws2_32.dll", 2),
    ("listen", "ws2_32.dll", 13),
    ("accept", "ws2_32.dll", 1),
    ("connect", "ws2_32.dll", 4),
    ("send", "ws2_32.dll", 19),
    ("recv", "ws2_32.dll", 16),
    ("closesocket", "ws2_32.dll", 3),
    ("D3D12CreateDevice", "d3d12.dll", 101),
    ("D3D12GetDebugInterface", "d3d12.dll", 102),
    ("D2D1MakeRotateMatrix", "d2d1.dll", 2),
    ("D2D1MakeSkewMatrix", "d2d1.dll", 3),
    ("D2D1IsMatrixInvertible", "d2d1.dll", 4),
    ("D2D1InvertMatrix", "d2d1.dll", 5),
    ("D2D1CreateDevice", "d2d1.dll", 7),
    ("D2D1CreateDeviceContext", "d2d1.dll", 8),
];

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
    let mut dry_run = false;
    let mut first = None;
    let mut imports = BTreeMap::new();
    let mut macros = Vec::new();
    let mut macro_headers = Vec::new();
    for argument in std::env::args().skip(1) {
        if argument == "--partial" {
            partial = true;
        } else if argument == "--sdk-imports" {
            sdk_imports = true;
        } else if argument == "--sdk-macros" {
            sdk_macros = true;
        } else if argument == "--dry-run" {
            dry_run = true;
        } else if let Some(value) = argument.strip_prefix("--import=") {
            let (name, library) = value
                .split_once('=')
                .ok_or("use --import=Function=library.dll")?;
            if name.is_empty()
                || library.is_empty()
                || imports
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
                "unknown option `{argument}`; use --compare-constants alone, or --partial, --sdk-imports, --sdk-macros, --dry-run, --first=N, --import=Function=library.dll, --macro=NAME, --macro-header=header.h, or header names"
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
        macro_headers.extend(sdk_macro_headers(&include)?);
        macro_headers.sort();
        macro_headers.dedup();
    }
    let mut arguments = vec![
        "-fms-extensions".into(),
        "-include".into(),
        std::fs::canonicalize("crates\\tools\\win32-2\\src\\sal.h")?
            .display()
            .to_string(),
        "-resource-dir".into(),
        std::fs::canonicalize(helpers::clang_resource_dir())?
            .display()
            .to_string(),
    ];
    for directory in ["ucrt", "um", "shared", "winrt", "cppwinrt"] {
        arguments.extend([
            "-isystem".into(),
            include.join(directory).display().to_string(),
        ]);
    }
    let mut inputs = inputs(&headers, &arguments);
    for input in &mut inputs {
        input.macros.clone_from(&macros);
        input.macro_headers.clone_from(&macro_headers);
    }
    inputs.push(Input {
        name: "win32-2-guid-definitions.h".into(),
        source: format!("#define WIN32_LEAN_AND_MEAN\n#define WIN32_NO_STATUS\n#define INITGUID\n{SUPPORT_PRELUDE}#include <windows.h>\n"),
        arguments: arguments.clone(),
        ..Default::default()
    });
    let started = Instant::now();
    let scratch = Path::new("target\\win32-2");
    std::fs::create_dir_all(scratch)?;
    if sdk_imports {
        let directory =
            helpers::nuget_package("microsoft.windows.sdk.cpp.x64", SDK_VERSION).join("c");
        let discovered = read_sdk_imports(&directory)?;
        let omitted = add_sdk_imports(&mut imports, discovered);
        std::fs::write(scratch.join("import-omissions.txt"), &omitted)?;
        println!(
            "SDK ownership: {} named imports; {} ordinal imports excluded (target\\win32-2\\import-omissions.txt)",
            imports.len(),
            omitted.lines().count(),
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
    configuration.push_str("\nImports:\n");
    for (symbol, import) in &imports {
        configuration.push_str(&format!(
            "{symbol}={}!{}\n",
            import.library,
            import.name.as_deref().unwrap_or(symbol),
        ));
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
        let result = projection::project_partial_with_imports(
            windows_clang2::capture_partial(
                &inputs,
                &[
                    windows_clang2::Target::X86,
                    windows_clang2::Target::X64,
                    windows_clang2::Target::Arm64,
                ],
            )?,
            "Windows.Win32",
            &imports,
        )?;
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
        let snapshots = windows_clang2::capture(
            &inputs,
            &[
                windows_clang2::Target::X86,
                windows_clang2::Target::X64,
                windows_clang2::Target::Arm64,
            ],
        )?;
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

fn sdk_macro_headers(include: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut pending = vec![include.join("um"), include.join("shared")];
    let mut headers = BTreeSet::new();
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
    imports: &mut BTreeMap<String, projection::Import>,
    discovered: BTreeMap<String, windows_rdl::implib::Import>,
) -> String {
    let mut omitted = String::new();
    for (symbol, import) in discovered {
        match import.name {
            windows_rdl::implib::ImportName::Name(name) => {
                imports.entry(symbol.clone()).or_insert(projection::Import {
                    library: import.dll,
                    name: (name != symbol).then_some(name),
                });
            }
            windows_rdl::implib::ImportName::Ordinal(ordinal) => {
                if !imports.contains_key(&symbol) {
                    omitted.push_str(&format!(
                        "{symbol}: {} ordinal {ordinal} requires ordinal import metadata\n",
                        import.dll,
                    ));
                }
            }
        }
    }
    omitted
}

fn read_sdk_imports(
    directory: &Path,
) -> Result<BTreeMap<String, windows_rdl::implib::Import>, Box<dyn std::error::Error>> {
    let mut imports = BTreeMap::new();
    for library in import_policy::IMPORT_LIBS {
        let path = ["um", "ucrt"]
            .into_iter()
            .map(|part| directory.join(part).join("x64").join(library))
            .find(|path| path.is_file())
            .ok_or_else(|| {
                format!(
                    "SDK import library `{library}` is missing from {}",
                    directory.display()
                )
            })?;
        let bytes = std::fs::read(&path)?;
        let members = windows_rdl::implib::read(&bytes)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        for import in members {
            imports.entry(import.symbol.clone()).or_insert(import);
        }
    }
    for entry in import_policy::LIBRARY_OVERRIDES {
        if imports.get(entry.symbol).map(|import| import.dll.as_str()) != entry.sdk_library {
            return Err(format!(
                "SDK library mapping for `{}` changed; remove or update the override",
                entry.symbol
            )
            .into());
        }
    }
    for entry in import_policy::LIBRARY_OVERRIDES {
        imports.insert(
            entry.symbol.into(),
            windows_rdl::implib::Import {
                symbol: entry.symbol.into(),
                dll: entry.corrected_library.into(),
                name: windows_rdl::implib::ImportName::Name(entry.symbol.into()),
            },
        );
    }
    for &(symbol, library, ordinal) in NAMED_SDK_IMPORTS {
        let import = imports
            .get_mut(symbol)
            .filter(|import| {
                import.dll.eq_ignore_ascii_case(library)
                    && import.name == windows_rdl::implib::ImportName::Ordinal(ordinal)
            })
            .ok_or_else(|| {
                format!(
                    "SDK ordinal mapping for `{symbol}` changed; update its named-export policy"
                )
            })?;
        import.dll = library.into();
        import.name = windows_rdl::implib::ImportName::Name(symbol.into());
    }
    Ok(imports)
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
    use windows_rdl::implib::{Import, ImportName};

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
        assert_eq!(
            add_sdk_imports(&mut imports, discovered),
            "Ordinal: sdk.dll ordinal 123 requires ordinal import metadata\n"
        );
        assert_eq!(imports.len(), 4);
        assert_eq!(imports["Original"].name, None);
        assert_eq!(imports["Local"].name.as_deref(), Some("Export"));
        assert_eq!(imports["Original"].library, "sdk.dll");
        for name in ["Override", "NamedOverride"] {
            assert_eq!(imports[name].library, "explicit.dll");
            assert_eq!(imports[name].name, None);
        }
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK import libraries"]
    fn sdk_library_policy() {
        let directory =
            helpers::nuget_package("microsoft.windows.sdk.cpp.x64", SDK_VERSION).join("c");
        let imports = read_sdk_imports(&directory).unwrap();
        assert!(imports.len() > 20_000);
        for &(symbol, library, _) in NAMED_SDK_IMPORTS {
            assert_eq!(imports[symbol].dll, library);
            assert_eq!(imports[symbol].name, ImportName::Name(symbol.into()));
        }
        for (symbol, library) in [
            ("CreateFileW", "kernel32.dll"),
            ("MessageBoxW", "user32.dll"),
            ("DWriteCreateFactory", "dwrite.dll"),
            ("DecodeRemotePointer", "kernelbase.dll"),
            (
                "OpenTraceFromFile",
                "api-ms-win-eventing-consumer-l1-1-2.dll",
            ),
            ("InitializeXamlDiagnosticsEx", "windows.ui.xaml.dll"),
        ] {
            assert!(
                imports[symbol].dll.eq_ignore_ascii_case(library),
                "{symbol}"
            );
            assert_eq!(
                imports[symbol].name,
                ImportName::Name(symbol.into()),
                "{symbol}"
            );
        }
        assert!(imports["WaitOnAddress"].dll.starts_with("api-ms-win-"));
        assert!(matches!(
            imports["GetScaleFactorForWindow"].name,
            ImportName::Ordinal(_)
        ));
        assert_eq!(
            imports["GetScaleFactorForWindow"].dll,
            "api-ms-win-shcore-scaling-l1-1-1.dll"
        );
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK headers"]
    fn sdk_macro_scope() {
        let include = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
            .join("c")
            .join("Include")
            .join(helpers::marketing_dir(SDK_VERSION));
        let headers = sdk_macro_headers(&include).unwrap();
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
        assert!(sdk_macro_headers(&include.join("missing")).is_err());
    }
}
