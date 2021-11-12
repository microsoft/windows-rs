#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: *const ::core::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::core::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Decompress(decompressorhandle: isize, compresseddata: *const ::core::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::core::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
}
pub struct COMPRESSOR_HANDLE(i32);
pub struct COMPRESS_ALGORITHM(i32);
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
pub struct COMPRESS_ALLOCATION_ROUTINES(i32);
pub struct COMPRESS_INFORMATION_CLASS(i32);
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_RAW: u32 = 536870912u32;
pub struct PFN_COMPRESS_ALLOCATE(i32);
pub struct PFN_COMPRESS_FREE(i32);
