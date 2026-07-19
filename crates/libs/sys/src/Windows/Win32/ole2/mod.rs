#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl"))]
windows_link::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : *mut core::ffi::c_void, pdropsource : *mut core::ffi::c_void, dwokeffects : u32, pdweffect : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
windows_link::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::HACCEL, caccelentries : i32, lpmsg : *mut super::MSG, lpwcmd : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("ole32.dll" "system" fn OleBuildVersion() -> u32);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : *mut core::ffi::c_void, lpolestream : *mut OLESTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : *mut core::ffi::c_void, cfformat : super::CLIPFORMAT, lwidth : i32, lheight : i32, dwsize : u32, pmedium : *mut super::STGMEDIUM, polestm : *mut OLESTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : *mut OLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : *const OLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : *mut OLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : *mut super::STGMEDIUM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : *const OLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : *mut super::STGMEDIUM, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreate(rclsid : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "unknwnbase")]
windows_link::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const windows_sys::core::GUID, punkouter : *mut core::ffi::c_void, flags : u32, pcf : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *mut u32, rgformatetc : *mut super::FORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *mut u32, rgformatetc : *mut super::FORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const windows_sys::core::GUID, lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const windows_sys::core::GUID, lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *mut u32, rgformatetc : *mut super::FORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *mut u32, rgformatetc : *mut super::FORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *mut u32, rgformatetc : *mut super::FORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, renderopt : u32, lpformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *mut u32, rgformatetc : *mut super::FORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::HMENU, lpmenuwidths : *mut super::OLEMENUGROUPWIDTHS) -> super::HOLEMENU);
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : *mut core::ffi::c_void, iid : *const windows_sys::core::GUID, renderopt : u32, pformatetc : *mut super::FORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : super::HOLEMENU) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : *mut core::ffi::c_void, pclsidnew : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn OleDraw(punknown : *mut core::ffi::c_void, dwaspect : u32, hdcdraw : super::HDC, lprcbounds : *const super::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::HANDLE, cfformat : super::CLIPFORMAT, uiflags : u32) -> super::HANDLE);
windows_link::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const windows_sys::core::GUID, pclsidnew : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleGetClipboard(ppdataobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleGetClipboardWithEnterpriseInfo(dataobject : *mut *mut core::ffi::c_void, dataenterpriseid : *mut windows_sys::core::PWSTR, sourcedescription : *mut windows_sys::core::PWSTR, targetdescription : *mut windows_sys::core::PWSTR, datadescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const windows_sys::core::GUID, lpszlabel : windows_sys::core::PCWSTR, fusetypeaslabel : windows_sys::core::BOOL) -> super::HGLOBAL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : windows_sys::core::PCWSTR, fusefileaslabel : windows_sys::core::BOOL) -> super::HGLOBAL);
windows_link::link!("ole32.dll" "system" fn OleInitialize(pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn OleIsRunning(pobject : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "objidl", feature = "oleidl"))]
windows_link::link!("ole32.dll" "system" fn OleLoad(pstg : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, pclientsite : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : *mut core::ffi::c_void, iidinterface : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleLockRunning(punknown : *mut core::ffi::c_void, flock : windows_sys::core::BOOL, flastunlockcloses : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::HICON, lpszlabel : windows_sys::core::PCWSTR, lpszsourcefile : windows_sys::core::PCWSTR, iiconindex : u32) -> super::HGLOBAL);
windows_link::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : *mut core::ffi::c_void, fvisible : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const windows_sys::core::GUID, dwdirection : u32, ppenum : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oleidl")]
windows_link::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const windows_sys::core::GUID, ppenum : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const windows_sys::core::GUID, dwaspect : u32, pdwstatus : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const windows_sys::core::GUID, dwformoftype : u32, pszusertype : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleRun(punknown : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleSave(pps : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, fsameasload : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
windows_link::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : *mut core::ffi::c_void, pstm : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const windows_sys::core::GUID, clsidnew : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : *mut core::ffi::c_void, fcontained : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winnt"))]
windows_link::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : super::HOLEMENU, hwndframe : super::HWND, hwndactiveobject : super::HWND, lpframe : *mut core::ffi::c_void, lpactiveobj : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winuser"))]
windows_link::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : *mut core::ffi::c_void, lpframeinfo : *mut super::OLEINPLACEFRAMEINFO, lpmsg : *mut super::MSG) -> windows_sys::core::HRESULT);
windows_link::link!("ole32.dll" "system" fn OleUninitialize());
#[cfg(all(feature = "objidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : *mut core::ffi::c_void, pcf : *mut super::CLIPFORMAT, lplpszusertype : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oleidl", feature = "windef"))]
windows_link::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::HWND, pdroptarget : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : *mut super::STGMEDIUM));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("ole32.dll" "system" fn SetConvertStg(pstg : *mut core::ffi::c_void, fconvert : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "wtypes"))]
windows_link::link!("ole32.dll" "system" fn WriteFmtUserTypeStg(pstg : *mut core::ffi::c_void, cf : super::CLIPFORMAT, lpszusertype : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const DATA_E_FORMATETC: i32 = -2147221404;
pub const EMBDHLP_CREATENOW: u32 = 0;
pub const EMBDHLP_DELAYCREATE: u32 = 65536;
pub const EMBDHLP_INPROC_HANDLER: u32 = 0;
pub const EMBDHLP_INPROC_SERVER: u32 = 1;
pub const E_DRAW: i32 = -2147221184;
pub type LPOLESTREAM = *mut OLESTREAM;
pub type LPOLESTREAMVTBL = *mut OLESTREAMVTBL;
pub const OLECREATE_LEAVERUNNING: u32 = 1;
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6;
pub const OLEIVERB_HIDE: i32 = -3;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5;
pub const OLEIVERB_OPEN: i32 = -2;
pub const OLEIVERB_PRIMARY: u32 = 0;
pub const OLEIVERB_SHOW: i32 = -1;
pub const OLEIVERB_UIACTIVATE: i32 = -4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLESTREAM {
    pub lpstbl: LPOLESTREAMVTBL,
}
impl Default for OLESTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: *const windows_sys::core::GUID, szclass: windows_sys::core::PCWSTR, sztopicname: windows_sys::core::PCWSTR, szitemname: windows_sys::core::PCWSTR, szuncname: windows_sys::core::PCWSTR, linkupdatingoption: u32, pvcontext: *const core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OLESTREAMVTBL {
    pub Get: *mut u8,
    pub Put: *mut u8,
}
impl Default for OLESTREAMVTBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OLESTREAM_CONVERSION_DEFAULT: u32 = 0;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: u32 = 1;
