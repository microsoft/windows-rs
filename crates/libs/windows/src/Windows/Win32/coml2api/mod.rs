#[cfg(all(feature = "minwindef", feature = "objidl"))]
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal(hglobal: Option<super::HGLOBAL>, fdeleteonrelease: bool) -> windows_core::Result<super::LPLOCKBYTES> {
    windows_core::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::HGLOBAL, fdeleteonrelease : windows_core::BOOL, pplkbyt : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateILockBytesOnHGlobal(hglobal.unwrap_or(core::mem::zeroed()) as _, fdeleteonrelease.into(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const super::FMTID, oszname: super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const super::FMTID, oszname : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { FmtIdToPropStgName(pfmtid, oszname as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn GetConvertStg<P0>(pstg: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn GetConvertStg(pstg : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetConvertStg(pstg.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "objidl"))]
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<P0>(plkbyt: P0, phglobal: *mut super::HGLOBAL) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPLOCKBYTES>,
{
    windows_core::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : *mut core::ffi::c_void, phglobal : *mut super::HGLOBAL) -> windows_core::HRESULT);
    unsafe { GetHGlobalFromILockBytes(plkbyt.param().abi(), phglobal as _) }
}
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropStgNameToFmtId(oszname: super::LPOLESTR) -> windows_core::Result<super::FMTID> {
    windows_core::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : super::LPOLESTR, pfmtid : *mut super::FMTID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropStgNameToFmtId(oszname, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "guiddef", feature = "objidl"))]
#[inline]
pub unsafe fn ReadClassStg<P0>(pstg: P0) -> windows_core::Result<super::CLSID>
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn ReadClassStg(pstg : *mut core::ffi::c_void, pclsid : *mut super::CLSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ReadClassStg(pstg.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
#[inline]
pub unsafe fn ReadClassStm<P0>(pstm: P0) -> windows_core::Result<super::CLSID>
where
    P0: windows_core::Param<super::LPSTREAM>,
{
    windows_core::link!("ole32.dll" "system" fn ReadClassStm(pstm : *mut core::ffi::c_void, pclsid : *mut super::CLSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ReadClassStm(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StgCreateDocfile(pwcsname: Option<*const super::WCHAR>, grfmode: super::DWORD, reserved: super::DWORD, ppstgopen: *mut Option<super::IStorage>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : *const super::WCHAR, grfmode : super::DWORD, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgCreateDocfile(pwcsname.unwrap_or(core::mem::zeroed()) as _, grfmode, reserved, core::mem::transmute(ppstgopen)) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<P0>(plkbyt: P0, grfmode: super::DWORD, reserved: super::DWORD) -> windows_core::Result<super::IStorage>
where
    P0: windows_core::Param<super::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : *mut core::ffi::c_void, grfmode : super::DWORD, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreateDocfileOnILockBytes(plkbyt.param().abi(), grfmode, reserved, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "objidl", feature = "propidlbase", feature = "shared"))]
#[inline]
pub unsafe fn StgCreatePropSetStg<P0>(pstorage: P0, dwreserved: super::DWORD, pppropsetstg: *mut Option<super::IPropertySetStorage>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : *mut core::ffi::c_void, dwreserved : super::DWORD, pppropsetstg : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgCreatePropSetStg(pstorage.param().abi(), dwreserved, core::mem::transmute(pppropsetstg)) }
}
#[cfg(all(feature = "guiddef", feature = "propidlbase", feature = "shared"))]
#[inline]
pub unsafe fn StgCreatePropStg<P0>(punk: P0, fmtid: *const super::IID, pclsid: *const super::CLSID, grfflags: super::DWORD, dwreserved: super::DWORD, pppropstg: *mut Option<super::IPropertyStorage>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreatePropStg(punk : *mut core::ffi::c_void, fmtid : *const super::IID, pclsid : *const super::CLSID, grfflags : super::DWORD, dwreserved : super::DWORD, pppropstg : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgCreatePropStg(punk.param().abi(), fmtid, pclsid, grfflags, dwreserved, core::mem::transmute(pppropstg)) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StgCreateStorageEx(pwcsname: Option<*const super::WCHAR>, grfmode: super::DWORD, stgfmt: super::DWORD, grfattrs: super::DWORD, pstgoptions: Option<*mut STGOPTIONS>, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, riid: *const super::IID, ppobjectopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : *const super::WCHAR, grfmode : super::DWORD, stgfmt : super::DWORD, grfattrs : super::DWORD, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, riid : *const super::IID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgCreateStorageEx(pwcsname.unwrap_or(core::mem::zeroed()) as _, grfmode, stgfmt, grfattrs, pstgoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, riid, ppobjectopen as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StgIsStorageFile(pwcsname: *const super::WCHAR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : *const super::WCHAR) -> windows_core::HRESULT);
    unsafe { StgIsStorageFile(pwcsname) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgIsStorageILockBytes<P0>(plkbyt: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgIsStorageILockBytes(plkbyt.param().abi()) }
}
#[cfg(all(feature = "guiddef", feature = "propidlbase", feature = "shared"))]
#[inline]
pub unsafe fn StgOpenPropStg<P0>(punk: P0, fmtid: *const super::IID, grfflags: super::DWORD, dwreserved: super::DWORD, pppropstg: *mut Option<super::IPropertyStorage>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenPropStg(punk : *mut core::ffi::c_void, fmtid : *const super::IID, grfflags : super::DWORD, dwreserved : super::DWORD, pppropstg : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgOpenPropStg(punk.param().abi(), fmtid, grfflags, dwreserved, core::mem::transmute(pppropstg)) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StgOpenStorage<P1>(pwcsname: Option<*const super::WCHAR>, pstgpriority: P1, grfmode: super::DWORD, snbexclude: Option<super::SNB>, reserved: super::DWORD) -> windows_core::Result<super::IStorage>
where
    P1: windows_core::Param<super::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : *const super::WCHAR, pstgpriority : *mut core::ffi::c_void, grfmode : super::DWORD, snbexclude : super::SNB, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenStorage(pwcsname.unwrap_or(core::mem::zeroed()) as _, pstgpriority.param().abi(), grfmode, snbexclude.unwrap_or(core::mem::zeroed()) as _, reserved, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StgOpenStorageEx(pwcsname: *const super::WCHAR, grfmode: super::DWORD, stgfmt: super::DWORD, grfattrs: super::DWORD, pstgoptions: Option<*mut STGOPTIONS>, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, riid: *const super::IID, ppobjectopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : *const super::WCHAR, grfmode : super::DWORD, stgfmt : super::DWORD, grfattrs : super::DWORD, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, riid : *const super::IID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgOpenStorageEx(pwcsname, grfmode, stgfmt, grfattrs, pstgoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, riid, ppobjectopen as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<P0, P1>(plkbyt: P0, pstgpriority: P1, grfmode: super::DWORD, snbexclude: Option<super::SNB>, reserved: super::DWORD, ppstgopen: *mut Option<super::IStorage>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ILockBytes>,
    P1: windows_core::Param<super::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : *mut core::ffi::c_void, pstgpriority : *mut core::ffi::c_void, grfmode : super::DWORD, snbexclude : super::SNB, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgOpenStorageOnILockBytes(plkbyt.param().abi(), pstgpriority.param().abi(), grfmode, snbexclude.unwrap_or(core::mem::zeroed()) as _, reserved, core::mem::transmute(ppstgopen)) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn StgSetTimes(lpszname: *const super::WCHAR, pctime: Option<*const super::FILETIME>, patime: Option<*const super::FILETIME>, pmtime: Option<*const super::FILETIME>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgSetTimes(lpszname : *const super::WCHAR, pctime : *const super::FILETIME, patime : *const super::FILETIME, pmtime : *const super::FILETIME) -> windows_core::HRESULT);
    unsafe { StgSetTimes(lpszname, pctime.unwrap_or(core::mem::zeroed()) as _, patime.unwrap_or(core::mem::zeroed()) as _, pmtime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl"))]
#[inline]
pub unsafe fn WriteClassStg<P0>(pstg: P0, rclsid: *const super::IID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn WriteClassStg(pstg : *mut core::ffi::c_void, rclsid : *const super::IID) -> windows_core::HRESULT);
    unsafe { WriteClassStg(pstg.param().abi(), rclsid) }
}
#[cfg(all(feature = "guiddef", feature = "objidlbase"))]
#[inline]
pub unsafe fn WriteClassStm<P0>(pstm: P0, rclsid: *const super::IID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTREAM>,
{
    windows_core::link!("ole32.dll" "system" fn WriteClassStm(pstm : *mut core::ffi::c_void, rclsid : *const super::IID) -> windows_core::HRESULT);
    unsafe { WriteClassStm(pstm.param().abi(), rclsid) }
}
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
