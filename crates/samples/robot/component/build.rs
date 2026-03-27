fn main() {
    println!("cargo:rerun-if-changed=src/robot.rdl");

    let reference = "../../../libs/bindgen/default";

    windows_rdl::reader()
        .input("src/robot.rdl")
        .reference(reference)
        .output("robot.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "robot.winmd",
        reference,
        "--out",
        "src/bindings.rs",
        "--filter",
        "Robotics",
        "--flat",
        "--implement",
    ])
    .unwrap();
}
