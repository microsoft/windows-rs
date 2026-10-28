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
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Compress(compressorhandle: COMPRESSOR_HANDLE, uncompresseddata: Option<super::LPCVOID>, uncompresseddatasize: super::SIZE_T, compressedbuffer: Option<super::PVOID>, compressedbuffersize: super::SIZE_T, compresseddatasize: super::PSIZE_T) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn Compress(compressorhandle : COMPRESSOR_HANDLE, uncompresseddata : super::LPCVOID, uncompresseddatasize : super::SIZE_T, compressedbuffer : super::PVOID, compressedbuffersize : super::SIZE_T, compresseddatasize : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { Compress(compressorhandle, uncompresseddata.unwrap_or(core::mem::zeroed()) as _, uncompresseddatasize, compressedbuffer.unwrap_or(core::mem::zeroed()) as _, compressedbuffersize, compresseddatasize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateCompressor(algorithm: super::DWORD, allocationroutines: Option<PCOMPRESS_ALLOCATION_ROUTINES>, compressorhandle: PCOMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn CreateCompressor(algorithm : super::DWORD, allocationroutines : PCOMPRESS_ALLOCATION_ROUTINES, compressorhandle : PCOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CreateCompressor(algorithm, allocationroutines.unwrap_or(core::mem::zeroed()) as _, compressorhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDecompressor(algorithm: super::DWORD, allocationroutines: Option<PCOMPRESS_ALLOCATION_ROUTINES>, decompressorhandle: PDECOMPRESSOR_HANDLE) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn CreateDecompressor(algorithm : super::DWORD, allocationroutines : PCOMPRESS_ALLOCATION_ROUTINES, decompressorhandle : PDECOMPRESSOR_HANDLE) -> windows_core::BOOL);
    unsafe { CreateDecompressor(algorithm, allocationroutines.unwrap_or(core::mem::zeroed()) as _, decompressorhandle as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Decompress(decompressorhandle: DECOMPRESSOR_HANDLE, compresseddata: Option<super::LPCVOID>, compresseddatasize: super::SIZE_T, uncompressedbuffer: Option<super::PVOID>, uncompressedbuffersize: super::SIZE_T, uncompresseddatasize: Option<super::PSIZE_T>) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn Decompress(decompressorhandle : DECOMPRESSOR_HANDLE, compresseddata : super::LPCVOID, compresseddatasize : super::SIZE_T, uncompressedbuffer : super::PVOID, uncompressedbuffersize : super::SIZE_T, uncompresseddatasize : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { Decompress(decompressorhandle, compresseddata.unwrap_or(core::mem::zeroed()) as _, compresseddatasize, uncompressedbuffer.unwrap_or(core::mem::zeroed()) as _, uncompressedbuffersize, uncompresseddatasize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: super::PVOID, compressinformationsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn QueryCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : super::PVOID, compressinformationsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { QueryCompressorInformation(compressorhandle, compressinformationclass, compressinformation as _, compressinformationsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryDecompressorInformation(decompressorhandle: DECOMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: super::PVOID, compressinformationsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn QueryDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : super::PVOID, compressinformationsize : super::SIZE_T) -> windows_core::BOOL);
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetCompressorInformation(compressorhandle: COMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: super::LPCVOID, compressinformationsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn SetCompressorInformation(compressorhandle : COMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : super::LPCVOID, compressinformationsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { SetCompressorInformation(compressorhandle, compressinformationclass, compressinformation, compressinformationsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetDecompressorInformation(decompressorhandle: DECOMPRESSOR_HANDLE, compressinformationclass: COMPRESS_INFORMATION_CLASS, compressinformation: super::LPCVOID, compressinformationsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("cabinet.dll" "system" fn SetDecompressorInformation(decompressorhandle : DECOMPRESSOR_HANDLE, compressinformationclass : COMPRESS_INFORMATION_CLASS, compressinformation : super::LPCVOID, compressinformationsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { SetDecompressorInformation(decompressorhandle, compressinformationclass, compressinformation, compressinformationsize) }
}
pub type COMPRESSOR_HANDLE = *mut COMPRESSOR_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPRESSOR_HANDLE__ {
    pub unused: i32,
}
pub const COMPRESS_ALGORITHM_INVALID: i32 = 0;
pub const COMPRESS_ALGORITHM_LZMS: i32 = 5;
pub const COMPRESS_ALGORITHM_MAX: i32 = 6;
pub const COMPRESS_ALGORITHM_MSZIP: i32 = 2;
pub const COMPRESS_ALGORITHM_NULL: i32 = 1;
pub const COMPRESS_ALGORITHM_XPRESS: i32 = 3;
pub const COMPRESS_ALGORITHM_XPRESS_HUFF: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct COMPRESS_ALLOCATION_ROUTINES {
    pub Allocate: PFN_COMPRESS_ALLOCATE,
    pub Free: PFN_COMPRESS_FREE,
    pub UserContext: super::PVOID,
}
pub type COMPRESS_INFORMATION_CLASS = i32;
pub const COMPRESS_INFORMATION_CLASS_BLOCK_SIZE: COMPRESS_INFORMATION_CLASS = 1;
pub const COMPRESS_INFORMATION_CLASS_INVALID: COMPRESS_INFORMATION_CLASS = 0;
pub const COMPRESS_INFORMATION_CLASS_LEVEL: COMPRESS_INFORMATION_CLASS = 2;
pub const COMPRESS_RAW: i32 = 536870912;
pub type DECOMPRESSOR_HANDLE = *mut COMPRESSOR_HANDLE__;
pub type PCOMPRESSOR_HANDLE = *mut *mut COMPRESSOR_HANDLE__;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCOMPRESS_ALLOCATION_ROUTINES = *mut COMPRESS_ALLOCATION_ROUTINES;
pub type PDECOMPRESSOR_HANDLE = *mut *mut COMPRESSOR_HANDLE__;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_COMPRESS_ALLOCATE = Option<unsafe extern "C" fn(usercontext: super::PVOID, size: super::SIZE_T) -> super::PVOID>;
#[cfg(feature = "winnt")]
pub type PFN_COMPRESS_FREE = Option<unsafe extern "C" fn(usercontext: super::PVOID, memory: super::PVOID)>;
