#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn BeginBufferedAnimation(hwnd : super::windef::HWND, hdctarget : super::windef::HDC, prctarget : *const super::windef::RECT, dwformat : BP_BUFFERFORMAT, ppaintparams : *const BP_PAINTPARAMS, panimationparams : *const BP_ANIMATIONPARAMS, phdcfrom : *mut super::windef::HDC, phdcto : *mut super::windef::HDC) -> HANIMATIONBUFFER);
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn BeginBufferedPaint(hdctarget : super::windef::HDC, prctarget : *const super::windef::RECT, dwformat : BP_BUFFERFORMAT, ppaintparams : *const BP_PAINTPARAMS, phdc : *mut super::windef::HDC) -> HPAINTBUFFER);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn BeginPanningFeedback(hwnd : super::windef::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn BufferedPaintClear(hbufferedpaint : HPAINTBUFFER, prc : *const super::windef::RECT) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn BufferedPaintInit() -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn BufferedPaintRenderAnimation(hwnd : super::windef::HWND, hdctarget : super::windef::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn BufferedPaintSetAlpha(hbufferedpaint : HPAINTBUFFER, prc : *const super::windef::RECT, alpha : u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn BufferedPaintStopAllAnimations(hwnd : super::windef::HWND) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn BufferedPaintUnInit() -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn CloseThemeData(htheme : super::shobjidl_core::HTHEME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeBackground(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, prect : *const super::windef::RECT, pcliprect : *const super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeBackgroundEx(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, prect : *const super::windef::RECT, poptions : *const DTBGOPTS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeEdge(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, pdestrect : *const super::windef::RECT, uedge : u32, uflags : u32, pcontentrect : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "commctrl", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeIcon(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, prect : *const super::windef::RECT, himl : *const super::commctrl::_IMAGELIST, iimageindex : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeParentBackground(hwnd : super::windef::HWND, hdc : super::windef::HDC, prc : *const super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeParentBackgroundEx(hwnd : super::windef::HWND, hdc : super::windef::HDC, dwflags : u32, prc : *const super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeText(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, psztext : windows_sys::core::PCWSTR, cchtext : i32, dwtextflags : u32, dwtextflags2 : u32, prect : *const super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn DrawThemeTextEx(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, psztext : windows_sys::core::PCWSTR, cchtext : i32, dwtextflags : u32, prect : *mut super::windef::RECT, poptions : *const DTTOPTS) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn EnableThemeDialogTexture(hwnd : super::windef::HWND, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn EnableTheming(fenable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("uxtheme.dll" "system" fn EndBufferedAnimation(hbpanimation : HANIMATIONBUFFER, fupdatetarget : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("uxtheme.dll" "system" fn EndBufferedPaint(hbufferedpaint : HPAINTBUFFER, fupdatetarget : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn EndPanningFeedback(hwnd : super::windef::HWND, fanimateback : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetBufferedPaintBits(hbufferedpaint : HPAINTBUFFER, ppbbuffer : *mut *mut super::wingdi::RGBQUAD, pcxrow : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetBufferedPaintDC(hbufferedpaint : HPAINTBUFFER) -> super::windef::HDC);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetBufferedPaintTargetDC(hbufferedpaint : HPAINTBUFFER) -> super::windef::HDC);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetBufferedPaintTargetRect(hbufferedpaint : HPAINTBUFFER, prc : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn GetCurrentThemeName(pszthemefilename : windows_sys::core::PWSTR, cchmaxnamechars : i32, pszcolorbuff : windows_sys::core::PWSTR, cchmaxcolorchars : i32, pszsizebuff : windows_sys::core::PWSTR, cchmaxsizechars : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeAnimationProperty(htheme : super::shobjidl_core::HTHEME, istoryboardid : i32, itargetid : i32, eproperty : TA_PROPERTY, pvproperty : *mut core::ffi::c_void, cbsize : u32, pcbsizeout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeAnimationTransform(htheme : super::shobjidl_core::HTHEME, istoryboardid : i32, itargetid : i32, dwtransformindex : u32, ptransform : *mut TA_TRANSFORM, cbsize : u32, pcbsizeout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn GetThemeAppProperties() -> u32);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeBackgroundContentRect(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, pboundingrect : *const super::windef::RECT, pcontentrect : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeBackgroundExtent(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, pcontentrect : *const super::windef::RECT, pextentrect : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeBackgroundRegion(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, prect : *const super::windef::RECT, pregion : *mut super::minwindef::HRGN) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeBitmap(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, dwflags : u32, phbitmap : *mut super::windef::HBITMAP) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeBool(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pfval : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeColor(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pcolor : *mut super::windef::COLORREF) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn GetThemeDocumentationProperty(pszthemename : windows_sys::core::PCWSTR, pszpropertyname : windows_sys::core::PCWSTR, pszvaluebuff : windows_sys::core::PWSTR, cchmaxvalchars : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeEnumValue(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pival : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeFilename(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pszthemefilename : windows_sys::core::PWSTR, cchmaxbuffchars : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeFont(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, ipropid : i32, pfont : *mut super::wingdi::LOGFONTW) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeInt(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pival : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeIntList(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pintlist : *mut INTLIST) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeMargins(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, ipropid : i32, prc : *const super::windef::RECT, pmargins : *mut MARGINS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeMetric(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, ipropid : i32, pival : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemePartSize(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, prc : *const super::windef::RECT, esize : THEMESIZE, psz : *mut super::windef::SIZE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemePosition(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, ppoint : *mut super::windef::POINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemePropertyOrigin(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, porigin : *mut PROPERTYORIGIN) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeRect(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, prect : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeStream(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, ppvstream : *mut *mut core::ffi::c_void, pcbstream : *mut u32, hinst : super::minwindef::HINSTANCE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeString(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32, ipropid : i32, pszbuff : windows_sys::core::PWSTR, cchmaxbuffchars : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysBool(htheme : super::shobjidl_core::HTHEME, iboolid : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysColor(htheme : super::shobjidl_core::HTHEME, icolorid : i32) -> super::windef::COLORREF);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysColorBrush(htheme : super::shobjidl_core::HTHEME, icolorid : i32) -> super::windef::HBRUSH);
#[cfg(all(feature = "shobjidl_core", feature = "wingdi", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysFont(htheme : super::shobjidl_core::HTHEME, ifontid : i32, plf : *mut super::wingdi::LOGFONTW) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysInt(htheme : super::shobjidl_core::HTHEME, iintid : i32, pivalue : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysSize(htheme : super::shobjidl_core::HTHEME, isizeid : i32) -> i32);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeSysString(htheme : super::shobjidl_core::HTHEME, istringid : i32, pszstringbuff : windows_sys::core::PWSTR, cchmaxstringchars : i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeTextExtent(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, psztext : windows_sys::core::PCWSTR, cchcharcount : i32, dwtextflags : u32, pboundingrect : *const super::windef::RECT, pextentrect : *mut super::windef::RECT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeTextMetrics(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, ptm : *mut super::wingdi::TEXTMETRICW) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeTimingFunction(htheme : super::shobjidl_core::HTHEME, itimingfunctionid : i32, ptimingfunction : *mut TA_TIMINGFUNCTION, cbsize : u32, pcbsizeout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetThemeTransitionDuration(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateidfrom : i32, istateidto : i32, ipropid : i32, pdwduration : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn GetWindowTheme(hwnd : super::windef::HWND) -> super::shobjidl_core::HTHEME);
#[cfg(all(feature = "minwindef", feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn HitTestThemeBackground(htheme : super::shobjidl_core::HTHEME, hdc : super::windef::HDC, ipartid : i32, istateid : i32, dwoptions : u32, prect : *const super::windef::RECT, hrgn : super::minwindef::HRGN, pttest : super::windef::POINT, pwhittestcode : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("uxtheme.dll" "system" fn IsAppThemed() -> windows_sys::core::BOOL);
windows_link::link!("uxtheme.dll" "system" fn IsCompositionActive() -> windows_sys::core::BOOL);
windows_link::link!("uxtheme.dll" "system" fn IsThemeActive() -> windows_sys::core::BOOL);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn IsThemeBackgroundPartiallyTransparent(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn IsThemeDialogTextureEnabled(hwnd : super::windef::HWND) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shobjidl_core", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn IsThemePartDefined(htheme : super::shobjidl_core::HTHEME, ipartid : i32, istateid : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn OpenThemeData(hwnd : super::windef::HWND, pszclasslist : windows_sys::core::PCWSTR) -> super::shobjidl_core::HTHEME);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn OpenThemeDataEx(hwnd : super::windef::HWND, pszclasslist : windows_sys::core::PCWSTR, dwflags : u32) -> super::shobjidl_core::HTHEME);
#[cfg(all(feature = "shobjidl_core", feature = "windef", feature = "winnt"))]
windows_link::link!("uxtheme.dll" "system" fn OpenThemeDataForDpi(hwnd : super::windef::HWND, pszclasslist : windows_sys::core::PCWSTR, dpi : u32) -> super::shobjidl_core::HTHEME);
windows_link::link!("uxtheme.dll" "system" fn SetThemeAppProperties(dwflags : u32));
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn SetWindowTheme(hwnd : super::windef::HWND, pszsubappname : windows_sys::core::PCWSTR, pszsubidlist : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn SetWindowThemeAttribute(hwnd : super::windef::HWND, eattribute : WINDOWTHEMEATTRIBUTETYPE, pvattribute : *const core::ffi::c_void, cbattribute : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("uxtheme.dll" "system" fn UpdatePanningFeedback(hwnd : super::windef::HWND, ltotaloverpanoffsetx : i32, ltotaloverpanoffsety : i32, fininertia : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub const BPAS_CUBIC: BP_ANIMATIONSTYLE = 2;
pub const BPAS_LINEAR: BP_ANIMATIONSTYLE = 1;
pub const BPAS_NONE: BP_ANIMATIONSTYLE = 0;
pub const BPAS_SINE: BP_ANIMATIONSTYLE = 3;
pub const BPBF_COMPATIBLEBITMAP: BP_BUFFERFORMAT = 0;
pub const BPBF_COMPOSITED: u32 = 2;
pub const BPBF_DIB: BP_BUFFERFORMAT = 1;
pub const BPBF_TOPDOWNDIB: BP_BUFFERFORMAT = 2;
pub const BPBF_TOPDOWNMONODIB: BP_BUFFERFORMAT = 3;
pub const BPPF_ERASE: u32 = 1;
pub const BPPF_NOCLIP: u32 = 2;
pub const BPPF_NONCLIENT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BP_ANIMATIONPARAMS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub style: BP_ANIMATIONSTYLE,
    pub dwDuration: u32,
}
pub type BP_ANIMATIONSTYLE = i32;
pub type BP_BUFFERFORMAT = i32;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct BP_PAINTPARAMS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub prcExclude: *const super::windef::RECT,
    pub pBlendFunction: *const super::wingdi::BLENDFUNCTION,
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl Default for BP_PAINTPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DTBGOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub rcClip: super::windef::RECT,
}
pub const DTBG_CLIPRECT: u32 = 1;
pub const DTBG_COMPUTINGREGION: u32 = 16;
pub const DTBG_DRAWSOLID: u32 = 2;
pub const DTBG_MIRRORDC: u32 = 32;
pub const DTBG_NOMIRROR: u32 = 64;
pub const DTBG_OMITBORDER: u32 = 4;
pub const DTBG_OMITCONTENT: u32 = 8;
pub const DTBG_VALIDBITS: u32 = 127;
pub const DTPB_USECTLCOLORSTATIC: u32 = 2;
pub const DTPB_USEERASEBKGND: u32 = 4;
pub const DTPB_WINDOWDC: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DTTOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub crText: super::windef::COLORREF,
    pub crBorder: super::windef::COLORREF,
    pub crShadow: super::windef::COLORREF,
    pub iTextShadowType: i32,
    pub ptShadowOffset: super::windef::POINT,
    pub iBorderSize: i32,
    pub iFontPropId: i32,
    pub iColorPropId: i32,
    pub iStateId: i32,
    pub fApplyOverlay: windows_sys::core::BOOL,
    pub iGlowSize: i32,
    pub pfnDrawTextCallback: DTT_CALLBACK_PROC,
    pub lParam: super::minwindef::LPARAM,
}
pub const DTT_APPLYOVERLAY: u32 = 1024;
pub const DTT_BORDERCOLOR: u32 = 2;
pub const DTT_BORDERSIZE: u32 = 32;
pub const DTT_CALCRECT: u32 = 512;
pub const DTT_CALLBACK: u32 = 4096;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type DTT_CALLBACK_PROC = Option<unsafe extern "system" fn(hdc: super::windef::HDC, psztext: windows_sys::core::PWSTR, cchtext: i32, prc: *mut super::windef::RECT, dwflags: u32, lparam: super::minwindef::LPARAM) -> i32>;
pub const DTT_COLORPROP: u32 = 128;
pub const DTT_COMPOSITED: u32 = 8192;
pub const DTT_FLAGS2VALIDBITS: u32 = 1;
pub const DTT_FONTPROP: u32 = 64;
pub const DTT_GLOWSIZE: u32 = 2048;
pub const DTT_GRAYED: u32 = 1;
pub const DTT_SHADOWCOLOR: u32 = 4;
pub const DTT_SHADOWOFFSET: u32 = 16;
pub const DTT_SHADOWTYPE: u32 = 8;
pub const DTT_STATEID: u32 = 256;
pub const DTT_TEXTCOLOR: u32 = 1;
pub const DTT_VALIDBITS: u32 = 12287;
pub const ETDT_DISABLE: u32 = 1;
pub const ETDT_ENABLE: u32 = 2;
pub const ETDT_ENABLEAEROWIZARDTAB: u32 = 10;
pub const ETDT_ENABLETAB: u32 = 6;
pub const ETDT_USEAEROWIZARDTABTEXTURE: u32 = 8;
pub const ETDT_USETABTEXTURE: u32 = 4;
pub const ETDT_VALIDBITS: u32 = 15;
pub const GBF_COPY: u32 = 2;
pub const GBF_DIRECT: u32 = 1;
pub const GBF_VALIDBITS: u32 = 3;
#[cfg(feature = "winnt")]
pub type HANIMATIONBUFFER = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type HPAINTBUFFER = super::winnt::HANDLE;
pub const HTTB_BACKGROUNDSEG: u32 = 0;
pub const HTTB_CAPTION: u32 = 4;
pub const HTTB_FIXEDBORDER: u32 = 2;
pub const HTTB_RESIZINGBORDER: u32 = 240;
pub const HTTB_RESIZINGBORDER_BOTTOM: u32 = 128;
pub const HTTB_RESIZINGBORDER_LEFT: u32 = 16;
pub const HTTB_RESIZINGBORDER_RIGHT: u32 = 64;
pub const HTTB_RESIZINGBORDER_TOP: u32 = 32;
pub const HTTB_SIZINGTEMPLATE: u32 = 256;
pub const HTTB_SYSTEMSIZINGMARGINS: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTLIST {
    pub iValueCount: i32,
    pub iValues: [i32; 402],
}
impl Default for INTLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MARGINS {
    pub cxLeftWidth: i32,
    pub cxRightWidth: i32,
    pub cyTopHeight: i32,
    pub cyBottomHeight: i32,
}
pub const MAX_INTLIST_COUNT: u32 = 402;
pub const MAX_THEMECOLOR: u32 = 64;
pub const MAX_THEMESIZE: u32 = 64;
pub const OTD_FORCE_RECT_SIZING: u32 = 1;
pub const OTD_NONCLIENT: u32 = 2;
pub const OTD_VALIDBITS: u32 = 3;
pub type PBP_ANIMATIONPARAMS = *mut BP_ANIMATIONPARAMS;
#[cfg(all(feature = "windef", feature = "wingdi"))]
pub type PBP_PAINTPARAMS = *mut BP_PAINTPARAMS;
#[cfg(feature = "windef")]
pub type PDTBGOPTS = *mut DTBGOPTS;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PDTTOPTS = *mut DTTOPTS;
pub type PINTLIST = *mut INTLIST;
pub type PMARGINS = *mut MARGINS;
pub const PO_CLASS: PROPERTYORIGIN = 2;
pub const PO_GLOBAL: PROPERTYORIGIN = 3;
pub const PO_NOTFOUND: PROPERTYORIGIN = 4;
pub const PO_PART: PROPERTYORIGIN = 1;
pub const PO_STATE: PROPERTYORIGIN = 0;
pub type PROPERTYORIGIN = i32;
pub type PTA_CUBIC_BEZIER = *mut TA_CUBIC_BEZIER;
pub type PTA_TIMINGFUNCTION = *mut TA_TIMINGFUNCTION;
pub type PTA_TRANSFORM = *mut TA_TRANSFORM;
pub type PTA_TRANSFORM_2D = *mut TA_TRANSFORM_2D;
pub type PTA_TRANSFORM_CLIP = *mut TA_TRANSFORM_CLIP;
pub type PTA_TRANSFORM_OPACITY = *mut TA_TRANSFORM_OPACITY;
pub type PWTA_OPTIONS = *mut WTA_OPTIONS;
pub const STAP_ALLOW_CONTROLS: u32 = 2;
pub const STAP_ALLOW_NONCLIENT: u32 = 1;
pub const STAP_ALLOW_WEBCONTENT: u32 = 4;
pub const STAP_VALIDBITS: u32 = 7;
pub const SZ_THDOCPROP_AUTHOR: windows_sys::core::PCWSTR = windows_sys::core::w!("author");
pub const SZ_THDOCPROP_CANONICALNAME: windows_sys::core::PCWSTR = windows_sys::core::w!("ThemeName");
pub const SZ_THDOCPROP_DISPLAYNAME: windows_sys::core::PCWSTR = windows_sys::core::w!("DisplayName");
pub const SZ_THDOCPROP_TOOLTIP: windows_sys::core::PCWSTR = windows_sys::core::w!("ToolTip");
pub const TAPF_ALLOWCOLLECTION: TA_PROPERTY_FLAG = 4;
pub const TAPF_HASBACKGROUND: TA_PROPERTY_FLAG = 8;
pub const TAPF_HASPERSPECTIVE: TA_PROPERTY_FLAG = 16;
pub const TAPF_HASSTAGGER: TA_PROPERTY_FLAG = 1;
pub const TAPF_ISRTLAWARE: TA_PROPERTY_FLAG = 2;
pub const TAPF_NONE: TA_PROPERTY_FLAG = 0;
pub const TAP_FLAGS: TA_PROPERTY = 0;
pub const TAP_STAGGERDELAY: TA_PROPERTY = 2;
pub const TAP_STAGGERDELAYCAP: TA_PROPERTY = 3;
pub const TAP_STAGGERDELAYFACTOR: TA_PROPERTY = 4;
pub const TAP_TRANSFORMCOUNT: TA_PROPERTY = 1;
pub const TAP_ZORDER: TA_PROPERTY = 5;
pub const TATF_HASINITIALVALUES: TA_TRANSFORM_FLAG = 2;
pub const TATF_HASORIGINVALUES: TA_TRANSFORM_FLAG = 4;
pub const TATF_NONE: TA_TRANSFORM_FLAG = 0;
pub const TATF_TARGETVALUES_USER: TA_TRANSFORM_FLAG = 1;
pub const TATT_CLIP: TA_TRANSFORM_TYPE = 3;
pub const TATT_OPACITY: TA_TRANSFORM_TYPE = 2;
pub const TATT_SCALE_2D: TA_TRANSFORM_TYPE = 1;
pub const TATT_TRANSLATE_2D: TA_TRANSFORM_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TA_CUBIC_BEZIER {
    pub header: TA_TIMINGFUNCTION,
    pub rX0: f32,
    pub rY0: f32,
    pub rX1: f32,
    pub rY1: f32,
}
pub type TA_PROPERTY = i32;
pub type TA_PROPERTY_FLAG = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TA_TIMINGFUNCTION {
    pub eTimingFunctionType: TA_TIMINGFUNCTION_TYPE,
}
pub type TA_TIMINGFUNCTION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TA_TRANSFORM {
    pub eTransformType: TA_TRANSFORM_TYPE,
    pub dwTimingFunctionId: u32,
    pub dwStartTime: u32,
    pub dwDurationTime: u32,
    pub eFlags: TA_TRANSFORM_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TA_TRANSFORM_2D {
    pub header: TA_TRANSFORM,
    pub rX: f32,
    pub rY: f32,
    pub rInitialX: f32,
    pub rInitialY: f32,
    pub rOriginX: f32,
    pub rOriginY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TA_TRANSFORM_CLIP {
    pub header: TA_TRANSFORM,
    pub rLeft: f32,
    pub rTop: f32,
    pub rRight: f32,
    pub rBottom: f32,
    pub rInitialLeft: f32,
    pub rInitialTop: f32,
    pub rInitialRight: f32,
    pub rInitialBottom: f32,
}
pub type TA_TRANSFORM_FLAG = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TA_TRANSFORM_OPACITY {
    pub header: TA_TRANSFORM,
    pub rOpacity: f32,
    pub rInitialOpacity: f32,
}
pub type TA_TRANSFORM_TYPE = i32;
pub type THEMESIZE = i32;
pub const TS_DRAW: THEMESIZE = 2;
pub const TS_MIN: THEMESIZE = 0;
pub const TS_TRUE: THEMESIZE = 1;
pub const TTFT_CUBIC_BEZIER: TA_TIMINGFUNCTION_TYPE = 1;
pub const TTFT_UNDEFINED: TA_TIMINGFUNCTION_TYPE = 0;
pub type WINDOWTHEMEATTRIBUTETYPE = i32;
pub const WTA_NONCLIENT: WINDOWTHEMEATTRIBUTETYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTA_OPTIONS {
    pub dwFlags: u32,
    pub dwMask: u32,
}
pub const WTNCA_NODRAWCAPTION: u32 = 1;
pub const WTNCA_NODRAWICON: u32 = 2;
pub const WTNCA_NOMIRRORHELP: u32 = 8;
pub const WTNCA_NOSYSMENU: u32 = 4;
pub const WTNCA_VALIDBITS: u32 = 15;
