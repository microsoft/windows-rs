fn main() {
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

    cppwinrt::cppwinrt([
        "-in",
        "../constructors/metadata.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "-out",
        "src",
    ])
    .unwrap();

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/interop.cpp")
        .compile("interop");
}
