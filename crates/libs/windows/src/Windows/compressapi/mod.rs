#[inline]
pub unsafe fn CloseCompressor(compressorhandle: COMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn CloseCompressor(compressorhandle : COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CloseCompressor(compressorhandle) }
}
#[inline]
pub unsafe fn CloseDecompressor(decompressorhandle: DECOMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn CloseDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CloseDecompressor(decompressorhandle) }
}
#[inline]
pub unsafe fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: Option<*const core::ffi::c_void>, uncompresseddatasize: usize, compressedbuffer: Option<*mut core::ffi::c_void>, compressedbuffersize: usize, compresseddatasize: *mut usize) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn Compress(compressorhandle : COMPRESSOR_HANDLE, uncompresseddata : *const core::ffi::c_void, uncompresseddatasize : usize, compressedbuffer : *mut core::ffi::c_void, compressedbuffersize : usize, compresseddatasize : *mut usize) -> windows_core::BOOL);
    unsafe { Compress(compressorhandle, uncompresseddata.unwrap_or(core::mem::zeroed()) as _, uncompresseddatasize, compressedbuffer.unwrap_or(core::mem::zeroed()) as _, compressedbuffersize, compresseddatasize as _) }
}
#[inline]
pub unsafe fn CreateCompressor(algorithm: u32, allocationroutines: Option<*const COMPRESS_ALLOCATION_ROUTINES>, compressorhandle: *mut COMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn CreateCompressor(algorithm : u32, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle : *mut COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CreateCompressor(algorithm, allocationroutines.unwrap_or(core::mem::zeroed()) as _, compressorhandle as _) }
}
#[inline]
pub unsafe fn CreateDecompressor(algorithm: u32, allocationroutines: Option<*const COMPRESS_ALLOCATION_ROUTINES>, decompressorhandle: *mut COMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn CreateDecompressor(algorithm : u32, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle : *mut COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CreateDecompressor(algorithm, allocationroutines.unwrap_or(core::mem::zeroed()) as _, decompressorhandle as _) }
}
#[inline]
pub unsafe fn Decompress(decompressorhandle: DECOMPRESSOR_HANDLE, compresseddata: Option<*const core::ffi::c_void>, compresseddatasize: usize, uncompressedbuffer: Option<*mut core::ffi::c_void>, uncompressedbuffersize: usize, uncompresseddatasize: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn Decompress(decompressorhandle : DECOMPRESSOR_HANDLE, compresseddata : *const core::ffi::c_void, compresseddatasize : usize, uncompressedbuffer : *mut core::ffi::c_void, uncompressedbuffersize : usize, uncompresseddatasize : *mut usize) -> windows_core::BOOL);
    unsafe { Decompress(decompressorhandle, compresseddata.unwrap_or(core::mem::zeroed()) as _, compresseddatasize, uncompressedbuffer.unwrap_or(core::mem::zeroed()) as _, uncompressedbuffersize, uncompresseddatasize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut core::ffi::c_void, compressinformationsize: usize) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn QueryCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { QueryCompressorInformation(compressorhandle, compressinformationclass, compressinformation as _, compressinformationsize) }
}
#[inline]
pub unsafe fn QueryDecompressorInformation(decompressorhandle: DECOMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *mut core::ffi::c_void, compressinformationsize: usize) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn QueryDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { QueryDecompressorInformation(decompressorhandle, compressinformationclass, compressinformation as _, compressinformationsize) }
}
#[inline]
pub unsafe fn ResetCompressor(compressorhandle: COMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn ResetCompressor(compressorhandle : COMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { ResetCompressor(compressorhandle) }
}
#[inline]
pub unsafe fn ResetDecompressor(decompressorhandle: DECOMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn ResetDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { ResetDecompressor(decompressorhandle) }
}
#[inline]
pub unsafe fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const core::ffi::c_void, compressinformationsize: usize) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn SetCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { SetCompressorInformation(compressorhandle, compressinformationclass, compressinformation, compressinformationsize) }
}
#[inline]
pub unsafe fn SetDecompressorInformation(decompressorhandle: DECOMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: *const core::ffi::c_void, compressinformationsize: usize) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn SetDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> windows_core::BOOL);
    unsafe { SetDecompressorInformation(decompressorhandle, compressinformationclass, compressinformation, compressinformationsize) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct COMPRESSOR_HANDLE(pub *mut core::ffi::c_void);
impl Default for COMPRESSOR_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0;
pub const COMPRESS_ALGORITHM_LZMS: u32 = 5;
pub const COMPRESS_ALGORITHM_MAX: u32 = 6;
pub const COMPRESS_ALGORITHM_MSZIP: u32 = 2;
pub const COMPRESS_ALGORITHM_NULL: u32 = 1;
pub const COMPRESS_ALGORITHM_XPRESS: u32 = 3;
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: PFN_COMPRESS_ALLOCATE,
    pub Free: PFN_COMPRESS_FREE,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for COMPRESS_ALLOCATION_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type COMPRESS_INFORMATION_CLASS = i32;
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = 1;
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = 0;
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = 2;
pub const COMPRESS_RAW: u32 = 536870912;
pub type DECOMPRESSOR_HANDLE = COMPRESSOR_HANDLE;
pub type PCOMPRESSOR_HANDLE = *mut COMPRESSOR_HANDLE;
pub type PCOMPRESS_ALLOCATION_ROUTINES = *mut COMPRESS_ALLOCATION_ROUTINES;
pub type PDECOMPRESSOR_HANDLE = *mut COMPRESSOR_HANDLE;
pub type PFN_COMPRESS_ALLOCATE = Option<unsafe extern "C" fn(usercontext: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type PFN_COMPRESS_FREE = Option<unsafe extern "C" fn(usercontext: *const core::ffi::c_void, memory: *const core::ffi::c_void)>;
