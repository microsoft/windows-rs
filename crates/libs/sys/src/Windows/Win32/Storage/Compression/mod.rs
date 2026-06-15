windows_link::link!("cabinet.dll" "system" fn CloseCompressor(compressorhandle : COMPRESSOR_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn CloseDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn Compress(compressorhandle : COMPRESSOR_HANDLE, uncompresseddata : *const core::ffi::c_void, uncompresseddatasize : usize, compressedbuffer : *mut core::ffi::c_void, compressedbuffersize : usize, compresseddatasize : *mut usize) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn CreateCompressor(algorithm : COMPRESS_ALGORITHM, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, compressorhandle : *mut COMPRESSOR_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn CreateDecompressor(algorithm : COMPRESS_ALGORITHM, allocationroutines : *const COMPRESS_ALLOCATION_ROUTINES, decompressorhandle : *mut DECOMPRESSOR_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn Decompress(decompressorhandle : DECOMPRESSOR_HANDLE, compresseddata : *const core::ffi::c_void, compresseddatasize : usize, uncompressedbuffer : *mut core::ffi::c_void, uncompressedbuffersize : usize, uncompresseddatasize : *mut usize) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn QueryCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn QueryDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *mut core::ffi::c_void, compressinformationsize : usize) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn ResetCompressor(compressorhandle : COMPRESSOR_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn ResetDecompressor(decompressorhandle : DECOMPRESSOR_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn SetCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> windows_sys::core::BOOL);
windows_link::link!("cabinet.dll" "system" fn SetDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : *const core::ffi::c_void, compressinformationsize : usize) -> windows_sys::core::BOOL);
pub type COMPRESSOR_HANDLE = *mut core::ffi::c_void;
pub type COMPRESS_ALGORITHM = u32;
pub const COMPRESS_ALGORITHM_INVALID: u32 = 0;
pub const COMPRESS_ALGORITHM_LZMS: COMPRESS_ALGORITHM = 5;
pub const COMPRESS_ALGORITHM_MAX: u32 = 6;
pub const COMPRESS_ALGORITHM_MSZIP: COMPRESS_ALGORITHM = 2;
pub const COMPRESS_ALGORITHM_NULL: u32 = 1;
pub const COMPRESS_ALGORITHM_XPRESS: COMPRESS_ALGORITHM = 3;
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: COMPRESS_ALGORITHM = 4;
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
pub type DECOMPRESSOR_HANDLE = *mut core::ffi::c_void;
pub type PFN_COMPRESS_ALLOCATE = Option<unsafe extern "C" fn(usercontext: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void>;
pub type PFN_COMPRESS_FREE = Option<unsafe extern "C" fn(usercontext: *const core::ffi::c_void, memory: *const core::ffi::c_void)>;
