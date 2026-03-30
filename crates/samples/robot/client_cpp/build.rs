fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    println!("cargo:rerun-if-changed=../component/robot.winmd");
    println!("cargo:rerun-if-changed=src/main.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    let include = std::env::var("OUT_DIR").unwrap();

    cppwinrt::cppwinrt([
        "-in",
        "../component/robot.winmd",
        "../../../libs/bindgen/default",
        "-out",
        &include,
    ]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/main.cpp")
        .include(include)
        .compile("main_cpp");
}
