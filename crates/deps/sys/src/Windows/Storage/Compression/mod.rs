#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CompressAlgorithm(i32);
#[repr(transparent)]
pub struct Compressor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Decompressor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompressor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompressorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDecompressor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDecompressorFactory(pub *mut ::core::ffi::c_void);
