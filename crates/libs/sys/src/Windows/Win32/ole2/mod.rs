#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut super::LPOLEADVISEHOLDER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl"))]
windows_link::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : super::LPDATAOBJECT, pdropsource : super::LPDROPSOURCE, dwokeffects : u32, pdweffect : super::LPDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
windows_link::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::HACCEL, caccelentries : i32, lpmsg : super::LPMSG, lpwcmd : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn OleBuildVersion() -> u32);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : super::LPSTORAGE, lpolestream : LPOLESTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : super::LPSTORAGE, cfformat : super::CLIPFORMAT, lwidth : i32, lheight : i32, dwsize : u32, pmedium : super::LPSTGMEDIUM, polestm : LPOLESTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : LPOLESTREAM, pstg : super::LPSTORAGE, ptd : *const super::DVTARGETDEVICE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : LPOLESTREAM, pstg : super::LPSTORAGE, ptd : *const super::DVTARGETDEVICE, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : LPOLESTREAM, pstg : super::LPSTORAGE, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : super::LPSTGMEDIUM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : LPOLESTREAM, pstg : super::LPSTORAGE, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : super::LPSTGMEDIUM, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreate(rclsid : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, flags : u32, pcf : super::LPCLASSFACTORY, riid : *const windows_sys::core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : super::LPDATAOBJECT, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : super::LPDATAOBJECT, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const windows_sys::core::GUID, lpszfilename : super::LPCOLESTR, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const windows_sys::core::GUID, lpszfilename : super::LPCOLESTR, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : super::LPMONIKER, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : super::LPMONIKER, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : super::LPDATAOBJECT, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : super::LPDATAOBJECT, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : super::LPCOLESTR, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : super::LPCOLESTR, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::HMENU, lpmenuwidths : super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : super::LPDATAOBJECT, iid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : super::LPOLECLIENTSITE, pstg : super::LPSTORAGE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : super::HOLEMENU) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl"))]
windows_link::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : super::LPSTORAGE, pclsidnew : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn OleDraw(punknown : *mut core::ffi::c_void, dwaspect : u32, hdcdraw : super::HDC, lprcbounds : super::LPCRECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::HANDLE, cfformat : super::CLIPFORMAT, uiflags : u32) -> super::HANDLE);
windows_link::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const windows_sys::core::GUID, pclsidnew : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleGetClipboard(ppdataobj : *mut super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleGetClipboardWithEnterpriseInfo(dataobject : *mut *mut core::ffi::c_void, dataenterpriseid : *mut windows_sys::core::PWSTR, sourcedescription : *mut windows_sys::core::PWSTR, targetdescription : *mut windows_sys::core::PWSTR, datadescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const windows_sys::core::GUID, lpszlabel : super::LPOLESTR, fusetypeaslabel : windows_sys::core::BOOL) -> super::HGLOBAL);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : super::LPOLESTR, fusefileaslabel : windows_sys::core::BOOL) -> super::HGLOBAL);
windows_link::link!("ole32.dll" "system" fn OleInitialize(pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn OleIsRunning(pobject : super::LPOLEOBJECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidl", feature = "oleidl"))]
windows_link::link!("ole32.dll" "system" fn OleLoad(pstg : super::LPSTORAGE, riid : *const windows_sys::core::GUID, pclientsite : super::LPOLECLIENTSITE, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : super::LPSTREAM, iidinterface : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleLockRunning(punknown : *mut core::ffi::c_void, flock : windows_sys::core::BOOL, flastunlockcloses : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::HICON, lpszlabel : super::LPOLESTR, lpszsourcefile : super::LPOLESTR, iiconindex : u32) -> super::HGLOBAL);
windows_link::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : *mut core::ffi::c_void, fvisible : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const windows_sys::core::GUID, dwdirection : u32, ppenum : *mut super::LPENUMFORMATETC) -> windows_sys::core::HRESULT);
#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const windows_sys::core::GUID, ppenum : *mut super::LPENUMOLEVERB) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const windows_sys::core::GUID, dwaspect : u32, pdwstatus : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const windows_sys::core::GUID, dwformoftype : u32, pszusertype : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleRun(punknown : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleSave(pps : super::LPPERSISTSTORAGE, pstg : super::LPSTORAGE, fsameasload : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : super::LPPERSISTSTREAM, pstm : super::LPSTREAM) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const windows_sys::core::GUID, clsidnew : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : super::LPDATAOBJECT) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : *mut core::ffi::c_void, fcontained : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : super::HOLEMENU, hwndframe : super::HWND, hwndactiveobject : super::HWND, lpframe : super::LPOLEINPLACEFRAME, lpactiveobj : super::LPOLEINPLACEACTIVEOBJECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winuser"))]
windows_link::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : super::LPOLEINPLACEFRAME, lpframeinfo : super::LPOLEINPLACEFRAMEINFO, lpmsg : super::LPMSG) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleUninitialize());
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : super::LPSTORAGE, pcf : *mut super::CLIPFORMAT, lplpszusertype : *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::HWND, pdroptarget : super::LPDROPTARGET) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
pub type LPOLESTREAM = *mut OLESTREAM;
pub type LPOLESTREAMVTBL = *mut OLESTREAMVTBL;
pub const OLECREATE_LEAVERUNNING: i32 = 1;
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6;
pub const OLEIVERB_HIDE: i32 = -3;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5;
pub const OLEIVERB_OPEN: i32 = -2;
pub const OLEIVERB_PRIMARY: i32 = 0;
pub const OLEIVERB_SHOW: i32 = -1;
pub const OLEIVERB_UIACTIVATE: i32 = -4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OLESTREAM {
    pub lpstbl: LPOLESTREAMVTBL,
}
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: super::LPCLSID, szclass: super::LPOLESTR, sztopicname: super::LPOLESTR, szitemname: super::LPOLESTR, szuncname: super::LPOLESTR, linkupdatingoption: u32, pvcontext: *const core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OLESTREAMVTBL {
    pub Get: *mut u8,
    pub Put: *mut u8,
}
pub const OLESTREAM_CONVERSION_DEFAULT: i32 = 0;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: i32 = 1;
