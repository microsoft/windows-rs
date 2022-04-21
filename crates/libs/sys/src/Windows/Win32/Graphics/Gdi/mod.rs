#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontMemResourceEx(pfileview: *const ::core::ffi::c_void, cjsize: u32, pvresrved: *mut ::core::ffi::c_void, pnumfonts: *const u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn AddFontResourceA(param0: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn AddFontResourceExA(name: ::windows_sys::core::PCSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn AddFontResourceExW(name: ::windows_sys::core::PCWSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn AddFontResourceW(param0: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AngleArc(hdc: HDC, x: i32, y: i32, r: u32, startangle: f32, sweepangle: f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnimatePalette(hpal: HPALETTE, istartindex: u32, centries: u32, ppe: *const PALETTEENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Arc(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ArcTo(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPaint(hwnd: super::super::Foundation::HWND, lppaint: *mut PAINTSTRUCT) -> HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BitBlt(hdc: HDC, x: i32, y: i32, cx: i32, cy: i32, hdcsrc: HDC, x1: i32, y1: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDC(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsA(lpdevmode: *const DEVMODEA, dwflags: CDS_TYPE) -> DISP_CHANGE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExA(lpszdevicename: ::windows_sys::core::PCSTR, lpdevmode: *const DEVMODEA, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExW(lpszdevicename: ::windows_sys::core::PCWSTR, lpdevmode: *const DEVMODEW, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsW(lpdevmode: *const DEVMODEW, dwflags: CDS_TYPE) -> DISP_CHANGE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Chord(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClientToScreen(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CloseEnhMetaFile(hdc: HDC) -> HENHMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseFigure(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CloseMetaFile(hdc: HDC) -> HMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CombineRgn(hrgndst: HRGN, hrgnsrc1: HRGN, hrgnsrc2: HRGN, imode: RGN_COMBINE_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CombineTransform(lpxfout: *mut XFORM, lpxf1: *const XFORM, lpxf2: *const XFORM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CopyEnhMetaFileA(henh: HENHMETAFILE, lpfilename: ::windows_sys::core::PCSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CopyEnhMetaFileW(henh: HENHMETAFILE, lpfilename: ::windows_sys::core::PCWSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CopyMetaFileA(param0: HMETAFILE, param1: ::windows_sys::core::PCSTR) -> HMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CopyMetaFileW(param0: HMETAFILE, param1: ::windows_sys::core::PCWSTR) -> HMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateBitmap(nwidth: i32, nheight: i32, nplanes: u32, nbitcount: u32, lpbits: *const ::core::ffi::c_void) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateBitmapIndirect(pbm: *const BITMAP) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateBrushIndirect(plbrush: *const LOGBRUSH) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateCompatibleBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateCompatibleDC(hdc: HDC) -> CreatedHDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCA(pwszdriver: ::windows_sys::core::PCSTR, pwszdevice: ::windows_sys::core::PCSTR, pszport: ::windows_sys::core::PCSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCW(pwszdriver: ::windows_sys::core::PCWSTR, pwszdevice: ::windows_sys::core::PCWSTR, pszport: ::windows_sys::core::PCWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateDIBPatternBrush(h: isize, iusage: DIB_USAGE) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateDIBPatternBrushPt(lppackeddib: *const ::core::ffi::c_void, iusage: DIB_USAGE) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDIBSection(hdc: HDC, pbmi: *const BITMAPINFO, usage: DIB_USAGE, ppvbits: *mut *mut ::core::ffi::c_void, hsection: super::super::Foundation::HANDLE, offset: u32) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateDIBitmap(hdc: HDC, pbmih: *const BITMAPINFOHEADER, flinit: u32, pjbits: *const ::core::ffi::c_void, pbmi: *const BITMAPINFO, iusage: DIB_USAGE) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateDiscardableBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEllipticRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileA(hdc: HDC, lpfilename: ::windows_sys::core::PCSTR, lprc: *const super::super::Foundation::RECT, lpdesc: ::windows_sys::core::PCSTR) -> HdcMetdataEnhFileHandle;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileW(hdc: HDC, lpfilename: ::windows_sys::core::PCWSTR, lprc: *const super::super::Foundation::RECT, lpdesc: ::windows_sys::core::PCWSTR) -> HdcMetdataEnhFileHandle;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateFontA(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: ::windows_sys::core::PCSTR) -> HFONT;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectExA(param0: *const ENUMLOGFONTEXDVA) -> HFONT;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateFontIndirectExW(param0: *const ENUMLOGFONTEXDVW) -> HFONT;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateFontIndirectW(lplf: *const LOGFONTW) -> HFONT;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateFontPackage(puchsrcbuffer: *const u8, ulsrcbuffersize: u32, ppuchfontpackagebuffer: *mut *mut u8, pulfontpackagebuffersize: *mut u32, pulbyteswritten: *mut u32, usflag: u16, usttcindex: u16, ussubsetformat: u16, ussubsetlanguage: u16, ussubsetplatform: CREATE_FONT_PACKAGE_SUBSET_PLATFORM, ussubsetencoding: CREATE_FONT_PACKAGE_SUBSET_ENCODING, pussubsetkeeplist: *const u16, ussubsetlistcount: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateFontW(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: ::windows_sys::core::PCWSTR) -> HFONT;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateHalftonePalette(hdc: HDC) -> HPALETTE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateHatchBrush(ihatch: HATCH_BRUSH_STYLE, color: u32) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICA(pszdriver: ::windows_sys::core::PCSTR, pszdevice: ::windows_sys::core::PCSTR, pszport: ::windows_sys::core::PCSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICW(pszdriver: ::windows_sys::core::PCWSTR, pszdevice: ::windows_sys::core::PCWSTR, pszport: ::windows_sys::core::PCWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateMetaFileA(pszfile: ::windows_sys::core::PCSTR) -> HdcMetdataFileHandle;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateMetaFileW(pszfile: ::windows_sys::core::PCWSTR) -> HdcMetdataFileHandle;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreatePalette(plpal: *const LOGPALETTE) -> HPALETTE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreatePatternBrush(hbm: HBITMAP) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreatePen(istyle: PEN_STYLE, cwidth: i32, color: u32) -> HPEN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePenIndirect(plpen: *const LOGPEN) -> HPEN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolyPolygonRgn(pptl: *const super::super::Foundation::POINT, pc: *const i32, cpoly: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolygonRgn(pptl: *const super::super::Foundation::POINT, cpoint: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRectRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceA(fdwhidden: u32, lpszfont: ::windows_sys::core::PCSTR, lpszfile: ::windows_sys::core::PCSTR, lpszpath: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceW(fdwhidden: u32, lpszfont: ::windows_sys::core::PCWSTR, lpszfile: ::windows_sys::core::PCWSTR, lpszpath: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn CreateSolidBrush(color: u32) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPtoLP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteDC(hdc: CreatedHDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteEnhMetaFile(hmf: HENHMETAFILE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMetaFile(hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteObject(ho: HGDIOBJ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawAnimatedRects(hwnd: super::super::Foundation::HWND, idani: i32, lprcfrom: *const super::super::Foundation::RECT, lprcto: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawCaption(hwnd: super::super::Foundation::HWND, hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: DRAW_CAPTION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEdge(hdc: HDC, qrc: *mut super::super::Foundation::RECT, edge: DRAWEDGE_FLAGS, grfflags: DRAW_EDGE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn DrawEscape(hdc: HDC, iescape: i32, cjin: i32, lpin: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFocusRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFrameControl(param0: HDC, param1: *mut super::super::Foundation::RECT, param2: DFC_TYPE, param3: DFCS_STATE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateA(hdc: HDC, hbrfore: HBRUSH, qfncallback: DRAWSTATEPROC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateW(hdc: HDC, hbrfore: HBRUSH, qfncallback: DRAWSTATEPROC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextA(hdc: HDC, lpchtext: ::windows_sys::core::PCSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExA(hdc: HDC, lpchtext: ::windows_sys::core::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExW(hdc: HDC, lpchtext: ::windows_sys::core::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextW(hdc: HDC, lpchtext: ::windows_sys::core::PCWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Ellipse(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPaint(hwnd: super::super::Foundation::HWND, lppaint: *const PAINTSTRUCT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesA(lpdevice: ::windows_sys::core::PCSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEA, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesW(lpdevice: ::windows_sys::core::PCWSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEW, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayMonitors(hdc: HDC, lprcclip: *const super::super::Foundation::RECT, lpfnenum: MONITORENUMPROC, dwdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsA(lpszdevicename: ::windows_sys::core::PCSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExA(lpszdevicename: ::windows_sys::core::PCSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExW(lpszdevicename: ::windows_sys::core::PCWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsW(lpszdevicename: ::windows_sys::core::PCWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, proc: ENHMFENUMPROC, param3: *const ::core::ffi::c_void, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesA(hdc: HDC, lplogfont: ::windows_sys::core::PCSTR, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExA(hdc: HDC, lplogfont: *const LOGFONTA, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExW(hdc: HDC, lplogfont: *const LOGFONTW, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesW(hdc: HDC, lplogfont: ::windows_sys::core::PCWSTR, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsA(hdc: HDC, lplogfont: ::windows_sys::core::PCSTR, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsW(hdc: HDC, lplogfont: ::windows_sys::core::PCWSTR, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMetaFile(hdc: HDC, hmf: HMETAFILE, proc: MFENUMPROC, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumObjects(hdc: HDC, ntype: OBJ_TYPE, lpfunc: GOBJENUMPROC, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRect(lprc1: *const super::super::Foundation::RECT, lprc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRgn(hrgn1: HRGN, hrgn2: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn ExcludeClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExcludeUpdateRgn(hdc: HDC, hwnd: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn ExtCreatePen(ipenstyle: PEN_STYLE, cwidth: u32, plbrush: *const LOGBRUSH, cstyle: u32, pstyle: *const u32) -> HPEN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtCreateRegion(lpx: *const XFORM, ncount: u32, lpdata: *const RGNDATA) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtFloodFill(hdc: HDC, x: i32, y: i32, color: u32, r#type: EXT_FLOOD_FILL_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn ExtSelectClipRgn(hdc: HDC, hrgn: HRGN, mode: RGN_COMBINE_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutA(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: ::windows_sys::core::PCSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutW(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: ::windows_sys::core::PCWSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FixBrushOrgEx(hdc: HDC, x: i32, y: i32, ptl: *const super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlattenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FloodFill(hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH, w: i32, h: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiAlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiComment(hdc: HDC, nsize: u32, lpdata: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiFlush() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GdiGetBatchLimit() -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, ncount: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GdiSetBatchLimit(dw: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiTransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetArcDirection(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAspectRatioFilterEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetBitmapBits(hbit: HBITMAP, cb: i32, lpvbits: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBitmapDimensionEx(hbit: HBITMAP, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetBkColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetBkMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBoundsRect(hdc: HDC, lprect: *mut super::super::Foundation::RECT, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBrushOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsA(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, pabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsW(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32A(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32W(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, piwidths: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetCharacterPlacementA(hdc: HDC, lpstring: ::windows_sys::core::PCSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSA, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetCharacterPlacementW(hdc: HDC, lpstring: ::windows_sys::core::PCWSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSW, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipBox(hdc: HDC, lprect: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorAdjustment(hdc: HDC, lpca: *mut COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetCurrentObject(hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPositionEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDC(hwnd: super::super::Foundation::HWND) -> HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetDCBrushColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCEx(hwnd: super::super::Foundation::HWND, hrgnclip: HRGN, flags: GET_DCX_FLAGS) -> HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetDCPenColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *mut RGBQUAD) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpvbits: *mut ::core::ffi::c_void, lpbmi: *mut BITMAPINFO, usage: DIB_USAGE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetDeviceCaps(hdc: HDC, index: GET_DEVICE_CAPS_INDEX) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetEnhMetaFileA(lpname: ::windows_sys::core::PCSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetEnhMetaFileBits(hemf: HENHMETAFILE, nsize: u32, lpdata: *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetEnhMetaFileDescriptionA(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetEnhMetaFileDescriptionW(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileHeader(hemf: HENHMETAFILE, nsize: u32, lpenhmetaheader: *mut ENHMETAHEADER) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetEnhMetaFilePaletteEntries(hemf: HENHMETAFILE, nnumentries: u32, lppaletteentries: *mut PALETTEENTRY) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetEnhMetaFileW(lpname: ::windows_sys::core::PCWSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetFontData(hdc: HDC, dwtable: u32, dwoffset: u32, pvbuffer: *mut ::core::ffi::c_void, cjbuffer: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetFontLanguageInfo(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetFontUnicodeRanges(hdc: HDC, lpgs: *mut GLYPHSET) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetGlyphIndicesA(hdc: HDC, lpstr: ::windows_sys::core::PCSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetGlyphIndicesW(hdc: HDC, lpstr: ::windows_sys::core::PCWSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineA(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineW(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetGraphicsMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetKerningPairsA(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetKerningPairsW(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetLayout(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetMapMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetMetaFileA(lpname: ::windows_sys::core::PCSTR) -> HMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetMetaFileBitsEx(hmf: HMETAFILE, cbbuffer: u32, lpdata: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetMetaFileW(lpname: ::windows_sys::core::PCWSTR) -> HMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetMetaRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMiterLimit(hdc: HDC, plimit: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoA(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetNearestColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetNearestPaletteIndex(h: HPALETTE, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetObjectA(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetObjectType(h: HGDIOBJ) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetObjectW(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsA(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICA) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsW(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICW) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPath(hdc: HDC, apt: *mut super::super::Foundation::POINT, aj: *mut u8, cpt: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetPixel(hdc: HDC, x: i32, y: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetPolyFillMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetROP2(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetRandomRgn(hdc: HDC, hrgn: HRGN, i: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRasterizerCaps(lpraststat: *mut RASTERIZER_STATUS, cjbytes: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegionData(hrgn: HRGN, ncount: u32, lprgndata: *mut RGNDATA) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRgnBox(hrgn: HRGN, lprc: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetStretchBltMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetSysColorBrush(nindex: i32) -> HBRUSH;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetSystemPaletteEntries(hdc: HDC, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetSystemPaletteUse(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTabbedTextExtentA(hdc: HDC, lpstring: ::windows_sys::core::PCSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTabbedTextExtentW(hdc: HDC, lpstring: ::windows_sys::core::PCWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTextAlign(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTextCharacterExtra(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTextColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointA(hdc: HDC, lpszstring: ::windows_sys::core::PCSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointI(hdc: HDC, lpwszstring: *const u16, cwchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointW(hdc: HDC, lpszstring: ::windows_sys::core::PCWSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32A(hdc: HDC, lpstring: ::windows_sys::core::PCSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32W(hdc: HDC, lpstring: ::windows_sys::core::PCWSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointA(hdc: HDC, lpstring: ::windows_sys::core::PCSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointI(hdc: HDC, pgiin: *const u16, cgi: i32, psize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointW(hdc: HDC, lpstring: ::windows_sys::core::PCWSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTextFaceA(hdc: HDC, c: i32, lpname: ::windows_sys::core::PSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetTextFaceW(hdc: HDC, c: i32, lpname: ::windows_sys::core::PWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsA(hdc: HDC, lptm: *mut TEXTMETRICA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsW(hdc: HDC, lptm: *mut TEXTMETRICW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn GetWinMetaFileBits(hemf: HENHMETAFILE, cbdata16: u32, pdata16: *mut u8, imapmode: i32, hdcref: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDC(hwnd: super::super::Foundation::HWND) -> HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgnBox(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWorldTransform(hdc: HDC, lpxf: *mut XFORM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, nmesh: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringA(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringW(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InflateRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn IntersectClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntersectRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsRectEmpty(lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LPtoDP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineDDA(xstart: i32, ystart: i32, xend: i32, yend: i32, lpproc: LINEDDAPROC, data: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineTo(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapA(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: ::windows_sys::core::PCSTR) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapW(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: ::windows_sys::core::PCWSTR) -> HBITMAP;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockWindowUpdate(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapWindowPoints(hwndfrom: super::super::Foundation::HWND, hwndto: super::super::Foundation::HWND, lppoints: *mut super::super::Foundation::POINT, cpoints: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MaskBlt(hdcdest: HDC, xdest: i32, ydest: i32, width: i32, height: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32, rop: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn MergeFontPackage(puchmergefontbuffer: *const u8, ulmergefontbuffersize: u32, puchfontpackagebuffer: *const u8, ulfontpackagebuffersize: u32, ppuchdestbuffer: *mut *mut u8, puldestbuffersize: *mut u32, pulbyteswritten: *mut u32, usmode: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyWorldTransform(hdc: HDC, lpxf: *const XFORM, mode: MODIFY_WORLD_TRANSFORM_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromPoint(pt: super::super::Foundation::POINT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromRect(lprc: *const super::super::Foundation::RECT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromWindow(hwnd: super::super::Foundation::HWND, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveToEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn OffsetClipRgn(hdc: HDC, x: i32, y: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn OffsetRgn(hrgn: HRGN, x: i32, y: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintDesktop(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn PathToRegion(hdc: HDC) -> HRGN;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Pie(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFileRecord(hdc: HDC, pht: *const HANDLETABLE, pmr: *const ENHMETARECORD, cht: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFile(hdc: HDC, hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFileRecord(hdc: HDC, lphandletable: *const HANDLETABLE, lpmr: *const METARECORD, noobjs: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlgBlt(hdcdest: HDC, lppoint: *const super::super::Foundation::POINT, hdcsrc: HDC, xsrc: i32, ysrc: i32, width: i32, height: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezier(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezierTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyDraw(hdc: HDC, apt: *const super::super::Foundation::POINT, aj: *const u8, cpt: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolygon(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const i32, csz: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolyline(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const u32, csz: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutA(hdc: HDC, ppt: *const POLYTEXTA, nstrings: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutW(hdc: HDC, ppt: *const POLYTEXTW, nstrings: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polygon(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polyline(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolylineTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRect(lprc: *const super::super::Foundation::RECT, pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRegion(hrgn: HRGN, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtVisible(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn RealizePalette(hdc: HDC) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectInRegion(hrgn: HRGN, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectVisible(hdc: HDC, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Rectangle(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RedrawWindow(hwnd: super::super::Foundation::HWND, lprcupdate: *const super::super::Foundation::RECT, hrgnupdate: HRGN, flags: REDRAW_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseDC(hwnd: super::super::Foundation::HWND, hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontMemResourceEx(h: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceA(lpfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExA(name: ::windows_sys::core::PCSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExW(name: ::windows_sys::core::PCWSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceW(lpfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCA(hdc: HDC, lpdm: *const DEVMODEA) -> HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCW(hdc: HDC, lpdm: *const DEVMODEW) -> HDC;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResizePalette(hpal: HPALETTE, n: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreDC(hdc: HDC, nsaveddc: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoundRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SaveDC(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleViewportExtEx(hdc: HDC, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleWindowExtEx(hdc: HDC, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScreenToClient(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectClipPath(hdc: HDC, mode: RGN_COMBINE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SelectClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectPalette(hdc: HDC, hpal: HPALETTE, bforcebkgd: super::super::Foundation::BOOL) -> HPALETTE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetArcDirection(hdc: HDC, dir: ARC_DIRECTION) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetBitmapBits(hbm: HBITMAP, cb: u32, pvbits: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBitmapDimensionEx(hbm: HBITMAP, w: i32, h: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetBkColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetBkMode(hdc: HDC, mode: BACKGROUND_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBoundsRect(hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: SET_BOUNDS_RECT_FLAGS) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorAdjustment(hdc: HDC, lpca: *const COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetDCBrushColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetDCPenColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *const RGBQUAD) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetDIBitsToDevice(hdc: HDC, xdest: i32, ydest: i32, w: u32, h: u32, xsrc: i32, ysrc: i32, startscan: u32, clines: u32, lpvbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetEnhMetaFileBits(nsize: u32, pb: *const u8) -> HENHMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetGraphicsMode(hdc: HDC, imode: GRAPHICS_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetLayout(hdc: HDC, l: DC_LAYOUT) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetMapMode(hdc: HDC, imode: HDC_MAP_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetMapperFlags(hdc: HDC, flags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetMetaFileBitsEx(cbbuffer: u32, lpdata: *const u8) -> HMETAFILE;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetMetaRgn(hdc: HDC) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMiterLimit(hdc: HDC, limit: f32, old: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *const PALETTEENTRY) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetPixel(hdc: HDC, x: i32, y: i32, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPixelV(hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetPolyFillMode(hdc: HDC, mode: CREATE_POLYGON_RGN_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetROP2(hdc: HDC, rop2: R2_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRect(lprc: *mut super::super::Foundation::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectEmpty(lprc: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectRgn(hrgn: HRGN, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetStretchBltMode(hdc: HDC, mode: STRETCH_BLT_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetSystemPaletteUse(hdc: HDC, r#use: SYSTEM_PALETTE_USE) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetTextAlign(hdc: HDC, align: TEXT_ALIGN_OPTIONS) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetTextCharacterExtra(hdc: HDC, extra: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn SetTextColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextJustification(hdc: HDC, extra: i32, count: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, bredraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWorldTransform(hdc: HDC, lpxf: *const XFORM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StretchBlt(hdcdest: HDC, xdest: i32, ydest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, wsrc: i32, hsrc: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn StretchDIBits(hdc: HDC, xdest: i32, ydest: i32, destwidth: i32, destheight: i32, xsrc: i32, ysrc: i32, srcwidth: i32, srcheight: i32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, iusage: DIB_USAGE, rop: ROP_CODE) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokeAndFillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokePath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SubtractRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTCharToUnicode(hdc: HDC, puccharcodes: *const u8, ulcharcodesize: u32, pusshortcodes: *mut u16, ulshortcodesize: u32, ulflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTDeleteEmbeddedFont(hfontreference: super::super::Foundation::HANDLE, ulflags: u32, pulstatus: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTEmbedFont(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTEmbedFontEx(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, pulcharcodeset: *const u32, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTEmbedFontFromFileA(hdc: HDC, szfontfilename: ::windows_sys::core::PCSTR, usttcindex: u16, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEnableEmbeddingForFacename(lpszfacename: ::windows_sys::core::PCSTR, benable: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTGetEmbeddedFontInfo(ulflags: TTEMBED_FLAGS, pulprivstatus: *mut u32, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut u32, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const ::core::ffi::c_void, pttloadinfo: *const TTLOADINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTGetEmbeddingType(hdc: HDC, pulembedtype: *mut EMBEDDED_FONT_PRIV_STATUS) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTGetNewFontName(phfontreference: *const super::super::Foundation::HANDLE, wzwinfamilyname: ::windows_sys::core::PWSTR, cchmaxwinname: i32, szmacfamilyname: ::windows_sys::core::PSTR, cchmaxmacname: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabled(hdc: HDC, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabledForFacename(lpszfacename: ::windows_sys::core::PCSTR, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTLoadEmbeddedFont(phfontreference: *mut super::super::Foundation::HANDLE, ulflags: u32, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const ::core::ffi::c_void, szwinfamilyname: ::windows_sys::core::PCWSTR, szmacfamilyname: ::windows_sys::core::PCSTR, pttloadinfo: *const TTLOADINFO) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTRunValidationTests(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMS) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TTRunValidationTestsEx(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMSEX) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TabbedTextOutA(hdc: HDC, x: i32, y: i32, lpstring: ::windows_sys::core::PCSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn TabbedTextOutW(hdc: HDC, x: i32, y: i32, lpstring: ::windows_sys::core::PCWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutA(hdc: HDC, x: i32, y: i32, lpstring: ::windows_sys::core::PCSTR, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutW(hdc: HDC, x: i32, y: i32, lpstring: ::windows_sys::core::PCWSTR, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnionRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnrealizeObject(h: HGDIOBJ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateColors(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WidenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromDC(hdc: HDC) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    pub fn wglSwapMultipleBuffers(param0: u32, param1: *const WGLSWAP) -> u32;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ABC {
    pub abcA: i32,
    pub abcB: u32,
    pub abcC: i32,
}
impl ::core::marker::Copy for ABC {}
impl ::core::clone::Clone for ABC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ABCFLOAT {
    pub abcfA: f32,
    pub abcfB: f32,
    pub abcfC: f32,
}
impl ::core::marker::Copy for ABCFLOAT {}
impl ::core::clone::Clone for ABCFLOAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ABORTDOC: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ABORTPATH {
    pub emr: EMR,
}
impl ::core::marker::Copy for ABORTPATH {}
impl ::core::clone::Clone for ABORTPATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ABSOLUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AC_SRC_ALPHA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AC_SRC_OVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANSI_CHARSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ARABIC_CHARSET: u32 = 178u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type ARC_DIRECTION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AD_COUNTERCLOCKWISE: ARC_DIRECTION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const AD_CLOCKWISE: ARC_DIRECTION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECT_FILTERING: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXESLISTA {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOA; 16],
}
impl ::core::marker::Copy for AXESLISTA {}
impl ::core::clone::Clone for AXESLISTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXESLISTW {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOW; 16],
}
impl ::core::marker::Copy for AXESLISTW {}
impl ::core::clone::Clone for AXESLISTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXISINFOA {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
impl ::core::marker::Copy for AXISINFOA {}
impl ::core::clone::Clone for AXISINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct AXISINFOW {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u16; 16],
}
impl ::core::marker::Copy for AXISINFOW {}
impl ::core::clone::Clone for AXISINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type BACKGROUND_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OPAQUE: BACKGROUND_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TRANSPARENT: BACKGROUND_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BALTIC_CHARSET: u32 = 186u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BANDINFO: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BEGIN_PATH: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for BITMAP {}
impl ::core::clone::Clone for BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPCOREHEADER {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
impl ::core::marker::Copy for BITMAPCOREHEADER {}
impl ::core::clone::Clone for BITMAPCOREHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPCOREINFO {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
impl ::core::marker::Copy for BITMAPCOREINFO {}
impl ::core::clone::Clone for BITMAPCOREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPFILEHEADER {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}
impl ::core::marker::Copy for BITMAPFILEHEADER {}
impl ::core::clone::Clone for BITMAPFILEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
impl ::core::marker::Copy for BITMAPINFO {}
impl ::core::clone::Clone for BITMAPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl ::core::marker::Copy for BITMAPINFOHEADER {}
impl ::core::clone::Clone for BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPV4HEADER {
    pub bV4Size: u32,
    pub bV4Width: i32,
    pub bV4Height: i32,
    pub bV4Planes: u16,
    pub bV4BitCount: u16,
    pub bV4V4Compression: u32,
    pub bV4SizeImage: u32,
    pub bV4XPelsPerMeter: i32,
    pub bV4YPelsPerMeter: i32,
    pub bV4ClrUsed: u32,
    pub bV4ClrImportant: u32,
    pub bV4RedMask: u32,
    pub bV4GreenMask: u32,
    pub bV4BlueMask: u32,
    pub bV4AlphaMask: u32,
    pub bV4CSType: u32,
    pub bV4Endpoints: CIEXYZTRIPLE,
    pub bV4GammaRed: u32,
    pub bV4GammaGreen: u32,
    pub bV4GammaBlue: u32,
}
impl ::core::marker::Copy for BITMAPV4HEADER {}
impl ::core::clone::Clone for BITMAPV4HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BITMAPV5HEADER {
    pub bV5Size: u32,
    pub bV5Width: i32,
    pub bV5Height: i32,
    pub bV5Planes: u16,
    pub bV5BitCount: u16,
    pub bV5Compression: u32,
    pub bV5SizeImage: u32,
    pub bV5XPelsPerMeter: i32,
    pub bV5YPelsPerMeter: i32,
    pub bV5ClrUsed: u32,
    pub bV5ClrImportant: u32,
    pub bV5RedMask: u32,
    pub bV5GreenMask: u32,
    pub bV5BlueMask: u32,
    pub bV5AlphaMask: u32,
    pub bV5CSType: u32,
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: u32,
    pub bV5GammaGreen: u32,
    pub bV5GammaBlue: u32,
    pub bV5Intent: u32,
    pub bV5ProfileData: u32,
    pub bV5ProfileSize: u32,
    pub bV5Reserved: u32,
}
impl ::core::marker::Copy for BITMAPV5HEADER {}
impl ::core::clone::Clone for BITMAPV5HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_BITFIELDS: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_JPEG: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_PNG: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_RGB: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_RLE4: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BI_RLE8: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BKMODE_LAST: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct BLENDFUNCTION {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
}
impl ::core::marker::Copy for BLENDFUNCTION {}
impl ::core::clone::Clone for BLENDFUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_DIBPATTERN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_DIBPATTERN8X8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_DIBPATTERNPT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_HATCHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_HOLLOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_INDEXED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_MONOPATTERN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_NULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_PATTERN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_PATTERN8X8: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BS_SOLID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CA_LOG_FILTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CA_NEGATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CBM_INIT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CCHFORMNAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_CHORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_CIRCLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_ELLIPSES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_INTERIORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_PIE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_ROUNDRECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_STYLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_WIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CC_WIDESTYLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CDS_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_FULLSCREEN: CDS_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_GLOBAL: CDS_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_NORESET: CDS_TYPE = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_RESET: CDS_TYPE = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_SET_PRIMARY: CDS_TYPE = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_TEST: CDS_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_UPDATEREGISTRY: CDS_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_VIDEOPARAMETERS: CDS_TYPE = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_ENABLE_UNSAFE_MODES: CDS_TYPE = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_DISABLE_UNSAFE_MODES: CDS_TYPE = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CDS_RESET_EX: CDS_TYPE = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CFP_ALLOCPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CFP_FREEPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CFP_REALLOCPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_GLYPHIDX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHECKJPEGFORMAT: u32 = 4119u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHECKPNGFORMAT: u32 = 4120u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHINESEBIG5_CHARSET: u32 = 136u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct CIEXYZ {
    pub ciexyzX: i32,
    pub ciexyzY: i32,
    pub ciexyzZ: i32,
}
impl ::core::marker::Copy for CIEXYZ {}
impl ::core::clone::Clone for CIEXYZ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
impl ::core::marker::Copy for CIEXYZTRIPLE {}
impl ::core::clone::Clone for CIEXYZTRIPLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLEARTYPE_NATURAL_QUALITY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_TO_PATH: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLOSECHANNEL: u32 = 4112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLR_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_CMYK_COLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_DEVICE_ICM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_GAMMA_RAMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_IN_GAMUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CM_OUT_OF_GAMUT: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct COLORADJUSTMENT {
    pub caSize: u16,
    pub caFlags: u16,
    pub caIlluminantIndex: u16,
    pub caRedGamma: u16,
    pub caGreenGamma: u16,
    pub caBlueGamma: u16,
    pub caReferenceBlack: u16,
    pub caReferenceWhite: u16,
    pub caContrast: i16,
    pub caBrightness: i16,
    pub caColorfulness: i16,
    pub caRedGreenTint: i16,
}
impl ::core::marker::Copy for COLORADJUSTMENT {}
impl ::core::clone::Clone for COLORADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct COLORCORRECTPALETTE {
    pub emr: EMR,
    pub ihPalette: u32,
    pub nFirstEntry: u32,
    pub nPalEntries: u32,
    pub nReserved: u32,
}
impl ::core::marker::Copy for COLORCORRECTPALETTE {}
impl ::core::clone::Clone for COLORCORRECTPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct COLORMATCHTOTARGET {
    pub emr: EMR,
    pub dwAction: u32,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for COLORMATCHTOTARGET {}
impl ::core::clone::Clone for COLORMATCHTOTARGET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORMATCHTOTARGET_EMBEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COMPLEXREGION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CP_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CP_RECTANGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CP_REGION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CREATECOLORSPACE_EMBEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CREATE_FONT_PACKAGE_SUBSET_ENCODING = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_STD_MAC_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_SYMBOL_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_UNICODE_CHAR_SET: CREATE_FONT_PACKAGE_SUBSET_ENCODING = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CREATE_FONT_PACKAGE_SUBSET_PLATFORM = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_UNICODE_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_ISO_PLATFORMID: CREATE_FONT_PACKAGE_SUBSET_PLATFORM = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type CREATE_POLYGON_RGN_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ALTERNATE: CREATE_POLYGON_RGN_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WINDING: CREATE_POLYGON_RGN_MODE = 2u32;
pub type CreatedHDC = isize;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNCENTER: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNLEFT: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNNONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEDOWNRIGHT: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPCENTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPLEFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPNONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCBA_FACEUPRIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_BITMAP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_DOWNLOAD: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_DOWNLOAD_OUTLINE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCTT_SUBDEV: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_BINADJUST: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_DATATYPE_PRODUCED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_EMF_COMPLIANT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DC_LAYOUT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: DC_LAYOUT = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_RTL: DC_LAYOUT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_MANUFACTURER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_MODEL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_CHARSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_PITCH: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct DESIGNVECTOR {
    pub dvReserved: u32,
    pub dvNumAxes: u32,
    pub dvValues: [i32; 16],
}
impl ::core::marker::Copy for DESIGNVECTOR {}
impl ::core::clone::Clone for DESIGNVECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEVICEDATA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEVICE_FONTTYPE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous1: DEVMODEA_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [u8; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEA_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEA_0 {
    pub Anonymous1: DEVMODEA_0_0,
    pub Anonymous2: DEVMODEA_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEA_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW {
    pub dmDeviceName: [u16; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous1: DEVMODEW_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [u16; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEW_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEW_0 {
    pub Anonymous1: DEVMODEW_0_0,
    pub Anonymous2: DEVMODEW_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW_0_1 {
    pub dmPosition: super::super::Foundation::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DEVMODEW_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEVMODEW_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVMODEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DFCS_STATE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONCLOSE: DFCS_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONMIN: DFCS_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONMAX: DFCS_STATE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONRESTORE: DFCS_STATE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CAPTIONHELP: DFCS_STATE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUARROW: DFCS_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUCHECK: DFCS_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUBULLET: DFCS_STATE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MENUARROWRIGHT: DFCS_STATE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLUP: DFCS_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLDOWN: DFCS_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLLEFT: DFCS_STATE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLRIGHT: DFCS_STATE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLCOMBOBOX: DFCS_STATE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLSIZEGRIP: DFCS_STATE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_SCROLLSIZEGRIPRIGHT: DFCS_STATE = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONCHECK: DFCS_STATE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONRADIOIMAGE: DFCS_STATE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONRADIOMASK: DFCS_STATE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONRADIO: DFCS_STATE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTON3STATE: DFCS_STATE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_BUTTONPUSH: DFCS_STATE = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_INACTIVE: DFCS_STATE = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_PUSHED: DFCS_STATE = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_CHECKED: DFCS_STATE = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_TRANSPARENT: DFCS_STATE = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_HOT: DFCS_STATE = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_ADJUSTRECT: DFCS_STATE = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_FLAT: DFCS_STATE = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFCS_MONO: DFCS_STATE = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DFC_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_CAPTION: DFC_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_MENU: DFC_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_SCROLL: DFC_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_BUTTON: DFC_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DFC_POPUPMENU: DFC_TYPE = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIBSECTION {
    pub dsBm: BITMAP,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [u32; 3],
    pub dshSection: super::super::Foundation::HANDLE,
    pub dsOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIBSECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIBSECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DIB_USAGE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DIB_RGB_COLORS: DIB_USAGE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DIB_PAL_COLORS: DIB_USAGE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DISPLAYCONFIG_COLOR_ENCODING = i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_RGB: DISPLAYCONFIG_COLOR_ENCODING = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR444: DISPLAYCONFIG_COLOR_ENCODING = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR422: DISPLAYCONFIG_COLOR_ENCODING = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR420: DISPLAYCONFIG_COLOR_ENCODING = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_INTENSITY: DISPLAYCONFIG_COLOR_ENCODING = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32: DISPLAYCONFIG_COLOR_ENCODING = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_MAXPATH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_SOURCE_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_FORCIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAYCONFIG_TARGET_IS_HMD: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAY_DEVICEA {
    pub cb: u32,
    pub DeviceName: [super::super::Foundation::CHAR; 32],
    pub DeviceString: [super::super::Foundation::CHAR; 128],
    pub StateFlags: u32,
    pub DeviceID: [super::super::Foundation::CHAR; 128],
    pub DeviceKey: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISPLAY_DEVICEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISPLAY_DEVICEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct DISPLAY_DEVICEW {
    pub cb: u32,
    pub DeviceName: [u16; 32],
    pub DeviceString: [u16; 128],
    pub StateFlags: u32,
    pub DeviceID: [u16; 128],
    pub DeviceKey: [u16; 128],
}
impl ::core::marker::Copy for DISPLAY_DEVICEW {}
impl ::core::clone::Clone for DISPLAY_DEVICEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ACC_DRIVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ATTACHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_DISCONNECT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_MODESPRUNED: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_MULTI_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_RDPUDD: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_REMOTE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_REMOVABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_TS_COMPATIBLE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DISP_CHANGE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_SUCCESSFUL: DISP_CHANGE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_RESTART: DISP_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_FAILED: DISP_CHANGE = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADMODE: DISP_CHANGE = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_NOTUPDATED: DISP_CHANGE = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADFLAGS: DISP_CHANGE = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADPARAM: DISP_CHANGE = -5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DISP_CHANGE_BADDUALVIEW: DISP_CHANGE = -6i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DI_APPBANDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DI_ROPS_READ_DESTINATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_AUTO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_CASSETTE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_ENVELOPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_ENVMANUAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_FORMSOURCE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LARGECAPACITY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LARGEFMT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LAST: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_LOWER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_MANUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_MIDDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_ONLYONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_SMALLFMT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_TRACTOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_UPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMBIN_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLLATE_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLLATE_TRUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLOR_COLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMCOLOR_MONOCHROME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDFO_CENTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDFO_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDFO_STRETCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDISPLAYFLAGS_TEXTMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_COARSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_ERRORDIFFUSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_FINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_GRAYSCALE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_LINEART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_RESERVED9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDITHER_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_180: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_270: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_90: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDO_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDUP_HORIZONTAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDUP_SIMPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMDUP_VERTICAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_DEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_DRIVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_SYSTEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICMMETHOD_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_ABS_COLORIMETRIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_COLORIMETRIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_CONTRAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_SATURATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMICM_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_GLOSSY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_TRANSPARENCY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMMEDIA_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMNUP_ONEUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMNUP_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMORIENT_LANDSCAPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMORIENT_PORTRAIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_10X11: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_10X14: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_11X17: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_12X11: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_15X11: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_9X11: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A2: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_EXTRA: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_EXTRA_TRANSVERSE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_ROTATED: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A3_TRANSVERSE: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4SMALL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_EXTRA: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_PLUS: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_ROTATED: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A4_TRANSVERSE: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5_EXTRA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5_ROTATED: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A5_TRANSVERSE: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A6: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A6_ROTATED: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_A_PLUS: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B4: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B4_JIS_ROTATED: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5_EXTRA: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5_JIS_ROTATED: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B5_TRANSVERSE: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B6_JIS: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B6_JIS_ROTATED: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_B_PLUS: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_CSHEET: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_DBL_JAPANESE_POSTCARD: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_DSHEET: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_10: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_11: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_12: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_14: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_9: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_B4: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_B5: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_B6: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C3: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C4: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C5: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C6: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_C65: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_DL: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_INVITE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_ITALY: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_MONARCH: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ENV_PERSONAL: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ESHEET: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_EXECUTIVE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FANFOLD_LGL_GERMAN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FANFOLD_STD_GERMAN: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FANFOLD_US: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_FOLIO: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_ISO_B4: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JAPANESE_POSTCARD: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU3: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU3_ROTATED: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU4: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_CHOU4_ROTATED: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU2: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU2_ROTATED: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU3: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_KAKU3_ROTATED: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_YOU4: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_JENV_YOU4_ROTATED: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LAST: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LEDGER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LEGAL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LEGAL_EXTRA: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTERSMALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_EXTRA: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_PLUS: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_ROTATED: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_LETTER_TRANSVERSE: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_NOTE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P16K: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P16K_ROTATED: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32K: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32KBIG: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32KBIG_ROTATED: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_P32K_ROTATED: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_1: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_10: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_10_ROTATED: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_1_ROTATED: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_2: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_2_ROTATED: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_3: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_3_ROTATED: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_4: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_4_ROTATED: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_5: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_5_ROTATED: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_6: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_6_ROTATED: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_7: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_7_ROTATED: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_8: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_8_ROTATED: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_9: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_PENV_9_ROTATED: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_QUARTO: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_RESERVED_48: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_RESERVED_49: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_STATEMENT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_TABLOID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_TABLOID_EXTRA: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMPAPER_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_DRAFT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_HIGH: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_LOW: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMRES_MEDIUM: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_BITMAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_DOWNLOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_DOWNLOAD_OUTLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DMTT_SUBDEV: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_BITSPERPEL: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COLLATE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COLOR: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_COPIES: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DEFAULTSOURCE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYFIXEDOUTPUT: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYFLAGS: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYFREQUENCY: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DISPLAYORIENTATION: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DITHERTYPE: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_DUPLEX: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_FORMNAME: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_ICMINTENT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_ICMMETHOD: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_INTERLACED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_LOGPIXELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_MEDIATYPE: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_NUP: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_ORIENTATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PANNINGHEIGHT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PANNINGWIDTH: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PAPERLENGTH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PAPERSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PAPERWIDTH: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PELSHEIGHT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PELSWIDTH: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_POSITION: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_PRINTQUALITY: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_SCALE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_SPECVERSION: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_TTOPTION: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DM_YRESOLUTION: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DOWNLOADFACE: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DOWNLOADHEADER: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRAFTMODE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DRAWEDGE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_RAISEDOUTER: DRAWEDGE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_SUNKENOUTER: DRAWEDGE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_RAISEDINNER: DRAWEDGE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_SUNKENINNER: DRAWEDGE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_OUTER: DRAWEDGE_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_INNER: DRAWEDGE_FLAGS = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_RAISED: DRAWEDGE_FLAGS = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BDR_SUNKEN: DRAWEDGE_FLAGS = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_RAISED: DRAWEDGE_FLAGS = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_SUNKEN: DRAWEDGE_FLAGS = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_ETCHED: DRAWEDGE_FLAGS = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EDGE_BUMP: DRAWEDGE_FLAGS = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRAWPATTERNRECT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DRAWSTATEPROC = ::core::option::Option<unsafe extern "system" fn(hdc: HDC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, cx: i32, cy: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DRAWSTATE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_COMPLEX: DRAWSTATE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_TEXT: DRAWSTATE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_PREFIXTEXT: DRAWSTATE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_ICON: DRAWSTATE_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DST_BITMAP: DRAWSTATE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_NORMAL: DRAWSTATE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_UNION: DRAWSTATE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_DISABLED: DRAWSTATE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_MONO: DRAWSTATE_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_HIDEPREFIX: DRAWSTATE_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_PREFIXONLY: DRAWSTATE_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSS_RIGHT: DRAWSTATE_FLAGS = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct DRAWTEXTPARAMS {
    pub cbSize: u32,
    pub iTabLength: i32,
    pub iLeftMargin: i32,
    pub iRightMargin: i32,
    pub uiLengthDrawn: u32,
}
impl ::core::marker::Copy for DRAWTEXTPARAMS {}
impl ::core::clone::Clone for DRAWTEXTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DRAW_CAPTION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_ACTIVE: DRAW_CAPTION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_BUTTONS: DRAW_CAPTION_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_GRADIENT: DRAW_CAPTION_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_ICON: DRAW_CAPTION_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_INBUTTON: DRAW_CAPTION_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_SMALLCAP: DRAW_CAPTION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_TEXT: DRAW_CAPTION_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DRAW_EDGE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_ADJUST: DRAW_EDGE_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_BOTTOM: DRAW_EDGE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_BOTTOMLEFT: DRAW_EDGE_FLAGS = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_BOTTOMRIGHT: DRAW_EDGE_FLAGS = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL: DRAW_EDGE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDBOTTOMLEFT: DRAW_EDGE_FLAGS = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: DRAW_EDGE_FLAGS = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDTOPLEFT: DRAW_EDGE_FLAGS = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_DIAGONAL_ENDTOPRIGHT: DRAW_EDGE_FLAGS = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_FLAT: DRAW_EDGE_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_LEFT: DRAW_EDGE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_MIDDLE: DRAW_EDGE_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_MONO: DRAW_EDGE_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_RECT: DRAW_EDGE_FLAGS = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_RIGHT: DRAW_EDGE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_SOFT: DRAW_EDGE_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_TOP: DRAW_EDGE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_TOPLEFT: DRAW_EDGE_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BF_TOPRIGHT: DRAW_EDGE_FLAGS = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type DRAW_TEXT_FORMAT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_BOTTOM: DRAW_TEXT_FORMAT = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_CALCRECT: DRAW_TEXT_FORMAT = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_CENTER: DRAW_TEXT_FORMAT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_EDITCONTROL: DRAW_TEXT_FORMAT = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_END_ELLIPSIS: DRAW_TEXT_FORMAT = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_EXPANDTABS: DRAW_TEXT_FORMAT = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_EXTERNALLEADING: DRAW_TEXT_FORMAT = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_HIDEPREFIX: DRAW_TEXT_FORMAT = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_INTERNAL: DRAW_TEXT_FORMAT = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_LEFT: DRAW_TEXT_FORMAT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_MODIFYSTRING: DRAW_TEXT_FORMAT = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_NOCLIP: DRAW_TEXT_FORMAT = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_NOFULLWIDTHCHARBREAK: DRAW_TEXT_FORMAT = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_NOPREFIX: DRAW_TEXT_FORMAT = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_PATH_ELLIPSIS: DRAW_TEXT_FORMAT = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_PREFIXONLY: DRAW_TEXT_FORMAT = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RIGHT: DRAW_TEXT_FORMAT = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RTLREADING: DRAW_TEXT_FORMAT = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_SINGLELINE: DRAW_TEXT_FORMAT = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_TABSTOP: DRAW_TEXT_FORMAT = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_TOP: DRAW_TEXT_FORMAT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_VCENTER: DRAW_TEXT_FORMAT = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_WORDBREAK: DRAW_TEXT_FORMAT = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_WORD_ELLIPSIS: DRAW_TEXT_FORMAT = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_CHARSTREAM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_DISPFILE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_METAFILE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_PLOTTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RASCAMERA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RASDISPLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DT_RASPRINTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EASTEUROPE_CHARSET: u32 = 238u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ELF_CULTURE_LATIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ELF_VENDOR_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ELF_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type EMBEDDED_FONT_PRIV_STATUS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_PREVIEWPRINT: EMBEDDED_FONT_PRIV_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_EDITABLE: EMBEDDED_FONT_PRIV_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_INSTALLABLE: EMBEDDED_FONT_PRIV_STATUS = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMBED_NOEMBEDDING: EMBEDDED_FONT_PRIV_STATUS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type EMBED_FONT_CHARSET = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_UNICODE: EMBED_FONT_CHARSET = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CHARSET_SYMBOL: EMBED_FONT_CHARSET = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMR {
    pub iType: u32,
    pub nSize: u32,
}
impl ::core::marker::Copy for EMR {}
impl ::core::clone::Clone for EMR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRALPHABLEND {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRALPHABLEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRALPHABLEND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRANGLEARC {
    pub emr: EMR,
    pub ptlCenter: super::super::Foundation::POINTL,
    pub nRadius: u32,
    pub eStartAngle: f32,
    pub eSweepAngle: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRANGLEARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRANGLEARC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRARC {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub ptlStart: super::super::Foundation::POINTL,
    pub ptlEnd: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRARC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRBITBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRBITBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRBITBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEBRUSHINDIRECT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub lb: LOGBRUSH32,
}
impl ::core::marker::Copy for EMRCREATEBRUSHINDIRECT {}
impl ::core::clone::Clone for EMRCREATEBRUSHINDIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEDIBPATTERNBRUSHPT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
impl ::core::marker::Copy for EMRCREATEDIBPATTERNBRUSHPT {}
impl ::core::clone::Clone for EMRCREATEDIBPATTERNBRUSHPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEMONOBRUSH {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
impl ::core::marker::Copy for EMRCREATEMONOBRUSH {}
impl ::core::clone::Clone for EMRCREATEMONOBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRCREATEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub lgpl: LOGPALETTE,
}
impl ::core::marker::Copy for EMRCREATEPALETTE {}
impl ::core::clone::Clone for EMRCREATEPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub lopn: LOGPEN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRCREATEPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRELLIPSE {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRELLIPSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRELLIPSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREOF {
    pub emr: EMR,
    pub nPalEntries: u32,
    pub offPalEntries: u32,
    pub nSizeLast: u32,
}
impl ::core::marker::Copy for EMREOF {}
impl ::core::clone::Clone for EMREOF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXCLUDECLIPRECT {
    pub emr: EMR,
    pub rclClip: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXCLUDECLIPRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXCLUDECLIPRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTCREATEFONTINDIRECTW {
    pub emr: EMR,
    pub ihFont: u32,
    pub elfw: EXTLOGFONTW,
}
impl ::core::marker::Copy for EMREXTCREATEFONTINDIRECTW {}
impl ::core::clone::Clone for EMREXTCREATEFONTINDIRECTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
    pub elp: EXTLOGPEN32,
}
impl ::core::marker::Copy for EMREXTCREATEPEN {}
impl ::core::clone::Clone for EMREXTCREATEPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl ::core::marker::Copy for EMREXTESCAPE {}
impl ::core::clone::Clone for EMREXTESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTFLOODFILL {
    pub emr: EMR,
    pub ptlStart: super::super::Foundation::POINTL,
    pub crColor: u32,
    pub iMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXTFLOODFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXTFLOODFILL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMREXTSELECTCLIPRGN {
    pub emr: EMR,
    pub cbRgnData: u32,
    pub iMode: u32,
    pub RgnData: [u8; 1],
}
impl ::core::marker::Copy for EMREXTSELECTCLIPRGN {}
impl ::core::clone::Clone for EMREXTSELECTCLIPRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub emrtext: EMRTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMREXTTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMREXTTEXTOUTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFILLPATH {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRFILLPATH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRFILLPATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFILLRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRFILLRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRFILLRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRFORMAT {
    pub dSignature: u32,
    pub nVersion: u32,
    pub cbData: u32,
    pub offData: u32,
}
impl ::core::marker::Copy for EMRFORMAT {}
impl ::core::clone::Clone for EMRFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFRAMERGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub szlStroke: super::super::Foundation::SIZE,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRFRAMERGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRFRAMERGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRGDICOMMENT {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGDICOMMENT {}
impl ::core::clone::Clone for EMRGDICOMMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRGLSBOUNDEDRECORD {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbData: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRGLSBOUNDEDRECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRGLSBOUNDEDRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRGLSRECORD {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRGLSRECORD {}
impl ::core::clone::Clone for EMRGLSRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRGRADIENTFILL {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nVer: u32,
    pub nTri: u32,
    pub ulMode: GRADIENT_FILL,
    pub Ver: [TRIVERTEX; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRGRADIENTFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRGRADIENTFILL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRINVERTRGN {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cbRgnData: u32,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRINVERTRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRINVERTRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRLINETO {
    pub emr: EMR,
    pub ptl: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRLINETO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRLINETO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRMASKBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRMASKBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRMASKBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRMODIFYWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
    pub iMode: u32,
}
impl ::core::marker::Copy for EMRMODIFYWORLDTRANSFORM {}
impl ::core::clone::Clone for EMRMODIFYWORLDTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRNAMEDESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbDriver: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl ::core::marker::Copy for EMRNAMEDESCAPE {}
impl ::core::clone::Clone for EMRNAMEDESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMROFFSETCLIPRGN {
    pub emr: EMR,
    pub ptlOffset: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMROFFSETCLIPRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMROFFSETCLIPRGN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPLGBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub aptlDest: [super::super::Foundation::POINTL; 3],
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub xMask: i32,
    pub yMask: i32,
    pub iUsageMask: u32,
    pub offBmiMask: u32,
    pub cbBmiMask: u32,
    pub offBitsMask: u32,
    pub cbBitsMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPLGBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPLGBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYDRAW {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
    pub abTypes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYDRAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYDRAW16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
    pub abTypes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYDRAW16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYDRAW16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cptl: u32,
    pub aptl: [super::super::Foundation::POINTL; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub cpts: u32,
    pub apts: [super::super::Foundation::POINTS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYLINE16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cptl: u32,
    pub aPolyCounts: [u32; 1],
    pub aptl: [super::super::Foundation::POINTL; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYPOLYLINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub nPolys: u32,
    pub cpts: u32,
    pub aPolyCounts: [u32; 1],
    pub apts: [super::super::Foundation::POINTS; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYPOLYLINE16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub cStrings: i32,
    pub aemrtext: [EMRTEXT; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRPOLYTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRPOLYTEXTOUTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRRESIZEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub cEntries: u32,
}
impl ::core::marker::Copy for EMRRESIZEPALETTE {}
impl ::core::clone::Clone for EMRRESIZEPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRRESTOREDC {
    pub emr: EMR,
    pub iRelative: i32,
}
impl ::core::marker::Copy for EMRRESTOREDC {}
impl ::core::clone::Clone for EMRRESTOREDC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRROUNDRECT {
    pub emr: EMR,
    pub rclBox: super::super::Foundation::RECTL,
    pub szlCorner: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRROUNDRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRROUNDRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSCALEVIEWPORTEXTEX {
    pub emr: EMR,
    pub xNum: i32,
    pub xDenom: i32,
    pub yNum: i32,
    pub yDenom: i32,
}
impl ::core::marker::Copy for EMRSCALEVIEWPORTEXTEX {}
impl ::core::clone::Clone for EMRSCALEVIEWPORTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSELECTCLIPPATH {
    pub emr: EMR,
    pub iMode: u32,
}
impl ::core::marker::Copy for EMRSELECTCLIPPATH {}
impl ::core::clone::Clone for EMRSELECTCLIPPATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSELECTOBJECT {
    pub emr: EMR,
    pub ihObject: u32,
}
impl ::core::marker::Copy for EMRSELECTOBJECT {}
impl ::core::clone::Clone for EMRSELECTOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSELECTPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
}
impl ::core::marker::Copy for EMRSELECTPALETTE {}
impl ::core::clone::Clone for EMRSELECTPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETARCDIRECTION {
    pub emr: EMR,
    pub iArcDirection: u32,
}
impl ::core::marker::Copy for EMRSETARCDIRECTION {}
impl ::core::clone::Clone for EMRSETARCDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETCOLORADJUSTMENT {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
impl ::core::marker::Copy for EMRSETCOLORADJUSTMENT {}
impl ::core::clone::Clone for EMRSETCOLORADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETCOLORSPACE {
    pub emr: EMR,
    pub ihCS: u32,
}
impl ::core::marker::Copy for EMRSETCOLORSPACE {}
impl ::core::clone::Clone for EMRSETCOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETDIBITSTODEVICE {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub iStartScan: u32,
    pub cScans: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETDIBITSTODEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETDIBITSTODEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETICMPROFILE {
    pub emr: EMR,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EMRSETICMPROFILE {}
impl ::core::clone::Clone for EMRSETICMPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETMAPPERFLAGS {
    pub emr: EMR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for EMRSETMAPPERFLAGS {}
impl ::core::clone::Clone for EMRSETMAPPERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETMITERLIMIT {
    pub emr: EMR,
    pub eMiterLimit: f32,
}
impl ::core::marker::Copy for EMRSETMITERLIMIT {}
impl ::core::clone::Clone for EMRSETMITERLIMIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETPALETTEENTRIES {
    pub emr: EMR,
    pub ihPal: u32,
    pub iStart: u32,
    pub cEntries: u32,
    pub aPalEntries: [PALETTEENTRY; 1],
}
impl ::core::marker::Copy for EMRSETPALETTEENTRIES {}
impl ::core::clone::Clone for EMRSETPALETTEENTRIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETPIXELV {
    pub emr: EMR,
    pub ptlPixel: super::super::Foundation::POINTL,
    pub crColor: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETPIXELV {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETPIXELV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETTEXTCOLOR {
    pub emr: EMR,
    pub crColor: u32,
}
impl ::core::marker::Copy for EMRSETTEXTCOLOR {}
impl ::core::clone::Clone for EMRSETTEXTCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETVIEWPORTEXTEX {
    pub emr: EMR,
    pub szlExtent: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETVIEWPORTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETVIEWPORTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETVIEWPORTORGEX {
    pub emr: EMR,
    pub ptlOrigin: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSETVIEWPORTORGEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSETVIEWPORTORGEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EMRSETWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
}
impl ::core::marker::Copy for EMRSETWORLDTRANSFORM {}
impl ::core::clone::Clone for EMRSETWORLDTRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSTRETCHBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSTRETCHBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSTRETCHBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSTRETCHDIBITS {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub iUsageSrc: u32,
    pub dwRop: u32,
    pub cxDest: i32,
    pub cyDest: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRSTRETCHDIBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRSTRETCHDIBITS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRTEXT {
    pub ptlReference: super::super::Foundation::POINTL,
    pub nChars: u32,
    pub offString: u32,
    pub fOptions: u32,
    pub rcl: super::super::Foundation::RECTL,
    pub offDx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EMRTRANSPARENTBLT {
    pub emr: EMR,
    pub rclBounds: super::super::Foundation::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: u32,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EMRTRANSPARENTBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EMRTRANSPARENTBLT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ABORTPATH: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ALPHABLEND: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ANGLEARC: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ARC: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ARCTO: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_BEGINPATH: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_BITBLT: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CHORD: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CLOSEFIGURE: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_COLORCORRECTPALETTE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_COLORMATCHTOTARGETW: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEBRUSHINDIRECT: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATECOLORSPACE: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATECOLORSPACEW: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEDIBPATTERNBRUSHPT: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEMONOBRUSH: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEPALETTE: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_CREATEPEN: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_DELETECOLORSPACE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_DELETEOBJECT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ELLIPSE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ENDPATH: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EOF: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXCLUDECLIPRECT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTCREATEFONTINDIRECTW: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTCREATEPEN: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTFLOODFILL: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTSELECTCLIPRGN: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTTEXTOUTA: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_EXTTEXTOUTW: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FILLPATH: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FILLRGN: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FLATTENPATH: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_FRAMERGN: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GDICOMMENT: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GLSBOUNDEDRECORD: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GLSRECORD: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_GRADIENTFILL: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_HEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_INTERSECTCLIPRECT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_INVERTRGN: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_LINETO: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MASKBLT: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MAX: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MODIFYWORLDTRANSFORM: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_MOVETOEX: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_OFFSETCLIPRGN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PAINTRGN: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PIE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PIXELFORMAT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_PLGBLT: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIER16: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIERTO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYBEZIERTO16: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYDRAW: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYDRAW16: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYGON: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYGON16: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINE16: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINETO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYLINETO16: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYGON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYGON16: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYLINE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYPOLYLINE16: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYTEXTOUTA: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_POLYTEXTOUTW: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_REALIZEPALETTE: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RECTANGLE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_105: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_106: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_107: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_108: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_109: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_110: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_117: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_119: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESERVED_120: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESIZEPALETTE: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_RESTOREDC: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_ROUNDRECT: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SAVEDC: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SCALEVIEWPORTEXTEX: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SCALEWINDOWEXTEX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SELECTCLIPPATH: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SELECTOBJECT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SELECTPALETTE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETARCDIRECTION: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETBKCOLOR: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETBKMODE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETBRUSHORGEX: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETCOLORADJUSTMENT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETCOLORSPACE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETDIBITSTODEVICE: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETICMMODE: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETICMPROFILEA: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETICMPROFILEW: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETLAYOUT: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMAPMODE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMAPPERFLAGS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMETARGN: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETMITERLIMIT: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETPALETTEENTRIES: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETPIXELV: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETPOLYFILLMODE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETROP2: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETSTRETCHBLTMODE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETTEXTALIGN: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETTEXTCOLOR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETVIEWPORTEXTEX: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETVIEWPORTORGEX: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETWINDOWEXTEX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETWINDOWORGEX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_SETWORLDTRANSFORM: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STRETCHBLT: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STRETCHDIBITS: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STROKEANDFILLPATH: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_STROKEPATH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_TRANSPARENTBLT: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EMR_WIDENPATH: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENABLEDUPLEX: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENABLEPAIRKERNING: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENABLERELATIVEWIDTHS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENCAPSULATED_POSTSCRIPT: u32 = 4116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENDDOC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const END_PATH: u32 = 4098u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENHMETAHEADER {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub rclFrame: super::super::Foundation::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::super::Foundation::SIZE,
    pub szlMillimeters: super::super::Foundation::SIZE,
    pub cbPixelFormat: u32,
    pub offPixelFormat: u32,
    pub bOpenGL: u32,
    pub szlMicrometers: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENHMETAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENHMETAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENHMETARECORD {
    pub iType: u32,
    pub nSize: u32,
    pub dParm: [u32; 1],
}
impl ::core::marker::Copy for ENHMETARECORD {}
impl ::core::clone::Clone for ENHMETARECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENHMETA_SIGNATURE: u32 = 1179469088u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENHMFENUMPROC = ::core::option::Option<unsafe extern "system" fn(hdc: HDC, lpht: *const HANDLETABLE, lpmr: *const ENHMETARECORD, nhandles: i32, data: super::super::Foundation::LPARAM) -> i32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUMLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUMLOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUMLOGFONTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUMLOGFONTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUMLOGFONTEXDVA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUMLOGFONTEXDVA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
impl ::core::marker::Copy for ENUMLOGFONTEXDVW {}
impl ::core::clone::Clone for ENUMLOGFONTEXDVW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfScript: [u16; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTEXW {}
impl ::core::clone::Clone for ENUMLOGFONTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct ENUMLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
}
impl ::core::marker::Copy for ENUMLOGFONTW {}
impl ::core::clone::Clone for ENUMLOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUMPAPERBINS: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUMPAPERMETRICS: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type ENUM_DISPLAY_SETTINGS_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUM_CURRENT_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ENUM_REGISTRY_SETTINGS: ENUM_DISPLAY_SETTINGS_MODE = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EPSPRINTING: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EPS_SIGNATURE: u32 = 1179865157u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_FORMAT: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_GENERIC: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_BASE: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_CMAP: u32 = 1060u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_DELTA_FORMAT: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_EBLC: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GDEF: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GLYF: u32 = 1061u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GPOS: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_GSUB: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HDMX: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HEAD: u32 = 1062u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HHEA: u32 = 1063u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HHEA_OR_VHEA: u32 = 1072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HMTX: u32 = 1064u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_HMTX_OR_VMTX: u32 = 1073u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_JSTF: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_LOCA: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_LTSH: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MAXP: u32 = 1066u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MERGE_CHECKSUMS: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MERGE_FORMATS: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_MERGE_NUMGLYPHS: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_NAME: u32 = 1067u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_OS2: u32 = 1069u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_POST: u32 = 1068u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_TTC_INDEX: u32 = 1015u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_TTO: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_VDMX: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_VHEA: u32 = 1070u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_INVALID_VMTX: u32 = 1071u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MEM: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_CMAP: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_EBDT: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_GLYF: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HEAD: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HHEA: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HHEA_OR_VHEA: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HMTX: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_HMTX_OR_VMTX: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_LOCA: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_MAXP: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_NAME: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_OS2: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_POST: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_VHEA: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_MISSING_VMTX: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_NOT_TTC: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_NO_GLYPHS: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER0: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER1: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER10: u32 = 1110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER11: u32 = 1111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER12: u32 = 1112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER13: u32 = 1113u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER14: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER15: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER16: u32 = 1116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER2: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER3: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER4: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER5: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER6: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER7: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER8: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_PARAMETER9: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_READCONTROL: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_READOUTOFBOUNDS: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_VERSION: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_WOULD_GROW: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_WRITECONTROL: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ERR_WRITEOUTOFBOUNDS: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type ETO_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_OPAQUE: ETO_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_CLIPPED: ETO_OPTIONS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_GLYPH_INDEX: ETO_OPTIONS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_RTLREADING: ETO_OPTIONS = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_NUMERICSLOCAL: ETO_OPTIONS = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_NUMERICSLATIN: ETO_OPTIONS = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_IGNORELANGUAGE: ETO_OPTIONS = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_PDY: ETO_OPTIONS = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ETO_REVERSE_INDEX_MAP: ETO_OPTIONS = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXTLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXTLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXTLOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EXTLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfVersion: u32,
    pub elfStyleSize: u32,
    pub elfMatch: u32,
    pub elfReserved: u32,
    pub elfVendorId: [u8; 4],
    pub elfCulture: u32,
    pub elfPanose: PANOSE,
}
impl ::core::marker::Copy for EXTLOGFONTW {}
impl ::core::clone::Clone for EXTLOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EXTLOGPEN {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: u32,
    pub elpHatch: usize,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
impl ::core::marker::Copy for EXTLOGPEN {}
impl ::core::clone::Clone for EXTLOGPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct EXTLOGPEN32 {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: u32,
    pub elpHatch: u32,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
impl ::core::marker::Copy for EXTLOGPEN32 {}
impl ::core::clone::Clone for EXTLOGPEN32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EXTTEXTOUT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const EXT_DEVICE_CAPS: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type EXT_FLOOD_FILL_TYPE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLOODFILLBORDER: EXT_FLOOD_FILL_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLOODFILLSURFACE: EXT_FLOOD_FILL_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ADDFONTFAILED: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_API_NOTIMPL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_CHARCODECOUNTINVALID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_CHARCODESETINVALID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_CHARSETINVALID: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_COULDNTCREATETEMPFILE: i32 = 513i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_DEVICETRUETYPEFONT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORACCESSINGEXCLUDELIST: i32 = 274i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORACCESSINGFACENAME: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORACCESSINGFONTDATA: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORCOMPRESSINGFONTDATA: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORCONVERTINGCHARS: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORCREATINGFONTFILE: i32 = 269i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORDECOMPRESSINGFONTDATA: i32 = 273i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERROREXPANDINGFONTDATA: i32 = 519i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORGETTINGDC: i32 = 520i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORREADINGFONTDATA: i32 = 267i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_ERRORUNICODECONVERSION: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_EXCEPTION: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_EXCEPTIONINCOMPRESSION: i32 = 522i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_EXCEPTIONINDECOMPRESSION: i32 = 521i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FACENAMEINVALID: i32 = 275i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FILE_NOT_FOUND: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FLAGSINVALID: i32 = 268i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTALREADYEXISTS: i32 = 270i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTDATAINVALID: i32 = 258i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTFAMILYNAMENOTINFULL: i32 = 285i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTFILECREATEFAILED: i32 = 515i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTFILENOTFOUND: i32 = 517i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTINSTALLFAILED: i32 = 272i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTNAMEALREADYEXISTS: i32 = 271i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTNOTEMBEDDABLE: i32 = 260i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTREFERENCEINVALID: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_FONTVARIATIONSIMULATED: i32 = 283i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_HDCINVALID: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_INPUTPARAMINVALID: i32 = 25i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NAMECHANGEFAILED: i32 = 259i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NOFREEMEMORY: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NONE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NOOS2: i32 = 265i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_NOTATRUETYPEFONT: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PBENABLEDINVALID: i32 = 280i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PERMISSIONSINVALID: i32 = 279i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PRIVSINVALID: i32 = 261i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_PRIVSTATUSINVALID: i32 = 278i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_READFROMSTREAMFAILED: i32 = 263i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_RESERVEDPARAMNOTNULL: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_RESOURCEFILECREATEFAILED: i32 = 518i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SAVETOSTREAMFAILED: i32 = 264i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_STATUSINVALID: i32 = 277i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_STREAMINVALID: i32 = 276i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SUBSETTINGEXCEPTION: i32 = 281i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SUBSETTINGFAILED: i32 = 262i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_SUBSTRING_TEST_FAIL: i32 = 282i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_T2NOFREEMEMORY: i32 = 266i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_TTC_INDEX_OUT_OF_RANGE: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const E_WINDOWSAPI: i32 = 516i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_CUSTPAPER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_MIRROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_NEGATIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_NUP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_OUTPUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PRIVATE_BEGIN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PRIVATE_END: u32 = 8191u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PROTOCOL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FEATURESETTING_PSLEVEL: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct FIXED {
    pub fract: u16,
    pub value: i16,
}
impl ::core::marker::Copy for FIXED {}
impl ::core::clone::Clone for FIXED {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FIXED_PITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLI_GLYPHS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLI_MASK: u32 = 4155u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FLUSHOUTPUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FONTENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: u32, param3: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FONTENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: *const LOGFONTW, param1: *const TEXTMETRICW, param2: u32, param3: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FONTMAPPER_MAX: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type FONT_CLIP_PRECISION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_CHARACTER_PRECIS: FONT_CLIP_PRECISION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_DEFAULT_PRECIS: FONT_CLIP_PRECISION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_DFA_DISABLE: FONT_CLIP_PRECISION = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_EMBEDDED: FONT_CLIP_PRECISION = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_LH_ANGLES: FONT_CLIP_PRECISION = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_MASK: FONT_CLIP_PRECISION = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_STROKE_PRECIS: FONT_CLIP_PRECISION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIP_TT_ALWAYS: FONT_CLIP_PRECISION = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type FONT_LICENSE_PRIVS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_PREVIEWPRINT: FONT_LICENSE_PRIVS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_EDITABLE: FONT_LICENSE_PRIVS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_INSTALLABLE: FONT_LICENSE_PRIVS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_NOEMBEDDING: FONT_LICENSE_PRIVS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LICENSE_DEFAULT: FONT_LICENSE_PRIVS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type FONT_OUTPUT_PRECISION = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_CHARACTER_PRECIS: FONT_OUTPUT_PRECISION = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_DEFAULT_PRECIS: FONT_OUTPUT_PRECISION = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_DEVICE_PRECIS: FONT_OUTPUT_PRECISION = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_OUTLINE_PRECIS: FONT_OUTPUT_PRECISION = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_PS_ONLY_PRECIS: FONT_OUTPUT_PRECISION = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_RASTER_PRECIS: FONT_OUTPUT_PRECISION = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_STRING_PRECIS: FONT_OUTPUT_PRECISION = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_STROKE_PRECIS: FONT_OUTPUT_PRECISION = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_TT_ONLY_PRECIS: FONT_OUTPUT_PRECISION = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_TT_PRECIS: FONT_OUTPUT_PRECISION = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type FONT_PITCH_AND_FAMILY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_DECORATIVE: FONT_PITCH_AND_FAMILY = 80u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_DONTCARE: FONT_PITCH_AND_FAMILY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_MODERN: FONT_PITCH_AND_FAMILY = 48u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_ROMAN: FONT_PITCH_AND_FAMILY = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_SCRIPT: FONT_PITCH_AND_FAMILY = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FF_SWISS: FONT_PITCH_AND_FAMILY = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type FONT_QUALITY = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANTIALIASED_QUALITY: FONT_QUALITY = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLEARTYPE_QUALITY: FONT_QUALITY = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_QUALITY: FONT_QUALITY = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRAFT_QUALITY: FONT_QUALITY = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NONANTIALIASED_QUALITY: FONT_QUALITY = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PROOF_QUALITY: FONT_QUALITY = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type FONT_RESOURCE_CHARACTERISTICS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FR_PRIVATE: FONT_RESOURCE_CHARACTERISTICS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FR_NOT_ENUM: FONT_RESOURCE_CHARACTERISTICS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_ARABIC: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_BALTIC: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_CHINESESIMP: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_CHINESETRAD: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_CYRILLIC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_GREEK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_HEBREW: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_JISJAPAN: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_JOHAB: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_LATIN1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_LATIN2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_SYMBOL: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_THAI: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_TURKISH: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_VIETNAMESE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FS_WANSUNG: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_BLACK: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_BOLD: u32 = 700u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_DEMIBOLD: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_DONTCARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_EXTRABOLD: u32 = 800u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_EXTRALIGHT: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_HEAVY: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_LIGHT: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_MEDIUM: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_NORMAL: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_REGULAR: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_SEMIBOLD: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_THIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_ULTRABOLD: u32 = 800u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const FW_ULTRALIGHT: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GB2312_CHARSET: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_ARABIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_HEBREW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATINNUMBER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATINNUMERICSEPARATOR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LATINNUMERICTERMINATOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_LOCALNUMBER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_NEUTRAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_NUMERICSEPARATOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_POSTBOUNDLTR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_POSTBOUNDRTL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_PREBOUNDLTR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPCLASS_PREBOUNDRTL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPGLYPH_LINKAFTER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCPGLYPH_LINKBEFORE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_DBCS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_ERROR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_JUSTIFYIN: i32 = 2097152i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GCP_RESULTSA {
    pub lStructSize: u32,
    pub lpOutString: ::windows_sys::core::PSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: ::windows_sys::core::PSTR,
    pub lpGlyphs: ::windows_sys::core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl ::core::marker::Copy for GCP_RESULTSA {}
impl ::core::clone::Clone for GCP_RESULTSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GCP_RESULTSW {
    pub lStructSize: u32,
    pub lpOutString: ::windows_sys::core::PWSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: ::windows_sys::core::PSTR,
    pub lpGlyphs: ::windows_sys::core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl ::core::marker::Copy for GCP_RESULTSW {}
impl ::core::clone::Clone for GCP_RESULTSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_BEGINGROUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_ENDGROUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_IDENTIFIER: u32 = 1128875079u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_MULTIFORMATS: u32 = 1073741828u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_UNICODE_END: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_UNICODE_STRING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDIPLUS_TS_QUERYVER: u32 = 4122u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDIPLUS_TS_RECORD: u32 = 4123u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDIREGISTERDDRAWPACKETVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GDI_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETCOLORTABLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETDEVICEUNITS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETEXTENDEDTEXTMETRICS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETEXTENTTABLE: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETFACENAME: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPAIRKERNTABLE: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPENWIDTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPHYSPAGESIZE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETPRINTINGOFFSET: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSCALINGFACTOR: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETPAPERBINS: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETPAPERMETRICS: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETPRINTORIENT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETSETSCREENPARAMS: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETTECHNOLGY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETTECHNOLOGY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETTRACKKERNTABLE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETVECTORBRUSHSIZE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GETVECTORPENSIZE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GET_CHARACTER_PLACEMENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_CLASSIN: GET_CHARACTER_PLACEMENT_FLAGS = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_DIACRITIC: GET_CHARACTER_PLACEMENT_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_DISPLAYZWG: GET_CHARACTER_PLACEMENT_FLAGS = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_GLYPHSHAPE: GET_CHARACTER_PLACEMENT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_JUSTIFY: GET_CHARACTER_PLACEMENT_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_KASHIDA: GET_CHARACTER_PLACEMENT_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_LIGATE: GET_CHARACTER_PLACEMENT_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_MAXEXTENT: GET_CHARACTER_PLACEMENT_FLAGS = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NEUTRALOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NUMERICOVERRIDE: GET_CHARACTER_PLACEMENT_FLAGS = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NUMERICSLATIN: GET_CHARACTER_PLACEMENT_FLAGS = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_NUMERICSLOCAL: GET_CHARACTER_PLACEMENT_FLAGS = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_REORDER: GET_CHARACTER_PLACEMENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_SYMSWAPOFF: GET_CHARACTER_PLACEMENT_FLAGS = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GCP_USEKERNING: GET_CHARACTER_PLACEMENT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GET_DCX_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_WINDOW: GET_DCX_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_CACHE: GET_DCX_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_PARENTCLIP: GET_DCX_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_CLIPSIBLINGS: GET_DCX_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_CLIPCHILDREN: GET_DCX_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_NORESETATTRS: GET_DCX_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_LOCKWINDOWUPDATE: GET_DCX_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_EXCLUDERGN: GET_DCX_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_INTERSECTRGN: GET_DCX_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_INTERSECTUPDATE: GET_DCX_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCX_VALIDATE: GET_DCX_FLAGS = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GET_DEVICE_CAPS_INDEX = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DRIVERVERSION: GET_DEVICE_CAPS_INDEX = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TECHNOLOGY: GET_DEVICE_CAPS_INDEX = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HORZSIZE: GET_DEVICE_CAPS_INDEX = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VERTSIZE: GET_DEVICE_CAPS_INDEX = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HORZRES: GET_DEVICE_CAPS_INDEX = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VERTRES: GET_DEVICE_CAPS_INDEX = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BITSPIXEL: GET_DEVICE_CAPS_INDEX = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PLANES: GET_DEVICE_CAPS_INDEX = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMBRUSHES: GET_DEVICE_CAPS_INDEX = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMPENS: GET_DEVICE_CAPS_INDEX = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMMARKERS: GET_DEVICE_CAPS_INDEX = 20u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMFONTS: GET_DEVICE_CAPS_INDEX = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMCOLORS: GET_DEVICE_CAPS_INDEX = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PDEVICESIZE: GET_DEVICE_CAPS_INDEX = 26u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CURVECAPS: GET_DEVICE_CAPS_INDEX = 28u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LINECAPS: GET_DEVICE_CAPS_INDEX = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POLYGONALCAPS: GET_DEVICE_CAPS_INDEX = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TEXTCAPS: GET_DEVICE_CAPS_INDEX = 34u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CLIPCAPS: GET_DEVICE_CAPS_INDEX = 36u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RASTERCAPS: GET_DEVICE_CAPS_INDEX = 38u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECTX: GET_DEVICE_CAPS_INDEX = 40u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECTY: GET_DEVICE_CAPS_INDEX = 42u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ASPECTXY: GET_DEVICE_CAPS_INDEX = 44u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LOGPIXELSX: GET_DEVICE_CAPS_INDEX = 88u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LOGPIXELSY: GET_DEVICE_CAPS_INDEX = 90u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SIZEPALETTE: GET_DEVICE_CAPS_INDEX = 104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NUMRESERVED: GET_DEVICE_CAPS_INDEX = 106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORRES: GET_DEVICE_CAPS_INDEX = 108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALWIDTH: GET_DEVICE_CAPS_INDEX = 110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALHEIGHT: GET_DEVICE_CAPS_INDEX = 111u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALOFFSETX: GET_DEVICE_CAPS_INDEX = 112u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PHYSICALOFFSETY: GET_DEVICE_CAPS_INDEX = 113u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SCALINGFACTORX: GET_DEVICE_CAPS_INDEX = 114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SCALINGFACTORY: GET_DEVICE_CAPS_INDEX = 115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VREFRESH: GET_DEVICE_CAPS_INDEX = 116u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DESKTOPVERTRES: GET_DEVICE_CAPS_INDEX = 117u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DESKTOPHORZRES: GET_DEVICE_CAPS_INDEX = 118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLTALIGNMENT: GET_DEVICE_CAPS_INDEX = 119u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SHADEBLENDCAPS: GET_DEVICE_CAPS_INDEX = 120u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORMGMTCAPS: GET_DEVICE_CAPS_INDEX = 121u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GET_GLYPH_OUTLINE_FORMAT = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_BEZIER: GET_GLYPH_OUTLINE_FORMAT = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_BITMAP: GET_GLYPH_OUTLINE_FORMAT = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GLYPH_INDEX: GET_GLYPH_OUTLINE_FORMAT = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GRAY2_BITMAP: GET_GLYPH_OUTLINE_FORMAT = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GRAY4_BITMAP: GET_GLYPH_OUTLINE_FORMAT = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_GRAY8_BITMAP: GET_GLYPH_OUTLINE_FORMAT = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_METRICS: GET_GLYPH_OUTLINE_FORMAT = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_NATIVE: GET_GLYPH_OUTLINE_FORMAT = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGO_UNHINTED: GET_GLYPH_OUTLINE_FORMAT = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GET_PS_FEATURESETTING: u32 = 4121u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GET_STOCK_OBJECT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACK_BRUSH: GET_STOCK_OBJECT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DKGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_BRUSH: GET_STOCK_OBJECT_FLAGS = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HOLLOW_BRUSH: GET_STOCK_OBJECT_FLAGS = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LTGRAY_BRUSH: GET_STOCK_OBJECT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NULL_BRUSH: GET_STOCK_OBJECT_FLAGS = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITE_BRUSH: GET_STOCK_OBJECT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACK_PEN: GET_STOCK_OBJECT_FLAGS = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DC_PEN: GET_STOCK_OBJECT_FLAGS = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NULL_PEN: GET_STOCK_OBJECT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITE_PEN: GET_STOCK_OBJECT_FLAGS = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANSI_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ANSI_VAR_FONT: GET_STOCK_OBJECT_FLAGS = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEVICE_DEFAULT_FONT: GET_STOCK_OBJECT_FLAGS = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_GUI_FONT: GET_STOCK_OBJECT_FLAGS = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSTEM_FONT: GET_STOCK_OBJECT_FLAGS = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSTEM_FIXED_FONT: GET_STOCK_OBJECT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DEFAULT_PALETTE: GET_STOCK_OBJECT_FLAGS = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: u32,
    pub gmBlackBoxY: u32,
    pub gmptGlyphOrigin: super::super::Foundation::POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GLYPHMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GLYPHMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GLYPHSET {
    pub cbThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRanges: u32,
    pub ranges: [WCRANGE; 1],
}
impl ::core::marker::Copy for GLYPHSET {}
impl ::core::clone::Clone for GLYPHSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GM_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type GOBJENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GRADIENT_FILL = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_RECT_H: GRADIENT_FILL = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_RECT_V: GRADIENT_FILL = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_TRIANGLE: GRADIENT_FILL = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GRADIENT_FILL_OP_FLAG: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GRADIENT_RECT {
    pub UpperLeft: u32,
    pub LowerRight: u32,
}
impl ::core::marker::Copy for GRADIENT_RECT {}
impl ::core::clone::Clone for GRADIENT_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct GRADIENT_TRIANGLE {
    pub Vertex1: u32,
    pub Vertex2: u32,
    pub Vertex3: u32,
}
impl ::core::marker::Copy for GRADIENT_TRIANGLE {}
impl ::core::clone::Clone for GRADIENT_TRIANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type GRAPHICS_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GM_COMPATIBLE: GRAPHICS_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GM_ADVANCED: GRAPHICS_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type GRAYSTRINGPROC = ::core::option::Option<unsafe extern "system" fn(param0: HDC, param1: super::super::Foundation::LPARAM, param2: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GREEK_CHARSET: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const GS_8BIT_INDICES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct HANDLETABLE {
    pub objectHandle: [HGDIOBJ; 1],
}
impl ::core::marker::Copy for HANDLETABLE {}
impl ::core::clone::Clone for HANDLETABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HANGEUL_CHARSET: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HANGUL_CHARSET: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type HATCH_BRUSH_STYLE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_BDIAGONAL: HATCH_BRUSH_STYLE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_CROSS: HATCH_BRUSH_STYLE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_DIAGCROSS: HATCH_BRUSH_STYLE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_FDIAGONAL: HATCH_BRUSH_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_HORIZONTAL: HATCH_BRUSH_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_VERTICAL: HATCH_BRUSH_STYLE = 1u32;
pub type HBITMAP = isize;
pub type HBRUSH = isize;
pub type HDC = isize;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type HDC_MAP_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_ANISOTROPIC: HDC_MAP_MODE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_HIENGLISH: HDC_MAP_MODE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_HIMETRIC: HDC_MAP_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_ISOTROPIC: HDC_MAP_MODE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_LOENGLISH: HDC_MAP_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_LOMETRIC: HDC_MAP_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_TEXT: HDC_MAP_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_TWIPS: HDC_MAP_MODE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HEBREW_CHARSET: u32 = 177u32;
pub type HENHMETAFILE = isize;
pub type HFONT = isize;
pub type HGDIOBJ = isize;
pub type HMETAFILE = isize;
pub type HMONITOR = isize;
pub type HPALETTE = isize;
pub type HPEN = isize;
pub type HRGN = isize;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HS_API_MAX: u32 = 12u32;
pub type HdcMetdataEnhFileHandle = isize;
pub type HdcMetdataFileHandle = isize;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_B: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_C: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D50: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D55: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D65: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_D75: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_DAYLIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_DEVICE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_F2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_FLUORESCENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_MAX_INDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_NTSC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const ILLUMINANT_TUNGSTEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const JOHAB_CHARSET: u32 = 130u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct KERNINGPAIR {
    pub wFirst: u16,
    pub wSecond: u16,
    pub iKernAmount: i32,
}
impl ::core::marker::Copy for KERNINGPAIR {}
impl ::core::clone::Clone for KERNINGPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_BTT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LAYOUT_VBH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_CALIBRATED_RGB: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_ABS_COLORIMETRIC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_BUSINESS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_GRAPHICS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LCS_GM_IMAGES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_INTERIORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_MARKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_POLYLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_POLYMARKER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_STYLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_WIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LC_WIDESTYLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LF_FACESIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LF_FULLFACESIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LINEDDAPROC = ::core::option::Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: super::super::Foundation::LPARAM)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct LOGBRUSH {
    pub lbStyle: u32,
    pub lbColor: u32,
    pub lbHatch: usize,
}
impl ::core::marker::Copy for LOGBRUSH {}
impl ::core::clone::Clone for LOGBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct LOGBRUSH32 {
    pub lbStyle: u32,
    pub lbColor: u32,
    pub lbHatch: u32,
}
impl ::core::marker::Copy for LOGBRUSH32 {}
impl ::core::clone::Clone for LOGBRUSH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGFONTA {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOGFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct LOGFONTW {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u16; 32],
}
impl ::core::marker::Copy for LOGFONTW {}
impl ::core::clone::Clone for LOGFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct LOGPALETTE {
    pub palVersion: u16,
    pub palNumEntries: u16,
    pub palPalEntry: [PALETTEENTRY; 1],
}
impl ::core::marker::Copy for LOGPALETTE {}
impl ::core::clone::Clone for LOGPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOGPEN {
    pub lopnStyle: u32,
    pub lopnWidth: super::super::Foundation::POINT,
    pub lopnColor: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOGPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_DOUBLEBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SHARE_ACCUM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SHARE_DEPTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SHARE_STENCIL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_STEREO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SUPPORT_GDI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SUPPORT_OPENGL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SWAP_COPY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_SWAP_EXCHANGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_TRANSPARENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_TYPE_COLORINDEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const LPD_TYPE_RGBA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDEVCAPS = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR, param1: ::windows_sys::core::PCSTR, param2: u32, param3: ::windows_sys::core::PCSTR, param4: *mut DEVMODEA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDEVMODE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, param2: *mut DEVMODEA, param3: ::windows_sys::core::PCSTR, param4: ::windows_sys::core::PCSTR, param5: *mut DEVMODEA, param6: ::windows_sys::core::PCSTR, param7: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MAC_CHARSET: u32 = 77u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
impl ::core::marker::Copy for MAT2 {}
impl ::core::clone::Clone for MAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MAXSTRETCHBLTMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const METAFILE_DRIVER: u32 = 2049u32;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct METAHEADER {
    pub mtType: u16,
    pub mtHeaderSize: u16,
    pub mtVersion: u16,
    pub mtSize: u32,
    pub mtNoObjects: u16,
    pub mtMaxRecord: u32,
    pub mtNoParameters: u16,
}
impl ::core::marker::Copy for METAHEADER {}
impl ::core::clone::Clone for METAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct METARECORD {
    pub rdSize: u32,
    pub rdFunction: u16,
    pub rdParm: [u16; 1],
}
impl ::core::marker::Copy for METARECORD {}
impl ::core::clone::Clone for METARECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ANIMATEPALETTE: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ARC: u32 = 2071u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_BITBLT: u32 = 2338u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CHORD: u32 = 2096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEBRUSHINDIRECT: u32 = 764u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEFONTINDIRECT: u32 = 763u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEPALETTE: u32 = 247u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEPATTERNBRUSH: u32 = 505u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEPENINDIRECT: u32 = 762u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_CREATEREGION: u32 = 1791u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DELETEOBJECT: u32 = 496u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DIBBITBLT: u32 = 2368u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DIBCREATEPATTERNBRUSH: u32 = 322u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_DIBSTRETCHBLT: u32 = 2881u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ELLIPSE: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ESCAPE: u32 = 1574u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_EXCLUDECLIPRECT: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_EXTFLOODFILL: u32 = 1352u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_EXTTEXTOUT: u32 = 2610u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_FILLREGION: u32 = 552u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_FLOODFILL: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_FRAMEREGION: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_INTERSECTCLIPRECT: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_INVERTREGION: u32 = 298u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_LINETO: u32 = 531u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_MOVETO: u32 = 532u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_OFFSETCLIPRGN: u32 = 544u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_OFFSETVIEWPORTORG: u32 = 529u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_OFFSETWINDOWORG: u32 = 527u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_PAINTREGION: u32 = 299u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_PATBLT: u32 = 1565u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_PIE: u32 = 2074u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_POLYGON: u32 = 804u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_POLYLINE: u32 = 805u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_POLYPOLYGON: u32 = 1336u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_REALIZEPALETTE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_RECTANGLE: u32 = 1051u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_RESIZEPALETTE: u32 = 313u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_RESTOREDC: u32 = 295u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_ROUNDRECT: u32 = 1564u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SAVEDC: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SCALEVIEWPORTEXT: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SCALEWINDOWEXT: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SELECTCLIPREGION: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SELECTOBJECT: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SELECTPALETTE: u32 = 564u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETBKCOLOR: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETBKMODE: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETDIBTODEV: u32 = 3379u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETLAYOUT: u32 = 329u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETMAPMODE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETMAPPERFLAGS: u32 = 561u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETPALENTRIES: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETPIXEL: u32 = 1055u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETPOLYFILLMODE: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETRELABS: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETROP2: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETSTRETCHBLTMODE: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTALIGN: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTCHAREXTRA: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTCOLOR: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETTEXTJUSTIFICATION: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETVIEWPORTEXT: u32 = 526u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETVIEWPORTORG: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETWINDOWEXT: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_SETWINDOWORG: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_STRETCHBLT: u32 = 2851u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_STRETCHDIB: u32 = 3907u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const META_TEXTOUT: u32 = 1313u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MFCOMMENT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type MFENUMPROC = ::core::option::Option<unsafe extern "system" fn(hdc: HDC, lpht: *const HANDLETABLE, lpmr: *const METARECORD, nobj: i32, param4: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_MAX_AXES_NAMELEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MM_MAX_NUMAXES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type MODIFY_WORLD_TRANSFORM_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MWT_IDENTITY: MODIFY_WORLD_TRANSFORM_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MWT_LEFTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MWT_RIGHTMULTIPLY: MODIFY_WORLD_TRANSFORM_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type MONITORENUMPROC = ::core::option::Option<unsafe extern "system" fn(param0: HMONITOR, param1: HDC, param2: *mut super::super::Foundation::RECT, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: super::super::Foundation::RECT,
    pub rcWork: super::super::Foundation::RECT,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFOEXA {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITORINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITORINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFOEXW {
    pub monitorInfo: MONITORINFO,
    pub szDevice: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITORINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITORINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type MONITOR_FROM_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONITOR_DEFAULTTONEAREST: MONITOR_FROM_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONITOR_DEFAULTTONULL: MONITOR_FROM_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONITOR_DEFAULTTOPRIMARY: MONITOR_FROM_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MONO_FONT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MOUSETRAILS: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NEWFRAME: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct NEWTEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
impl ::core::marker::Copy for NEWTEXTMETRICA {}
impl ::core::clone::Clone for NEWTEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct NEWTEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
    pub ntmFlags: u32,
    pub ntmSizeEM: u32,
    pub ntmCellHeight: u32,
    pub ntmAvgWidth: u32,
}
impl ::core::marker::Copy for NEWTEXTMETRICW {}
impl ::core::clone::Clone for NEWTEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NEWTRANSPARENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NEXTBAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_BOLD: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_DSIG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_ITALIC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_MULTIPLEMASTER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_NONNEGATIVE_AC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_PS_OPENTYPE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_REGULAR: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_TT_OPENTYPE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NTM_TYPE1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NULLREGION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type OBJ_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_PEN: OBJ_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_BRUSH: OBJ_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_DC: OBJ_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_METADC: OBJ_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_PAL: OBJ_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_FONT: OBJ_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_BITMAP: OBJ_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_REGION: OBJ_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_METAFILE: OBJ_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_MEMDC: OBJ_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_EXTPEN: OBJ_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_ENHMETADC: OBJ_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_ENHMETAFILE: OBJ_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OBJ_COLORSPACE: OBJ_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OEM_CHARSET: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OPENCHANNEL: u32 = 4110u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OUTLINETEXTMETRICA {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: ::windows_sys::core::PSTR,
    pub otmpFaceName: ::windows_sys::core::PSTR,
    pub otmpStyleName: ::windows_sys::core::PSTR,
    pub otmpFullName: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OUTLINETEXTMETRICA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OUTLINETEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OUTLINETEXTMETRICW {
    pub otmSize: u32,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: u8,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: u32,
    pub otmfsType: u32,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: u32,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: u32,
    pub otmsCapEmHeight: u32,
    pub otmsXHeight: u32,
    pub otmrcFontBox: super::super::Foundation::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::super::Foundation::POINT,
    pub otmptSubscriptOffset: super::super::Foundation::POINT,
    pub otmptSuperscriptSize: super::super::Foundation::POINT,
    pub otmptSuperscriptOffset: super::super::Foundation::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: ::windows_sys::core::PSTR,
    pub otmpFaceName: ::windows_sys::core::PSTR,
    pub otmpStyleName: ::windows_sys::core::PSTR,
    pub otmpFullName: ::windows_sys::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OUTLINETEXTMETRICW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OUTLINETEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const OUT_SCREEN_OUTLINE_PRECIS: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: super::super::Foundation::BOOL,
    pub rcPaint: super::super::Foundation::RECT,
    pub fRestore: super::super::Foundation::BOOL,
    pub fIncUpdate: super::super::Foundation::BOOL,
    pub rgbReserved: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAINTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAINTSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
impl ::core::marker::Copy for PALETTEENTRY {}
impl ::core::clone::Clone for PALETTEENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct PANOSE {
    pub bFamilyType: u8,
    pub bSerifStyle: u8,
    pub bWeight: u8,
    pub bProportion: u8,
    pub bContrast: u8,
    pub bStrokeVariation: u8,
    pub bArmStyle: u8,
    pub bLetterform: u8,
    pub bMidline: u8,
    pub bXHeight: u8,
}
impl ::core::marker::Copy for PANOSE {}
impl ::core::clone::Clone for PANOSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PANOSE_COUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_ARMSTYLE_INDEX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_DOUBLE_SERIF: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_HORZ: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_SINGLE_SERIF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_VERT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_BENT_ARMS_WEDGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_HIGH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_INDEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_LOW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_MEDIUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_MEDIUM_HIGH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_MEDIUM_LOW: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_NONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_VERY_HIGH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CONTRAST_VERY_LOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_CULTURE_LATIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILYTYPE_INDEX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_DECORATIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_PICTORIAL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_SCRIPT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_FAMILY_TEXT_DISPLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETTERFORM_INDEX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_BOXED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_CONTACT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_FLATTENED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_OFF_CENTER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_ROUNDED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_SQUARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_NORMAL_WEIGHTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_BOXED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_CONTACT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_FLATTENED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_OFF_CENTER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_ROUNDED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_SQUARE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_LETT_OBLIQUE_WEIGHTED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_CONSTANT_POINTED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_CONSTANT_SERIFED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_CONSTANT_TRIMMED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_HIGH_POINTED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_HIGH_SERIFED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_HIGH_TRIMMED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_INDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_LOW_POINTED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_LOW_SERIFED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_LOW_TRIMMED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_STANDARD_POINTED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_STANDARD_SERIFED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_MIDLINE_STANDARD_TRIMMED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_NO_FIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROPORTION_INDEX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_CONDENSED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_EVEN_WIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_EXPANDED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_MODERN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_MONOSPACED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_OLD_STYLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_VERY_CONDENSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_PROP_VERY_EXPANDED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIFSTYLE_INDEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_BONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_COVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_EXAGGERATED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_FLARED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_NORMAL_SANS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_OBTUSE_COVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_OBTUSE_SANS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_PERP_SANS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_ROUNDED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_SQUARE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_SQUARE_COVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_THIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_SERIF_TRIANGLE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_HORZ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_VERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STRAIGHT_ARMS_WEDGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKEVARIATION_INDEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_DIAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_HORZ: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_TRAN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_GRADUAL_VERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_INSTANT_VERT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_RAPID_HORZ: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_STROKE_RAPID_VERT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_BLACK: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_BOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_BOOK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_DEMI: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_HEAVY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_INDEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_LIGHT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_MEDIUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_NORD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_THIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_WEIGHT_VERY_LIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_CONSTANT_LARGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_CONSTANT_SMALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_CONSTANT_STD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_DUCKING_LARGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_DUCKING_SMALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_DUCKING_STD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PAN_XHEIGHT_INDEX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PASSTHROUGH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_EXPLICIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_INTERIORS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_NOCOLLAPSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_PATHS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_POLYGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_POLYPOLYGON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_RECTANGLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_RESERVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_SCANLINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_STYLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_TRAPEZOID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_WIDE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_WIDESTYLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PC_WINDPOLYGON: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct PELARRAY {
    pub paXCount: i32,
    pub paYCount: i32,
    pub paXExt: i32,
    pub paYExt: i32,
    pub paRGBs: u8,
}
impl ::core::marker::Copy for PELARRAY {}
impl ::core::clone::Clone for PELARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type PEN_STYLE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_GEOMETRIC: PEN_STYLE = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_COSMETIC: PEN_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_SOLID: PEN_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DASH: PEN_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DOT: PEN_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DASHDOT: PEN_STYLE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_DASHDOTDOT: PEN_STYLE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_NULL: PEN_STYLE = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_INSIDEFRAME: PEN_STYLE = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_USERSTYLE: PEN_STYLE = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ALTERNATE: PEN_STYLE = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_STYLE_MASK: PEN_STYLE = 15u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_ROUND: PEN_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_SQUARE: PEN_STYLE = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_FLAT: PEN_STYLE = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_ENDCAP_MASK: PEN_STYLE = 3840u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_ROUND: PEN_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_BEVEL: PEN_STYLE = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_MITER: PEN_STYLE = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_JOIN_MASK: PEN_STYLE = 61440u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PS_TYPE_MASK: PEN_STYLE = 983040u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct POINTFX {
    pub x: FIXED,
    pub y: FIXED,
}
impl ::core::marker::Copy for POINTFX {}
impl ::core::clone::Clone for POINTFX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POLYFILL_LAST: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLYTEXTA {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: ::windows_sys::core::PCSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLYTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLYTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLYTEXTW {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: ::windows_sys::core::PCWSTR,
    pub uiFlags: u32,
    pub rcl: super::super::Foundation::RECT,
    pub pdx: *mut i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLYTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLYTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_DATA: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_IDENTIFY: u32 = 4117u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_IGNORE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_INJECTION: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const POSTSCRIPT_PASSTHROUGH: u32 = 4115u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_CPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_IPM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_LPM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PRINTRATEUNIT_PPM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PR_JOBSTATUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSIDENT_GDICENTRIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSIDENT_PSCENTRIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSINJECT_DLFONT: u32 = 3722304989u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_ASCII: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_BCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_BINARY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PSPROTOCOL_TBCP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_BEZIERTO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_CLOSEFIGURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_LINETO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PT_MOVETO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_ALL_PATHS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_DATABASE_CURRENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_INCLUDE_HMD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_ONLY_ACTIVE_PATHS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_VIRTUAL_MODE_AWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_DIBTOSCREEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_GETDIBITS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_SETDIBITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QDI_STRETCHDIB: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QUERYDIBSUPPORT: u32 = 3073u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QUERYESCSUPPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const QUERYROPSUPPORT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type R2_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_BLACK: R2_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTMERGEPEN: R2_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MASKNOTPEN: R2_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTCOPYPEN: R2_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MASKPENNOT: R2_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOT: R2_MODE = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_XORPEN: R2_MODE = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTMASKPEN: R2_MODE = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MASKPEN: R2_MODE = 9i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOTXORPEN: R2_MODE = 10i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_NOP: R2_MODE = 11i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MERGENOTPEN: R2_MODE = 12i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_COPYPEN: R2_MODE = 13i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MERGEPENNOT: R2_MODE = 14i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_MERGEPEN: R2_MODE = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_WHITE: R2_MODE = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const R2_LAST: R2_MODE = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct RASTERIZER_STATUS {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
impl ::core::marker::Copy for RASTERIZER_STATUS {}
impl ::core::clone::Clone for RASTERIZER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RASTER_FONTTYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BANDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BIGFONT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BITBLT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_BITMAP64: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_DEVBITS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_DIBTODEV: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_DI_BITMAP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_FLOODFILL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_GDI20_OUTPUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_GDI20_STATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_OP_DX_OUTPUT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_PALETTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_SAVEBITMAP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_SCALING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_STRETCHBLT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RC_STRETCHDIB: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDH_RECTANGLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type READEMBEDPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void, param2: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type REDRAW_WINDOW_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_INVALIDATE: REDRAW_WINDOW_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_INTERNALPAINT: REDRAW_WINDOW_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_ERASE: REDRAW_WINDOW_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_VALIDATE: REDRAW_WINDOW_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOINTERNALPAINT: REDRAW_WINDOW_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOERASE: REDRAW_WINDOW_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOCHILDREN: REDRAW_WINDOW_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_ALLCHILDREN: REDRAW_WINDOW_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_UPDATENOW: REDRAW_WINDOW_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_ERASENOW: REDRAW_WINDOW_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_FRAME: REDRAW_WINDOW_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RDW_NOFRAME: REDRAW_WINDOW_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RELATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RESTORE_CTM: u32 = 4100u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
impl ::core::marker::Copy for RGBQUAD {}
impl ::core::clone::Clone for RGBQUAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct RGBTRIPLE {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}
impl ::core::marker::Copy for RGBTRIPLE {}
impl ::core::clone::Clone for RGBTRIPLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RGNDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RGNDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RGNDATAHEADER {
    pub dwSize: u32,
    pub iType: u32,
    pub nCount: u32,
    pub nRgnSize: u32,
    pub rcBound: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RGNDATAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RGNDATAHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type RGN_COMBINE_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_AND: RGN_COMBINE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_OR: RGN_COMBINE_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_XOR: RGN_COMBINE_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_DIFF: RGN_COMBINE_MODE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_COPY: RGN_COMBINE_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_MIN: RGN_COMBINE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_MAX: RGN_COMBINE_MODE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RGN_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type ROP_CODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCCOPY: ROP_CODE = 13369376u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCPAINT: ROP_CODE = 15597702u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCAND: ROP_CODE = 8913094u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCINVERT: ROP_CODE = 6684742u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SRCERASE: ROP_CODE = 4457256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NOTSRCCOPY: ROP_CODE = 3342344u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NOTSRCERASE: ROP_CODE = 1114278u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MERGECOPY: ROP_CODE = 12583114u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const MERGEPAINT: ROP_CODE = 12255782u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PATCOPY: ROP_CODE = 15728673u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PATPAINT: ROP_CODE = 16452105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const PATINVERT: ROP_CODE = 5898313u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DSTINVERT: ROP_CODE = 5570569u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACKNESS: ROP_CODE = 66u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITENESS: ROP_CODE = 16711778u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const NOMIRRORBITMAP: ROP_CODE = 2147483648u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const CAPTUREBLT: ROP_CODE = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const RUSSIAN_CHARSET: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SAVE_CTM: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_CONST_ALPHA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_GRAD_RECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_GRAD_TRI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_PIXEL_ALPHA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SB_PREMULT_ALPHA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SC_SCREENSAVE: u32 = 61760u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_ALLOW_CHANGES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_ALLOW_PATH_ORDER_CHANGES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_APPLY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_FORCE_MODE_ENUMERATION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_NO_OPTIMIZATION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_PATH_PERSIST_IF_REQUIRED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_SAVE_TO_DATABASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_CLONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_EXTEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_INTERNAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_TOPOLOGY_SUPPLIED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_VALIDATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_VIRTUAL_MODE_AWARE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SELECTDIB: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SELECTPAPERSOURCE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETABORTPROC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETALLJUSTVALUES: u32 = 771u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETCHARSET: u32 = 772u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETCOLORTABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETCOPYCOUNT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETDIBSCALING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETICMPROFILE_EMBEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETKERNTRACK: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETLINECAP: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETLINEJOIN: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SETMITERLIMIT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_ARC_DIRECTION: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_BACKGROUND_COLOR: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_BOUNDS: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type SET_BOUNDS_RECT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_ACCUMULATE: SET_BOUNDS_RECT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_DISABLE: SET_BOUNDS_RECT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_ENABLE: SET_BOUNDS_RECT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const DCB_RESET: SET_BOUNDS_RECT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_CLIP_BOX: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_MIRROR_MODE: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_POLY_MODE: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_SCREEN_ANGLE: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SET_SPREAD: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SHIFTJIS_CHARSET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SIMPLEREGION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SPCLPASSTHROUGH2: u32 = 4568u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_APPABORT: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_NOTREPORTED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_OUTOFDISK: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_OUTOFMEMORY: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SP_USERABORT: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STARTDOC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STOCK_LAST: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCHBLT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type STRETCH_BLT_MODE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const BLACKONWHITE: STRETCH_BLT_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const COLORONCOLOR: STRETCH_BLT_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const HALFTONE: STRETCH_BLT_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_ANDSCANS: STRETCH_BLT_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_DELETESCANS: STRETCH_BLT_MODE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_HALFTONE: STRETCH_BLT_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const STRETCH_ORSCANS: STRETCH_BLT_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WHITEONBLACK: STRETCH_BLT_MODE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYMBOL_CHARSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSRGN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type SYSTEM_PALETTE_USE = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_NOSTATIC: SYSTEM_PALETTE_USE = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_NOSTATIC256: SYSTEM_PALETTE_USE = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const SYSPAL_STATIC: SYSTEM_PALETTE_USE = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_CP_STROKE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_CR_90: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_CR_ANY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_EA_DOUBLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_IA_ABLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_OP_CHARACTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_OP_STROKE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_RA_ABLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SA_CONTIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SA_DOUBLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SA_INTEGER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SCROLLBLT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SF_X_YINDEP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_SO_ABLE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_UA_ABLE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TC_VA_ABLE: u32 = 16384u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TEXTMETRICA {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u8,
    pub tmLastChar: u8,
    pub tmDefaultChar: u8,
    pub tmBreakChar: u8,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
impl ::core::marker::Copy for TEXTMETRICA {}
impl ::core::clone::Clone for TEXTMETRICA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TEXTMETRICW {
    pub tmHeight: i32,
    pub tmAscent: i32,
    pub tmDescent: i32,
    pub tmInternalLeading: i32,
    pub tmExternalLeading: i32,
    pub tmAveCharWidth: i32,
    pub tmMaxCharWidth: i32,
    pub tmWeight: i32,
    pub tmOverhang: i32,
    pub tmDigitizedAspectX: i32,
    pub tmDigitizedAspectY: i32,
    pub tmFirstChar: u16,
    pub tmLastChar: u16,
    pub tmDefaultChar: u16,
    pub tmBreakChar: u16,
    pub tmItalic: u8,
    pub tmUnderlined: u8,
    pub tmStruckOut: u8,
    pub tmPitchAndFamily: u8,
    pub tmCharSet: u8,
}
impl ::core::marker::Copy for TEXTMETRICW {}
impl ::core::clone::Clone for TEXTMETRICW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type TEXT_ALIGN_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_NOUPDATECP: TEXT_ALIGN_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_UPDATECP: TEXT_ALIGN_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_LEFT: TEXT_ALIGN_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_RIGHT: TEXT_ALIGN_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_CENTER: TEXT_ALIGN_OPTIONS = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_TOP: TEXT_ALIGN_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_BOTTOM: TEXT_ALIGN_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_BASELINE: TEXT_ALIGN_OPTIONS = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_RTLREADING: TEXT_ALIGN_OPTIONS = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TA_MASK: TEXT_ALIGN_OPTIONS = 287u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_BASELINE: TEXT_ALIGN_OPTIONS = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_LEFT: TEXT_ALIGN_OPTIONS = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_RIGHT: TEXT_ALIGN_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_CENTER: TEXT_ALIGN_OPTIONS = 6u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_BOTTOM: TEXT_ALIGN_OPTIONS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VTA_TOP: TEXT_ALIGN_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const THAI_CHARSET: u32 = 222u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_DEVICE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_FIXED_PITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_TRUETYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TMPF_VECTOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TRANSFORM_CTM: u32 = 4107u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TRIVERTEX {
    pub x: i32,
    pub y: i32,
    pub Red: u16,
    pub Green: u16,
    pub Blue: u16,
    pub Alpha: u16,
}
impl ::core::marker::Copy for TRIVERTEX {}
impl ::core::clone::Clone for TRIVERTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TRUETYPE_FONTTYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTDELETE_DONTREMOVEFONT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTEMBEDINFO {
    pub usStructSize: u16,
    pub usRootStrSize: u16,
    pub pusRootStr: *mut u16,
}
impl ::core::marker::Copy for TTEMBEDINFO {}
impl ::core::clone::Clone for TTEMBEDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_EUDCEMBEDDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_FAILIFVARIATIONSIMULATED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type TTEMBED_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_EMBEDEUDC: TTEMBED_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_RAW: TTEMBED_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_SUBSET: TTEMBED_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_TTCOMPRESSED: TTEMBED_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_SUBSETCANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_VARIATIONSIMULATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_WEBOBJECT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTEMBED_XORENCRYPTDATA: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_APPLE_PLATFORMID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_DELTA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_DONT_CARE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_COMPRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_GLYPHLIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_SUBSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_FLAGS_TTC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_LANG_KEEP_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_MS_PLATFORMID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_SUBSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFCFP_SUBSET1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFMFP_DELTA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFMFP_SUBSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTFMFP_SUBSET1: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTLOADINFO {
    pub usStructSize: u16,
    pub usRefStrSize: u16,
    pub pusRefStr: *mut u16,
}
impl ::core::marker::Copy for TTLOADINFO {}
impl ::core::clone::Clone for TTLOADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type TTLOAD_EMBEDDED_FONT_STATUS = u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_FONT_SUBSETTED: TTLOAD_EMBEDDED_FONT_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_FONT_IN_SYSSTARTUP: TTLOAD_EMBEDDED_FONT_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_EUDC_OVERWRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_EUDC_SET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TTLOAD_PRIVATE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTPOLYCURVE {
    pub wType: u16,
    pub cpfx: u16,
    pub apfx: [POINTFX; 1],
}
impl ::core::marker::Copy for TTPOLYCURVE {}
impl ::core::clone::Clone for TTPOLYCURVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTPOLYGONHEADER {
    pub cb: u32,
    pub dwType: u32,
    pub pfxStart: POINTFX,
}
impl ::core::marker::Copy for TTPOLYGONHEADER {}
impl ::core::clone::Clone for TTPOLYGONHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTVALIDATIONTESTSPARAMS {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pusCharCodeSet: *mut u16,
}
impl ::core::marker::Copy for TTVALIDATIONTESTSPARAMS {}
impl ::core::clone::Clone for TTVALIDATIONTESTSPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct TTVALIDATIONTESTSPARAMSEX {
    pub ulStructSize: u32,
    pub lTestFromSize: i32,
    pub lTestToSize: i32,
    pub ulCharSet: u32,
    pub usReserved1: u16,
    pub usCharCodeCount: u16,
    pub pulCharCodeSet: *mut u32,
}
impl ::core::marker::Copy for TTVALIDATIONTESTSPARAMSEX {}
impl ::core::clone::Clone for TTVALIDATIONTESTSPARAMSEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_POLYGON_TYPE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_PRIM_CSPLINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_PRIM_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TT_PRIM_QSPLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const TURKISH_CHARSET: u32 = 162u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VARIABLE_PITCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const VIETNAMESE_CHARSET: u32 = 163u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct WCRANGE {
    pub wcLow: u16,
    pub cGlyphs: u16,
}
impl ::core::marker::Copy for WCRANGE {}
impl ::core::clone::Clone for WCRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct WGLSWAP {
    pub hdc: HDC,
    pub uiFlags: u32,
}
impl ::core::marker::Copy for WGLSWAP {}
impl ::core::clone::Clone for WGLSWAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_FONT_LINES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_FONT_POLYGONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAPMULTIPLE_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_MAIN_PLANE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY10: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY11: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY12: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY13: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY14: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY15: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY5: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY6: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY7: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY8: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_OVERLAY9: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY1: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY10: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY11: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY12: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY13: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY14: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY15: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY2: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY3: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY4: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY6: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY7: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY8: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub const WGL_SWAP_UNDERLAY9: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub type WRITEEMBEDPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *const ::core::ffi::c_void, param2: u32) -> u32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
pub struct XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
impl ::core::marker::Copy for XFORM {}
impl ::core::clone::Clone for XFORM {
    fn clone(&self) -> Self {
        *self
    }
}
