fn main() {
    println!("cargo:rerun-if-changed=../component/robot.winmd");

    windows_bindgen::bindgen([
        "--in",
        "../component/robot.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Robotics",
        "--flat",
    ])
    .unwrap();
}
