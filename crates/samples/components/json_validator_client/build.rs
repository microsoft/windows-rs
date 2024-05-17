fn main() {
    println!("cargo:rerun-if-changed=src/client.cpp");

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .file("src/client.cpp")
        .compile("client");
}
