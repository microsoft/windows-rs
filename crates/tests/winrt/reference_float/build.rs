fn main() {
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
        "metadata.winmd",
        "src/metadata.idl",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }

    _ = windows_bindgen::bindgen([
        "--in",
        "metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_reference_float",
        "--implement",
        "--no-comment",
        "--flat",
    ]);
}
