#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPRESSOR_HANDLE(pub isize);
impl COMPRESSOR_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for COMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for COMPRESSOR_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for COMPRESSOR_HANDLE {}
impl ::core::fmt::Debug for COMPRESSOR_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESSOR_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<COMPRESSOR_HANDLE>> for COMPRESSOR_HANDLE {
    fn from(optional: ::core::option::Option<COMPRESSOR_HANDLE>) -> COMPRESSOR_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for COMPRESSOR_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPRESS_ALGORITHM(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(2u32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(3u32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(4u32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(5u32);
impl ::core::marker::Copy for COMPRESS_ALGORITHM {}
impl ::core::clone::Clone for COMPRESS_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPRESS_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMPRESS_ALGORITHM {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMPRESS_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESS_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: PFN_COMPRESS_ALLOCATE,
    pub Free: PFN_COMPRESS_FREE,
    pub UserContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for COMPRESS_ALLOCATION_ROUTINES {}
impl ::core::clone::Clone for COMPRESS_ALLOCATION_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPRESS_ALLOCATION_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPRESS_ALLOCATION_ROUTINES").field("Allocate", &self.Allocate.map(|f| f as usize)).field("Free", &self.Free.map(|f| f as usize)).field("UserContext", &self.UserContext).finish()
    }
}
unsafe impl ::windows::core::Abi for COMPRESS_ALLOCATION_ROUTINES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPRESS_ALLOCATION_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPRESS_ALLOCATION_ROUTINES>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPRESS_ALLOCATION_ROUTINES {}
impl ::core::default::Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPRESS_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for COMPRESS_INFORMATION_CLASS {}
impl ::core::clone::Clone for COMPRESS_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPRESS_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMPRESS_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMPRESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPRESS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub const COMPRESS_RAW: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseCompressor<'a, P0>(compressorhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<COMPRESSOR_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
    }
    CloseCompressor(compressorhandle.into())
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
    }
    CloseDecompressor(decompressorhandle)
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Compress<'a, P0>(compressorhandle: P0, uncompresseddata: *const ::core::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::core::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<COMPRESSOR_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: *const ::core::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::core::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
    }
    Compress(compressorhandle.into(), ::core::mem::transmute(uncompresseddata), uncompresseddatasize, ::core::mem::transmute(compressedbuffer), compressedbuffersize, ::core::mem::transmute(compresseddatasize))
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle: *mut isize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle: *mut isize) -> super::super::Foundation::BOOL;
    }
    CreateCompressor(algorithm, ::core::mem::transmute(allocationroutines), ::core::mem::transmute(compressorhandle))
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL;
    }
    CreateDecompressor(algorithm, ::core::mem::transmute(allocationroutines), ::core::mem::transmute(decompressorhandle))
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Decompress(decompressorhandle: isize, compresseddata: *const ::core::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::core::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn Decompress(decompressorhandle: isize, compresseddata: *const ::core::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::core::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
    }
    Decompress(decompressorhandle, ::core::mem::transmute(compresseddata), compresseddatasize, ::core::mem::transmute(uncompressedbuffer), uncompressedbuffersize, ::core::mem::transmute(uncompresseddatasize))
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub type PFN_COMPRESS_ALLOCATE = ::core::option::Option<unsafe extern "system" fn(usercontext: *const ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Storage_Compression\"`*"]
pub type PFN_COMPRESS_FREE = ::core::option::Option<unsafe extern "system" fn(usercontext: *const ::core::ffi::c_void, memory: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryCompressorInformation<'a, P0>(compressorhandle: P0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<COMPRESSOR_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    }
    QueryCompressorInformation(compressorhandle.into(), compressinformationclass, ::core::mem::transmute(compressinformation), compressinformationsize)
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    }
    QueryDecompressorInformation(decompressorhandle, compressinformationclass, ::core::mem::transmute(compressinformation), compressinformationsize)
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetCompressor<'a, P0>(compressorhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<COMPRESSOR_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ResetCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
    }
    ResetCompressor(compressorhandle.into())
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
    }
    ResetDecompressor(decompressorhandle)
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCompressorInformation<'a, P0>(compressorhandle: P0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<COMPRESSOR_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    }
    SetCompressorInformation(compressorhandle.into(), compressinformationclass, ::core::mem::transmute(compressinformation), compressinformationsize)
}
#[doc = "*Required features: `\"Win32_Storage_Compression\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
    }
    SetDecompressorInformation(decompressorhandle, compressinformationclass, ::core::mem::transmute(compressinformation), compressinformationsize)
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
