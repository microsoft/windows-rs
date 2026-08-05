fn main() {
    println!("cargo:rerun-if-changed=src/bench.rdl");

    windows_rdl::reader()
        .input("src/bench.rdl")
        .reference_default()
        .output("bench.winmd")
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input("bench.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("Bench")
        .flat()
        .implement_all()
        .write();
}
