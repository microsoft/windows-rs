fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    println!("cargo:rerun-if-changed=src/client.cpp");
    println!("cargo:rustc-link-lib=windows.0.52.0");

    cppwinrt::cppwinrt([
        "-in",
        "../json_validator_winrt/sample.winmd",
        &format!("{}\\System32\\WinMetadata", env!("windir")),
        "-out",
        "src",
    ])
    .unwrap();

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/client.cpp")
        .compile("client");
}
