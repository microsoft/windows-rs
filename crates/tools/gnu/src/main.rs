use std::collections::{BTreeMap, HashSet};
use std::io::prelude::*;
use std::process::{Command, Stdio};
use windows_bindgen::*;

fn main() {
    const ALL_PLATFORMS: [&str; 5] = [
        "x86_64_gnu",
        "i686_gnu",
        "x86_64_gnullvm",
        "aarch64_gnullvm",
        "i686_gnullvm",
    ];
    let mut platforms = HashSet::new();
    for platform in std::env::args().skip(1) {
        if ALL_PLATFORMS.contains(&&*platform) {
            platforms.insert(platform);
        } else if platform == "all" {
            platforms.extend(ALL_PLATFORMS.iter().map(|s| s.to_string()));
        } else {
            eprintln!("Unknown platform: {platform}");
            return;
        }
    }
    if platforms.is_empty() {
        eprintln!("Please specify at least one platform or use 'all' argument");
        return;
    };

    for platform in platforms {
        let tools = if platform.ends_with("_gnu") {
            &["dlltool", "ar", "objcopy"][..]
        } else {
            &["llvm-dlltool", "llvm-ar"][..]
        };
        for tool in tools {
            if Command::new(tool)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .is_err()
            {
                eprintln!("Could not find {tool}. Is it in your $PATH?");
                std::process::exit(1);
            }
        }

        build_platform(&platform, tools[0], tools[1]);
    }
}

fn build_platform(platform: &str, dlltool: &str, ar: &str) {
    println!("Platform: {platform}");

    let libraries = libraries();
    let output = std::path::PathBuf::from(format!("crates/targets/{platform}/lib"));
    std::fs::create_dir_all(&output).unwrap();
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, dlltool, library, functions, platform);
    }

    build_mri(&output, ar, &libraries);

    for library in libraries.keys() {
        std::fs::remove_file(output.join(format!("lib{library}.a"))).unwrap();
    }
}

fn build_library(
    output: &std::path::Path,
    dlltool: &str,
    library: &str,
    functions: &BTreeMap<String, CallingConvention>,
    platform: &str,
) {
    println!("{library}");

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let def_path = output.join(format!("{library}.def"));
    let mut def = std::fs::File::create(def_path).unwrap();

    def.write_all(
        format!(
            r#"
LIBRARY {library}
EXPORTS
"#
        )
        .as_bytes(),
    )
    .unwrap();

    for (function, calling_convention) in functions {
        match calling_convention {
            CallingConvention::Stdcall(params) if platform.starts_with("i686_gnu") => def
                .write_all(format!("{function}@{params} @ 0\n").as_bytes())
                .unwrap(),
            _ => def
                .write_all(format!("{function} @ 0\n").as_bytes())
                .unwrap(),
        }
    }

    drop(def);

    let mut cmd = Command::new(dlltool);
    cmd.current_dir(output);

    if platform.starts_with("i686_gnu") {
        cmd.arg("-k");
    }

    cmd.arg("-d");
    cmd.arg(format!("{library}.def"));
    cmd.arg("-l");
    cmd.arg(format!("lib{library}.a"));
    cmd.arg("-m");
    cmd.arg(match platform {
        "i686_gnu" | "i686_gnullvm" => "i386",
        "x86_64_gnu" | "x86_64_gnullvm" => "i386:x86-64",
        "aarch64_gnullvm" => "arm64",
        _ => unreachable!(),
    });
    if dlltool == "dlltool" {
        // Work around https://sourceware.org/bugzilla/show_bug.cgi?id=29497
        cmd.arg("-f");
        cmd.arg(match platform {
            "i686_gnu" => "--32",
            "x86_64_gnu" => "--64",
            _ => unreachable!(),
        });
        // Ensure consistency in the prefixes used by dlltool.
        cmd.arg("-t");
        cmd.arg(format!("{library}_").replace('.', "_").replace('-', "_"));
        // Ensure deterministic output. (dlltool might be built with DEFAULT_AR_DETERMINISTIC=0)
        cmd.arg("--deterministic-libraries");
    }
    cmd.output().unwrap();

    if dlltool == "dlltool" {
        // Work around lack of determinism in dlltool output, and at the same time remove
        // unnecessary sections and symbols.
        std::fs::rename(output.join(format!("lib{library}.a")), output.join("tmp.a")).unwrap();
        let mut cmd = Command::new("objcopy");
        cmd.current_dir(output);
        cmd.arg("--remove-section=.bss");
        cmd.arg("--remove-section=.data");
        cmd.arg("--strip-unneeded-symbol=fthunk");
        cmd.arg("--strip-unneeded-symbol=hname");
        cmd.arg("--strip-unneeded-symbol=.file");
        cmd.arg("--strip-unneeded-symbol=.text");
        cmd.arg("--strip-unneeded-symbol=.data");
        cmd.arg("--strip-unneeded-symbol=.bss");
        cmd.arg("--strip-unneeded-symbol=.idata$7");
        cmd.arg("--strip-unneeded-symbol=.idata$5");
        cmd.arg("--strip-unneeded-symbol=.idata$4");
        cmd.arg("tmp.a");
        cmd.arg(format!("lib{library}.a"));
        cmd.output().unwrap();

        std::fs::remove_file(output.join("tmp.a")).unwrap();
    }
    std::fs::remove_file(output.join(format!("{library}.def"))).unwrap();
}

fn build_mri(
    output: &std::path::Path,
    ar: &str,
    libraries: &BTreeMap<String, BTreeMap<String, CallingConvention>>,
) {
    let mri_path = output.join("unified.mri");
    let mut mri = std::fs::File::create(&mri_path).unwrap();
    println!("Generating {}", mri_path.to_string_lossy());

    mri.write_all("CREATE libwindows.0.53.0.a\n".as_bytes())
        .unwrap();

    for library in libraries.keys() {
        mri.write_all(format!("ADDLIB lib{library}.a\n").as_bytes())
            .unwrap();
    }

    mri.write_all(b"SAVE\nEND\n").unwrap();

    let mut cmd = Command::new(ar);
    cmd.current_dir(output);
    cmd.arg("-M");
    cmd.stdin(std::fs::File::open(&mri_path).unwrap());
    cmd.output().unwrap();

    std::fs::remove_file(&mri_path).unwrap();
}
