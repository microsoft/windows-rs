#[cfg(all(feature = "minwindef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::HGLOBAL, fdeleteonrelease : windows_sys::core::BOOL, pplkbyt : *mut super::LPLOCKBYTES) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const super::FMTID, oszname : super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn GetConvertStg(pstg : super::LPSTORAGE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : super::LPLOCKBYTES, phglobal : *mut super::HGLOBAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : super::LPOLESTR, pfmtid : *mut super::FMTID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn ReadClassStg(pstg : super::LPSTORAGE, pclsid : *mut super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn ReadClassStm(pstm : super::LPSTREAM, pclsid : *mut super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : *const super::WCHAR, grfmode : super::DWORD, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : *mut core::ffi::c_void, grfmode : super::DWORD, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "propidlbase", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : *mut core::ffi::c_void, dwreserved : super::DWORD, pppropsetstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "propidlbase", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgCreatePropStg(punk : *mut core::ffi::c_void, fmtid : *const super::IID, pclsid : *const super::CLSID, grfflags : super::DWORD, dwreserved : super::DWORD, pppropstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : *const super::WCHAR, grfmode : super::DWORD, stgfmt : super::DWORD, grfattrs : super::DWORD, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, riid : *const super::IID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : *const super::WCHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "propidlbase", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgOpenPropStg(punk : *mut core::ffi::c_void, fmtid : *const super::IID, grfflags : super::DWORD, dwreserved : super::DWORD, pppropstg : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : *const super::WCHAR, pstgpriority : *mut core::ffi::c_void, grfmode : super::DWORD, snbexclude : super::SNB, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : *const super::WCHAR, grfmode : super::DWORD, stgfmt : super::DWORD, grfattrs : super::DWORD, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, riid : *const super::IID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : *mut core::ffi::c_void, pstgpriority : *mut core::ffi::c_void, grfmode : super::DWORD, snbexclude : super::SNB, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn StgSetTimes(lpszname : *const super::WCHAR, pctime : *const super::FILETIME, patime : *const super::FILETIME, pmtime : *const super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn WriteClassStg(pstg : super::LPSTORAGE, rclsid : *const super::IID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn WriteClassStm(pstm : super::LPSTREAM, rclsid : *const super::IID) -> windows_sys::core::HRESULT);
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
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct STGOPTIONS {
    pub usVersion: super::USHORT,
    pub reserved: super::USHORT,
    pub ulSectorSize: super::ULONG,
    pub pwcsTemplateFile: *const super::WCHAR,
}
pub const STGOPTIONS_VERSION: i32 = 2;
