#[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::HGLOBAL, fdeleteonrelease : windows_sys::core::BOOL, pplkbyt : *mut super::LPLOCKBYTES) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const windows_sys::core::GUID, oszname : super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn GetConvertStg(pstg : super::LPSTORAGE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : super::LPLOCKBYTES, phglobal : *mut super::HGLOBAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : super::LPOLESTR, pfmtid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn ReadClassStg(pstg : super::LPSTORAGE, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn ReadClassStm(pstm : super::LPSTREAM, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : *const u16, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : *mut core::ffi::c_void, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "propidlbase"))]
windows_link::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : *mut core::ffi::c_void, dwreserved : u32, pppropsetstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "propidlbase")]
windows_link::link!("ole32.dll" "system" fn StgCreatePropStg(punk : *mut core::ffi::c_void, fmtid : *const windows_sys::core::GUID, pclsid : *const windows_sys::core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : *const u16, grfmode : u32, stgfmt : u32, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, riid : *const windows_sys::core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : *const u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "propidlbase")]
windows_link::link!("ole32.dll" "system" fn StgOpenPropStg(punk : *mut core::ffi::c_void, fmtid : *const windows_sys::core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : *const u16, pstgpriority : *mut core::ffi::c_void, grfmode : u32, snbexclude : super::SNB, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : *const u16, grfmode : u32, stgfmt : u32, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, riid : *const windows_sys::core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : *mut core::ffi::c_void, pstgpriority : *mut core::ffi::c_void, grfmode : u32, snbexclude : super::SNB, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn StgSetTimes(lpszname : *const u16, pctime : *const super::FILETIME, patime : *const super::FILETIME, pmtime : *const super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn WriteClassStg(pstg : super::LPSTORAGE, rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn WriteClassStm(pstm : super::LPSTREAM, rclsid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
pub const CCH_MAX_PROPSTG_NAME: i32 = 31;
pub const CWCSTORAGENAME: i32 = 32;
pub type STGFMT = u32;
pub const STGFMT_ANY: i32 = 4;
pub const STGFMT_DOCFILE: i32 = 5;
pub const STGFMT_DOCUMENT: i32 = 0;
pub const STGFMT_FILE: i32 = 3;
pub const STGFMT_NATIVE: i32 = 1;
pub const STGFMT_STORAGE: i32 = 0;
pub const STGM_CONVERT: i32 = 131072;
pub const STGM_CREATE: i32 = 4096;
pub const STGM_DELETEONRELEASE: i32 = 67108864;
pub const STGM_DIRECT: i32 = 0;
pub const STGM_DIRECT_SWMR: i32 = 4194304;
pub const STGM_FAILIFTHERE: i32 = 0;
pub const STGM_NOSCRATCH: i32 = 1048576;
pub const STGM_NOSNAPSHOT: i32 = 2097152;
pub const STGM_PRIORITY: i32 = 262144;
pub const STGM_READ: i32 = 0;
pub const STGM_READWRITE: i32 = 2;
pub const STGM_SHARE_DENY_NONE: i32 = 64;
pub const STGM_SHARE_DENY_READ: i32 = 48;
pub const STGM_SHARE_DENY_WRITE: i32 = 32;
pub const STGM_SHARE_EXCLUSIVE: i32 = 16;
pub const STGM_SIMPLE: i32 = 134217728;
pub const STGM_TRANSACTED: i32 = 65536;
pub const STGM_WRITE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STGOPTIONS {
    pub usVersion: u16,
    pub reserved: u16,
    pub ulSectorSize: u32,
    pub pwcsTemplateFile: *const u16,
}
pub const STGOPTIONS_VERSION: i32 = 2;
