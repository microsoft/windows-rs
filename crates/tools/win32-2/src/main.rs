mod coverage;
#[path = "../../win32/src/headers.rs"]
mod headers;
#[cfg(test)]
mod import_availability;
#[path = "../../win32/src/imports.rs"]
mod import_policy;
mod named_exports;

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
    if std::env::args().nth(1).as_deref() == Some("--refresh-named-exports") {
        if std::env::args().len() != 2 {
            return Err(
                "--refresh-named-exports cannot be combined with generation arguments".into(),
            );
        }
        return named_exports::refresh();
    }
    if std::env::args().nth(1).as_deref() == Some("--compare-functions") {
        if std::env::args().len() != 2 {
            return Err("--compare-functions cannot be combined with generation arguments".into());
        }
        return named_exports::compare();
    }
    let mut headers = Vec::new();
    let mut partial = false;
    let mut sdk_imports = false;
    let mut sdk_macros = false;
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
                "unknown option `{argument}`; use --compare-constants, --compare-functions, or --refresh-named-exports alone; or --partial, --sdk-imports, --sdk-macros, --dry-run, --first=N, --import=Function=library.dll, --macro=NAME, --macro-header=header.h, or header names"
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
    let resource = std::path::PathBuf::from(helpers::clang_resource_dir());
    let arguments = sdk_arguments(&include, &resource)?;
    let mut inputs = inputs(&headers, &arguments);
    for input in &mut inputs {
        input.macros.clone_from(&macros);
        input.macro_headers.clone_from(&macro_headers);
    }
    inputs.extend(guid_inputs(&headers, &arguments));
    let namespace_headers: Vec<_> = headers
        .iter()
        .filter(|header| {
            matches!(
                header.as_str(),
                "memorybuffer.h" | "robuffer.h" | "windows.ui.composition.interop.h"
            )
        })
        .cloned()
        .collect();
    for input in &mut inputs {
        input.namespace_headers = Some(namespace_headers.clone());
        input.root_paths = Some(
            ["um", "shared", "cppwinrt"]
                .iter()
                .map(|directory| include.join(directory))
                .chain(
                    headers
                        .iter()
                        .map(|header| include.join("winrt").join(header))
                        .filter(|path| path.is_file()),
                )
                .collect(),
        );
    }
    let started = Instant::now();
    let scratch = Path::new("target\\win32-2");
    std::fs::create_dir_all(scratch)?;
    let reference_path = "crates\\libs\\default\\Windows.winmd";
    let native_prefixes = ["", "ABI"];
    let delegate_prefix = Some("I");
    let reference_metadata = windows_metadata::reader::Index::read(reference_path)
        .ok_or("failed to read authoritative WinRT metadata")?;
    let references = projection::References {
        metadata: &reference_metadata,
        native_prefixes: &native_prefixes,
        delegate_prefix,
    };
    let external_interfaces = references.native_interfaces()?;
    for input in &mut inputs {
        input.external_interfaces.clone_from(&external_interfaces);
    }
    let mut configuration = format!(
        "SDK: {SDK_VERSION}\nLLVM: {}\nTargets: x86, x64, ARM64\nPartial: {partial}\nCommand arguments: {:?}\nWinRT reference: {reference_path}\nNative namespace wrappers: {native_prefixes:?}\nNative delegate prefix: {delegate_prefix:?}\nExternal COM definitions: authoritative metadata\n",
        helpers::LIBCLANG_VERSION,
        std::env::args().skip(1).collect::<Vec<_>>(),
    );
    for input in &inputs {
        configuration.push_str(&format!(
            "\nInput: {}\nArguments: {:?}\nMacros: {:?}\nMacro headers: {:?}\nNamespace headers: {:?}\nRoot paths: {:?}\nSource:\n{}\n",
            input.name, input.arguments, input.macros, input.macro_headers, input.namespace_headers,
            input.root_paths, input.source,
        ));
    }
    let evidence = sdk_imports
        .then(named_exports::Evidence::load)
        .transpose()?;
    if evidence.is_some() {
        configuration.push_str(&format!(
            "\nNamed-linkage baseline: {}\n",
            named_exports::BASELINE
        ));
    }
    std::fs::write(scratch.join("capture-configuration.txt"), &configuration)?;
    let captured = if dry_run {
        windows_clang2::PartialCapture {
            snapshots: Vec::new(),
            omissions: Vec::new(),
        }
    } else {
        println!(
            "Capturing {} headers in {} translation units for x86, x64, and ARM64",
            headers.len(),
            inputs.len()
        );
        let targets = [Target::X86, Target::X64, Target::Arm64];
        if partial {
            windows_clang2::capture_partial(&inputs, &targets)?
        } else {
            windows_clang2::PartialCapture {
                snapshots: windows_clang2::capture(&inputs, &targets)?,
                omissions: Vec::new(),
            }
        }
    };
    let mut import_evidence = String::new();
    if sdk_imports {
        let evidence = evidence.as_ref().unwrap();
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let (arch, _) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let discovered = read_sdk_imports(&directory, target, &mut import_evidence)?;
            import_evidence.push_str(&add_sdk_imports(
                &mut imports,
                target,
                discovered,
                evidence,
                &captured.snapshots,
            )?);
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
        import_evidence.push_str(&default_stdcall_imports(&captured.snapshots, &mut imports));
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
    let options = projection::Options {
        imports: Some(&imports),
        references: Some(&references),
    };
    let output = if partial {
        let projection_started = Instant::now();
        let result = projection::project_partial_with_options(captured, "Windows.Win32", &options)?;
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
        projection::project_with_options(
            &windows_clang2::merge(&captured.snapshots)?,
            "Windows.Win32",
            &options,
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
        .reference(reference_path)
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

fn default_stdcall_imports(
    snapshots: &[windows_clang2::Snapshot],
    imports: &mut projection::Imports,
) -> String {
    let mut decisions = BTreeSet::new();
    for snapshot in snapshots
        .iter()
        .filter(|snapshot| snapshot.target == Target::X86)
    {
        let Some(symbols) = imports.symbols.get(&snapshot.target) else {
            continue;
        };
        for value in &snapshot.values {
            if imports.functions.contains_key(&value.name) {
                continue;
            }
            for observation in &value.observations {
                let windows_clang2::ValueKind::Function {
                    alias: None,
                    mangling,
                    stdcall_candidate: Some(candidate),
                    ..
                } = &observation.kind
                else {
                    continue;
                };
                if symbols.contains_key(mangling) {
                    continue;
                }
                let Some(import) = symbols.get(&candidate.mangling) else {
                    continue;
                };
                imports
                    .default_stdcall
                    .entry(snapshot.target)
                    .or_default()
                    .insert(candidate.mangling.clone(), import.clone());
                decisions.insert(format!(
                    "X86 {}: implicit cdecl {mangling} -> compiler stdcall {} = {}!{}\n",
                    value.name,
                    candidate.mangling,
                    import.library,
                    import.name.as_deref().unwrap_or(&value.name)
                ));
            }
        }
    }
    decisions.into_iter().collect()
}

fn add_sdk_imports(
    imports: &mut projection::Imports,
    target: Target,
    discovered: BTreeMap<String, windows_rdl::implib::Import>,
    evidence: &named_exports::Evidence,
    snapshots: &[windows_clang2::Snapshot],
) -> Result<String, String> {
    let mut omitted = String::new();
    let native = named_exports::native_symbols(snapshots, target)?;
    let symbols = imports.symbols.entry(target).or_default();
    for (symbol, import) in discovered {
        if import.kind != windows_rdl::implib::ImportKind::Code {
            omitted.push_str(&format!(
                "{target:?} {symbol}: {} {:?} import is not a function\n",
                import.dll, import.kind,
            ));
            continue;
        }
        let name = match &import.name {
            windows_rdl::implib::ImportName::Name(name) => name.clone(),
            windows_rdl::implib::ImportName::Ordinal(ordinal) => {
                if let Some(source) = native.get(&symbol)
                    && imports.functions.contains_key(source)
                {
                    omitted.push_str(&format!(
                        "{target:?} {symbol}: explicit import for `{source}` takes precedence over {}!#{ordinal}\n",
                        import.dll,
                    ));
                    continue;
                }
                let Some(name) = evidence.resolve(target, &import, &native)? else {
                    omitted.push_str(&format!(
                        "{target:?} {symbol}: {}!#{ordinal} has no matching native/baseline named-export evidence\n",
                        import.dll,
                    ));
                    continue;
                };
                omitted.push_str(&format!(
                    "{target:?} {symbol}: {}!#{ordinal} publishes by baseline export name `{name}` ({})\n",
                    import.dll,
                    named_exports::BASELINE,
                ));
                name.into()
            }
        };
        symbols.insert(
            symbol,
            projection::Import {
                library: import.dll,
                name: Some(name),
            },
        );
    }
    Ok(omitted)
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

fn sdk_arguments(include: &Path, resource: &Path) -> Result<Vec<String>, std::io::Error> {
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
            if header == "windows.ui.composition.interop.h" {
                // The interop header only forward-declares its desktop WinRT dependency.
                source.push_str("#include <windows.ui.composition.desktop.h>\n");
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
    fn composition_dependency_scope() {
        let selected = inputs(&["windows.ui.composition.interop.h".into()], &[]);
        assert_eq!(selected.len(), 1);
        assert!(selected[0].source.ends_with(
            "#include <windows.ui.composition.desktop.h>\n\
             #include <windows.ui.composition.interop.h>\n"
        ));
        let unrelated = inputs(&["robuffer.h".into()], &[]);
        assert!(
            unrelated
                .iter()
                .all(|input| !input.source.contains("windows.ui.composition.desktop.h"))
        );
    }

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
        let evidence = named_exports::Evidence::load().unwrap();
        let omitted =
            add_sdk_imports(&mut imports, Target::X64, discovered, &evidence, &[]).unwrap();
        assert!(omitted.contains(
            "Ordinal: sdk.dll!#123 has no matching native/baseline named-export evidence"
        ));
        assert!(omitted.contains(
            "Override: sdk.dll!#456 has no matching native/baseline named-export evidence"
        ));
        let symbols = &imports.symbols[&Target::X64];
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols["Original"].name.as_deref(), Some("Original"));
        assert_eq!(symbols["Local"].name.as_deref(), Some("Export"));
        assert!(!symbols.contains_key("Ordinal"));
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
            &evidence,
            &[],
        )
        .unwrap();
        assert!(omitted.contains("Data import is not a function"));
        assert!(!imports.symbols[&Target::X64].contains_key("Data"));
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK headers, import libraries, and native companion"]
    fn sdk_library_policy() {
        let evidence = named_exports::Evidence::load().unwrap();
        let mut discovered = BTreeMap::new();
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
            discovered.insert(target, imports);
            if target == Target::Arm64 {
                assert!(evidence.contains("unselected machine 0xa641"));
            } else if target == Target::X86 {
                assert!(evidence.contains("SDK does not provide winhvplatform.lib"));
                assert!(evidence.contains("SDK does not provide vertdll.lib"));
            }
        }
        let fixtures =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\clang2\\input");
        let adshlp = evidence.names(Target::X64, "activeds.dll");
        assert_eq!(adshlp.len(), 22);
        let winsock = evidence.names(Target::X64, "ws2_32.dll");
        assert_eq!(winsock.len(), 125);
        for (header, golden, mut roots) in [
            (
                "sdk_import_targets.h",
                "sdk_named_imports.rdl",
                ["ILClone", "ILFree", "WSAStartup"]
                    .map(String::from)
                    .to_vec(),
            ),
            ("sdk_adshlp.h", "sdk_adshlp.rdl", adshlp),
            ("sdk_winsock.h", "sdk_winsock.rdl", winsock),
            (
                "sdk_stillimage.h",
                "sdk_stillimage.rdl",
                vec!["StiCreateInstanceW".into(), "StiCreateInstance".into()],
            ),
            ("sdk_xaudio_virtual.h", "sdk_xaudio_virtual.rdl", vec![]),
            (
                "sdk_file_segment.h",
                "sdk_file_segment_imports.rdl",
                ["ReadFileScatter", "WriteFileGather"]
                    .map(String::from)
                    .to_vec(),
            ),
            (
                "sdk_vss_classes.h",
                "sdk_vss_classes.rdl",
                vec![
                    "CreateVssExpressWriterInternal".into(),
                    "CreateVssExamineWriterMetadataInternal".into(),
                ],
            ),
            (
                "sdk_variadics.h",
                "sdk_variadics.rdl",
                [
                    "wsprintfA",
                    "wsprintfW",
                    "wnsprintfA",
                    "wnsprintfW",
                    "TraceMessage",
                    "NdrClientCall2",
                    "SetupWriteTextLog",
                    "SetupWriteTextLogError",
                    "u_formatMessage",
                ]
                .map(String::from)
                .to_vec(),
            ),
            (
                "sdk_threadpool.h",
                "sdk_threadpool.rdl",
                [
                    "CreateThreadpoolIo",
                    "CreateThreadpoolTimer",
                    "CreateThreadpoolWait",
                    "CreateThreadpoolWork",
                    "TrySubmitThreadpoolCallback",
                ]
                .map(String::from)
                .to_vec(),
            ),
        ] {
            let expected_functions = roots.len();
            if header == "sdk_stillimage.h" {
                roots.extend(["LPSTILLIMAGE", "LPSTILLIMAGEW", "PSTI", "PSTIW"].map(String::from));
            } else if header == "sdk_xaudio_virtual.h" {
                roots.push("IXAudio2Extension".into());
            }
            if header == "sdk_vss_classes.h" {
                roots.extend(
                    [
                        "IVssComponentEx2",
                        "IVssCreateWriterMetadata",
                        "IVssExamineWriterMetadataEx2",
                        "IVssWriterComponents",
                    ]
                    .map(String::from),
                );
            }
            let input = Input {
                name: header.into(),
                source: std::fs::read_to_string(fixtures.join(header)).unwrap(),
                arguments: sdk_test_arguments(),
                roots,
                macros: if header == "sdk_stillimage.h" {
                    vec!["StiCreateInstance".into()]
                } else {
                    vec![]
                },
                ..Default::default()
            };
            let snapshots =
                windows_clang2::capture(&[input], &[Target::X86, Target::X64, Target::Arm64])
                    .unwrap();
            if header == "sdk_stillimage.h" {
                for snapshot in &snapshots {
                    let record = snapshot
                        .records
                        .iter()
                        .find(|record| record.name == "IStillImageW")
                        .unwrap();
                    let definition = record
                        .observations
                        .iter()
                        .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                        .unwrap();
                    let non_pure: Vec<_> = definition
                        .methods
                        .iter()
                        .filter(|method| !method.pure)
                        .map(|method| (method.name.as_str(), method.slot))
                        .collect();
                    assert_eq!(
                        non_pure,
                        [
                            ("GetDeviceValue", 7),
                            ("SetDeviceValue", 8),
                            ("LaunchApplicationForDevice", 15),
                            ("SetupDeviceParameters", 16),
                        ]
                    );
                }
            }
            if header == "sdk_xaudio_virtual.h" {
                for snapshot in &snapshots {
                    let record = snapshot
                        .records
                        .iter()
                        .find(|record| record.name == "IXAudio2Extension")
                        .unwrap();
                    let definition = record
                        .observations
                        .iter()
                        .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                        .unwrap();
                    assert_eq!(
                        definition
                            .methods
                            .iter()
                            .map(|method| {
                                (
                                    method.name.as_str(),
                                    method.slot,
                                    method.overrides,
                                    method.pure,
                                )
                            })
                            .collect::<Vec<_>>(),
                        [
                            ("QueryInterface", 0, true, true),
                            ("AddRef", 1, true, true),
                            ("Release", 2, true, true),
                            ("GetProcessingQuantum", 3, false, false),
                            ("GetProcessor", 4, false, false),
                        ]
                    );
                }
            }
            if header == "sdk_vss_classes.h" {
                for snapshot in &snapshots {
                    for (name, count) in [
                        ("IVssExpressWriter", 4),
                        ("IVssCreateExpressWriterMetadata", 7),
                    ] {
                        let record = snapshot
                            .records
                            .iter()
                            .find(|record| record.name == name)
                            .unwrap();
                        assert_eq!(record.kind, windows_clang2::RecordKind::Class);
                        let definition = record
                            .observations
                            .iter()
                            .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                            .unwrap();
                        assert_eq!(
                            definition
                                .methods
                                .iter()
                                .map(|method| method.slot)
                                .collect::<Vec<_>>(),
                            (3..3 + count).collect::<Vec<_>>()
                        );
                    }
                }
            }
            if header == "sdk_winsock.h" {
                let mut raw = discovered[&Target::X64]["htons"].clone();
                raw.dll = "other.dll".into();
                let mut explicit = projection::Imports::default();
                explicit.functions.insert(
                    "htons".into(),
                    projection::Import {
                        library: "explicit.dll".into(),
                        name: None,
                    },
                );
                let message = add_sdk_imports(
                    &mut explicit,
                    Target::X64,
                    [("htons".into(), raw)].into(),
                    &evidence,
                    &snapshots,
                )
                .unwrap();
                assert!(message.contains("explicit import for `htons` takes precedence"));
                assert!(!explicit.symbols[&Target::X64].contains_key("htons"));
                assert_eq!(explicit.functions["htons"].library, "explicit.dll");
            }
            let mut published = projection::Imports::default();
            for target in [Target::X86, Target::X64, Target::Arm64] {
                let publication = add_sdk_imports(
                    &mut published,
                    target,
                    discovered[&target].clone(),
                    &evidence,
                    &snapshots,
                )
                .unwrap();
                if header == "sdk_winsock.h" {
                    for (name, ordinal, x86) in [
                        ("htons", 9, "_htons@4"),
                        ("select", 18, "_select@20"),
                        ("WSAGetLastError", 111, "_WSAGetLastError@0"),
                    ] {
                        let symbol = if target == Target::X86 { x86 } else { name };
                        assert_eq!(
                            discovered[&target][symbol].name,
                            ImportName::Ordinal(ordinal)
                        );
                        assert_eq!(
                            published.symbols[&target][symbol].name.as_deref(),
                            Some(name)
                        );
                        assert!(publication.contains(&format!("baseline export name `{name}`")));
                    }
                }
            }
            let output = projection::project_with_imports(
                &windows_clang2::merge(&snapshots).unwrap(),
                "Fixture",
                &published,
            )
            .unwrap();
            assert_eq!(output.functions, expected_functions);
            let actual: String = output
                .partitions
                .values()
                .map(|part| part.rdl.as_str())
                .collect();
            let expected_path = fixtures.join(golden);
            if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
                std::fs::write(&expected_path, &actual).unwrap();
            }
            let expected = std::fs::read_to_string(expected_path).unwrap();
            assert_eq!(actual.replace("\r\n", "\n"), expected.replace("\r\n", "\n"));
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

    pub(super) fn sdk_test_arguments() -> Vec<String> {
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
        sdk_arguments(&include, &resource).unwrap()
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK and import libraries"]
    fn sdk_stdcall() {
        let targets = [Target::X86, Target::X64, Target::Arm64];
        let mut imports = projection::Imports::default();
        let evidence = named_exports::Evidence::load().unwrap();
        for target in targets {
            let (arch, _) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let discovered = read_sdk_imports(&directory, target, &mut String::new()).unwrap();
            add_sdk_imports(&mut imports, target, discovered, &evidence, &[]).unwrap();
        }
        let fixtures =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\clang2\\input");
        let input = Input {
            name: "sdk_import_gaps.h".into(),
            source: std::fs::read_to_string(fixtures.join("sdk_import_gaps.h")).unwrap(),
            arguments: sdk_test_arguments(),
            roots: ["DhcpAddServer", "DnsValidateName_W"]
                .map(String::from)
                .into(),
            ..Default::default()
        };
        let snapshots = windows_clang2::capture(&[input], &targets).unwrap();
        let resolution = windows_clang2::merge(&snapshots).unwrap();
        assert!(projection::project_with_imports(&resolution, "Fixture", &imports).is_err());
        let decisions = default_stdcall_imports(&snapshots, &mut imports);
        assert!(
            decisions
                .contains("implicit cdecl _DhcpAddServer -> compiler stdcall _DhcpAddServer@20")
        );
        assert!(decisions.contains(
            "implicit cdecl _DnsValidateName_W -> compiler stdcall _DnsValidateName_W@8"
        ));
        let output = projection::project_with_imports(&resolution, "Fixture", &imports).unwrap();
        assert_eq!(output.functions, 2);
        let actual: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        let golden = fixtures.join("sdk_import_gaps.rdl");
        if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
            std::fs::write(&golden, &actual).unwrap();
        }
        assert_eq!(
            actual,
            std::fs::read_to_string(golden)
                .unwrap()
                .replace("\r\n", "\n")
        );
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK headers and import libraries"]
    fn sdk_function_aliases() {
        helpers::ensure_libclang();
        let fixtures =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\clang2\\input");
        let input = Input {
            name: "sdk_psapi.h".into(),
            source: std::fs::read_to_string(fixtures.join("sdk_psapi.h")).unwrap(),
            arguments: sdk_test_arguments(),
            roots: vec!["K32EnumProcesses".into()],
            macro_headers: vec!["psapi.h".into()],
            ..Default::default()
        };
        let captured =
            windows_clang2::capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64])
                .unwrap();
        assert_eq!(captured.omissions.len(), 2, "{:?}", captured.omissions);
        assert!(
            captured.omissions.iter().all(|message| {
                message.contains("Macro(\"_PSAPI_H_\")")
                    || message.contains("Macro(\"PENUM_PAGE_FILE_CALLBACK\")")
            }),
            "{:?}",
            captured.omissions
        );
        let snapshots = captured.snapshots;
        let mut aliases = BTreeSet::new();
        for snapshot in &snapshots {
            let mut count = 0;
            for value in &snapshot.values {
                for observation in &value.observations {
                    if let windows_clang2::ValueKind::Function {
                        alias: Some(alias), ..
                    } = &observation.kind
                    {
                        let native = snapshot
                            .values
                            .iter()
                            .find(|value| value.name == alias.name)
                            .unwrap();
                        assert!(
                            native
                                .observations
                                .iter()
                                .any(|native| native.canonical == observation.canonical)
                        );
                        aliases.insert((value.name.clone(), alias.name.clone()));
                        count += 1;
                    }
                }
            }
            assert_eq!(count, 34);
        }
        assert_eq!(aliases.len(), 34);
        let evidence = named_exports::Evidence::load().unwrap();
        let mut imports = projection::Imports::default();
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let (arch, _) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let discovered = read_sdk_imports(&directory, target, &mut String::new()).unwrap();
            add_sdk_imports(&mut imports, target, discovered, &evidence, &snapshots).unwrap();
        }
        let output = projection::project_with_imports(
            &windows_clang2::merge(&snapshots).unwrap(),
            "Fixture",
            &imports,
        )
        .unwrap();
        assert_eq!(output.functions, 61);
        let actual: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        let golden = fixtures.join("sdk_psapi.rdl");
        if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
            std::fs::write(&golden, &actual).unwrap();
        }
        assert_eq!(
            actual,
            std::fs::read_to_string(golden)
                .unwrap()
                .replace("\r\n", "\n")
        );
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..");
        let scratch = root.join("target\\test-clang2-sdk-psapi");
        std::fs::create_dir_all(&scratch).unwrap();
        let winmd = scratch.join("input.winmd");
        let rdl = scratch.join("roundtrip.rdl");
        let again = scratch.join("roundtrip.winmd");
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_texts([&actual])
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
                let index = windows_metadata::reader::Index::new_for_architecture(
                    vec![windows_metadata::reader::File::read(path).unwrap()],
                    bit,
                );
                let methods: BTreeMap<_, _> = index
                    .expect("Fixture", "Apis")
                    .methods()
                    .map(|method| (method.name(), method))
                    .collect();
                assert_eq!(methods.len(), 61);
                for (alias, native) in &aliases {
                    let alias = methods[alias.as_str()];
                    let target = methods[native.as_str()];
                    assert_eq!(alias.signature(&[]), target.signature(&[]));
                    for method in [alias, target] {
                        let import = method.impl_map().unwrap();
                        assert_eq!(import.import_name(), native);
                        assert!(
                            import
                                .import_scope()
                                .name()
                                .eq_ignore_ascii_case("kernel32.dll")
                        );
                        assert_eq!(method.calling_convention(), "system");
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "requires the pinned Windows SDK headers, import libraries, and native companion"]
    fn sdk_public_redeclarations() {
        helpers::ensure_libclang();
        let names = [
            "HSTRING_UserFree",
            "HSTRING_UserMarshal",
            "HSTRING_UserSize",
            "HSTRING_UserUnmarshal",
            "PerfStartProvider",
            "VerSetConditionMask",
        ];
        let fixtures =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\clang2\\input");
        let input = Input {
            name: "sdk_public_redeclarations.h".into(),
            source: std::fs::read_to_string(fixtures.join("sdk_public_redeclarations.h")).unwrap(),
            arguments: sdk_test_arguments(),
            roots: names.map(String::from).into(),
            ..Default::default()
        };
        let snapshots =
            windows_clang2::capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
        for snapshot in &snapshots {
            for name in names {
                let value = snapshot
                    .values
                    .iter()
                    .find(|value| value.name == name)
                    .unwrap();
                assert!(value.observations.len() >= 2, "{name}");
                let canonical = value.observations[0].canonical;
                assert!(
                    value
                        .observations
                        .iter()
                        .all(|observation| observation.canonical == canonical),
                    "{name}"
                );
            }
        }
        let evidence = named_exports::Evidence::load().unwrap();
        let mut imports = projection::Imports::default();
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let (arch, _) = import_target(target);
            let directory =
                helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                    .join("c");
            let discovered = read_sdk_imports(&directory, target, &mut String::new()).unwrap();
            add_sdk_imports(&mut imports, target, discovered, &evidence, &snapshots).unwrap();
        }
        let output = projection::project_with_imports(
            &windows_clang2::merge(&snapshots).unwrap(),
            "Fixture",
            &imports,
        )
        .unwrap();
        assert_eq!(output.functions, names.len());
        let actual: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        let golden = fixtures.join("sdk_public_redeclarations.rdl");
        if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
            std::fs::write(&golden, &actual).unwrap();
        }
        assert_eq!(
            actual,
            std::fs::read_to_string(golden)
                .unwrap()
                .replace("\r\n", "\n")
        );
        let partial = projection::project_partial_with_imports(
            windows_clang2::PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
            &imports,
        )
        .unwrap();
        assert_eq!(partial.output, output);
        assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
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
        let arguments = sdk_arguments(&include, &resource).unwrap();
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
