fn main() {
    println!("cargo:rerun-if-changed=../component/robot.winmd");

    windows_bindgen::builder()
        .input("../component/robot.winmd")
        .input("../../../libs/bindgen/default")
        .output("src/bindings.rs")
        .filter("Robotics")
        .flat()
        .write()
        .unwrap();
}
