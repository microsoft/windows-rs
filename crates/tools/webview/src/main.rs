use windows_clang::*;
use windows_rdl::*;

fn default_type_references() -> std::collections::BTreeMap<String, windows_clang2::TypeReference> {
    let files = [windows_default::WINRT, windows_default::WIN32]
        .into_iter()
        .map(|bytes| windows_metadata::reader::File::new(bytes.to_vec()).unwrap())
        .collect();
    let index = windows_metadata::reader::Index::new(files);
    let mut references = std::collections::BTreeMap::new();
    let mut ambiguous = std::collections::BTreeSet::new();
    for (namespace, name, ty) in index.iter() {
        let reference = windows_clang2::TypeReference::new(
            namespace,
            ty.category() == windows_metadata::reader::TypeCategory::Interface,
        );
        if references
            .insert(name.to_string(), reference.clone())
            .is_some_and(|existing| existing != reference)
        {
            ambiguous.insert(name.to_string());
        }
    }
    references.retain(|name, _| !ambiguous.contains(name));
    references
}

// WebView2 owns its SDK pin here: the headers are downloaded from this exact NuGet package
// (via `nuget_package`, like the other header scrapers) instead of being vendored, so a version
// bump is a one-line edit that re-fetches byte-stable headers. `tool-reactor` reads the pin to
// refresh its committed Core.winmd and rejects drift in `windows-reactor-setup`.
const WEBVIEW2_PKG: &str = "Microsoft.Web.WebView2";
const WEBVIEW2_VERSION: &str = "1.0.4078.44";

fn main() {
    let time = std::time::Instant::now();

    // Like `tool-win32`, provision and pin libclang before the first parse: download
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
    let inputs = [
        include.join("WebView2.h"),
        include_winrt.join("WebView2Interop.h"),
    ];
    let args = [
        "-x",
        "c++",
        "--target=x86_64-pc-windows-msvc",
        "-fms-extensions",
        &include_arg,
    ];
    if std::env::var_os("WINDOWS_CLANG2").is_some() {
        let inputs = inputs.map(|path| {
            windows_clang2::Input::new(
                path.to_string_lossy(),
                std::fs::read_to_string(&path).unwrap(),
            )
        });
        let references = default_type_references();
        let rdl = windows_clang2::extract(inputs, &args).unwrap();
        let rdl = rdl
            .emit_with_library_and_references("WebView2", "WebView2Loader.dll", &references)
            .unwrap();
        std::fs::create_dir_all("target/webview").unwrap();
        std::fs::write("target/webview/WebView2.rdl", rdl).unwrap();
    } else {
        clang()
            .args(args)
            .input(inputs[0].clone())
            .input(inputs[1].clone())
            .reference_default()
            .output("target/webview/WebView2.rdl")
            .namespace("WebView2")
            .library("WebView2Loader.dll")
            .write()
            .unwrap();
    }

    reader()
        .input("target/webview/WebView2.rdl")
        .reference_default()
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
