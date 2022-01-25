fn main() {
    let mut from = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    from.push("message.txt");

    let mut to = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    to.push("message.txt");

    std::fs::copy(from, to).unwrap();
}
