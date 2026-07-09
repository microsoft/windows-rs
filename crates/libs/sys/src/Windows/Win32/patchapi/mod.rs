windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileA(patchfilename : windows_sys::core::PCSTR, oldfilename : windows_sys::core::PCSTR, newfilename : windows_sys::core::PCSTR, applyoptionflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileByBuffers(patchfilemapped : *const u8, patchfilesize : u32, oldfilemapped : *const u8, oldfilesize : u32, newfilebuffer : *mut super::minwindef::PBYTE, newfilebuffersize : u32, newfileactualsize : *mut u32, newfiletime : *mut super::minwindef::FILETIME, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandles(patchfilehandle : super::winnt::HANDLE, oldfilehandle : super::winnt::HANDLE, newfilehandle : super::winnt::HANDLE, applyoptionflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandlesEx(patchfilehandle : super::winnt::HANDLE, oldfilehandle : super::winnt::HANDLE, newfilehandle : super::winnt::HANDLE, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileExA(patchfilename : windows_sys::core::PCSTR, oldfilename : windows_sys::core::PCSTR, newfilename : windows_sys::core::PCSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileExW(patchfilename : windows_sys::core::PCWSTR, oldfilename : windows_sys::core::PCWSTR, newfilename : windows_sys::core::PCWSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("mspatcha.dll" "system" fn ApplyPatchToFileW(patchfilename : windows_sys::core::PCWSTR, oldfilename : windows_sys::core::PCWSTR, newfilename : windows_sys::core::PCWSTR, applyoptionflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn CreatePatchFileA(oldfilename : windows_sys::core::PCSTR, newfilename : windows_sys::core::PCSTR, patchfilename : windows_sys::core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatchc.dll" "system" fn CreatePatchFileByHandles(oldfilehandle : super::winnt::HANDLE, newfilehandle : super::winnt::HANDLE, patchfilehandle : super::winnt::HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatchc.dll" "system" fn CreatePatchFileByHandlesEx(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_H, newfilehandle : super::winnt::HANDLE, patchfilehandle : super::winnt::HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn CreatePatchFileExA(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_A, newfilename : windows_sys::core::PCSTR, patchfilename : windows_sys::core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn CreatePatchFileExW(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_W, newfilename : windows_sys::core::PCWSTR, patchfilename : windows_sys::core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn CreatePatchFileW(oldfilename : windows_sys::core::PCWSTR, newfilename : windows_sys::core::PCWSTR, patchfilename : windows_sys::core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileA(patchfilename : windows_sys::core::PCSTR, patchheaderfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileByHandles(patchfilehandle : super::winnt::HANDLE, patchheaderfilehandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileW(patchfilename : windows_sys::core::PCWSTR, patchheaderfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn GetFilePatchSignatureA(filename : windows_sys::core::PCSTR, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn GetFilePatchSignatureByBuffer(filebufferwritable : *mut u8, filesize : u32, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatchc.dll" "system" fn GetFilePatchSignatureByHandle(filehandle : super::winnt::HANDLE, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn GetFilePatchSignatureW(filename : windows_sys::core::PCWSTR, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mspatchc.dll" "system" fn NormalizeFileForPatchSignature(filebuffer : *mut core::ffi::c_void, filesize : u32, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, newfilecoffbase : u32, newfilecofftime : u32, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE) -> i32);
windows_link::link!("mspatcha.dll" "system" fn TestApplyPatchToFileA(patchfilename : windows_sys::core::PCSTR, oldfilename : windows_sys::core::PCSTR, applyoptionflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByBuffers(patchfilebuffer : *const u8, patchfilesize : u32, oldfilebuffer : *const u8, oldfilesize : u32, newfilesize : *mut u32, applyoptionflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByHandles(patchfilehandle : super::winnt::HANDLE, oldfilehandle : super::winnt::HANDLE, applyoptionflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("mspatcha.dll" "system" fn TestApplyPatchToFileW(patchfilename : windows_sys::core::PCWSTR, oldfilename : windows_sys::core::PCWSTR, applyoptionflags : u32) -> windows_sys::core::BOOL);
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7;
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525;
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618;
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617;
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521;
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526;
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522;
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619;
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622;
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621;
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524;
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523;
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: windows_sys::core::PCSTR,
    pub OldFileNameW: windows_sys::core::PCWSTR,
    pub OldFileHandle: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: windows_sys::core::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
impl Default for PATCH_OLD_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::winnt::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PATCH_OLD_FILE_INFO_H {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: windows_sys::core::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
impl Default for PATCH_OLD_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: windows_sys::core::PCSTR,
    pub OldFileSymbolPathArray: *mut windows_sys::core::PCSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut core::ffi::c_void,
    pub InterleaveMapArray: *mut PPATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
impl Default for PATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288;
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824;
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536;
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152;
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072;
pub const PATCH_OPTION_NO_REBASE: u32 = 262144;
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304;
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648;
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216;
pub const PATCH_OPTION_USE_BEST: u32 = 0;
pub const PATCH_OPTION_USE_LZX_A: u32 = 1;
pub const PATCH_OPTION_USE_LZX_B: u32 = 2;
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3;
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159;
pub type PATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *mut core::ffi::c_void, currentposition: u32, maximumposition: u32) -> windows_sys::core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2;
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648;
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4;
pub type PATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: windows_sys::core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256;
pub type PPATCH_IGNORE_RANGE = *mut PATCH_IGNORE_RANGE;
pub type PPATCH_INTERLEAVE_MAP = *mut PATCH_INTERLEAVE_MAP;
#[cfg(feature = "Win32_winnt")]
pub type PPATCH_OLD_FILE_INFO = *mut PATCH_OLD_FILE_INFO;
pub type PPATCH_OLD_FILE_INFO_A = *mut PATCH_OLD_FILE_INFO_A;
#[cfg(feature = "Win32_winnt")]
pub type PPATCH_OLD_FILE_INFO_H = *mut PATCH_OLD_FILE_INFO_H;
pub type PPATCH_OLD_FILE_INFO_W = *mut PATCH_OLD_FILE_INFO_W;
pub type PPATCH_OPTION_DATA = *mut PATCH_OPTION_DATA;
pub type PPATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *mut core::ffi::c_void, currentposition: u32, maximumposition: u32) -> windows_sys::core::BOOL>;
pub type PPATCH_RETAIN_RANGE = *mut PATCH_RETAIN_RANGE;
pub type PPATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: windows_sys::core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
