fn main() {
    msvc_main();
}

#[cfg(not(target_env = "msvc"))]
fn msvc_main() {}

#[cfg(target_env = "msvc")]
fn msvc_main() {
    println!("cargo:rerun-if-changed=src/component.rdl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));

    windows_rdl::reader()
        .input("src/component.rdl")
        .input(&metadata_dir)
        .output("component.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "component.winmd",
        &metadata_dir,
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_component",
        "--flat",
        "--implement",
        "--no-comment",
        "--reference",
        "windows",
    ])
    .unwrap();
}
