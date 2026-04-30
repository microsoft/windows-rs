fn main() {
    if !cfg!(windows) {
        return;
    }

    println!("cargo:rerun-if-changed=../activation/metadata.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../activation/metadata.winmd",
        "default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_activation",
        "--no-comment",
        "--flat",
    ])
    .unwrap();
}
