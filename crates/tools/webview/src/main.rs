use helpers::*;
use std::collections::BTreeSet;
use windows_clang2::{FunctionImport, Input, ProjectionOptions, ReferenceKind, TypeReference};
use windows_rdl::*;

// WebView2 owns its SDK pin here: the headers are downloaded from this exact NuGet package
// (via `nuget_package`, like the other header scrapers) instead of being vendored, so a version
// bump is a one-line edit that re-fetches byte-stable headers. `tool-reactor-metadata` reads the
// pin to refresh its committed Core.winmd and rejects drift in `windows-reactor-setup`.
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VERSION: &str = "1.0.4078.44";

const EXPORTS: &[&str] = &[
    "CompareBrowserVersions",
    "CreateCoreWebView2Environment",
    "CreateCoreWebView2EnvironmentWithOptions",
    "GetAvailableCoreWebView2BrowserVersionString",
    "GetAvailableCoreWebView2BrowserVersionStringWithOptions",
];

fn main() {
    let time = std::time::Instant::now();
    ensure_libclang();
    assert_libclang_version();

    let native = nuget_package(WEBVIEW2_PKG, WEBVIEW2_VERSION)
        .join("build")
        .join("native");
    let sdk_version = read_str_const("crates/tools/win32/src/main.rs", "SDK_VERSION");
    let (marketing, _) = sdk_version.rsplit_once('.').unwrap();
    let sdk = nuget_package("microsoft.windows.sdk.cpp", &sdk_version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"));
    let arguments = [
        "-x".to_string(),
        "c++".into(),
        // x86 retains the source stdcall distinction that x64 normalizes to its platform ABI.
        "--target=i686-pc-windows-msvc".into(),
        "-fms-extensions".into(),
        "-include".into(),
        "crates/libs/clang2/src/sal.h".into(),
        "-isystem".into(),
        sdk.join("shared").to_str().unwrap().into(),
        "-isystem".into(),
        sdk.join("um").to_str().unwrap().into(),
        format!("-I{}", native.join("include").display()),
        format!("-I{}", native.join("include-winrt").display()),
    ];
    let roots: Vec<_> = include_str!("webview.txt")
        .lines()
        .filter_map(|line| line.trim().strip_prefix("WebView2."))
        .map(|name| name.split(':').next().unwrap())
        .chain(["POINT", "RECT"])
        .chain(EXPORTS.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let snapshot = windows_clang2::capture(
        [
            Input::new("webview.hpp", "#include <WebView2.h>"),
            Input::new("interop.hpp", "#include <WebView2Interop.h>"),
        ],
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &roots,
    )
    .unwrap();
    let mut options = ProjectionOptions::new("WebView2");
    for (native, namespace, name, kind) in [
        ("_GUID", "System", "Guid", ReferenceKind::Value),
        (
            "HRESULT",
            "Windows.Foundation",
            "HResult",
            ReferenceKind::Value,
        ),
        ("BOOL", "Windows.Win32", "BOOL", ReferenceKind::Value),
        (
            "IUnknown",
            "Windows.Win32",
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
        ("HWND", "Windows.Win32", "HWND", ReferenceKind::Value),
        ("HICON", "Windows.Win32", "HICON", ReferenceKind::Value),
        ("HCURSOR", "Windows.Win32", "HCURSOR", ReferenceKind::Value),
        ("HANDLE", "Windows.Win32", "HANDLE", ReferenceKind::Value),
        ("PWSTR", "Windows.Win32", "PWSTR", ReferenceKind::Value),
        ("LPWSTR", "Windows.Win32", "PWSTR", ReferenceKind::Value),
        ("PCWSTR", "Windows.Win32", "PCWSTR", ReferenceKind::Value),
        ("LPCWSTR", "Windows.Win32", "PCWSTR", ReferenceKind::Value),
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
    let library = std::fs::read(native.join("x86").join("WebView2Loader.dll.lib")).unwrap();
    let mut found = BTreeSet::new();
    for import in implib::read(&library).unwrap() {
        let implib::ImportTarget::Name(name) = import.target else {
            continue;
        };
        if !EXPORTS.contains(&name.as_str()) {
            continue;
        }
        assert_eq!(import.kind, implib::ImportKind::Code);
        found.insert(name.clone());
        let value = FunctionImport {
            library: import.dll,
            target: windows_clang2::ImportTarget::Name(name),
        };
        if let Some(previous) = options.imports.insert(import.symbol.clone(), value) {
            assert_eq!(previous, options.imports[&import.symbol]);
        }
    }
    for export in EXPORTS {
        assert!(
            found.contains(*export),
            "missing import-library export: {export}"
        );
    }
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    assert!(
        plan.omitted().is_empty(),
        "omitted WebView roots: {:?}",
        plan.omitted()
    );
    std::fs::create_dir_all("target/webview").unwrap();
    std::fs::write("target/webview/WebView2.rdl", plan.rdl()).unwrap();

    reader()
        .input("target/webview/WebView2.rdl")
        .reference_default()
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
