#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BinaryStringEncoding(pub i32);
impl BinaryStringEncoding {
    pub const Utf8: BinaryStringEncoding = BinaryStringEncoding(0i32);
    pub const Utf16LE: BinaryStringEncoding = BinaryStringEncoding(1i32);
    pub const Utf16BE: BinaryStringEncoding = BinaryStringEncoding(2i32);
}
#[repr(transparent)]
pub struct ICryptographicBufferStatics(pub *mut ::core::ffi::c_void);
