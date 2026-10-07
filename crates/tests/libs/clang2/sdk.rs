use std::path::PathBuf;
use windows_clang2::{Input, ProjectionOptions, ReferenceKind, Snapshot, TypeReference, capture};

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

pub fn webview_options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("WebView2");
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
    options
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

pub fn arguments(target: &str) -> Vec<String> {
    let include = include();
    let sal = tools().join("win32").join("src").join("sal.h");
    [
        "-x",
        "c++",
        target,
        // Install the annotation shim after the SDK's macro definitions.
        "-include",
        "specstrings.h",
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
