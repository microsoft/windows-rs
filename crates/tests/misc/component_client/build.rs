fn main() {
    msvc_main();
}

#[cfg(not(target_env = "msvc"))]
fn msvc_main() {}

#[cfg(target_env = "msvc")]
fn msvc_main() {
    windows_bindgen::bindgen([
        "--in",
        "../component/component.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
