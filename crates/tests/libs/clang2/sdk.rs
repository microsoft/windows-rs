use std::path::{Path, PathBuf};
use windows_clang2::{
    Input, ProjectionOptions, ReferenceKind, Snapshot, StringKind, TypeReference, capture,
};

pub fn animation_bindings(winmd: &Path, output: &Path) {
    let mut arguments = vec![
        "--in".to_string(),
        "default".into(),
        winmd.to_str().unwrap().into(),
        "--out".into(),
        output.to_str().unwrap().into(),
        "--flat".into(),
        "--minimal".into(),
        "--dead-code".into(),
        "--filter".into(),
    ];
    arguments.extend(
        include_str!("../../../tools/bindings/src/animation.txt")
            .split_once("--filter")
            .unwrap()
            .1
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string),
    );
    windows_bindgen::bindgen(arguments);
}

pub fn include() -> PathBuf {
    let version = helpers::read_str_const(
        tools().join("win32").join("src").join("main.rs"),
        "SDK_VERSION",
    );
    let (marketing, _) = version.rsplit_once('.').unwrap();
    helpers::nuget_package("microsoft.windows.sdk.cpp", &version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"))
}

pub fn tools() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("tools")
}

pub fn wdk_include() -> PathBuf {
    let version = helpers::read_str_const(
        tools().join("win32").join("src").join("km.rs"),
        "WDK_VERSION",
    );
    let (marketing, _) = version.rsplit_once('.').unwrap();
    helpers::nuget_package("microsoft.windows.wdk.x64", &version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"))
}

pub fn wdk_arguments(target: &str) -> Vec<String> {
    let wdk = wdk_include();
    let sdk = include();
    let mut args: Vec<String> = ["-x", "c++", target, "-DNTDDI_VERSION=0x0A000010"]
        .into_iter()
        .map(str::to_string)
        .collect();
    let defines: &[&str] = if target.contains("x86_64") {
        &["-D_AMD64_", "-DAMD64", "-D_WIN64"]
    } else if target.contains("aarch64") {
        &["-D_ARM64_", "-DARM64", "-D_WIN64"]
    } else {
        assert!(target.contains("i686"), "unsupported WDK target: {target}");
        &["-D_X86_", "-Di386=1"]
    };
    args.extend(defines.iter().map(|value| value.to_string()));
    for dir in [
        wdk.join("km"),
        wdk.join("shared"),
        sdk.join("shared"),
        sdk.join("um"),
        sdk.join("ucrt"),
    ] {
        args.extend(["-isystem".into(), dir.to_str().unwrap().into()]);
    }
    args.extend([
        "-include".into(),
        tools()
            .join("..")
            .join("libs")
            .join("clang2")
            .join("src")
            .join("sal.h")
            .to_str()
            .unwrap()
            .into(),
    ]);
    args
}

pub fn webview_headers() -> [PathBuf; 2] {
    let version = helpers::read_str_const(
        tools().join("webview").join("src").join("main.rs"),
        "WEBVIEW2_VERSION",
    );
    let native = helpers::nuget_package("Microsoft.Web.WebView2", &version)
        .join("build")
        .join("native");
    [
        native.join("include").join("WebView2.h"),
        native.join("include-winrt").join("WebView2Interop.h"),
    ]
}

pub fn webview_roots() -> Vec<&'static str> {
    include_str!("../../../tools/webview/src/webview.txt")
        .lines()
        .filter_map(|line| line.trim().strip_prefix("WebView2."))
        .map(|name| name.split(':').next().unwrap())
        .chain(["POINT", "RECT"])
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn capture_webview(target: &str, roots: &[&str], swapped: bool) -> Snapshot {
    let mut arguments = arguments(target);
    for header in webview_headers() {
        arguments.push(format!("-I{}", header.parent().unwrap().display()));
    }
    let mut inputs = [
        Input::new(
            if swapped { "z.hpp" } else { "a.hpp" },
            "#include <WebView2.h>",
        ),
        Input::new(
            if swapped { "a.hpp" } else { "z.hpp" },
            "#include <WebView2Interop.h>",
        ),
    ];
    if swapped {
        inputs.reverse();
    }
    capture(
        inputs,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        roots,
    )
    .unwrap()
}

pub fn webview_library(target: &str) -> PathBuf {
    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("aarch64") {
        "arm64"
    } else {
        assert!(target.contains("i686"), "unsupported target: {target}");
        "x86"
    };
    webview_headers()[0]
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(arch)
        .join("WebView2Loader.dll.lib")
}

pub const WEBVIEW_EXPORTS: &[&str] = &[
    "CompareBrowserVersions",
    "CreateCoreWebView2Environment",
    "CreateCoreWebView2EnvironmentWithOptions",
    "GetAvailableCoreWebView2BrowserVersionString",
    "GetAvailableCoreWebView2BrowserVersionStringWithOptions",
];

pub fn webview_options(target: &str) -> ProjectionOptions {
    let mut options = ProjectionOptions::new("WebView2");
    options.imports = imports(&webview_library(target), WEBVIEW_EXPORTS);
    for (native, namespace, name, kind) in [
        (
            "_GUID",
            "Windows.Win32.Foundation",
            "GUID",
            ReferenceKind::Value,
        ),
        (
            "HRESULT",
            "Windows.Win32.Foundation",
            "HRESULT",
            ReferenceKind::Value,
        ),
        (
            "BOOL",
            "Windows.Win32.Foundation",
            "BOOL",
            ReferenceKind::Value,
        ),
        (
            "IUnknown",
            "Windows.Win32.System.Com",
            "IUnknown",
            ReferenceKind::Interface,
        ),
        (
            "IStream",
            "Windows.Win32",
            "IStream",
            ReferenceKind::Interface,
        ),
        (
            "tagVARIANT",
            "Windows.Win32",
            "VARIANT",
            ReferenceKind::Value,
        ),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: namespace.into(),
                name: name.into(),
                kind,
            },
        );
    }
    for (native, name) in [
        ("HWND", "HWND"),
        ("HICON", "HICON"),
        ("HCURSOR", "HCURSOR"),
        ("HANDLE", "HANDLE"),
        ("PWSTR", "PWSTR"),
        ("LPWSTR", "PWSTR"),
        ("PCWSTR", "PCWSTR"),
        ("LPCWSTR", "PCWSTR"),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: "Windows.Win32".into(),
                name: name.into(),
                kind: ReferenceKind::Value,
            },
        );
    }
    options
}

pub fn webview_bindings(winmd: &Path, reference: &Path, output: &Path, roots: &[&str]) {
    let mut args: Vec<String> = [
        "--in",
        "default",
        winmd.to_str().unwrap(),
        reference.to_str().unwrap(),
        "--out",
        output.to_str().unwrap(),
        "--flat",
        "--filter",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    args.extend(roots.iter().map(|root| format!("WebView2.{root}")));
    windows_bindgen::bindgen(args);
}

pub fn capture_sdk(target: &str, source: &str, roots: &[&str]) -> Snapshot {
    let arguments = arguments(target);
    capture(
        [Input::new("wrapper.hpp", source)],
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        roots,
    )
    .unwrap()
}

pub const EXPRESSION_ROOTS: &[&str] = &[
    "TYPE_ID",
    "POINTER_ID",
    "PAREN_ID",
    "CHAIN_ID",
    "LiteralId",
    "CopiedId",
    "LITERAL_ALIAS",
];

pub const SDK_EXPRESSION_SOURCE: &str =
    "#include <windows.h>\n#include <ks.h>\n#include <ksmedia.h>\n#include <codecapi.h>";
pub const SDK_EXPRESSION_ROOTS: &[&str] = &[
    "KSPROPSETID_General",
    "KSMEDIUMSETID_MidiBus",
    "CODECAPI_AVEncCommonFormatConstraint",
];

pub const CRYPTO_ROOTS: &[&str] = &[
    "BCryptOpenAlgorithmProvider",
    "BCryptDeriveKeyPBKDF2",
    "BCryptCloseAlgorithmProvider",
    "BCryptGetProperty",
    "BCryptCreateHash",
    "BCryptHashData",
    "BCryptFinishHash",
    "BCryptDestroyHash",
    "BCRYPT_ALG_HANDLE_HMAC_FLAG",
];

pub const DATA_ROOTS: &[&str] = &[
    "PKEY_Address_Country",
    "DEVPKEY_Device_ClassGuid",
    "FOLDERID_Documents",
    "MFVideoFormat_RGB32",
    "IID_IAVIFile",
    "FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE",
    "NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID",
];

pub const DATA_DEFINITIONS: &str = concat!(
    "#include <guiddef.h>\n#include <cguid.h>\n#include <initguid.h>\n",
    include_str!("input/sdk_data.h")
);

pub fn projection_metadata() -> PathBuf {
    tools()
        .join("..")
        .join("libs")
        .join("clang2")
        .join("metadata.rdl")
}

pub fn crypto_library(target: &str) -> PathBuf {
    library(target, "bcrypt.lib")
}

pub fn library(target: &str, name: &str) -> PathBuf {
    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("aarch64") {
        "arm64"
    } else {
        assert!(target.contains("i686"), "unsupported target: {target}");
        "x86"
    };
    let version = helpers::read_str_const(
        tools().join("win32").join("src").join("main.rs"),
        "SDK_VERSION",
    );
    helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), &version)
        .join("c")
        .join("um")
        .join(arch)
        .join(name)
}

pub fn optional_input_options(target: &str) -> ProjectionOptions {
    let mut options = ProjectionOptions::new("SdkOptional");
    options.imports = imports(&library(target, "gdi32.lib"), &["LineDDA"]);
    options
        .pointer_sized
        .insert("LPARAM".into(), windows_clang2::PointerSized::Signed);
    options
}

pub const OPTIONAL_INPUT_ROOTS: &[&str] = &[
    "OptionalInteger",
    "OptionalUnsigned",
    "OptionalSignedWord",
    "OptionalEnum",
    "OptionalFloat",
    "OptionalRecord",
    "OptionalPointer",
    "OptionalInvoke",
];

pub fn optional_value_options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("clang2_optional_inputs.dll".into());
    options.pointer_sized.insert(
        "OptionalWord".into(),
        windows_clang2::PointerSized::Unsigned,
    );
    options.pointer_sized.insert(
        "OptionalSigned".into(),
        windows_clang2::PointerSized::Signed,
    );
    options
}

pub fn crypto_options(target: &str) -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Crypto");
    let exports: Vec<_> = CRYPTO_ROOTS
        .iter()
        .copied()
        .filter(|root| root.starts_with("BCrypt"))
        .collect();
    options.imports = imports(&crypto_library(target), &exports);
    options.string_references.insert(
        StringKind::WideConst,
        TypeReference {
            namespace: "Windows.Win32".into(),
            name: "PCWSTR".into(),
            kind: ReferenceKind::Value,
        },
    );
    for (native, name) in [
        ("BCRYPT_ALG_HANDLE", "BCRYPT_ALG_HANDLE"),
        ("BCRYPT_HASH_HANDLE", "BCRYPT_HASH_HANDLE"),
        ("BCRYPT_HANDLE", "BCRYPT_HANDLE"),
        ("LPCWSTR", "PCWSTR"),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: "Windows.Win32".into(),
                name: name.into(),
                kind: ReferenceKind::Value,
            },
        );
    }
    options
}

fn imports(
    library: &Path,
    exports: &[&str],
) -> std::collections::BTreeMap<String, windows_clang2::FunctionImport> {
    let bytes = std::fs::read(library).unwrap();
    let mut imports = std::collections::BTreeMap::new();
    let mut found = std::collections::BTreeSet::new();
    for import in windows_rdl::implib::read(&bytes).unwrap() {
        let windows_rdl::implib::ImportTarget::Name(name) = import.target else {
            continue;
        };
        if !exports.contains(&name.as_str()) {
            continue;
        }
        assert_eq!(import.kind, windows_rdl::implib::ImportKind::Code);
        found.insert(name.clone());
        let value = windows_clang2::FunctionImport {
            library: import.dll,
            target: windows_clang2::ImportTarget::Name(name),
        };
        if let Some(previous) = imports.insert(import.symbol.clone(), value) {
            assert_eq!(previous, imports[&import.symbol]);
        }
    }
    for root in exports {
        assert!(
            found.contains(*root),
            "missing import-library export: {root}"
        );
    }
    imports
}

pub fn arguments(target: &str) -> Vec<String> {
    let include = include();
    let sal = tools()
        .join("..")
        .join("libs")
        .join("clang2")
        .join("src")
        .join("sal.h");
    [
        "-x",
        "c++",
        target,
        "-include",
        sal.to_str().unwrap(),
        "-isystem",
        include.join("shared").to_str().unwrap(),
        "-isystem",
        include.join("um").to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}
