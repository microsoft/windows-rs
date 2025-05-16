fn main() {
    println!("cargo:rerun-if-changed=../events/metadata.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../events/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_events",
        "--no-comment",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();
}
