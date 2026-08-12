fn main() {
    if !cfg!(windows) {
        return;
    }

    println!("cargo:rerun-if-changed=../activation/metadata.winmd");

    windows_bindgen2::builder()
        .input("../activation/metadata.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("test_activation")
        .flat()
        .write()
        .unwrap();
}
