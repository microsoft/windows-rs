#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CompressAlgorithm(i32);
pub struct Compressor(i32);
pub struct Decompressor(i32);
pub struct ICompressor(pub *mut ::core::ffi::c_void);
pub struct ICompressorFactory(pub *mut ::core::ffi::c_void);
pub struct IDecompressor(pub *mut ::core::ffi::c_void);
pub struct IDecompressorFactory(pub *mut ::core::ffi::c_void);
