fn main() {
    println!("cargo:rerun-if-changed=../component/robot.winmd");

    windows_bindgen2::builder()
        .input("../component/robot.winmd")
        .input_default()
        .output("src/bindings.rs")
        .filter("Robotics")
        .flat()
        .write()
        .unwrap();
}
