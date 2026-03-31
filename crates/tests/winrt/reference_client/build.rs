fn main() {
    println!("cargo:rerun-if-changed=../reference/metadata.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../reference/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_reference",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
