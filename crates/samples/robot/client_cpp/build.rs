fn main() {
    msvc_main();
}

#[cfg(not(target_env = "msvc"))]
fn msvc_main() {}

#[cfg(target_env = "msvc")]
fn msvc_main() {
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
        .flag("/W4")
        .flag("/WX")
        .file("src/main.cpp")
        .include(include)
        .compile("main_cpp");
}
