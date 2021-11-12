#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompressAlgorithm(pub i32);
impl CompressAlgorithm {
    pub const InvalidAlgorithm: CompressAlgorithm = CompressAlgorithm(0i32);
    pub const NullAlgorithm: CompressAlgorithm = CompressAlgorithm(1i32);
    pub const Mszip: CompressAlgorithm = CompressAlgorithm(2i32);
    pub const Xpress: CompressAlgorithm = CompressAlgorithm(3i32);
    pub const XpressHuff: CompressAlgorithm = CompressAlgorithm(4i32);
    pub const Lzms: CompressAlgorithm = CompressAlgorithm(5i32);
}
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
