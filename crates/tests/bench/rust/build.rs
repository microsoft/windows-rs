fn main() {
    println!("cargo:rerun-if-changed=../component/bench.winmd");

    windows_bindgen::builder()
        .input("../component/bench.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("Bench")
        .flat()
        .write();
}
