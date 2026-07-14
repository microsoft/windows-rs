#[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt"))]
#[inline]
pub unsafe fn CreateILockBytesOnHGlobal(hglobal: Option<super::minwindef::HGLOBAL>, fdeleteonrelease: bool) -> windows_core::Result<super::objidl::ILockBytes> {
    windows_core::link!("ole32.dll" "system" fn CreateILockBytesOnHGlobal(hglobal : super::minwindef::HGLOBAL, fdeleteonrelease : windows_core::BOOL, pplkbyt : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateILockBytesOnHGlobal(hglobal.unwrap_or(core::mem::zeroed()) as _, fdeleteonrelease.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn FmtIdToPropStgName(pfmtid: *const windows_core::GUID, oszname: windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn FmtIdToPropStgName(pfmtid : *const windows_core::GUID, oszname : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { FmtIdToPropStgName(pfmtid, oszname) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn GetConvertStg<P0>(pstg: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn GetConvertStg(pstg : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetConvertStg(pstg.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "winnt"))]
#[inline]
pub unsafe fn GetHGlobalFromILockBytes<P0>(plkbyt: P0) -> windows_core::Result<super::minwindef::HGLOBAL>
where
    P0: windows_core::Param<super::objidl::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn GetHGlobalFromILockBytes(plkbyt : *mut core::ffi::c_void, phglobal : *mut super::minwindef::HGLOBAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetHGlobalFromILockBytes(plkbyt.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PropStgNameToFmtId<P0>(oszname: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ole32.dll" "system" fn PropStgNameToFmtId(oszname : windows_core::PCWSTR, pfmtid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PropStgNameToFmtId(oszname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn ReadClassStg<P0>(pstg: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::objidl::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn ReadClassStg(pstg : *mut core::ffi::c_void, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ReadClassStg(pstg.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn ReadClassStm<P0>(pstm: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn ReadClassStm(pstm : *mut core::ffi::c_void, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ReadClassStm(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgCreateDocfile(pwcsname: Option<*const u16>, grfmode: u32, reserved: Option<u32>) -> windows_core::Result<super::objidl::IStorage> {
    windows_core::link!("ole32.dll" "system" fn StgCreateDocfile(pwcsname : *const u16, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreateDocfile(pwcsname.unwrap_or(core::mem::zeroed()) as _, grfmode, reserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgCreateDocfileOnILockBytes<P0>(plkbyt: P0, grfmode: u32, reserved: u32) -> windows_core::Result<super::objidl::IStorage>
where
    P0: windows_core::Param<super::objidl::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreateDocfileOnILockBytes(plkbyt : *mut core::ffi::c_void, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreateDocfileOnILockBytes(plkbyt.param().abi(), grfmode, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "objidl", feature = "propidlbase"))]
#[inline]
pub unsafe fn StgCreatePropSetStg<P0>(pstorage: P0, dwreserved: Option<u32>) -> windows_core::Result<super::propidlbase::IPropertySetStorage>
where
    P0: windows_core::Param<super::objidl::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreatePropSetStg(pstorage : *mut core::ffi::c_void, dwreserved : u32, pppropsetstg : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreatePropSetStg(pstorage.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "propidlbase")]
#[inline]
pub unsafe fn StgCreatePropStg<P0>(punk: P0, fmtid: *const windows_core::GUID, pclsid: *const windows_core::GUID, grfflags: u32, dwreserved: Option<u32>) -> windows_core::Result<super::propidlbase::IPropertyStorage>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn StgCreatePropStg(punk : *mut core::ffi::c_void, fmtid : *const windows_core::GUID, pclsid : *const windows_core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgCreatePropStg(punk.param().abi(), fmtid, pclsid, grfflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StgCreateStorageEx<T>(pwcsname: Option<*const u16>, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: Option<*mut STGOPTIONS>, psecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn StgCreateStorageEx(pwcsname : *const u16, grfmode : u32, stgfmt : u32, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, riid : *const windows_core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { StgCreateStorageEx(pwcsname.unwrap_or(core::mem::zeroed()) as _, grfmode, stgfmt, grfattrs, pstgoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn StgIsStorageFile(pwcsname: *const u16) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgIsStorageFile(pwcsname : *const u16) -> windows_core::HRESULT);
    unsafe { StgIsStorageFile(pwcsname) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgIsStorageILockBytes<P0>(plkbyt: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::ILockBytes>,
{
    windows_core::link!("ole32.dll" "system" fn StgIsStorageILockBytes(plkbyt : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StgIsStorageILockBytes(plkbyt.param().abi()) }
}
#[cfg(feature = "propidlbase")]
#[inline]
pub unsafe fn StgOpenPropStg<P0>(punk: P0, fmtid: *const windows_core::GUID, grfflags: u32, dwreserved: Option<u32>) -> windows_core::Result<super::propidlbase::IPropertyStorage>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenPropStg(punk : *mut core::ffi::c_void, fmtid : *const windows_core::GUID, grfflags : u32, dwreserved : u32, pppropstg : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenPropStg(punk.param().abi(), fmtid, grfflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgOpenStorage<P1>(pwcsname: Option<*const u16>, pstgpriority: P1, grfmode: u32, snbexclude: Option<*const windows_core::PCWSTR>, reserved: u32) -> windows_core::Result<super::objidl::IStorage>
where
    P1: windows_core::Param<super::objidl::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorage(pwcsname : *const u16, pstgpriority : *mut core::ffi::c_void, grfmode : u32, snbexclude : *const windows_core::PCWSTR, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenStorage(pwcsname.unwrap_or(core::mem::zeroed()) as _, pstgpriority.param().abi(), grfmode, snbexclude.unwrap_or(core::mem::zeroed()) as _, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StgOpenStorageEx<T>(pwcsname: *const u16, grfmode: u32, stgfmt: u32, grfattrs: u32, pstgoptions: Option<*mut STGOPTIONS>, psecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorageEx(pwcsname : *const u16, grfmode : u32, stgfmt : u32, grfattrs : u32, pstgoptions : *mut STGOPTIONS, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, riid : *const windows_core::GUID, ppobjectopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { StgOpenStorageEx(pwcsname, grfmode, stgfmt, grfattrs, pstgoptions.unwrap_or(core::mem::zeroed()) as _, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn StgOpenStorageOnILockBytes<P0, P1>(plkbyt: P0, pstgpriority: P1, grfmode: u32, snbexclude: Option<*const windows_core::PCWSTR>, reserved: Option<u32>) -> windows_core::Result<super::objidl::IStorage>
where
    P0: windows_core::Param<super::objidl::ILockBytes>,
    P1: windows_core::Param<super::objidl::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn StgOpenStorageOnILockBytes(plkbyt : *mut core::ffi::c_void, pstgpriority : *mut core::ffi::c_void, grfmode : u32, snbexclude : *const windows_core::PCWSTR, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        StgOpenStorageOnILockBytes(plkbyt.param().abi(), pstgpriority.param().abi(), grfmode, snbexclude.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn StgSetTimes(lpszname: *const u16, pctime: Option<*const super::minwindef::FILETIME>, patime: Option<*const super::minwindef::FILETIME>, pmtime: Option<*const super::minwindef::FILETIME>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn StgSetTimes(lpszname : *const u16, pctime : *const super::minwindef::FILETIME, patime : *const super::minwindef::FILETIME, pmtime : *const super::minwindef::FILETIME) -> windows_core::HRESULT);
    unsafe { StgSetTimes(lpszname, pctime.unwrap_or(core::mem::zeroed()) as _, patime.unwrap_or(core::mem::zeroed()) as _, pmtime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn WriteClassStg<P0>(pstg: P0, rclsid: *const windows_core::GUID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IStorage>,
{
    windows_core::link!("ole32.dll" "system" fn WriteClassStg(pstg : *mut core::ffi::c_void, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { WriteClassStg(pstg.param().abi(), rclsid) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn WriteClassStm<P0>(pstm: P0, rclsid: *const windows_core::GUID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("ole32.dll" "system" fn WriteClassStm(pstm : *mut core::ffi::c_void, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { WriteClassStm(pstm.param().abi(), rclsid) }
}
pub const CCH_MAX_PROPSTG_NAME: u32 = 31;
pub const CWCSTORAGENAME: u32 = 32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct STGFMT(pub u32);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
