fn main() {
    if !cfg!(windows) {
        return;
    }

    println!("cargo:rerun-if-changed=../overloads/metadata.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../overloads/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_overloads",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
