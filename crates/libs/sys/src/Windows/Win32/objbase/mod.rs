#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn BindMoniker(pmk : super::LPMONIKER, grfopt : u32, iidresult : *const windows_sys::core::GUID, ppvresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : *mut core::ffi::c_void, lpvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoBuildVersion() -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : u16, ndostime : u16, lpfiletime : *mut super::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpdosdate : super::LPWORD, lpdostime : super::LPWORD) -> windows_sys::core::BOOL);
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
#[cfg(all(feature = "objidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoInstall(pbc : *mut core::ffi::c_void, dwflags : u32, pclassspec : *const super::uCLSSPEC, pquery : *const super::QUERYCONTEXT, pszcodebase : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : super::LPOLESTR, bautofree : windows_sys::core::BOOL) -> super::HINSTANCE);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_sys::core::GUID, pchannelhook : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : *mut core::ffi::c_void, pulicookie : *mut super::ULARGE_INTEGER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : super::LPMALLOCSPY) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : super::LPMESSAGEFILTER, lplpmessagefilter : *mut super::LPMESSAGEFILTER) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : super::ULARGE_INTEGER) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const windows_sys::core::GUID, clsidnew : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateBindCtx(reserved : u32, ppbc : *mut super::LPBC) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const windows_sys::core::GUID, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut super::LPDATAADVISEHOLDER) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CreateDataCache(punkouter : *mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID, iid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : super::LPCOLESTR, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : super::LPMONIKER, pmkrest : super::LPMONIKER, ppmkcomposite : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : super::LPCOLESTR, lpszitem : super::LPCOLESTR, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : *mut core::ffi::c_void, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : *mut core::ffi::c_void, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "urlmon", feature = "windef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::HWND, psztitle : super::LPCOLESTR, pibsccaller : *mut core::ffi::c_void, ppibsc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : *const core::ffi::c_void, pulreserved : *const u32, appshr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn GetClassFile(szfilename : super::LPCOLESTR, pclsid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : u32, pprot : *mut super::LPRUNNINGOBJECTTABLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : super::LPBC, szusername : super::LPCOLESTR, pcheaten : *mut u32, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : super::LPMONIKER, pmkother : super::LPMONIKER, ppmkcommon : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : super::LPMONIKER, pmkdest : super::LPMONIKER, ppmkrelpath : *mut super::LPMONIKER, dwreserved : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : *const super::OLECHAR, ppflb : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : *mut core::ffi::c_void, ppflb : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : *mut core::ffi::c_void, grfmode : u32, asyncflags : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : *const super::OLECHAR, grfmode : u32, reserved : u32, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1;
pub const ASYNC_MODE_DEFAULT: i32 = 0;
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
pub type COMSD = i32;
pub const MARSHALINTERFACE_MIN: i32 = 500;
pub const SD_ACCESSPERMISSIONS: COMSD = 1;
pub const SD_ACCESSRESTRICTIONS: COMSD = 3;
pub const SD_LAUNCHPERMISSIONS: COMSD = 0;
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2;
pub const STGTY_REPEAT: i32 = 256;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0;
pub const STG_TOEND: u32 = 4294967295;
