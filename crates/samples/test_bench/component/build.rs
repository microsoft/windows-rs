fn main() {
    println!("cargo:rerun-if-changed=src/bench.rdl");

    let reference = "../../../libs/bindgen/default";

    windows_rdl::reader()
        .input("src/bench.rdl")
        .input(reference)
        .output("bench.winmd")
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input("bench.winmd")
        .input(reference)
        .output("src/bindings.rs")
        .filter("Bench")
        .flat()
        .implement(std::iter::empty::<&str>())
        .write();
}
