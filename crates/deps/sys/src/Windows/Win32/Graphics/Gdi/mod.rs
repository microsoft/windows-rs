#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontMemResourceEx(pfileview: *const ::core::ffi::c_void, cjsize: u32, pvresrved: *mut ::core::ffi::c_void, pnumfonts: *const u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceA(param0: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceExA(name: super::super::Foundation::PSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceExW(name: super::super::Foundation::PWSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceW(param0: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AngleArc(hdc: HDC, x: i32, y: i32, r: u32, startangle: f32, sweepangle: f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnimatePalette(hpal: HPALETTE, istartindex: u32, centries: u32, ppe: *const PALETTEENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Arc(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ArcTo(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPaint(hwnd: super::super::Foundation::HWND, lppaint: *mut PAINTSTRUCT) -> HDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BitBlt(hdc: HDC, x: i32, y: i32, cx: i32, cy: i32, hdcsrc: HDC, x1: i32, y1: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDC(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsA(lpdevmode: *const DEVMODEA, dwflags: CDS_TYPE) -> DISP_CHANGE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExA(lpszdevicename: super::super::Foundation::PSTR, lpdevmode: *const DEVMODEA, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExW(lpszdevicename: super::super::Foundation::PWSTR, lpdevmode: *const DEVMODEW, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsW(lpdevmode: *const DEVMODEW, dwflags: CDS_TYPE) -> DISP_CHANGE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Chord(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClientToScreen(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CloseEnhMetaFile(hdc: HDC) -> HENHMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseFigure(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CloseMetaFile(hdc: HDC) -> HMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CombineRgn(hrgndst: HRGN, hrgnsrc1: HRGN, hrgnsrc2: HRGN, imode: RGN_COMBINE_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CombineTransform(lpxfout: *mut XFORM, lpxf1: *const XFORM, lpxf2: *const XFORM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyEnhMetaFileA(henh: HENHMETAFILE, lpfilename: super::super::Foundation::PSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyEnhMetaFileW(henh: HENHMETAFILE, lpfilename: super::super::Foundation::PWSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyMetaFileA(param0: HMETAFILE, param1: super::super::Foundation::PSTR) -> HMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyMetaFileW(param0: HMETAFILE, param1: super::super::Foundation::PWSTR) -> HMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateBitmap(nwidth: i32, nheight: i32, nplanes: u32, nbitcount: u32, lpbits: *const ::core::ffi::c_void) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateBitmapIndirect(pbm: *const BITMAP) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateBrushIndirect(plbrush: *const LOGBRUSH) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateCompatibleBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateCompatibleDC(hdc: HDC) -> CreatedHDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCA(pwszdriver: super::super::Foundation::PSTR, pwszdevice: super::super::Foundation::PSTR, pszport: super::super::Foundation::PSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCW(pwszdriver: super::super::Foundation::PWSTR, pwszdevice: super::super::Foundation::PWSTR, pszport: super::super::Foundation::PWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDIBPatternBrush(h: isize, iusage: DIB_USAGE) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDIBPatternBrushPt(lppackeddib: *const ::core::ffi::c_void, iusage: DIB_USAGE) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDIBSection(hdc: HDC, pbmi: *const BITMAPINFO, usage: DIB_USAGE, ppvbits: *mut *mut ::core::ffi::c_void, hsection: super::super::Foundation::HANDLE, offset: u32) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDIBitmap(hdc: HDC, pbmih: *const BITMAPINFOHEADER, flinit: u32, pjbits: *const ::core::ffi::c_void, pbmi: *const BITMAPINFO, iusage: DIB_USAGE) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDiscardableBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEllipticRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileA(hdc: HDC, lpfilename: super::super::Foundation::PSTR, lprc: *const super::super::Foundation::RECT, lpdesc: super::super::Foundation::PSTR) -> HdcMetdataEnhFileHandle;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileW(hdc: HDC, lpfilename: super::super::Foundation::PWSTR, lprc: *const super::super::Foundation::RECT, lpdesc: super::super::Foundation::PWSTR) -> HdcMetdataEnhFileHandle;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontA(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: super::super::Foundation::PSTR) -> HFONT;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectExA(param0: *const ENUMLOGFONTEXDVA) -> HFONT;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateFontIndirectExW(param0: *const ENUMLOGFONTEXDVW) -> HFONT;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateFontIndirectW(lplf: *const LOGFONTW) -> HFONT;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateFontPackage(
        puchsrcbuffer: *const u8,
        ulsrcbuffersize: u32,
        ppuchfontpackagebuffer: *mut *mut u8,
        pulfontpackagebuffersize: *mut u32,
        pulbyteswritten: *mut u32,
        usflag: u16,
        usttcindex: u16,
        ussubsetformat: u16,
        ussubsetlanguage: u16,
        ussubsetplatform: CREATE_FONT_PACKAGE_SUBSET_PLATFORM,
        ussubsetencoding: CREATE_FONT_PACKAGE_SUBSET_ENCODING,
        pussubsetkeeplist: *const u16,
        ussubsetlistcount: u16,
        lpfnallocate: ::windows::runtime::RawPtr,
        lpfnreallocate: ::windows::runtime::RawPtr,
        lpfnfree: ::windows::runtime::RawPtr,
        lpvreserved: *mut ::core::ffi::c_void,
    ) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontW(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: super::super::Foundation::PWSTR) -> HFONT;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateHalftonePalette(hdc: HDC) -> HPALETTE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateHatchBrush(ihatch: HATCH_BRUSH_STYLE, color: u32) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICA(pszdriver: super::super::Foundation::PSTR, pszdevice: super::super::Foundation::PSTR, pszport: super::super::Foundation::PSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICW(pszdriver: super::super::Foundation::PWSTR, pszdevice: super::super::Foundation::PWSTR, pszport: super::super::Foundation::PWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMetaFileA(pszfile: super::super::Foundation::PSTR) -> HdcMetdataFileHandle;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMetaFileW(pszfile: super::super::Foundation::PWSTR) -> HdcMetdataFileHandle;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreatePalette(plpal: *const LOGPALETTE) -> HPALETTE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreatePatternBrush(hbm: HBITMAP) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreatePen(istyle: PEN_STYLE, cwidth: i32, color: u32) -> HPEN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePenIndirect(plpen: *const LOGPEN) -> HPEN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolyPolygonRgn(pptl: *const super::super::Foundation::POINT, pc: *const i32, cpoly: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolygonRgn(pptl: *const super::super::Foundation::POINT, cpoint: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRectRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceA(fdwhidden: u32, lpszfont: super::super::Foundation::PSTR, lpszfile: super::super::Foundation::PSTR, lpszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceW(fdwhidden: u32, lpszfont: super::super::Foundation::PWSTR, lpszfile: super::super::Foundation::PWSTR, lpszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateSolidBrush(color: u32) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPtoLP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteDC(hdc: CreatedHDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteEnhMetaFile(hmf: HENHMETAFILE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMetaFile(hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteObject(ho: HGDIOBJ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawAnimatedRects(hwnd: super::super::Foundation::HWND, idani: i32, lprcfrom: *const super::super::Foundation::RECT, lprcto: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawCaption(hwnd: super::super::Foundation::HWND, hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: DRAW_CAPTION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEdge(hdc: HDC, qrc: *mut super::super::Foundation::RECT, edge: DRAWEDGE_FLAGS, grfflags: DRAW_EDGE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEscape(hdc: HDC, iescape: i32, cjin: i32, lpin: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFocusRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFrameControl(param0: HDC, param1: *mut super::super::Foundation::RECT, param2: DFC_TYPE, param3: DFCS_STATE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateA(hdc: HDC, hbrfore: HBRUSH, qfncallback: ::windows::runtime::RawPtr, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateW(hdc: HDC, hbrfore: HBRUSH, qfncallback: ::windows::runtime::RawPtr, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextA(hdc: HDC, lpchtext: super::super::Foundation::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExA(hdc: HDC, lpchtext: super::super::Foundation::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExW(hdc: HDC, lpchtext: super::super::Foundation::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextW(hdc: HDC, lpchtext: super::super::Foundation::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Ellipse(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPaint(hwnd: super::super::Foundation::HWND, lppaint: *const PAINTSTRUCT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesA(lpdevice: super::super::Foundation::PSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEA, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesW(lpdevice: super::super::Foundation::PWSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEW, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayMonitors(hdc: HDC, lprcclip: *const super::super::Foundation::RECT, lpfnenum: ::windows::runtime::RawPtr, dwdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsA(lpszdevicename: super::super::Foundation::PSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExA(lpszdevicename: super::super::Foundation::PSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExW(lpszdevicename: super::super::Foundation::PWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsW(lpszdevicename: super::super::Foundation::PWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, proc: ::windows::runtime::RawPtr, param3: *const ::core::ffi::c_void, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesA(hdc: HDC, lplogfont: super::super::Foundation::PSTR, lpproc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExA(hdc: HDC, lplogfont: *const LOGFONTA, lpproc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExW(hdc: HDC, lplogfont: *const LOGFONTW, lpproc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesW(hdc: HDC, lplogfont: super::super::Foundation::PWSTR, lpproc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsA(hdc: HDC, lplogfont: super::super::Foundation::PSTR, lpproc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsW(hdc: HDC, lplogfont: super::super::Foundation::PWSTR, lpproc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMetaFile(hdc: HDC, hmf: HMETAFILE, proc: ::windows::runtime::RawPtr, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumObjects(hdc: HDC, ntype: OBJ_TYPE, lpfunc: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRect(lprc1: *const super::super::Foundation::RECT, lprc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRgn(hrgn1: HRGN, hrgn2: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn ExcludeClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExcludeUpdateRgn(hdc: HDC, hwnd: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn ExtCreatePen(ipenstyle: PEN_STYLE, cwidth: u32, plbrush: *const LOGBRUSH, cstyle: u32, pstyle: *const u32) -> HPEN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtCreateRegion(lpx: *const XFORM, ncount: u32, lpdata: *const RGNDATA) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtFloodFill(hdc: HDC, x: i32, y: i32, color: u32, r#type: EXT_FLOOD_FILL_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn ExtSelectClipRgn(hdc: HDC, hrgn: HRGN, mode: RGN_COMBINE_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutA(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: super::super::Foundation::PSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutW(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: super::super::Foundation::PWSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FixBrushOrgEx(hdc: HDC, x: i32, y: i32, ptl: *const super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlattenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FloodFill(hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH, w: i32, h: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiAlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiComment(hdc: HDC, nsize: u32, lpdata: *const u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiFlush() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GdiGetBatchLimit() -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, ncount: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GdiSetBatchLimit(dw: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiTransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetArcDirection(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAspectRatioFilterEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetBitmapBits(hbit: HBITMAP, cb: i32, lpvbits: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBitmapDimensionEx(hbit: HBITMAP, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetBkColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetBkMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBoundsRect(hdc: HDC, lprect: *mut super::super::Foundation::RECT, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBrushOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsA(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, pabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsW(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32A(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32W(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, piwidths: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharacterPlacementA(hdc: HDC, lpstring: super::super::Foundation::PSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSA, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharacterPlacementW(hdc: HDC, lpstring: super::super::Foundation::PWSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSW, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipBox(hdc: HDC, lprect: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorAdjustment(hdc: HDC, lpca: *mut COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetCurrentObject(hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPositionEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDC(hwnd: super::super::Foundation::HWND) -> HDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDCBrushColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCEx(hwnd: super::super::Foundation::HWND, hrgnclip: HRGN, flags: GET_DCX_FLAGS) -> HDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDCPenColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *mut RGBQUAD) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpvbits: *mut ::core::ffi::c_void, lpbmi: *mut BITMAPINFO, usage: DIB_USAGE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDeviceCaps(hdc: HDC, index: GET_DEVICE_CAPS_INDEX) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileA(lpname: super::super::Foundation::PSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetEnhMetaFileBits(hemf: HENHMETAFILE, nsize: u32, lpdata: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileDescriptionA(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileDescriptionW(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileHeader(hemf: HENHMETAFILE, nsize: u32, lpenhmetaheader: *mut ENHMETAHEADER) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetEnhMetaFilePaletteEntries(hemf: HENHMETAFILE, nnumentries: u32, lppaletteentries: *mut PALETTEENTRY) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileW(lpname: super::super::Foundation::PWSTR) -> HENHMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetFontData(hdc: HDC, dwtable: u32, dwoffset: u32, pvbuffer: *mut ::core::ffi::c_void, cjbuffer: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetFontLanguageInfo(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetFontUnicodeRanges(hdc: HDC, lpgs: *mut GLYPHSET) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphIndicesA(hdc: HDC, lpstr: super::super::Foundation::PSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphIndicesW(hdc: HDC, lpstr: super::super::Foundation::PWSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineA(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineW(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetGraphicsMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetKerningPairsA(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetKerningPairsW(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetLayout(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetMapMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMetaFileA(lpname: super::super::Foundation::PSTR) -> HMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetMetaFileBitsEx(hmf: HMETAFILE, cbbuffer: u32, lpdata: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMetaFileW(lpname: super::super::Foundation::PWSTR) -> HMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetMetaRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMiterLimit(hdc: HDC, plimit: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoA(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetNearestColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetNearestPaletteIndex(h: HPALETTE, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetObjectA(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetObjectType(h: HGDIOBJ) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetObjectW(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsA(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICA) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsW(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICW) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPath(hdc: HDC, apt: *mut super::super::Foundation::POINT, aj: *mut u8, cpt: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetPixel(hdc: HDC, x: i32, y: i32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetPolyFillMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetROP2(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetRandomRgn(hdc: HDC, hrgn: HRGN, i: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRasterizerCaps(lpraststat: *mut RASTERIZER_STATUS, cjbytes: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegionData(hrgn: HRGN, ncount: u32, lprgndata: *mut RGNDATA) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRgnBox(hrgn: HRGN, lprc: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetStretchBltMode(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetSysColorBrush(nindex: i32) -> HBRUSH;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetSystemPaletteEntries(hdc: HDC, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetSystemPaletteUse(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTabbedTextExtentA(hdc: HDC, lpstring: super::super::Foundation::PSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTabbedTextExtentW(hdc: HDC, lpstring: super::super::Foundation::PWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetTextAlign(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetTextCharacterExtra(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetTextColor(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointA(hdc: HDC, lpszstring: super::super::Foundation::PSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointI(hdc: HDC, lpwszstring: *const u16, cwchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointW(hdc: HDC, lpszstring: super::super::Foundation::PWSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32A(hdc: HDC, lpstring: super::super::Foundation::PSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32W(hdc: HDC, lpstring: super::super::Foundation::PWSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointA(hdc: HDC, lpstring: super::super::Foundation::PSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointI(hdc: HDC, pgiin: *const u16, cgi: i32, psize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointW(hdc: HDC, lpstring: super::super::Foundation::PWSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextFaceA(hdc: HDC, c: i32, lpname: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextFaceW(hdc: HDC, c: i32, lpname: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsA(hdc: HDC, lptm: *mut TEXTMETRICA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsW(hdc: HDC, lptm: *mut TEXTMETRICW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetWinMetaFileBits(hemf: HENHMETAFILE, cbdata16: u32, pdata16: *mut u8, imapmode: i32, hdcref: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDC(hwnd: super::super::Foundation::HWND) -> HDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgnBox(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWorldTransform(hdc: HDC, lpxf: *mut XFORM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, nmesh: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringA(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: ::windows::runtime::RawPtr, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringW(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: ::windows::runtime::RawPtr, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InflateRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn IntersectClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntersectRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsRectEmpty(lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LPtoDP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineDDA(xstart: i32, ystart: i32, xend: i32, yend: i32, lpproc: ::windows::runtime::RawPtr, data: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineTo(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapA(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: super::super::Foundation::PSTR) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapW(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: super::super::Foundation::PWSTR) -> HBITMAP;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockWindowUpdate(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapWindowPoints(hwndfrom: super::super::Foundation::HWND, hwndto: super::super::Foundation::HWND, lppoints: *mut super::super::Foundation::POINT, cpoints: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MaskBlt(hdcdest: HDC, xdest: i32, ydest: i32, width: i32, height: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32, rop: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn MergeFontPackage(puchmergefontbuffer: *const u8, ulmergefontbuffersize: u32, puchfontpackagebuffer: *const u8, ulfontpackagebuffersize: u32, ppuchdestbuffer: *mut *mut u8, puldestbuffersize: *mut u32, pulbyteswritten: *mut u32, usmode: u16, lpfnallocate: ::windows::runtime::RawPtr, lpfnreallocate: ::windows::runtime::RawPtr, lpfnfree: ::windows::runtime::RawPtr, lpvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyWorldTransform(hdc: HDC, lpxf: *const XFORM, mode: MODIFY_WORLD_TRANSFORM_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromPoint(pt: super::super::Foundation::POINT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromRect(lprc: *const super::super::Foundation::RECT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromWindow(hwnd: super::super::Foundation::HWND, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveToEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn OffsetClipRgn(hdc: HDC, x: i32, y: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn OffsetRgn(hrgn: HRGN, x: i32, y: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintDesktop(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn PathToRegion(hdc: HDC) -> HRGN;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Pie(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFileRecord(hdc: HDC, pht: *const HANDLETABLE, pmr: *const ENHMETARECORD, cht: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFile(hdc: HDC, hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFileRecord(hdc: HDC, lphandletable: *const HANDLETABLE, lpmr: *const METARECORD, noobjs: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlgBlt(hdcdest: HDC, lppoint: *const super::super::Foundation::POINT, hdcsrc: HDC, xsrc: i32, ysrc: i32, width: i32, height: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezier(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezierTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyDraw(hdc: HDC, apt: *const super::super::Foundation::POINT, aj: *const u8, cpt: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolygon(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const i32, csz: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolyline(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const u32, csz: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutA(hdc: HDC, ppt: *const POLYTEXTA, nstrings: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutW(hdc: HDC, ppt: *const POLYTEXTW, nstrings: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polygon(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polyline(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolylineTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRect(lprc: *const super::super::Foundation::RECT, pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRegion(hrgn: HRGN, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtVisible(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn RealizePalette(hdc: HDC) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectInRegion(hrgn: HRGN, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectVisible(hdc: HDC, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Rectangle(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RedrawWindow(hwnd: super::super::Foundation::HWND, lprcupdate: *const super::super::Foundation::RECT, hrgnupdate: HRGN, flags: REDRAW_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseDC(hwnd: super::super::Foundation::HWND, hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontMemResourceEx(h: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceA(lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExA(name: super::super::Foundation::PSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExW(name: super::super::Foundation::PWSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceW(lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCA(hdc: HDC, lpdm: *const DEVMODEA) -> HDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCW(hdc: HDC, lpdm: *const DEVMODEW) -> HDC;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResizePalette(hpal: HPALETTE, n: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreDC(hdc: HDC, nsaveddc: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoundRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SaveDC(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleViewportExtEx(hdc: HDC, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleWindowExtEx(hdc: HDC, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScreenToClient(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectClipPath(hdc: HDC, mode: RGN_COMBINE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SelectClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectPalette(hdc: HDC, hpal: HPALETTE, bforcebkgd: super::super::Foundation::BOOL) -> HPALETTE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetArcDirection(hdc: HDC, dir: ARC_DIRECTION) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetBitmapBits(hbm: HBITMAP, cb: u32, pvbits: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBitmapDimensionEx(hbm: HBITMAP, w: i32, h: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetBkColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetBkMode(hdc: HDC, mode: BACKGROUND_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBoundsRect(hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: SET_BOUNDS_RECT_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorAdjustment(hdc: HDC, lpca: *const COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDCBrushColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDCPenColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *const RGBQUAD) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDIBitsToDevice(hdc: HDC, xdest: i32, ydest: i32, w: u32, h: u32, xsrc: i32, ysrc: i32, startscan: u32, clines: u32, lpvbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetEnhMetaFileBits(nsize: u32, pb: *const u8) -> HENHMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetGraphicsMode(hdc: HDC, imode: GRAPHICS_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetLayout(hdc: HDC, l: DC_LAYOUT) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMapMode(hdc: HDC, imode: HDC_MAP_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMapperFlags(hdc: HDC, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMetaFileBitsEx(cbbuffer: u32, lpdata: *const u8) -> HMETAFILE;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMetaRgn(hdc: HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMiterLimit(hdc: HDC, limit: f32, old: *mut f32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *const PALETTEENTRY) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetPixel(hdc: HDC, x: i32, y: i32, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPixelV(hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetPolyFillMode(hdc: HDC, mode: CREATE_POLYGON_RGN_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetROP2(hdc: HDC, rop2: R2_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRect(lprc: *mut super::super::Foundation::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectEmpty(lprc: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectRgn(hrgn: HRGN, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetStretchBltMode(hdc: HDC, mode: STRETCH_BLT_MODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetSystemPaletteUse(hdc: HDC, r#use: SYSTEM_PALETTE_USE) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetTextAlign(hdc: HDC, align: TEXT_ALIGN_OPTIONS) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetTextCharacterExtra(hdc: HDC, extra: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetTextColor(hdc: HDC, color: u32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextJustification(hdc: HDC, extra: i32, count: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, bredraw: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWorldTransform(hdc: HDC, lpxf: *const XFORM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StretchBlt(hdcdest: HDC, xdest: i32, ydest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, wsrc: i32, hsrc: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn StretchDIBits(hdc: HDC, xdest: i32, ydest: i32, destwidth: i32, destheight: i32, xsrc: i32, ysrc: i32, srcwidth: i32, srcheight: i32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, iusage: DIB_USAGE, rop: ROP_CODE) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokeAndFillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokePath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SubtractRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTCharToUnicode(hdc: HDC, puccharcodes: *const u8, ulcharcodesize: u32, pusshortcodes: *mut u16, ulshortcodesize: u32, ulflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTDeleteEmbeddedFont(hfontreference: super::super::Foundation::HANDLE, ulflags: u32, pulstatus: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTEmbedFont(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: ::windows::runtime::RawPtr, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTEmbedFontEx(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: ::windows::runtime::RawPtr, lpvwritestream: *const ::core::ffi::c_void, pulcharcodeset: *const u32, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEmbedFontFromFileA(hdc: HDC, szfontfilename: super::super::Foundation::PSTR, usttcindex: u16, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: ::windows::runtime::RawPtr, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEnableEmbeddingForFacename(lpszfacename: super::super::Foundation::PSTR, benable: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTGetEmbeddedFontInfo(ulflags: TTEMBED_FLAGS, pulprivstatus: *mut u32, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut u32, lpfnreadfromstream: ::windows::runtime::RawPtr, lpvreadstream: *const ::core::ffi::c_void, pttloadinfo: *const TTLOADINFO) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTGetEmbeddingType(hdc: HDC, pulembedtype: *mut EMBEDDED_FONT_PRIV_STATUS) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTGetNewFontName(phfontreference: *const super::super::Foundation::HANDLE, wzwinfamilyname: super::super::Foundation::PWSTR, cchmaxwinname: i32, szmacfamilyname: super::super::Foundation::PSTR, cchmaxmacname: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabled(hdc: HDC, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabledForFacename(lpszfacename: super::super::Foundation::PSTR, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTLoadEmbeddedFont(phfontreference: *mut super::super::Foundation::HANDLE, ulflags: u32, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream: ::windows::runtime::RawPtr, lpvreadstream: *const ::core::ffi::c_void, szwinfamilyname: super::super::Foundation::PWSTR, szmacfamilyname: super::super::Foundation::PSTR, pttloadinfo: *const TTLOADINFO) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTRunValidationTests(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTRunValidationTestsEx(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMSEX) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TabbedTextOutA(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TabbedTextOutW(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutA(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PSTR, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutW(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PWSTR, c: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnionRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnrealizeObject(h: HGDIOBJ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateColors(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WidenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromDC(hdc: HDC) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn wglSwapMultipleBuffers(param0: u32, param1: *const WGLSWAP) -> u32;
}
