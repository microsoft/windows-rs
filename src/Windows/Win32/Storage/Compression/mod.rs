#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct COMPRESSOR_HANDLE(pub isize);
impl ::core::default::Default for COMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for COMPRESSOR_HANDLE {}
unsafe impl ::windows::core::Abi for COMPRESSOR_HANDLE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPRESS_ALGORITHM(pub u32);
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(2u32);
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(3u32);
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(4u32);
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = COMPRESS_ALGORITHM(5u32);
impl ::core::convert::From<u32> for COMPRESS_ALGORITHM {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COMPRESS_ALGORITHM {
    type Abi = Self;
}
impl ::core::ops::BitOr for COMPRESS_ALGORITHM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for COMPRESS_ALGORITHM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for COMPRESS_ALGORITHM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for COMPRESS_ALGORITHM {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for COMPRESS_ALGORITHM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0u32;
pub const COMPRESS_ALGORITHM_MAX: u32 = 6u32;
pub const COMPRESS_ALGORITHM_NULL: u32 = 1u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: ::core::option::Option<PFN_COMPRESS_ALLOCATE>,
    pub Free: ::core::option::Option<PFN_COMPRESS_FREE>,
    pub UserContext: *mut ::core::ffi::c_void,
}
impl COMPRESS_ALLOCATION_ROUTINES {}
impl ::core::default::Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COMPRESS_ALLOCATION_ROUTINES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMPRESS_ALLOCATION_ROUTINES").field("UserContext", &self.UserContext).finish()
    }
}
impl ::core::cmp::PartialEq for COMPRESS_ALLOCATION_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.Allocate.map(|f| f as usize) == other.Allocate.map(|f| f as usize) && self.Free.map(|f| f as usize) == other.Free.map(|f| f as usize) && self.UserContext == other.UserContext
    }
}
impl ::core::cmp::Eq for COMPRESS_ALLOCATION_ROUTINES {}
unsafe impl ::windows::core::Abi for COMPRESS_ALLOCATION_ROUTINES {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPRESS_INFORMATION_CLASS(pub i32);
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(0i32);
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(1i32);
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = COMPRESS_INFORMATION_CLASS(2i32);
impl ::core::convert::From<i32> for COMPRESS_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COMPRESS_INFORMATION_CLASS {
    type Abi = Self;
}
pub const COMPRESS_RAW: u32 = 536870912u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseCompressor<'a, Param0: ::windows::core::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseCompressor(compressorhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseDecompressor(::core::mem::transmute(decompressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Compress<'a, Param0: ::windows::core::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0, uncompresseddata: *const ::core::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::core::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: *const ::core::ffi::c_void, uncompresseddatasize: usize, compressedbuffer: *mut ::core::ffi::c_void, compressedbuffersize: usize, compresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(Compress(compressorhandle.into_param().abi(), ::core::mem::transmute(uncompresseddata), ::core::mem::transmute(uncompresseddatasize), ::core::mem::transmute(compressedbuffer), ::core::mem::transmute(compressedbuffersize), ::core::mem::transmute(compresseddatasize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle: *mut isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const ::core::mem::ManuallyDrop<COMPRESS_ALLOCATION_ROUTINES>, compressorhandle: *mut isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateCompressor(::core::mem::transmute(algorithm), ::core::mem::transmute(allocationroutines), ::core::mem::transmute(compressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDecompressor(algorithm: COMPRESS_ALGORITHM, allocationroutines: *const ::core::mem::ManuallyDrop<COMPRESS_ALLOCATION_ROUTINES>, decompressorhandle: *mut isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateDecompressor(::core::mem::transmute(algorithm), ::core::mem::transmute(allocationroutines), ::core::mem::transmute(decompressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Decompress(decompressorhandle: isize, compresseddata: *const ::core::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::core::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Decompress(decompressorhandle: isize, compresseddata: *const ::core::ffi::c_void, compresseddatasize: usize, uncompressedbuffer: *mut ::core::ffi::c_void, uncompressedbuffersize: usize, uncompresseddatasize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(Decompress(::core::mem::transmute(decompressorhandle), ::core::mem::transmute(compresseddata), ::core::mem::transmute(compresseddatasize), ::core::mem::transmute(uncompressedbuffer), ::core::mem::transmute(uncompressedbuffersize), ::core::mem::transmute(uncompresseddatasize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type PFN_COMPRESS_ALLOCATE = unsafe extern "system" fn(usercontext: *const ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void;
pub type PFN_COMPRESS_FREE = unsafe extern "system" fn(usercontext: *const ::core::ffi::c_void, memory: *const ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryCompressorInformation<'a, Param0: ::windows::core::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryCompressorInformation(compressorhandle.into_param().abi(), ::core::mem::transmute(compressinformationclass), ::core::mem::transmute(compressinformation), ::core::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryDecompressorInformation(::core::mem::transmute(decompressorhandle), ::core::mem::transmute(compressinformationclass), ::core::mem::transmute(compressinformation), ::core::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetCompressor<'a, Param0: ::windows::core::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetCompressor(compressorhandle: COMPRESSOR_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ResetCompressor(compressorhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetDecompressor(decompressorhandle: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ResetDecompressor(::core::mem::transmute(decompressorhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCompressorInformation<'a, Param0: ::windows::core::IntoParam<'a, COMPRESSOR_HANDLE>>(compressorhandle: Param0, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCompressorInformation(compressorhandle.into_param().abi(), ::core::mem::transmute(compressinformationclass), ::core::mem::transmute(compressinformation), ::core::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDecompressorInformation(decompressorhandle: isize, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const ::core::ffi::c_void, compressinformationsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDecompressorInformation(::core::mem::transmute(decompressorhandle), ::core::mem::transmute(compressinformationclass), ::core::mem::transmute(compressinformation), ::core::mem::transmute(compressinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
