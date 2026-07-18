#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn BindMoniker(pmk : *mut core::ffi::c_void, grfopt : u32, iidresult : *const windows_sys::core::GUID, ppvresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : *mut core::ffi::c_void, lpvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoBuildVersion() -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : u16, ndostime : u16, lpfiletime : *mut super::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpdosdate : *mut u16, lpdostime : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoFreeAllLibraries());
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoFreeLibrary(hinst : super::HINSTANCE));
#[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetInstanceFromFile(pserverinfo : *const super::COSERVERINFO, pclsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, grfmode : u32, pwszname : *const super::OLECHAR, dwcount : u32, presults : *mut super::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "objidlbase", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetInstanceFromIStorage(pserverinfo : *const super::COSERVERINFO, pclsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, dwclsctx : u32, pstg : *mut core::ffi::c_void, dwcount : u32, presults : *mut super::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_sys::core::PCWSTR, pbindoptions : *const super::BIND_OPTS, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::PSECURITY_DESCRIPTOR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoInitialize(pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn CoInstall(pbc : *mut core::ffi::c_void, dwflags : u32, pclassspec : *const super::uCLSSPEC, pquery : *const super::QUERYCONTEXT, pszcodebase : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : windows_sys::core::PCWSTR, bautofree : windows_sys::core::BOOL) -> super::HINSTANCE);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_sys::core::GUID, pchannelhook : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : *mut core::ffi::c_void, pulicookie : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : *mut core::ffi::c_void, lplpmessagefilter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : u64) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const windows_sys::core::GUID, clsidnew : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateBindCtx(reserved : u32, ppbc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const windows_sys::core::GUID, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateDataCache(punkouter : *mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID, iid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : windows_sys::core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : *mut core::ffi::c_void, pmkrest : *mut core::ffi::c_void, ppmkcomposite : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : windows_sys::core::PCWSTR, lpszitem : windows_sys::core::PCWSTR, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : *mut core::ffi::c_void, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "urlmon", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::HWND, psztitle : windows_sys::core::PCWSTR, pibsccaller : *mut core::ffi::c_void, ppibsc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : *const core::ffi::c_void, pulreserved : *const u32, appshr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn GetClassFile(szfilename : windows_sys::core::PCWSTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : u32, pprot : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : *mut core::ffi::c_void, szusername : windows_sys::core::PCWSTR, pcheaten : *mut u32, ppmk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : *mut core::ffi::c_void, pmkother : *mut core::ffi::c_void, ppmkcommon : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : *mut core::ffi::c_void, pmkdest : *mut core::ffi::c_void, ppmkrelpath : *mut *mut core::ffi::c_void, dwreserved : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : *const super::OLECHAR, ppflb : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : *mut core::ffi::c_void, ppflb : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : *mut core::ffi::c_void, grfmode : u32, asyncflags : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : *const super::OLECHAR, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const ASYNC_MODE_COMPATIBILITY: u32 = 1;
pub const ASYNC_MODE_DEFAULT: u32 = 0;
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
pub type COMSD = i32;
pub const MARSHALINTERFACE_MIN: u32 = 500;
pub const SD_ACCESSPERMISSIONS: COMSD = 1;
pub const SD_ACCESSRESTRICTIONS: COMSD = 3;
pub const SD_LAUNCHPERMISSIONS: COMSD = 0;
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2;
pub const STGTY_REPEAT: u32 = 256;
pub const STG_LAYOUT_INTERLEAVED: u32 = 1;
pub const STG_LAYOUT_SEQUENTIAL: u32 = 0;
pub const STG_TOEND: u32 = 4294967295;
