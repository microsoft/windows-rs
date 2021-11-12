#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Security_Cryptography_Core")]
pub mod Core;
#[cfg(feature = "Security_Cryptography_DataProtection")]
pub mod DataProtection;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BinaryStringEncoding(i32);
#[repr(transparent)]
pub struct CryptographicBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICryptographicBufferStatics(pub *mut ::core::ffi::c_void);
