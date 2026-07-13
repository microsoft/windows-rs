#[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::minwindef::HGLOBAL, fdeleteonrelease : windows_sys::core::BOOL, pplkbyt : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const windows_sys::core::GUID, oszname : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn GetConvertStg(pstg : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : *mut core::ffi::c_void, phglobal : *mut super::minwindef::HGLOBAL) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : windows_sys::core::PCWSTR, pfmtid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn ReadClassStg(pstg : *mut core::ffi::c_void, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn ReadClassStm(pstm : *mut core::ffi::c_void, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : *const u16, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : *mut core::ffi::c_void, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "propidlbase"))]
windows_link::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : *mut core::ffi::c_void, dwreserved : u32, pppropsetstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "propidlbase")]
windows_link::link!("ole32.dll" "system" fn StgCreatePropStg(punk : *mut core::ffi::c_void, fmtid : *const windows_sys::core::GUID, pclsid : *const windows_sys::core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : *const u16, grfmode : u32, stgfmt : u32, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, riid : *const windows_sys::core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : *const u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "propidlbase")]
windows_link::link!("ole32.dll" "system" fn StgOpenPropStg(punk : *mut core::ffi::c_void, fmtid : *const windows_sys::core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : *const u16, pstgpriority : *mut core::ffi::c_void, grfmode : u32, snbexclude : *const windows_sys::core::PCWSTR, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : *const u16, grfmode : u32, stgfmt : u32, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, riid : *const windows_sys::core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : *mut core::ffi::c_void, pstgpriority : *mut core::ffi::c_void, grfmode : u32, snbexclude : *const windows_sys::core::PCWSTR, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn StgSetTimes(lpszname : *const u16, pctime : *const super::minwindef::FILETIME, patime : *const super::minwindef::FILETIME, pmtime : *const super::minwindef::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn WriteClassStg(pstg : *mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn WriteClassStm(pstm : *mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
pub const CCH_MAX_PROPSTG_NAME: u32 = 31;
pub const CWCSTORAGENAME: u32 = 32;
pub type STGFMT = u32;
pub const STGFMT_ANY: u32 = 4;
pub const STGFMT_DOCFILE: u32 = 5;
pub const STGFMT_DOCUMENT: u32 = 0;
pub const STGFMT_FILE: u32 = 3;
pub const STGFMT_NATIVE: u32 = 1;
pub const STGFMT_STORAGE: u32 = 0;
pub const STGM_CONVERT: u32 = 131072;
pub const STGM_CREATE: u32 = 4096;
pub const STGM_DELETEONRELEASE: u32 = 67108864;
pub const STGM_DIRECT: u32 = 0;
pub const STGM_DIRECT_SWMR: u32 = 4194304;
pub const STGM_FAILIFTHERE: u32 = 0;
pub const STGM_NOSCRATCH: u32 = 1048576;
pub const STGM_NOSNAPSHOT: u32 = 2097152;
pub const STGM_PRIORITY: u32 = 262144;
pub const STGM_READ: u32 = 0;
pub const STGM_READWRITE: u32 = 2;
pub const STGM_SHARE_DENY_NONE: u32 = 64;
pub const STGM_SHARE_DENY_READ: u32 = 48;
pub const STGM_SHARE_DENY_WRITE: u32 = 32;
pub const STGM_SHARE_EXCLUSIVE: u32 = 16;
pub const STGM_SIMPLE: u32 = 134217728;
pub const STGM_TRANSACTED: u32 = 65536;
pub const STGM_WRITE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: *const u16,
}
impl Default for STGOPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STGOPTIONS_VERSION: u32 = 2;
