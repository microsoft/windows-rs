fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--filter",
        "CoGetCallerTID",
        "IsCharLowerA",
        "SysFreeString",
        "IStringable",
        "--flat",
        "--no-comment",
    ])
    .unwrap();
}
