#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BstrFromVector(psa: *const super::Com::SAFEARRAY, pbstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ClearCustData(pcustdata: *mut super::Com::CUSTDATA);
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn CreateErrorInfo(pperrinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn CreateOleAdviseHolder(ppoaholder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateStdDispatch(punkouter: ::windows::runtime::RawPtr, pvthis: *mut ::core::ffi::c_void, ptinfo: ::windows::runtime::RawPtr, ppunkstddisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateTypeLib(syskind: super::Com::SYSKIND, szfile: super::super::Foundation::PWSTR, ppctlib: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateTypeLib2(syskind: super::Com::SYSKIND, szfile: super::super::Foundation::PWSTR, ppctlib: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispCallFunc(pvinstance: *const ::core::ffi::c_void, ovft: usize, cc: super::Com::CALLCONV, vtreturn: u16, cactuals: u32, prgvt: *const u16, prgpvarg: *const *const super::Com::VARIANT, pvargresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetIDsOfNames(ptinfo: ::windows::runtime::RawPtr, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetParam(pdispparams: *const super::Com::DISPPARAMS, position: u32, vttarg: u16, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, puargerr: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispInvoke(_this: *mut ::core::ffi::c_void, ptinfo: ::windows::runtime::RawPtr, dispidmember: i32, wflags: u16, pparams: *mut super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DoDragDrop(pdataobj: ::windows::runtime::RawPtr, pdropsource: ::windows::runtime::RawPtr, dwokeffects: u32, pdweffect: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn GetActiveObject(rclsid: *const ::windows::runtime::GUID, pvreserved: *mut ::core::ffi::c_void, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltMonthNames(lcid: u32, prgp: *mut *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn GetRecordInfoFromGuids(rguidtypelib: *const ::windows::runtime::GUID, uvermajor: u32, uverminor: u32, lcid: u32, rguidtypeinfo: *const ::windows::runtime::GUID, pprecinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetRecordInfoFromTypeInfo(ptypeinfo: ::windows::runtime::RawPtr, pprecinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN);
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn IsAccelerator(haccel: super::super::UI::WindowsAndMessaging::HACCEL, caccelentries: i32, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LHashValOfNameSys(syskind: super::Com::SYSKIND, lcid: u32, szname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LHashValOfNameSysA(syskind: super::Com::SYSKIND, lcid: u32, szname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadRegTypeLib(rguid: *const ::windows::runtime::GUID, wvermajor: u16, wverminor: u16, lcid: u32, pptlib: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LoadTypeLib(szfile: super::super::Foundation::PWSTR, pptlib: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LoadTypeLibEx(szfile: super::super::Foundation::PWSTR, regkind: REGKIND, pptlib: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OaBuildVersion() -> u32;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OaEnablePerUserTLibRegistration();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleBuildVersion() -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreate(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleCreateDefaultHandler(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleCreateEmbeddingHelper(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, flags: u32, pcf: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateEx(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleCreateFontIndirect(lpfontdesc: *mut FONTDESC, riid: *const ::windows::runtime::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromData(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromDataEx(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromFile(rclsid: *const ::windows::runtime::GUID, lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromFileEx(rclsid: *const ::windows::runtime::GUID, lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLink(pmklinksrc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkEx(pmklinksrc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkFromData(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkFromDataEx(psrcdataobj: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkToFile(lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, renderopt: u32, lpformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkToFileEx(lpszfilename: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *mut u32, rgformatetc: *mut super::Com::FORMATETC, lpadvisesink: ::windows::runtime::RawPtr, rgdwconnection: *mut u32, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleCreateMenuDescriptor(hmenucombined: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OleMenuGroupWidths) -> isize;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleCreatePictureIndirect(lppictdesc: *mut PICTDESC, riid: *const ::windows::runtime::GUID, fown: super::super::Foundation::BOOL, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrame(hwndowner: super::super::Foundation::HWND, x: u32, y: u32, lpszcaption: super::super::Foundation::PWSTR, cobjects: u32, ppunk: *mut ::windows::runtime::RawPtr, cpages: u32, ppageclsid: *mut ::windows::runtime::GUID, lcid: u32, dwreserved: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrameIndirect(lpparams: *mut OCPFIPARAMS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateStaticFromData(psrcdataobj: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, renderopt: u32, pformatetc: *mut super::Com::FORMATETC, pclientsite: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleDoAutoConvert(pstg: ::windows::runtime::RawPtr, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleDraw(punknown: ::windows::runtime::RawPtr, dwaspect: u32, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleDuplicateData(hsrc: super::super::Foundation::HANDLE, cfformat: u16, uiflags: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleFlushClipboard() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleGetAutoConvert(clsidold: *const ::windows::runtime::GUID, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboard(ppdataobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut ::windows::runtime::RawPtr, dataenterpriseid: *mut super::super::Foundation::PWSTR, sourcedescription: *mut super::super::Foundation::PWSTR, targetdescription: *mut super::super::Foundation::PWSTR, datadescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfClass(rclsid: *const ::windows::runtime::GUID, lpszlabel: super::super::Foundation::PWSTR, fusetypeaslabel: super::super::Foundation::BOOL) -> isize;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfFile(lpszpath: super::super::Foundation::PWSTR, fusefileaslabel: super::super::Foundation::BOOL) -> isize;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleIconToCursor(hinstexe: super::super::Foundation::HINSTANCE, hicon: super::super::UI::WindowsAndMessaging::HICON) -> super::super::UI::WindowsAndMessaging::HCURSOR;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleInitialize(pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleIsCurrentClipboard(pdataobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleIsRunning(pobject: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleLoad(pstg: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pclientsite: ::windows::runtime::RawPtr, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadFromStream(pstm: ::windows::runtime::RawPtr, iidinterface: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPicture(lpstream: ::windows::runtime::RawPtr, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureEx(lpstream: ::windows::runtime::RawPtr, lsize: i32, frunmode: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFile(varfilename: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lplpdisppicture: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFileEx(varfilename: ::core::mem::ManuallyDrop<super::Com::VARIANT>, xsizedesired: u32, ysizedesired: u32, dwflags: u32, lplpdisppicture: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLoadPicturePath(szurlorpath: super::super::Foundation::PWSTR, punkcaller: ::windows::runtime::RawPtr, dwreserved: u32, clrreserved: u32, riid: *const ::windows::runtime::GUID, ppvret: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLockRunning(punknown: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleMetafilePictFromIconAndLabel(hicon: super::super::UI::WindowsAndMessaging::HICON, lpszlabel: super::super::Foundation::PWSTR, lpszsourcefile: super::super::Foundation::PWSTR, iiconindex: u32) -> isize;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleNoteObjectVisible(punknown: ::windows::runtime::RawPtr, fvisible: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryCreateFromData(psrcdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryLinkFromData(psrcdataobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleRegEnumFormatEtc(clsid: *const ::windows::runtime::GUID, dwdirection: u32, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleRegEnumVerbs(clsid: *const ::windows::runtime::GUID, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleRegGetMiscStatus(clsid: *const ::windows::runtime::GUID, dwaspect: u32, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleRegGetUserType(clsid: *const ::windows::runtime::GUID, dwformoftype: u32, pszusertype: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleRun(punknown: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleSave(pps: ::windows::runtime::RawPtr, pstg: ::windows::runtime::RawPtr, fsameasload: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSavePictureFile(lpdisppicture: ::windows::runtime::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSaveToStream(ppstm: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleSetAutoConvert(clsidold: *const ::windows::runtime::GUID, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSetClipboard(pdataobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetContainedObject(punknown: ::windows::runtime::RawPtr, fcontained: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetMenuDescriptor(holemenu: isize, hwndframe: super::super::Foundation::HWND, hwndactiveobject: super::super::Foundation::HWND, lpframe: ::windows::runtime::RawPtr, lpactiveobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleTranslateAccelerator(lpframe: ::windows::runtime::RawPtr, lpframeinfo: *mut OIFI, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleTranslateColor(clr: u32, hpal: super::super::Graphics::Gdi::HPALETTE, lpcolorref: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuA(lpoleobj: ::windows::runtime::RawPtr, lpszshorttype: super::super::Foundation::PSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuW(lpoleobj: ::windows::runtime::RawPtr, lpszshorttype: super::super::Foundation::PWSTR, hmenu: super::super::UI::WindowsAndMessaging::HMENU, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: super::super::Foundation::BOOL, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyA(param0: *const ::core::mem::ManuallyDrop<OLEUIBUSYA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyW(param0: *const ::core::mem::ManuallyDrop<OLEUIBUSYW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUICanConvertOrActivateAs(rclsid: *const ::windows::runtime::GUID, fislinkedobject: super::super::Foundation::BOOL, wformat: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconA(param0: *const ::core::mem::ManuallyDrop<OLEUICHANGEICONA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconW(param0: *const ::core::mem::ManuallyDrop<OLEUICHANGEICONW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceA(param0: *const ::core::mem::ManuallyDrop<OLEUICHANGESOURCEA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceW(param0: *const ::core::mem::ManuallyDrop<OLEUICHANGESOURCEW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertA(param0: *const ::core::mem::ManuallyDrop<OLEUICONVERTA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertW(param0: *const ::core::mem::ManuallyDrop<OLEUICONVERTW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksA(param0: *const ::core::mem::ManuallyDrop<OLEUIEDITLINKSA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksW(param0: *const ::core::mem::ManuallyDrop<OLEUIEDITLINKSW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectA(param0: *const ::core::mem::ManuallyDrop<OLEUIINSERTOBJECTA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectW(param0: *const ::core::mem::ManuallyDrop<OLEUIINSERTOBJECTW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesA(param0: *const ::core::mem::ManuallyDrop<OLEUIOBJECTPROPSA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesW(param0: *const ::core::mem::ManuallyDrop<OLEUIOBJECTPROPSW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialA(param0: *const ::core::mem::ManuallyDrop<OLEUIPASTESPECIALA>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialW(param0: *const ::core::mem::ManuallyDrop<OLEUIPASTESPECIALW>) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserA(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserW(ntemplate: i32, hwndparent: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksA(lpoleuilinkcntr: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, lpsztitle: super::super::Foundation::PSTR, clinks: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksW(lpoleuilinkcntr: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, lpsztitle: super::super::Foundation::PWSTR, clinks: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleUninitialize();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn QueryPathOfRegTypeLib(guid: *const ::windows::runtime::GUID, wmaj: u16, wmin: u16, lcid: u32, lpbstrpathname: *mut *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn RegisterActiveObject(punk: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, dwflags: u32, pdwregister: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDragDrop(hwnd: super::super::Foundation::HWND, pdroptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn RegisterTypeLib(ptlib: ::windows::runtime::RawPtr, szfullpath: super::super::Foundation::PWSTR, szhelpdir: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn RegisterTypeLibForUser(ptlib: ::windows::runtime::RawPtr, szfullpath: super::super::Foundation::PWSTR, szhelpdir: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseStgMedium(param0: *mut ::core::mem::ManuallyDrop<super::Com::STGMEDIUM>);
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn RevokeActiveObject(dwregister: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevokeDragDrop(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAccessData(psa: *const super::Com::SAFEARRAY, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAddRef(psa: *const super::Com::SAFEARRAY, ppdatatorelease: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocData(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptor(cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptorEx(vt: u16, cdims: u32, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopy(psa: *const super::Com::SAFEARRAY, ppsaout: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopyData(psasource: *const super::Com::SAFEARRAY, psatarget: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreate(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateEx(vt: u16, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVector(vt: u16, llbound: i32, celements: u32) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVectorEx(vt: u16, llbound: i32, celements: u32, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroy(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyData(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyDescriptor(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetDim(psa: *const super::Com::SAFEARRAY) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElemsize(psa: *const super::Com::SAFEARRAY) -> u32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetIID(psa: *const super::Com::SAFEARRAY, pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetLBound(psa: *const super::Com::SAFEARRAY, ndim: u32, pllbound: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetUBound(psa: *const super::Com::SAFEARRAY, ndim: u32, plubound: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetVartype(psa: *const super::Com::SAFEARRAY, pvt: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayLock(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPtrOfIndex(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPutElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayRedim(psa: *mut super::Com::SAFEARRAY, psaboundnew: *const super::Com::SAFEARRAYBOUND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn SafeArrayReleaseData(pdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayReleaseDescriptor(psa: *const super::Com::SAFEARRAY);
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetIID(psa: *const super::Com::SAFEARRAY, guid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetRecordInfo(psa: *const super::Com::SAFEARRAY, prinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnaccessData(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnlock(psa: *const super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToVariantTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, pvtime: *mut f64) -> i32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLib(libid: *const ::windows::runtime::GUID, wvermajor: u16, wverminor: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLibForUser(libid: *const ::windows::runtime::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAbs(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAdd(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAnd(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromCy(cyin: super::Com::CY, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromDate(datein: f64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromDec(pdecin: *const super::super::Foundation::DECIMAL, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromI1(cin: super::super::Foundation::CHAR, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromI2(sin: i16, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromI4(lin: i32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromI8(i64in: i64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromR4(fltin: f32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromR8(dblin: f64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI1(bin: u8, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI2(uiin: u16, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI4(ulin: u32, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI8(i64in: u64, pboolout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCat(bstrleft: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrright: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrresult: *mut *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCmp(bstrleft: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrright: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcid: u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromBool(boolin: i16, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromCy(cyin: super::Com::CY, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDate(datein: f64, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDec(pdecin: *const super::super::Foundation::DECIMAL, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI1(cin: super::super::Foundation::CHAR, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI2(ival: i16, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI4(lin: i32, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI8(i64in: i64, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR4(fltin: f32, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR8(dblin: f64, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI1(bval: u8, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI2(uiin: u16, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI4(ulin: u32, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI8(ui64in: u64, lcid: u32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCat(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCmp(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, lcid: u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAbs(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAdd(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmp(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmpR8(cyleft: super::Com::CY, dblright: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFix(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromBool(boolin: i16, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDate(datein: f64, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromDec(pdecin: *const super::super::Foundation::DECIMAL, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromI1(cin: super::super::Foundation::CHAR, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI2(sin: i16, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI4(lin: i32, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI8(i64in: i64, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR4(fltin: f32, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR8(dblin: f64, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI1(bin: u8, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI2(uiin: u16, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI4(ulin: u32, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI8(ui64in: u64, pcyout: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyInt(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMul(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI4(cyleft: super::Com::CY, lright: i32, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI8(cyleft: super::Com::CY, lright: i64, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyNeg(cyin: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyRound(cyin: super::Com::CY, cdecimals: i32, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCySub(cyleft: super::Com::CY, cyright: super::Com::CY, pcyresult: *mut super::Com::CY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromBool(boolin: i16, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromCy(cyin: super::Com::CY, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromDec(pdecin: *const super::super::Foundation::DECIMAL, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromI1(cin: super::super::Foundation::CHAR, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromI2(sin: i16, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromI4(lin: i32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromI8(i64in: i64, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromR4(fltin: f32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromR8(dblin: f64, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI1(bin: u8, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI2(uiin: u16, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI4(ulin: u32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI8(ui64in: u64, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: u32, dwflags: u32, pdateout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAbs(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAdd(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmp(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmpR8(pdecleft: *const super::super::Foundation::DECIMAL, dblright: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecDiv(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFix(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromBool(boolin: i16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromCy(cyin: super::Com::CY, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromDate(datein: f64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI1(cin: super::super::Foundation::CHAR, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI2(uiin: i16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI4(lin: i32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI8(i64in: i64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR4(fltin: f32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR8(dblin: f64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI1(bin: u8, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI2(uiin: u16, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI4(ulin: u32, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI8(ui64in: u64, pdecout: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecInt(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecMul(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecNeg(pdecin: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecRound(pdecin: *const super::super::Foundation::DECIMAL, cdecimals: i32, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecSub(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL, pdecresult: *mut super::super::Foundation::DECIMAL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDiv(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarEqv(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFix(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormat(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pstrformat: super::super::Foundation::PWSTR, ifirstday: i32, ifirstweek: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatCurrency(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatDateTime(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, inamedformat: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatFromTokens(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pstrformat: super::super::Foundation::PWSTR, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcid: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatNumber(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatPercent(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromBool(boolin: i16, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarI1FromCy(cyin: super::Com::CY, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDate(datein: f64, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarI1FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI2(uiin: i16, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI4(lin: i32, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI8(i64in: i64, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromR4(fltin: f32, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromR8(dblin: f64, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI1(bin: u8, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI2(uiin: u16, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI4(ulin: u32, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI8(i64in: u64, pcout: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromBool(boolin: i16, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromCy(cyin: super::Com::CY, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromDate(datein: f64, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromDec(pdecin: *const super::super::Foundation::DECIMAL, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromI1(cin: super::super::Foundation::CHAR, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromI4(lin: i32, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromI8(i64in: i64, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromR4(fltin: f32, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromR8(dblin: f64, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI1(bin: u8, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI2(uiin: u16, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI4(ulin: u32, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI8(ui64in: u64, psout: *mut i16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromBool(boolin: i16, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromCy(cyin: super::Com::CY, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromDate(datein: f64, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromDec(pdecin: *const super::super::Foundation::DECIMAL, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromI1(cin: super::super::Foundation::CHAR, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromI2(sin: i16, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromI8(i64in: i64, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromR4(fltin: f32, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromR8(dblin: f64, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI1(bin: u8, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI2(uiin: u16, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI4(ulin: u32, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI8(ui64in: u64, plout: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromBool(boolin: i16, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromCy(cyin: super::Com::CY, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromDate(datein: f64, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromI1(cin: super::super::Foundation::CHAR, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromI2(sin: i16, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromR4(fltin: f32, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromR8(dblin: f64, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI1(bin: u8, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI2(uiin: u16, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI4(ulin: u32, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI8(ui64in: u64, pi64out: *mut i64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarIdiv(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarImp(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarInt(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMod(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMul(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNeg(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNot(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32, pvar: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarOr(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarParseNumFromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarPow(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4CmpR8(fltleft: f32, dblright: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromBool(boolin: i16, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromCy(cyin: super::Com::CY, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromDate(datein: f64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromDec(pdecin: *const super::super::Foundation::DECIMAL, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromI1(cin: super::super::Foundation::CHAR, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromI2(sin: i16, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromI4(lin: i32, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromI8(i64in: i64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromR8(dblin: f64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI1(bin: u8, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI2(uiin: u16, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI4(ulin: u32, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI8(ui64in: u64, pfltout: *mut f32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromBool(boolin: i16, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromCy(cyin: super::Com::CY, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromDate(datein: f64, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromI1(cin: super::super::Foundation::CHAR, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromI2(sin: i16, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromI4(lin: i32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromI8(i64in: i64, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromR4(fltin: f32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI1(bin: u8, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI2(uiin: u16, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI4(ulin: u32, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI8(ui64in: u64, pdblout: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8Pow(dblleft: f64, dblright: f64, pdblresult: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8Round(dblin: f64, cdecimals: i32, pdblresult: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarRound(pvarin: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, cdecimals: i32, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarSub(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarTokenizeFormatString(pstrformat: super::super::Foundation::PWSTR, rgbtok: *mut u8, cbtok: i32, ifirstday: i32, ifirstweek: i32, lcid: u32, pcbactual: *const i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromBool(boolin: i16, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromCy(cyin: super::Com::CY, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromDate(datein: f64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromI1(cin: super::super::Foundation::CHAR, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromI2(sin: i16, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromI4(lin: i32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromI8(i64in: i64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromR4(fltin: f32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromR8(dblin: f64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromUI2(uiin: u16, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromUI4(ulin: u32, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromUI8(ui64in: u64, pbout: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromBool(boolin: i16, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromCy(cyin: super::Com::CY, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromDate(datein: f64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromDec(pdecin: *const super::super::Foundation::DECIMAL, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromI1(cin: super::super::Foundation::CHAR, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromI2(uiin: i16, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromI4(lin: i32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromI8(i64in: i64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromR4(fltin: f32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromUI1(bin: u8, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromUI4(ulin: u32, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromUI8(i64in: u64, puiout: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromBool(boolin: i16, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromCy(cyin: super::Com::CY, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromDate(datein: f64, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromDec(pdecin: *const super::super::Foundation::DECIMAL, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromI1(cin: super::super::Foundation::CHAR, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromI2(uiin: i16, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromI4(lin: i32, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromI8(i64in: i64, plout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromR4(fltin: f32, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromR8(dblin: f64, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromUI1(bin: u8, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromUI2(uiin: u16, pulout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromUI8(ui64in: u64, plout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromBool(boolin: i16, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromCy(cyin: super::Com::CY, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromDate(datein: f64, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromDec(pdecin: *const super::super::Foundation::DECIMAL, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromDisp(pdispin: ::windows::runtime::RawPtr, lcid: u32, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromI1(cin: super::super::Foundation::CHAR, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromI2(sin: i16, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromI8(ui64in: i64, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromR4(fltin: f32, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromR8(dblin: f64, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromStr(strin: super::super::Foundation::PWSTR, lcid: u32, dwflags: u32, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromUI1(bin: u8, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromUI2(uiin: u16, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromUI4(ulin: u32, pi64out: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUdateFromDate(datein: f64, dwflags: u32, pudateout: *mut UDATE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32, pbstrout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarXor(pvarleft: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarright: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeType(pvargdest: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarsrc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wflags: u16, vt: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeTypeEx(pvargdest: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarsrc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, lcid: u32, wflags: u16, vt: u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantClear(pvarg: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopy(pvargdest: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargsrc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopyInd(pvardest: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvargsrc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantInit(pvarg: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>);
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> i32;
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VectorFromBstr(bstr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsa: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
}
