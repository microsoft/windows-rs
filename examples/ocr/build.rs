fn main() {
    let mut from = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    from.push("message.png");

    let mut to = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    to.push("message.png");

    std::fs::copy(from, to).unwrap();
}
