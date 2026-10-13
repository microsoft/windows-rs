use std::path::Path;
use std::time::Instant;
use windows_clang2::{Input, projection};

const SDK_VERSION: &str = "10.0.28000.2270";

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
    let mut headers: Vec<_> = std::env::args().skip(1).collect();
    if headers.is_empty() {
        headers.push("windows.h".into());
    }
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
        .join("c")
        .join("Include")
        .join(helpers::marketing_dir(SDK_VERSION));
    let mut arguments = vec![
        "-DWIN32_LEAN_AND_MEAN".into(),
        "-resource-dir".into(),
        std::fs::canonicalize(helpers::clang_resource_dir())?
            .display()
            .to_string(),
    ];
    for directory in ["ucrt", "um", "shared"] {
        arguments.extend([
            "-isystem".into(),
            include.join(directory).display().to_string(),
        ]);
    }
    let input = Input {
        name: "win32-2.h".into(),
        source: headers
            .iter()
            .map(|header| format!("#include <{header}>\n"))
            .collect(),
        arguments,
        ..Default::default()
    };
    println!("Capturing {} for x86, x64, and ARM64", headers.join(", "));
    let started = Instant::now();
    let output = projection::generate(&[input], "Windows.Win32")?;
    let generated = started.elapsed();
    let scratch = Path::new("target\\win32-2");
    std::fs::create_dir_all(scratch)?;
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
        .collect::<Result<Vec<_>, _>>()?;
    for path in &previous {
        if !path.is_file() || path.extension().is_none_or(|extension| extension != "rdl") {
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
        "SDK projection: {} records, {} aliases, {} headers; capture/merge/project {:.3}s, compile {:.3}s",
        output.records,
        output.aliases,
        output.partitions.len(),
        generated.as_secs_f64(),
        compiled.as_secs_f64()
    );
    println!("Rebuilt metadata\\win32. Validation WinMD: target\\win32-2\\Windows.Win32.winmd");
    Ok(())
}
