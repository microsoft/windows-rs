fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    println!("cargo:rerun-if-changed=src/interop.cpp");

    windows_bindgen::bindgen([
        "--in",
        "../constructors/metadata.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_constructors",
        "--config",
        "no-bindgen-comment",
    ])
    .unwrap();

    let include = std::env::var("OUT_DIR").unwrap();

    cppwinrt::cppwinrt([
        "-in",
        "../constructors/metadata.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "-out",
        &include,
    ])
    .unwrap();

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/interop.cpp")
        .include(include)
        .compile("interop");
}
