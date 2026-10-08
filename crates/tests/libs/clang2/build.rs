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
        build_webview();
        build_wdk();
        build_crypto();
        build_constants();
        build_ordinal();
    }
}

fn build_ordinal() {
    let source = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    for file in ["input/ordinal.h", "input/ordinal.cpp", "input/ordinal.def"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let output = cc::Build::new()
        .cpp(true)
        .get_compiler()
        .to_command()
        .current_dir(&out)
        .arg("/LD")
        .arg(source.join("input/ordinal.cpp"))
        .arg(format!("/Fo{}", out.join("ordinal.obj").display()))
        .arg(format!("/Fe{}", out.join("clang2_ordinal.dll").display()))
        .arg("/link")
        .arg(format!(
            "/DEF:{}",
            source.join("input/ordinal.def").display()
        ))
        .arg(format!(
            "/IMPLIB:{}",
            out.join("clang2_ordinal.lib").display()
        ))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "ordinal DLL build failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    println!("cargo:rustc-link-search=native={}", out.display());

    let imports =
        windows_rdl::implib::read(&std::fs::read(out.join("clang2_ordinal.lib")).unwrap()).unwrap();
    assert_eq!(imports.len(), 1);
    let mut options = windows_clang2::ProjectionOptions::new("Test");
    for import in imports {
        assert_eq!(import.kind, windows_rdl::implib::ImportKind::Code);
        let windows_rdl::implib::ImportTarget::Ordinal(ordinal) = import.target else {
            panic!("NONAME export must produce an ordinal import");
        };
        assert_eq!(ordinal, 17);
        options.imports.insert(
            import.symbol,
            windows_clang2::FunctionImport {
                library: import.dll,
                target: windows_clang2::ImportTarget::Ordinal(ordinal),
            },
        );
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let plan = windows_clang2::capture(
        [windows_clang2::Input::new(
            "ordinal.h",
            include_str!("input/ordinal.h"),
        )],
        &["-x", "c++", &target],
        &["OrdinalOnly"],
    )
    .unwrap()
    .resolve()
    .unwrap()
    .project(&options)
    .unwrap();
    std::fs::write(out.join("ordinal.rdl"), plan.rdl()).unwrap();
    let winmd = out.join("ordinal.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .output(&winmd)
        .write()
        .unwrap();
    for (name, style) in [
        ("ordinal", None),
        ("ordinal_minimal", Some("--minimal")),
        ("ordinal_sys", Some("--sys")),
    ] {
        let output = out.join(format!("{name}.rs"));
        let mut args = vec![
            "--in",
            winmd.to_str().unwrap(),
            "--out",
            output.to_str().unwrap(),
            "--flat",
            "--filter",
            "Test",
        ];
        args.extend(style);
        windows_bindgen::bindgen(args);
    }
}

fn build_constants() {
    for file in ["input/guid_constants.h", "input/guid_constants.cpp"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "constants.hpp",
            concat!(
                "#define DEFINE_VALUES\n",
                include_str!("input/guid_constants.h")
            ),
        )],
        &["-x", "c++", &target],
        &["ID", "KEY"],
    )
    .unwrap();
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&windows_clang2::ProjectionOptions::new("Test"))
        .unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out.join("guid_constants.rdl"), plan.rdl()).unwrap();
    let winmd = out.join("guid_constants.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    for sys in [false, true] {
        let output = out.join(if sys {
            "guid_constants_sys.rs"
        } else {
            "guid_constants.rs"
        });
        let mut args = vec![
            "--in",
            "default",
            winmd.to_str().unwrap(),
            "--out",
            output.to_str().unwrap(),
            "--flat",
            "--filter",
            "Test",
        ];
        if sys {
            args.push("--sys");
        }
        windows_bindgen::bindgen(args);
    }
    cc::Build::new()
        .cpp(true)
        .file("input/guid_constants.cpp")
        .compile("clang2_constants");
}

fn build_crypto() {
    println!("cargo:rerun-if-changed=input/sdk_buffers.h");
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    println!(
        "cargo:rerun-if-changed={}",
        sdk::projection_metadata().display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        sdk::crypto_library(&target).display()
    );
    let snapshot = sdk::capture_sdk(
        &target,
        include_str!("input/sdk_buffers.h"),
        sdk::CRYPTO_ROOTS,
    );
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&sdk::crypto_options(&target))
        .unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out.join("crypto.rdl"), plan.rdl()).unwrap();
    let winmd = out.join("crypto.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .input(sdk::projection_metadata())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    let mut args: Vec<String> = [
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        out.join("crypto.rs").to_str().unwrap(),
        "--flat",
        "--filter",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    args.extend(
        sdk::CRYPTO_ROOTS
            .iter()
            .map(|root| format!("Crypto.{root}")),
    );
    windows_bindgen::bindgen(args);
}

fn build_wdk() {
    for file in ["input/wdk_layout.h", "input/wdk_layout.cpp", "sdk.rs"] {
        println!("cargo:rerun-if-changed={file}");
    }
    println!(
        "cargo:rerun-if-changed={}",
        sdk::tools()
            .join("win32")
            .join("src")
            .join("km.rs")
            .display()
    );
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let args = sdk::wdk_arguments(&target);
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "wdk.hpp",
            include_str!("input/wdk_layout.h"),
        )],
        &args.iter().map(String::as_str).collect::<Vec<_>>(),
        &["DeviceIoControl", "QuerySecurity", "WdkLayout", "WdkMutate"],
    )
    .unwrap();
    let mut options = windows_clang2::ProjectionOptions::new("Wdk");
    options.library = Some("clang2_wdk.dll".into());
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out.join("wdk.rdl"), plan.rdl()).unwrap();
    let winmd = out.join("wdk.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        out.join("wdk.rs").to_str().unwrap(),
        "--filter",
        "Wdk",
        "--flat",
        "--sys",
        "--extern",
    ]);
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .warnings_into_errors(true)
        .define("NTDDI_VERSION", "0x0A000010");
    for arg in &args {
        if let Some(define) = arg.strip_prefix("-D") {
            if let Some((name, value)) = define.split_once('=') {
                build.define(name, value);
            } else if define != "_WIN64" {
                build.define(define, None);
            }
        }
    }
    for pair in args.windows(2) {
        if pair[0] == "-isystem" {
            build.include(&pair[1]);
        }
    }
    build.file("input/wdk_layout.cpp").compile("clang2_wdk");
}

fn build_webview() {
    for file in ["main.rs", "webview.txt"] {
        println!(
            "cargo:rerun-if-changed={}",
            sdk::tools()
                .join("webview")
                .join("src")
                .join(file)
                .display()
        );
    }
    println!("cargo:rerun-if-changed=input/webview_reference.rdl");
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    println!(
        "cargo:rerun-if-changed={}",
        sdk::webview_library(&target).display()
    );
    let snapshot = sdk::capture_webview(&target, &sdk::webview_roots(), false);
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&sdk::webview_options(&target))
        .unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let reference = out.join("webview_reference.winmd");
    windows_rdl::reader()
        .input_text(include_str!("input/webview_reference.rdl"))
        .reference_default()
        .output(&reference)
        .write()
        .unwrap();
    let winmd = out.join("webview.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .reference_default()
        .reference(&reference)
        .output(&winmd)
        .write()
        .unwrap();
    sdk::webview_bindings(
        &winmd,
        &reference,
        &out.join("webview.rs"),
        &sdk::webview_roots(),
    );
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
    println!(
        "cargo:rerun-if-changed={}",
        sdk::tools()
            .join("win32")
            .join("src")
            .join("main.rs")
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        sdk::tools()
            .join("..")
            .join("libs")
            .join("clang2")
            .join("src")
            .join("sal.h")
            .display()
    );
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = sdk::capture_sdk(
        &target,
        include_str!("input/com.h"),
        &["ComFactory", "ComProperties"],
    );
    let mut options = ProjectionOptions::new("Windows.Win32.System.Com");
    options.library = Some("clang2_com.dll".into());
    for (native, namespace, name) in [
        ("_GUID", "Windows.Win32.Foundation", "GUID"),
        ("HRESULT", "Windows.Win32.Foundation", "HRESULT"),
        ("HWND", "Windows.Win32", "HWND"),
        ("LPCWSTR", "Windows.Win32.Foundation", "PCWSTR"),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: namespace.into(),
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
        "default",
        winmd.to_str().unwrap(),
        reference.to_str().unwrap(),
        "--out",
        out.join("com.rs").to_str().unwrap(),
        "--filter",
        "Windows.Win32.System.Com.IClassFactory",
        "Windows.Win32.System.Com.IProperties",
        "--flat",
    ]);
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        reference.to_str().unwrap(),
        "--out",
        out.join("com_sys.rs").to_str().unwrap(),
        "--filter",
        "Windows.Win32.System.Com.ComFactory",
        "Windows.Win32.System.Com.ComProperties",
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
    println!("cargo:rerun-if-changed=input/enums.h");
    println!("cargo:rerun-if-changed=input/layouts.h");
    println!("cargo:rerun-if-changed=input/layouts.cpp");
    helpers::ensure_libclang();
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "abi.hpp",
            include_str!("input/abi.h"),
        )],
        &["-x", "c++", &target, "-I", "input"],
        &[
            "AbiLayout",
            "AbiRoundtrip",
            "AbiGet",
            "AbiCall",
            "ConvertEnum",
            "AbiEnums",
            "AbiEnumCall",
            "LayoutEvidence",
            "LayoutMutate",
            "LayoutInvoke",
        ],
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
        .file("input/layouts.cpp")
        .compile("clang2_abi");
}
