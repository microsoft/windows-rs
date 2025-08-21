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

    windows_bindgen::bindgen([
        "--in",
        "metadata.winmd",
        "default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_activation",
        "--implement",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
