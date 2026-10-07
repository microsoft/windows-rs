#[allow(dead_code)]
mod sdk;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBCLANG_PATH");
    let path = std::env::var_os("LIBCLANG_PATH")
        .map_or_else(helpers::libclang_dir, std::path::PathBuf::from);
    println!("cargo:rustc-env=LIBCLANG_PATH={}", path.display());
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        build_abi();
        build_com();
    }
}

fn build_com() {
    use windows_clang2::{ProjectionOptions, ReferenceKind, TypeReference};
    for file in [
        "input/com.h",
        "input/com.cpp",
        "input/com_reference.rdl",
        "sdk.rs",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }
    for file in ["main.rs", "sal.h"] {
        println!(
            "cargo:rerun-if-changed={}",
            sdk::tools().join("win32").join("src").join(file).display()
        );
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = sdk::capture_sdk(&target, include_str!("input/com.h"), &["ComFactory"]);
    let mut options = ProjectionOptions::new("Windows.Win32.System.Com");
    options.library = Some("clang2_com.dll".into());
    for (native, name) in [("_GUID", "GUID"), ("HRESULT", "HRESULT")] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: "Windows.Win32.Foundation".into(),
                name: name.into(),
                kind: ReferenceKind::Value,
            },
        );
    }
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out.join("com.rdl"), plan.rdl()).unwrap();
    let reference = out.join("com_reference.winmd");
    windows_rdl::reader()
        .input_text(include_str!("input/com_reference.rdl"))
        .output(&reference)
        .write()
        .unwrap();
    let winmd = out.join("com.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .reference_default()
        .reference(&reference)
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        reference.to_str().unwrap(),
        "--out",
        out.join("com.rs").to_str().unwrap(),
        "--filter",
        "Windows.Win32.System.Com.IClassFactory",
        "--flat",
    ]);
    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        reference.to_str().unwrap(),
        "--out",
        out.join("com_sys.rs").to_str().unwrap(),
        "--filter",
        "Windows.Win32.System.Com.ComFactory",
        "--flat",
        "--sys",
        "--extern",
    ]);
    let include = sdk::include();
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .warnings_into_errors(true)
        .include(include.join("shared"))
        .include(include.join("um"))
        .file("input/com.cpp")
        .compile("clang2_com");
}

fn build_abi() {
    println!("cargo:rerun-if-changed=input/abi.h");
    println!("cargo:rerun-if-changed=input/abi.cpp");
    helpers::ensure_libclang();
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "abi.hpp",
            include_str!("input/abi.h"),
        )],
        &["-x", "c++", &target],
        &["AbiLayout", "AbiRoundtrip", "AbiGet", "AbiCall"],
    )
    .unwrap();
    let mut options = windows_clang2::ProjectionOptions::new("Abi");
    options.library = Some("clang2_abi.dll".into());
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out.join("abi.rdl"), plan.rdl()).unwrap();
    let winmd = out.join("abi.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        "--out",
        out.join("abi.rs").to_str().unwrap(),
        "--filter",
        "Abi",
        "--flat",
        "--sys",
        "--extern",
    ]);
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .warnings_into_errors(true)
        .file("input/abi.cpp")
        .compile("clang2_abi");
}
