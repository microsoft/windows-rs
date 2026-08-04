fn main() {
    println!("cargo:rerun-if-changed=src/robot.rdl");

    windows_rdl::reader()
        .input("src/robot.rdl")
        .input_default()
        .output("robot.winmd")
        .write()
        .unwrap();

    windows_bindgen::builder()
        .input("robot.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("Robotics")
        .flat()
        .implement_all()
        .write();
}
