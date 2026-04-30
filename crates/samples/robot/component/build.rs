fn main() {
    println!("cargo:rerun-if-changed=src/robot.rdl");

    let reference = "../../../libs/bindgen/default";

    windows_rdl::reader()
        .input("src/robot.rdl")
        .input(reference)
        .output("robot.winmd")
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input("robot.winmd")
        .input(reference)
        .output("src/bindings.rs")
        .filter("Robotics")
        .flat()
        .implement()
        .write()
        .unwrap();
}
