#[path = "../../win32/src/headers.rs"]
mod headers;

use std::path::Path;
use std::time::Instant;
use windows_clang2::{Input, projection};

const SDK_VERSION: &str = "10.0.28000.2270";
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
    let mut headers = Vec::new();
    let mut partial = false;
    let mut first = None;
    let mut imports = std::collections::BTreeMap::new();
    let mut macros = Vec::new();
    for argument in std::env::args().skip(1) {
        if argument == "--partial" {
            partial = true;
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
        } else if let Some(count) = argument.strip_prefix("--first=") {
            if first.replace(count.parse::<usize>()?).is_some() {
                return Err("--first may only be specified once".into());
            }
        } else if argument.starts_with('-') {
            return Err(format!(
                "unknown option `{argument}`; use --partial, --first=N, --import=Function=library.dll, --macro=NAME, or header names"
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
    let mut arguments = vec![
        "-fms-extensions".into(),
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
    }
    println!(
        "Capturing {} headers in {} translation units for x86, x64, and ARM64",
        headers.len(),
        inputs.len()
    );
    let started = Instant::now();
    let scratch = Path::new("target\\win32-2");
    std::fs::create_dir_all(scratch)?;
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
        return Err("no declarations could be projected; canonical metadata is unchanged".into());
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
    println!("Rebuilt metadata\\win32. Validation WinMD: target\\win32-2\\Windows.Win32.winmd");
    Ok(())
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
        let mut source = PRELUDE.to_string();
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
