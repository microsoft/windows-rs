fn main() {
    println!("cargo:rerun-if-changed=src/client.cpp");
    println!("cargo:rustc-link-lib=ole32");

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .file("src/client.cpp")
        .compile("client");
}
