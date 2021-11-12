#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[link(name = "windows")]
extern "system" {}
pub struct BinaryStringEncoding(i32);
pub struct CryptographicBuffer(i32);
pub struct ICryptographicBufferStatics(pub *mut ::core::ffi::c_void);
