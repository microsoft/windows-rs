use helpers::*;
use windows_clang::{Input, Target, capture_partial, projection};
use windows_rdl::*;

// WebView2 owns its SDK pin here: the headers are downloaded from this exact NuGet package
// (via `nuget_package`, like the other header scrapers) instead of being vendored, so a version
// bump is a one-line edit that re-fetches byte-stable headers. `tool-reactor-metadata` reads the
// pin to refresh its committed Core.winmd and rejects drift in `windows-reactor-setup`.
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
    let metadata = windows_metadata::reader::Index::new(vec![
        windows_metadata::reader::File::read("crates\\libs\\default\\Windows.winmd").unwrap(),
        windows_metadata::reader::File::read("crates\\libs\\default\\Windows.Win32.winmd").unwrap(),
    ]);
    let references = projection::References {
        global_namespace: Some("Windows.Win32"),
        metadata: &metadata,
        native_prefixes: &["", "ABI"],
        delegate_prefix: Some("I"),
    };
    let external_interfaces = references.native_interfaces().unwrap();
    assert!(
        external_interfaces.contains("IUnknown"),
        "missing authoritative IUnknown metadata"
    );
    let inputs: Vec<_> = [
        include.join("WebView2.h"),
        include_winrt.join("WebView2Interop.h"),
    ]
    .into_iter()
    .map(|path| Input {
        name: path.display().to_string(),
        source: std::fs::read_to_string(&path).unwrap(),
        arguments: vec![
            "-fms-extensions".into(),
            include_arg.clone(),
            "-include".into(),
            std::path::absolute("crates\\tools\\win32\\src\\sal.h")
                .unwrap()
                .display()
                .to_string(),
        ],
        root_paths: Some(vec![path]),
        external_interfaces: external_interfaces.clone(),
        ..Default::default()
    })
    .collect();
    let imports = [
        "CompareBrowserVersions",
        "CreateCoreWebView2Environment",
        "CreateCoreWebView2EnvironmentWithOptions",
        "GetAvailableCoreWebView2BrowserVersionString",
        "GetAvailableCoreWebView2BrowserVersionStringWithOptions",
    ]
    .into_iter()
    .map(|name| {
        (
            name.into(),
            projection::Import {
                library: "WebView2Loader.dll".into(),
                name: None,
            },
        )
    })
    .collect();
    let captured = capture_partial(&inputs, &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let projected = projection::project_partial_with_options(
        captured,
        "WebView2",
        &projection::Options {
            imports: Some(&imports),
            references: Some(&references),
            ..Default::default()
        },
    )
    .unwrap();
    std::fs::create_dir_all("target\\webview").unwrap();
    std::fs::write(
        "target\\webview\\omissions.txt",
        projected.omissions.join("\n") + "\n",
    )
    .unwrap();
    if !projected.omissions.is_empty() {
        eprintln!(
            "PARTIAL: {} WebView2 omissions; see target\\webview\\omissions.txt",
            projected.omissions.len()
        );
    }
    let output = projected.output;
    assert_eq!(
        output.functions,
        imports.functions.len(),
        "missing WebView2 loader imports"
    );
    std::fs::write(
        "target\\webview\\WebView2.rdl",
        output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect::<String>(),
    )
    .unwrap();

    reader()
        .input("metadata\\metadata.rdl")
        .input("target/webview/WebView2.rdl")
        .reference_default()
        .output("target/webview/WebView2.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen(["--etc", "crates/tools/webview/src/webview.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
