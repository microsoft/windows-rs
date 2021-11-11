#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub mod Dialogs;
#[cfg(feature = "Win32_UI_Controls_RichEdit")]
pub mod RichEdit;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BeginBufferedAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut super::super::Graphics::Gdi::HDC, phdcto: *mut super::super::Graphics::Gdi::HDC) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BeginBufferedPaint(hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, phdc: *mut super::super::Graphics::Gdi::HDC) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPanningFeedback(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintClear(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn BufferedPaintInit() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BufferedPaintRenderAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT, alpha: u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintStopAllAnimations(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn BufferedPaintUnInit() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckDlgButton(hdlg: super::super::Foundation::HWND, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckRadioButton(hdlg: super::super::Foundation::HWND, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn CloseThemeData(htheme: isize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateMappedBitmap(hinstance: super::super::Foundation::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: *const COLORMAP, inummaps: i32) -> super::super::Graphics::Gdi::HBITMAP;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut ::core::mem::ManuallyDrop<PROPSHEETPAGEA>) -> HPROPSHEETPAGE;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut ::core::mem::ManuallyDrop<PROPSHEETPAGEW>) -> HPROPSHEETPAGE;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStatusWindowA(style: i32, lpsztext: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, wid: u32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStatusWindowW(style: i32, lpsztext: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, wid: u32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateToolbarEx(hwnd: super::super::Foundation::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: super::super::Foundation::HINSTANCE, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: super::super::Foundation::HWND, nid: i32, hinst: super::super::Foundation::HINSTANCE, hbuddy: super::super::Foundation::HWND, nupper: i32, nlower: i32, npos: i32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_Clone(hdpa: HDPA, hdpanew: HDPA) -> HDPA;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_Create(citemgrow: i32) -> HDPA;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_CreateEx(cpgrow: i32, hheap: super::super::Foundation::HANDLE) -> HDPA;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_DeleteAllPtrs(hdpa: HDPA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_DeletePtr(hdpa: HDPA, i: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Destroy(hdpa: HDPA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_DestroyCallback(hdpa: HDPA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_EnumCallback(hdpa: HDPA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_GetPtr(hdpa: HDPA, i: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_GetPtrIndex(hdpa: HDPA, p: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_GetSize(hdpa: HDPA) -> u64;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Grow(pdpa: HDPA, cp: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_InsertPtr(hdpa: HDPA, i: i32, p: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DPA_LoadStream(phdpa: *mut HDPA, pfn: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pvinstdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Merge(hdpadest: HDPA, hdpasrc: HDPA, dwflags: u32, pfncompare: ::windows::runtime::RawPtr, pfnmerge: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DPA_SaveStream(hdpa: HDPA, pfn: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pvinstdata: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Search(hdpa: HDPA, pfind: *const ::core::ffi::c_void, istart: i32, pfncompare: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM, options: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_SetPtr(hdpa: HDPA, i: i32, p: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Sort(hdpa: HDPA, pfncompare: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_Clone(hdsa: HDSA) -> HDSA;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_DeleteAllItems(hdsa: HDSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_DeleteItem(hdsa: HDSA, i: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_Destroy(hdsa: HDSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_DestroyCallback(hdsa: HDSA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_EnumCallback(hdsa: HDSA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_GetItem(hdsa: HDSA, i: i32, pitem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_GetItemPtr(hdsa: HDSA, i: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_GetSize(hdsa: HDSA) -> u64;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_InsertItem(hdsa: HDSA, i: i32, pitem: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_SetItem(hdsa: HDSA, i: i32, pitem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_Sort(pdsa: HDSA, pfncompare: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPropertySheetPage(param0: HPROPSHEETPAGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DestroySyntheticPointerDevice(device: HSYNTHETICPOINTERDEVICE);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListA(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListComboBoxA(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListComboBoxW(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListW(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectComboBoxExA(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectComboBoxExW(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectExA(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectExW(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawInsert(handparent: super::super::Foundation::HWND, hlb: super::super::Foundation::HWND, nitem: i32);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawShadowText(hdc: super::super::Graphics::Gdi::HDC, psztext: super::super::Foundation::PWSTR, cch: u32, prc: *const super::super::Foundation::RECT, dwflags: u32, crtext: u32, crshadow: u32, ixoffset: i32, iyoffset: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawStatusTextA(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: super::super::Foundation::PSTR, uflags: u32);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawStatusTextW(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: super::super::Foundation::PWSTR, uflags: u32);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeBackground(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pcliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeBackgroundEx(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, poptions: *const DTBGOPTS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeEdge(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pdestrect: *const super::super::Foundation::RECT, uedge: u32, uflags: u32, pcontentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeIcon(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, himl: HIMAGELIST, iimageindex: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeParentBackground(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeParentBackgroundEx(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeText(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchtext: i32, dwtextflags: u32, dwtextflags2: u32, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeTextEx(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchtext: i32, dwtextflags: u32, prect: *mut super::super::Foundation::RECT, poptions: *const ::core::mem::ManuallyDrop<DTTOPTS>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn EnableScrollBar(hwnd: super::super::Foundation::HWND, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableThemeDialogTexture(hwnd: super::super::Foundation::HWND, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableTheming(fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndBufferedAnimation(hbpanimation: isize, fupdatetarget: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndBufferedPaint(hbufferedpaint: isize, fupdatetarget: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPanningFeedback(hwnd: super::super::Foundation::HWND, fanimateback: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvaluateProximityToPolygon(numvertices: u32, controlpolygon: *const super::super::Foundation::POINT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvaluateProximityToRect(controlboundingbox: *const super::super::Foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_EnableScrollBar(param0: super::super::Foundation::HWND, param1: i32, param2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_GetScrollProp(param0: super::super::Foundation::HWND, propindex: WSB_PROP, param2: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_SetScrollProp(param0: super::super::Foundation::HWND, index: WSB_PROP, newvalue: isize, param3: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_ShowScrollBar(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut super::super::Graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBufferedPaintTargetRect(hbufferedpaint: isize, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComboBoxInfo(hwndcombo: super::super::Foundation::HWND, pcbi: *mut COMBOBOXINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentThemeName(pszthemefilename: super::super::Foundation::PWSTR, cchmaxnamechars: i32, pszcolorbuff: super::super::Foundation::PWSTR, cchmaxcolorchars: i32, pszsizebuff: super::super::Foundation::PWSTR, cchmaxsizechars: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveClientRect(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT, lpinfo: *const i32);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetListBoxInfo(hwnd: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetMUILanguage() -> u16;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeAnimationProperty(htheme: isize, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: *mut ::core::ffi::c_void, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeAnimationTransform(htheme: isize, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: *mut TA_TRANSFORM, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeAppProperties() -> u32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundContentRect(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pboundingrect: *const super::super::Foundation::RECT, pcontentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundExtent(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pcontentrect: *const super::super::Foundation::RECT, pextentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundRegion(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pregion: *mut super::super::Graphics::Gdi::HRGN) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeBitmap(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeBool(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pfval: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeColor(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pcolor: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeDocumentationProperty(pszthemename: super::super::Foundation::PWSTR, pszpropertyname: super::super::Foundation::PWSTR, pszvaluebuff: super::super::Foundation::PWSTR, cchmaxvalchars: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeEnumValue(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeFilename(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszthemefilename: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeFont(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, pfont: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeInt(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeIntList(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pintlist: *mut INTLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeMargins(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, prc: *const super::super::Foundation::RECT, pmargins: *mut MARGINS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeMetric(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pival: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemePartSize(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prc: *const super::super::Foundation::RECT, esize: THEMESIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemePosition(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppoint: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemePropertyOrigin(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, porigin: *mut PROPERTYORIGIN) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeRect(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, prect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeStream(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::core::ffi::c_void, pcbstream: *mut u32, hinst: super::super::Foundation::HINSTANCE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeString(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeSysBool(htheme: isize, iboolid: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeSysColor(htheme: isize, icolorid: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeSysColorBrush(htheme: isize, icolorid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Graphics::Gdi::HBRUSH;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeSysFont(htheme: isize, ifontid: THEME_PROPERTY_SYMBOL_ID, plf: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeSysInt(htheme: isize, iintid: i32, pivalue: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeSysSize(htheme: isize, isizeid: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeSysString(htheme: isize, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: super::super::Foundation::PWSTR, cchmaxstringchars: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeTextExtent(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchcharcount: i32, dwtextflags: u32, pboundingrect: *const super::super::Foundation::RECT, pextentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeTextMetrics(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeTimingFunction(htheme: isize, itimingfunctionid: i32, ptimingfunction: *mut TA_TIMINGFUNCTION, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeTransitionDuration(htheme: isize, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32, pdwduration: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTheme(hwnd: super::super::Foundation::HWND) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn HIMAGELIST_QueryInterface(himl: HIMAGELIST, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn HitTestThemeBackground(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, dwoptions: u32, prect: *const super::super::Foundation::RECT, hrgn: super::super::Graphics::Gdi::HRGN, pttest: super::super::Foundation::POINT, pwhittestcode: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ImageList_Add(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ImageList_AddMasked(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_BeginDrag(himltrack: HIMAGELIST, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_CoCreateInstance(rclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Copy(himldst: HIMAGELIST, idst: i32, himlsrc: HIMAGELIST, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Destroy(himl: HIMAGELIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragEnter(hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragLeave(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragMove(x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragShowNolock(fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_Draw(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_DrawEx(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: u32, rgbfg: u32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_EndDrag();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_GetBkColor(himl: HIMAGELIST) -> u32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_GetDragImage(ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn ImageList_GetIcon(himl: HIMAGELIST, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_GetIconSize(himl: HIMAGELIST, cx: *mut i32, cy: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_GetImageCount(himl: HIMAGELIST) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_GetImageInfo(himl: HIMAGELIST, i: i32, pimageinfo: *mut IMAGEINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ImageList_LoadImageA(hi: super::super::Foundation::HINSTANCE, lpbmp: super::super::Foundation::PSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ImageList_LoadImageW(hi: super::super::Foundation::HINSTANCE, lpbmp: super::super::Foundation::PWSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_Merge(himl1: HIMAGELIST, i1: i32, himl2: HIMAGELIST, i2: i32, dx: i32, dy: i32) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_Read(pstm: ::windows::runtime::RawPtr) -> HIMAGELIST;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_ReadEx(dwflags: u32, pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Remove(himl: HIMAGELIST, i: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_Replace(himl: HIMAGELIST, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn ImageList_ReplaceIcon(himl: HIMAGELIST, i: i32, hicon: super::WindowsAndMessaging::HICON) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_SetBkColor(himl: HIMAGELIST, clrbk: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetDragCursorImage(himldrag: HIMAGELIST, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetIconSize(himl: HIMAGELIST, cx: i32, cy: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetImageCount(himl: HIMAGELIST, unewcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetOverlayImage(himl: HIMAGELIST, iimage: i32, ioverlay: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ImageList_Write(himl: HIMAGELIST, pstm: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_WriteEx(himl: HIMAGELIST, dwflags: u32, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn InitCommonControls();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn InitMUILanguage(uilang: u16);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeFlatSB(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsAppThemed() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharLowerW(ch: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCompositionActive() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDlgButtonChecked(hdlg: super::super::Foundation::HWND, nidbutton: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeActive() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeBackgroundPartiallyTransparent(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeDialogTextureEnabled(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemePartDefined(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LBItemFromPt(hlb: super::super::Foundation::HWND, pt: super::super::Foundation::POINT, bautoscroll: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn LoadIconMetric(hinst: super::super::Foundation::HINSTANCE, pszname: super::super::Foundation::PWSTR, lims: _LI_METRIC, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn LoadIconWithScaleDown(hinst: super::super::Foundation::HINSTANCE, pszname: super::super::Foundation::PWSTR, cx: i32, cy: i32, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeDragList(hlb: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn MenuHelp(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, hmainmenu: super::WindowsAndMessaging::HMENU, hinst: super::super::Foundation::HINSTANCE, hwndstatus: super::super::Foundation::HWND, lpwids: *const u32);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeData(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeDataEx(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR, dwflags: OPEN_THEME_DATA_FLAGS) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn PropertySheetA(param0: *mut ::core::mem::ManuallyDrop<PROPSHEETHEADERA_V2>) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn PropertySheetW(param0: *mut ::core::mem::ManuallyDrop<PROPSHEETHEADERW_V2>) -> isize;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterPointerDeviceNotifications(window: super::super::Foundation::HWND, notifyrange: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTouchHitTestingWindow(hwnd: super::super::Foundation::HWND, value: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollInfo(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollPos(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollRange(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn SetThemeAppProperties(dwflags: u32);
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTheme(hwnd: super::super::Foundation::HWND, pszsubappname: super::super::Foundation::PWSTR, pszsubidlist: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowThemeAttribute(hwnd: super::super::Foundation::HWND, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowHideMenuCtl(hwnd: super::super::Foundation::HWND, uflags: usize, lpinfo: *const i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShowScrollBar(hwnd: super::super::Foundation::HWND, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Str_SetPtrW(ppsz: *mut super::super::Foundation::PWSTR, psz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TaskDialog(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszwindowtitle: super::super::Foundation::PWSTR, pszmaininstruction: super::super::Foundation::PWSTR, pszcontent: super::super::Foundation::PWSTR, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: super::super::Foundation::PWSTR, pnbutton: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn TaskDialogIndirect(ptaskconfig: *const ::core::mem::ManuallyDrop<TASKDIALOGCONFIG>, pnbutton: *mut i32, pnradiobutton: *mut i32, pfverificationflagchecked: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninitializeFlatSB(param0: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePanningFeedback(hwnd: super::super::Foundation::HWND, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
