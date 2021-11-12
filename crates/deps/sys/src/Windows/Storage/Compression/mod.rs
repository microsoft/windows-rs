#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompressAlgorithm(pub i32);
impl CompressAlgorithm {
    pub const InvalidAlgorithm: Self = Self(0i32);
    pub const NullAlgorithm: Self = Self(1i32);
    pub const Mszip: Self = Self(2i32);
    pub const Xpress: Self = Self(3i32);
    pub const XpressHuff: Self = Self(4i32);
    pub const Lzms: Self = Self(5i32);
}
impl ::core::marker::Copy for CompressAlgorithm {}
impl ::core::clone::Clone for CompressAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Compressor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Compressor {}
impl ::core::clone::Clone for Compressor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Decompressor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Decompressor {}
impl ::core::clone::Clone for Decompressor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompressor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompressor {}
impl ::core::clone::Clone for ICompressor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompressorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompressorFactory {}
impl ::core::clone::Clone for ICompressorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDecompressor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDecompressor {}
impl ::core::clone::Clone for IDecompressor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDecompressorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDecompressorFactory {}
impl ::core::clone::Clone for IDecompressorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
