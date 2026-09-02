fn main() {
    println!("cargo:rerun-if-changed=src/shaders.hlsl");
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::copy("src/shaders.hlsl", output.join("shaders.hlsl")).unwrap();
}
