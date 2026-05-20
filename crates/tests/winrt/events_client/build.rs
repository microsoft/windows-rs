fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../events/metadata.winmd");
    println!("cargo:rerun-if-changed=../../../libs/bindgen/default");

    windows_bindgen::bindgen([
        "--in",
        "../events/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_events",
        "--flat",
        "--reference",
        "windows",
    ])
    .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "../events/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/auto_bindings.rs",
        "--filter",
        "test_events",
        "--flat",
        "--reference",
        "windows",
        "--minimal",
    ])
    .unwrap();
}
