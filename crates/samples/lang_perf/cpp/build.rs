fn main() {
    msvc_main();
}

#[cfg(not(target_env = "msvc"))]
fn msvc_main() {}

#[cfg(target_env = "msvc")]
fn msvc_main() {
    println!("cargo:rerun-if-changed=../component/lang.winmd");
    println!("cargo:rerun-if-changed=src/bench.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    let include = std::env::var("OUT_DIR").unwrap();

    cppwinrt::cppwinrt([
        "-in",
        "../component/lang.winmd",
        "../../../libs/bindgen/default",
        "-out",
        &include,
    ]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .flag("/W4")
        .file("src/bench.cpp")
        .include(include)
        .compile("lang_perf_cpp");
}
