fn main() {
    println!("cargo:rerun-if-changed=src/metadata.rdl");

    windows_rdl::reader()
        .input("src/metadata.rdl")
        .input("../../../libs/bindgen/default")
        .output("metadata.winmd")
        .write()
        .unwrap();

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
