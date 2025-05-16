fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    println!("cargo:rerun-if-changed=src/interop.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    windows_bindgen::bindgen([
        "--in",
        "../composable/metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_composable",
        "--no-comment",
        "--flat",
    ])
    .unwrap();

    let include = std::env::var("OUT_DIR").unwrap();

    cppwinrt::cppwinrt([
        "-in",
        "../composable/metadata.winmd",
        "../../../libs/bindgen/default",
        "-out",
        &include,
    ]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/interop.cpp")
        .include(include)
        .compile("interop");
}
