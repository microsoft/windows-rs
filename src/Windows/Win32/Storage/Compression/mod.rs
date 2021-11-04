#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct COMPRESSOR_HANDLE(pub isize);
impl ::std::default::Default for COMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for COMPRESSOR_HANDLE {}
unsafe impl ::windows::runtime::Abi for COMPRESSOR_HANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Compression`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPRESS_ALGORITHM(pub u32);
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(2u32);
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(3u32);
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(4u32);
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(5u32);
impl ::std::convert::From<u32> for COMPRESS_ALGORITHM {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPRESS_ALGORITHM {
    type Abi = Self;
}
impl ::std::ops::BitOr for COMPRESS_ALGORITHM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMPRESS_ALGORITHM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMPRESS_ALGORITHM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMPRESS_ALGORITHM {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMPRESS_ALGORITHM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: ::std::option::Option<PFN_COMPRESS_ALLOCATE>,
    pub Free: ::std::option::Option<PFN_COMPRESS_FREE>,
    pub UserContext: *mut ::std::ffi::c_void,
}
impl COMPRESS_ALLOCATION_ROUTINES {}
impl ::std::default::Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMPRESS_ALLOCATION_ROUTINES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPRESS_ALLOCATION_ROUTINES").field("UserContext", &self.UserContext).finish()
    }
}
impl ::std::cmp::PartialEq for COMPRESS_ALLOCATION_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.Allocate.map(|f| f as usize) == other.Allocate.map(|f| f as usize) && self.Free.map(|f| f as usize) == other.Free.map(|f| f as usize) && self.UserContext == other.UserContext
    }
}
impl ::std::cmp::Eq for COMPRESS_ALLOCATION_ROUTINES {}
unsafe impl ::windows::runtime::Abi for COMPRESS_ALLOCATION_ROUTINES {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Storage_Compression`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPRESS_INFORMATION_CLASS(pub i32);
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(0i32);
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(1i32);
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(2i32);
impl ::std::convert::From<i32> for COMPRESS_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPRESS_INFORMATION_CLASS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub const COMPRESS_RAW: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseCompressor<'a, Param0: ::windows::runtime::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseCompressor(compressorhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseDecompressor(::std::mem::transmute(decompressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Compress<'a, Param0: ::windows::runtime::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0, uncompresseddata: *const ::std::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::std::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: *const ::std::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::std::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Compress(compressorhandle.into_param().abi(), ::std::mem::transmute(uncompresseddata), ::std::mem::transmute(uncompresseddatasize), ::std::mem::transmute(compressedbuffer), ::std::mem::transmute(compressedbuffersize), ::std::mem::transmute(compresseddatasize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle: *mut isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const ::std::mem::ManuallyDrop<COMPRESS_ALLOCATION_ROUTINES>, compressorhandle: *mut isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateCompressor(::std::mem::transmute(algorithm), ::std::mem::transmute(allocationroutines), ::std::mem::transmute(compressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const ::std::mem::ManuallyDrop<COMPRESS_ALLOCATION_ROUTINES>, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDecompressor(::std::mem::transmute(algorithm), ::std::mem::transmute(allocationroutines), ::std::mem::transmute(decompressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Decompress(decompressorhandle: isize, compresseddata: *const ::std::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::std::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Decompress(decompressorhandle: isize, compresseddata: *const ::std::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::std::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Decompress(::std::mem::transmute(decompressorhandle), ::std::mem::transmute(compresseddata), ::std::mem::transmute(compresseddatasize), ::std::mem::transmute(uncompressedbuffer), ::std::mem::transmute(uncompressedbuffersize), ::std::mem::transmute(uncompresseddatasize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub type PFN_COMPRESS_ALLOCATE = unsafe extern "system" fn(usercontext: *const ::std::ffi::c_void, size: usize) -> *mut ::std::ffi::c_void;
#[doc = "*Required features: `Win32_Storage_Compression`*"]
pub type PFN_COMPRESS_FREE = unsafe extern "system" fn(usercontext: *const ::std::ffi::c_void, memory: *const ::std::ffi::c_void);
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryCompressorInformation<'a, Param0: ::windows::runtime::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryCompressorInformation(compressorhandle.into_param().abi(), ::std::mem::transmute(compressinformationclass), ::std::mem::transmute(compressinformation), ::std::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryDecompressorInformation(::std::mem::transmute(decompressorhandle), ::std::mem::transmute(compressinformationclass), ::std::mem::transmute(compressinformation), ::std::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetCompressor<'a, Param0: ::windows::runtime::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ResetCompressor(compressorhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ResetDecompressor(::std::mem::transmute(decompressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCompressorInformation<'a, Param0: ::windows::runtime::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCompressorInformation(compressorhandle.into_param().abi(), ::std::mem::transmute(compressinformationclass), ::std::mem::transmute(compressinformation), ::std::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::std::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDecompressorInformation(::std::mem::transmute(decompressorhandle), ::std::mem::transmute(compressinformationclass), ::std::mem::transmute(compressinformation), ::std::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
