fn main() {
    println!("cargo:rerun-if-changed=../component/lang.winmd");

    windows_bindgen::builder()
        .input("../component/lang.winmd")
        .input("../../../libs/bindgen/default")
        .output("src/bindings.rs")
        .filter("LangPerf")
        .flat()
        .write();
}
