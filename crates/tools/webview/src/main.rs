use windows_clang::*;
use windows_rdl::*;

fn main() {
    let time = std::time::Instant::now();

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
        ])
        .input("crates/tools/webview/WebView2.h")
        .input("crates/tools/webview/WebView2Interop.h")
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
    // (Microsoft.Web.WebView2.Core.winmd), generated from the same WinUI metadata
    // `tool_reactor` fetches on demand (staged under `target/reactor/winmd`). The
    // control's `CoreWebView2` is bridged to the COM `ICoreWebView2` above via
    // `ICoreWebView2Interop2::GetComICoreWebView2`.
    let winmd_dir = tool_reactor::stage::winmd_dir()
        .to_str()
        .expect("winmd dir path is valid UTF-8");
    windows_bindgen::bindgen([
        "--in",
        winmd_dir,
        "default",
        "--etc",
        "crates/tools/webview/src/reactor.txt",
    ]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
