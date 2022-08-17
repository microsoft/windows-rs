use std::collections::{BTreeMap, BTreeSet};
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    for cmd in ["dlltool", "ar", "objcopy"] {
        if Command::new(cmd).stdout(Stdio::null()).stderr(Stdio::null()).spawn().is_err() {
            eprintln!("Could not find {}. Is it in your $PATH?", cmd);
            return;
        }
     }

    const ALL_PLATFORMS: [&str; 2] = ["x86_64_gnu", "i686_gnu"];
    let mut platforms = BTreeSet::new();
    for platform in std::env::args().skip(1) {
        if ALL_PLATFORMS.contains(&&*platform) {
            platforms.insert(platform);
        } else if platform == "all" {
            platforms.extend(ALL_PLATFORMS.iter().map(|s| s.to_string()));
        } else {
            eprintln!("Unknown platform: {}", platform);
            return;
        }
    }
    if platforms.is_empty() {
        eprintln!("Please specify at least one platform or use 'all' argument");
        return;
    };

    for platform in platforms {
        build_platform(&platform);
    }
}

fn build_platform(platform: &str) {
    println!("Platform: {}", platform);

    let libraries = lib::libraries();
    let output = std::path::PathBuf::from(format!("crates/targets/{}/lib", platform));
    let _ = std::fs::remove_dir_all(&output);
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, library, functions, platform);
    }

    build_mri(&output, &libraries);

    for library in libraries.keys() {
        std::fs::remove_file(output.join(format!("lib{}.a", library))).unwrap();
    }
}

fn build_library(output: &std::path::Path, library: &str, functions: &BTreeMap<String, lib::CallingConvention>, platform: &str) {
    println!("{}", library);

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let def_path = output.join(format!("{}.def", library));
    let mut def = std::fs::File::create(&def_path).unwrap();

    def.write_all(
        format!(
            r#"
LIBRARY {}
EXPORTS
"#,
            library
        )
        .as_bytes(),
    )
    .unwrap();

    for (function, calling_convention) in functions {
        match calling_convention {
            lib::CallingConvention::Stdcall(params) if platform.eq("i686_gnu") => def.write_all(format!("{}@{} @ 0\n", function, params).as_bytes()).unwrap(),
            _ => def.write_all(format!("{} @ 0\n", function).as_bytes()).unwrap(),
        }
    }

    drop(def);

    let mut cmd = Command::new("dlltool");
    cmd.current_dir(output);

    if platform.eq("i686_gnu") {
        cmd.arg("-k");
    }

    cmd.arg("-d");
    cmd.arg(format!("{}.def", library));
    cmd.arg("-l");
    cmd.arg(format!("lib{}.a", library));
    cmd.arg("-m");
    cmd.arg(match platform {
        "i686_gnu" => "i386",
        "x86_64_gnu" => "i386:x86-64",
        _ => unreachable!(),
    });
    // Work around https://sourceware.org/bugzilla/show_bug.cgi?id=29497
    cmd.arg("-f");
    cmd.arg(match platform {
        "i686_gnu" => "--32",
        "x86_64_gnu" => "--64",
        _ => unreachable!(),
    });
    // Ensure consistency in the prefixes used by dlltool.
    cmd.arg("-t");
    if library.contains('.') {
        cmd.arg(format!("{}_", library).replace('.', "_").replace('-', "_"));
    } else {
        cmd.arg(format!("{}_dll_", library).replace('-', "_"));
    }
    cmd.output().unwrap();

    // Work around lack of determinism in dlltool output, and at the same time remove
    // unnecessary sections and symbols.
    std::fs::rename(output.join(format!("lib{}.a", library)), output.join("tmp.a")).unwrap();
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
    cmd.arg(format!("lib{}.a", library));
    cmd.output().unwrap();

    std::fs::remove_file(output.join("tmp.a")).unwrap();
    std::fs::remove_file(output.join(format!("{}.def", library))).unwrap();
}

fn build_mri(output: &std::path::Path, libraries: &BTreeMap<String, BTreeMap<String, lib::CallingConvention>>) {
    let mri_path = output.join("unified.mri");
    let mut mri = std::fs::File::create(&mri_path).unwrap();
    println!("Generating {}", mri_path.to_string_lossy());

    mri.write_all(b"CREATE libwindows.a\n").unwrap();

    for library in libraries.keys() {
        mri.write_all(format!("ADDLIB lib{}.a\n", library).as_bytes()).unwrap();
    }

    mri.write_all(b"SAVE\nEND\n").unwrap();

    let mut cmd = Command::new("ar");
    cmd.current_dir(output);
    cmd.arg("-M");
    cmd.stdin(std::fs::File::open(&mri_path).unwrap());
    cmd.output().unwrap();

    std::fs::remove_file(&mri_path).unwrap();
}
