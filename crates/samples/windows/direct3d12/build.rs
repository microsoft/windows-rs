fn main() {
    println!("!cargo:rerun-if-changed=src/shaders.hlsl");
    std::fs::copy(
        "src/shaders.hlsl",
        std::env::var("OUT_DIR").unwrap() + "/../../../shaders.hlsl",
    )
    .expect("Copy");
}
