fn main() {
    println!("cargo:rerun-if-changed=src/test.rdl");

    windows_rdl::reader()
        .output("test.winmd")
        .input("src/test.rdl")
        .reference("../../../libs/bindgen/default")
        .write()
        .unwrap();

    _ = windows_bindgen::bindgen([
        "--in",
        "default",
        "test.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Test",
        "IStringable",
        "Vector2",
        "IVector",
        "IAsyncAction",
        "--implement",
        "--flat",
        "--no-deps",
        "--no-comment",
    ]);
}
