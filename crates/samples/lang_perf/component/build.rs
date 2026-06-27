fn main() {
    println!("cargo:rerun-if-changed=src/lang.rdl");

    let reference = "../../../libs/bindgen/default";

    windows_rdl::reader()
        .input("src/lang.rdl")
        .input(reference)
        .output("lang.winmd")
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input("lang.winmd")
        .input(reference)
        .output("src/bindings.rs")
        .filter("LangPerf")
        .flat()
        .implement(std::iter::empty::<&str>())
        .write();
}
