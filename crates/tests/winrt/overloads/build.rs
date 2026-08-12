fn main() {
    if !cfg!(windows) {
        return;
    }

    println!("cargo:rerun-if-changed=src/metadata.idl");

    let mut command = std::process::Command::new("midlrt.exe");
    command.args([
        "/winrt",
        "/nomidl",
        "/h",
        "nul",
        "/metadata_dir",
        "../../../libs/default",
        "/reference",
        "../../../libs/default/Windows.winmd",
        "/winmd",
        "metadata.winmd",
        "src/metadata.idl",
    ]);

    assert!(command.status().unwrap().success(), "Failed to run midlrt");

    windows_bindgen2::builder()
        .input("metadata.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("test_overloads")
        .implement_all()
        .flat()
        .write()
        .unwrap();
}
