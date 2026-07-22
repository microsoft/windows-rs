use windows_clang::*;
use windows_rdl::*;

// WebView2 owns its SDK pin here: the headers are downloaded from this exact NuGet package
// (via `nuget_package`, like the other header scrapers) instead of being vendored, so a version
// bump is a one-line edit that re-fetches a byte-stable corpus. `tool_reactor` reads
// `WEBVIEW2_VERSION` back and fails loudly if `windows-reactor-setup` drifts from it.
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VERSION: &str = "1.0.4022.49";

fn main() {
    let time = std::time::Instant::now();

    // Like `tool_win32`/`tool_wdk`, provision and pin libclang before the first parse: download
    // the exact `LIBCLANG_VERSION` wheel on demand (unless `LIBCLANG_PATH` is set) and assert the
    // loaded version, so the WebView2 corpus is generated against the same clang everywhere — in
    // CI and on a fresh checkout — instead of whatever LLVM happens to be installed.
    ensure_libclang();
    assert_libclang_version();

    // The pinned NuGet package lays the C/C++ headers out under `build/native`: the core API and
    // options header live in `include/`, while the COM<->WinRT bridge header sits in
    // `include-winrt/`.
    let pkg = nuget_package(WEBVIEW2_PKG, WEBVIEW2_VERSION);
    let include = pkg.join("build").join("native").join("include");
    let include_winrt = pkg.join("build").join("native").join("include-winrt");
    // `WebView2Interop.h` (in `include-winrt/`) `#include`s `"WebView2.h"` from the sibling
    // `include/` dir, so that directory has to be on the header search path.
    let include_arg = format!("-I{}", include.display());

    // WebView2 ships only a C/C++ header, so the binding pipeline starts there:
    // WebView2*.h -> WebView2.rdl (clang) -> WebView2.winmd (reader) -> bindings.rs (bindgen).
    // Each header is parsed as its own translation unit (only its own declarations are
    // emitted, not its #includes), so both headers are listed: WebView2.h yields the core
    // COM API and WebView2Interop.h yields the ICoreWebView2Interop2::GetComICoreWebView2
    // bridge used to reuse these COM wrappers from the WinUI/WinRT WebView2 XAML control.
    clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
            &include_arg,
        ])
        .input(include.join("WebView2.h").to_str().unwrap())
        .input(include_winrt.join("WebView2Interop.h").to_str().unwrap())
        .input("crates/libs/bindgen/default/Windows.Win32.winmd")
        .input("crates/libs/bindgen/default/Windows.winmd")
        .output("target/webview/WebView2.rdl")
        .namespace("WebView2")
        .library("WebView2Loader.dll")
        .write()
        .unwrap();

    reader()
        .input("target/webview/WebView2.rdl")
        .input("crates/libs/bindgen/default/Windows.Win32.winmd")
        .input("crates/libs/bindgen/default/Windows.winmd")
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    // Feature-gated WinRT bindings for the `reactor` integration: the WinUI XAML
    // `WebView2` control (Microsoft.UI.Xaml.winmd) and the WinRT `CoreWebView2`
    // (Microsoft.Web.WebView2.Core.winmd), generated straight from the vendored
    // winmd metadata. The control's `CoreWebView2` is bridged to the COM
    // `ICoreWebView2` above via `ICoreWebView2Interop2::GetComICoreWebView2`.
    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/reactor.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
