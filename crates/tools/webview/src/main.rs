use helpers::*;
use std::fs;
use windows_rdl::*;

const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
/// One pin for the COM headers, WinRT metadata, and self-contained projection DLL.
const WEBVIEW2_VERSION: &str = "1.0.4078.44";
const CORE_WINMD: &str = "crates/tools/reactor-metadata/winmd/Microsoft.Web.WebView2.Core.winmd";
const SETUP_VERSION: &str = "crates/libs/reactor-setup/assets/webview2-version.txt";

fn main() {
    let time = std::time::Instant::now();

    // Provision and version-check libclang before the first parse.
    ensure_libclang();

    // The pinned NuGet package lays the C/C++ headers out under `build/native`: the core API and
    // options header live in `include/`, while the COM<->WinRT bridge header sits in
    // `include-winrt/`.
    let pkg = nuget_package(WEBVIEW2_PKG, WEBVIEW2_VERSION);
    let include = pkg.join("build").join("native").join("include");
    let include_winrt = pkg.join("build").join("native").join("include-winrt");
    // `WebView2Interop.h` (in `include-winrt/`) `#include`s `"WebView2.h"` from the sibling
    // `include/` dir, so that directory has to be on the header search path.
    let include_arg = format!("-I{}", include.display());
    let sdk_version = read_str_const("crates/tools/win32/src/main.rs", "SDK_VERSION");
    let sdk_include = nuget_package("microsoft.windows.sdk.cpp", &sdk_version)
        .join("c")
        .join("Include")
        .join(marketing_dir(&sdk_version));
    let resource_dir = clang_resource_dir();
    let sdk_args = ["ucrt", "um", "shared", "winrt", "cppwinrt"].map(|dir| {
        let path = sdk_include.join(dir);
        assert!(path.is_dir(), "missing SDK directory `{}`", path.display());
        format!("-isystem{}", path.display())
    });

    // WebView2 ships only a C/C++ header, so the binding pipeline starts there:
    // WebView2*.h -> WebView2.rdl (clang) -> WebView2.winmd (reader) -> bindings.rs (bindgen).
    // Each header is parsed as its own translation unit (only its own declarations are
    // emitted, not its #includes), so both headers are listed: WebView2.h yields the core
    // COM API and WebView2Interop.h yields the ICoreWebView2Interop2::GetComICoreWebView2
    // bridge used to reuse these COM wrappers from the WinUI/WinRT WebView2 XAML control.
    windows_clang::clang()
        .inputs([
            include.join("WebView2.h"),
            include_winrt.join("WebView2Interop.h"),
        ])
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
            "-resource-dir",
            &resource_dir,
            &include_arg,
        ])
        .args(sdk_args)
        .reference_default()
        .symbols([
            "CompareBrowserVersions",
            "CreateCoreWebView2Environment",
            "CreateCoreWebView2EnvironmentWithOptions",
            "GetAvailableCoreWebView2BrowserVersionString",
            "GetAvailableCoreWebView2BrowserVersionStringWithOptions",
        ])
        .namespace("WebView2")
        .library("WebView2Loader.dll")
        .output("target/webview/WebView2.rdl")
        .write()
        .unwrap();

    reader()
        .input("target/webview/WebView2.rdl")
        .reference_default()
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    fs::copy(
        pkg.join("lib/Microsoft.Web.WebView2.Core.winmd"),
        CORE_WINMD,
    )
    .unwrap_or_else(|error| panic!("failed to write `{CORE_WINMD}`: {error}"));
    fs::write(SETUP_VERSION, WEBVIEW2_VERSION)
        .unwrap_or_else(|error| panic!("failed to write `{SETUP_VERSION}`: {error}"));

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
