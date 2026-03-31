fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    println!("cargo:rerun-if-changed=src/client.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    let include = std::env::var("OUT_DIR").unwrap();

    cppwinrt::cppwinrt([
        "-in",
        "../json_validator_winrt/sample.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "-out",
        &include,
    ]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/client.cpp")
        .include(include)
        .compile("client");
}
