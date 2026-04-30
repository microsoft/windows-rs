fn main() {
    msvc_main();
}

#[cfg(not(target_env = "msvc"))]
fn msvc_main() {}

#[cfg(target_env = "msvc")]
fn msvc_main() {
    println!("cargo:rerun-if-changed=src/robot.rdl");
    println!("cargo:rerun-if-changed=src/component.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");
    println!("cargo:rustc-link-arg-cdylib=/export:DllGetActivationFactory");
    println!("cargo:rustc-link-arg-cdylib=/export:CreateRobotFromHandle");

    let include = std::env::var("OUT_DIR").unwrap();
    let reference = "../../../libs/bindgen/default";

    windows_rdl::reader()
        .input("src/robot.rdl")
        .input(reference)
        .output("robot.winmd")
        .write()
        .unwrap();

    cppwinrt::cppwinrt(["-in", "robot.winmd", reference, "-out", &include]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .flag("/W4")
        .flag("/WX")
        .file("src/component.cpp")
        .include(include)
        .compile("component");
}
