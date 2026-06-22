use windows_rdl::*;

fn main() {
    let time = std::time::Instant::now();

    // WebView2 ships only a C/C++ header, so the binding pipeline starts there:
    // WebView2.h -> WebView2.rdl (clang) -> WebView2.winmd (reader) -> bindings.rs (bindgen).
    clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input("crates/tools/webview/WebView2.h")
        .input("crates/libs/bindgen/default/Windows.Win32.winmd")
        .output("target/webview/WebView2.rdl")
        .namespace("WebView2")
        .library("WebView2Loader.dll")
        .write()
        .unwrap();

    reader()
        .input("target/webview/WebView2.rdl")
        .input("crates/libs/bindgen/default")
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
