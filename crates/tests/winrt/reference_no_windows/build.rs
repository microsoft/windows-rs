fn main() {
    println!("cargo:rerun-if-changed=src/test.rdl");

    windows_rdl::reader()
        .output("test.winmd")
        .input("src/test.rdl")
        .reference("../../../libs/bindgen/default")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "default",
        "test.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Test",
        "IStringable",
        "--implement",
        "--flat",
        "--no-comment",
    ])
    .unwrap();
}
