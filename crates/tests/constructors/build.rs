fn main() {
    let mut command = std::process::Command::new("midlrt.exe");
    command.args([
        "/winrt",
        "/nomidl",
        "/h",
        "nul",
        "/metadata_dir",
        "../../libs/bindgen/default",
        "/reference",
        "../../libs/bindgen/default/Windows.winmd",
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
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_constructors", // TODO: this doesn't work when specifying type directly
        "--config",
        "implement",
        "no-bindgen-comment",
        // "flatten", TODO: this does more than just remove the namespace
    ])
    .unwrap();
}
