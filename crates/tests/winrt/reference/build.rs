fn main() {
    windows_rdl::reader()
        .output("metadata.winmd")
        .input("src/metadata.rdl")
        .reference("../../../libs/bindgen/default")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_reference",
        "--implement",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
