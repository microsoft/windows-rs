fn main() {
    msvc_main();
}

#[cfg(not(target_env = "msvc"))]
fn msvc_main() {}

#[cfg(target_env = "msvc")]
fn msvc_main() {
    println!("cargo:rerun-if-changed=src/test.rdl");
    println!("cargo:rerun-if-changed=src/interop.cpp");
    println!("cargo:rustc-link-lib=onecoreuap");

    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    let include = std::env::var("OUT_DIR").unwrap();

    windows_rdl::reader()
        .input("src/test.rdl")
        .input(&metadata_dir)
        .output("test.winmd")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "test.winmd",
        &metadata_dir,
        "--out",
        "src/bindings.rs",
        "--filter",
        "Test",
        "--implement",
        "--flat",
        "--no-comment",
        "--reference",
        "windows_collections,flat,Windows",
    ])
    .unwrap();

    cppwinrt::cppwinrt(["-in", "test.winmd", &metadata_dir, "-out", &include]);

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file("src/interop.cpp")
        .include(include)
        .compile("interop");
}
