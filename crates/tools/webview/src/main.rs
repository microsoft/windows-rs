use windows_clang::*;
use windows_rdl::*;

// WebView2 owns its SDK pin here: the headers are downloaded from this exact NuGet package
// instead of being vendored, so a version bump is a one-line edit that re-fetches byte-stable
// headers.
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VERSION: &str = "1.0.4078.44";

fn main() {
    let time = std::time::Instant::now();
    let runtime_version =
        helpers::read_str_const("crates/libs/reactor-setup/src/lib.rs", "WEBVIEW2_VER");
    assert_eq!(
        WEBVIEW2_VERSION, runtime_version,
        "windows-reactor-setup's WebView2 runtime must match tool_webview"
    );

    // Like `tool_win32`, provision and pin libclang before the first parse: download
    // the exact `LIBCLANG_VERSION` wheel on demand (unless `LIBCLANG_PATH` is set) and assert the
    // loaded version, so the WebView2 metadata is generated against the same clang everywhere - in
    // CI and on a fresh checkout - instead of whatever LLVM happens to be installed.
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
        .input(include.join("WebView2.h"))
        .input(include_winrt.join("WebView2Interop.h"))
        .reference_default()
        .output("target/webview/WebView2.rdl")
        .namespace("WebView2")
        .library("WebView2Loader.dll")
        .write()
        .unwrap();

    reader()
        .input("target/webview/WebView2.rdl")
        .reference_default()
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    // Feature-gated WinRT bindings for the `reactor` integration. The control's WinUI metadata
    // comes from the Windows App SDK package matching windows-reactor-setup's runtime pin.
    let metadata = helpers::windows_app_sdk_metadata(nuget_package);
    windows_bindgen::builder()
        .inputs([
            metadata.foundation,
            metadata.interactive,
            metadata.winui,
            pkg.join("lib").join("Microsoft.Web.WebView2.Core.winmd"),
            "crates/libs/default/Windows.winmd".into(),
        ])
        .output("crates/libs/webview/src/reactor_bindings.rs")
        .flat()
        .minimal()
        .implements([
            "Windows.Foundation.TypedEventHandler",
            "Microsoft.UI.Xaml.RoutedEventHandler",
        ])
        .filter_file("crates/tools/webview/src/reactor.txt")
        .write();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
