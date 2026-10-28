#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut super::LPOLEADVISEHOLDER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : super::LPDATAOBJECT, pdropsource : super::LPDROPSOURCE, dwokeffects : super::DWORD, pdweffect : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
windows_link::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::HACCEL, caccelentries : i32, lpmsg : super::LPMSG, lpwcmd : *mut super::WORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("ole32.dll" "system" fn OleBuildVersion() -> super::DWORD);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : super::LPSTORAGE, lpolestream : LPOLESTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : super::LPSTORAGE, cfformat : super::CLIPFORMAT, lwidth : super::LONG, lheight : super::LONG, dwsize : super::DWORD, pmedium : super::LPSTGMEDIUM, polestm : LPOLESTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : LPOLESTREAM, pstg : super::LPSTORAGE, ptd : *const super::DVTARGETDEVICE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : LPOLESTREAM, pstg : super::LPSTORAGE, ptd : *const super::DVTARGETDEVICE, opt : super::DWORD, pvcallbackcontext : super::PVOID, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : LPOLESTREAM, pstg : super::LPSTORAGE, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut super::LONG, plheight : *mut super::LONG, pdwsize : *mut super::DWORD, pmedium : super::LPSTGMEDIUM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : LPOLESTREAM, pstg : super::LPSTORAGE, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut super::LONG, plheight : *mut super::LONG, pdwsize : *mut super::DWORD, pmedium : super::LPSTGMEDIUM, opt : super::DWORD, pvcallbackcontext : super::PVOID, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreate(rclsid : *const super::IID, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const super::IID, punkouter : super::LPUNKNOWN, riid : *const super::IID, lplpobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const super::IID, punkouter : super::LPUNKNOWN, flags : super::DWORD, pcf : super::LPCLASSFACTORY, riid : *const super::IID, lplpobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const super::IID, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : super::LPDATAOBJECT, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : super::LPDATAOBJECT, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const super::IID, lpszfilename : super::LPCOLESTR, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const super::IID, lpszfilename : super::LPCOLESTR, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : super::LPMONIKER, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : super::LPMONIKER, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : super::LPDATAOBJECT, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : super::LPDATAOBJECT, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : super::LPCOLESTR, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : super::LPCOLESTR, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::HMENU, lpmenuwidths : super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : super::LPDATAOBJECT, iid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : super::HOLEMENU) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : super::LPSTORAGE, pclsidnew : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "unknwnbase", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn OleDraw(punknown : super::LPUNKNOWN, dwaspect : super::DWORD, hdcdraw : super::HDC, lprcbounds : super::LPCRECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::HANDLE, cfformat : super::CLIPFORMAT, uiflags : super::UINT) -> super::HANDLE);
windows_link::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const super::IID, pclsidnew : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleGetClipboard(ppdataobj : *mut super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleGetClipboardWithEnterpriseInfo(dataobject : *mut *mut core::ffi::c_void, dataenterpriseid : *mut windows_sys::core::PWSTR, sourcedescription : *mut windows_sys::core::PWSTR, targetdescription : *mut windows_sys::core::PWSTR, datadescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const super::IID, lpszlabel : super::LPOLESTR, fusetypeaslabel : windows_sys::core::BOOL) -> super::HGLOBAL);
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : super::LPOLESTR, fusefileaslabel : windows_sys::core::BOOL) -> super::HGLOBAL);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn OleInitialize(pvreserved : super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn OleIsRunning(pobject : super::LPOLEOBJECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl"))]
windows_link::link!("ole32.dll" "system" fn OleLoad(pstg : super::LPSTORAGE, riid : *const super::IID, pclientsite : super::LPOLECLIENTSITE, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : super::LPSTREAM, iidinterface : *const super::IID, ppvobj : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn OleLockRunning(punknown : super::LPUNKNOWN, flock : windows_sys::core::BOOL, flastunlockcloses : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::HICON, lpszlabel : super::LPOLESTR, lpszsourcefile : super::LPOLESTR, iiconindex : super::UINT) -> super::HGLOBAL);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : super::LPUNKNOWN, fvisible : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const super::IID, dwdirection : super::DWORD, ppenum : *mut super::LPENUMFORMATETC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "oleidl"))]
windows_link::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const super::IID, ppenum : *mut super::LPENUMOLEVERB) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const super::IID, dwaspect : super::DWORD, pdwstatus : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const super::IID, dwformoftype : super::DWORD, pszusertype : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn OleRun(punknown : super::LPUNKNOWN) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleSave(pps : super::LPPERSISTSTORAGE, pstg : super::LPSTORAGE, fsameasload : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : super::LPPERSISTSTREAM, pstm : super::LPSTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const super::IID, clsidnew : *const super::IID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : super::LPUNKNOWN, fcontained : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : super::HOLEMENU, hwndframe : super::HWND, hwndactiveobject : super::HWND, lpframe : super::LPOLEINPLACEFRAME, lpactiveobj : super::LPOLEINPLACEACTIVEOBJECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef", feature = "winuser"))]
windows_link::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : super::LPOLEINPLACEFRAME, lpframeinfo : super::LPOLEINPLACEFRAMEINFO, lpmsg : super::LPMSG) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleUninitialize());
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : super::LPSTORAGE, pcf : *mut super::CLIPFORMAT, lplpszusertype : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::HWND, pdroptarget : super::LPDROPTARGET) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : super::LPSTGMEDIUM));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn SetConvertStg(pstg : super::LPSTORAGE, fconvert : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn WriteFmtUserTypeStg(pstg : super::LPSTORAGE, cf : super::CLIPFORMAT, lpszusertype : super::LPOLESTR) -> windows_sys::core::HRESULT);
pub const DATA_E_FORMATETC: windows_sys::core::HRESULT = 0x80040064_u32 as _;
pub const EMBDHLP_CREATENOW: i32 = 0;
pub const EMBDHLP_DELAYCREATE: i32 = 65536;
pub const EMBDHLP_INPROC_HANDLER: i32 = 0;
pub const EMBDHLP_INPROC_SERVER: i32 = 1;
pub const E_DRAW: windows_sys::core::HRESULT = 0x80040140_u32 as _;
#[cfg(feature = "shared")]
pub type LPOLESTREAM = *mut OLESTREAM;
#[cfg(feature = "shared")]
pub type LPOLESTREAMVTBL = *mut OLESTREAMVTBL;
pub const OLECREATE_LEAVERUNNING: i32 = 1;
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6;
pub const OLEIVERB_HIDE: i32 = -3;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5;
pub const OLEIVERB_OPEN: i32 = -2;
pub const OLEIVERB_PRIMARY: i32 = 0;
pub const OLEIVERB_SHOW: i32 = -1;
pub const OLEIVERB_UIACTIVATE: i32 = -4;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct OLESTREAM {
    pub lpstbl: LPOLESTREAMVTBL,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: super::LPCLSID, szclass: super::LPOLESTR, sztopicname: super::LPOLESTR, szitemname: super::LPOLESTR, szuncname: super::LPOLESTR, linkupdatingoption: super::ULONG, pvcontext: super::PVOID) -> windows_sys::core::HRESULT>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct OLESTREAMVTBL {
    pub Get: Option<unsafe extern "system" fn(LPOLESTREAM, *mut core::ffi::c_void, super::DWORD) -> super::DWORD>,
    pub Put: Option<unsafe extern "system" fn(LPOLESTREAM, *const core::ffi::c_void, super::DWORD) -> super::DWORD>,
}
pub const OLESTREAM_CONVERSION_DEFAULT: i32 = 0;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: i32 = 1;
