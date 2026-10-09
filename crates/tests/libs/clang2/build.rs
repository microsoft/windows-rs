#[allow(dead_code)]
#[path = "../../../tools/bindings/src/animation.rs"]
mod animation;
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
        build_sdk_data();
        build_guid_expressions();
        build_ordinal();
        build_animation();
        build_string_termination();
        build_bitfields();
        build_optional_counts();
    }
}

fn build_optional_counts() {
    for file in [
        "input/optional_output_counts.h",
        "input/optional_output_counts.cpp",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "optional.hpp",
            include_str!("input/optional_output_counts.h"),
        )],
        &["-x", "c++", &target],
        &[
            "FillRequired",
            "FillOptional",
            "FillInout",
            "InvokeOptional",
        ],
    )
    .unwrap();
    let mut options = windows_clang2::ProjectionOptions::new("Test");
    options.library = Some("clang2_optional_counts.dll".into());
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let winmd = out.join("optional_counts.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .input(sdk::projection_metadata())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        out.join("optional_counts.rs").to_str().unwrap(),
        "--flat",
        "--sys",
        "--extern",
        "--filter",
        "Test",
    ]);
    cc::Build::new()
        .cpp(true)
        .warnings_into_errors(true)
        .file("input/optional_output_counts.cpp")
        .compile("clang2_optional_counts");
}

fn build_bitfields() {
    for file in ["input/bitfields.h", "input/bitfields.cpp"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "bits.hpp",
            include_str!("input/bitfields.h"),
        )],
        &["-x", "c++", &target],
        &[
            "BitUnits",
            "BitPacked",
            "BitNested",
            "BitFull",
            "BitLayoutEvidence",
            "BitRead",
            "BitWrite",
            "BitPackedRead",
            "BitPackedWrite",
            "BitInvoke",
        ],
    )
    .unwrap();
    let mut options = windows_clang2::ProjectionOptions::new("Bits");
    options.library = Some("clang2_bits.dll".into());
    let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let winmd = out.join("bitfields.winmd");
    windows_rdl::reader()
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        winmd.to_str().unwrap(),
        "--out",
        out.join("bitfields_types.rs").to_str().unwrap(),
        "--flat",
        "--filter",
        "Bits.BitUnits",
        "Bits.BitPacked",
        "Bits.BitNested",
        "Bits.BitFull",
    ]);
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .warnings_into_errors(true)
        .file("input/bitfields.cpp")
        .compile("clang2_bits");
}

fn build_string_termination() {
    for file in ["input/double_null_native.h", "input/double_null_native.cpp"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = windows_clang2::capture(
        [windows_clang2::Input::new(
            "strings.hpp",
            include_str!("input/double_null_native.h"),
        )],
        &["-x", "c++", &target],
        &["MultiLength", "MakeMulti"],
    )
    .unwrap();
    let mut options = windows_clang2::ProjectionOptions::new("Test");
    options.library = Some("clang2_strings.dll".into());
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let winmd = out.join("string_termination_native.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .input(sdk::projection_metadata())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        out.join("string_termination_native.rs").to_str().unwrap(),
        "--flat",
        "--sys",
        "--extern",
        "--filter",
        "Test",
    ]);
    cc::Build::new()
        .cpp(true)
        .warnings_into_errors(true)
        .file("input/double_null_native.cpp")
        .compile("clang2_strings");
}

fn build_animation() {
    let include = sdk::include();
    let sal = sdk::tools()
        .join("..")
        .join("libs")
        .join("clang2")
        .join("src")
        .join("sal.h");
    for file in [
        sdk::tools().join("bindings/src/animation.rs"),
        sdk::tools().join("bindings/src/animation.txt"),
        sal.clone(),
        include.join("um/UIAnimation.h"),
        include.join("um/dcomp.h"),
        sdk::projection_metadata(),
    ] {
        println!("cargo:rerun-if-changed={}", file.display());
    }
    for file in ["input/animation.cpp", "input/animation_slots.cpp"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let plan = animation::project(
        &std::env::var("TARGET").unwrap(),
        &include,
        &sal,
        animation::inputs(),
    )
    .unwrap();
    assert!(plan.omitted().is_empty(), "{:?}", plan.omitted());
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let winmd = output.join("animation.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .input(sdk::projection_metadata())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    sdk::animation_bindings(&winmd, &output.join("animation.rs"));
    let mut build = cc::Build::new();
    build.cpp(true);
    for directory in ["shared", "um", "ucrt"] {
        build.include(include.join(directory));
    }
    build
        .file("input/animation.cpp")
        .file("input/animation_slots.cpp")
        .compile("clang2_animation");
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

fn build_sdk_data() {
    for file in ["input/sdk_data.h", "input/sdk_data.cpp"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let snapshot = sdk::capture_sdk(&target, sdk::DATA_DEFINITIONS, sdk::DATA_ROOTS);
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&windows_clang2::ProjectionOptions::new("Test"))
        .unwrap();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let winmd = out.join("sdk_data.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    for sys in [false, true] {
        let output = out.join(if sys {
            "sdk_data_sys.rs"
        } else {
            "sdk_data.rs"
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
    let include = sdk::include();
    cc::Build::new()
        .cpp(true)
        .warnings_into_errors(true)
        .include(include.join("shared"))
        .include(include.join("um"))
        .include(include.join("ucrt"))
        .file("input/sdk_data.cpp")
        .compile("clang2_sdk_data");
}

fn build_guid_expressions() {
    for file in ["input/guid_expressions.h", "input/guid_expressions.cpp"] {
        println!("cargo:rerun-if-changed={file}");
    }
    let target = format!("--target={}", std::env::var("TARGET").unwrap());
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    for (name, source, roots) in [
        (
            "guid_expressions",
            include_str!("input/guid_expressions.h"),
            sdk::EXPRESSION_ROOTS,
        ),
        (
            "sdk_expressions",
            sdk::SDK_EXPRESSION_SOURCE,
            sdk::SDK_EXPRESSION_ROOTS,
        ),
    ] {
        let snapshot = sdk::capture_sdk(&target, source, roots);
        let plan = snapshot
            .resolve()
            .unwrap()
            .project(&windows_clang2::ProjectionOptions::new("Test"))
            .unwrap();
        let winmd = out.join(format!("{name}.winmd"));
        windows_rdl::reader()
            .input_text(&plan.rdl())
            .reference_default()
            .output(&winmd)
            .write()
            .unwrap();
        for sys in [false, true] {
            let output = out.join(format!("{name}{}.rs", if sys { "_sys" } else { "" }));
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
    }
    let include = sdk::include();
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .warnings_into_errors(true)
        .include(include.join("shared"))
        .include(include.join("um"))
        .include(include.join("ucrt"))
        .file("input/guid_expressions.cpp")
        .compile("clang2_guid_expressions");
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
        &[
            "ComFactory",
            "ComProperties",
            "FirstFactory",
            "SecondFactory",
        ],
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
        "Windows.Win32.System.Com.FirstFactory",
        "Windows.Win32.System.Com.SecondFactory",
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
    println!("cargo:rerun-if-changed=input/packed.h");
    println!("cargo:rerun-if-changed=input/union_calls.h");
    println!("cargo:rerun-if-changed=input/union_calls.cpp");
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
            "UnionMeasure",
            "UnionInvoke",
            "UnionObject",
            "UnionVirtualInvoke",
            "AbiGet",
            "AbiCall",
            "ConvertEnum",
            "AbiEnums",
            "AbiEnumCall",
            "LayoutEvidence",
            "LayoutMutate",
            "LayoutInvoke",
            "PackedLayoutEvidence",
            "PackedMutate",
            "PackedInvoke",
            "PackedChoice",
            "PackedAnonymous",
            "Packed2",
            "Packed4",
            "PackedContainer",
            "PackedField",
            "PackedGap",
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
        .file("input/union_calls.cpp")
        .compile("clang2_abi");
}
