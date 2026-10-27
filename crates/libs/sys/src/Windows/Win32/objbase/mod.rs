#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn BindMoniker(pmk : super::LPMONIKER, grfopt : super::DWORD, iidresult : *const super::IID, ppvresult : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : *mut core::ffi::c_void, lpvreserved : super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn CoBuildVersion() -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : super::WORD, ndostime : super::WORD, lpfiletime : *mut super::FILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpdosdate : super::LPWORD, lpdostime : super::LPWORD) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn CoFreeAllLibraries());
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoFreeLibrary(hinst : super::HINSTANCE));
#[cfg(all(feature = "guiddef", feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetInstanceFromFile(pserverinfo : *const super::COSERVERINFO, pclsid : *const super::CLSID, punkouter : *mut core::ffi::c_void, dwclsctx : super::DWORD, grfmode : super::DWORD, pwszname : *const super::OLECHAR, dwcount : super::DWORD, presults : *mut super::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoGetInstanceFromIStorage(pserverinfo : *const super::COSERVERINFO, pclsid : *const super::CLSID, punkouter : *mut core::ffi::c_void, dwclsctx : super::DWORD, pstg : *mut core::ffi::c_void, dwcount : super::DWORD, presults : *mut super::MULTI_QI) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_sys::core::PCWSTR, pbindoptions : *const super::BIND_OPTS, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::PSECURITY_DESCRIPTOR) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn CoInitialize(pvreserved : super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoInstall(pbc : *mut core::ffi::c_void, dwflags : super::DWORD, pclassspec : *const super::uCLSSPEC, pquery : *const super::QUERYCONTEXT, pszcodebase : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const super::IID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : super::LPOLESTR, bautofree : windows_sys::core::BOOL) -> super::HINSTANCE);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_sys::core::GUID, pchannelhook : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : *mut core::ffi::c_void, pulicookie : *mut super::ULARGE_INTEGER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : super::LPMALLOCSPY) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : super::LPMESSAGEFILTER, lplpmessagefilter : *mut super::LPMESSAGEFILTER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : super::ULARGE_INTEGER) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const super::IID, clsidnew : *const super::IID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn CreateBindCtx(reserved : super::DWORD, ppbc : *mut super::LPBC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const super::IID, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CreateDataCache(punkouter : super::LPUNKNOWN, rclsid : *const super::IID, iid : *const super::IID, ppv : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : super::LPCOLESTR, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : super::LPMONIKER, pmkrest : super::LPMONIKER, ppmkcomposite : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : super::LPCOLESTR, lpszitem : super::LPCOLESTR, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : super::LPUNKNOWN, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : super::LPUNKNOWN, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "urlmon", feature = "windef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::HWND, psztitle : super::LPCOLESTR, pibsccaller : *mut core::ffi::c_void, ppibsc : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : super::LPVOID, pulreserved : *const super::ULONG, appshr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn GetClassFile(szfilename : super::LPCOLESTR, pclsid : *mut super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : super::DWORD, pprot : *mut super::LPRUNNINGOBJECTTABLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : super::LPBC, szusername : super::LPCOLESTR, pcheaten : *mut super::ULONG, ppmk : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : super::LPMONIKER, pmkother : super::LPMONIKER, ppmkcommon : *mut super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : super::LPMONIKER, pmkdest : super::LPMONIKER, ppmkrelpath : *mut super::LPMONIKER, dwreserved : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnFile(pwcsname : *const super::OLECHAR, ppflb : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn StgGetIFillLockBytesOnILockBytes(pilb : *mut core::ffi::c_void, ppflb : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgOpenAsyncDocfileOnIFillLockBytes(pflb : *mut core::ffi::c_void, grfmode : super::DWORD, asyncflags : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("dflayout.dll" "system" fn StgOpenLayoutDocfile(pwcsdfname : *const super::OLECHAR, grfmode : super::DWORD, reserved : super::DWORD, ppstgopen : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
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
