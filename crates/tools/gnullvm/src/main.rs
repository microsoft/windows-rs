use std::collections::BTreeMap;
use std::io::prelude::*;

fn main() {
    let mut cmd = std::process::Command::new("where");
    cmd.arg("llvm-dlltool.exe");

    let output = cmd.output().unwrap();

    if !output.status.success() {
        println!("llvm-dlltool.exe not found");
        return;
    }

    let target = std::env::args().collect::<Vec<_>>();
    let mut platform_and_target = vec![];
    if target.iter().any(|x| x == "x86_64") || target.iter().any(|x| x == "all") {
        platform_and_target.push(("x86_64_gnullvm", "i386:x86-64"));
    }
    if target.iter().any(|x| x == "aarch64") || target.iter().any(|x| x == "all") {
        platform_and_target.push(("aarch64_gnullvm", "arm64"));
    }
    if platform_and_target.is_empty() {
        println!("Please specify at least one architecture or use 'all' argument");
        return;
    };

    let libraries = lib::libraries();

    for (platform, dlltool_target) in platform_and_target {
        let output = std::path::PathBuf::from(format!("crates/targets/{}/lib", platform));
        let _ = std::fs::remove_dir_all(&output);
        std::fs::create_dir_all(&output).unwrap();

        for (library, functions) in &libraries {
            build_library(&output, library, functions, dlltool_target);
        }

        build_mri(&output, &libraries);

        for library in libraries.keys() {
            std::fs::remove_file(output.join(format!("lib{}.a", library))).unwrap();
        }
    }
}

fn build_library(output: &std::path::Path, library: &str, functions: &BTreeMap<String, lib::CallingConvention>, dlltool_target: &str) {
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

    for function in functions.keys() {
        def.write_all(format!("{}\n", function).as_bytes()).unwrap();
    }

    drop(def);

    let mut cmd = std::process::Command::new("llvm-dlltool");
    cmd.current_dir(&output);

    cmd.arg("-m");
    cmd.arg(dlltool_target);
    cmd.arg("-d");
    cmd.arg(format!("{}.def", library));
    cmd.arg("-l");
    cmd.arg(format!("lib{}.a", library));
    cmd.output().unwrap();

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

    let mut cmd = std::process::Command::new("llvm-ar");
    cmd.current_dir(&output);
    cmd.arg("-M");
    cmd.stdin(std::fs::File::open(&mri_path).unwrap());
    cmd.output().unwrap();

    std::fs::remove_file(&mri_path).unwrap();
}
