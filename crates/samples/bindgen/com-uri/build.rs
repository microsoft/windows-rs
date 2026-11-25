fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .full()
        .filters(["CreateUri", "IUri::{GetDomain,GetPort}"])
        .write();
}
