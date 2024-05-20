fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    println!("cargo:rerun-if-changed=src/client.cpp");
    println!("cargo:rustc-link-lib=windows.0.52.0");

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .file("src/client.cpp")
        .compile("client");
}
