#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BstrFromVector(psa: *const super::Com::SAFEARRAY, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ClearCustData(pcustdata: *mut super::Com::CUSTDATA);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: u32, pptinfo: *mut super::Com::ITypeInfo) -> ::windows_sys::core::HRESULT;
    pub fn CreateErrorInfo(pperrinfo: *mut ICreateErrorInfo) -> ::windows_sys::core::HRESULT;
    pub fn CreateOleAdviseHolder(ppoaholder: *mut IOleAdviseHolder) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateStdDispatch(punkouter: ::windows_sys::core::IUnknown, pvthis: *mut ::core::ffi::c_void, ptinfo: super::Com::ITypeInfo, ppunkstddisp: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateTypeLib(syskind: super::Com::SYSKIND, szfile: super::super::Foundation::PWSTR, ppctlib: *mut ICreateTypeLib) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateTypeLib2(syskind: super::Com::SYSKIND, szfile: super::super::Foundation::PWSTR, ppctlib: *mut ICreateTypeLib2) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispCallFunc(pvinstance: *const ::core::ffi::c_void, ovft: usize, cc: super::Com::CALLCONV, vtreturn: u16, cactuals: u32, prgvt: *const u16, prgpvarg: *const *const super::Com::VARIANT, pvargresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetIDsOfNames(ptinfo: super::Com::ITypeInfo, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, rgdispid: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetParam(pdispparams: *const super::Com::DISPPARAMS, position: u32, vttarg: u16, pvarresult: *mut super::Com::VARIANT, puargerr: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispInvoke(_this: *mut ::core::ffi::c_void, ptinfo: super::Com::ITypeInfo, dispidmember: i32, wflags: u16, pparams: *mut super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DoDragDrop(pdataobj: super::Com::IDataObject, pdropsource: IDropSource, dwokeffects: u32, pdweffect: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32;
    pub fn GetActiveObject(rclsid: *const ::windows_sys::core::GUID, pvreserved: *mut ::core::ffi::c_void, ppunk: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltMonthNames(lcid: u32, prgp: *mut *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn GetRecordInfoFromGuids(rguidtypelib: *const ::windows_sys::core::GUID, uvermajor: u32, uverminor: u32, lcid: u32, rguidtypeinfo: *const ::windows_sys::core::GUID, pprecinfo: *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetRecordInfoFromTypeInfo(ptypeinfo: super::Com::ITypeInfo, pprecinfo: *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn IsAccelerator(haccel: super::super::UI::WindowsAndMessaging::HACCEL, caccelentries: i32, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LHashValOfNameSys(syskind: super::Com::SYSKIND, lcid: u32, szname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LHashValOfNameSysA(syskind: super::Com::SYSKIND, lcid: u32, szname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadRegTypeLib(rguid: *const ::windows_sys::core::GUID, wvermajor: u16, wverminor: u16, lcid: u32, pptlib: *mut super::Com::ITypeLib) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LoadTypeLib(szfile: super::super::Foundation::PWSTR, pptlib: *mut super::Com::ITypeLib) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LoadTypeLibEx(szfile: super::super::Foundation::PWSTR, regkind: REGKIND, pptlib: *mut super::Com::ITypeLib) -> ::windows_sys::core::HRESULT;
    pub fn OaBuildVersion() -> u32;
    pub fn OaEnablePerUserTLibRegistration();
    pub fn OleBuildVersion() -> u32;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreate(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn OleCreateDefaultHandler(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleCreateEmbeddingHelper(clsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, flags: u32, pcf: super::Com::IClassFactory, riid: *const ::windows_sys::core::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateEx(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleCreateFontIndirect(lpfontdesc: *mut FONTDESC, riid: *const ::windows_sys::core::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromData(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromDataEx(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromFile(rclsid: *const ::windows_sys::core::GUID, lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromFileEx(rclsid: *const ::windows_sys::core::GUID, lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLink(pmklinksrc: super::Com::IMoniker, riid: *const ::windows_sys::core::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkEx(pmklinksrc: super::Com::IMoniker, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkFromData(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkFromDataEx(psrcdataobj: super::Com::IDataObject, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkToFile(lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkToFileEx(lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: super::Com::IAdviseSink, rgdwconnection: *mut u32, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleCreateMenuDescriptor(hmenucombined: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleCreatePictureIndirect(lppictdesc: *mut PICTDESC, riid: *const ::windows_sys::core::GUID, fown: super::super::Foundation::BOOL, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrame(hwndowner: super::super::Foundation::HWND, x: u32, y: u32, lpszcaption: super::super::Foundation::PWSTR, cobjects: u32, ppunk: *mut ::windows_sys::core::IUnknown, cpages: u32, ppageclsid: *mut ::windows_sys::core::GUID, lcid: u32, dwreserved: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrameIndirect(lpparams: *mut OCPFIPARAMS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateStaticFromData(psrcdataobj: super::Com::IDataObject, iid: *const ::windows_sys::core::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: IOleClientSite, pstg: super::Com::StructuredStorage::IStorage, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleDoAutoConvert(pstg: super::Com::StructuredStorage::IStorage, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleDraw(punknown: ::windows_sys::core::IUnknown, dwaspect: u32, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleDuplicateData(hsrc: super::super::Foundation::HANDLE, cfformat: u16, uiflags: u32) -> super::super::Foundation::HANDLE;
    pub fn OleFlushClipboard() -> ::windows_sys::core::HRESULT;
    pub fn OleGetAutoConvert(clsidold: *const ::windows_sys::core::GUID, pclsidnew: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboard(ppdataobj: *mut super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut super::Com::IDataObject, dataenterpriseid: *mut super::super::Foundation::PWSTR, sourcedescription: *mut super::super::Foundation::PWSTR, targetdescription: *mut super::super::Foundation::PWSTR, datadescription: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfClass(rclsid: *const ::windows_sys::core::GUID, lpszlabel: super::super::Foundation::PWSTR, fusetypeaslabel: super::super::Foundation::BOOL) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfFile(lpszpath: super::super::Foundation::PWSTR, fusefileaslabel: super::super::Foundation::BOOL) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleIconToCursor(hinstexe: super::super::Foundation::HINSTANCE, hicon: super::super::UI::WindowsAndMessaging::HICON) -> super::super::UI::WindowsAndMessaging::HCURSOR;
    pub fn OleInitialize(pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleIsCurrentClipboard(pdataobj: super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleIsRunning(pobject: IOleObject) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleLoad(pstg: super::Com::StructuredStorage::IStorage, riid: *const ::windows_sys::core::GUID, pclientsite: IOleClientSite, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadFromStream(pstm: super::Com::IStream, iidinterface: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPicture(lpstream: super::Com::IStream, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureEx(lpstream: super::Com::IStream, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows_sys::core::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFile(varfilename: super::Com::VARIANT, lplpdisppicture: *mut super::Com::IDispatch) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFileEx(varfilename: super::Com::VARIANT, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpdisppicture: *mut super::Com::IDispatch) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLoadPicturePath(szurlorpath: super::super::Foundation::PWSTR, punkcaller: ::windows_sys::core::IUnknown, dwreserved: u32, clrreserved: u32, riid: *const ::windows_sys::core::GUID, ppvret: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLockRunning(punknown: ::windows_sys::core::IUnknown, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleMetafilePictFromIconAndLabel(hicon: super::super::UI::WindowsAndMessaging::HICON, lpszlabel: super::super::Foundation::PWSTR, lpszsourcefile: super::super::Foundation::PWSTR, iiconindex: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleNoteObjectVisible(punknown: ::windows_sys::core::IUnknown, fvisible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryCreateFromData(psrcdataobject: super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryLinkFromData(psrcdataobject: super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleRegEnumFormatEtc(clsid: *const ::windows_sys::core::GUID, dwdirection: u32, ppenum: *mut super::Com::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    pub fn OleRegEnumVerbs(clsid: *const ::windows_sys::core::GUID, ppenum: *mut IEnumOLEVERB) -> ::windows_sys::core::HRESULT;
    pub fn OleRegGetMiscStatus(clsid: *const ::windows_sys::core::GUID, dwaspect: u32, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleRegGetUserType(clsid: *const ::windows_sys::core::GUID, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn OleRun(punknown: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleSave(pps: super::Com::StructuredStorage::IPersistStorage, pstg: super::Com::StructuredStorage::IStorage, fsameasload: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSavePictureFile(lpdisppicture: super::Com::IDispatch, bstrfilename: super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSaveToStream(ppstm: super::Com::IPersistStream, pstm: super::Com::IStream) -> ::windows_sys::core::HRESULT;
    pub fn OleSetAutoConvert(clsidold: *const ::windows_sys::core::GUID, clsidnew: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSetClipboard(pdataobj: super::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetContainedObject(punknown: ::windows_sys::core::IUnknown, fcontained: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetMenuDescriptor(holemenu: isize, hwndframe: super::super::Foundation::HWND, hwndactiveobject: super::super::Foundation::HWND, lpframe: IOleInPlaceFrame, lpactiveobj: IOleInPlaceActiveObject) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleTranslateAccelerator(lpframe: IOleInPlaceFrame, lpframeinfo: *mut OIFI, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleTranslateColor(clr: u32, hpal: super::super::Graphics::Gdi::HPALETTE, lpcolorref: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuA(lpoleobj: IOleObject, lpszshorttype: super::super::Foundation::PSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuW(lpoleobj: IOleObject, lpszshorttype: super::super::Foundation::PWSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyA(param0: *const OLEUIBUSYA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyW(param0: *const OLEUIBUSYW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUICanConvertOrActivateAs(rclsid: *const ::windows_sys::core::GUID, fislinkedobject: super::super::Foundation::BOOL, wformat: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconA(param0: *const OLEUICHANGEICONA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconW(param0: *const OLEUICHANGEICONW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceA(param0: *const OLEUICHANGESOURCEA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceW(param0: *const OLEUICHANGESOURCEW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertA(param0: *const OLEUICONVERTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertW(param0: *const OLEUICONVERTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksA(param0: *const OLEUIEDITLINKSA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksW(param0: *const OLEUIEDITLINKSW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectA(param0: *const OLEUIINSERTOBJECTA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectW(param0: *const OLEUIINSERTOBJECTW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesA(param0: *const OLEUIOBJECTPROPSA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesW(param0: *const OLEUIOBJECTPROPSW) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialA(param0: *const OLEUIPASTESPECIALA) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialW(param0: *const OLEUIPASTESPECIALW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserA(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserW(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksA(lpoleuilinkcntr: IOleUILinkContainerA, hwndparent: super::super::Foundation::HWND, lpsztitle: super::super::Foundation::PSTR, clinks: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksW(lpoleuilinkcntr: IOleUILinkContainerW, hwndparent: super::super::Foundation::HWND, lpsztitle: super::super::Foundation::PWSTR, clinks: i32) -> super::super::Foundation::BOOL;
    pub fn OleUninitialize();
    pub fn QueryPathOfRegTypeLib(guid: *const ::windows_sys::core::GUID, wmaj: u16, wmin: u16, lcid: u32, lpbstrpathname: *mut *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn RegisterActiveObject(punk: ::windows_sys::core::IUnknown, rclsid: *const ::windows_sys::core::GUID, dwflags: u32, pdwregister: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDragDrop(hwnd: super::super::Foundation::HWND, pdroptarget: IDropTarget) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn RegisterTypeLib(ptlib: super::Com::ITypeLib, szfullpath: super::super::Foundation::PWSTR, szhelpdir: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn RegisterTypeLibForUser(ptlib: super::Com::ITypeLib, szfullpath: super::super::Foundation::PWSTR, szhelpdir: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM);
    pub fn RevokeActiveObject(dwregister: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevokeDragDrop(hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAccessData(psa: *const super::Com::SAFEARRAY, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAddRef(psa: *const super::Com::SAFEARRAY, ppdatatorelease: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocData(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptor(cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptorEx(vt: u16, cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopy(psa: *const super::Com::SAFEARRAY, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopyData(psasource: *const super::Com::SAFEARRAY, psatarget: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreate(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateEx(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVector(vt: u16, llbound: i32, celements: u32) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVectorEx(vt: u16, llbound: i32, celements: u32, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroy(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyData(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyDescriptor(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetDim(psa: *const super::Com::SAFEARRAY) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElemsize(psa: *const super::Com::SAFEARRAY) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetIID(psa: *const super::Com::SAFEARRAY, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetLBound(psa: *const super::Com::SAFEARRAY, ndim: u32, pllbound: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: *mut IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetUBound(psa: *const super::Com::SAFEARRAY, ndim: u32, plubound: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetVartype(psa: *const super::Com::SAFEARRAY, pvt: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayLock(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPtrOfIndex(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPutElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayRedim(psa: *mut super::Com::SAFEARRAY, psaboundnew: *const super::Com::SAFEARRAYBOUND) -> ::windows_sys::core::HRESULT;
    pub fn SafeArrayReleaseData(pdata: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayReleaseDescriptor(psa: *const super::Com::SAFEARRAY);
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetIID(psa: *const super::Com::SAFEARRAY, guid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: IRecordInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnaccessData(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnlock(psa: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToVariantTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, pvtime: *mut f64) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLib(libid: *const ::windows_sys::core::GUID, wvermajor: u16, wverminor: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLibForUser(libid: *const ::windows_sys::core::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAbs(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAdd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAnd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromCy(cyin: super::Com::CY, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromDate(datein: f64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromDec(pdecin: *const super::super::Foundation::DECIMAL, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromI1(cin: super::super::Foundation::CHAR, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromI2(sin: i16, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromI4(lin: i32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromI8(i64in: i64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromR4(fltin: f32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromR8(dblin: f64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromUI1(bin: u8, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromUI2(uiin: u16, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromUI4(ulin: u32, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarBoolFromUI8(i64in: u64, pboolout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCat(bstrleft: super::super::Foundation::BSTR, bstrright: super::super::Foundation::BSTR, pbstrresult: *mut *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCmp(bstrleft: super::super::Foundation::BSTR, bstrright: super::super::Foundation::BSTR, lcid: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromBool(boolin: i16, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromCy(cyin: super::Com::CY, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDate(datein: f64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDec(pdecin: *const super::super::Foundation::DECIMAL, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromDisp(pdispin: super::Com::IDispatch, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI1(cin: super::super::Foundation::CHAR, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI2(ival: i16, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI4(lin: i32, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI8(i64in: i64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR4(fltin: f32, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR8(dblin: f64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI1(bval: u8, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI2(uiin: u16, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI4(ulin: u32, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI8(ui64in: u64, lcid: u32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCat(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCmp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, lcid: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAbs(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAdd(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmp(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmpR8(cyleft: super::Com::CY, dblright: f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFix(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromBool(boolin: i16, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDate(datein: f64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromDec(pdecin: *const super::super::Foundation::DECIMAL, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromI1(cin: super::super::Foundation::CHAR, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI2(sin: i16, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI4(lin: i32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI8(i64in: i64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR4(fltin: f32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR8(dblin: f64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI1(bin: u8, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI2(uiin: u16, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI4(ulin: u32, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI8(ui64in: u64, pcyout: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyInt(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMul(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI4(cyleft: super::Com::CY, lright: i32, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI8(cyleft: super::Com::CY, lright: i64, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyNeg(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyRound(cyin: super::Com::CY, cdecimals: i32, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCySub(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromBool(boolin: i16, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromCy(cyin: super::Com::CY, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromDec(pdecin: *const super::super::Foundation::DECIMAL, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromI1(cin: super::super::Foundation::CHAR, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromI2(sin: i16, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromI4(lin: i32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromI8(i64in: i64, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromR4(fltin: f32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromR8(dblin: f64, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromUI1(bin: u8, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromUI2(uiin: u16, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromUI4(ulin: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarDateFromUI8(ui64in: u64, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAbs(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAdd(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmp(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmpR8(pdecleft: *const super::super::Foundation::DECIMAL, dblright: f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecDiv(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFix(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromBool(boolin: i16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromCy(cyin: super::Com::CY, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromDate(datein: f64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromDisp(pdispin: super::Com::IDispatch, lcid: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI1(cin: super::super::Foundation::CHAR, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI2(uiin: i16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI4(lin: i32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI8(i64in: i64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR4(fltin: f32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR8(dblin: f64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI1(bin: u8, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI2(uiin: u16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI4(ulin: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI8(ui64in: u64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecInt(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecMul(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecNeg(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecRound(pdecin: *const super::super::Foundation::DECIMAL, cdecimals: i32, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecSub(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarEqv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFix(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormat(pvarin: *const super::Com::VARIANT, pstrformat: super::super::Foundation::PWSTR, ifirstday: i32, ifirstweek: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatCurrency(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatDateTime(pvarin: *const super::Com::VARIANT, inamedformat: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatFromTokens(pvarin: *const super::Com::VARIANT, pstrformat: super::super::Foundation::PWSTR, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR, lcid: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatNumber(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatPercent(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromBool(boolin: i16, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarI1FromCy(cyin: super::Com::CY, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDate(datein: f64, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarI1FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI2(uiin: i16, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI4(lin: i32, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI8(i64in: i64, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromR4(fltin: f32, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromR8(dblin: f64, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI1(bin: u8, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI2(uiin: u16, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI4(ulin: u32, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI8(i64in: u64, pcout: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromBool(boolin: i16, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromCy(cyin: super::Com::CY, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromDate(datein: f64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromDec(pdecin: *const super::super::Foundation::DECIMAL, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromDisp(pdispin: super::Com::IDispatch, lcid: u32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromI1(cin: super::super::Foundation::CHAR, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromI4(lin: i32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromI8(i64in: i64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromR4(fltin: f32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromR8(dblin: f64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromUI1(bin: u8, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromUI2(uiin: u16, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromUI4(ulin: u32, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI2FromUI8(ui64in: u64, psout: *mut i16) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromBool(boolin: i16, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromCy(cyin: super::Com::CY, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromDate(datein: f64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromDec(pdecin: *const super::super::Foundation::DECIMAL, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromDisp(pdispin: super::Com::IDispatch, lcid: u32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromI1(cin: super::super::Foundation::CHAR, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromI2(sin: i16, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromI8(i64in: i64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromR4(fltin: f32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromR8(dblin: f64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromUI1(bin: u8, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromUI2(uiin: u16, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromUI4(ulin: u32, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI4FromUI8(ui64in: u64, plout: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromBool(boolin: i16, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromCy(cyin: super::Com::CY, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromDate(datein: f64, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromI1(cin: super::super::Foundation::CHAR, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromI2(sin: i16, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromR4(fltin: f32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromR8(dblin: f64, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromUI1(bin: u8, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromUI2(uiin: u16, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromUI4(ulin: u32, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    pub fn VarI8FromUI8(ui64in: u64, pi64out: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarIdiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarImp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarInt(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMod(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMul(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNeg(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNot(pvarin: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32, pvar: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarOr(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarParseNumFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarPow(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    pub fn VarR4CmpR8(fltleft: f32, dblright: f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromBool(boolin: i16, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromCy(cyin: super::Com::CY, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromDate(datein: f64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromDec(pdecin: *const super::super::Foundation::DECIMAL, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromI1(cin: super::super::Foundation::CHAR, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromI2(sin: i16, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromI4(lin: i32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromI8(i64in: i64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromR8(dblin: f64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromUI1(bin: u8, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromUI2(uiin: u16, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromUI4(ulin: u32, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR4FromUI8(ui64in: u64, pfltout: *mut f32) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromBool(boolin: i16, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromCy(cyin: super::Com::CY, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromDate(datein: f64, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromI1(cin: super::super::Foundation::CHAR, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromI2(sin: i16, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromI4(lin: i32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromI8(i64in: i64, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromR4(fltin: f32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromUI1(bin: u8, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromUI2(uiin: u16, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromUI4(ulin: u32, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8FromUI8(ui64in: u64, pdblout: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8Pow(dblleft: f64, dblright: f64, pdblresult: *mut f64) -> ::windows_sys::core::HRESULT;
    pub fn VarR8Round(dblin: f64, cdecimals: i32, pdblresult: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarRound(pvarin: *const super::Com::VARIANT, cdecimals: i32, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarSub(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarTokenizeFormatString(pstrformat: super::super::Foundation::PWSTR, rgbtok: *mut u8, cbtok: i32, ifirstday: i32, ifirstweek: i32, lcid: u32, pcbactual: *const i32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromBool(boolin: i16, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromCy(cyin: super::Com::CY, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromDate(datein: f64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromI1(cin: super::super::Foundation::CHAR, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromI2(sin: i16, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromI4(lin: i32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromI8(i64in: i64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromR4(fltin: f32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromR8(dblin: f64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromUI2(uiin: u16, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromUI4(ulin: u32, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI1FromUI8(ui64in: u64, pbout: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromBool(boolin: i16, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromCy(cyin: super::Com::CY, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromDate(datein: f64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromDec(pdecin: *const super::super::Foundation::DECIMAL, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromDisp(pdispin: super::Com::IDispatch, lcid: u32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromI1(cin: super::super::Foundation::CHAR, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromI2(uiin: i16, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromI4(lin: i32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromI8(i64in: i64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromR4(fltin: f32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromUI1(bin: u8, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromUI4(ulin: u32, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI2FromUI8(i64in: u64, puiout: *mut u16) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromBool(boolin: i16, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromCy(cyin: super::Com::CY, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromDate(datein: f64, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromDec(pdecin: *const super::super::Foundation::DECIMAL, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromI1(cin: super::super::Foundation::CHAR, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromI2(uiin: i16, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromI4(lin: i32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromI8(i64in: i64, plout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromR4(fltin: f32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromR8(dblin: f64, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromUI1(bin: u8, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromUI2(uiin: u16, pulout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI4FromUI8(ui64in: u64, plout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromBool(boolin: i16, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromCy(cyin: super::Com::CY, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromDate(datein: f64, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromDisp(pdispin: super::Com::IDispatch, lcid: u32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromI1(cin: super::super::Foundation::CHAR, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromI2(sin: i16, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromI8(ui64in: i64, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromR4(fltin: f32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromR8(dblin: f64, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromUI1(bin: u8, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromUI2(uiin: u16, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn VarUI8FromUI4(ulin: u32, pi64out: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUdateFromDate(datein: f64, dwflags: u32, pudateout: *mut UDATE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32, pbstrout: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarXor(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, pvarresult: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeType(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, wflags: u16, vt: u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeTypeEx(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, wflags: u16, vt: u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantClear(pvarg: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopy(pvargdest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopyInd(pvardest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantInit(pvarg: *mut super::Com::VARIANT);
    pub fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VectorFromBstr(bstr: super::super::Foundation::BSTR, ppsa: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct ACTIVATEFLAGS(pub i32);
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = ACTIVATEFLAGS(1i32);
pub const ACTIVEOBJECT_STRONG: u32 = 0u32;
pub const ACTIVEOBJECT_WEAK: u32 = 1u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ARRAYDESC(i32);
#[repr(C)]
pub struct AspectInfo(i32);
#[repr(transparent)]
pub struct AspectInfoFlag(pub i32);
pub const DVASPECTINFOFLAG_CANOPTIMIZE: AspectInfoFlag = AspectInfoFlag(1i32);
#[repr(transparent)]
pub struct BINDSPEED(pub i32);
pub const BINDSPEED_INDEFINITE: BINDSPEED = BINDSPEED(1i32);
pub const BINDSPEED_MODERATE: BINDSPEED = BINDSPEED(2i32);
pub const BINDSPEED_IMMEDIATE: BINDSPEED = BINDSPEED(3i32);
pub const BZ_DISABLECANCELBUTTON: i32 = 1i32;
pub const BZ_DISABLERETRYBUTTON: i32 = 4i32;
pub const BZ_DISABLESWITCHTOBUTTON: i32 = 2i32;
pub const BZ_NOTRESPONDINGDIALOG: i32 = 8i32;
#[repr(C)]
pub struct CADWORD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CALPOLESTR(i32);
#[repr(C)]
pub struct CAUUID(i32);
pub const CF_CONVERTONLY: i32 = 256i32;
pub const CF_DISABLEACTIVATEAS: i32 = 64i32;
pub const CF_DISABLEDISPLAYASICON: i32 = 32i32;
pub const CF_HIDECHANGEICON: i32 = 128i32;
pub const CF_SELECTACTIVATEAS: i32 = 16i32;
pub const CF_SELECTCONVERTTO: i32 = 8i32;
pub const CF_SETACTIVATEDEFAULT: i32 = 4i32;
pub const CF_SETCONVERTDEFAULT: i32 = 2i32;
pub const CF_SHOWHELPBUTTON: i32 = 1i32;
#[repr(transparent)]
pub struct CHANGEKIND(pub i32);
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = CHANGEKIND(0i32);
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = CHANGEKIND(1i32);
pub const CHANGEKIND_SETNAMES: CHANGEKIND = CHANGEKIND(2i32);
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = CHANGEKIND(3i32);
pub const CHANGEKIND_GENERAL: CHANGEKIND = CHANGEKIND(4i32);
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = CHANGEKIND(5i32);
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = CHANGEKIND(6i32);
pub const CHANGEKIND_MAX: CHANGEKIND = CHANGEKIND(7i32);
pub const CIF_SELECTCURRENT: i32 = 2i32;
pub const CIF_SELECTDEFAULT: i32 = 4i32;
pub const CIF_SELECTFROMFILE: i32 = 8i32;
pub const CIF_SHOWHELP: i32 = 1i32;
pub const CIF_USEICONEXE: i32 = 16i32;
#[repr(C)]
pub struct CLEANLOCALSTORAGE(i32);
pub const CLSID_CColorPropPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 199447041, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_CFontPropPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 199447040, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_CPicturePropPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 199447042, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_ConvertVBX: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4220454946, data2: 356, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
pub const CLSID_PersistPropset: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4220454945, data2: 356, data3: 4123, data4: [132, 237, 8, 0, 43, 46, 199, 19] };
pub const CLSID_StdFont: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 199447043, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CLSID_StdPicture: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 199447044, data2: 36753, data3: 4558, data4: [157, 227, 0, 170, 0, 75, 184, 81] };
pub const CONNECT_E_ADVISELIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const CONNECT_E_CANNOTCONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
pub const CONNECT_E_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
pub const CONNECT_E_NOCONNECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub const CONNECT_E_OVERRIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220989i32 as _);
pub const CONNECT_S_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262656i32 as _);
pub const CONNECT_S_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262671i32 as _);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct CONTROLINFO(i32);
pub const CSF_EXPLORER: i32 = 8i32;
pub const CSF_ONLYGETSOURCE: i32 = 4i32;
pub const CSF_SHOWHELP: i32 = 1i32;
pub const CSF_VALIDSOURCE: i32 = 2i32;
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
#[repr(transparent)]
pub struct CTRLINFO(pub i32);
pub const CTRLINFO_EATS_RETURN: CTRLINFO = CTRLINFO(1i32);
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = CTRLINFO(2i32);
pub const DD_DEFDRAGDELAY: u32 = 200u32;
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
pub const DD_DEFSCROLLINSET: u32 = 11u32;
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
#[repr(transparent)]
pub struct DISCARDCACHE(pub i32);
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = DISCARDCACHE(0i32);
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = DISCARDCACHE(1i32);
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
pub const DISPATCH_METHOD: u32 = 1u32;
pub const DISPATCH_PROPERTYGET: u32 = 2u32;
pub const DISPATCH_PROPERTYPUT: u32 = 4u32;
pub const DISPATCH_PROPERTYPUTREF: u32 = 8u32;
pub const DISPID_ABOUTBOX: i32 = -552i32;
pub const DISPID_ACCELERATOR: i32 = -543i32;
pub const DISPID_ADDITEM: i32 = -553i32;
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
pub const DISPID_APPEARANCE: i32 = -520i32;
pub const DISPID_AUTOSIZE: i32 = -500i32;
pub const DISPID_BACKCOLOR: i32 = -501i32;
pub const DISPID_BACKSTYLE: i32 = -502i32;
pub const DISPID_BORDERCOLOR: i32 = -503i32;
pub const DISPID_BORDERSTYLE: i32 = -504i32;
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
pub const DISPID_BORDERWIDTH: i32 = -505i32;
pub const DISPID_CAPTION: i32 = -518i32;
pub const DISPID_CLEAR: i32 = -554i32;
pub const DISPID_CLICK: i32 = -600i32;
pub const DISPID_CLICK_VALUE: i32 = -610i32;
pub const DISPID_COLLECT: i32 = -8i32;
pub const DISPID_COLUMN: i32 = -529i32;
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
pub const DISPID_DBLCLICK: i32 = -601i32;
pub const DISPID_DESTRUCTOR: i32 = -7i32;
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
pub const DISPID_DOCLICK: i32 = -551i32;
pub const DISPID_DRAWMODE: i32 = -507i32;
pub const DISPID_DRAWSTYLE: i32 = -508i32;
pub const DISPID_DRAWWIDTH: i32 = -509i32;
pub const DISPID_Delete: i32 = -801i32;
pub const DISPID_ENABLED: i32 = -514i32;
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
pub const DISPID_ERROREVENT: i32 = -608i32;
pub const DISPID_EVALUATE: i32 = -5i32;
pub const DISPID_FILLCOLOR: i32 = -510i32;
pub const DISPID_FILLSTYLE: i32 = -511i32;
pub const DISPID_FONT: i32 = -512i32;
pub const DISPID_FONT_BOLD: u32 = 3u32;
pub const DISPID_FONT_CHANGED: u32 = 9u32;
pub const DISPID_FONT_CHARSET: u32 = 8u32;
pub const DISPID_FONT_ITALIC: u32 = 4u32;
pub const DISPID_FONT_NAME: u32 = 0u32;
pub const DISPID_FONT_SIZE: u32 = 2u32;
pub const DISPID_FONT_STRIKE: u32 = 6u32;
pub const DISPID_FONT_UNDER: u32 = 5u32;
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
pub const DISPID_FORECOLOR: i32 = -513i32;
pub const DISPID_GROUPNAME: i32 = -541i32;
pub const DISPID_HWND: i32 = -515i32;
pub const DISPID_IMEMODE: i32 = -542i32;
pub const DISPID_KEYDOWN: i32 = -602i32;
pub const DISPID_KEYPRESS: i32 = -603i32;
pub const DISPID_KEYUP: i32 = -604i32;
pub const DISPID_LIST: i32 = -528i32;
pub const DISPID_LISTCOUNT: i32 = -531i32;
pub const DISPID_LISTINDEX: i32 = -526i32;
pub const DISPID_MAXLENGTH: i32 = -533i32;
pub const DISPID_MOUSEDOWN: i32 = -605i32;
pub const DISPID_MOUSEICON: i32 = -522i32;
pub const DISPID_MOUSEMOVE: i32 = -606i32;
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
pub const DISPID_MOUSEUP: i32 = -607i32;
pub const DISPID_MULTILINE: i32 = -537i32;
pub const DISPID_MULTISELECT: i32 = -532i32;
pub const DISPID_NEWENUM: i32 = -4i32;
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
pub const DISPID_Name: i32 = -800i32;
pub const DISPID_Object: i32 = -802i32;
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
pub const DISPID_PICTURE: i32 = -523i32;
pub const DISPID_PICT_HANDLE: u32 = 0u32;
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
pub const DISPID_PICT_HPAL: u32 = 2u32;
pub const DISPID_PICT_RENDER: u32 = 6u32;
pub const DISPID_PICT_TYPE: u32 = 3u32;
pub const DISPID_PICT_WIDTH: u32 = 4u32;
pub const DISPID_PROPERTYPUT: i32 = -3i32;
pub const DISPID_Parent: i32 = -803i32;
pub const DISPID_READYSTATE: i32 = -525i32;
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
pub const DISPID_REFRESH: i32 = -550i32;
pub const DISPID_REMOVEITEM: i32 = -555i32;
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
pub const DISPID_SCROLLBARS: i32 = -535i32;
pub const DISPID_SELECTED: i32 = -527i32;
pub const DISPID_SELLENGTH: i32 = -548i32;
pub const DISPID_SELSTART: i32 = -547i32;
pub const DISPID_SELTEXT: i32 = -546i32;
pub const DISPID_STARTENUM: i32 = -1i32;
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
pub const DISPID_TABSTOP: i32 = -516i32;
pub const DISPID_TEXT: i32 = -517i32;
pub const DISPID_THIS: i32 = -613i32;
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
pub const DISPID_UNKNOWN: i32 = -1i32;
pub const DISPID_VALID: i32 = -524i32;
pub const DISPID_VALUE: u32 = 0u32;
pub const DISPID_WORDWRAP: i32 = -536i32;
#[repr(transparent)]
pub struct DOCMISC(pub i32);
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = DOCMISC(1i32);
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = DOCMISC(2i32);
pub const DOCMISC_CANTOPENEDIT: DOCMISC = DOCMISC(4i32);
pub const DOCMISC_NOFILESUPPORT: DOCMISC = DOCMISC(8i32);
pub const DROPEFFECT_COPY: u32 = 1u32;
pub const DROPEFFECT_LINK: u32 = 4u32;
pub const DROPEFFECT_MOVE: u32 = 2u32;
pub const DROPEFFECT_NONE: u32 = 0u32;
pub const DROPEFFECT_SCROLL: u32 = 2147483648u32;
#[repr(transparent)]
pub struct DVASPECT2(pub i32);
pub const DVASPECT_OPAQUE: DVASPECT2 = DVASPECT2(16i32);
pub const DVASPECT_TRANSPARENT: DVASPECT2 = DVASPECT2(32i32);
pub const ELF_DISABLECANCELLINK: i32 = 16i32;
pub const ELF_DISABLECHANGESOURCE: i32 = 8i32;
pub const ELF_DISABLEOPENSOURCE: i32 = 4i32;
pub const ELF_DISABLEUPDATENOW: i32 = 2i32;
pub const ELF_SHOWHELP: i32 = 1i32;
pub const EMBDHLP_CREATENOW: i32 = 0i32;
pub const EMBDHLP_DELAYCREATE: i32 = 65536i32;
pub const EMBDHLP_INPROC_HANDLER: i32 = 0i32;
pub const EMBDHLP_INPROC_SERVER: i32 = 1i32;
#[repr(transparent)]
pub struct ENUM_CONTROLS_WHICH_FLAGS(pub u32);
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1u32);
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(2u32);
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(3u32);
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(4u32);
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(134217728u32);
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(268435456u32);
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(536870912u32);
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1073741824u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ExtentInfo(i32);
#[repr(transparent)]
pub struct ExtentMode(pub i32);
pub const DVEXTENT_CONTENT: ExtentMode = ExtentMode(0i32);
pub const DVEXTENT_INTEGRAL: ExtentMode = ExtentMode(1i32);
pub const FADF_AUTO: u32 = 1u32;
pub const FADF_BSTR: u32 = 256u32;
pub const FADF_DISPATCH: u32 = 1024u32;
pub const FADF_EMBEDDED: u32 = 4u32;
pub const FADF_FIXEDSIZE: u32 = 16u32;
pub const FADF_HAVEIID: u32 = 64u32;
pub const FADF_HAVEVARTYPE: u32 = 128u32;
pub const FADF_RECORD: u32 = 32u32;
pub const FADF_RESERVED: u32 = 61448u32;
pub const FADF_STATIC: u32 = 2u32;
pub const FADF_UNKNOWN: u32 = 512u32;
pub const FADF_VARIANT: u32 = 2048u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct FONTDESC(i32);
#[repr(transparent)]
pub struct FUNCFLAGS(pub i32);
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = FUNCFLAGS(1i32);
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = FUNCFLAGS(2i32);
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = FUNCFLAGS(4i32);
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = FUNCFLAGS(8i32);
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = FUNCFLAGS(16i32);
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = FUNCFLAGS(32i32);
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = FUNCFLAGS(64i32);
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = FUNCFLAGS(128i32);
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = FUNCFLAGS(256i32);
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = FUNCFLAGS(512i32);
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = FUNCFLAGS(1024i32);
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = FUNCFLAGS(2048i32);
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = FUNCFLAGS(4096i32);
pub const GC_WCH_SIBLING: i32 = 1i32;
#[repr(transparent)]
pub struct GUIDKIND(pub i32);
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = GUIDKIND(1i32);
pub const GUID_CHECKVALUEEXCLUSIVE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536076, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_COLOR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536065, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTBOLD: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536079, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTITALIC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536080, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTNAME: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536077, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTSIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536078, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTSTRIKETHROUGH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536082, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_FONTUNDERSCORE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536081, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_HANDLE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536083, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_HIMETRIC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536064, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_OPTIONVALUEEXCLUSIVE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536075, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_TRISTATE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536074, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XPOS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536070, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XPOSPIXEL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536066, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XSIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536072, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_XSIZEPIXEL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536068, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YPOS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536071, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YPOSPIXEL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536067, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YSIZE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536073, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
pub const GUID_YSIZEPIXEL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1716536069, data2: 48655, data3: 4122, data4: [139, 187, 0, 170, 0, 48, 12, 171] };
#[repr(transparent)]
pub struct HITRESULT(pub i32);
pub const HITRESULT_OUTSIDE: HITRESULT = HITRESULT(0i32);
pub const HITRESULT_TRANSPARENT: HITRESULT = HITRESULT(1i32);
pub const HITRESULT_CLOSE: HITRESULT = HITRESULT(2i32);
pub const HITRESULT_HIT: HITRESULT = HITRESULT(3i32);
#[repr(transparent)]
pub struct IAdviseSinkEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanHandleException(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClassFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContinue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContinueCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateTypeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateTypeInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateTypeLib(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateTypeLib2(pub *mut ::core::ffi::c_void);
pub const IDC_BZ_ICON: u32 = 601u32;
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
pub const IDC_BZ_RETRY: u32 = 600u32;
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
pub const IDC_CI_BROWSE: u32 = 130u32;
pub const IDC_CI_CURRENT: u32 = 121u32;
pub const IDC_CI_CURRENTICON: u32 = 122u32;
pub const IDC_CI_DEFAULT: u32 = 123u32;
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
pub const IDC_CI_FROMFILE: u32 = 125u32;
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
pub const IDC_CI_GROUP: u32 = 120u32;
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
pub const IDC_CI_ICONLIST: u32 = 127u32;
pub const IDC_CI_LABEL: u32 = 128u32;
pub const IDC_CI_LABELEDIT: u32 = 129u32;
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
pub const IDC_CV_CHANGEICON: u32 = 153u32;
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
pub const IDC_CV_CONVERTTO: u32 = 155u32;
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
pub const IDC_EL_CANCELLINK: u32 = 209u32;
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
pub const IDC_EL_COL1: u32 = 220u32;
pub const IDC_EL_COL2: u32 = 221u32;
pub const IDC_EL_COL3: u32 = 222u32;
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
pub const IDC_EL_LINKTYPE: u32 = 217u32;
pub const IDC_EL_MANUAL: u32 = 212u32;
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
pub const IDC_EL_UPDATENOW: u32 = 210u32;
pub const IDC_GP_CONVERT: u32 = 1013u32;
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
pub const IDC_IO_CREATENEW: u32 = 2100u32;
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
pub const IDC_IO_FILE: u32 = 2106u32;
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
pub const IDC_IO_FILETEXT: u32 = 2112u32;
pub const IDC_IO_FILETYPE: u32 = 2113u32;
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
pub const IDC_IO_LINKFILE: u32 = 2102u32;
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
pub const IDC_LP_DATE: u32 = 1018u32;
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
pub const IDC_LP_MANUAL: u32 = 1017u32;
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
pub const IDC_LP_TIME: u32 = 1019u32;
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
pub const IDC_OLEUIHELP: u32 = 99u32;
pub const IDC_PS_CHANGEICON: u32 = 508u32;
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
pub const IDC_PS_PASTE: u32 = 500u32;
pub const IDC_PS_PASTELINK: u32 = 501u32;
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
pub const IDC_PS_PASTELIST: u32 = 503u32;
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
pub const IDC_PU_CONVERT: u32 = 902u32;
pub const IDC_PU_ICON: u32 = 908u32;
pub const IDC_PU_LINKS: u32 = 900u32;
pub const IDC_PU_TEXT: u32 = 901u32;
pub const IDC_UL_METER: u32 = 1029u32;
pub const IDC_UL_PERCENT: u32 = 1031u32;
pub const IDC_UL_PROGRESS: u32 = 1032u32;
pub const IDC_UL_STOP: u32 = 1030u32;
pub const IDC_VP_ASICON: u32 = 1003u32;
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
pub const IDC_VP_EDITABLE: u32 = 1002u32;
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
pub const IDC_VP_PERCENT: u32 = 1000u32;
pub const IDC_VP_RELATIVE: u32 = 1005u32;
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
pub const IDC_VP_SCALETXT: u32 = 1034u32;
pub const IDC_VP_SPIN: u32 = 1006u32;
pub const IDD_BUSY: u32 = 1006u32;
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
pub const IDD_CHANGEICON: u32 = 1001u32;
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
pub const IDD_CHANGESOURCE: u32 = 1009u32;
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
pub const IDD_CONVERT: u32 = 1002u32;
pub const IDD_CONVERT4: u32 = 1103u32;
pub const IDD_CONVERTONLY: u32 = 1012u32;
pub const IDD_CONVERTONLY4: u32 = 1104u32;
pub const IDD_EDITLINKS: u32 = 1004u32;
pub const IDD_EDITLINKS4: u32 = 1105u32;
pub const IDD_GNRLPROPS: u32 = 1100u32;
pub const IDD_GNRLPROPS4: u32 = 1106u32;
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
pub const IDD_INSERTOBJECT: u32 = 1000u32;
pub const IDD_LINKPROPS: u32 = 1102u32;
pub const IDD_LINKPROPS4: u32 = 1107u32;
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
pub const IDD_PASTESPECIAL: u32 = 1003u32;
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
pub const IDD_SERVERNOTREG: u32 = 1021u32;
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
pub const IDD_UPDATELINKS: u32 = 1007u32;
pub const IDD_VIEWPROPS: u32 = 1101u32;
pub const IDLFLAG_FIN: u32 = 1u32;
pub const IDLFLAG_FLCID: u32 = 4u32;
pub const IDLFLAG_FOUT: u32 = 2u32;
pub const IDLFLAG_FRETVAL: u32 = 8u32;
pub const IDLFLAG_NONE: u32 = 0u32;
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
pub const ID_DEFAULTINST: i32 = -2i32;
#[repr(transparent)]
pub struct IDispError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDispatchEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropSourceNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnterpriseDropTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumOLEVERB(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumOleDocumentViews(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumOleUndoUnits(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumVARIANT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFont(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontDisp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontEventsDisp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGNOREMIME(pub i32);
pub const IGNOREMIME_PROMPT: IGNOREMIME = IGNOREMIME(1i32);
pub const IGNOREMIME_TEXT: IGNOREMIME = IGNOREMIME(2i32);
#[repr(transparent)]
pub struct IGetOleObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetVBAObject(pub *mut ::core::ffi::c_void);
pub const IMPLTYPEFLAG_FDEFAULT: u32 = 1u32;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: u32 = 8u32;
pub const IMPLTYPEFLAG_FRESTRICTED: u32 = 4u32;
pub const IMPLTYPEFLAG_FSOURCE: u32 = 2u32;
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
pub const INSTALL_SCOPE_USER: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct INTERFACEDATA(i32);
pub const IOF_CHECKDISPLAYASICON: i32 = 16i32;
pub const IOF_CHECKLINK: i32 = 8i32;
pub const IOF_CREATEFILEOBJECT: i32 = 64i32;
pub const IOF_CREATELINKOBJECT: i32 = 128i32;
pub const IOF_CREATENEWOBJECT: i32 = 32i32;
pub const IOF_DISABLEDISPLAYASICON: i32 = 1024i32;
pub const IOF_DISABLELINK: i32 = 256i32;
pub const IOF_HIDECHANGEICON: i32 = 2048i32;
pub const IOF_SELECTCREATECONTROL: i32 = 8192i32;
pub const IOF_SELECTCREATEFROMFILE: i32 = 4i32;
pub const IOF_SELECTCREATENEW: i32 = 2i32;
pub const IOF_SHOWHELP: i32 = 1i32;
pub const IOF_SHOWINSERTCONTROL: i32 = 4096i32;
pub const IOF_VERIFYSERVERSEXIST: i32 = 512i32;
#[repr(transparent)]
pub struct IObjectIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleAdviseHolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleCache2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleCacheControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleClientSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleCommandTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleControlSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleDocumentSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleDocumentView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceActiveObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceObjectWindowless(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceSiteEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceSiteWindowless(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleInPlaceUIWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleItemContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleParentUndoUnit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUILinkContainerA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUILinkContainerW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUILinkInfoA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUILinkInfoW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUIObjInfoA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUIObjInfoW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUndoManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleUndoUnit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOleWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParseDisplayName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerPropertyBrowsing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistPropertyBag(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistPropertyBag2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPicture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPicture2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPictureDisp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerInactive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyNotifySink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyPage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyPageSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectFocus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProtectedModeMenuServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideClassInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideClassInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideMultipleClassInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideRuntimeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQuickActivate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRecordInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleFrameSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpecifyPropertyPages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITypeChangeEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITypeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITypeMarshal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVBFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVBGetControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVariantChangeType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObject2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewObjectEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IZoomEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LIBFLAGS(pub i32);
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = LIBFLAGS(1i32);
pub const LIBFLAG_FCONTROL: LIBFLAGS = LIBFLAGS(2i32);
pub const LIBFLAG_FHIDDEN: LIBFLAGS = LIBFLAGS(4i32);
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = LIBFLAGS(8i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LICINFO(i32);
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
pub const LOCALE_USE_NLS: u32 = 268435456u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNOLEUIHOOK = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> u32;
pub const LP_COLOR: u32 = 4u32;
pub const LP_DEFAULT: u32 = 0u32;
pub const LP_MONOCHROME: u32 = 1u32;
pub const LP_VGACOLOR: u32 = 2u32;
#[repr(transparent)]
pub struct MEDIAPLAYBACK_STATE(pub i32);
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(0i32);
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(1i32);
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(2i32);
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(3i32);
pub const MEMBERID_NIL: i32 = -1i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct METHODDATA(i32);
pub const MK_ALT: u32 = 32u32;
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
#[repr(transparent)]
pub struct MULTICLASSINFO_FLAGS(pub u32);
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(1u32);
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(2u32);
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(4u32);
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(8u32);
#[repr(C)]
pub struct NUMPARSE(i32);
pub const NUMPRS_CURRENCY: u32 = 1024u32;
pub const NUMPRS_DECIMAL: u32 = 256u32;
pub const NUMPRS_EXPONENT: u32 = 2048u32;
pub const NUMPRS_HEX_OCT: u32 = 64u32;
pub const NUMPRS_INEXACT: u32 = 131072u32;
pub const NUMPRS_LEADING_MINUS: u32 = 16u32;
pub const NUMPRS_LEADING_PLUS: u32 = 4u32;
pub const NUMPRS_LEADING_WHITE: u32 = 1u32;
pub const NUMPRS_NEG: u32 = 65536u32;
pub const NUMPRS_PARENS: u32 = 128u32;
pub const NUMPRS_STD: u32 = 8191u32;
pub const NUMPRS_THOUSANDS: u32 = 512u32;
pub const NUMPRS_TRAILING_MINUS: u32 = 32u32;
pub const NUMPRS_TRAILING_PLUS: u32 = 8u32;
pub const NUMPRS_TRAILING_WHITE: u32 = 2u32;
pub const NUMPRS_USE_ALL: u32 = 4096u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OBJECTDESCRIPTOR(i32);
pub const OCM__BASE: u32 = 8192u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OCPFIPARAMS(i32);
pub const OF_GET: u32 = 2u32;
pub const OF_HANDLER: u32 = 4u32;
pub const OF_SET: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OIFI(i32);
#[repr(transparent)]
pub struct OLECLOSE(pub i32);
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = OLECLOSE(0i32);
pub const OLECLOSE_NOSAVE: OLECLOSE = OLECLOSE(1i32);
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = OLECLOSE(2i32);
#[repr(C)]
pub struct OLECMD(i32);
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
pub const OLECMDERR_E_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221245i32 as _);
pub const OLECMDERR_E_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221247i32 as _);
pub const OLECMDERR_E_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221248i32 as _);
pub const OLECMDERR_E_NOHELP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221246i32 as _);
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const OLECMDERR_E_UNKNOWNGROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221244i32 as _);
#[repr(transparent)]
pub struct OLECMDEXECOPT(pub i32);
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = OLECMDEXECOPT(0i32);
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(1i32);
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(2i32);
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = OLECMDEXECOPT(3i32);
#[repr(transparent)]
pub struct OLECMDF(pub i32);
pub const OLECMDF_SUPPORTED: OLECMDF = OLECMDF(1i32);
pub const OLECMDF_ENABLED: OLECMDF = OLECMDF(2i32);
pub const OLECMDF_LATCHED: OLECMDF = OLECMDF(4i32);
pub const OLECMDF_NINCHED: OLECMDF = OLECMDF(8i32);
pub const OLECMDF_INVISIBLE: OLECMDF = OLECMDF(16i32);
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = OLECMDF(32i32);
#[repr(transparent)]
pub struct OLECMDID(pub i32);
pub const OLECMDID_OPEN: OLECMDID = OLECMDID(1i32);
pub const OLECMDID_NEW: OLECMDID = OLECMDID(2i32);
pub const OLECMDID_SAVE: OLECMDID = OLECMDID(3i32);
pub const OLECMDID_SAVEAS: OLECMDID = OLECMDID(4i32);
pub const OLECMDID_SAVECOPYAS: OLECMDID = OLECMDID(5i32);
pub const OLECMDID_PRINT: OLECMDID = OLECMDID(6i32);
pub const OLECMDID_PRINTPREVIEW: OLECMDID = OLECMDID(7i32);
pub const OLECMDID_PAGESETUP: OLECMDID = OLECMDID(8i32);
pub const OLECMDID_SPELL: OLECMDID = OLECMDID(9i32);
pub const OLECMDID_PROPERTIES: OLECMDID = OLECMDID(10i32);
pub const OLECMDID_CUT: OLECMDID = OLECMDID(11i32);
pub const OLECMDID_COPY: OLECMDID = OLECMDID(12i32);
pub const OLECMDID_PASTE: OLECMDID = OLECMDID(13i32);
pub const OLECMDID_PASTESPECIAL: OLECMDID = OLECMDID(14i32);
pub const OLECMDID_UNDO: OLECMDID = OLECMDID(15i32);
pub const OLECMDID_REDO: OLECMDID = OLECMDID(16i32);
pub const OLECMDID_SELECTALL: OLECMDID = OLECMDID(17i32);
pub const OLECMDID_CLEARSELECTION: OLECMDID = OLECMDID(18i32);
pub const OLECMDID_ZOOM: OLECMDID = OLECMDID(19i32);
pub const OLECMDID_GETZOOMRANGE: OLECMDID = OLECMDID(20i32);
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = OLECMDID(21i32);
pub const OLECMDID_REFRESH: OLECMDID = OLECMDID(22i32);
pub const OLECMDID_STOP: OLECMDID = OLECMDID(23i32);
pub const OLECMDID_HIDETOOLBARS: OLECMDID = OLECMDID(24i32);
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = OLECMDID(25i32);
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = OLECMDID(26i32);
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = OLECMDID(27i32);
pub const OLECMDID_SETTITLE: OLECMDID = OLECMDID(28i32);
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = OLECMDID(29i32);
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = OLECMDID(30i32);
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = OLECMDID(31i32);
pub const OLECMDID_FIND: OLECMDID = OLECMDID(32i32);
pub const OLECMDID_DELETE: OLECMDID = OLECMDID(33i32);
pub const OLECMDID_HTTPEQUIV: OLECMDID = OLECMDID(34i32);
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = OLECMDID(35i32);
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = OLECMDID(36i32);
pub const OLECMDID_ONUNLOAD: OLECMDID = OLECMDID(37i32);
pub const OLECMDID_PROPERTYBAG2: OLECMDID = OLECMDID(38i32);
pub const OLECMDID_PREREFRESH: OLECMDID = OLECMDID(39i32);
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = OLECMDID(40i32);
pub const OLECMDID_SHOWMESSAGE: OLECMDID = OLECMDID(41i32);
pub const OLECMDID_SHOWFIND: OLECMDID = OLECMDID(42i32);
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = OLECMDID(43i32);
pub const OLECMDID_SHOWPRINT: OLECMDID = OLECMDID(44i32);
pub const OLECMDID_CLOSE: OLECMDID = OLECMDID(45i32);
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = OLECMDID(46i32);
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = OLECMDID(47i32);
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = OLECMDID(48i32);
pub const OLECMDID_PRINT2: OLECMDID = OLECMDID(49i32);
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = OLECMDID(50i32);
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = OLECMDID(51i32);
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = OLECMDID(52i32);
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = OLECMDID(55i32);
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = OLECMDID(56i32);
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = OLECMDID(57i32);
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = OLECMDID(58i32);
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = OLECMDID(59i32);
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = OLECMDID(60i32);
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = OLECMDID(61i32);
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = OLECMDID(62i32);
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = OLECMDID(63i32);
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = OLECMDID(64i32);
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = OLECMDID(65i32);
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = OLECMDID(66i32);
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = OLECMDID(67i32);
pub const OLECMDID_SHOWTASKDLG: OLECMDID = OLECMDID(68i32);
pub const OLECMDID_POPSTATEEVENT: OLECMDID = OLECMDID(69i32);
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = OLECMDID(70i32);
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = OLECMDID(71i32);
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = OLECMDID(72i32);
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = OLECMDID(73i32);
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = OLECMDID(74i32);
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = OLECMDID(75i32);
pub const OLECMDID_UPDATE_CARET: OLECMDID = OLECMDID(76i32);
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = OLECMDID(77i32);
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = OLECMDID(78i32);
pub const OLECMDID_SETFAVICON: OLECMDID = OLECMDID(79i32);
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = OLECMDID(80i32);
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = OLECMDID(81i32);
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = OLECMDID(82i32);
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = OLECMDID(83i32);
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = OLECMDID(84i32);
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = OLECMDID(85i32);
#[repr(transparent)]
pub struct OLECMDID_BROWSERSTATEFLAG(pub i32);
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(1i32);
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(2i32);
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(4i32);
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(8i32);
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(16i32);
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(32i32);
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(64i32);
#[repr(transparent)]
pub struct OLECMDID_OPTICAL_ZOOMFLAG(pub i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(1i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(16i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(32i32);
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(64i32);
#[repr(transparent)]
pub struct OLECMDID_PAGEACTIONFLAG(pub i32);
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32i32);
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(64i32);
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(128i32);
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(256i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1024i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2048i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4096i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8192i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16384i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32768i32);
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(65536i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(131072i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(262144i32);
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(524288i32);
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1048576i32);
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2097152i32);
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4194304i32);
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8388608i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16777216i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(33554432i32);
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(67108864i32);
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(134217728i32);
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(268435456i32);
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(536870912i32);
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1073741824i32);
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(-2147483648i32);
#[repr(transparent)]
pub struct OLECMDID_REFRESHFLAG(pub i32);
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(0i32);
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1i32);
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2i32);
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(3i32);
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4i32);
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(5i32);
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(255i32);
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4096i32);
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8192i32);
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16384i32);
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(32768i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(65536i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(131072i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(262144i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(524288i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1048576i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2097152i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4194304i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8388608i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16777216i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(33554432i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(67108864i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(134217728i32);
#[repr(transparent)]
pub struct OLECMDID_VIEWPORT_MODE_FLAG(pub i32);
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(1i32);
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(2i32);
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(65536i32);
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(131072i32);
#[repr(transparent)]
pub struct OLECMDID_WINDOWSTATE_FLAG(pub i32);
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(1i32);
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(2i32);
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(65536i32);
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(131072i32);
#[repr(C)]
pub struct OLECMDTEXT(i32);
#[repr(transparent)]
pub struct OLECMDTEXTF(pub i32);
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = OLECMDTEXTF(0i32);
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = OLECMDTEXTF(1i32);
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = OLECMDTEXTF(2i32);
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
#[repr(transparent)]
pub struct OLECONTF(pub i32);
pub const OLECONTF_EMBEDDINGS: OLECONTF = OLECONTF(1i32);
pub const OLECONTF_LINKS: OLECONTF = OLECONTF(2i32);
pub const OLECONTF_OTHERS: OLECONTF = OLECONTF(4i32);
pub const OLECONTF_ONLYUSER: OLECONTF = OLECONTF(8i32);
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = OLECONTF(16i32);
pub const OLECREATE_LEAVERUNNING: u32 = 1u32;
#[repr(transparent)]
pub struct OLEDCFLAGS(pub i32);
pub const OLEDC_NODRAW: OLEDCFLAGS = OLEDCFLAGS(1i32);
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = OLEDCFLAGS(2i32);
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = OLEDCFLAGS(4i32);
#[repr(transparent)]
pub struct OLEGETMONIKER(pub i32);
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = OLEGETMONIKER(1i32);
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = OLEGETMONIKER(2i32);
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = OLEGETMONIKER(3i32);
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = OLEGETMONIKER(4i32);
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6i32;
pub const OLEIVERB_HIDE: i32 = -3i32;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5i32;
pub const OLEIVERB_OPEN: i32 = -2i32;
pub const OLEIVERB_PRIMARY: i32 = 0i32;
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
pub const OLEIVERB_SHOW: i32 = -1i32;
pub const OLEIVERB_UIACTIVATE: i32 = -4i32;
#[repr(transparent)]
pub struct OLELINKBIND(pub i32);
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = OLELINKBIND(1i32);
#[repr(transparent)]
pub struct OLEMISC(pub i32);
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = OLEMISC(1i32);
pub const OLEMISC_ONLYICONIC: OLEMISC = OLEMISC(2i32);
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = OLEMISC(4i32);
pub const OLEMISC_STATIC: OLEMISC = OLEMISC(8i32);
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = OLEMISC(16i32);
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = OLEMISC(32i32);
pub const OLEMISC_ISLINKOBJECT: OLEMISC = OLEMISC(64i32);
pub const OLEMISC_INSIDEOUT: OLEMISC = OLEMISC(128i32);
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(256i32);
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = OLEMISC(512i32);
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = OLEMISC(1024i32);
pub const OLEMISC_ALWAYSRUN: OLEMISC = OLEMISC(2048i32);
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = OLEMISC(4096i32);
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = OLEMISC(8192i32);
pub const OLEMISC_NOUIACTIVATE: OLEMISC = OLEMISC(16384i32);
pub const OLEMISC_ALIGNABLE: OLEMISC = OLEMISC(32768i32);
pub const OLEMISC_SIMPLEFRAME: OLEMISC = OLEMISC(65536i32);
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = OLEMISC(131072i32);
pub const OLEMISC_IMEMODE: OLEMISC = OLEMISC(262144i32);
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(524288i32);
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = OLEMISC(1048576i32);
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = OLEMISC(2097152i32);
#[repr(transparent)]
pub struct OLERENDER(pub i32);
pub const OLERENDER_NONE: OLERENDER = OLERENDER(0i32);
pub const OLERENDER_DRAW: OLERENDER = OLERENDER(1i32);
pub const OLERENDER_FORMAT: OLERENDER = OLERENDER(2i32);
pub const OLERENDER_ASIS: OLERENDER = OLERENDER(3i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
#[repr(C)]
pub struct OLEUIBUSYA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
#[repr(C)]
pub struct OLEUIBUSYW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEUICHANGEICONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEUICHANGEICONW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[repr(C)]
pub struct OLEUICHANGESOURCEA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[repr(C)]
pub struct OLEUICHANGESOURCEW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEUICONVERTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEUICONVERTW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEUIEDITLINKSA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEUIEDITLINKSW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUIGNRLPROPSA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUIGNRLPROPSW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct OLEUIINSERTOBJECTA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct OLEUIINSERTOBJECTW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUILINKPROPSA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUILINKPROPSW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUIOBJECTPROPSA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUIOBJECTPROPSW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct OLEUIPASTEENTRYA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct OLEUIPASTEENTRYW(i32);
#[repr(transparent)]
pub struct OLEUIPASTEFLAG(pub i32);
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2048i32);
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = OLEUIPASTEFLAG(0i32);
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(512i32);
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1024i32);
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1i32);
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2i32);
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = OLEUIPASTEFLAG(4i32);
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = OLEUIPASTEFLAG(8i32);
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = OLEUIPASTEFLAG(16i32);
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = OLEUIPASTEFLAG(32i32);
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = OLEUIPASTEFLAG(64i32);
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = OLEUIPASTEFLAG(128i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct OLEUIPASTESPECIALA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct OLEUIPASTESPECIALW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUIVIEWPROPSA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct OLEUIVIEWPROPSW(i32);
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
pub const OLEUI_CANCEL: u32 = 2u32;
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
pub const OLEUI_FALSE: u32 = 0u32;
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
pub const OLEUI_OK: u32 = 1u32;
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
pub const OLEUI_SUCCESS: u32 = 1u32;
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
#[repr(transparent)]
pub struct OLEUPDATE(pub i32);
pub const OLEUPDATE_ALWAYS: OLEUPDATE = OLEUPDATE(1i32);
pub const OLEUPDATE_ONCALL: OLEUPDATE = OLEUPDATE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OLEVERB(i32);
#[repr(transparent)]
pub struct OLEVERBATTRIB(pub i32);
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = OLEVERBATTRIB(1i32);
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = OLEVERBATTRIB(2i32);
pub const OLEVERB_PRIMARY: u32 = 0u32;
#[repr(transparent)]
pub struct OLEWHICHMK(pub i32);
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = OLEWHICHMK(1i32);
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = OLEWHICHMK(2i32);
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = OLEWHICHMK(3i32);
#[repr(transparent)]
pub struct OLE_TRISTATE(pub i32);
pub const triUnchecked: OLE_TRISTATE = OLE_TRISTATE(0i32);
pub const triChecked: OLE_TRISTATE = OLE_TRISTATE(1i32);
pub const triGray: OLE_TRISTATE = OLE_TRISTATE(2i32);
pub const OPF_DISABLECONVERT: i32 = 8i32;
pub const OPF_NOFILLDEFAULT: i32 = 2i32;
pub const OPF_OBJECTISLINK: i32 = 1i32;
pub const OPF_SHOWHELP: i32 = 4i32;
pub const OT_EMBEDDED: i32 = 2i32;
pub const OT_LINK: i32 = 1i32;
pub const OT_STATIC: i32 = 3i32;
#[repr(C)]
pub struct OleMenuGroupWidths(i32);
#[repr(transparent)]
pub struct PAGEACTION_UI(pub i32);
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = PAGEACTION_UI(0i32);
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = PAGEACTION_UI(1i32);
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = PAGEACTION_UI(2i32);
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = PAGEACTION_UI(3i32);
#[repr(C)]
pub struct PAGERANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PAGESET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PARAMDATA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct PARAMDESC(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct PARAMDESCEX(i32);
pub const PARAMFLAG_FHASCUSTDATA: u32 = 64u32;
pub const PARAMFLAG_FHASDEFAULT: u32 = 32u32;
pub const PARAMFLAG_FIN: u32 = 1u32;
pub const PARAMFLAG_FLCID: u32 = 4u32;
pub const PARAMFLAG_FOPT: u32 = 16u32;
pub const PARAMFLAG_FOUT: u32 = 2u32;
pub const PARAMFLAG_FRETVAL: u32 = 8u32;
pub const PARAMFLAG_NONE: u32 = 0u32;
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
pub const PERPROP_E_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
pub const PERPROP_E_NOPAGEAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub const PERPROP_S_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262656i32 as _);
pub const PERPROP_S_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262671i32 as _);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PICTDESC(i32);
pub const PICTYPE_BITMAP: u32 = 1u32;
pub const PICTYPE_ENHMETAFILE: u32 = 4u32;
pub const PICTYPE_ICON: u32 = 3u32;
pub const PICTYPE_METAFILE: u32 = 2u32;
pub const PICTYPE_NONE: u32 = 0u32;
pub const PICTYPE_UNINITIALIZED: i32 = -1i32;
#[repr(transparent)]
pub struct POINTERINACTIVE(pub i32);
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = POINTERINACTIVE(1i32);
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = POINTERINACTIVE(2i32);
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = POINTERINACTIVE(4i32);
#[repr(C)]
pub struct POINTF(i32);
#[repr(transparent)]
pub struct PRINTFLAG(pub u32);
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = PRINTFLAG(1u32);
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = PRINTFLAG(2u32);
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = PRINTFLAG(4u32);
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = PRINTFLAG(8u32);
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = PRINTFLAG(16u32);
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = PRINTFLAG(32u32);
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = PRINTFLAG(64u32);
#[repr(transparent)]
pub struct PROPBAG2_TYPE(pub i32);
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = PROPBAG2_TYPE(0i32);
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = PROPBAG2_TYPE(1i32);
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = PROPBAG2_TYPE(2i32);
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = PROPBAG2_TYPE(3i32);
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = PROPBAG2_TYPE(4i32);
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = PROPBAG2_TYPE(5i32);
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = PROPBAG2_TYPE(6i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROPPAGEINFO(i32);
#[repr(transparent)]
pub struct PROPPAGESTATUS(pub i32);
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = PROPPAGESTATUS(1i32);
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = PROPPAGESTATUS(2i32);
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = PROPPAGESTATUS(4i32);
pub const PSF_CHECKDISPLAYASICON: i32 = 8i32;
pub const PSF_DISABLEDISPLAYASICON: i32 = 16i32;
pub const PSF_HIDECHANGEICON: i32 = 32i32;
pub const PSF_NOREFRESHDATAOBJECT: i32 = 128i32;
pub const PSF_SELECTPASTE: i32 = 2i32;
pub const PSF_SELECTPASTELINK: i32 = 4i32;
pub const PSF_SHOWHELP: i32 = 1i32;
pub const PSF_STAYONCLIPBOARDCHANGE: i32 = 64i32;
pub const PS_MAXLINKTYPES: u32 = 8u32;
#[repr(transparent)]
pub struct PictureAttributes(pub i32);
pub const PICTURE_SCALABLE: PictureAttributes = PictureAttributes(1i32);
pub const PICTURE_TRANSPARENT: PictureAttributes = PictureAttributes(2i32);
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct QACONTAINER(i32);
#[repr(transparent)]
pub struct QACONTAINERFLAGS(pub i32);
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = QACONTAINERFLAGS(1i32);
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = QACONTAINERFLAGS(2i32);
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = QACONTAINERFLAGS(4i32);
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = QACONTAINERFLAGS(8i32);
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = QACONTAINERFLAGS(16i32);
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = QACONTAINERFLAGS(32i32);
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = QACONTAINERFLAGS(64i32);
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = QACONTAINERFLAGS(128i32);
#[repr(C)]
pub struct QACONTROL(i32);
#[repr(transparent)]
pub struct READYSTATE(pub i32);
pub const READYSTATE_UNINITIALIZED: READYSTATE = READYSTATE(0i32);
pub const READYSTATE_LOADING: READYSTATE = READYSTATE(1i32);
pub const READYSTATE_LOADED: READYSTATE = READYSTATE(2i32);
pub const READYSTATE_INTERACTIVE: READYSTATE = READYSTATE(3i32);
pub const READYSTATE_COMPLETE: READYSTATE = READYSTATE(4i32);
#[repr(transparent)]
pub struct REGKIND(pub i32);
pub const REGKIND_DEFAULT: REGKIND = REGKIND(0i32);
pub const REGKIND_REGISTER: REGKIND = REGKIND(1i32);
pub const REGKIND_NONE: REGKIND = REGKIND(2i32);
pub const SELFREG_E_CLASS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
pub const SELFREG_E_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
pub const SELFREG_E_TYPELIB: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub const SELFREG_S_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262656i32 as _);
pub const SELFREG_S_LAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262671i32 as _);
#[repr(transparent)]
pub struct SF_TYPE(pub i32);
pub const SF_ERROR: SF_TYPE = SF_TYPE(10i32);
pub const SF_I1: SF_TYPE = SF_TYPE(16i32);
pub const SF_I2: SF_TYPE = SF_TYPE(2i32);
pub const SF_I4: SF_TYPE = SF_TYPE(3i32);
pub const SF_I8: SF_TYPE = SF_TYPE(20i32);
pub const SF_BSTR: SF_TYPE = SF_TYPE(8i32);
pub const SF_UNKNOWN: SF_TYPE = SF_TYPE(13i32);
pub const SF_DISPATCH: SF_TYPE = SF_TYPE(9i32);
pub const SF_VARIANT: SF_TYPE = SF_TYPE(12i32);
pub const SF_RECORD: SF_TYPE = SF_TYPE(36i32);
pub const SF_HAVEIID: SF_TYPE = SF_TYPE(32781i32);
pub const SID_GetCaller: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1192741952, data2: 48313, data3: 4560, data4: [147, 54, 0, 160, 201, 13, 202, 169] };
pub const SID_ProvideRuntimeContext: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1956971532, data2: 56588, data3: 18672, data4: [172, 133, 25, 76, 50, 89, 24, 10] };
pub const SID_VariantConversion: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 521147521, data2: 48333, data3: 4560, data4: [147, 54, 0, 160, 201, 13, 202, 169] };
pub const STDOLE2_LCID: u32 = 0u32;
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
pub const STDOLE_LCID: u32 = 0u32;
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
pub const STDOLE_MINORVERNUM: u32 = 0u32;
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
#[repr(transparent)]
pub struct TYPEFLAGS(pub i32);
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = TYPEFLAGS(1i32);
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = TYPEFLAGS(2i32);
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = TYPEFLAGS(4i32);
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = TYPEFLAGS(8i32);
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = TYPEFLAGS(16i32);
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = TYPEFLAGS(32i32);
pub const TYPEFLAG_FDUAL: TYPEFLAGS = TYPEFLAGS(64i32);
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = TYPEFLAGS(128i32);
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = TYPEFLAGS(256i32);
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = TYPEFLAGS(512i32);
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = TYPEFLAGS(1024i32);
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = TYPEFLAGS(2048i32);
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = TYPEFLAGS(4096i32);
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = TYPEFLAGS(8192i32);
pub const TYPEFLAG_FPROXY: TYPEFLAGS = TYPEFLAGS(16384i32);
#[repr(transparent)]
pub struct UASFLAGS(pub i32);
pub const UAS_NORMAL: UASFLAGS = UASFLAGS(0i32);
pub const UAS_BLOCKED: UASFLAGS = UASFLAGS(1i32);
pub const UAS_NOPARENTENABLE: UASFLAGS = UASFLAGS(2i32);
pub const UAS_MASK: UASFLAGS = UASFLAGS(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UDATE(i32);
#[repr(transparent)]
pub struct UPDFCACHE_FLAGS(pub u32);
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483647u32);
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483646u32);
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(8u32);
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(16u32);
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483648u32);
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(1u32);
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2u32);
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(4u32);
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(18u32);
#[repr(transparent)]
pub struct USERCLASSTYPE(pub i32);
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = USERCLASSTYPE(1i32);
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = USERCLASSTYPE(2i32);
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = USERCLASSTYPE(3i32);
pub const VARCMP_EQ: u32 = 1u32;
pub const VARCMP_GT: u32 = 2u32;
pub const VARCMP_LT: u32 = 0u32;
pub const VARCMP_NULL: u32 = 3u32;
#[repr(transparent)]
pub struct VARENUM(pub i32);
pub const VT_EMPTY: VARENUM = VARENUM(0i32);
pub const VT_NULL: VARENUM = VARENUM(1i32);
pub const VT_I2: VARENUM = VARENUM(2i32);
pub const VT_I4: VARENUM = VARENUM(3i32);
pub const VT_R4: VARENUM = VARENUM(4i32);
pub const VT_R8: VARENUM = VARENUM(5i32);
pub const VT_CY: VARENUM = VARENUM(6i32);
pub const VT_DATE: VARENUM = VARENUM(7i32);
pub const VT_BSTR: VARENUM = VARENUM(8i32);
pub const VT_DISPATCH: VARENUM = VARENUM(9i32);
pub const VT_ERROR: VARENUM = VARENUM(10i32);
pub const VT_BOOL: VARENUM = VARENUM(11i32);
pub const VT_VARIANT: VARENUM = VARENUM(12i32);
pub const VT_UNKNOWN: VARENUM = VARENUM(13i32);
pub const VT_DECIMAL: VARENUM = VARENUM(14i32);
pub const VT_I1: VARENUM = VARENUM(16i32);
pub const VT_UI1: VARENUM = VARENUM(17i32);
pub const VT_UI2: VARENUM = VARENUM(18i32);
pub const VT_UI4: VARENUM = VARENUM(19i32);
pub const VT_I8: VARENUM = VARENUM(20i32);
pub const VT_UI8: VARENUM = VARENUM(21i32);
pub const VT_INT: VARENUM = VARENUM(22i32);
pub const VT_UINT: VARENUM = VARENUM(23i32);
pub const VT_VOID: VARENUM = VARENUM(24i32);
pub const VT_HRESULT: VARENUM = VARENUM(25i32);
pub const VT_PTR: VARENUM = VARENUM(26i32);
pub const VT_SAFEARRAY: VARENUM = VARENUM(27i32);
pub const VT_CARRAY: VARENUM = VARENUM(28i32);
pub const VT_USERDEFINED: VARENUM = VARENUM(29i32);
pub const VT_LPSTR: VARENUM = VARENUM(30i32);
pub const VT_LPWSTR: VARENUM = VARENUM(31i32);
pub const VT_RECORD: VARENUM = VARENUM(36i32);
pub const VT_INT_PTR: VARENUM = VARENUM(37i32);
pub const VT_UINT_PTR: VARENUM = VARENUM(38i32);
pub const VT_FILETIME: VARENUM = VARENUM(64i32);
pub const VT_BLOB: VARENUM = VARENUM(65i32);
pub const VT_STREAM: VARENUM = VARENUM(66i32);
pub const VT_STORAGE: VARENUM = VARENUM(67i32);
pub const VT_STREAMED_OBJECT: VARENUM = VARENUM(68i32);
pub const VT_STORED_OBJECT: VARENUM = VARENUM(69i32);
pub const VT_BLOB_OBJECT: VARENUM = VARENUM(70i32);
pub const VT_CF: VARENUM = VARENUM(71i32);
pub const VT_CLSID: VARENUM = VARENUM(72i32);
pub const VT_VERSIONED_STREAM: VARENUM = VARENUM(73i32);
pub const VT_BSTR_BLOB: VARENUM = VARENUM(4095i32);
pub const VT_VECTOR: VARENUM = VARENUM(4096i32);
pub const VT_ARRAY: VARENUM = VARENUM(8192i32);
pub const VT_BYREF: VARENUM = VARENUM(16384i32);
pub const VT_RESERVED: VARENUM = VARENUM(32768i32);
pub const VT_ILLEGAL: VARENUM = VARENUM(65535i32);
pub const VT_ILLEGALMASKED: VARENUM = VARENUM(4095i32);
pub const VT_TYPEMASK: VARENUM = VARENUM(4095i32);
#[repr(transparent)]
pub struct VARFLAGS(pub i32);
pub const VARFLAG_FREADONLY: VARFLAGS = VARFLAGS(1i32);
pub const VARFLAG_FSOURCE: VARFLAGS = VARFLAGS(2i32);
pub const VARFLAG_FBINDABLE: VARFLAGS = VARFLAGS(4i32);
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = VARFLAGS(8i32);
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = VARFLAGS(16i32);
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = VARFLAGS(32i32);
pub const VARFLAG_FHIDDEN: VARFLAGS = VARFLAGS(64i32);
pub const VARFLAG_FRESTRICTED: VARFLAGS = VARFLAGS(128i32);
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = VARFLAGS(256i32);
pub const VARFLAG_FUIDEFAULT: VARFLAGS = VARFLAGS(512i32);
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = VARFLAGS(1024i32);
pub const VARFLAG_FREPLACEABLE: VARFLAGS = VARFLAGS(2048i32);
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = VARFLAGS(4096i32);
pub const VARIANT_ALPHABOOL: u32 = 2u32;
pub const VARIANT_CALENDAR_GREGORIAN: u32 = 64u32;
pub const VARIANT_CALENDAR_HIJRI: u32 = 8u32;
pub const VARIANT_CALENDAR_THAI: u32 = 32u32;
pub const VARIANT_LOCALBOOL: u32 = 16u32;
pub const VARIANT_NOUSEROVERRIDE: u32 = 4u32;
pub const VARIANT_NOVALUEPROP: u32 = 1u32;
pub const VARIANT_USE_NLS: u32 = 128u32;
#[repr(transparent)]
pub struct VIEWSTATUS(pub i32);
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = VIEWSTATUS(1i32);
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = VIEWSTATUS(2i32);
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = VIEWSTATUS(4i32);
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = VIEWSTATUS(8i32);
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = VIEWSTATUS(16i32);
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = VIEWSTATUS(32i32);
pub const VPF_DISABLERELATIVE: i32 = 2i32;
pub const VPF_DISABLESCALE: i32 = 4i32;
pub const VPF_SELECTRELATIVE: i32 = 1i32;
pub const VTDATEGRE_MAX: u32 = 2958465u32;
pub const VTDATEGRE_MIN: i32 = -657434i32;
pub const VT_BLOB_PROPSET: u32 = 75u32;
pub const VT_STORED_PROPSET: u32 = 74u32;
pub const VT_STREAMED_PROPSET: u32 = 73u32;
pub const VT_VERBOSE_ENUM: u32 = 76u32;
pub const WIN32: u32 = 100u32;
#[repr(transparent)]
pub struct WPCSETTING(pub i32);
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = WPCSETTING(1i32);
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = WPCSETTING(2i32);
#[repr(transparent)]
pub struct XFORMCOORDS(pub i32);
pub const XFORMCOORDS_POSITION: XFORMCOORDS = XFORMCOORDS(1i32);
pub const XFORMCOORDS_SIZE: XFORMCOORDS = XFORMCOORDS(2i32);
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = XFORMCOORDS(4i32);
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = XFORMCOORDS(8i32);
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = XFORMCOORDS(16i32);
#[repr(C)]
pub struct _wireBRECORD(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct _wireSAFEARRAY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct _wireSAFEARRAY_UNION(i32);
#[repr(C)]
pub struct _wireSAFEARR_BRECORD(i32);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _wireSAFEARR_BSTR(i32);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _wireSAFEARR_DISPATCH(i32);
#[repr(C)]
pub struct _wireSAFEARR_HAVEIID(i32);
#[repr(C)]
pub struct _wireSAFEARR_UNKNOWN(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct _wireSAFEARR_VARIANT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct _wireVARIANT(i32);
