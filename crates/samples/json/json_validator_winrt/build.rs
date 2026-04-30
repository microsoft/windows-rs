fn main() {
    println!("cargo:rerun-if-changed=src/sample.rdl");

    let windows_foundation = format!(
        "{}\\System32\\WinMetadata\\Windows.Foundation.winmd",
        env!("windir")
    );

    windows_rdl::reader()
        .input("src/sample.rdl")
        .input(&windows_foundation)
        .output("sample.winmd")
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input("sample.winmd")
        .input(&windows_foundation)
        .output("src/bindings.rs")
        .filter("Sample")
        .flat()
        .implement()
        .write()
        .unwrap();
}
