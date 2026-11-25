fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .filters([
            "DataProtectionProvider::{CreateOverloadExplicit,ProtectAsync,UnprotectAsync}",
            "CryptographicBuffer::{ConvertStringToBinary,ConvertBinaryToString}",
            "BinaryStringEncoding::Utf8",
            "IBuffer::get_Length",
        ])
        .write();
}
