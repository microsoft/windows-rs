fn main() {
    println!("cargo:rerun-if-changed=src/test.idl");
    let mut command = std::process::Command::new("midlrt.exe");

    command.args([
        "/winrt",
        "/nomidl",
        "/h",
        "nul",
        "/metadata_dir",
        "../../../libs/bindgen/default",
        "/reference",
        "../../../libs/bindgen/default/Windows.winmd",
        "/winmd",
        "test.winmd",
        "src/test.idl",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }

    _ = windows_bindgen::bindgen([
        "--in",
        "default",
        "test.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Test",
        "Vector2",
        "IVector",
        "IAsyncAction",
        "--implement",
        "--flat",
        "--no-comment",
        "--no-deps",
        "--reference",
        "windows,skip-root,Windows.Foundation.IStringable",
    ]);
}
