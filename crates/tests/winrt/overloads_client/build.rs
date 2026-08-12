fn main() {
    if !cfg!(windows) {
        return;
    }

    println!("cargo:rerun-if-changed=../overloads/metadata.winmd");

    windows_bindgen2::builder()
        .input("../overloads/metadata.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("test_overloads")
        .flat()
        .write()
        .unwrap();
}
