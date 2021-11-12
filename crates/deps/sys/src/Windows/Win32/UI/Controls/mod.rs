#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub mod Dialogs;
#[cfg(feature = "Win32_UI_Controls_RichEdit")]
pub mod RichEdit;
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BeginBufferedAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut super::super::Graphics::Gdi::HDC, phdcto: *mut super::super::Graphics::Gdi::HDC) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BeginBufferedPaint(hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, phdc: *mut super::super::Graphics::Gdi::HDC) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPanningFeedback(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintClear(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    pub fn BufferedPaintInit() -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BufferedPaintRenderAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT, alpha: u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintStopAllAnimations(hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    pub fn BufferedPaintUnInit() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckDlgButton(hdlg: super::super::Foundation::HWND, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckRadioButton(hdlg: super::super::Foundation::HWND, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> super::super::Foundation::BOOL;
    pub fn CloseThemeData(htheme: isize) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateMappedBitmap(hinstance: super::super::Foundation::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: *const COLORMAP, inummaps: i32) -> super::super::Graphics::Gdi::HBITMAP;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStatusWindowA(style: i32, lpsztext: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, wid: u32) -> super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStatusWindowW(style: i32, lpsztext: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, wid: u32) -> super::super::Foundation::HWND;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateToolbarEx(hwnd: super::super::Foundation::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: super::super::Foundation::HINSTANCE, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: super::super::Foundation::HWND, nid: i32, hinst: super::super::Foundation::HINSTANCE, hbuddy: super::super::Foundation::HWND, nupper: i32, nlower: i32, npos: i32) -> super::super::Foundation::HWND;
    pub fn DPA_Clone(hdpa: HDPA, hdpanew: HDPA) -> HDPA;
    pub fn DPA_Create(citemgrow: i32) -> HDPA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_CreateEx(cpgrow: i32, hheap: super::super::Foundation::HANDLE) -> HDPA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_DeleteAllPtrs(hdpa: HDPA) -> super::super::Foundation::BOOL;
    pub fn DPA_DeletePtr(hdpa: HDPA, i: i32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Destroy(hdpa: HDPA) -> super::super::Foundation::BOOL;
    pub fn DPA_DestroyCallback(hdpa: HDPA, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void);
    pub fn DPA_EnumCallback(hdpa: HDPA, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void);
    pub fn DPA_GetPtr(hdpa: HDPA, i: isize) -> *mut ::core::ffi::c_void;
    pub fn DPA_GetPtrIndex(hdpa: HDPA, p: *const ::core::ffi::c_void) -> i32;
    pub fn DPA_GetSize(hdpa: HDPA) -> u64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Grow(pdpa: HDPA, cp: i32) -> super::super::Foundation::BOOL;
    pub fn DPA_InsertPtr(hdpa: HDPA, i: i32, p: *const ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DPA_LoadStream(phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: super::super::System::Com::IStream, pvinstdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Merge(hdpadest: HDPA, hdpasrc: HDPA, dwflags: u32, pfncompare: PFNDACOMPARE, pfnmerge: PFNDPAMERGE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DPA_SaveStream(hdpa: HDPA, pfn: PFNDPASTREAM, pstream: super::super::System::Com::IStream, pvinstdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Search(hdpa: HDPA, pfind: *const ::core::ffi::c_void, istart: i32, pfncompare: PFNDACOMPARE, lparam: super::super::Foundation::LPARAM, options: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_SetPtr(hdpa: HDPA, i: i32, p: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Sort(hdpa: HDPA, pfncompare: PFNDACOMPARE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    pub fn DSA_Clone(hdsa: HDSA) -> HDSA;
    pub fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_DeleteAllItems(hdsa: HDSA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_DeleteItem(hdsa: HDSA, i: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_Destroy(hdsa: HDSA) -> super::super::Foundation::BOOL;
    pub fn DSA_DestroyCallback(hdsa: HDSA, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void);
    pub fn DSA_EnumCallback(hdsa: HDSA, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_GetItem(hdsa: HDSA, i: i32, pitem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn DSA_GetItemPtr(hdsa: HDSA, i: i32) -> *mut ::core::ffi::c_void;
    pub fn DSA_GetSize(hdsa: HDSA) -> u64;
    pub fn DSA_InsertItem(hdsa: HDSA, i: i32, pitem: *const ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_SetItem(hdsa: HDSA, i: i32, pitem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_Sort(pdsa: HDSA, pfncompare: PFNDACOMPARE, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPropertySheetPage(param0: HPROPSHEETPAGE) -> super::super::Foundation::BOOL;
    pub fn DestroySyntheticPointerDevice(device: HSYNTHETICPOINTERDEVICE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListA(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListComboBoxA(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListComboBoxW(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListW(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectComboBoxExA(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectComboBoxExW(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectExA(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectExW(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawInsert(handparent: super::super::Foundation::HWND, hlb: super::super::Foundation::HWND, nitem: i32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawShadowText(hdc: super::super::Graphics::Gdi::HDC, psztext: super::super::Foundation::PWSTR, cch: u32, prc: *const super::super::Foundation::RECT, dwflags: u32, crtext: u32, crshadow: u32, ixoffset: i32, iyoffset: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawStatusTextA(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: super::super::Foundation::PSTR, uflags: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawStatusTextW(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: super::super::Foundation::PWSTR, uflags: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeBackground(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pcliprect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeBackgroundEx(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, poptions: *const DTBGOPTS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeEdge(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pdestrect: *const super::super::Foundation::RECT, uedge: u32, uflags: u32, pcontentrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeIcon(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, himl: HIMAGELIST, iimageindex: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeParentBackground(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, prc: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeParentBackgroundEx(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeText(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchtext: i32, dwtextflags: u32, dwtextflags2: u32, prect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeTextEx(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchtext: i32, dwtextflags: u32, prect: *mut super::super::Foundation::RECT, poptions: *const DTTOPTS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn EnableScrollBar(hwnd: super::super::Foundation::HWND, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableThemeDialogTexture(hwnd: super::super::Foundation::HWND, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableTheming(fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndBufferedAnimation(hbpanimation: isize, fupdatetarget: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndBufferedPaint(hbufferedpaint: isize, fupdatetarget: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPanningFeedback(hwnd: super::super::Foundation::HWND, fanimateback: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvaluateProximityToPolygon(numvertices: u32, controlpolygon: *const super::super::Foundation::POINT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvaluateProximityToRect(controlboundingbox: *const super::super::Foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_EnableScrollBar(param0: super::super::Foundation::HWND, param1: i32, param2: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_GetScrollProp(param0: super::super::Foundation::HWND, propindex: WSB_PROP, param2: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: super::super::Foundation::BOOL) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_SetScrollProp(param0: super::super::Foundation::HWND, index: WSB_PROP, newvalue: isize, param3: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: super::super::Foundation::BOOL) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_ShowScrollBar(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut super::super::Graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBufferedPaintTargetRect(hbufferedpaint: isize, prc: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComboBoxInfo(hwndcombo: super::super::Foundation::HWND, pcbi: *mut COMBOBOXINFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentThemeName(pszthemefilename: super::super::Foundation::PWSTR, cchmaxnamechars: i32, pszcolorbuff: super::super::Foundation::PWSTR, cchmaxcolorchars: i32, pszsizebuff: super::super::Foundation::PWSTR, cchmaxsizechars: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveClientRect(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT, lpinfo: *const i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetListBoxInfo(hwnd: super::super::Foundation::HWND) -> u32;
    pub fn GetMUILanguage() -> u16;
    pub fn GetThemeAnimationProperty(htheme: isize, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: *mut ::core::ffi::c_void, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeAnimationTransform(htheme: isize, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: *mut TA_TRANSFORM, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeAppProperties() -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundContentRect(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pboundingrect: *const super::super::Foundation::RECT, pcontentrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundExtent(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pcontentrect: *const super::super::Foundation::RECT, pextentrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundRegion(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pregion: *mut super::super::Graphics::Gdi::HRGN) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeBitmap(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeBool(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pfval: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeColor(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pcolor: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeDocumentationProperty(pszthemename: super::super::Foundation::PWSTR, pszpropertyname: super::super::Foundation::PWSTR, pszvaluebuff: super::super::Foundation::PWSTR, cchmaxvalchars: i32) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeEnumValue(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeFilename(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszthemefilename: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeFont(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, pfont: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeInt(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeIntList(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pintlist: *mut INTLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeMargins(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, prc: *const super::super::Foundation::RECT, pmargins: *mut MARGINS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeMetric(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pival: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemePartSize(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prc: *const super::super::Foundation::RECT, esize: THEMESIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemePosition(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppoint: *mut super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT;
    pub fn GetThemePropertyOrigin(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, porigin: *mut PROPERTYORIGIN) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeRect(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, prect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeStream(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::core::ffi::c_void, pcbstream: *mut u32, hinst: super::super::Foundation::HINSTANCE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeString(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeSysBool(htheme: isize, iboolid: i32) -> super::super::Foundation::BOOL;
    pub fn GetThemeSysColor(htheme: isize, icolorid: i32) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeSysColorBrush(htheme: isize, icolorid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Graphics::Gdi::HBRUSH;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeSysFont(htheme: isize, ifontid: THEME_PROPERTY_SYMBOL_ID, plf: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeSysInt(htheme: isize, iintid: i32, pivalue: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeSysSize(htheme: isize, isizeid: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeSysString(htheme: isize, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: super::super::Foundation::PWSTR, cchmaxstringchars: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeTextExtent(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchcharcount: i32, dwtextflags: u32, pboundingrect: *const super::super::Foundation::RECT, pextentrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeTextMetrics(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeTimingFunction(htheme: isize, itimingfunctionid: i32, ptimingfunction: *mut TA_TIMINGFUNCTION, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetThemeTransitionDuration(htheme: isize, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32, pdwduration: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTheme(hwnd: super::super::Foundation::HWND) -> isize;
    pub fn HIMAGELIST_QueryInterface(himl: HIMAGELIST, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn HitTestThemeBackground(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, dwoptions: u32, prect: *const super::super::Foundation::RECT, hrgn: super::super::Graphics::Gdi::HRGN, pttest: super::super::Foundation::POINT, pwhittestcode: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ImageList_Add(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ImageList_AddMasked(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_BeginDrag(himltrack: HIMAGELIST, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL;
    pub fn ImageList_CoCreateInstance(rclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Copy(himldst: HIMAGELIST, idst: i32, himlsrc: HIMAGELIST, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> super::super::Foundation::BOOL;
    pub fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Destroy(himl: HIMAGELIST) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragEnter(hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragLeave(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragMove(x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragShowNolock(fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_Draw(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_DrawEx(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: u32, rgbfg: u32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> super::super::Foundation::BOOL;
    pub fn ImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST;
    pub fn ImageList_EndDrag();
    pub fn ImageList_GetBkColor(himl: HIMAGELIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_GetDragImage(ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT) -> HIMAGELIST;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn ImageList_GetIcon(himl: HIMAGELIST, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_GetIconSize(himl: HIMAGELIST, cx: *mut i32, cy: *mut i32) -> super::super::Foundation::BOOL;
    pub fn ImageList_GetImageCount(himl: HIMAGELIST) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_GetImageInfo(himl: HIMAGELIST, i: i32, pimageinfo: *mut IMAGEINFO) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ImageList_LoadImageA(hi: super::super::Foundation::HINSTANCE, lpbmp: super::super::Foundation::PSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ImageList_LoadImageW(hi: super::super::Foundation::HINSTANCE, lpbmp: super::super::Foundation::PWSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
    pub fn ImageList_Merge(himl1: HIMAGELIST, i1: i32, himl2: HIMAGELIST, i2: i32, dx: i32, dy: i32) -> HIMAGELIST;
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_Read(pstm: super::super::System::Com::IStream) -> HIMAGELIST;
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_ReadEx(dwflags: u32, pstm: super::super::System::Com::IStream, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Remove(himl: HIMAGELIST, i: i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_Replace(himl: HIMAGELIST, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn ImageList_ReplaceIcon(himl: HIMAGELIST, i: i32, hicon: super::WindowsAndMessaging::HICON) -> i32;
    pub fn ImageList_SetBkColor(himl: HIMAGELIST, clrbk: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetDragCursorImage(himldrag: HIMAGELIST, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetIconSize(himl: HIMAGELIST, cx: i32, cy: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetImageCount(himl: HIMAGELIST, unewcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetOverlayImage(himl: HIMAGELIST, iimage: i32, ioverlay: i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ImageList_Write(himl: HIMAGELIST, pstm: super::super::System::Com::IStream) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_WriteEx(himl: HIMAGELIST, dwflags: u32, pstm: super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    pub fn InitCommonControls();
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> super::super::Foundation::BOOL;
    pub fn InitMUILanguage(uilang: u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeFlatSB(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsAppThemed() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharLowerW(ch: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCompositionActive() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDlgButtonChecked(hdlg: super::super::Foundation::HWND, nidbutton: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeActive() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeBackgroundPartiallyTransparent(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeDialogTextureEnabled(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemePartDefined(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LBItemFromPt(hlb: super::super::Foundation::HWND, pt: super::super::Foundation::POINT, bautoscroll: super::super::Foundation::BOOL) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn LoadIconMetric(hinst: super::super::Foundation::HINSTANCE, pszname: super::super::Foundation::PWSTR, lims: _LI_METRIC, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn LoadIconWithScaleDown(hinst: super::super::Foundation::HINSTANCE, pszname: super::super::Foundation::PWSTR, cx: i32, cy: i32, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeDragList(hlb: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn MenuHelp(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, hmainmenu: super::WindowsAndMessaging::HMENU, hinst: super::super::Foundation::HINSTANCE, hwndstatus: super::super::Foundation::HWND, lpwids: *const u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeData(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeDataEx(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR, dwflags: OPEN_THEME_DATA_FLAGS) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::LRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn PropertySheetA(param0: *mut PROPSHEETHEADERA_V2) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn PropertySheetW(param0: *mut PROPSHEETHEADERW_V2) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterPointerDeviceNotifications(window: super::super::Foundation::HWND, notifyrange: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTouchHitTestingWindow(hwnd: super::super::Foundation::HWND, value: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollInfo(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: super::super::Foundation::BOOL) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollPos(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: super::super::Foundation::BOOL) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollRange(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    pub fn SetThemeAppProperties(dwflags: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTheme(hwnd: super::super::Foundation::HWND, pszsubappname: super::super::Foundation::PWSTR, pszsubidlist: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowThemeAttribute(hwnd: super::super::Foundation::HWND, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowHideMenuCtl(hwnd: super::super::Foundation::HWND, uflags: usize, lpinfo: *const i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShowScrollBar(hwnd: super::super::Foundation::HWND, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Str_SetPtrW(ppsz: *mut super::super::Foundation::PWSTR, psz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TaskDialog(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszwindowtitle: super::super::Foundation::PWSTR, pszmaininstruction: super::super::Foundation::PWSTR, pszcontent: super::super::Foundation::PWSTR, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: super::super::Foundation::PWSTR, pnbutton: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: *mut i32, pnradiobutton: *mut i32, pfverificationflagchecked: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninitializeFlatSB(param0: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePanningFeedback(hwnd: super::super::Foundation::HWND, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
pub const ACM_ISPLAYING: u32 = 1128u32;
pub const ACM_OPEN: u32 = 1127u32;
pub const ACM_OPENA: u32 = 1124u32;
pub const ACM_OPENW: u32 = 1127u32;
pub const ACM_PLAY: u32 = 1125u32;
pub const ACM_STOP: u32 = 1126u32;
pub const ACN_START: u32 = 1u32;
pub const ACN_STOP: u32 = 2u32;
pub const ACS_AUTOPLAY: u32 = 4u32;
pub const ACS_CENTER: u32 = 1u32;
pub const ACS_TIMER: u32 = 8u32;
pub const ACS_TRANSPARENT: u32 = 2u32;
pub const BCM_FIRST: u32 = 5632u32;
pub const BCM_GETIDEALSIZE: u32 = 5633u32;
pub const BCM_GETIMAGELIST: u32 = 5635u32;
pub const BCM_GETNOTE: u32 = 5642u32;
pub const BCM_GETNOTELENGTH: u32 = 5643u32;
pub const BCM_GETSPLITINFO: u32 = 5640u32;
pub const BCM_GETTEXTMARGIN: u32 = 5637u32;
pub const BCM_SETDROPDOWNSTATE: u32 = 5638u32;
pub const BCM_SETIMAGELIST: u32 = 5634u32;
pub const BCM_SETNOTE: u32 = 5641u32;
pub const BCM_SETSHIELD: u32 = 5644u32;
pub const BCM_SETSPLITINFO: u32 = 5639u32;
pub const BCM_SETTEXTMARGIN: u32 = 5636u32;
pub const BCN_DROPDOWN: u32 = 4294966048u32;
pub const BCN_FIRST: u32 = 4294966046u32;
pub const BCN_HOTITEMCHANGE: u32 = 4294966047u32;
pub const BCSIF_GLYPH: u32 = 1u32;
pub const BCSIF_IMAGE: u32 = 2u32;
pub const BCSIF_SIZE: u32 = 8u32;
pub const BCSIF_STYLE: u32 = 4u32;
pub const BCSS_ALIGNLEFT: u32 = 4u32;
pub const BCSS_IMAGE: u32 = 8u32;
pub const BCSS_NOSPLIT: u32 = 1u32;
pub const BCSS_STRETCH: u32 = 2u32;
#[repr(transparent)]
pub struct BGTYPE(pub i32);
pub const BT_IMAGEFILE: BGTYPE = BGTYPE(0i32);
pub const BT_BORDERFILL: BGTYPE = BGTYPE(1i32);
pub const BT_NONE: BGTYPE = BGTYPE(2i32);
#[repr(transparent)]
pub struct BORDERTYPE(pub i32);
pub const BT_RECT: BORDERTYPE = BORDERTYPE(0i32);
pub const BT_ROUNDRECT: BORDERTYPE = BORDERTYPE(1i32);
pub const BT_ELLIPSE: BORDERTYPE = BORDERTYPE(2i32);
#[repr(C)]
pub struct BP_ANIMATIONPARAMS(i32);
#[repr(transparent)]
pub struct BP_ANIMATIONSTYLE(pub i32);
pub const BPAS_NONE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(0i32);
pub const BPAS_LINEAR: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(1i32);
pub const BPAS_CUBIC: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(2i32);
pub const BPAS_SINE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(3i32);
#[repr(transparent)]
pub struct BP_BUFFERFORMAT(pub i32);
pub const BPBF_COMPATIBLEBITMAP: BP_BUFFERFORMAT = BP_BUFFERFORMAT(0i32);
pub const BPBF_DIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(1i32);
pub const BPBF_TOPDOWNDIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(2i32);
pub const BPBF_TOPDOWNMONODIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(3i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct BP_PAINTPARAMS(i32);
#[repr(transparent)]
pub struct BP_PAINTPARAMS_FLAGS(pub u32);
pub const BPPF_ERASE: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(1u32);
pub const BPPF_NOCLIP: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(2u32);
pub const BPPF_NONCLIENT: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(4u32);
pub const BST_DROPDOWNPUSHED: u32 = 1024u32;
pub const BST_HOT: u32 = 512u32;
pub const BS_COMMANDLINK: i32 = 14i32;
pub const BS_DEFCOMMANDLINK: i32 = 15i32;
pub const BS_DEFSPLITBUTTON: i32 = 13i32;
pub const BS_SPLITBUTTON: i32 = 12i32;
pub const BTNS_AUTOSIZE: u32 = 16u32;
pub const BTNS_BUTTON: u32 = 0u32;
pub const BTNS_CHECK: u32 = 2u32;
pub const BTNS_DROPDOWN: u32 = 8u32;
pub const BTNS_GROUP: u32 = 4u32;
pub const BTNS_NOPREFIX: u32 = 32u32;
pub const BTNS_SEP: u32 = 1u32;
pub const BTNS_SHOWTEXT: u32 = 64u32;
pub const BTNS_WHOLEDROPDOWN: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct BUTTON_IMAGELIST(i32);
#[repr(transparent)]
pub struct BUTTON_IMAGELIST_ALIGN(pub u32);
pub const BUTTON_IMAGELIST_ALIGN_LEFT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(0u32);
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(1u32);
pub const BUTTON_IMAGELIST_ALIGN_TOP: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(2u32);
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(3u32);
pub const BUTTON_IMAGELIST_ALIGN_CENTER: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(4u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct BUTTON_SPLITINFO(i32);
pub const CBEMAXSTRLEN: u32 = 260u32;
pub const CBEM_GETCOMBOCONTROL: u32 = 1030u32;
pub const CBEM_GETEDITCONTROL: u32 = 1031u32;
pub const CBEM_GETEXSTYLE: u32 = 1033u32;
pub const CBEM_GETEXTENDEDSTYLE: u32 = 1033u32;
pub const CBEM_GETIMAGELIST: u32 = 1027u32;
pub const CBEM_GETITEM: u32 = 1037u32;
pub const CBEM_GETITEMA: u32 = 1028u32;
pub const CBEM_GETITEMW: u32 = 1037u32;
pub const CBEM_GETUNICODEFORMAT: u32 = 8198u32;
pub const CBEM_HASEDITCHANGED: u32 = 1034u32;
pub const CBEM_INSERTITEM: u32 = 1035u32;
pub const CBEM_INSERTITEMA: u32 = 1025u32;
pub const CBEM_INSERTITEMW: u32 = 1035u32;
pub const CBEM_SETEXSTYLE: u32 = 1032u32;
pub const CBEM_SETEXTENDEDSTYLE: u32 = 1038u32;
pub const CBEM_SETIMAGELIST: u32 = 1026u32;
pub const CBEM_SETITEM: u32 = 1036u32;
pub const CBEM_SETITEMA: u32 = 1029u32;
pub const CBEM_SETITEMW: u32 = 1036u32;
pub const CBEM_SETUNICODEFORMAT: u32 = 8197u32;
pub const CBEM_SETWINDOWTHEME: u32 = 8203u32;
pub const CBENF_DROPDOWN: u32 = 4u32;
pub const CBENF_ESCAPE: u32 = 3u32;
pub const CBENF_KILLFOCUS: u32 = 1u32;
pub const CBENF_RETURN: u32 = 2u32;
pub const CBES_EX_CASESENSITIVE: u32 = 16u32;
pub const CBES_EX_NOEDITIMAGE: u32 = 1u32;
pub const CBES_EX_NOEDITIMAGEINDENT: u32 = 2u32;
pub const CBES_EX_NOSIZELIMIT: u32 = 8u32;
pub const CBES_EX_PATHWORDBREAKPROC: u32 = 4u32;
pub const CBES_EX_TEXTENDELLIPSIS: u32 = 32u32;
pub const CBM_FIRST: u32 = 5888u32;
pub const CB_GETCUEBANNER: u32 = 5892u32;
pub const CB_GETMINVISIBLE: u32 = 5890u32;
pub const CB_SETCUEBANNER: u32 = 5891u32;
pub const CB_SETMINVISIBLE: u32 = 5889u32;
pub const CCF_NOTEXT: u32 = 1u32;
pub const CCHCCCLASS: u32 = 32u32;
pub const CCHCCDESC: u32 = 32u32;
pub const CCHCCTEXT: u32 = 256u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct CCINFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct CCINFOW(i32);
pub const CCM_DPISCALE: u32 = 8204u32;
pub const CCM_FIRST: u32 = 8192u32;
pub const CCM_GETCOLORSCHEME: u32 = 8195u32;
pub const CCM_GETDROPTARGET: u32 = 8196u32;
pub const CCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const CCM_GETVERSION: u32 = 8200u32;
pub const CCM_LAST: u32 = 8704u32;
pub const CCM_SETBKCOLOR: u32 = 8193u32;
pub const CCM_SETCOLORSCHEME: u32 = 8194u32;
pub const CCM_SETNOTIFYWINDOW: u32 = 8201u32;
pub const CCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const CCM_SETVERSION: u32 = 8199u32;
pub const CCM_SETWINDOWTHEME: u32 = 8203u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CCSTYLEA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CCSTYLEFLAGA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CCSTYLEFLAGW(i32);
#[repr(C)]
pub struct CCSTYLEW(i32);
pub const CCS_ADJUSTABLE: i32 = 32i32;
pub const CCS_BOTTOM: i32 = 3i32;
pub const CCS_NODIVIDER: i32 = 64i32;
pub const CCS_NOMOVEY: i32 = 2i32;
pub const CCS_NOPARENTALIGN: i32 = 8i32;
pub const CCS_NORESIZE: i32 = 4i32;
pub const CCS_TOP: i32 = 1i32;
pub const CCS_VERT: i32 = 128i32;
pub const CDDS_ITEM: u32 = 65536u32;
pub const CDDS_POSTERASE: u32 = 4u32;
pub const CDIS_CHECKED: u32 = 8u32;
pub const CDIS_DEFAULT: u32 = 32u32;
pub const CDIS_DISABLED: u32 = 4u32;
pub const CDIS_DROPHILITED: u32 = 4096u32;
pub const CDIS_FOCUS: u32 = 16u32;
pub const CDIS_GRAYED: u32 = 2u32;
pub const CDIS_HOT: u32 = 64u32;
pub const CDIS_INDETERMINATE: u32 = 256u32;
pub const CDIS_MARKED: u32 = 128u32;
pub const CDIS_NEARHOT: u32 = 1024u32;
pub const CDIS_OTHERSIDEHOT: u32 = 2048u32;
pub const CDIS_SELECTED: u32 = 1u32;
pub const CDIS_SHOWKEYBOARDCUES: u32 = 512u32;
pub const CDRF_DODEFAULT: u32 = 0u32;
pub const CDRF_DOERASE: u32 = 8u32;
pub const CDRF_NEWFONT: u32 = 2u32;
pub const CDRF_NOTIFYITEMDRAW: u32 = 32u32;
pub const CDRF_NOTIFYPOSTERASE: u32 = 64u32;
pub const CDRF_NOTIFYPOSTPAINT: u32 = 16u32;
pub const CDRF_NOTIFYSUBITEMDRAW: u32 = 32u32;
pub const CDRF_SKIPDEFAULT: u32 = 4u32;
pub const CDRF_SKIPPOSTPAINT: u32 = 256u32;
#[repr(transparent)]
pub struct CLOCKPARTS(pub i32);
pub const CLP_TIME: CLOCKPARTS = CLOCKPARTS(1i32);
#[repr(transparent)]
pub struct CLOCKSTATES(pub i32);
pub const CLS_NORMAL: CLOCKSTATES = CLOCKSTATES(1i32);
pub const CLS_HOT: CLOCKSTATES = CLOCKSTATES(2i32);
pub const CLS_PRESSED: CLOCKSTATES = CLOCKSTATES(3i32);
pub const CLR_DEFAULT: i32 = -16777216i32;
pub const CLR_HILIGHT: i32 = -16777216i32;
pub const CLR_NONE: i32 = -1i32;
pub const CMB_MASKED: u32 = 2u32;
#[repr(C)]
pub struct COLORMAP(i32);
pub const COLORMGMTDLGORD: u32 = 1551u32;
#[repr(C)]
pub struct COLORSCHEME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMBOBOXEXITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMBOBOXEXITEMW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMBOBOXINFO(i32);
#[repr(transparent)]
pub struct COMBOBOXINFO_BUTTON_STATE(pub u32);
pub const STATE_SYSTEM_INVISIBLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(32768u32);
pub const STATE_SYSTEM_PRESSED: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(8u32);
pub const STATE_SYSTEM_FOCUSABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1048576u32);
pub const STATE_SYSTEM_OFFSCREEN: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(65536u32);
pub const STATE_SYSTEM_UNAVAILABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1u32);
#[repr(transparent)]
pub struct COMBOBOX_EX_ITEM_FLAGS(pub u32);
pub const CBEIF_DI_SETITEM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(268435456u32);
pub const CBEIF_IMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(2u32);
pub const CBEIF_INDENT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(16u32);
pub const CBEIF_LPARAM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(32u32);
pub const CBEIF_OVERLAY: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(8u32);
pub const CBEIF_SELECTEDIMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(4u32);
pub const CBEIF_TEXT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(1u32);
pub const COMCTL32_VERSION: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COMPAREITEMSTRUCT(i32);
#[repr(transparent)]
pub struct CONTENTALIGNMENT(pub i32);
pub const CA_LEFT: CONTENTALIGNMENT = CONTENTALIGNMENT(0i32);
pub const CA_CENTER: CONTENTALIGNMENT = CONTENTALIGNMENT(1i32);
pub const CA_RIGHT: CONTENTALIGNMENT = CONTENTALIGNMENT(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DATETIMEPICKERINFO(i32);
pub const DA_ERR: i32 = -1i32;
pub const DA_LAST: u32 = 2147483647u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DELETEITEMSTRUCT(i32);
#[repr(transparent)]
pub struct DLG_BUTTON_CHECK_STATE(pub u32);
pub const BST_CHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(1u32);
pub const BST_INDETERMINATE: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(2u32);
pub const BST_UNCHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(0u32);
#[repr(transparent)]
pub struct DLG_DIR_LIST_FILE_TYPE(pub u32);
pub const DDL_ARCHIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32u32);
pub const DDL_DIRECTORY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16u32);
pub const DDL_DRIVES: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16384u32);
pub const DDL_EXCLUSIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32768u32);
pub const DDL_HIDDEN: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(2u32);
pub const DDL_READONLY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(1u32);
pub const DDL_READWRITE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(0u32);
pub const DDL_SYSTEM: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(4u32);
pub const DDL_POSTMSGS: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(8192u32);
pub const DL_COPYCURSOR: u32 = 2u32;
pub const DL_CURSORSET: u32 = 0u32;
pub const DL_MOVECURSOR: u32 = 3u32;
pub const DL_STOPCURSOR: u32 = 1u32;
#[repr(transparent)]
pub struct DPAMM_MESSAGE(pub u32);
pub const DPAMM_MERGE: DPAMM_MESSAGE = DPAMM_MESSAGE(1u32);
pub const DPAMM_DELETE: DPAMM_MESSAGE = DPAMM_MESSAGE(2u32);
pub const DPAMM_INSERT: DPAMM_MESSAGE = DPAMM_MESSAGE(3u32);
pub const DPAM_INTERSECT: u32 = 8u32;
pub const DPAM_NORMAL: u32 = 2u32;
pub const DPAM_SORTED: u32 = 1u32;
pub const DPAM_UNION: u32 = 4u32;
#[repr(C)]
pub struct DPASTREAMINFO(i32);
pub const DPAS_INSERTAFTER: u32 = 4u32;
pub const DPAS_INSERTBEFORE: u32 = 2u32;
pub const DPAS_SORTED: u32 = 1u32;
pub const DPA_APPEND: u32 = 2147483647u32;
pub const DPA_ERR: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRAGLISTINFO(i32);
#[repr(transparent)]
pub struct DRAGLISTINFO_NOTIFICATION_FLAGS(pub u32);
pub const DL_BEGINDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1157u32);
pub const DL_CANCELDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1160u32);
pub const DL_DRAGGING: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1158u32);
pub const DL_DROPPED: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1159u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DRAWITEMSTRUCT(i32);
#[repr(transparent)]
pub struct DRAWITEMSTRUCT_CTL_TYPE(pub u32);
pub const ODT_BUTTON: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(4u32);
pub const ODT_COMBOBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(3u32);
pub const ODT_LISTBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(2u32);
pub const ODT_LISTVIEW: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(102u32);
pub const ODT_MENU: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(1u32);
pub const ODT_STATIC: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(5u32);
pub const ODT_TAB: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(101u32);
#[repr(transparent)]
pub struct DRAW_THEME_PARENT_BACKGROUND_FLAGS(pub u32);
pub const DTPB_WINDOWDC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(1u32);
pub const DTPB_USECTLCOLORSTATIC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(2u32);
pub const DTPB_USEERASEBKGND: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(4u32);
pub const DSA_APPEND: u32 = 2147483647u32;
pub const DSA_ERR: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DTBGOPTS(i32);
pub const DTBG_CLIPRECT: u32 = 1u32;
pub const DTBG_COMPUTINGREGION: u32 = 16u32;
pub const DTBG_DRAWSOLID: u32 = 2u32;
pub const DTBG_MIRRORDC: u32 = 32u32;
pub const DTBG_NOMIRROR: u32 = 64u32;
pub const DTBG_OMITBORDER: u32 = 4u32;
pub const DTBG_OMITCONTENT: u32 = 8u32;
pub const DTM_CLOSEMONTHCAL: u32 = 4109u32;
pub const DTM_FIRST: u32 = 4096u32;
pub const DTM_GETDATETIMEPICKERINFO: u32 = 4110u32;
pub const DTM_GETIDEALSIZE: u32 = 4111u32;
pub const DTM_GETMCCOLOR: u32 = 4103u32;
pub const DTM_GETMCFONT: u32 = 4106u32;
pub const DTM_GETMCSTYLE: u32 = 4108u32;
pub const DTM_GETMONTHCAL: u32 = 4104u32;
pub const DTM_GETRANGE: u32 = 4099u32;
pub const DTM_GETSYSTEMTIME: u32 = 4097u32;
pub const DTM_SETFORMAT: u32 = 4146u32;
pub const DTM_SETFORMATA: u32 = 4101u32;
pub const DTM_SETFORMATW: u32 = 4146u32;
pub const DTM_SETMCCOLOR: u32 = 4102u32;
pub const DTM_SETMCFONT: u32 = 4105u32;
pub const DTM_SETMCSTYLE: u32 = 4107u32;
pub const DTM_SETRANGE: u32 = 4100u32;
pub const DTM_SETSYSTEMTIME: u32 = 4098u32;
pub const DTS_APPCANPARSE: u32 = 16u32;
pub const DTS_LONGDATEFORMAT: u32 = 4u32;
pub const DTS_RIGHTALIGN: u32 = 32u32;
pub const DTS_SHORTDATECENTURYFORMAT: u32 = 12u32;
pub const DTS_SHORTDATEFORMAT: u32 = 0u32;
pub const DTS_SHOWNONE: u32 = 2u32;
pub const DTS_TIMEFORMAT: u32 = 9u32;
pub const DTS_UPDOWN: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DTTOPTS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type DTT_CALLBACK_PROC = unsafe extern "system" fn(hdc: super::super::Graphics::Gdi::HDC, psztext: super::super::Foundation::PWSTR, cchtext: i32, prc: *mut super::super::Foundation::RECT, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> i32;
pub const DTT_FLAGS2VALIDBITS: u32 = 1u32;
pub const DTT_GRAYED: u32 = 1u32;
pub const ECM_FIRST: u32 = 5376u32;
#[repr(transparent)]
pub struct EC_ENDOFLINE(pub i32);
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = EC_ENDOFLINE(0i32);
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = EC_ENDOFLINE(1i32);
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = EC_ENDOFLINE(2i32);
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = EC_ENDOFLINE(3i32);
#[repr(transparent)]
pub struct EC_SEARCHWEB_ENTRYPOINT(pub i32);
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(0i32);
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EDITBALLOONTIP(i32);
#[repr(transparent)]
pub struct EDITBALLOONTIP_ICON(pub u32);
pub const TTI_ERROR: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(3u32);
pub const TTI_INFO: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(1u32);
pub const TTI_NONE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(0u32);
pub const TTI_WARNING: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(2u32);
pub const TTI_INFO_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(4u32);
pub const TTI_WARNING_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(5u32);
pub const TTI_ERROR_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(6u32);
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCA = unsafe extern "system" fn(lpch: super::super::Foundation::PSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCW = unsafe extern "system" fn(lpch: super::super::Foundation::PWSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32;
#[repr(transparent)]
pub struct EMPTYMARKUPPARTS(pub i32);
pub const EMP_MARKUPTEXT: EMPTYMARKUPPARTS = EMPTYMARKUPPARTS(1i32);
pub const EM_CANUNDO: u32 = 198u32;
pub const EM_CHARFROMPOS: u32 = 215u32;
pub const EM_EMPTYUNDOBUFFER: u32 = 205u32;
pub const EM_ENABLEFEATURE: u32 = 218u32;
pub const EM_ENABLESEARCHWEB: u32 = 5390u32;
pub const EM_FILELINEFROMCHAR: u32 = 5395u32;
pub const EM_FILELINEINDEX: u32 = 5396u32;
pub const EM_FILELINELENGTH: u32 = 5397u32;
pub const EM_FMTLINES: u32 = 200u32;
pub const EM_GETCARETINDEX: u32 = 5394u32;
pub const EM_GETCUEBANNER: u32 = 5378u32;
pub const EM_GETENDOFLINE: u32 = 5389u32;
pub const EM_GETEXTENDEDSTYLE: u32 = 5387u32;
pub const EM_GETFILELINE: u32 = 5398u32;
pub const EM_GETFILELINECOUNT: u32 = 5399u32;
pub const EM_GETFIRSTVISIBLELINE: u32 = 206u32;
pub const EM_GETHANDLE: u32 = 189u32;
pub const EM_GETHILITE: u32 = 5382u32;
pub const EM_GETIMESTATUS: u32 = 217u32;
pub const EM_GETLIMITTEXT: u32 = 213u32;
pub const EM_GETLINE: u32 = 196u32;
pub const EM_GETLINECOUNT: u32 = 186u32;
pub const EM_GETMARGINS: u32 = 212u32;
pub const EM_GETMODIFY: u32 = 184u32;
pub const EM_GETPASSWORDCHAR: u32 = 210u32;
pub const EM_GETRECT: u32 = 178u32;
pub const EM_GETSEL: u32 = 176u32;
pub const EM_GETTHUMB: u32 = 190u32;
pub const EM_GETWORDBREAKPROC: u32 = 209u32;
pub const EM_HIDEBALLOONTIP: u32 = 5380u32;
pub const EM_LIMITTEXT: u32 = 197u32;
pub const EM_LINEFROMCHAR: u32 = 201u32;
pub const EM_LINEINDEX: u32 = 187u32;
pub const EM_LINELENGTH: u32 = 193u32;
pub const EM_LINESCROLL: u32 = 182u32;
pub const EM_NOSETFOCUS: u32 = 5383u32;
pub const EM_POSFROMCHAR: u32 = 214u32;
pub const EM_REPLACESEL: u32 = 194u32;
pub const EM_SCROLL: u32 = 181u32;
pub const EM_SCROLLCARET: u32 = 183u32;
pub const EM_SEARCHWEB: u32 = 5391u32;
pub const EM_SETCARETINDEX: u32 = 5393u32;
pub const EM_SETCUEBANNER: u32 = 5377u32;
pub const EM_SETENDOFLINE: u32 = 5388u32;
pub const EM_SETEXTENDEDSTYLE: u32 = 5386u32;
pub const EM_SETHANDLE: u32 = 188u32;
pub const EM_SETHILITE: u32 = 5381u32;
pub const EM_SETIMESTATUS: u32 = 216u32;
pub const EM_SETLIMITTEXT: u32 = 197u32;
pub const EM_SETMARGINS: u32 = 211u32;
pub const EM_SETMODIFY: u32 = 185u32;
pub const EM_SETPASSWORDCHAR: u32 = 204u32;
pub const EM_SETREADONLY: u32 = 207u32;
pub const EM_SETRECT: u32 = 179u32;
pub const EM_SETRECTNP: u32 = 180u32;
pub const EM_SETSEL: u32 = 177u32;
pub const EM_SETTABSTOPS: u32 = 203u32;
pub const EM_SETWORDBREAKPROC: u32 = 208u32;
pub const EM_SHOWBALLOONTIP: u32 = 5379u32;
pub const EM_TAKEFOCUS: u32 = 5384u32;
pub const EM_UNDO: u32 = 199u32;
#[repr(transparent)]
pub struct ENABLE_SCROLL_BAR_ARROWS(pub u32);
pub const ESB_DISABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(3u32);
pub const ESB_DISABLE_DOWN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_LEFT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_LTUP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_RIGHT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_RTDN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_UP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_ENABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(0u32);
pub const ES_EX_ALLOWEOL_CR: i32 = 1i32;
pub const ES_EX_ALLOWEOL_LF: i32 = 2i32;
pub const ES_EX_CONVERT_EOL_ON_PASTE: i32 = 4i32;
pub const ES_EX_ZOOMABLE: i32 = 16i32;
pub const ETDT_DISABLE: u32 = 1u32;
pub const ETDT_ENABLE: u32 = 2u32;
pub const ETDT_USEAEROWIZARDTABTEXTURE: u32 = 8u32;
pub const ETDT_USETABTEXTURE: u32 = 4u32;
#[repr(transparent)]
pub struct FEEDBACK_TYPE(pub i32);
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(1i32);
pub const FEEDBACK_PEN_BARRELVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(2i32);
pub const FEEDBACK_PEN_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(3i32);
pub const FEEDBACK_PEN_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(4i32);
pub const FEEDBACK_PEN_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(5i32);
pub const FEEDBACK_PEN_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(6i32);
pub const FEEDBACK_TOUCH_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(7i32);
pub const FEEDBACK_TOUCH_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(8i32);
pub const FEEDBACK_TOUCH_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(9i32);
pub const FEEDBACK_TOUCH_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(10i32);
pub const FEEDBACK_GESTURE_PRESSANDTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(11i32);
pub const FEEDBACK_MAX: FEEDBACK_TYPE = FEEDBACK_TYPE(-1i32);
pub const FILEOPENORD: u32 = 1536u32;
#[repr(transparent)]
pub struct FILLTYPE(pub i32);
pub const FT_SOLID: FILLTYPE = FILLTYPE(0i32);
pub const FT_VERTGRADIENT: FILLTYPE = FILLTYPE(1i32);
pub const FT_HORZGRADIENT: FILLTYPE = FILLTYPE(2i32);
pub const FT_RADIALGRADIENT: FILLTYPE = FILLTYPE(3i32);
pub const FT_TILEIMAGE: FILLTYPE = FILLTYPE(4i32);
pub const FINDDLGORD: u32 = 1540u32;
pub const FONTDLGORD: u32 = 1542u32;
pub const FORMATDLGORD30: u32 = 1544u32;
pub const FORMATDLGORD31: u32 = 1543u32;
pub const FSB_ENCARTA_MODE: u32 = 1u32;
pub const FSB_FLAT_MODE: u32 = 2u32;
pub const FSB_REGULAR_MODE: u32 = 0u32;
pub const GDTR_MAX: u32 = 2u32;
pub const GDTR_MIN: u32 = 1u32;
pub const GDT_ERROR: i32 = -1i32;
pub const GDT_NONE: u32 = 1u32;
pub const GDT_VALID: u32 = 0u32;
#[repr(transparent)]
pub struct GET_THEME_BITMAP_FLAGS(pub u32);
pub const GBF_DIRECT: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(1u32);
pub const GBF_COPY: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(2u32);
pub const GBF_VALIDBITS: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(3u32);
#[repr(transparent)]
pub struct GLYPHFONTSIZINGTYPE(pub i32);
pub const GFST_NONE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(0i32);
pub const GFST_SIZE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(1i32);
pub const GFST_DPI: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(2i32);
#[repr(transparent)]
pub struct GLYPHTYPE(pub i32);
pub const GT_NONE: GLYPHTYPE = GLYPHTYPE(0i32);
pub const GT_IMAGEGLYPH: GLYPHTYPE = GLYPHTYPE(1i32);
pub const GT_FONTGLYPH: GLYPHTYPE = GLYPHTYPE(2i32);
pub const GMR_DAYSTATE: u32 = 1u32;
pub const GMR_VISIBLE: u32 = 0u32;
#[repr(transparent)]
pub struct GRIDCELLBACKGROUNDSTATES(pub i32);
pub const MCGCB_SELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(1i32);
pub const MCGCB_HOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(2i32);
pub const MCGCB_SELECTEDHOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(3i32);
pub const MCGCB_SELECTEDNOTFOCUSED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(4i32);
pub const MCGCB_TODAY: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(5i32);
pub const MCGCB_TODAYSELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(6i32);
#[repr(transparent)]
pub struct GRIDCELLSTATES(pub i32);
pub const MCGC_HOT: GRIDCELLSTATES = GRIDCELLSTATES(1i32);
pub const MCGC_HASSTATE: GRIDCELLSTATES = GRIDCELLSTATES(2i32);
pub const MCGC_HASSTATEHOT: GRIDCELLSTATES = GRIDCELLSTATES(3i32);
pub const MCGC_TODAY: GRIDCELLSTATES = GRIDCELLSTATES(4i32);
pub const MCGC_TODAYSELECTED: GRIDCELLSTATES = GRIDCELLSTATES(5i32);
pub const MCGC_SELECTED: GRIDCELLSTATES = GRIDCELLSTATES(6i32);
pub const MCGC_SELECTEDHOT: GRIDCELLSTATES = GRIDCELLSTATES(7i32);
#[repr(transparent)]
pub struct GRIDCELLUPPERSTATES(pub i32);
pub const MCGCU_HOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(1i32);
pub const MCGCU_HASSTATE: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(2i32);
pub const MCGCU_HASSTATEHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(3i32);
pub const MCGCU_SELECTED: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(4i32);
pub const MCGCU_SELECTEDHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(5i32);
#[repr(transparent)]
pub struct HALIGN(pub i32);
pub const HA_LEFT: HALIGN = HALIGN(0i32);
pub const HA_CENTER: HALIGN = HALIGN(1i32);
pub const HA_RIGHT: HALIGN = HALIGN(2i32);
pub const HDFT_HASNOVALUE: u32 = 32768u32;
pub const HDFT_ISDATE: u32 = 2u32;
pub const HDFT_ISNUMBER: u32 = 1u32;
pub const HDFT_ISSTRING: u32 = 0u32;
pub const HDF_BITMAP: u32 = 8192u32;
pub const HDF_BITMAP_ON_RIGHT: u32 = 4096u32;
pub const HDF_CENTER: u32 = 2u32;
pub const HDF_CHECKBOX: u32 = 64u32;
pub const HDF_CHECKED: u32 = 128u32;
pub const HDF_FIXEDWIDTH: u32 = 256u32;
pub const HDF_IMAGE: u32 = 2048u32;
pub const HDF_JUSTIFYMASK: u32 = 3u32;
pub const HDF_LEFT: u32 = 0u32;
pub const HDF_OWNERDRAW: u32 = 32768u32;
pub const HDF_RIGHT: u32 = 1u32;
pub const HDF_RTLREADING: u32 = 4u32;
pub const HDF_SORTDOWN: u32 = 512u32;
pub const HDF_SORTUP: u32 = 1024u32;
pub const HDF_SPLITBUTTON: u32 = 16777216u32;
pub const HDF_STRING: u32 = 16384u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HDHITTESTINFO(i32);
pub const HDIS_FOCUSED: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct HDITEMA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct HDITEMW(i32);
#[repr(transparent)]
pub struct HDI_MASK(pub u32);
pub const HDI_WIDTH: HDI_MASK = HDI_MASK(1u32);
pub const HDI_HEIGHT: HDI_MASK = HDI_MASK(1u32);
pub const HDI_TEXT: HDI_MASK = HDI_MASK(2u32);
pub const HDI_FORMAT: HDI_MASK = HDI_MASK(4u32);
pub const HDI_LPARAM: HDI_MASK = HDI_MASK(8u32);
pub const HDI_BITMAP: HDI_MASK = HDI_MASK(16u32);
pub const HDI_IMAGE: HDI_MASK = HDI_MASK(32u32);
pub const HDI_DI_SETITEM: HDI_MASK = HDI_MASK(64u32);
pub const HDI_ORDER: HDI_MASK = HDI_MASK(128u32);
pub const HDI_FILTER: HDI_MASK = HDI_MASK(256u32);
pub const HDI_STATE: HDI_MASK = HDI_MASK(512u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct HDLAYOUT(i32);
pub const HDM_CLEARFILTER: u32 = 4632u32;
pub const HDM_CREATEDRAGIMAGE: u32 = 4624u32;
pub const HDM_DELETEITEM: u32 = 4610u32;
pub const HDM_EDITFILTER: u32 = 4631u32;
pub const HDM_FIRST: u32 = 4608u32;
pub const HDM_GETBITMAPMARGIN: u32 = 4629u32;
pub const HDM_GETFOCUSEDITEM: u32 = 4635u32;
pub const HDM_GETIMAGELIST: u32 = 4617u32;
pub const HDM_GETITEM: u32 = 4619u32;
pub const HDM_GETITEMA: u32 = 4611u32;
pub const HDM_GETITEMCOUNT: u32 = 4608u32;
pub const HDM_GETITEMDROPDOWNRECT: u32 = 4633u32;
pub const HDM_GETITEMRECT: u32 = 4615u32;
pub const HDM_GETITEMW: u32 = 4619u32;
pub const HDM_GETORDERARRAY: u32 = 4625u32;
pub const HDM_GETOVERFLOWRECT: u32 = 4634u32;
pub const HDM_GETUNICODEFORMAT: u32 = 8198u32;
pub const HDM_HITTEST: u32 = 4614u32;
pub const HDM_INSERTITEM: u32 = 4618u32;
pub const HDM_INSERTITEMA: u32 = 4609u32;
pub const HDM_INSERTITEMW: u32 = 4618u32;
pub const HDM_LAYOUT: u32 = 4613u32;
pub const HDM_ORDERTOINDEX: u32 = 4623u32;
pub const HDM_SETBITMAPMARGIN: u32 = 4628u32;
pub const HDM_SETFILTERCHANGETIMEOUT: u32 = 4630u32;
pub const HDM_SETFOCUSEDITEM: u32 = 4636u32;
pub const HDM_SETHOTDIVIDER: u32 = 4627u32;
pub const HDM_SETIMAGELIST: u32 = 4616u32;
pub const HDM_SETITEM: u32 = 4620u32;
pub const HDM_SETITEMA: u32 = 4612u32;
pub const HDM_SETITEMW: u32 = 4620u32;
pub const HDM_SETORDERARRAY: u32 = 4626u32;
pub const HDM_SETUNICODEFORMAT: u32 = 8197u32;
#[repr(C)]
pub struct HDPA(i32);
#[repr(C)]
pub struct HDSA(i32);
pub const HDSIL_NORMAL: u32 = 0u32;
pub const HDSIL_STATE: u32 = 1u32;
pub const HDS_BUTTONS: u32 = 2u32;
pub const HDS_CHECKBOXES: u32 = 1024u32;
pub const HDS_DRAGDROP: u32 = 64u32;
pub const HDS_FILTERBAR: u32 = 256u32;
pub const HDS_FLAT: u32 = 512u32;
pub const HDS_FULLDRAG: u32 = 128u32;
pub const HDS_HIDDEN: u32 = 8u32;
pub const HDS_HORZ: u32 = 0u32;
pub const HDS_HOTTRACK: u32 = 4u32;
pub const HDS_NOSIZING: u32 = 2048u32;
pub const HDS_OVERFLOW: u32 = 4096u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HD_TEXTFILTERA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HD_TEXTFILTERW(i32);
#[repr(transparent)]
pub struct HEADER_CONTROL_NOTIFICATION_BUTTON(pub u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_LEFT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(0u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_RIGHT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(1u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_MIDDLE: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(2u32);
pub const HHT_ABOVE: u32 = 256u32;
pub const HHT_BELOW: u32 = 512u32;
pub const HHT_NOWHERE: u32 = 1u32;
pub const HHT_ONDIVIDER: u32 = 4u32;
pub const HHT_ONDIVOPEN: u32 = 8u32;
pub const HHT_ONDROPDOWN: u32 = 8192u32;
pub const HHT_ONFILTER: u32 = 16u32;
pub const HHT_ONFILTERBUTTON: u32 = 32u32;
pub const HHT_ONHEADER: u32 = 2u32;
pub const HHT_ONITEMSTATEICON: u32 = 4096u32;
pub const HHT_ONOVERFLOW: u32 = 16384u32;
pub const HHT_TOLEFT: u32 = 2048u32;
pub const HHT_TORIGHT: u32 = 1024u32;
#[repr(C)]
pub struct HIMAGELIST(i32);
pub const HIST_ADDTOFAVORITES: u32 = 3u32;
pub const HIST_BACK: u32 = 0u32;
pub const HIST_FAVORITES: u32 = 2u32;
pub const HIST_FORWARD: u32 = 1u32;
pub const HIST_VIEWTREE: u32 = 4u32;
pub const HKCOMB_A: u32 = 8u32;
pub const HKCOMB_C: u32 = 4u32;
pub const HKCOMB_CA: u32 = 64u32;
pub const HKCOMB_NONE: u32 = 1u32;
pub const HKCOMB_S: u32 = 2u32;
pub const HKCOMB_SA: u32 = 32u32;
pub const HKCOMB_SC: u32 = 16u32;
pub const HKCOMB_SCA: u32 = 128u32;
pub const HKM_GETHOTKEY: u32 = 1026u32;
pub const HKM_SETHOTKEY: u32 = 1025u32;
pub const HKM_SETRULES: u32 = 1027u32;
pub const HOTKEYF_ALT: u32 = 4u32;
pub const HOTKEYF_CONTROL: u32 = 2u32;
pub const HOTKEYF_EXT: u32 = 128u32;
pub const HOTKEYF_SHIFT: u32 = 1u32;
pub const HOVER_DEFAULT: u32 = 4294967295u32;
#[repr(C)]
pub struct HPROPSHEETPAGE(i32);
#[repr(C)]
pub struct HSYNTHETICPOINTERDEVICE(i32);
#[repr(C)]
pub struct HTREEITEM(i32);
pub const HTTB_BACKGROUNDSEG: u32 = 0u32;
pub const HTTB_CAPTION: u32 = 4u32;
pub const HTTB_FIXEDBORDER: u32 = 2u32;
pub const HTTB_RESIZINGBORDER_BOTTOM: u32 = 128u32;
pub const HTTB_RESIZINGBORDER_LEFT: u32 = 16u32;
pub const HTTB_RESIZINGBORDER_RIGHT: u32 = 64u32;
pub const HTTB_RESIZINGBORDER_TOP: u32 = 32u32;
pub const HTTB_SIZINGTEMPLATE: u32 = 256u32;
pub const HTTB_SYSTEMSIZINGMARGINS: u32 = 512u32;
#[repr(transparent)]
pub struct HYPERLINKSTATES(pub i32);
pub const HLS_NORMALTEXT: HYPERLINKSTATES = HYPERLINKSTATES(1i32);
pub const HLS_LINKTEXT: HYPERLINKSTATES = HYPERLINKSTATES(2i32);
#[repr(transparent)]
pub struct ICONEFFECT(pub i32);
pub const ICE_NONE: ICONEFFECT = ICONEFFECT(0i32);
pub const ICE_GLOW: ICONEFFECT = ICONEFFECT(1i32);
pub const ICE_SHADOW: ICONEFFECT = ICONEFFECT(2i32);
pub const ICE_PULSE: ICONEFFECT = ICONEFFECT(3i32);
pub const ICE_ALPHA: ICONEFFECT = ICONEFFECT(4i32);
pub const IDB_HIST_DISABLED: u32 = 14u32;
pub const IDB_HIST_HOT: u32 = 13u32;
pub const IDB_HIST_LARGE_COLOR: u32 = 9u32;
pub const IDB_HIST_NORMAL: u32 = 12u32;
pub const IDB_HIST_PRESSED: u32 = 15u32;
pub const IDB_HIST_SMALL_COLOR: u32 = 8u32;
pub const IDB_STD_LARGE_COLOR: u32 = 1u32;
pub const IDB_STD_SMALL_COLOR: u32 = 0u32;
pub const IDB_VIEW_LARGE_COLOR: u32 = 5u32;
pub const IDB_VIEW_SMALL_COLOR: u32 = 4u32;
pub const IDC_MANAGE_LINK: u32 = 1592u32;
pub const ID_PSRESTARTWINDOWS: u32 = 2u32;
#[repr(transparent)]
pub struct IImageList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageList2(pub *mut ::core::ffi::c_void);
pub const ILDI_PURGE: u32 = 1u32;
pub const ILDI_QUERYACCESS: u32 = 8u32;
pub const ILDI_RESETACCESS: u32 = 4u32;
pub const ILDI_STANDBY: u32 = 2u32;
pub const ILDRF_IMAGELOWQUALITY: u32 = 1u32;
pub const ILDRF_OVERLAYLOWQUALITY: u32 = 16u32;
pub const ILD_ASYNC: u32 = 32768u32;
pub const ILD_BLEND25: u32 = 2u32;
pub const ILD_DPISCALE: u32 = 16384u32;
pub const ILD_IMAGE: u32 = 32u32;
pub const ILD_OVERLAYMASK: u32 = 3840u32;
pub const ILD_PRESERVEALPHA: u32 = 4096u32;
pub const ILD_ROP: u32 = 64u32;
pub const ILD_SCALE: u32 = 8192u32;
pub const ILD_TRANSPARENT: u32 = 1u32;
pub const ILFIP_ALWAYS: u32 = 0u32;
pub const ILFIP_FROMSTANDBY: u32 = 1u32;
pub const ILGOS_ALWAYS: u32 = 0u32;
pub const ILGOS_FROMSTANDBY: u32 = 1u32;
pub const ILGT_ASYNC: u32 = 1u32;
pub const ILGT_NORMAL: u32 = 0u32;
pub const ILP_DOWNLEVEL: u32 = 1u32;
pub const ILP_NORMAL: u32 = 0u32;
pub const ILR_DEFAULT: u32 = 0u32;
pub const ILR_HORIZONTAL_CENTER: u32 = 1u32;
pub const ILR_HORIZONTAL_LEFT: u32 = 0u32;
pub const ILR_HORIZONTAL_RIGHT: u32 = 2u32;
pub const ILR_SCALE_ASPECTRATIO: u32 = 256u32;
pub const ILR_SCALE_CLIP: u32 = 0u32;
pub const ILR_VERTICAL_BOTTOM: u32 = 32u32;
pub const ILR_VERTICAL_CENTER: u32 = 16u32;
pub const ILR_VERTICAL_TOP: u32 = 0u32;
pub const ILS_ALPHA: u32 = 8u32;
pub const ILS_GLOW: u32 = 1u32;
pub const ILS_NORMAL: u32 = 0u32;
pub const ILS_SATURATE: u32 = 4u32;
pub const ILS_SHADOW: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct IMAGEINFO(i32);
#[repr(transparent)]
pub struct IMAGELAYOUT(pub i32);
pub const IL_VERTICAL: IMAGELAYOUT = IMAGELAYOUT(0i32);
pub const IL_HORIZONTAL: IMAGELAYOUT = IMAGELAYOUT(1i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct IMAGELISTDRAWPARAMS(i32);
#[repr(C)]
pub struct IMAGELISTSTATS(i32);
#[repr(transparent)]
pub struct IMAGELIST_CREATION_FLAGS(pub u32);
pub const ILC_MASK: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(1u32);
pub const ILC_COLOR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(0u32);
pub const ILC_COLORDDB: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(254u32);
pub const ILC_COLOR4: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(4u32);
pub const ILC_COLOR8: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8u32);
pub const ILC_COLOR16: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(16u32);
pub const ILC_COLOR24: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(24u32);
pub const ILC_COLOR32: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32u32);
pub const ILC_PALETTE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(2048u32);
pub const ILC_MIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8192u32);
pub const ILC_PERITEMMIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32768u32);
pub const ILC_ORIGINALSIZE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(65536u32);
pub const ILC_HIGHQUALITYSCALE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(131072u32);
#[repr(transparent)]
pub struct IMAGESELECTTYPE(pub i32);
pub const IST_NONE: IMAGESELECTTYPE = IMAGESELECTTYPE(0i32);
pub const IST_SIZE: IMAGESELECTTYPE = IMAGESELECTTYPE(1i32);
pub const IST_DPI: IMAGESELECTTYPE = IMAGESELECTTYPE(2i32);
#[repr(transparent)]
pub struct IMAGE_LIST_COPY_FLAGS(pub u32);
pub const ILCF_MOVE: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(0u32);
pub const ILCF_SWAP: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(1u32);
#[repr(transparent)]
pub struct IMAGE_LIST_DRAW_STYLE(pub u32);
pub const ILD_BLEND: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_BLEND50: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_FOCUS: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
pub const ILD_MASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16u32);
pub const ILD_NORMAL: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(0u32);
pub const ILD_SELECTED: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
#[repr(transparent)]
pub struct IMAGE_LIST_ITEM_FLAGS(pub u32);
pub const ILIF_ALPHA: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(1u32);
pub const ILIF_LOWQUALITY: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(2u32);
pub const INFOTIPSIZE: u32 = 1024u32;
#[repr(C)]
pub struct INITCOMMONCONTROLSEX(i32);
#[repr(transparent)]
pub struct INITCOMMONCONTROLSEX_ICC(pub u32);
pub const ICC_ANIMATE_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(128u32);
pub const ICC_BAR_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4u32);
pub const ICC_COOL_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1024u32);
pub const ICC_DATE_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(256u32);
pub const ICC_HOTKEY_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(64u32);
pub const ICC_INTERNET_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2048u32);
pub const ICC_LINK_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32768u32);
pub const ICC_LISTVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1u32);
pub const ICC_NATIVEFNTCTL_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8192u32);
pub const ICC_PAGESCROLLER_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4096u32);
pub const ICC_PROGRESS_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32u32);
pub const ICC_STANDARD_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16384u32);
pub const ICC_TAB_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8u32);
pub const ICC_TREEVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2u32);
pub const ICC_UPDOWN_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16u32);
pub const ICC_USEREX_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(512u32);
pub const ICC_WIN95_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(255u32);
#[repr(C)]
pub struct INTLIST(i32);
pub const INVALID_LINK_INDEX: i32 = -1i32;
pub const IPM_CLEARADDRESS: u32 = 1124u32;
pub const IPM_GETADDRESS: u32 = 1126u32;
pub const IPM_ISBLANK: u32 = 1129u32;
pub const IPM_SETADDRESS: u32 = 1125u32;
pub const IPM_SETFOCUS: u32 = 1128u32;
pub const IPM_SETRANGE: u32 = 1127u32;
pub const I_IMAGECALLBACK: i32 = -1i32;
pub const I_IMAGENONE: i32 = -2i32;
pub const I_INDENTCALLBACK: i32 = -1i32;
pub const ImageList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2085055394, data2: 689, data3: 18676, data4: [128, 72, 178, 70, 25, 221, 192, 88] };
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LHITTESTINFO(i32);
pub const LIF_ITEMID: u32 = 4u32;
pub const LIF_ITEMINDEX: u32 = 1u32;
pub const LIF_STATE: u32 = 2u32;
pub const LIF_URL: u32 = 8u32;
#[repr(transparent)]
pub struct LINKPARTS(pub i32);
pub const LP_HYPERLINK: LINKPARTS = LINKPARTS(1i32);
pub const LIS_DEFAULTCOLORS: u32 = 16u32;
pub const LIS_ENABLED: u32 = 2u32;
pub const LIS_FOCUSED: u32 = 1u32;
pub const LIS_HOTTRACK: u32 = 8u32;
pub const LIS_VISITED: u32 = 4u32;
#[repr(C)]
pub struct LITEM(i32);
pub const LM_GETIDEALHEIGHT: u32 = 1793u32;
pub const LM_GETIDEALSIZE: u32 = 1793u32;
pub const LM_GETITEM: u32 = 1795u32;
pub const LM_HITTEST: u32 = 1792u32;
pub const LM_SETITEM: u32 = 1794u32;
#[repr(transparent)]
pub struct LOGOFFBUTTONSSTATES(pub i32);
pub const SPLS_NORMAL: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(1i32);
pub const SPLS_HOT: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(2i32);
pub const SPLS_PRESSED: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(3i32);
#[cfg(feature = "Win32_Foundation")]
pub type LPFNADDPROPSHEETPAGES = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: LPFNSVADDPROPSHEETPAGE, param2: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCINFOA = unsafe extern "system" fn(acci: *mut CCINFOA) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCINFOW = unsafe extern "system" fn(acci: *mut CCINFOW) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCSIZETOTEXTA = unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: super::super::Foundation::PSTR) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCSIZETOTEXTW = unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: super::super::Foundation::PWSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCCSTYLEA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCCSTYLEW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEW) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEA) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKW = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEW) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNSVADDPROPSHEETPAGE = unsafe extern "system" fn(param0: HPROPSHEETPAGE, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
pub const LVA_ALIGNLEFT: u32 = 1u32;
pub const LVA_ALIGNTOP: u32 = 2u32;
pub const LVA_DEFAULT: u32 = 0u32;
pub const LVA_SNAPTOGRID: u32 = 5u32;
pub const LVBKIF_FLAG_ALPHABLEND: u32 = 536870912u32;
pub const LVBKIF_FLAG_TILEOFFSET: u32 = 256u32;
pub const LVBKIF_SOURCE_HBITMAP: u32 = 1u32;
pub const LVBKIF_SOURCE_MASK: u32 = 3u32;
pub const LVBKIF_SOURCE_NONE: u32 = 0u32;
pub const LVBKIF_SOURCE_URL: u32 = 2u32;
pub const LVBKIF_STYLE_MASK: u32 = 16u32;
pub const LVBKIF_STYLE_NORMAL: u32 = 0u32;
pub const LVBKIF_STYLE_TILE: u32 = 16u32;
pub const LVBKIF_TYPE_WATERMARK: u32 = 268435456u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct LVBKIMAGEA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct LVBKIMAGEW(i32);
pub const LVCDRF_NOGROUPFRAME: u32 = 131072u32;
pub const LVCDRF_NOSELECT: u32 = 65536u32;
pub const LVCFMT_FILL: u32 = 2097152u32;
pub const LVCFMT_LINE_BREAK: u32 = 1048576u32;
pub const LVCFMT_NO_TITLE: u32 = 8388608u32;
pub const LVCFMT_WRAP: u32 = 4194304u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVCOLUMNA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVCOLUMNW(i32);
#[repr(transparent)]
pub struct LVCOLUMNW_FORMAT(pub u32);
pub const LVCFMT_LEFT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(0u32);
pub const LVCFMT_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(1u32);
pub const LVCFMT_CENTER: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2u32);
pub const LVCFMT_JUSTIFYMASK: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(3u32);
pub const LVCFMT_IMAGE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2048u32);
pub const LVCFMT_BITMAP_ON_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(4096u32);
pub const LVCFMT_COL_HAS_IMAGES: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(32768u32);
pub const LVCFMT_FIXED_WIDTH: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(256u32);
pub const LVCFMT_NO_DPI_SCALE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(262144u32);
pub const LVCFMT_FIXED_RATIO: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(524288u32);
pub const LVCFMT_SPLITBUTTON: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(16777216u32);
#[repr(transparent)]
pub struct LVCOLUMNW_MASK(pub u32);
pub const LVCF_FMT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(1u32);
pub const LVCF_WIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(2u32);
pub const LVCF_TEXT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(4u32);
pub const LVCF_SUBITEM: LVCOLUMNW_MASK = LVCOLUMNW_MASK(8u32);
pub const LVCF_IMAGE: LVCOLUMNW_MASK = LVCOLUMNW_MASK(16u32);
pub const LVCF_ORDER: LVCOLUMNW_MASK = LVCOLUMNW_MASK(32u32);
pub const LVCF_MINWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(64u32);
pub const LVCF_DEFAULTWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(128u32);
pub const LVCF_IDEALWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(256u32);
pub const LVFF_ITEMCOUNT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVFINDINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVFINDINFOW(i32);
#[repr(transparent)]
pub struct LVFINDINFOW_FLAGS(pub u32);
pub const LVFI_PARAM: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(1u32);
pub const LVFI_PARTIAL: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(8u32);
pub const LVFI_STRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(2u32);
pub const LVFI_SUBSTRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(4u32);
pub const LVFI_WRAP: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(32u32);
pub const LVFI_NEARESTXY: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(64u32);
pub const LVFIS_FOCUSED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVFOOTERINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVFOOTERITEM(i32);
#[repr(transparent)]
pub struct LVFOOTERITEM_MASK(pub u32);
pub const LVFIF_TEXT: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(1u32);
pub const LVFIF_STATE: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(2u32);
pub const LVGA_FOOTER_CENTER: u32 = 16u32;
pub const LVGA_FOOTER_LEFT: u32 = 8u32;
pub const LVGA_FOOTER_RIGHT: u32 = 32u32;
pub const LVGF_ALIGN: u32 = 8u32;
pub const LVGF_DESCRIPTIONBOTTOM: u32 = 2048u32;
pub const LVGF_DESCRIPTIONTOP: u32 = 1024u32;
pub const LVGF_EXTENDEDIMAGE: u32 = 8192u32;
pub const LVGF_GROUPID: u32 = 16u32;
pub const LVGF_ITEMS: u32 = 16384u32;
pub const LVGF_SUBSET: u32 = 32768u32;
pub const LVGF_SUBSETITEMS: u32 = 65536u32;
pub const LVGF_SUBTITLE: u32 = 256u32;
pub const LVGF_TASK: u32 = 512u32;
pub const LVGF_TITLEIMAGE: u32 = 4096u32;
pub const LVGGR_GROUP: u32 = 0u32;
pub const LVGGR_HEADER: u32 = 1u32;
pub const LVGGR_LABEL: u32 = 2u32;
pub const LVGGR_SUBSETLINK: u32 = 3u32;
pub const LVGIT_UNFOLDED: u32 = 1u32;
pub const LVGMF_BORDERCOLOR: u32 = 2u32;
pub const LVGMF_BORDERSIZE: u32 = 1u32;
pub const LVGMF_NONE: u32 = 0u32;
pub const LVGMF_TEXTCOLOR: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVGROUP(i32);
#[repr(C)]
pub struct LVGROUPMETRICS(i32);
#[repr(transparent)]
pub struct LVGROUP_MASK(pub u32);
pub const LVGF_NONE: LVGROUP_MASK = LVGROUP_MASK(0u32);
pub const LVGF_HEADER: LVGROUP_MASK = LVGROUP_MASK(1u32);
pub const LVGF_FOOTER: LVGROUP_MASK = LVGROUP_MASK(2u32);
pub const LVGF_STATE: LVGROUP_MASK = LVGROUP_MASK(4u32);
pub const LVGS_COLLAPSED: u32 = 1u32;
pub const LVGS_COLLAPSIBLE: u32 = 8u32;
pub const LVGS_FOCUSED: u32 = 16u32;
pub const LVGS_HIDDEN: u32 = 2u32;
pub const LVGS_NOHEADER: u32 = 4u32;
pub const LVGS_NORMAL: u32 = 0u32;
pub const LVGS_SELECTED: u32 = 32u32;
pub const LVGS_SUBSETED: u32 = 64u32;
pub const LVGS_SUBSETLINKFOCUSED: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVHITTESTINFO(i32);
#[repr(transparent)]
pub struct LVHITTESTINFO_FLAGS(pub u32);
pub const LVHT_ABOVE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_BELOW: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16u32);
pub const LVHT_NOWHERE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1u32);
pub const LVHT_ONITEMICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2u32);
pub const LVHT_ONITEMLABEL: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4u32);
pub const LVHT_ONITEMSTATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_TOLEFT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(64u32);
pub const LVHT_TORIGHT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(32u32);
pub const LVHT_EX_GROUP_HEADER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(268435456u32);
pub const LVHT_EX_GROUP_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(536870912u32);
pub const LVHT_EX_GROUP_COLLAPSE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1073741824u32);
pub const LVHT_EX_GROUP_BACKGROUND: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2147483648u32);
pub const LVHT_EX_GROUP_STATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16777216u32);
pub const LVHT_EX_GROUP_SUBSETLINK: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(33554432u32);
pub const LVHT_EX_GROUP: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4076863488u32);
pub const LVHT_EX_ONCONTENTS: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(67108864u32);
pub const LVHT_EX_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(134217728u32);
pub const LVIF_COLFMT: u32 = 65536u32;
pub const LVIF_COLUMNS: u32 = 512u32;
pub const LVIF_DI_SETITEM: u32 = 4096u32;
pub const LVIF_GROUPID: u32 = 256u32;
pub const LVIF_IMAGE: u32 = 2u32;
pub const LVIF_INDENT: u32 = 16u32;
pub const LVIF_NORECOMPUTE: u32 = 2048u32;
pub const LVIF_PARAM: u32 = 4u32;
pub const LVIF_STATE: u32 = 8u32;
pub const LVIF_TEXT: u32 = 1u32;
pub const LVIM_AFTER: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVINSERTGROUPSORTED(i32);
#[repr(C)]
pub struct LVINSERTMARK(i32);
pub const LVIR_BOUNDS: u32 = 0u32;
pub const LVIR_ICON: u32 = 1u32;
pub const LVIR_LABEL: u32 = 2u32;
pub const LVIR_SELECTBOUNDS: u32 = 3u32;
pub const LVIS_ACTIVATING: u32 = 32u32;
pub const LVIS_CUT: u32 = 4u32;
pub const LVIS_DROPHILITED: u32 = 8u32;
pub const LVIS_FOCUSED: u32 = 1u32;
pub const LVIS_GLOW: u32 = 16u32;
pub const LVIS_OVERLAYMASK: u32 = 3840u32;
pub const LVIS_SELECTED: u32 = 2u32;
pub const LVIS_STATEIMAGEMASK: u32 = 61440u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVITEMA(i32);
#[repr(transparent)]
pub struct LVITEMA_GROUP_ID(pub i32);
pub const I_GROUPIDCALLBACK: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-1i32);
pub const I_GROUPIDNONE: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-2i32);
#[repr(C)]
pub struct LVITEMINDEX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVITEMW(i32);
pub const LVKF_ALT: u32 = 1u32;
pub const LVKF_CONTROL: u32 = 2u32;
pub const LVKF_SHIFT: u32 = 4u32;
pub const LVM_APPROXIMATEVIEWRECT: u32 = 4160u32;
pub const LVM_ARRANGE: u32 = 4118u32;
pub const LVM_CANCELEDITLABEL: u32 = 4275u32;
pub const LVM_CREATEDRAGIMAGE: u32 = 4129u32;
pub const LVM_DELETEALLITEMS: u32 = 4105u32;
pub const LVM_DELETECOLUMN: u32 = 4124u32;
pub const LVM_DELETEITEM: u32 = 4104u32;
pub const LVM_EDITLABEL: u32 = 4214u32;
pub const LVM_EDITLABELA: u32 = 4119u32;
pub const LVM_EDITLABELW: u32 = 4214u32;
pub const LVM_ENABLEGROUPVIEW: u32 = 4253u32;
pub const LVM_ENSUREVISIBLE: u32 = 4115u32;
pub const LVM_FINDITEM: u32 = 4179u32;
pub const LVM_FINDITEMA: u32 = 4109u32;
pub const LVM_FINDITEMW: u32 = 4179u32;
pub const LVM_FIRST: u32 = 4096u32;
pub const LVM_GETBKCOLOR: u32 = 4096u32;
pub const LVM_GETBKIMAGE: u32 = 4235u32;
pub const LVM_GETBKIMAGEA: u32 = 4165u32;
pub const LVM_GETBKIMAGEW: u32 = 4235u32;
pub const LVM_GETCALLBACKMASK: u32 = 4106u32;
pub const LVM_GETCOLUMN: u32 = 4191u32;
pub const LVM_GETCOLUMNA: u32 = 4121u32;
pub const LVM_GETCOLUMNORDERARRAY: u32 = 4155u32;
pub const LVM_GETCOLUMNW: u32 = 4191u32;
pub const LVM_GETCOLUMNWIDTH: u32 = 4125u32;
pub const LVM_GETCOUNTPERPAGE: u32 = 4136u32;
pub const LVM_GETEDITCONTROL: u32 = 4120u32;
pub const LVM_GETEMPTYTEXT: u32 = 4300u32;
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 4151u32;
pub const LVM_GETFOCUSEDGROUP: u32 = 4189u32;
pub const LVM_GETFOOTERINFO: u32 = 4302u32;
pub const LVM_GETFOOTERITEM: u32 = 4304u32;
pub const LVM_GETFOOTERITEMRECT: u32 = 4303u32;
pub const LVM_GETFOOTERRECT: u32 = 4301u32;
pub const LVM_GETGROUPCOUNT: u32 = 4248u32;
pub const LVM_GETGROUPINFO: u32 = 4245u32;
pub const LVM_GETGROUPINFOBYINDEX: u32 = 4249u32;
pub const LVM_GETGROUPMETRICS: u32 = 4252u32;
pub const LVM_GETGROUPRECT: u32 = 4194u32;
pub const LVM_GETGROUPSTATE: u32 = 4188u32;
pub const LVM_GETHEADER: u32 = 4127u32;
pub const LVM_GETHOTCURSOR: u32 = 4159u32;
pub const LVM_GETHOTITEM: u32 = 4157u32;
pub const LVM_GETHOVERTIME: u32 = 4168u32;
pub const LVM_GETIMAGELIST: u32 = 4098u32;
pub const LVM_GETINSERTMARK: u32 = 4263u32;
pub const LVM_GETINSERTMARKCOLOR: u32 = 4267u32;
pub const LVM_GETINSERTMARKRECT: u32 = 4265u32;
pub const LVM_GETISEARCHSTRING: u32 = 4213u32;
pub const LVM_GETISEARCHSTRINGA: u32 = 4148u32;
pub const LVM_GETISEARCHSTRINGW: u32 = 4213u32;
pub const LVM_GETITEM: u32 = 4171u32;
pub const LVM_GETITEMA: u32 = 4101u32;
pub const LVM_GETITEMCOUNT: u32 = 4100u32;
pub const LVM_GETITEMINDEXRECT: u32 = 4305u32;
pub const LVM_GETITEMPOSITION: u32 = 4112u32;
pub const LVM_GETITEMRECT: u32 = 4110u32;
pub const LVM_GETITEMSPACING: u32 = 4147u32;
pub const LVM_GETITEMSTATE: u32 = 4140u32;
pub const LVM_GETITEMTEXT: u32 = 4211u32;
pub const LVM_GETITEMTEXTA: u32 = 4141u32;
pub const LVM_GETITEMTEXTW: u32 = 4211u32;
pub const LVM_GETITEMW: u32 = 4171u32;
pub const LVM_GETNEXTITEM: u32 = 4108u32;
pub const LVM_GETNEXTITEMINDEX: u32 = 4307u32;
pub const LVM_GETNUMBEROFWORKAREAS: u32 = 4169u32;
pub const LVM_GETORIGIN: u32 = 4137u32;
pub const LVM_GETOUTLINECOLOR: u32 = 4272u32;
pub const LVM_GETSELECTEDCOLUMN: u32 = 4270u32;
pub const LVM_GETSELECTEDCOUNT: u32 = 4146u32;
pub const LVM_GETSELECTIONMARK: u32 = 4162u32;
pub const LVM_GETSTRINGWIDTH: u32 = 4183u32;
pub const LVM_GETSTRINGWIDTHA: u32 = 4113u32;
pub const LVM_GETSTRINGWIDTHW: u32 = 4183u32;
pub const LVM_GETSUBITEMRECT: u32 = 4152u32;
pub const LVM_GETTEXTBKCOLOR: u32 = 4133u32;
pub const LVM_GETTEXTCOLOR: u32 = 4131u32;
pub const LVM_GETTILEINFO: u32 = 4261u32;
pub const LVM_GETTILEVIEWINFO: u32 = 4259u32;
pub const LVM_GETTOOLTIPS: u32 = 4174u32;
pub const LVM_GETTOPINDEX: u32 = 4135u32;
pub const LVM_GETUNICODEFORMAT: u32 = 8198u32;
pub const LVM_GETVIEW: u32 = 4239u32;
pub const LVM_GETVIEWRECT: u32 = 4130u32;
pub const LVM_GETWORKAREAS: u32 = 4166u32;
pub const LVM_HASGROUP: u32 = 4257u32;
pub const LVM_HITTEST: u32 = 4114u32;
pub const LVM_INSERTCOLUMNA: u32 = 4123u32;
pub const LVM_INSERTCOLUMNW: u32 = 4193u32;
pub const LVM_INSERTGROUP: u32 = 4241u32;
pub const LVM_INSERTGROUPSORTED: u32 = 4255u32;
pub const LVM_INSERTITEM: u32 = 4173u32;
pub const LVM_INSERTITEMA: u32 = 4103u32;
pub const LVM_INSERTITEMW: u32 = 4173u32;
pub const LVM_INSERTMARKHITTEST: u32 = 4264u32;
pub const LVM_ISGROUPVIEWENABLED: u32 = 4271u32;
pub const LVM_ISITEMVISIBLE: u32 = 4278u32;
pub const LVM_MAPIDTOINDEX: u32 = 4277u32;
pub const LVM_MAPINDEXTOID: u32 = 4276u32;
pub const LVM_MOVEGROUP: u32 = 4247u32;
pub const LVM_MOVEITEMTOGROUP: u32 = 4250u32;
pub const LVM_REDRAWITEMS: u32 = 4117u32;
pub const LVM_REMOVEALLGROUPS: u32 = 4256u32;
pub const LVM_REMOVEGROUP: u32 = 4246u32;
pub const LVM_SCROLL: u32 = 4116u32;
pub const LVM_SETBKCOLOR: u32 = 4097u32;
pub const LVM_SETBKIMAGE: u32 = 4234u32;
pub const LVM_SETBKIMAGEA: u32 = 4164u32;
pub const LVM_SETBKIMAGEW: u32 = 4234u32;
pub const LVM_SETCALLBACKMASK: u32 = 4107u32;
pub const LVM_SETCOLUMN: u32 = 4192u32;
pub const LVM_SETCOLUMNA: u32 = 4122u32;
pub const LVM_SETCOLUMNORDERARRAY: u32 = 4154u32;
pub const LVM_SETCOLUMNW: u32 = 4192u32;
pub const LVM_SETCOLUMNWIDTH: u32 = 4126u32;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 4150u32;
pub const LVM_SETGROUPINFO: u32 = 4243u32;
pub const LVM_SETGROUPMETRICS: u32 = 4251u32;
pub const LVM_SETHOTCURSOR: u32 = 4158u32;
pub const LVM_SETHOTITEM: u32 = 4156u32;
pub const LVM_SETHOVERTIME: u32 = 4167u32;
pub const LVM_SETICONSPACING: u32 = 4149u32;
pub const LVM_SETIMAGELIST: u32 = 4099u32;
pub const LVM_SETINFOTIP: u32 = 4269u32;
pub const LVM_SETINSERTMARK: u32 = 4262u32;
pub const LVM_SETINSERTMARKCOLOR: u32 = 4266u32;
pub const LVM_SETITEM: u32 = 4172u32;
pub const LVM_SETITEMA: u32 = 4102u32;
pub const LVM_SETITEMCOUNT: u32 = 4143u32;
pub const LVM_SETITEMINDEXSTATE: u32 = 4306u32;
pub const LVM_SETITEMPOSITION: u32 = 4111u32;
pub const LVM_SETITEMPOSITION32: u32 = 4145u32;
pub const LVM_SETITEMSTATE: u32 = 4139u32;
pub const LVM_SETITEMTEXT: u32 = 4212u32;
pub const LVM_SETITEMTEXTA: u32 = 4142u32;
pub const LVM_SETITEMTEXTW: u32 = 4212u32;
pub const LVM_SETITEMW: u32 = 4172u32;
pub const LVM_SETOUTLINECOLOR: u32 = 4273u32;
pub const LVM_SETSELECTEDCOLUMN: u32 = 4236u32;
pub const LVM_SETSELECTIONMARK: u32 = 4163u32;
pub const LVM_SETTEXTBKCOLOR: u32 = 4134u32;
pub const LVM_SETTEXTCOLOR: u32 = 4132u32;
pub const LVM_SETTILEINFO: u32 = 4260u32;
pub const LVM_SETTILEVIEWINFO: u32 = 4258u32;
pub const LVM_SETTOOLTIPS: u32 = 4170u32;
pub const LVM_SETUNICODEFORMAT: u32 = 8197u32;
pub const LVM_SETVIEW: u32 = 4238u32;
pub const LVM_SETWORKAREAS: u32 = 4161u32;
pub const LVM_SORTGROUPS: u32 = 4254u32;
pub const LVM_SORTITEMS: u32 = 4144u32;
pub const LVM_SORTITEMSEX: u32 = 4177u32;
pub const LVM_SUBITEMHITTEST: u32 = 4153u32;
pub const LVM_UPDATE: u32 = 4138u32;
pub const LVNI_ABOVE: u32 = 256u32;
pub const LVNI_ALL: u32 = 0u32;
pub const LVNI_BELOW: u32 = 512u32;
pub const LVNI_CUT: u32 = 4u32;
pub const LVNI_DROPHILITED: u32 = 8u32;
pub const LVNI_FOCUSED: u32 = 1u32;
pub const LVNI_PREVIOUS: u32 = 32u32;
pub const LVNI_SAMEGROUPONLY: u32 = 128u32;
pub const LVNI_SELECTED: u32 = 2u32;
pub const LVNI_TOLEFT: u32 = 1024u32;
pub const LVNI_TORIGHT: u32 = 2048u32;
pub const LVNI_VISIBLEONLY: u32 = 64u32;
pub const LVNI_VISIBLEORDER: u32 = 16u32;
pub const LVNSCH_DEFAULT: i32 = -1i32;
pub const LVNSCH_ERROR: i32 = -2i32;
pub const LVNSCH_IGNORE: i32 = -3i32;
pub const LVSCW_AUTOSIZE: i32 = -1i32;
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVSETINFOTIP(i32);
pub const LVSICF_NOINVALIDATEALL: u32 = 1u32;
pub const LVSICF_NOSCROLL: u32 = 2u32;
pub const LVSIL_GROUPHEADER: u32 = 3u32;
pub const LVSIL_NORMAL: u32 = 0u32;
pub const LVSIL_SMALL: u32 = 1u32;
pub const LVSIL_STATE: u32 = 2u32;
pub const LVS_ALIGNLEFT: u32 = 2048u32;
pub const LVS_ALIGNMASK: u32 = 3072u32;
pub const LVS_ALIGNTOP: u32 = 0u32;
pub const LVS_AUTOARRANGE: u32 = 256u32;
pub const LVS_EDITLABELS: u32 = 512u32;
pub const LVS_EX_AUTOAUTOARRANGE: u32 = 16777216u32;
pub const LVS_EX_AUTOCHECKSELECT: u32 = 134217728u32;
pub const LVS_EX_AUTOSIZECOLUMNS: u32 = 268435456u32;
pub const LVS_EX_BORDERSELECT: u32 = 32768u32;
pub const LVS_EX_CHECKBOXES: u32 = 4u32;
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648u32;
pub const LVS_EX_COLUMNSNAPPOINTS: u32 = 1073741824u32;
pub const LVS_EX_DOUBLEBUFFER: u32 = 65536u32;
pub const LVS_EX_FLATSB: u32 = 256u32;
pub const LVS_EX_FULLROWSELECT: u32 = 32u32;
pub const LVS_EX_GRIDLINES: u32 = 1u32;
pub const LVS_EX_HEADERDRAGDROP: u32 = 16u32;
pub const LVS_EX_HEADERINALLVIEWS: u32 = 33554432u32;
pub const LVS_EX_HIDELABELS: u32 = 131072u32;
pub const LVS_EX_INFOTIP: u32 = 1024u32;
pub const LVS_EX_JUSTIFYCOLUMNS: u32 = 2097152u32;
pub const LVS_EX_LABELTIP: u32 = 16384u32;
pub const LVS_EX_MULTIWORKAREAS: u32 = 8192u32;
pub const LVS_EX_ONECLICKACTIVATE: u32 = 64u32;
pub const LVS_EX_REGIONAL: u32 = 512u32;
pub const LVS_EX_SIMPLESELECT: u32 = 1048576u32;
pub const LVS_EX_SINGLEROW: u32 = 262144u32;
pub const LVS_EX_SNAPTOGRID: u32 = 524288u32;
pub const LVS_EX_SUBITEMIMAGES: u32 = 2u32;
pub const LVS_EX_TRACKSELECT: u32 = 8u32;
pub const LVS_EX_TRANSPARENTBKGND: u32 = 4194304u32;
pub const LVS_EX_TRANSPARENTSHADOWTEXT: u32 = 8388608u32;
pub const LVS_EX_TWOCLICKACTIVATE: u32 = 128u32;
pub const LVS_EX_UNDERLINECOLD: u32 = 4096u32;
pub const LVS_EX_UNDERLINEHOT: u32 = 2048u32;
pub const LVS_ICON: u32 = 0u32;
pub const LVS_LIST: u32 = 3u32;
pub const LVS_NOCOLUMNHEADER: u32 = 16384u32;
pub const LVS_NOLABELWRAP: u32 = 128u32;
pub const LVS_NOSCROLL: u32 = 8192u32;
pub const LVS_NOSORTHEADER: u32 = 32768u32;
pub const LVS_OWNERDATA: u32 = 4096u32;
pub const LVS_OWNERDRAWFIXED: u32 = 1024u32;
pub const LVS_REPORT: u32 = 1u32;
pub const LVS_SHAREIMAGELISTS: u32 = 64u32;
pub const LVS_SHOWSELALWAYS: u32 = 8u32;
pub const LVS_SINGLESEL: u32 = 4u32;
pub const LVS_SMALLICON: u32 = 2u32;
pub const LVS_SORTASCENDING: u32 = 16u32;
pub const LVS_SORTDESCENDING: u32 = 32u32;
pub const LVS_TYPEMASK: u32 = 3u32;
pub const LVS_TYPESTYLEMASK: u32 = 64512u32;
#[repr(C)]
pub struct LVTILEINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LVTILEVIEWINFO(i32);
#[repr(transparent)]
pub struct LVTILEVIEWINFO_FLAGS(pub u32);
pub const LVTVIF_EXTENDED: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(4u32);
pub const LVTVIF_AUTOSIZE: u32 = 0u32;
pub const LVTVIF_FIXEDHEIGHT: u32 = 2u32;
pub const LVTVIF_FIXEDSIZE: u32 = 3u32;
pub const LVTVIF_FIXEDWIDTH: u32 = 1u32;
pub const LVTVIM_COLUMNS: u32 = 2u32;
pub const LVTVIM_LABELMARGIN: u32 = 4u32;
pub const LVTVIM_TILESIZE: u32 = 1u32;
pub const LV_MAX_WORKAREAS: u32 = 16u32;
pub const LV_VIEW_DETAILS: u32 = 1u32;
pub const LV_VIEW_ICON: u32 = 0u32;
pub const LV_VIEW_LIST: u32 = 3u32;
pub const LV_VIEW_MAX: u32 = 4u32;
pub const LV_VIEW_SMALLICON: u32 = 2u32;
pub const LV_VIEW_TILE: u32 = 4u32;
pub const LWS_IGNORERETURN: u32 = 2u32;
pub const LWS_NOPREFIX: u32 = 4u32;
pub const LWS_RIGHT: u32 = 32u32;
pub const LWS_TRANSPARENT: u32 = 1u32;
pub const LWS_USECUSTOMTEXT: u32 = 16u32;
pub const LWS_USEVISUALSTYLE: u32 = 8u32;
#[repr(C)]
pub struct MARGINS(i32);
#[repr(transparent)]
pub struct MARKUPTEXTSTATES(pub i32);
pub const EMT_NORMALTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(1i32);
pub const EMT_LINKTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(2i32);
pub const MAXPROPPAGES: u32 = 100u32;
pub const MAX_INTLIST_COUNT: u32 = 402u32;
pub const MAX_LINKID_TEXT: u32 = 48u32;
pub const MAX_THEMECOLOR: u32 = 64u32;
pub const MAX_THEMESIZE: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MCGRIDINFO(i32);
#[repr(transparent)]
pub struct MCGRIDINFO_FLAGS(pub u32);
pub const MCGIF_DATE: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(1u32);
pub const MCGIF_RECT: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(2u32);
pub const MCGIF_NAME: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(4u32);
#[repr(transparent)]
pub struct MCGRIDINFO_PART(pub u32);
pub const MCGIP_CALENDARCONTROL: MCGRIDINFO_PART = MCGRIDINFO_PART(0u32);
pub const MCGIP_NEXT: MCGRIDINFO_PART = MCGRIDINFO_PART(1u32);
pub const MCGIP_PREV: MCGRIDINFO_PART = MCGRIDINFO_PART(2u32);
pub const MCGIP_FOOTER: MCGRIDINFO_PART = MCGRIDINFO_PART(3u32);
pub const MCGIP_CALENDAR: MCGRIDINFO_PART = MCGRIDINFO_PART(4u32);
pub const MCGIP_CALENDARHEADER: MCGRIDINFO_PART = MCGRIDINFO_PART(5u32);
pub const MCGIP_CALENDARBODY: MCGRIDINFO_PART = MCGRIDINFO_PART(6u32);
pub const MCGIP_CALENDARROW: MCGRIDINFO_PART = MCGRIDINFO_PART(7u32);
pub const MCGIP_CALENDARCELL: MCGRIDINFO_PART = MCGRIDINFO_PART(8u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MCHITTESTINFO(i32);
pub const MCHT_CALENDAR: u32 = 131072u32;
pub const MCHT_CALENDARBK: u32 = 131072u32;
pub const MCHT_CALENDARCONTROL: u32 = 1048576u32;
pub const MCHT_NEXT: u32 = 16777216u32;
pub const MCHT_NOWHERE: u32 = 0u32;
pub const MCHT_PREV: u32 = 33554432u32;
pub const MCHT_TITLE: u32 = 65536u32;
pub const MCHT_TITLEBK: u32 = 65536u32;
pub const MCHT_TODAYLINK: u32 = 196608u32;
pub const MCMV_CENTURY: u32 = 3u32;
pub const MCMV_DECADE: u32 = 2u32;
pub const MCMV_MAX: u32 = 3u32;
pub const MCMV_MONTH: u32 = 0u32;
pub const MCMV_YEAR: u32 = 1u32;
pub const MCM_FIRST: u32 = 4096u32;
pub const MCM_GETCALENDARBORDER: u32 = 4127u32;
pub const MCM_GETCALENDARCOUNT: u32 = 4119u32;
pub const MCM_GETCALENDARGRIDINFO: u32 = 4120u32;
pub const MCM_GETCALID: u32 = 4123u32;
pub const MCM_GETCOLOR: u32 = 4107u32;
pub const MCM_GETCURRENTVIEW: u32 = 4118u32;
pub const MCM_GETCURSEL: u32 = 4097u32;
pub const MCM_GETFIRSTDAYOFWEEK: u32 = 4112u32;
pub const MCM_GETMAXSELCOUNT: u32 = 4099u32;
pub const MCM_GETMAXTODAYWIDTH: u32 = 4117u32;
pub const MCM_GETMINREQRECT: u32 = 4105u32;
pub const MCM_GETMONTHDELTA: u32 = 4115u32;
pub const MCM_GETMONTHRANGE: u32 = 4103u32;
pub const MCM_GETRANGE: u32 = 4113u32;
pub const MCM_GETSELRANGE: u32 = 4101u32;
pub const MCM_GETTODAY: u32 = 4109u32;
pub const MCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const MCM_HITTEST: u32 = 4110u32;
pub const MCM_SETCALENDARBORDER: u32 = 4126u32;
pub const MCM_SETCALID: u32 = 4124u32;
pub const MCM_SETCOLOR: u32 = 4106u32;
pub const MCM_SETCURRENTVIEW: u32 = 4128u32;
pub const MCM_SETCURSEL: u32 = 4098u32;
pub const MCM_SETDAYSTATE: u32 = 4104u32;
pub const MCM_SETFIRSTDAYOFWEEK: u32 = 4111u32;
pub const MCM_SETMAXSELCOUNT: u32 = 4100u32;
pub const MCM_SETMONTHDELTA: u32 = 4116u32;
pub const MCM_SETRANGE: u32 = 4114u32;
pub const MCM_SETSELRANGE: u32 = 4102u32;
pub const MCM_SETTODAY: u32 = 4108u32;
pub const MCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const MCM_SIZERECTTOMIN: u32 = 4125u32;
pub const MCSC_BACKGROUND: u32 = 0u32;
pub const MCSC_MONTHBK: u32 = 4u32;
pub const MCSC_TEXT: u32 = 1u32;
pub const MCSC_TITLEBK: u32 = 2u32;
pub const MCSC_TITLETEXT: u32 = 3u32;
pub const MCSC_TRAILINGTEXT: u32 = 5u32;
pub const MCS_DAYSTATE: u32 = 1u32;
pub const MCS_MULTISELECT: u32 = 2u32;
pub const MCS_NOSELCHANGEONNAV: u32 = 256u32;
pub const MCS_NOTODAY: u32 = 16u32;
pub const MCS_NOTODAYCIRCLE: u32 = 8u32;
pub const MCS_NOTRAILINGDATES: u32 = 64u32;
pub const MCS_SHORTDAYSOFWEEK: u32 = 128u32;
pub const MCS_WEEKNUMBERS: u32 = 4u32;
#[repr(C)]
pub struct MEASUREITEMSTRUCT(i32);
#[repr(transparent)]
pub struct MENUBANDPARTS(pub i32);
pub const MDP_NEWAPPBUTTON: MENUBANDPARTS = MENUBANDPARTS(1i32);
pub const MDP_SEPERATOR: MENUBANDPARTS = MENUBANDPARTS(2i32);
#[repr(transparent)]
pub struct MENUBANDSTATES(pub i32);
pub const MDS_NORMAL: MENUBANDSTATES = MENUBANDSTATES(1i32);
pub const MDS_HOT: MENUBANDSTATES = MENUBANDSTATES(2i32);
pub const MDS_PRESSED: MENUBANDSTATES = MENUBANDSTATES(3i32);
pub const MDS_DISABLED: MENUBANDSTATES = MENUBANDSTATES(4i32);
pub const MDS_CHECKED: MENUBANDSTATES = MENUBANDSTATES(5i32);
pub const MDS_HOTCHECKED: MENUBANDSTATES = MENUBANDSTATES(6i32);
#[repr(transparent)]
pub struct MONTHCALPARTS(pub i32);
pub const MC_BACKGROUND: MONTHCALPARTS = MONTHCALPARTS(1i32);
pub const MC_BORDERS: MONTHCALPARTS = MONTHCALPARTS(2i32);
pub const MC_GRIDBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(3i32);
pub const MC_COLHEADERSPLITTER: MONTHCALPARTS = MONTHCALPARTS(4i32);
pub const MC_GRIDCELLBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(5i32);
pub const MC_GRIDCELL: MONTHCALPARTS = MONTHCALPARTS(6i32);
pub const MC_GRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(7i32);
pub const MC_TRAILINGGRIDCELL: MONTHCALPARTS = MONTHCALPARTS(8i32);
pub const MC_TRAILINGGRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(9i32);
pub const MC_NAVNEXT: MONTHCALPARTS = MONTHCALPARTS(10i32);
pub const MC_NAVPREV: MONTHCALPARTS = MONTHCALPARTS(11i32);
#[repr(transparent)]
pub struct MOREPROGRAMSARROWBACKSTATES(pub i32);
pub const SPSB_NORMAL: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(1i32);
pub const SPSB_HOT: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(2i32);
pub const SPSB_PRESSED: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(3i32);
#[repr(transparent)]
pub struct MOREPROGRAMSARROWSTATES(pub i32);
pub const SPS_NORMAL: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(1i32);
pub const SPS_HOT: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(2i32);
pub const SPS_PRESSED: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(3i32);
#[repr(transparent)]
pub struct MOREPROGRAMSTABSTATES(pub i32);
pub const SPMPT_NORMAL: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(1i32);
pub const SPMPT_HOT: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(2i32);
pub const SPMPT_SELECTED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(3i32);
pub const SPMPT_DISABLED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(4i32);
pub const SPMPT_FOCUSED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(5i32);
pub const MSGF_COMMCTRL_BEGINDRAG: u32 = 16896u32;
pub const MSGF_COMMCTRL_DRAGSELECT: u32 = 16898u32;
pub const MSGF_COMMCTRL_SIZEHEADER: u32 = 16897u32;
pub const MSGF_COMMCTRL_TOOLBARCUST: u32 = 16899u32;
pub const MULTIFILEOPENORD: u32 = 1537u32;
#[repr(transparent)]
pub struct NAVNEXTSTATES(pub i32);
pub const MCNN_NORMAL: NAVNEXTSTATES = NAVNEXTSTATES(1i32);
pub const MCNN_HOT: NAVNEXTSTATES = NAVNEXTSTATES(2i32);
pub const MCNN_PRESSED: NAVNEXTSTATES = NAVNEXTSTATES(3i32);
pub const MCNN_DISABLED: NAVNEXTSTATES = NAVNEXTSTATES(4i32);
#[repr(transparent)]
pub struct NAVPREVSTATES(pub i32);
pub const MCNP_NORMAL: NAVPREVSTATES = NAVPREVSTATES(1i32);
pub const MCNP_HOT: NAVPREVSTATES = NAVPREVSTATES(2i32);
pub const MCNP_PRESSED: NAVPREVSTATES = NAVPREVSTATES(3i32);
pub const MCNP_DISABLED: NAVPREVSTATES = NAVPREVSTATES(4i32);
pub const NEWFILEOPENORD: u32 = 1547u32;
pub const NEWFILEOPENV2ORD: u32 = 1552u32;
pub const NEWFILEOPENV3ORD: u32 = 1553u32;
pub const NEWFORMATDLGWITHLINK: u32 = 1591u32;
pub const NFS_ALL: u32 = 16u32;
pub const NFS_BUTTON: u32 = 8u32;
pub const NFS_EDIT: u32 = 1u32;
pub const NFS_LISTCOMBO: u32 = 4u32;
pub const NFS_STATIC: u32 = 2u32;
pub const NFS_USEFONTASSOC: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMBCDROPDOWN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMBCHOTITEM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCBEDRAGBEGINA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCBEDRAGBEGINW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCBEENDEDITA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCBEENDEDITW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCHAR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCOMBOBOXEXA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCOMBOBOXEXW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMCUSTOMDRAW(i32);
#[repr(transparent)]
pub struct NMCUSTOMDRAW_DRAW_STAGE(pub u32);
pub const CDDS_POSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(2u32);
pub const CDDS_PREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(3u32);
pub const CDDS_PREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(1u32);
pub const CDDS_ITEMPOSTERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65540u32);
pub const CDDS_ITEMPOSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65538u32);
pub const CDDS_ITEMPREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65539u32);
pub const CDDS_ITEMPREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65537u32);
pub const CDDS_SUBITEM: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(131072u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMCUSTOMSPLITRECTINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMCUSTOMTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMECHANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMEFORMATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMEFORMATQUERYA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMEFORMATQUERYW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMEFORMATW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMESTRINGA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMESTRINGW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWNA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWNW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMDAYSTATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMHDDISPINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMHDDISPINFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMHDFILTERBTNCLICK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMHDR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMHEADERA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMHEADERW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMIPADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMITEMACTIVATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMKEY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLINK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLISTVIEW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVCACHEHINT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMLVCUSTOMDRAW(i32);
#[repr(transparent)]
pub struct NMLVCUSTOMDRAW_ALIGN(pub u32);
pub const LVGA_HEADER_CENTER: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(2u32);
pub const LVGA_HEADER_LEFT: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(1u32);
pub const LVGA_HEADER_RIGHT: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(4u32);
#[repr(transparent)]
pub struct NMLVCUSTOMDRAW_ITEM_TYPE(pub u32);
pub const LVCDI_ITEM: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(0u32);
pub const LVCDI_GROUP: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(1u32);
pub const LVCDI_ITEMSLIST: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(2u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVDISPINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVDISPINFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVEMPTYMARKUP(i32);
#[repr(transparent)]
pub struct NMLVEMPTYMARKUP_FLAGS(pub u32);
pub const EMF_CENTERED: NMLVEMPTYMARKUP_FLAGS = NMLVEMPTYMARKUP_FLAGS(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVFINDITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVFINDITEMW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVGETINFOTIPA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVGETINFOTIPW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVKEYDOWN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVLINK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVODSTATECHANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMLVSCROLL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMMOUSE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMOBJECTNOTIFY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMPGCALCSIZE(i32);
#[repr(transparent)]
pub struct NMPGCALCSIZE_FLAGS(pub u32);
pub const PGF_CALCHEIGHT: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(2u32);
pub const PGF_CALCWIDTH: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMPGHOTITEM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMPGSCROLL(i32);
#[repr(transparent)]
pub struct NMPGSCROLL_DIR(pub u32);
pub const PGF_SCROLLDOWN: NMPGSCROLL_DIR = NMPGSCROLL_DIR(2u32);
pub const PGF_SCROLLLEFT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(4u32);
pub const PGF_SCROLLRIGHT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(8u32);
pub const PGF_SCROLLUP: NMPGSCROLL_DIR = NMPGSCROLL_DIR(1u32);
#[repr(transparent)]
pub struct NMPGSCROLL_KEYS(pub u16);
pub const PGK_NONE: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(0u16);
pub const PGK_SHIFT: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(1u16);
pub const PGK_CONTROL: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(2u16);
pub const PGK_MENU: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(4u16);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMRBAUTOSIZE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMREBAR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMREBARAUTOBREAK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMREBARCHEVRON(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMREBARCHILDSIZE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMREBARSPLITTER(i32);
#[repr(transparent)]
pub struct NMREBAR_MASK_FLAGS(pub u32);
pub const RBNM_ID: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(1u32);
pub const RBNM_LPARAM: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(4u32);
pub const RBNM_STYLE: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(2u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMSEARCHWEB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMSELCHANGE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMTBCUSTOMDRAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBDISPINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBDISPINFOW(i32);
#[repr(transparent)]
pub struct NMTBDISPINFOW_MASK(pub u32);
pub const TBNF_IMAGE: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(1u32);
pub const TBNF_TEXT: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(2u32);
pub const TBNF_DI_SETITEM: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(268435456u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBGETINFOTIPA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBGETINFOTIPW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBHOTITEM(i32);
#[repr(transparent)]
pub struct NMTBHOTITEM_FLAGS(pub u32);
pub const HICF_ACCELERATOR: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(4u32);
pub const HICF_ARROWKEYS: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(2u32);
pub const HICF_DUPACCEL: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(8u32);
pub const HICF_ENTERING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(16u32);
pub const HICF_LEAVING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(32u32);
pub const HICF_LMOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(128u32);
pub const HICF_MOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(1u32);
pub const HICF_OTHER: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(0u32);
pub const HICF_RESELECT: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(64u32);
pub const HICF_TOGGLEDROPDOWN: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(256u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBRESTORE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTBSAVE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTCKEYDOWN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTOOLBARA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTOOLBARW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTOOLTIPSCREATED(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTRBTHUMBPOSCHANGING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTREEVIEWA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTREEVIEWW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMTTCUSTOMDRAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTTDISPINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTTDISPINFOW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMTVASYNCDRAW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct NMTVCUSTOMDRAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVDISPINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVDISPINFOEXA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVDISPINFOEXW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVDISPINFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVGETINFOTIPA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVGETINFOTIPW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVITEMCHANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVKEYDOWN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMTVSTATEIMAGECHANGING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMUPDOWN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NMVIEWCHANGE(i32);
pub const NM_GETCUSTOMSPLITRECT: u32 = 4294966049u32;
pub const ODT_HEADER: u32 = 100u32;
#[repr(transparent)]
pub struct OFFSETTYPE(pub i32);
pub const OT_TOPLEFT: OFFSETTYPE = OFFSETTYPE(0i32);
pub const OT_TOPRIGHT: OFFSETTYPE = OFFSETTYPE(1i32);
pub const OT_TOPMIDDLE: OFFSETTYPE = OFFSETTYPE(2i32);
pub const OT_BOTTOMLEFT: OFFSETTYPE = OFFSETTYPE(3i32);
pub const OT_BOTTOMRIGHT: OFFSETTYPE = OFFSETTYPE(4i32);
pub const OT_BOTTOMMIDDLE: OFFSETTYPE = OFFSETTYPE(5i32);
pub const OT_MIDDLELEFT: OFFSETTYPE = OFFSETTYPE(6i32);
pub const OT_MIDDLERIGHT: OFFSETTYPE = OFFSETTYPE(7i32);
pub const OT_LEFTOFCAPTION: OFFSETTYPE = OFFSETTYPE(8i32);
pub const OT_RIGHTOFCAPTION: OFFSETTYPE = OFFSETTYPE(9i32);
pub const OT_LEFTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(10i32);
pub const OT_RIGHTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(11i32);
pub const OT_ABOVELASTBUTTON: OFFSETTYPE = OFFSETTYPE(12i32);
pub const OT_BELOWLASTBUTTON: OFFSETTYPE = OFFSETTYPE(13i32);
#[repr(transparent)]
pub struct OPENBOXSTATES(pub i32);
pub const SPOB_NORMAL: OPENBOXSTATES = OPENBOXSTATES(1i32);
pub const SPOB_HOT: OPENBOXSTATES = OPENBOXSTATES(2i32);
pub const SPOB_SELECTED: OPENBOXSTATES = OPENBOXSTATES(3i32);
pub const SPOB_DISABLED: OPENBOXSTATES = OPENBOXSTATES(4i32);
pub const SPOB_FOCUSED: OPENBOXSTATES = OPENBOXSTATES(5i32);
#[repr(transparent)]
pub struct OPEN_THEME_DATA_FLAGS(pub u32);
pub const OTD_FORCE_RECT_SIZING: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(1u32);
pub const OTD_NONCLIENT: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(2u32);
#[repr(transparent)]
pub struct PAGEPARTS(pub i32);
pub const PGRP_UP: PAGEPARTS = PAGEPARTS(1i32);
pub const PGRP_DOWN: PAGEPARTS = PAGEPARTS(2i32);
pub const PGRP_UPHORZ: PAGEPARTS = PAGEPARTS(3i32);
pub const PGRP_DOWNHORZ: PAGEPARTS = PAGEPARTS(4i32);
pub const PAGESETUPDLGORD: u32 = 1546u32;
pub const PAGESETUPDLGORDMOTIF: u32 = 1550u32;
pub const PBM_DELTAPOS: u32 = 1027u32;
pub const PBM_GETBARCOLOR: u32 = 1039u32;
pub const PBM_GETBKCOLOR: u32 = 1038u32;
pub const PBM_GETPOS: u32 = 1032u32;
pub const PBM_GETRANGE: u32 = 1031u32;
pub const PBM_GETSTATE: u32 = 1041u32;
pub const PBM_GETSTEP: u32 = 1037u32;
pub const PBM_SETBARCOLOR: u32 = 1033u32;
pub const PBM_SETBKCOLOR: u32 = 8193u32;
pub const PBM_SETMARQUEE: u32 = 1034u32;
pub const PBM_SETPOS: u32 = 1026u32;
pub const PBM_SETRANGE: u32 = 1025u32;
pub const PBM_SETRANGE32: u32 = 1030u32;
pub const PBM_SETSTATE: u32 = 1040u32;
pub const PBM_SETSTEP: u32 = 1028u32;
pub const PBM_STEPIT: u32 = 1029u32;
#[repr(C)]
pub struct PBRANGE(i32);
pub const PBST_ERROR: u32 = 2u32;
pub const PBST_NORMAL: u32 = 1u32;
pub const PBST_PAUSED: u32 = 3u32;
pub const PBS_MARQUEE: u32 = 8u32;
pub const PBS_SMOOTH: u32 = 1u32;
pub const PBS_SMOOTHREVERSE: u32 = 16u32;
pub const PBS_VERTICAL: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDACOMPARE = unsafe extern "system" fn(p1: *const ::core::ffi::c_void, p2: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDACOMPARECONST = unsafe extern "system" fn(p1: *const ::core::ffi::c_void, p2: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
pub type PFNDAENUMCALLBACK = unsafe extern "system" fn(p: *const ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> i32;
pub type PFNDAENUMCALLBACKCONST = unsafe extern "system" fn(p: *const ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDPAMERGE = unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::core::ffi::c_void, pvsrc: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDPAMERGECONST = unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::core::ffi::c_void, pvsrc: *const ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_System_Com")]
pub type PFNDPASTREAM = unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: super::super::System::Com::IStream, pvinstdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNLVCOMPARE = unsafe extern "system" fn(param0: super::super::Foundation::LPARAM, param1: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> i32;
pub type PFNLVGROUPCOMPARE = unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut ::core::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNPROPSHEETCALLBACK = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNTVCOMPARE = unsafe extern "system" fn(lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM, lparamsort: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFTASKDIALOGCALLBACK = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lprefdata: isize) -> ::windows_sys::core::HRESULT;
pub const PGB_BOTTOMORRIGHT: u32 = 1u32;
pub const PGB_TOPORLEFT: u32 = 0u32;
pub const PGF_DEPRESSED: u32 = 4u32;
pub const PGF_GRAYED: u32 = 2u32;
pub const PGF_HOT: u32 = 8u32;
pub const PGF_INVISIBLE: u32 = 0u32;
pub const PGF_NORMAL: u32 = 1u32;
pub const PGM_FIRST: u32 = 5120u32;
pub const PGM_FORWARDMOUSE: u32 = 5123u32;
pub const PGM_GETBKCOLOR: u32 = 5125u32;
pub const PGM_GETBORDER: u32 = 5127u32;
pub const PGM_GETBUTTONSIZE: u32 = 5131u32;
pub const PGM_GETBUTTONSTATE: u32 = 5132u32;
pub const PGM_GETDROPTARGET: u32 = 8196u32;
pub const PGM_GETPOS: u32 = 5129u32;
pub const PGM_RECALCSIZE: u32 = 5122u32;
pub const PGM_SETBKCOLOR: u32 = 5124u32;
pub const PGM_SETBORDER: u32 = 5126u32;
pub const PGM_SETBUTTONSIZE: u32 = 5130u32;
pub const PGM_SETCHILD: u32 = 5121u32;
pub const PGM_SETPOS: u32 = 5128u32;
pub const PGM_SETSCROLLINFO: u32 = 5133u32;
pub const PGS_AUTOSCROLL: u32 = 2u32;
pub const PGS_DRAGNDROP: u32 = 4u32;
pub const PGS_HORZ: u32 = 1u32;
pub const PGS_VERT: u32 = 0u32;
#[repr(C)]
pub struct POINTER_DEVICE_CURSOR_INFO(i32);
#[repr(transparent)]
pub struct POINTER_DEVICE_CURSOR_TYPE(pub i32);
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(0i32);
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(1i32);
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(2i32);
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(-1i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct POINTER_DEVICE_INFO(i32);
#[repr(C)]
pub struct POINTER_DEVICE_PROPERTY(i32);
#[repr(transparent)]
pub struct POINTER_DEVICE_TYPE(pub i32);
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(1i32);
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(2i32);
pub const POINTER_DEVICE_TYPE_TOUCH: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(3i32);
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(4i32);
pub const POINTER_DEVICE_TYPE_MAX: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(-1i32);
#[repr(transparent)]
pub struct POINTER_FEEDBACK_MODE(pub i32);
pub const POINTER_FEEDBACK_DEFAULT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(1i32);
pub const POINTER_FEEDBACK_INDIRECT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(2i32);
pub const POINTER_FEEDBACK_NONE: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(3i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct POINTER_TYPE_INFO(i32);
pub const PRINTDLGEXORD: u32 = 1549u32;
pub const PRINTDLGORD: u32 = 1538u32;
pub const PRNSETUPDLGORD: u32 = 1539u32;
#[repr(transparent)]
pub struct PROPERTYORIGIN(pub i32);
pub const PO_STATE: PROPERTYORIGIN = PROPERTYORIGIN(0i32);
pub const PO_PART: PROPERTYORIGIN = PROPERTYORIGIN(1i32);
pub const PO_CLASS: PROPERTYORIGIN = PROPERTYORIGIN(2i32);
pub const PO_GLOBAL: PROPERTYORIGIN = PROPERTYORIGIN(3i32);
pub const PO_NOTFOUND: PROPERTYORIGIN = PROPERTYORIGIN(4i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETHEADERA_V1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETHEADERA_V2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETHEADERW_V1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETHEADERW_V2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEA_V1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEA_V2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEA_V3(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEW_V1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEW_V2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct PROPSHEETPAGEW_V3(i32);
pub const PROP_LG_CXDLG: u32 = 252u32;
pub const PROP_LG_CYDLG: u32 = 218u32;
pub const PROP_MED_CXDLG: u32 = 227u32;
pub const PROP_MED_CYDLG: u32 = 215u32;
pub const PROP_SM_CXDLG: u32 = 212u32;
pub const PROP_SM_CYDLG: u32 = 188u32;
pub const PSBTN_APPLYNOW: u32 = 4u32;
pub const PSBTN_BACK: u32 = 0u32;
pub const PSBTN_CANCEL: u32 = 5u32;
pub const PSBTN_FINISH: u32 = 2u32;
pub const PSBTN_HELP: u32 = 6u32;
pub const PSBTN_MAX: u32 = 6u32;
pub const PSBTN_NEXT: u32 = 1u32;
pub const PSBTN_OK: u32 = 3u32;
pub const PSCB_BUTTONPRESSED: u32 = 3u32;
pub const PSCB_INITIALIZED: u32 = 1u32;
pub const PSCB_PRECREATE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PSHNOTIFY(i32);
pub const PSH_AEROWIZARD: u32 = 16384u32;
pub const PSH_DEFAULT: u32 = 0u32;
pub const PSH_HASHELP: u32 = 512u32;
pub const PSH_HEADER: u32 = 524288u32;
pub const PSH_HEADERBITMAP: u32 = 134217728u32;
pub const PSH_MODELESS: u32 = 1024u32;
pub const PSH_NOAPPLYNOW: u32 = 128u32;
pub const PSH_NOCONTEXTHELP: u32 = 33554432u32;
pub const PSH_NOMARGIN: u32 = 268435456u32;
pub const PSH_PROPSHEETPAGE: u32 = 8u32;
pub const PSH_PROPTITLE: u32 = 1u32;
pub const PSH_RESIZABLE: u32 = 67108864u32;
pub const PSH_RTLREADING: u32 = 2048u32;
pub const PSH_STRETCHWATERMARK: u32 = 262144u32;
pub const PSH_USECALLBACK: u32 = 256u32;
pub const PSH_USEHBMHEADER: u32 = 1048576u32;
pub const PSH_USEHBMWATERMARK: u32 = 65536u32;
pub const PSH_USEHICON: u32 = 2u32;
pub const PSH_USEHPLWATERMARK: u32 = 131072u32;
pub const PSH_USEICONID: u32 = 4u32;
pub const PSH_USEPAGELANG: u32 = 2097152u32;
pub const PSH_USEPSTARTPAGE: u32 = 64u32;
pub const PSH_WATERMARK: u32 = 32768u32;
pub const PSH_WIZARD: u32 = 32u32;
pub const PSH_WIZARD97: u32 = 8192u32;
pub const PSH_WIZARDCONTEXTHELP: u32 = 4096u32;
pub const PSH_WIZARDHASFINISH: u32 = 16u32;
pub const PSH_WIZARD_LITE: u32 = 4194304u32;
pub const PSM_ADDPAGE: u32 = 1127u32;
pub const PSM_APPLY: u32 = 1134u32;
pub const PSM_CANCELTOCLOSE: u32 = 1131u32;
pub const PSM_CHANGED: u32 = 1128u32;
pub const PSM_ENABLEWIZBUTTONS: u32 = 1163u32;
pub const PSM_GETCURRENTPAGEHWND: u32 = 1142u32;
pub const PSM_GETRESULT: u32 = 1159u32;
pub const PSM_GETTABCONTROL: u32 = 1140u32;
pub const PSM_HWNDTOINDEX: u32 = 1153u32;
pub const PSM_IDTOINDEX: u32 = 1157u32;
pub const PSM_INDEXTOHWND: u32 = 1154u32;
pub const PSM_INDEXTOID: u32 = 1158u32;
pub const PSM_INDEXTOPAGE: u32 = 1156u32;
pub const PSM_INSERTPAGE: u32 = 1143u32;
pub const PSM_ISDIALOGMESSAGE: u32 = 1141u32;
pub const PSM_PAGETOINDEX: u32 = 1155u32;
pub const PSM_PRESSBUTTON: u32 = 1137u32;
pub const PSM_QUERYSIBLINGS: u32 = 1132u32;
pub const PSM_REBOOTSYSTEM: u32 = 1130u32;
pub const PSM_RECALCPAGESIZES: u32 = 1160u32;
pub const PSM_REMOVEPAGE: u32 = 1126u32;
pub const PSM_RESTARTWINDOWS: u32 = 1129u32;
pub const PSM_SETBUTTONTEXT: u32 = 1164u32;
pub const PSM_SETBUTTONTEXTW: u32 = 1164u32;
pub const PSM_SETCURSEL: u32 = 1125u32;
pub const PSM_SETCURSELID: u32 = 1138u32;
pub const PSM_SETFINISHTEXT: u32 = 1145u32;
pub const PSM_SETFINISHTEXTA: u32 = 1139u32;
pub const PSM_SETFINISHTEXTW: u32 = 1145u32;
pub const PSM_SETHEADERSUBTITLE: u32 = 1152u32;
pub const PSM_SETHEADERSUBTITLEA: u32 = 1151u32;
pub const PSM_SETHEADERSUBTITLEW: u32 = 1152u32;
pub const PSM_SETHEADERTITLE: u32 = 1150u32;
pub const PSM_SETHEADERTITLEA: u32 = 1149u32;
pub const PSM_SETHEADERTITLEW: u32 = 1150u32;
pub const PSM_SETNEXTTEXT: u32 = 1161u32;
pub const PSM_SETNEXTTEXTW: u32 = 1161u32;
pub const PSM_SETTITLE: u32 = 1144u32;
pub const PSM_SETTITLEA: u32 = 1135u32;
pub const PSM_SETTITLEW: u32 = 1144u32;
pub const PSM_SETWIZBUTTONS: u32 = 1136u32;
pub const PSM_SHOWWIZBUTTONS: u32 = 1162u32;
pub const PSM_UNCHANGED: u32 = 1133u32;
pub const PSNRET_INVALID: u32 = 1u32;
pub const PSNRET_INVALID_NOCHANGEPAGE: u32 = 2u32;
pub const PSNRET_MESSAGEHANDLED: u32 = 3u32;
pub const PSNRET_NOERROR: u32 = 0u32;
#[repr(transparent)]
pub struct PSPCB_MESSAGE(pub u32);
pub const PSPCB_ADDREF: PSPCB_MESSAGE = PSPCB_MESSAGE(0u32);
pub const PSPCB_CREATE: PSPCB_MESSAGE = PSPCB_MESSAGE(2u32);
pub const PSPCB_RELEASE: PSPCB_MESSAGE = PSPCB_MESSAGE(1u32);
pub const PSPCB_SI_INITDIALOG: PSPCB_MESSAGE = PSPCB_MESSAGE(1025u32);
pub const PSP_DEFAULT: u32 = 0u32;
pub const PSP_DLGINDIRECT: u32 = 1u32;
pub const PSP_HASHELP: u32 = 32u32;
pub const PSP_HIDEHEADER: u32 = 2048u32;
pub const PSP_PREMATURE: u32 = 1024u32;
pub const PSP_RTLREADING: u32 = 16u32;
pub const PSP_USECALLBACK: u32 = 128u32;
pub const PSP_USEFUSIONCONTEXT: u32 = 16384u32;
pub const PSP_USEHEADERSUBTITLE: u32 = 8192u32;
pub const PSP_USEHEADERTITLE: u32 = 4096u32;
pub const PSP_USEHICON: u32 = 2u32;
pub const PSP_USEICONID: u32 = 4u32;
pub const PSP_USEREFPARENT: u32 = 64u32;
pub const PSP_USETITLE: u32 = 8u32;
pub const PSWIZBF_ELEVATIONREQUIRED: u32 = 1u32;
pub const PSWIZB_BACK: u32 = 1u32;
pub const PSWIZB_CANCEL: u32 = 16u32;
pub const PSWIZB_DISABLEDFINISH: u32 = 8u32;
pub const PSWIZB_FINISH: u32 = 4u32;
pub const PSWIZB_NEXT: u32 = 2u32;
pub const PSWIZB_RESTORE: u32 = 1u32;
pub const PSWIZB_SHOW: u32 = 0u32;
pub const RBAB_ADDBAND: u32 = 2u32;
pub const RBAB_AUTOSIZE: u32 = 1u32;
pub const RBBIM_BACKGROUND: u32 = 128u32;
pub const RBBIM_CHEVRONLOCATION: u32 = 4096u32;
pub const RBBIM_CHEVRONSTATE: u32 = 8192u32;
pub const RBBIM_CHILD: u32 = 16u32;
pub const RBBIM_CHILDSIZE: u32 = 32u32;
pub const RBBIM_COLORS: u32 = 2u32;
pub const RBBIM_HEADERSIZE: u32 = 2048u32;
pub const RBBIM_ID: u32 = 256u32;
pub const RBBIM_IDEALSIZE: u32 = 512u32;
pub const RBBIM_IMAGE: u32 = 8u32;
pub const RBBIM_LPARAM: u32 = 1024u32;
pub const RBBIM_SIZE: u32 = 64u32;
pub const RBBIM_STYLE: u32 = 1u32;
pub const RBBIM_TEXT: u32 = 4u32;
pub const RBBS_BREAK: u32 = 1u32;
pub const RBBS_CHILDEDGE: u32 = 4u32;
pub const RBBS_FIXEDBMP: u32 = 32u32;
pub const RBBS_FIXEDSIZE: u32 = 2u32;
pub const RBBS_GRIPPERALWAYS: u32 = 128u32;
pub const RBBS_HIDDEN: u32 = 8u32;
pub const RBBS_HIDETITLE: u32 = 1024u32;
pub const RBBS_NOGRIPPER: u32 = 256u32;
pub const RBBS_NOVERT: u32 = 16u32;
pub const RBBS_TOPALIGN: u32 = 2048u32;
pub const RBBS_USECHEVRON: u32 = 512u32;
pub const RBBS_VARIABLEHEIGHT: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RBHITTESTINFO(i32);
pub const RBHT_CAPTION: u32 = 2u32;
pub const RBHT_CHEVRON: u32 = 8u32;
pub const RBHT_CLIENT: u32 = 3u32;
pub const RBHT_GRABBER: u32 = 4u32;
pub const RBHT_NOWHERE: u32 = 1u32;
pub const RBHT_SPLITTER: u32 = 16u32;
pub const RBIM_IMAGELIST: u32 = 1u32;
pub const RBSTR_CHANGERECT: u32 = 1u32;
pub const RBS_AUTOSIZE: u32 = 8192u32;
pub const RBS_BANDBORDERS: u32 = 1024u32;
pub const RBS_DBLCLKTOGGLE: u32 = 32768u32;
pub const RBS_FIXEDORDER: u32 = 2048u32;
pub const RBS_REGISTERDROP: u32 = 4096u32;
pub const RBS_TOOLTIPS: u32 = 256u32;
pub const RBS_VARHEIGHT: u32 = 512u32;
pub const RBS_VERTICALGRIPPER: u32 = 16384u32;
pub const RB_BEGINDRAG: u32 = 1048u32;
pub const RB_DELETEBAND: u32 = 1026u32;
pub const RB_DRAGMOVE: u32 = 1050u32;
pub const RB_ENDDRAG: u32 = 1049u32;
pub const RB_GETBANDBORDERS: u32 = 1058u32;
pub const RB_GETBANDCOUNT: u32 = 1036u32;
pub const RB_GETBANDINFO: u32 = 1052u32;
pub const RB_GETBANDINFOA: u32 = 1053u32;
pub const RB_GETBANDINFOW: u32 = 1052u32;
pub const RB_GETBANDMARGINS: u32 = 1064u32;
pub const RB_GETBARHEIGHT: u32 = 1051u32;
pub const RB_GETBARINFO: u32 = 1027u32;
pub const RB_GETBKCOLOR: u32 = 1044u32;
pub const RB_GETCOLORSCHEME: u32 = 8195u32;
pub const RB_GETDROPTARGET: u32 = 8196u32;
pub const RB_GETEXTENDEDSTYLE: u32 = 1066u32;
pub const RB_GETPALETTE: u32 = 1062u32;
pub const RB_GETRECT: u32 = 1033u32;
pub const RB_GETROWCOUNT: u32 = 1037u32;
pub const RB_GETROWHEIGHT: u32 = 1038u32;
pub const RB_GETTEXTCOLOR: u32 = 1046u32;
pub const RB_GETTOOLTIPS: u32 = 1041u32;
pub const RB_GETUNICODEFORMAT: u32 = 8198u32;
pub const RB_HITTEST: u32 = 1032u32;
pub const RB_IDTOINDEX: u32 = 1040u32;
pub const RB_INSERTBAND: u32 = 1034u32;
pub const RB_INSERTBANDA: u32 = 1025u32;
pub const RB_INSERTBANDW: u32 = 1034u32;
pub const RB_MAXIMIZEBAND: u32 = 1055u32;
pub const RB_MINIMIZEBAND: u32 = 1054u32;
pub const RB_MOVEBAND: u32 = 1063u32;
pub const RB_PUSHCHEVRON: u32 = 1067u32;
pub const RB_SETBANDINFO: u32 = 1035u32;
pub const RB_SETBANDINFOA: u32 = 1030u32;
pub const RB_SETBANDINFOW: u32 = 1035u32;
pub const RB_SETBANDWIDTH: u32 = 1068u32;
pub const RB_SETBARINFO: u32 = 1028u32;
pub const RB_SETBKCOLOR: u32 = 1043u32;
pub const RB_SETCOLORSCHEME: u32 = 8194u32;
pub const RB_SETEXTENDEDSTYLE: u32 = 1065u32;
pub const RB_SETPALETTE: u32 = 1061u32;
pub const RB_SETPARENT: u32 = 1031u32;
pub const RB_SETTEXTCOLOR: u32 = 1045u32;
pub const RB_SETTOOLTIPS: u32 = 1042u32;
pub const RB_SETUNICODEFORMAT: u32 = 8197u32;
pub const RB_SETWINDOWTHEME: u32 = 8203u32;
pub const RB_SHOWBAND: u32 = 1059u32;
pub const RB_SIZETORECT: u32 = 1047u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct REBARBANDINFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct REBARBANDINFOW(i32);
#[repr(C)]
pub struct REBARINFO(i32);
pub const REPLACEDLGORD: u32 = 1541u32;
pub const RUNDLGORD: u32 = 1545u32;
pub const SBARS_SIZEGRIP: u32 = 256u32;
pub const SBARS_TOOLTIPS: u32 = 2048u32;
pub const SBT_NOBORDERS: u32 = 256u32;
pub const SBT_NOTABPARSING: u32 = 2048u32;
pub const SBT_OWNERDRAW: u32 = 4096u32;
pub const SBT_POPOUT: u32 = 512u32;
pub const SBT_RTLREADING: u32 = 1024u32;
pub const SBT_TOOLTIPS: u32 = 2048u32;
pub const SB_GETBORDERS: u32 = 1031u32;
pub const SB_GETICON: u32 = 1044u32;
pub const SB_GETPARTS: u32 = 1030u32;
pub const SB_GETRECT: u32 = 1034u32;
pub const SB_GETTEXT: u32 = 1037u32;
pub const SB_GETTEXTA: u32 = 1026u32;
pub const SB_GETTEXTLENGTH: u32 = 1036u32;
pub const SB_GETTEXTLENGTHA: u32 = 1027u32;
pub const SB_GETTEXTLENGTHW: u32 = 1036u32;
pub const SB_GETTEXTW: u32 = 1037u32;
pub const SB_GETTIPTEXTA: u32 = 1042u32;
pub const SB_GETTIPTEXTW: u32 = 1043u32;
pub const SB_GETUNICODEFORMAT: u32 = 8198u32;
pub const SB_ISSIMPLE: u32 = 1038u32;
pub const SB_SETBKCOLOR: u32 = 8193u32;
pub const SB_SETICON: u32 = 1039u32;
pub const SB_SETMINHEIGHT: u32 = 1032u32;
pub const SB_SETPARTS: u32 = 1028u32;
pub const SB_SETTEXT: u32 = 1035u32;
pub const SB_SETTEXTA: u32 = 1025u32;
pub const SB_SETTEXTW: u32 = 1035u32;
pub const SB_SETTIPTEXTA: u32 = 1040u32;
pub const SB_SETTIPTEXTW: u32 = 1041u32;
pub const SB_SETUNICODEFORMAT: u32 = 8197u32;
pub const SB_SIMPLE: u32 = 1033u32;
pub const SB_SIMPLEID: u32 = 255u32;
#[repr(transparent)]
pub struct SIZINGTYPE(pub i32);
pub const ST_TRUESIZE: SIZINGTYPE = SIZINGTYPE(0i32);
pub const ST_STRETCH: SIZINGTYPE = SIZINGTYPE(1i32);
pub const ST_TILE: SIZINGTYPE = SIZINGTYPE(2i32);
#[repr(transparent)]
pub struct SOFTWAREEXPLORERSTATES(pub i32);
pub const SPSE_NORMAL: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(1i32);
pub const SPSE_HOT: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(2i32);
pub const SPSE_SELECTED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(3i32);
pub const SPSE_DISABLED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(4i32);
pub const SPSE_FOCUSED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(5i32);
#[repr(transparent)]
pub struct STARTPANELPARTS(pub i32);
pub const SPP_USERPANE: STARTPANELPARTS = STARTPANELPARTS(1i32);
pub const SPP_MOREPROGRAMS: STARTPANELPARTS = STARTPANELPARTS(2i32);
pub const SPP_MOREPROGRAMSARROW: STARTPANELPARTS = STARTPANELPARTS(3i32);
pub const SPP_PROGLIST: STARTPANELPARTS = STARTPANELPARTS(4i32);
pub const SPP_PROGLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(5i32);
pub const SPP_PLACESLIST: STARTPANELPARTS = STARTPANELPARTS(6i32);
pub const SPP_PLACESLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(7i32);
pub const SPP_LOGOFF: STARTPANELPARTS = STARTPANELPARTS(8i32);
pub const SPP_LOGOFFBUTTONS: STARTPANELPARTS = STARTPANELPARTS(9i32);
pub const SPP_USERPICTURE: STARTPANELPARTS = STARTPANELPARTS(10i32);
pub const SPP_PREVIEW: STARTPANELPARTS = STARTPANELPARTS(11i32);
pub const SPP_MOREPROGRAMSTAB: STARTPANELPARTS = STARTPANELPARTS(12i32);
pub const SPP_NSCHOST: STARTPANELPARTS = STARTPANELPARTS(13i32);
pub const SPP_SOFTWAREEXPLORER: STARTPANELPARTS = STARTPANELPARTS(14i32);
pub const SPP_OPENBOX: STARTPANELPARTS = STARTPANELPARTS(15i32);
pub const SPP_SEARCHVIEW: STARTPANELPARTS = STARTPANELPARTS(16i32);
pub const SPP_MOREPROGRAMSARROWBACK: STARTPANELPARTS = STARTPANELPARTS(17i32);
pub const SPP_TOPMATCH: STARTPANELPARTS = STARTPANELPARTS(18i32);
pub const SPP_LOGOFFSPLITBUTTONDROPDOWN: STARTPANELPARTS = STARTPANELPARTS(19i32);
#[repr(transparent)]
pub struct STATICPARTS(pub i32);
pub const STAT_TEXT: STATICPARTS = STATICPARTS(1i32);
pub const STD_COPY: u32 = 1u32;
pub const STD_CUT: u32 = 0u32;
pub const STD_DELETE: u32 = 5u32;
pub const STD_FILENEW: u32 = 6u32;
pub const STD_FILEOPEN: u32 = 7u32;
pub const STD_FILESAVE: u32 = 8u32;
pub const STD_FIND: u32 = 12u32;
pub const STD_HELP: u32 = 11u32;
pub const STD_PASTE: u32 = 2u32;
pub const STD_PRINT: u32 = 14u32;
pub const STD_PRINTPRE: u32 = 9u32;
pub const STD_PROPERTIES: u32 = 10u32;
pub const STD_REDOW: u32 = 4u32;
pub const STD_REPLACE: u32 = 13u32;
pub const STD_UNDO: u32 = 3u32;
#[repr(transparent)]
pub struct TASKBANDPARTS(pub i32);
pub const TDP_GROUPCOUNT: TASKBANDPARTS = TASKBANDPARTS(1i32);
pub const TDP_FLASHBUTTON: TASKBANDPARTS = TASKBANDPARTS(2i32);
pub const TDP_FLASHBUTTONGROUPMENU: TASKBANDPARTS = TASKBANDPARTS(3i32);
#[repr(transparent)]
pub struct TASKBARPARTS(pub i32);
pub const TBP_BACKGROUNDBOTTOM: TASKBARPARTS = TASKBARPARTS(1i32);
pub const TBP_BACKGROUNDRIGHT: TASKBARPARTS = TASKBARPARTS(2i32);
pub const TBP_BACKGROUNDTOP: TASKBARPARTS = TASKBARPARTS(3i32);
pub const TBP_BACKGROUNDLEFT: TASKBARPARTS = TASKBARPARTS(4i32);
pub const TBP_SIZINGBARBOTTOM: TASKBARPARTS = TASKBARPARTS(5i32);
pub const TBP_SIZINGBARRIGHT: TASKBARPARTS = TASKBARPARTS(6i32);
pub const TBP_SIZINGBARTOP: TASKBARPARTS = TASKBARPARTS(7i32);
pub const TBP_SIZINGBARLEFT: TASKBARPARTS = TASKBARPARTS(8i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct TASKDIALOGCONFIG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TASKDIALOG_BUTTON(i32);
#[repr(transparent)]
pub struct TASKDIALOG_COMMON_BUTTON_FLAGS(pub i32);
pub const TDCBF_OK_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(1i32);
pub const TDCBF_YES_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(2i32);
pub const TDCBF_NO_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(4i32);
pub const TDCBF_CANCEL_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(8i32);
pub const TDCBF_RETRY_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(16i32);
pub const TDCBF_CLOSE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(32i32);
#[repr(transparent)]
pub struct TASKDIALOG_ELEMENTS(pub i32);
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(0i32);
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(1i32);
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(2i32);
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(3i32);
#[repr(transparent)]
pub struct TASKDIALOG_FLAGS(pub i32);
pub const TDF_ENABLE_HYPERLINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1i32);
pub const TDF_USE_HICON_MAIN: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2i32);
pub const TDF_USE_HICON_FOOTER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4i32);
pub const TDF_ALLOW_DIALOG_CANCELLATION: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8i32);
pub const TDF_USE_COMMAND_LINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16i32);
pub const TDF_USE_COMMAND_LINKS_NO_ICON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32i32);
pub const TDF_EXPAND_FOOTER_AREA: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(64i32);
pub const TDF_EXPANDED_BY_DEFAULT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(128i32);
pub const TDF_VERIFICATION_FLAG_CHECKED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(256i32);
pub const TDF_SHOW_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(512i32);
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1024i32);
pub const TDF_CALLBACK_TIMER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2048i32);
pub const TDF_POSITION_RELATIVE_TO_WINDOW: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4096i32);
pub const TDF_RTL_LAYOUT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8192i32);
pub const TDF_NO_DEFAULT_RADIO_BUTTON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16384i32);
pub const TDF_CAN_BE_MINIMIZED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32768i32);
pub const TDF_NO_SET_FOREGROUND: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(65536i32);
pub const TDF_SIZE_TO_CONTENT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16777216i32);
#[repr(transparent)]
pub struct TASKDIALOG_ICON_ELEMENTS(pub i32);
pub const TDIE_ICON_MAIN: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(0i32);
pub const TDIE_ICON_FOOTER: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(1i32);
#[repr(transparent)]
pub struct TASKDIALOG_MESSAGES(pub i32);
pub const TDM_NAVIGATE_PAGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1125i32);
pub const TDM_CLICK_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1126i32);
pub const TDM_SET_MARQUEE_PROGRESS_BAR: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1127i32);
pub const TDM_SET_PROGRESS_BAR_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1128i32);
pub const TDM_SET_PROGRESS_BAR_RANGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1129i32);
pub const TDM_SET_PROGRESS_BAR_POS: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1130i32);
pub const TDM_SET_PROGRESS_BAR_MARQUEE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1131i32);
pub const TDM_SET_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1132i32);
pub const TDM_CLICK_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1134i32);
pub const TDM_ENABLE_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1135i32);
pub const TDM_ENABLE_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1136i32);
pub const TDM_CLICK_VERIFICATION: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1137i32);
pub const TDM_UPDATE_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1138i32);
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1139i32);
pub const TDM_UPDATE_ICON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1140i32);
#[repr(transparent)]
pub struct TASKDIALOG_NOTIFICATIONS(pub i32);
pub const TDN_CREATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(0i32);
pub const TDN_NAVIGATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(1i32);
pub const TDN_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(2i32);
pub const TDN_HYPERLINK_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(3i32);
pub const TDN_TIMER: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(4i32);
pub const TDN_DESTROYED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(5i32);
pub const TDN_RADIO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(6i32);
pub const TDN_DIALOG_CONSTRUCTED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(7i32);
pub const TDN_VERIFICATION_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(8i32);
pub const TDN_HELP: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(9i32);
pub const TDN_EXPANDO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(10i32);
#[repr(C)]
pub struct TA_CUBIC_BEZIER(i32);
#[repr(transparent)]
pub struct TA_PROPERTY(pub i32);
pub const TAP_FLAGS: TA_PROPERTY = TA_PROPERTY(0i32);
pub const TAP_TRANSFORMCOUNT: TA_PROPERTY = TA_PROPERTY(1i32);
pub const TAP_STAGGERDELAY: TA_PROPERTY = TA_PROPERTY(2i32);
pub const TAP_STAGGERDELAYCAP: TA_PROPERTY = TA_PROPERTY(3i32);
pub const TAP_STAGGERDELAYFACTOR: TA_PROPERTY = TA_PROPERTY(4i32);
pub const TAP_ZORDER: TA_PROPERTY = TA_PROPERTY(5i32);
#[repr(transparent)]
pub struct TA_PROPERTY_FLAG(pub u32);
pub const TAPF_NONE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(0u32);
pub const TAPF_HASSTAGGER: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(1u32);
pub const TAPF_ISRTLAWARE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(2u32);
pub const TAPF_ALLOWCOLLECTION: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(4u32);
pub const TAPF_HASBACKGROUND: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(8u32);
pub const TAPF_HASPERSPECTIVE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(16u32);
#[repr(C)]
pub struct TA_TIMINGFUNCTION(i32);
#[repr(transparent)]
pub struct TA_TIMINGFUNCTION_TYPE(pub i32);
pub const TTFT_UNDEFINED: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(0i32);
pub const TTFT_CUBIC_BEZIER: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(1i32);
#[repr(C)]
pub struct TA_TRANSFORM(i32);
#[repr(C)]
pub struct TA_TRANSFORM_2D(i32);
#[repr(C)]
pub struct TA_TRANSFORM_CLIP(i32);
#[repr(transparent)]
pub struct TA_TRANSFORM_FLAG(pub i32);
pub const TATF_NONE: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(0i32);
pub const TATF_TARGETVALUES_USER: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(1i32);
pub const TATF_HASINITIALVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(2i32);
pub const TATF_HASORIGINVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(4i32);
#[repr(C)]
pub struct TA_TRANSFORM_OPACITY(i32);
#[repr(transparent)]
pub struct TA_TRANSFORM_TYPE(pub i32);
pub const TATT_TRANSLATE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(0i32);
pub const TATT_SCALE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(1i32);
pub const TATT_OPACITY: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(2i32);
pub const TATT_CLIP: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TBADDBITMAP(i32);
pub const TBBF_LARGE: u32 = 1u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct TBBUTTON(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct TBBUTTON(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TBBUTTONINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TBBUTTONINFOW(i32);
#[repr(transparent)]
pub struct TBBUTTONINFOW_MASK(pub u32);
pub const TBIF_BYINDEX: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2147483648u32);
pub const TBIF_COMMAND: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(32u32);
pub const TBIF_IMAGE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(1u32);
pub const TBIF_LPARAM: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(16u32);
pub const TBIF_SIZE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(64u32);
pub const TBIF_STATE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(4u32);
pub const TBIF_STYLE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(8u32);
pub const TBIF_TEXT: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2u32);
pub const TBCDRF_BLENDICON: u32 = 2097152u32;
pub const TBCDRF_HILITEHOTTRACK: u32 = 131072u32;
pub const TBCDRF_NOBACKGROUND: u32 = 4194304u32;
pub const TBCDRF_NOEDGES: u32 = 65536u32;
pub const TBCDRF_NOETCHEDEFFECT: u32 = 1048576u32;
pub const TBCDRF_NOMARK: u32 = 524288u32;
pub const TBCDRF_NOOFFSET: u32 = 262144u32;
pub const TBCDRF_USECDCOLORS: u32 = 8388608u32;
pub const TBCD_CHANNEL: u32 = 3u32;
pub const TBCD_THUMB: u32 = 2u32;
pub const TBCD_TICS: u32 = 1u32;
pub const TBDDRET_DEFAULT: u32 = 0u32;
pub const TBDDRET_NODEFAULT: u32 = 1u32;
pub const TBDDRET_TREATPRESSED: u32 = 2u32;
#[repr(C)]
pub struct TBINSERTMARK(i32);
#[repr(transparent)]
pub struct TBINSERTMARK_FLAGS(pub u32);
pub const TBIMHT_NONE: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(0u32);
pub const TBIMHT_AFTER: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(1u32);
pub const TBIMHT_BACKGROUND: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(2u32);
#[repr(C)]
pub struct TBMETRICS(i32);
pub const TBMF_BARPAD: u32 = 2u32;
pub const TBMF_BUTTONSPACING: u32 = 4u32;
pub const TBMF_PAD: u32 = 1u32;
pub const TBM_CLEARSEL: u32 = 1043u32;
pub const TBM_CLEARTICS: u32 = 1033u32;
pub const TBM_GETBUDDY: u32 = 1057u32;
pub const TBM_GETCHANNELRECT: u32 = 1050u32;
pub const TBM_GETLINESIZE: u32 = 1048u32;
pub const TBM_GETNUMTICS: u32 = 1040u32;
pub const TBM_GETPAGESIZE: u32 = 1046u32;
pub const TBM_GETPTICS: u32 = 1038u32;
pub const TBM_GETRANGEMAX: u32 = 1026u32;
pub const TBM_GETRANGEMIN: u32 = 1025u32;
pub const TBM_GETSELEND: u32 = 1042u32;
pub const TBM_GETSELSTART: u32 = 1041u32;
pub const TBM_GETTHUMBLENGTH: u32 = 1052u32;
pub const TBM_GETTHUMBRECT: u32 = 1049u32;
pub const TBM_GETTIC: u32 = 1027u32;
pub const TBM_GETTICPOS: u32 = 1039u32;
pub const TBM_GETTOOLTIPS: u32 = 1054u32;
pub const TBM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TBM_SETBUDDY: u32 = 1056u32;
pub const TBM_SETLINESIZE: u32 = 1047u32;
pub const TBM_SETPAGESIZE: u32 = 1045u32;
pub const TBM_SETPOS: u32 = 1029u32;
pub const TBM_SETPOSNOTIFY: u32 = 1058u32;
pub const TBM_SETRANGE: u32 = 1030u32;
pub const TBM_SETRANGEMAX: u32 = 1032u32;
pub const TBM_SETRANGEMIN: u32 = 1031u32;
pub const TBM_SETSEL: u32 = 1034u32;
pub const TBM_SETSELEND: u32 = 1036u32;
pub const TBM_SETSELSTART: u32 = 1035u32;
pub const TBM_SETTHUMBLENGTH: u32 = 1051u32;
pub const TBM_SETTIC: u32 = 1028u32;
pub const TBM_SETTICFREQ: u32 = 1044u32;
pub const TBM_SETTIPSIDE: u32 = 1055u32;
pub const TBM_SETTOOLTIPS: u32 = 1053u32;
pub const TBM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TBNRF_ENDCUSTOMIZE: u32 = 2u32;
pub const TBNRF_HIDEHELP: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TBREPLACEBITMAP(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct TBSAVEPARAMSA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct TBSAVEPARAMSW(i32);
pub const TBSTATE_CHECKED: u32 = 1u32;
pub const TBSTATE_ELLIPSES: u32 = 64u32;
pub const TBSTATE_ENABLED: u32 = 4u32;
pub const TBSTATE_HIDDEN: u32 = 8u32;
pub const TBSTATE_INDETERMINATE: u32 = 16u32;
pub const TBSTATE_MARKED: u32 = 128u32;
pub const TBSTATE_PRESSED: u32 = 2u32;
pub const TBSTATE_WRAP: u32 = 32u32;
pub const TBSTYLE_ALTDRAG: u32 = 1024u32;
pub const TBSTYLE_AUTOSIZE: u32 = 16u32;
pub const TBSTYLE_BUTTON: u32 = 0u32;
pub const TBSTYLE_CHECK: u32 = 2u32;
pub const TBSTYLE_CUSTOMERASE: u32 = 8192u32;
pub const TBSTYLE_DROPDOWN: u32 = 8u32;
pub const TBSTYLE_EX_DOUBLEBUFFER: u32 = 128u32;
pub const TBSTYLE_EX_DRAWDDARROWS: u32 = 1u32;
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: u32 = 16u32;
pub const TBSTYLE_EX_MIXEDBUTTONS: u32 = 8u32;
pub const TBSTYLE_EX_MULTICOLUMN: u32 = 2u32;
pub const TBSTYLE_EX_VERTICAL: u32 = 4u32;
pub const TBSTYLE_FLAT: u32 = 2048u32;
pub const TBSTYLE_GROUP: u32 = 4u32;
pub const TBSTYLE_LIST: u32 = 4096u32;
pub const TBSTYLE_NOPREFIX: u32 = 32u32;
pub const TBSTYLE_REGISTERDROP: u32 = 16384u32;
pub const TBSTYLE_SEP: u32 = 1u32;
pub const TBSTYLE_TOOLTIPS: u32 = 256u32;
pub const TBSTYLE_TRANSPARENT: u32 = 32768u32;
pub const TBSTYLE_WRAPABLE: u32 = 512u32;
pub const TBS_AUTOTICKS: u32 = 1u32;
pub const TBS_BOTH: u32 = 8u32;
pub const TBS_BOTTOM: u32 = 0u32;
pub const TBS_DOWNISLEFT: u32 = 1024u32;
pub const TBS_ENABLESELRANGE: u32 = 32u32;
pub const TBS_FIXEDLENGTH: u32 = 64u32;
pub const TBS_HORZ: u32 = 0u32;
pub const TBS_LEFT: u32 = 4u32;
pub const TBS_NOTHUMB: u32 = 128u32;
pub const TBS_NOTICKS: u32 = 16u32;
pub const TBS_NOTIFYBEFOREMOVE: u32 = 2048u32;
pub const TBS_REVERSED: u32 = 512u32;
pub const TBS_RIGHT: u32 = 0u32;
pub const TBS_TOOLTIPS: u32 = 256u32;
pub const TBS_TOP: u32 = 4u32;
pub const TBS_TRANSPARENTBKGND: u32 = 4096u32;
pub const TBS_VERT: u32 = 2u32;
pub const TBTS_BOTTOM: u32 = 2u32;
pub const TBTS_LEFT: u32 = 1u32;
pub const TBTS_RIGHT: u32 = 3u32;
pub const TBTS_TOP: u32 = 0u32;
pub const TB_ADDBITMAP: u32 = 1043u32;
pub const TB_ADDBUTTONS: u32 = 1092u32;
pub const TB_ADDBUTTONSA: u32 = 1044u32;
pub const TB_ADDBUTTONSW: u32 = 1092u32;
pub const TB_ADDSTRING: u32 = 1101u32;
pub const TB_ADDSTRINGA: u32 = 1052u32;
pub const TB_ADDSTRINGW: u32 = 1101u32;
pub const TB_AUTOSIZE: u32 = 1057u32;
pub const TB_BOTTOM: u32 = 7u32;
pub const TB_BUTTONCOUNT: u32 = 1048u32;
pub const TB_BUTTONSTRUCTSIZE: u32 = 1054u32;
pub const TB_CHANGEBITMAP: u32 = 1067u32;
pub const TB_CHECKBUTTON: u32 = 1026u32;
pub const TB_COMMANDTOINDEX: u32 = 1049u32;
pub const TB_CUSTOMIZE: u32 = 1051u32;
pub const TB_DELETEBUTTON: u32 = 1046u32;
pub const TB_ENABLEBUTTON: u32 = 1025u32;
pub const TB_ENDTRACK: u32 = 8u32;
pub const TB_GETANCHORHIGHLIGHT: u32 = 1098u32;
pub const TB_GETBITMAP: u32 = 1068u32;
pub const TB_GETBITMAPFLAGS: u32 = 1065u32;
pub const TB_GETBUTTON: u32 = 1047u32;
pub const TB_GETBUTTONINFO: u32 = 1087u32;
pub const TB_GETBUTTONINFOA: u32 = 1089u32;
pub const TB_GETBUTTONINFOW: u32 = 1087u32;
pub const TB_GETBUTTONSIZE: u32 = 1082u32;
pub const TB_GETBUTTONTEXT: u32 = 1099u32;
pub const TB_GETBUTTONTEXTA: u32 = 1069u32;
pub const TB_GETBUTTONTEXTW: u32 = 1099u32;
pub const TB_GETCOLORSCHEME: u32 = 8195u32;
pub const TB_GETDISABLEDIMAGELIST: u32 = 1079u32;
pub const TB_GETEXTENDEDSTYLE: u32 = 1109u32;
pub const TB_GETHOTIMAGELIST: u32 = 1077u32;
pub const TB_GETHOTITEM: u32 = 1095u32;
pub const TB_GETIDEALSIZE: u32 = 1123u32;
pub const TB_GETIMAGELIST: u32 = 1073u32;
pub const TB_GETIMAGELISTCOUNT: u32 = 1122u32;
pub const TB_GETINSERTMARK: u32 = 1103u32;
pub const TB_GETINSERTMARKCOLOR: u32 = 1113u32;
pub const TB_GETITEMDROPDOWNRECT: u32 = 1127u32;
pub const TB_GETITEMRECT: u32 = 1053u32;
pub const TB_GETMAXSIZE: u32 = 1107u32;
pub const TB_GETMETRICS: u32 = 1125u32;
pub const TB_GETOBJECT: u32 = 1086u32;
pub const TB_GETPADDING: u32 = 1110u32;
pub const TB_GETPRESSEDIMAGELIST: u32 = 1129u32;
pub const TB_GETRECT: u32 = 1075u32;
pub const TB_GETROWS: u32 = 1064u32;
pub const TB_GETSTATE: u32 = 1042u32;
pub const TB_GETSTRING: u32 = 1115u32;
pub const TB_GETSTRINGA: u32 = 1116u32;
pub const TB_GETSTRINGW: u32 = 1115u32;
pub const TB_GETSTYLE: u32 = 1081u32;
pub const TB_GETTEXTROWS: u32 = 1085u32;
pub const TB_GETTOOLTIPS: u32 = 1059u32;
pub const TB_GETUNICODEFORMAT: u32 = 8198u32;
pub const TB_HASACCELERATOR: u32 = 1119u32;
pub const TB_HIDEBUTTON: u32 = 1028u32;
pub const TB_HITTEST: u32 = 1093u32;
pub const TB_INDETERMINATE: u32 = 1029u32;
pub const TB_INSERTBUTTON: u32 = 1091u32;
pub const TB_INSERTBUTTONA: u32 = 1045u32;
pub const TB_INSERTBUTTONW: u32 = 1091u32;
pub const TB_INSERTMARKHITTEST: u32 = 1105u32;
pub const TB_ISBUTTONCHECKED: u32 = 1034u32;
pub const TB_ISBUTTONENABLED: u32 = 1033u32;
pub const TB_ISBUTTONHIDDEN: u32 = 1036u32;
pub const TB_ISBUTTONHIGHLIGHTED: u32 = 1038u32;
pub const TB_ISBUTTONINDETERMINATE: u32 = 1037u32;
pub const TB_ISBUTTONPRESSED: u32 = 1035u32;
pub const TB_LINEDOWN: u32 = 1u32;
pub const TB_LINEUP: u32 = 0u32;
pub const TB_LOADIMAGES: u32 = 1074u32;
pub const TB_MAPACCELERATOR: u32 = 1114u32;
pub const TB_MAPACCELERATORA: u32 = 1102u32;
pub const TB_MAPACCELERATORW: u32 = 1114u32;
pub const TB_MARKBUTTON: u32 = 1030u32;
pub const TB_MOVEBUTTON: u32 = 1106u32;
pub const TB_PAGEDOWN: u32 = 3u32;
pub const TB_PAGEUP: u32 = 2u32;
pub const TB_PRESSBUTTON: u32 = 1027u32;
pub const TB_REPLACEBITMAP: u32 = 1070u32;
pub const TB_SAVERESTORE: u32 = 1100u32;
pub const TB_SAVERESTOREA: u32 = 1050u32;
pub const TB_SAVERESTOREW: u32 = 1100u32;
pub const TB_SETANCHORHIGHLIGHT: u32 = 1097u32;
pub const TB_SETBITMAPSIZE: u32 = 1056u32;
pub const TB_SETBOUNDINGSIZE: u32 = 1117u32;
pub const TB_SETBUTTONINFO: u32 = 1088u32;
pub const TB_SETBUTTONINFOA: u32 = 1090u32;
pub const TB_SETBUTTONINFOW: u32 = 1088u32;
pub const TB_SETBUTTONSIZE: u32 = 1055u32;
pub const TB_SETBUTTONWIDTH: u32 = 1083u32;
pub const TB_SETCMDID: u32 = 1066u32;
pub const TB_SETCOLORSCHEME: u32 = 8194u32;
pub const TB_SETDISABLEDIMAGELIST: u32 = 1078u32;
pub const TB_SETDRAWTEXTFLAGS: u32 = 1094u32;
pub const TB_SETEXTENDEDSTYLE: u32 = 1108u32;
pub const TB_SETHOTIMAGELIST: u32 = 1076u32;
pub const TB_SETHOTITEM: u32 = 1096u32;
pub const TB_SETHOTITEM2: u32 = 1118u32;
pub const TB_SETIMAGELIST: u32 = 1072u32;
pub const TB_SETINDENT: u32 = 1071u32;
pub const TB_SETINSERTMARK: u32 = 1104u32;
pub const TB_SETINSERTMARKCOLOR: u32 = 1112u32;
pub const TB_SETLISTGAP: u32 = 1120u32;
pub const TB_SETMAXTEXTROWS: u32 = 1084u32;
pub const TB_SETMETRICS: u32 = 1126u32;
pub const TB_SETPADDING: u32 = 1111u32;
pub const TB_SETPARENT: u32 = 1061u32;
pub const TB_SETPRESSEDIMAGELIST: u32 = 1128u32;
pub const TB_SETROWS: u32 = 1063u32;
pub const TB_SETSTATE: u32 = 1041u32;
pub const TB_SETSTYLE: u32 = 1080u32;
pub const TB_SETTOOLTIPS: u32 = 1060u32;
pub const TB_SETUNICODEFORMAT: u32 = 8197u32;
pub const TB_SETWINDOWTHEME: u32 = 8203u32;
pub const TB_THUMBPOSITION: u32 = 4u32;
pub const TB_THUMBTRACK: u32 = 5u32;
pub const TB_TOP: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCHITTESTINFO(i32);
#[repr(transparent)]
pub struct TCHITTESTINFO_FLAGS(pub u32);
pub const TCHT_NOWHERE: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(1u32);
pub const TCHT_ONITEM: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(6u32);
pub const TCHT_ONITEMICON: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(2u32);
pub const TCHT_ONITEMLABEL: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(4u32);
pub const TCIS_BUTTONPRESSED: u32 = 1u32;
pub const TCIS_HIGHLIGHTED: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCITEMHEADERA(i32);
#[repr(transparent)]
pub struct TCITEMHEADERA_MASK(pub u32);
pub const TCIF_IMAGE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(2u32);
pub const TCIF_RTLREADING: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(4u32);
pub const TCIF_TEXT: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(1u32);
pub const TCIF_PARAM: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(8u32);
pub const TCIF_STATE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(16u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCITEMHEADERW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCITEMW(i32);
pub const TCM_ADJUSTRECT: u32 = 4904u32;
pub const TCM_DELETEALLITEMS: u32 = 4873u32;
pub const TCM_DELETEITEM: u32 = 4872u32;
pub const TCM_DESELECTALL: u32 = 4914u32;
pub const TCM_FIRST: u32 = 4864u32;
pub const TCM_GETCURFOCUS: u32 = 4911u32;
pub const TCM_GETCURSEL: u32 = 4875u32;
pub const TCM_GETEXTENDEDSTYLE: u32 = 4917u32;
pub const TCM_GETIMAGELIST: u32 = 4866u32;
pub const TCM_GETITEM: u32 = 4924u32;
pub const TCM_GETITEMA: u32 = 4869u32;
pub const TCM_GETITEMCOUNT: u32 = 4868u32;
pub const TCM_GETITEMRECT: u32 = 4874u32;
pub const TCM_GETITEMW: u32 = 4924u32;
pub const TCM_GETROWCOUNT: u32 = 4908u32;
pub const TCM_GETTOOLTIPS: u32 = 4909u32;
pub const TCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TCM_HIGHLIGHTITEM: u32 = 4915u32;
pub const TCM_HITTEST: u32 = 4877u32;
pub const TCM_INSERTITEM: u32 = 4926u32;
pub const TCM_INSERTITEMA: u32 = 4871u32;
pub const TCM_INSERTITEMW: u32 = 4926u32;
pub const TCM_REMOVEIMAGE: u32 = 4906u32;
pub const TCM_SETCURFOCUS: u32 = 4912u32;
pub const TCM_SETCURSEL: u32 = 4876u32;
pub const TCM_SETEXTENDEDSTYLE: u32 = 4916u32;
pub const TCM_SETIMAGELIST: u32 = 4867u32;
pub const TCM_SETITEM: u32 = 4925u32;
pub const TCM_SETITEMA: u32 = 4870u32;
pub const TCM_SETITEMEXTRA: u32 = 4878u32;
pub const TCM_SETITEMSIZE: u32 = 4905u32;
pub const TCM_SETITEMW: u32 = 4925u32;
pub const TCM_SETMINTABWIDTH: u32 = 4913u32;
pub const TCM_SETPADDING: u32 = 4907u32;
pub const TCM_SETTOOLTIPS: u32 = 4910u32;
pub const TCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TCS_BOTTOM: u32 = 2u32;
pub const TCS_BUTTONS: u32 = 256u32;
pub const TCS_EX_FLATSEPARATORS: u32 = 1u32;
pub const TCS_EX_REGISTERDROP: u32 = 2u32;
pub const TCS_FIXEDWIDTH: u32 = 1024u32;
pub const TCS_FLATBUTTONS: u32 = 8u32;
pub const TCS_FOCUSNEVER: u32 = 32768u32;
pub const TCS_FOCUSONBUTTONDOWN: u32 = 4096u32;
pub const TCS_FORCEICONLEFT: u32 = 16u32;
pub const TCS_FORCELABELLEFT: u32 = 32u32;
pub const TCS_HOTTRACK: u32 = 64u32;
pub const TCS_MULTILINE: u32 = 512u32;
pub const TCS_MULTISELECT: u32 = 4u32;
pub const TCS_OWNERDRAWFIXED: u32 = 8192u32;
pub const TCS_RAGGEDRIGHT: u32 = 2048u32;
pub const TCS_RIGHT: u32 = 2u32;
pub const TCS_RIGHTJUSTIFY: u32 = 0u32;
pub const TCS_SCROLLOPPOSITE: u32 = 1u32;
pub const TCS_SINGLELINE: u32 = 0u32;
pub const TCS_TABS: u32 = 0u32;
pub const TCS_TOOLTIPS: u32 = 16384u32;
pub const TCS_VERTICAL: u32 = 128u32;
#[repr(transparent)]
pub struct TEXTSHADOWTYPE(pub i32);
pub const TST_NONE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(0i32);
pub const TST_SINGLE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(1i32);
pub const TST_CONTINUOUS: TEXTSHADOWTYPE = TEXTSHADOWTYPE(2i32);
#[repr(transparent)]
pub struct THEMESIZE(pub i32);
pub const TS_MIN: THEMESIZE = THEMESIZE(0i32);
pub const TS_TRUE: THEMESIZE = THEMESIZE(1i32);
pub const TS_DRAW: THEMESIZE = THEMESIZE(2i32);
#[repr(transparent)]
pub struct THEME_PROPERTY_SYMBOL_ID(pub u32);
pub const TMT_RESERVEDLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(0u32);
pub const TMT_RESERVEDHIGH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7999u32);
pub const TMT_DIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2u32);
pub const TMT_GLYPHDIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8u32);
pub const TMT_ENUM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(200u32);
pub const TMT_STRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(201u32);
pub const TMT_INT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(202u32);
pub const TMT_BOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(203u32);
pub const TMT_COLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(204u32);
pub const TMT_MARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(205u32);
pub const TMT_FILENAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(206u32);
pub const TMT_SIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(207u32);
pub const TMT_POSITION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(208u32);
pub const TMT_RECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(209u32);
pub const TMT_FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(210u32);
pub const TMT_INTLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(211u32);
pub const TMT_HBITMAP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(212u32);
pub const TMT_DISKSTREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(213u32);
pub const TMT_STREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(214u32);
pub const TMT_BITMAPREF: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(215u32);
pub const TMT_FLOAT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(216u32);
pub const TMT_FLOATLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(217u32);
pub const TMT_COLORSCHEMES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(401u32);
pub const TMT_SIZES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(402u32);
pub const TMT_CHARSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(403u32);
pub const TMT_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(600u32);
pub const TMT_DISPLAYNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_TOOLTIP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(602u32);
pub const TMT_COMPANY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(603u32);
pub const TMT_AUTHOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(604u32);
pub const TMT_COPYRIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(605u32);
pub const TMT_URL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(606u32);
pub const TMT_VERSION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(607u32);
pub const TMT_DESCRIPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_FIRST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_LAST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_CAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_SMALLCAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(802u32);
pub const TMT_MENUFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(803u32);
pub const TMT_STATUSFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(804u32);
pub const TMT_MSGBOXFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(805u32);
pub const TMT_ICONTITLEFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(806u32);
pub const TMT_HEADING1FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(807u32);
pub const TMT_HEADING2FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(808u32);
pub const TMT_BODYFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_FIRSTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_LASTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_FLATMENUS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_FIRSTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_LASTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_SIZINGBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_SCROLLBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1202u32);
pub const TMT_SCROLLBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1203u32);
pub const TMT_CAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1204u32);
pub const TMT_CAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1205u32);
pub const TMT_SMCAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1206u32);
pub const TMT_SMCAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1207u32);
pub const TMT_MENUBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1208u32);
pub const TMT_MENUBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1209u32);
pub const TMT_PADDEDBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_FIRSTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_LASTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_MINCOLORDEPTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_FIRSTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_LASTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_CSSNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_XMLNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1402u32);
pub const TMT_LASTUPDATED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1403u32);
pub const TMT_ALIAS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_FIRSTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_LASTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_SCROLLBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_BACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1602u32);
pub const TMT_ACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1603u32);
pub const TMT_INACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1604u32);
pub const TMT_MENU: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1605u32);
pub const TMT_WINDOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1606u32);
pub const TMT_WINDOWFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1607u32);
pub const TMT_MENUTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1608u32);
pub const TMT_WINDOWTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1609u32);
pub const TMT_CAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1610u32);
pub const TMT_ACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1611u32);
pub const TMT_INACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1612u32);
pub const TMT_APPWORKSPACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1613u32);
pub const TMT_HIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1614u32);
pub const TMT_HIGHLIGHTTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1615u32);
pub const TMT_BTNFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1616u32);
pub const TMT_BTNSHADOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1617u32);
pub const TMT_GRAYTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1618u32);
pub const TMT_BTNTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1619u32);
pub const TMT_INACTIVECAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1620u32);
pub const TMT_BTNHIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1621u32);
pub const TMT_DKSHADOW3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1622u32);
pub const TMT_LIGHT3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1623u32);
pub const TMT_INFOTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1624u32);
pub const TMT_INFOBK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1625u32);
pub const TMT_BUTTONALTERNATEFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1626u32);
pub const TMT_HOTTRACKING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1627u32);
pub const TMT_GRADIENTACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1628u32);
pub const TMT_GRADIENTINACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1629u32);
pub const TMT_MENUHILIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1630u32);
pub const TMT_MENUBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_FIRSTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_LASTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_FROMHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1801u32);
pub const TMT_FROMHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1802u32);
pub const TMT_FROMHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1803u32);
pub const TMT_FROMHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1804u32);
pub const TMT_FROMHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1805u32);
pub const TMT_TOHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1806u32);
pub const TMT_TOHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1807u32);
pub const TMT_TOHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1808u32);
pub const TMT_TOHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1809u32);
pub const TMT_TOHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1810u32);
pub const TMT_FROMCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2001u32);
pub const TMT_FROMCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2002u32);
pub const TMT_FROMCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2003u32);
pub const TMT_FROMCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2004u32);
pub const TMT_FROMCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2005u32);
pub const TMT_TOCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2006u32);
pub const TMT_TOCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2007u32);
pub const TMT_TOCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2008u32);
pub const TMT_TOCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2009u32);
pub const TMT_TOCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2010u32);
pub const TMT_TRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2201u32);
pub const TMT_AUTOSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2202u32);
pub const TMT_BORDERONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2203u32);
pub const TMT_COMPOSITED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2204u32);
pub const TMT_BGFILL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2205u32);
pub const TMT_GLYPHTRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2206u32);
pub const TMT_GLYPHONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2207u32);
pub const TMT_ALWAYSSHOWSIZINGBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2208u32);
pub const TMT_MIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2209u32);
pub const TMT_UNIFORMSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2210u32);
pub const TMT_INTEGRALSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2211u32);
pub const TMT_SOURCEGROW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2212u32);
pub const TMT_SOURCESHRINK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2213u32);
pub const TMT_DRAWBORDERS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2214u32);
pub const TMT_NOETCHEDEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2215u32);
pub const TMT_TEXTAPPLYOVERLAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2216u32);
pub const TMT_TEXTGLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2217u32);
pub const TMT_TEXTITALIC: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2218u32);
pub const TMT_COMPOSITEDOPAQUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2219u32);
pub const TMT_LOCALIZEDMIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2220u32);
pub const TMT_IMAGECOUNT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2401u32);
pub const TMT_ALPHALEVEL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2402u32);
pub const TMT_BORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2403u32);
pub const TMT_ROUNDCORNERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2404u32);
pub const TMT_ROUNDCORNERHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2405u32);
pub const TMT_GRADIENTRATIO1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2406u32);
pub const TMT_GRADIENTRATIO2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2407u32);
pub const TMT_GRADIENTRATIO3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2408u32);
pub const TMT_GRADIENTRATIO4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2409u32);
pub const TMT_GRADIENTRATIO5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2410u32);
pub const TMT_PROGRESSCHUNKSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2411u32);
pub const TMT_PROGRESSSPACESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2412u32);
pub const TMT_SATURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2413u32);
pub const TMT_TEXTBORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2414u32);
pub const TMT_ALPHATHRESHOLD: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2415u32);
pub const TMT_WIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2416u32);
pub const TMT_HEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2417u32);
pub const TMT_GLYPHINDEX: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2418u32);
pub const TMT_TRUESIZESTRETCHMARK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2419u32);
pub const TMT_MINDPI1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2420u32);
pub const TMT_MINDPI2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2421u32);
pub const TMT_MINDPI3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2422u32);
pub const TMT_MINDPI4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2423u32);
pub const TMT_MINDPI5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2424u32);
pub const TMT_TEXTGLOWSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2425u32);
pub const TMT_FRAMESPERSECOND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2426u32);
pub const TMT_PIXELSPERFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2427u32);
pub const TMT_ANIMATIONDELAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2428u32);
pub const TMT_GLOWINTENSITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2429u32);
pub const TMT_OPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2430u32);
pub const TMT_COLORIZATIONCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2431u32);
pub const TMT_COLORIZATIONOPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2432u32);
pub const TMT_MINDPI6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2433u32);
pub const TMT_MINDPI7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2434u32);
pub const TMT_GLYPHFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2601u32);
pub const TMT_IMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3001u32);
pub const TMT_IMAGEFILE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3002u32);
pub const TMT_IMAGEFILE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3003u32);
pub const TMT_IMAGEFILE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3004u32);
pub const TMT_IMAGEFILE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3005u32);
pub const TMT_IMAGEFILE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3006u32);
pub const TMT_GLYPHIMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3008u32);
pub const TMT_IMAGEFILE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3009u32);
pub const TMT_IMAGEFILE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3010u32);
pub const TMT_TEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3201u32);
pub const TMT_CLASSICVALUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3202u32);
pub const TMT_OFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3401u32);
pub const TMT_TEXTSHADOWOFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3402u32);
pub const TMT_MINSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3403u32);
pub const TMT_MINSIZE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3404u32);
pub const TMT_MINSIZE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3405u32);
pub const TMT_MINSIZE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3406u32);
pub const TMT_MINSIZE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3407u32);
pub const TMT_MINSIZE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3408u32);
pub const TMT_NORMALSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3409u32);
pub const TMT_MINSIZE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3410u32);
pub const TMT_MINSIZE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3411u32);
pub const TMT_SIZINGMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3601u32);
pub const TMT_CONTENTMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3602u32);
pub const TMT_CAPTIONMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3603u32);
pub const TMT_BORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3801u32);
pub const TMT_FILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3802u32);
pub const TMT_TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3803u32);
pub const TMT_EDGELIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3804u32);
pub const TMT_EDGEHIGHLIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3805u32);
pub const TMT_EDGESHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3806u32);
pub const TMT_EDGEDKSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3807u32);
pub const TMT_EDGEFILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3808u32);
pub const TMT_TRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3809u32);
pub const TMT_GRADIENTCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3810u32);
pub const TMT_GRADIENTCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3811u32);
pub const TMT_GRADIENTCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3812u32);
pub const TMT_GRADIENTCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3813u32);
pub const TMT_GRADIENTCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3814u32);
pub const TMT_SHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3815u32);
pub const TMT_GLOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3816u32);
pub const TMT_TEXTBORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3817u32);
pub const TMT_TEXTSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3818u32);
pub const TMT_GLYPHTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3819u32);
pub const TMT_GLYPHTRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3820u32);
pub const TMT_FILLCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3821u32);
pub const TMT_BORDERCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3822u32);
pub const TMT_ACCENTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3823u32);
pub const TMT_TEXTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3824u32);
pub const TMT_HEADING1TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3825u32);
pub const TMT_HEADING2TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3826u32);
pub const TMT_BODYTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3827u32);
pub const TMT_BGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4001u32);
pub const TMT_BORDERTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4002u32);
pub const TMT_FILLTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4003u32);
pub const TMT_SIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4004u32);
pub const TMT_HALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4005u32);
pub const TMT_CONTENTALIGNMENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4006u32);
pub const TMT_VALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4007u32);
pub const TMT_OFFSETTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4008u32);
pub const TMT_ICONEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4009u32);
pub const TMT_TEXTSHADOWTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4010u32);
pub const TMT_IMAGELAYOUT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4011u32);
pub const TMT_GLYPHTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4012u32);
pub const TMT_IMAGESELECTTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4013u32);
pub const TMT_GLYPHFONTSIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4014u32);
pub const TMT_TRUESIZESCALINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4015u32);
pub const TMT_USERPICTURE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5001u32);
pub const TMT_DEFAULTPANESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5002u32);
pub const TMT_BLENDCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5003u32);
pub const TMT_CUSTOMSPLITRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5004u32);
pub const TMT_ANIMATIONBUTTONRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5005u32);
pub const TMT_ANIMATIONDURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5006u32);
pub const TMT_TRANSITIONDURATIONS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(6000u32);
pub const TMT_SCALEDBACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7001u32);
pub const TMT_ATLASIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8000u32);
pub const TMT_ATLASINPUTIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8001u32);
pub const TMT_ATLASRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8002u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TOUCH_HIT_TESTING_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION(i32);
#[repr(transparent)]
pub struct TRAILINGGRIDCELLSTATES(pub i32);
pub const MCTGC_HOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(1i32);
pub const MCTGC_HASSTATE: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(2i32);
pub const MCTGC_HASSTATEHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(3i32);
pub const MCTGC_TODAY: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(4i32);
pub const MCTGC_TODAYSELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(5i32);
pub const MCTGC_SELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(6i32);
pub const MCTGC_SELECTEDHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(7i32);
#[repr(transparent)]
pub struct TRAILINGGRIDCELLUPPERSTATES(pub i32);
pub const MCTGCU_HOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(1i32);
pub const MCTGCU_HASSTATE: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(2i32);
pub const MCTGCU_HASSTATEHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(3i32);
pub const MCTGCU_SELECTED: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(4i32);
pub const MCTGCU_SELECTEDHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(5i32);
#[repr(transparent)]
pub struct TRAYNOTIFYPARTS(pub i32);
pub const TNP_BACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(1i32);
pub const TNP_ANIMBACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(2i32);
#[repr(transparent)]
pub struct TRUESIZESCALINGTYPE(pub i32);
pub const TSST_NONE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(0i32);
pub const TSST_SIZE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(1i32);
pub const TSST_DPI: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(2i32);
pub const TTDT_AUTOMATIC: u32 = 0u32;
pub const TTDT_AUTOPOP: u32 = 2u32;
pub const TTDT_INITIAL: u32 = 3u32;
pub const TTDT_RESHOW: u32 = 1u32;
pub const TTF_DI_SETITEM: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TTGETTITLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TTHITTESTINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TTHITTESTINFOW(i32);
pub const TTM_ACTIVATE: u32 = 1025u32;
pub const TTM_ADDTOOL: u32 = 1074u32;
pub const TTM_ADDTOOLA: u32 = 1028u32;
pub const TTM_ADDTOOLW: u32 = 1074u32;
pub const TTM_ADJUSTRECT: u32 = 1055u32;
pub const TTM_DELTOOL: u32 = 1075u32;
pub const TTM_DELTOOLA: u32 = 1029u32;
pub const TTM_DELTOOLW: u32 = 1075u32;
pub const TTM_ENUMTOOLS: u32 = 1082u32;
pub const TTM_ENUMTOOLSA: u32 = 1038u32;
pub const TTM_ENUMTOOLSW: u32 = 1082u32;
pub const TTM_GETBUBBLESIZE: u32 = 1054u32;
pub const TTM_GETCURRENTTOOL: u32 = 1083u32;
pub const TTM_GETCURRENTTOOLA: u32 = 1039u32;
pub const TTM_GETCURRENTTOOLW: u32 = 1083u32;
pub const TTM_GETDELAYTIME: u32 = 1045u32;
pub const TTM_GETMARGIN: u32 = 1051u32;
pub const TTM_GETMAXTIPWIDTH: u32 = 1049u32;
pub const TTM_GETTEXT: u32 = 1080u32;
pub const TTM_GETTEXTA: u32 = 1035u32;
pub const TTM_GETTEXTW: u32 = 1080u32;
pub const TTM_GETTIPBKCOLOR: u32 = 1046u32;
pub const TTM_GETTIPTEXTCOLOR: u32 = 1047u32;
pub const TTM_GETTITLE: u32 = 1059u32;
pub const TTM_GETTOOLCOUNT: u32 = 1037u32;
pub const TTM_GETTOOLINFO: u32 = 1077u32;
pub const TTM_GETTOOLINFOA: u32 = 1032u32;
pub const TTM_GETTOOLINFOW: u32 = 1077u32;
pub const TTM_HITTEST: u32 = 1079u32;
pub const TTM_HITTESTA: u32 = 1034u32;
pub const TTM_HITTESTW: u32 = 1079u32;
pub const TTM_NEWTOOLRECT: u32 = 1076u32;
pub const TTM_NEWTOOLRECTA: u32 = 1030u32;
pub const TTM_NEWTOOLRECTW: u32 = 1076u32;
pub const TTM_POP: u32 = 1052u32;
pub const TTM_POPUP: u32 = 1058u32;
pub const TTM_RELAYEVENT: u32 = 1031u32;
pub const TTM_SETDELAYTIME: u32 = 1027u32;
pub const TTM_SETMARGIN: u32 = 1050u32;
pub const TTM_SETMAXTIPWIDTH: u32 = 1048u32;
pub const TTM_SETTIPBKCOLOR: u32 = 1043u32;
pub const TTM_SETTIPTEXTCOLOR: u32 = 1044u32;
pub const TTM_SETTITLE: u32 = 1057u32;
pub const TTM_SETTITLEA: u32 = 1056u32;
pub const TTM_SETTITLEW: u32 = 1057u32;
pub const TTM_SETTOOLINFO: u32 = 1078u32;
pub const TTM_SETTOOLINFOA: u32 = 1033u32;
pub const TTM_SETTOOLINFOW: u32 = 1078u32;
pub const TTM_SETWINDOWTHEME: u32 = 8203u32;
pub const TTM_TRACKACTIVATE: u32 = 1041u32;
pub const TTM_TRACKPOSITION: u32 = 1042u32;
pub const TTM_UPDATE: u32 = 1053u32;
pub const TTM_UPDATETIPTEXT: u32 = 1081u32;
pub const TTM_UPDATETIPTEXTA: u32 = 1036u32;
pub const TTM_UPDATETIPTEXTW: u32 = 1081u32;
pub const TTM_WINDOWFROMPOINT: u32 = 1040u32;
pub const TTS_ALWAYSTIP: u32 = 1u32;
pub const TTS_BALLOON: u32 = 64u32;
pub const TTS_CLOSE: u32 = 128u32;
pub const TTS_NOANIMATE: u32 = 16u32;
pub const TTS_NOFADE: u32 = 32u32;
pub const TTS_NOPREFIX: u32 = 2u32;
pub const TTS_USEVISUALSTYLE: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TTTOOLINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TTTOOLINFOW(i32);
#[repr(transparent)]
pub struct TTTOOLINFO_FLAGS(pub u32);
pub const TTF_ABSOLUTE: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(128u32);
pub const TTF_CENTERTIP: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(2u32);
pub const TTF_IDISHWND: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(1u32);
pub const TTF_PARSELINKS: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(4096u32);
pub const TTF_RTLREADING: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(4u32);
pub const TTF_SUBCLASS: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(16u32);
pub const TTF_TRACK: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(32u32);
pub const TTF_TRANSPARENT: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(256u32);
pub const TVCDRF_NOIMAGES: u32 = 65536u32;
pub const TVC_BYKEYBOARD: u32 = 2u32;
pub const TVC_BYMOUSE: u32 = 1u32;
pub const TVC_UNKNOWN: u32 = 0u32;
pub const TVE_COLLAPSE: u32 = 1u32;
pub const TVE_COLLAPSERESET: u32 = 32768u32;
pub const TVE_EXPAND: u32 = 2u32;
pub const TVE_EXPANDPARTIAL: u32 = 16384u32;
pub const TVE_TOGGLE: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVGETITEMPARTRECTINFO(i32);
pub const TVGN_CARET: u32 = 9u32;
pub const TVGN_CHILD: u32 = 4u32;
pub const TVGN_DROPHILITE: u32 = 8u32;
pub const TVGN_FIRSTVISIBLE: u32 = 5u32;
pub const TVGN_LASTVISIBLE: u32 = 10u32;
pub const TVGN_NEXT: u32 = 1u32;
pub const TVGN_NEXTSELECTED: u32 = 11u32;
pub const TVGN_NEXTVISIBLE: u32 = 6u32;
pub const TVGN_PARENT: u32 = 3u32;
pub const TVGN_PREVIOUS: u32 = 2u32;
pub const TVGN_PREVIOUSVISIBLE: u32 = 7u32;
pub const TVGN_ROOT: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVHITTESTINFO(i32);
#[repr(transparent)]
pub struct TVHITTESTINFO_FLAGS(pub u32);
pub const TVHT_ABOVE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(256u32);
pub const TVHT_BELOW: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(512u32);
pub const TVHT_NOWHERE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1u32);
pub const TVHT_ONITEM: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(70u32);
pub const TVHT_ONITEMBUTTON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(16u32);
pub const TVHT_ONITEMICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2u32);
pub const TVHT_ONITEMINDENT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(8u32);
pub const TVHT_ONITEMLABEL: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(4u32);
pub const TVHT_ONITEMRIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(32u32);
pub const TVHT_ONITEMSTATEICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(64u32);
pub const TVHT_TOLEFT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2048u32);
pub const TVHT_TORIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1024u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVINSERTSTRUCTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVINSERTSTRUCTW(i32);
pub const TVIS_BOLD: u32 = 16u32;
pub const TVIS_CUT: u32 = 4u32;
pub const TVIS_DROPHILITED: u32 = 8u32;
pub const TVIS_EXPANDED: u32 = 32u32;
pub const TVIS_EXPANDEDONCE: u32 = 64u32;
pub const TVIS_EXPANDPARTIAL: u32 = 128u32;
pub const TVIS_EX_ALL: u32 = 2u32;
pub const TVIS_EX_DISABLED: u32 = 2u32;
pub const TVIS_EX_FLAT: u32 = 1u32;
pub const TVIS_OVERLAYMASK: u32 = 3840u32;
pub const TVIS_SELECTED: u32 = 2u32;
pub const TVIS_STATEIMAGEMASK: u32 = 61440u32;
pub const TVIS_USERMASK: u32 = 61440u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVITEMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVITEMEXA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVITEMEXW(i32);
#[repr(transparent)]
pub struct TVITEMEXW_CHILDREN(pub i32);
pub const I_ZERO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(0i32);
pub const I_ONE_OR_MORE: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(1i32);
pub const I_CHILDRENCALLBACK: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-1i32);
pub const I_CHILDRENAUTO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-2i32);
#[repr(transparent)]
pub struct TVITEMPART(pub i32);
pub const TVGIPR_BUTTON: TVITEMPART = TVITEMPART(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVITEMW(i32);
#[repr(transparent)]
pub struct TVITEM_MASK(pub u32);
pub const TVIF_CHILDREN: TVITEM_MASK = TVITEM_MASK(64u32);
pub const TVIF_DI_SETITEM: TVITEM_MASK = TVITEM_MASK(4096u32);
pub const TVIF_HANDLE: TVITEM_MASK = TVITEM_MASK(16u32);
pub const TVIF_IMAGE: TVITEM_MASK = TVITEM_MASK(2u32);
pub const TVIF_PARAM: TVITEM_MASK = TVITEM_MASK(4u32);
pub const TVIF_SELECTEDIMAGE: TVITEM_MASK = TVITEM_MASK(32u32);
pub const TVIF_STATE: TVITEM_MASK = TVITEM_MASK(8u32);
pub const TVIF_TEXT: TVITEM_MASK = TVITEM_MASK(1u32);
pub const TVIF_EXPANDEDIMAGE: TVITEM_MASK = TVITEM_MASK(512u32);
pub const TVIF_INTEGRAL: TVITEM_MASK = TVITEM_MASK(128u32);
pub const TVIF_STATEEX: TVITEM_MASK = TVITEM_MASK(256u32);
pub const TVI_FIRST: HTREEITEM = HTREEITEM(-65535i32 as _);
pub const TVI_LAST: HTREEITEM = HTREEITEM(-65534i32 as _);
pub const TVI_ROOT: HTREEITEM = HTREEITEM(-65536i32 as _);
pub const TVI_SORT: HTREEITEM = HTREEITEM(-65533i32 as _);
pub const TVM_CREATEDRAGIMAGE: u32 = 4370u32;
pub const TVM_DELETEITEM: u32 = 4353u32;
pub const TVM_EDITLABEL: u32 = 4417u32;
pub const TVM_EDITLABELA: u32 = 4366u32;
pub const TVM_EDITLABELW: u32 = 4417u32;
pub const TVM_ENDEDITLABELNOW: u32 = 4374u32;
pub const TVM_ENSUREVISIBLE: u32 = 4372u32;
pub const TVM_EXPAND: u32 = 4354u32;
pub const TVM_GETBKCOLOR: u32 = 4383u32;
pub const TVM_GETCOUNT: u32 = 4357u32;
pub const TVM_GETEDITCONTROL: u32 = 4367u32;
pub const TVM_GETEXTENDEDSTYLE: u32 = 4397u32;
pub const TVM_GETIMAGELIST: u32 = 4360u32;
pub const TVM_GETINDENT: u32 = 4358u32;
pub const TVM_GETINSERTMARKCOLOR: u32 = 4390u32;
pub const TVM_GETISEARCHSTRING: u32 = 4416u32;
pub const TVM_GETISEARCHSTRINGA: u32 = 4375u32;
pub const TVM_GETISEARCHSTRINGW: u32 = 4416u32;
pub const TVM_GETITEM: u32 = 4414u32;
pub const TVM_GETITEMA: u32 = 4364u32;
pub const TVM_GETITEMHEIGHT: u32 = 4380u32;
pub const TVM_GETITEMPARTRECT: u32 = 4424u32;
pub const TVM_GETITEMRECT: u32 = 4356u32;
pub const TVM_GETITEMSTATE: u32 = 4391u32;
pub const TVM_GETITEMW: u32 = 4414u32;
pub const TVM_GETLINECOLOR: u32 = 4393u32;
pub const TVM_GETNEXTITEM: u32 = 4362u32;
pub const TVM_GETSCROLLTIME: u32 = 4386u32;
pub const TVM_GETSELECTEDCOUNT: u32 = 4422u32;
pub const TVM_GETTEXTCOLOR: u32 = 4384u32;
pub const TVM_GETTOOLTIPS: u32 = 4377u32;
pub const TVM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TVM_GETVISIBLECOUNT: u32 = 4368u32;
pub const TVM_HITTEST: u32 = 4369u32;
pub const TVM_INSERTITEM: u32 = 4402u32;
pub const TVM_INSERTITEMA: u32 = 4352u32;
pub const TVM_INSERTITEMW: u32 = 4402u32;
pub const TVM_MAPACCIDTOHTREEITEM: u32 = 4394u32;
pub const TVM_MAPHTREEITEMTOACCID: u32 = 4395u32;
pub const TVM_SELECTITEM: u32 = 4363u32;
pub const TVM_SETAUTOSCROLLINFO: u32 = 4411u32;
pub const TVM_SETBKCOLOR: u32 = 4381u32;
pub const TVM_SETBORDER: u32 = 4387u32;
pub const TVM_SETEXTENDEDSTYLE: u32 = 4396u32;
pub const TVM_SETHOT: u32 = 4410u32;
pub const TVM_SETIMAGELIST: u32 = 4361u32;
pub const TVM_SETINDENT: u32 = 4359u32;
pub const TVM_SETINSERTMARK: u32 = 4378u32;
pub const TVM_SETINSERTMARKCOLOR: u32 = 4389u32;
pub const TVM_SETITEM: u32 = 4415u32;
pub const TVM_SETITEMA: u32 = 4365u32;
pub const TVM_SETITEMHEIGHT: u32 = 4379u32;
pub const TVM_SETITEMW: u32 = 4415u32;
pub const TVM_SETLINECOLOR: u32 = 4392u32;
pub const TVM_SETSCROLLTIME: u32 = 4385u32;
pub const TVM_SETTEXTCOLOR: u32 = 4382u32;
pub const TVM_SETTOOLTIPS: u32 = 4376u32;
pub const TVM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TVM_SHOWINFOTIP: u32 = 4423u32;
pub const TVM_SORTCHILDREN: u32 = 4371u32;
pub const TVM_SORTCHILDRENCB: u32 = 4373u32;
pub const TVNRET_DEFAULT: u32 = 0u32;
pub const TVNRET_SKIPNEW: u32 = 2u32;
pub const TVNRET_SKIPOLD: u32 = 1u32;
pub const TVSBF_XBORDER: u32 = 1u32;
pub const TVSBF_YBORDER: u32 = 2u32;
pub const TVSIL_NORMAL: u32 = 0u32;
pub const TVSIL_STATE: u32 = 2u32;
pub const TVSI_NOSINGLEEXPAND: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TVSORTCB(i32);
pub const TVS_CHECKBOXES: u32 = 256u32;
pub const TVS_DISABLEDRAGDROP: u32 = 16u32;
pub const TVS_EDITLABELS: u32 = 8u32;
pub const TVS_EX_AUTOHSCROLL: u32 = 32u32;
pub const TVS_EX_DIMMEDCHECKBOXES: u32 = 512u32;
pub const TVS_EX_DOUBLEBUFFER: u32 = 4u32;
pub const TVS_EX_DRAWIMAGEASYNC: u32 = 1024u32;
pub const TVS_EX_EXCLUSIONCHECKBOXES: u32 = 256u32;
pub const TVS_EX_FADEINOUTEXPANDOS: u32 = 64u32;
pub const TVS_EX_MULTISELECT: u32 = 2u32;
pub const TVS_EX_NOINDENTSTATE: u32 = 8u32;
pub const TVS_EX_NOSINGLECOLLAPSE: u32 = 1u32;
pub const TVS_EX_PARTIALCHECKBOXES: u32 = 128u32;
pub const TVS_EX_RICHTOOLTIP: u32 = 16u32;
pub const TVS_FULLROWSELECT: u32 = 4096u32;
pub const TVS_HASBUTTONS: u32 = 1u32;
pub const TVS_HASLINES: u32 = 2u32;
pub const TVS_INFOTIP: u32 = 2048u32;
pub const TVS_LINESATROOT: u32 = 4u32;
pub const TVS_NOHSCROLL: u32 = 32768u32;
pub const TVS_NONEVENHEIGHT: u32 = 16384u32;
pub const TVS_NOSCROLL: u32 = 8192u32;
pub const TVS_NOTOOLTIPS: u32 = 128u32;
pub const TVS_RTLREADING: u32 = 64u32;
pub const TVS_SHOWSELALWAYS: u32 = 32u32;
pub const TVS_SINGLEEXPAND: u32 = 1024u32;
pub const TVS_TRACKSELECT: u32 = 512u32;
pub const TV_FIRST: u32 = 4352u32;
#[repr(C)]
pub struct UDACCEL(i32);
pub const UDM_GETACCEL: u32 = 1132u32;
pub const UDM_GETBASE: u32 = 1134u32;
pub const UDM_GETBUDDY: u32 = 1130u32;
pub const UDM_GETPOS: u32 = 1128u32;
pub const UDM_GETPOS32: u32 = 1138u32;
pub const UDM_GETRANGE: u32 = 1126u32;
pub const UDM_GETRANGE32: u32 = 1136u32;
pub const UDM_GETUNICODEFORMAT: u32 = 8198u32;
pub const UDM_SETACCEL: u32 = 1131u32;
pub const UDM_SETBASE: u32 = 1133u32;
pub const UDM_SETBUDDY: u32 = 1129u32;
pub const UDM_SETPOS: u32 = 1127u32;
pub const UDM_SETPOS32: u32 = 1137u32;
pub const UDM_SETRANGE: u32 = 1125u32;
pub const UDM_SETRANGE32: u32 = 1135u32;
pub const UDM_SETUNICODEFORMAT: u32 = 8197u32;
pub const UDS_ALIGNLEFT: u32 = 8u32;
pub const UDS_ALIGNRIGHT: u32 = 4u32;
pub const UDS_ARROWKEYS: u32 = 32u32;
pub const UDS_AUTOBUDDY: u32 = 16u32;
pub const UDS_HORZ: u32 = 64u32;
pub const UDS_HOTTRACK: u32 = 256u32;
pub const UDS_NOTHOUSANDS: u32 = 128u32;
pub const UDS_SETBUDDYINT: u32 = 2u32;
pub const UDS_WRAP: u32 = 1u32;
pub const UD_MAXVAL: u32 = 32767u32;
#[repr(C)]
pub struct USAGE_PROPERTIES(i32);
#[repr(transparent)]
pub struct VALIGN(pub i32);
pub const VA_TOP: VALIGN = VALIGN(0i32);
pub const VA_CENTER: VALIGN = VALIGN(1i32);
pub const VA_BOTTOM: VALIGN = VALIGN(2i32);
pub const VIEW_DETAILS: u32 = 3u32;
pub const VIEW_LARGEICONS: u32 = 0u32;
pub const VIEW_LIST: u32 = 2u32;
pub const VIEW_NETCONNECT: u32 = 9u32;
pub const VIEW_NETDISCONNECT: u32 = 10u32;
pub const VIEW_NEWFOLDER: u32 = 11u32;
pub const VIEW_PARENTFOLDER: u32 = 8u32;
pub const VIEW_SMALLICONS: u32 = 1u32;
pub const VIEW_SORTDATE: u32 = 6u32;
pub const VIEW_SORTNAME: u32 = 4u32;
pub const VIEW_SORTSIZE: u32 = 5u32;
pub const VIEW_SORTTYPE: u32 = 7u32;
pub const VIEW_VIEWMENU: u32 = 12u32;
#[repr(transparent)]
pub struct WINDOWTHEMEATTRIBUTETYPE(pub i32);
pub const WTA_NONCLIENT: WINDOWTHEMEATTRIBUTETYPE = WINDOWTHEMEATTRIBUTETYPE(1i32);
pub const WIZ_BODYCX: u32 = 184u32;
pub const WIZ_BODYX: u32 = 92u32;
pub const WIZ_CXBMP: u32 = 80u32;
pub const WIZ_CXDLG: u32 = 276u32;
pub const WIZ_CYDLG: u32 = 140u32;
pub const WM_CTLCOLOR: u32 = 25u32;
pub const WM_MOUSEHOVER: u32 = 673u32;
pub const WM_MOUSELEAVE: u32 = 675u32;
#[repr(transparent)]
pub struct WORD_BREAK_ACTION(pub u32);
pub const WB_CLASSIFY: WORD_BREAK_ACTION = WORD_BREAK_ACTION(3u32);
pub const WB_ISDELIMITER: WORD_BREAK_ACTION = WORD_BREAK_ACTION(2u32);
pub const WB_LEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(0u32);
pub const WB_LEFTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(6u32);
pub const WB_MOVEWORDLEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(4u32);
pub const WB_MOVEWORDRIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(5u32);
pub const WB_RIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(1u32);
pub const WB_RIGHTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(7u32);
#[repr(transparent)]
pub struct WSB_PROP(pub i32);
pub const WSB_PROP_CXHSCROLL: WSB_PROP = WSB_PROP(2i32);
pub const WSB_PROP_CXHTHUMB: WSB_PROP = WSB_PROP(16i32);
pub const WSB_PROP_CXVSCROLL: WSB_PROP = WSB_PROP(8i32);
pub const WSB_PROP_CYHSCROLL: WSB_PROP = WSB_PROP(4i32);
pub const WSB_PROP_CYVSCROLL: WSB_PROP = WSB_PROP(1i32);
pub const WSB_PROP_CYVTHUMB: WSB_PROP = WSB_PROP(32i32);
pub const WSB_PROP_HBKGCOLOR: WSB_PROP = WSB_PROP(128i32);
pub const WSB_PROP_HSTYLE: WSB_PROP = WSB_PROP(512i32);
pub const WSB_PROP_PALETTE: WSB_PROP = WSB_PROP(2048i32);
pub const WSB_PROP_VBKGCOLOR: WSB_PROP = WSB_PROP(64i32);
pub const WSB_PROP_VSTYLE: WSB_PROP = WSB_PROP(256i32);
pub const WSB_PROP_WINSTYLE: WSB_PROP = WSB_PROP(1024i32);
pub const WSB_PROP_MASK: i32 = 4095i32;
#[repr(C)]
pub struct WTA_OPTIONS(i32);
pub const WTNCA_NODRAWCAPTION: u32 = 1u32;
pub const WTNCA_NODRAWICON: u32 = 2u32;
pub const WTNCA_NOMIRRORHELP: u32 = 8u32;
pub const WTNCA_NOSYSMENU: u32 = 4u32;
#[repr(transparent)]
pub struct _LI_METRIC(pub i32);
pub const LIM_SMALL: _LI_METRIC = _LI_METRIC(0i32);
pub const LIM_LARGE: _LI_METRIC = _LI_METRIC(1i32);
