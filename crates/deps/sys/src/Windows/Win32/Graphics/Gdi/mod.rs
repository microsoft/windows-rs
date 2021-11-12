#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontMemResourceEx(pfileview: *const ::core::ffi::c_void, cjsize: u32, pvresrved: *mut ::core::ffi::c_void, pnumfonts: *const u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceA(param0: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceExA(name: super::super::Foundation::PSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceExW(name: super::super::Foundation::PWSTR, fl: FONT_RESOURCE_CHARACTERISTICS, res: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceW(param0: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AngleArc(hdc: HDC, x: i32, y: i32, r: u32, startangle: f32, sweepangle: f32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnimatePalette(hpal: HPALETTE, istartindex: u32, centries: u32, ppe: *const PALETTEENTRY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Arc(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ArcTo(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPaint(hwnd: super::super::Foundation::HWND, lppaint: *mut PAINTSTRUCT) -> HDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BitBlt(hdc: HDC, x: i32, y: i32, cx: i32, cy: i32, hdcsrc: HDC, x1: i32, y1: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDC(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsA(lpdevmode: *const DEVMODEA, dwflags: CDS_TYPE) -> DISP_CHANGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExA(lpszdevicename: super::super::Foundation::PSTR, lpdevmode: *const DEVMODEA, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExW(lpszdevicename: super::super::Foundation::PWSTR, lpdevmode: *const DEVMODEW, hwnd: super::super::Foundation::HWND, dwflags: CDS_TYPE, lparam: *const ::core::ffi::c_void) -> DISP_CHANGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsW(lpdevmode: *const DEVMODEW, dwflags: CDS_TYPE) -> DISP_CHANGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Chord(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClientToScreen(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    pub fn CloseEnhMetaFile(hdc: HDC) -> HENHMETAFILE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseFigure(hdc: HDC) -> super::super::Foundation::BOOL;
    pub fn CloseMetaFile(hdc: HDC) -> HMETAFILE;
    pub fn CombineRgn(hrgndst: HRGN, hrgnsrc1: HRGN, hrgnsrc2: HRGN, imode: RGN_COMBINE_MODE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CombineTransform(lpxfout: *mut XFORM, lpxf1: *const XFORM, lpxf2: *const XFORM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyEnhMetaFileA(henh: HENHMETAFILE, lpfilename: super::super::Foundation::PSTR) -> HENHMETAFILE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyEnhMetaFileW(henh: HENHMETAFILE, lpfilename: super::super::Foundation::PWSTR) -> HENHMETAFILE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyMetaFileA(param0: HMETAFILE, param1: super::super::Foundation::PSTR) -> HMETAFILE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyMetaFileW(param0: HMETAFILE, param1: super::super::Foundation::PWSTR) -> HMETAFILE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    pub fn CreateBitmap(nwidth: i32, nheight: i32, nplanes: u32, nbitcount: u32, lpbits: *const ::core::ffi::c_void) -> HBITMAP;
    pub fn CreateBitmapIndirect(pbm: *const BITMAP) -> HBITMAP;
    pub fn CreateBrushIndirect(plbrush: *const LOGBRUSH) -> HBRUSH;
    pub fn CreateCompatibleBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    pub fn CreateCompatibleDC(hdc: HDC) -> CreatedHDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCA(pwszdriver: super::super::Foundation::PSTR, pwszdevice: super::super::Foundation::PSTR, pszport: super::super::Foundation::PSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCW(pwszdriver: super::super::Foundation::PWSTR, pwszdevice: super::super::Foundation::PWSTR, pszport: super::super::Foundation::PWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    pub fn CreateDIBPatternBrush(h: isize, iusage: DIB_USAGE) -> HBRUSH;
    pub fn CreateDIBPatternBrushPt(lppackeddib: *const ::core::ffi::c_void, iusage: DIB_USAGE) -> HBRUSH;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDIBSection(hdc: HDC, pbmi: *const BITMAPINFO, usage: DIB_USAGE, ppvbits: *mut *mut ::core::ffi::c_void, hsection: super::super::Foundation::HANDLE, offset: u32) -> HBITMAP;
    pub fn CreateDIBitmap(hdc: HDC, pbmih: *const BITMAPINFOHEADER, flinit: u32, pjbits: *const ::core::ffi::c_void, pbmi: *const BITMAPINFO, iusage: DIB_USAGE) -> HBITMAP;
    pub fn CreateDiscardableBitmap(hdc: HDC, cx: i32, cy: i32) -> HBITMAP;
    pub fn CreateEllipticRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEllipticRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileA(hdc: HDC, lpfilename: super::super::Foundation::PSTR, lprc: *const super::super::Foundation::RECT, lpdesc: super::super::Foundation::PSTR) -> HdcMetdataEnhFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileW(hdc: HDC, lpfilename: super::super::Foundation::PWSTR, lprc: *const super::super::Foundation::RECT, lpdesc: super::super::Foundation::PWSTR) -> HdcMetdataEnhFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontA(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: super::super::Foundation::PSTR) -> HFONT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectA(lplf: *const LOGFONTA) -> HFONT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectExA(param0: *const ENUMLOGFONTEXDVA) -> HFONT;
    pub fn CreateFontIndirectExW(param0: *const ENUMLOGFONTEXDVW) -> HFONT;
    pub fn CreateFontIndirectW(lplf: *const LOGFONTW) -> HFONT;
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
        lpfnallocate: CFP_ALLOCPROC,
        lpfnreallocate: CFP_REALLOCPROC,
        lpfnfree: CFP_FREEPROC,
        lpvreserved: *mut ::core::ffi::c_void,
    ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontW(cheight: i32, cwidth: i32, cescapement: i32, corientation: i32, cweight: i32, bitalic: u32, bunderline: u32, bstrikeout: u32, icharset: u32, ioutprecision: FONT_OUTPUT_PRECISION, iclipprecision: FONT_CLIP_PRECISION, iquality: FONT_QUALITY, ipitchandfamily: FONT_PITCH_AND_FAMILY, pszfacename: super::super::Foundation::PWSTR) -> HFONT;
    pub fn CreateHalftonePalette(hdc: HDC) -> HPALETTE;
    pub fn CreateHatchBrush(ihatch: HATCH_BRUSH_STYLE, color: u32) -> HBRUSH;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICA(pszdriver: super::super::Foundation::PSTR, pszdevice: super::super::Foundation::PSTR, pszport: super::super::Foundation::PSTR, pdm: *const DEVMODEA) -> CreatedHDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICW(pszdriver: super::super::Foundation::PWSTR, pszdevice: super::super::Foundation::PWSTR, pszport: super::super::Foundation::PWSTR, pdm: *const DEVMODEW) -> CreatedHDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMetaFileA(pszfile: super::super::Foundation::PSTR) -> HdcMetdataFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMetaFileW(pszfile: super::super::Foundation::PWSTR) -> HdcMetdataFileHandle;
    pub fn CreatePalette(plpal: *const LOGPALETTE) -> HPALETTE;
    pub fn CreatePatternBrush(hbm: HBITMAP) -> HBRUSH;
    pub fn CreatePen(istyle: PEN_STYLE, cwidth: i32, color: u32) -> HPEN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePenIndirect(plpen: *const LOGPEN) -> HPEN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolyPolygonRgn(pptl: *const super::super::Foundation::POINT, pc: *const i32, cpoly: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolygonRgn(pptl: *const super::super::Foundation::POINT, cpoint: i32, imode: CREATE_POLYGON_RGN_MODE) -> HRGN;
    pub fn CreateRectRgn(x1: i32, y1: i32, x2: i32, y2: i32) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRectRgnIndirect(lprect: *const super::super::Foundation::RECT) -> HRGN;
    pub fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceA(fdwhidden: u32, lpszfont: super::super::Foundation::PSTR, lpszfile: super::super::Foundation::PSTR, lpszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceW(fdwhidden: u32, lpszfont: super::super::Foundation::PWSTR, lpszfile: super::super::Foundation::PWSTR, lpszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    pub fn CreateSolidBrush(color: u32) -> HBRUSH;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPtoLP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteDC(hdc: CreatedHDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteEnhMetaFile(hmf: HENHMETAFILE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMetaFile(hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteObject(ho: HGDIOBJ) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawAnimatedRects(hwnd: super::super::Foundation::HWND, idani: i32, lprcfrom: *const super::super::Foundation::RECT, lprcto: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawCaption(hwnd: super::super::Foundation::HWND, hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: DRAW_CAPTION_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEdge(hdc: HDC, qrc: *mut super::super::Foundation::RECT, edge: DRAWEDGE_FLAGS, grfflags: DRAW_EDGE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEscape(hdc: HDC, iescape: i32, cjin: i32, lpin: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFocusRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFrameControl(param0: HDC, param1: *mut super::super::Foundation::RECT, param2: DFC_TYPE, param3: DFCS_STATE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateA(hdc: HDC, hbrfore: HBRUSH, qfncallback: DRAWSTATEPROC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateW(hdc: HDC, hbrfore: HBRUSH, qfncallback: DRAWSTATEPROC, ldata: super::super::Foundation::LPARAM, wdata: super::super::Foundation::WPARAM, x: i32, y: i32, cx: i32, cy: i32, uflags: DRAWSTATE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextA(hdc: HDC, lpchtext: super::super::Foundation::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExA(hdc: HDC, lpchtext: super::super::Foundation::PSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExW(hdc: HDC, lpchtext: super::super::Foundation::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT, lpdtp: *const DRAWTEXTPARAMS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextW(hdc: HDC, lpchtext: super::super::Foundation::PWSTR, cchtext: i32, lprc: *mut super::super::Foundation::RECT, format: DRAW_TEXT_FORMAT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Ellipse(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPaint(hwnd: super::super::Foundation::HWND, lppaint: *const PAINTSTRUCT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesA(lpdevice: super::super::Foundation::PSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEA, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesW(lpdevice: super::super::Foundation::PWSTR, idevnum: u32, lpdisplaydevice: *mut DISPLAY_DEVICEW, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayMonitors(hdc: HDC, lprcclip: *const super::super::Foundation::RECT, lpfnenum: MONITORENUMPROC, dwdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsA(lpszdevicename: super::super::Foundation::PSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExA(lpszdevicename: super::super::Foundation::PSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEA, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExW(lpszdevicename: super::super::Foundation::PWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsW(lpszdevicename: super::super::Foundation::PWSTR, imodenum: ENUM_DISPLAY_SETTINGS_MODE, lpdevmode: *mut DEVMODEW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, proc: ENHMFENUMPROC, param3: *const ::core::ffi::c_void, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesA(hdc: HDC, lplogfont: super::super::Foundation::PSTR, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExA(hdc: HDC, lplogfont: *const LOGFONTA, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExW(hdc: HDC, lplogfont: *const LOGFONTW, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesW(hdc: HDC, lplogfont: super::super::Foundation::PWSTR, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsA(hdc: HDC, lplogfont: super::super::Foundation::PSTR, lpproc: FONTENUMPROCA, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsW(hdc: HDC, lplogfont: super::super::Foundation::PWSTR, lpproc: FONTENUMPROCW, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMetaFile(hdc: HDC, hmf: HMETAFILE, proc: MFENUMPROC, param3: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumObjects(hdc: HDC, ntype: OBJ_TYPE, lpfunc: GOBJENUMPROC, lparam: super::super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRect(lprc1: *const super::super::Foundation::RECT, lprc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRgn(hrgn1: HRGN, hrgn2: HRGN) -> super::super::Foundation::BOOL;
    pub fn ExcludeClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExcludeUpdateRgn(hdc: HDC, hwnd: super::super::Foundation::HWND) -> i32;
    pub fn ExtCreatePen(ipenstyle: PEN_STYLE, cwidth: u32, plbrush: *const LOGBRUSH, cstyle: u32, pstyle: *const u32) -> HPEN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtCreateRegion(lpx: *const XFORM, ncount: u32, lpdata: *const RGNDATA) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtFloodFill(hdc: HDC, x: i32, y: i32, color: u32, r#type: EXT_FLOOD_FILL_TYPE) -> super::super::Foundation::BOOL;
    pub fn ExtSelectClipRgn(hdc: HDC, hrgn: HRGN, mode: RGN_COMBINE_MODE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutA(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: super::super::Foundation::PSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutW(hdc: HDC, x: i32, y: i32, options: ETO_OPTIONS, lprect: *const super::super::Foundation::RECT, lpstring: super::super::Foundation::PWSTR, c: u32, lpdx: *const i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FixBrushOrgEx(hdc: HDC, x: i32, y: i32, ptl: *const super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlattenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FloodFill(hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRect(hdc: HDC, lprc: *const super::super::Foundation::RECT, hbr: HBRUSH) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRgn(hdc: HDC, hrgn: HRGN, hbr: HBRUSH, w: i32, h: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiAlphaBlend(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, ftn: BLENDFUNCTION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiComment(hdc: HDC, nsize: u32, lpdata: *const u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiFlush() -> super::super::Foundation::BOOL;
    pub fn GdiGetBatchLimit() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, ncount: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    pub fn GdiSetBatchLimit(dw: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiTransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    pub fn GetArcDirection(hdc: HDC) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAspectRatioFilterEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    pub fn GetBitmapBits(hbit: HBITMAP, cb: i32, lpvbits: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBitmapDimensionEx(hbit: HBITMAP, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    pub fn GetBkColor(hdc: HDC) -> u32;
    pub fn GetBkMode(hdc: HDC) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBoundsRect(hdc: HDC, lprect: *mut super::super::Foundation::RECT, flags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBrushOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsA(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpabc: *mut ABCFLOAT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, pabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsW(hdc: HDC, wfirst: u32, wlast: u32, lpabc: *mut ABC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32A(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32W(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatA(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut f32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthI(hdc: HDC, gifirst: u32, cgi: u32, pgi: *const u16, piwidths: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthW(hdc: HDC, ifirst: u32, ilast: u32, lpbuffer: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharacterPlacementA(hdc: HDC, lpstring: super::super::Foundation::PSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSA, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharacterPlacementW(hdc: HDC, lpstring: super::super::Foundation::PWSTR, ncount: i32, nmexextent: i32, lpresults: *mut GCP_RESULTSW, dwflags: GET_CHARACTER_PLACEMENT_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipBox(hdc: HDC, lprect: *mut super::super::Foundation::RECT) -> i32;
    pub fn GetClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorAdjustment(hdc: HDC, lpca: *mut COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    pub fn GetCurrentObject(hdc: HDC, r#type: OBJ_TYPE) -> HGDIOBJ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPositionEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDC(hwnd: super::super::Foundation::HWND) -> HDC;
    pub fn GetDCBrushColor(hdc: HDC) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCEx(hwnd: super::super::Foundation::HWND, hrgnclip: HRGN, flags: GET_DCX_FLAGS) -> HDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCOrgEx(hdc: HDC, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    pub fn GetDCPenColor(hdc: HDC) -> u32;
    pub fn GetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *mut RGBQUAD) -> u32;
    pub fn GetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpvbits: *mut ::core::ffi::c_void, lpbmi: *mut BITMAPINFO, usage: DIB_USAGE) -> i32;
    pub fn GetDeviceCaps(hdc: HDC, index: GET_DEVICE_CAPS_INDEX) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileA(lpname: super::super::Foundation::PSTR) -> HENHMETAFILE;
    pub fn GetEnhMetaFileBits(hemf: HENHMETAFILE, nsize: u32, lpdata: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileDescriptionA(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileDescriptionW(hemf: HENHMETAFILE, cchbuffer: u32, lpdescription: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileHeader(hemf: HENHMETAFILE, nsize: u32, lpenhmetaheader: *mut ENHMETAHEADER) -> u32;
    pub fn GetEnhMetaFilePaletteEntries(hemf: HENHMETAFILE, nnumentries: u32, lppaletteentries: *mut PALETTEENTRY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileW(lpname: super::super::Foundation::PWSTR) -> HENHMETAFILE;
    pub fn GetFontData(hdc: HDC, dwtable: u32, dwoffset: u32, pvbuffer: *mut ::core::ffi::c_void, cjbuffer: u32) -> u32;
    pub fn GetFontLanguageInfo(hdc: HDC) -> u32;
    pub fn GetFontUnicodeRanges(hdc: HDC, lpgs: *mut GLYPHSET) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphIndicesA(hdc: HDC, lpstr: super::super::Foundation::PSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphIndicesW(hdc: HDC, lpstr: super::super::Foundation::PWSTR, c: i32, pgi: *mut u16, fl: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineA(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineW(hdc: HDC, uchar: u32, fuformat: GET_GLYPH_OUTLINE_FORMAT, lpgm: *mut GLYPHMETRICS, cjbuffer: u32, pvbuffer: *mut ::core::ffi::c_void, lpmat2: *const MAT2) -> u32;
    pub fn GetGraphicsMode(hdc: HDC) -> i32;
    pub fn GetKerningPairsA(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    pub fn GetKerningPairsW(hdc: HDC, npairs: u32, lpkernpair: *mut KERNINGPAIR) -> u32;
    pub fn GetLayout(hdc: HDC) -> u32;
    pub fn GetMapMode(hdc: HDC) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMetaFileA(lpname: super::super::Foundation::PSTR) -> HMETAFILE;
    pub fn GetMetaFileBitsEx(hmf: HMETAFILE, cbbuffer: u32, lpdata: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMetaFileW(lpname: super::super::Foundation::PWSTR) -> HMETAFILE;
    pub fn GetMetaRgn(hdc: HDC, hrgn: HRGN) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMiterLimit(hdc: HDC, plimit: *mut f32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoA(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> super::super::Foundation::BOOL;
    pub fn GetNearestColor(hdc: HDC, color: u32) -> u32;
    pub fn GetNearestPaletteIndex(h: HPALETTE, color: u32) -> u32;
    pub fn GetObjectA(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    pub fn GetObjectType(h: HGDIOBJ) -> u32;
    pub fn GetObjectW(h: HGDIOBJ, c: i32, pv: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsA(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsW(hdc: HDC, cjcopy: u32, potm: *mut OUTLINETEXTMETRICW) -> u32;
    pub fn GetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPath(hdc: HDC, apt: *mut super::super::Foundation::POINT, aj: *mut u8, cpt: i32) -> i32;
    pub fn GetPixel(hdc: HDC, x: i32, y: i32) -> u32;
    pub fn GetPolyFillMode(hdc: HDC) -> i32;
    pub fn GetROP2(hdc: HDC) -> i32;
    pub fn GetRandomRgn(hdc: HDC, hrgn: HRGN, i: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRasterizerCaps(lpraststat: *mut RASTERIZER_STATUS, cjbytes: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegionData(hrgn: HRGN, ncount: u32, lprgndata: *mut RGNDATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRgnBox(hrgn: HRGN, lprc: *mut super::super::Foundation::RECT) -> i32;
    pub fn GetStockObject(i: GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ;
    pub fn GetStretchBltMode(hdc: HDC) -> i32;
    pub fn GetSysColorBrush(nindex: i32) -> HBRUSH;
    pub fn GetSystemPaletteEntries(hdc: HDC, istart: u32, centries: u32, ppalentries: *mut PALETTEENTRY) -> u32;
    pub fn GetSystemPaletteUse(hdc: HDC) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTabbedTextExtentA(hdc: HDC, lpstring: super::super::Foundation::PSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTabbedTextExtentW(hdc: HDC, lpstring: super::super::Foundation::PWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32) -> u32;
    pub fn GetTextAlign(hdc: HDC) -> u32;
    pub fn GetTextCharacterExtra(hdc: HDC) -> i32;
    pub fn GetTextColor(hdc: HDC) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointA(hdc: HDC, lpszstring: super::super::Foundation::PSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointI(hdc: HDC, lpwszstring: *const u16, cwchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointW(hdc: HDC, lpszstring: super::super::Foundation::PWSTR, cchstring: i32, nmaxextent: i32, lpnfit: *mut i32, lpndx: *mut i32, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32A(hdc: HDC, lpstring: super::super::Foundation::PSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32W(hdc: HDC, lpstring: super::super::Foundation::PWSTR, c: i32, psizl: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointA(hdc: HDC, lpstring: super::super::Foundation::PSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointI(hdc: HDC, pgiin: *const u16, cgi: i32, psize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointW(hdc: HDC, lpstring: super::super::Foundation::PWSTR, c: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextFaceA(hdc: HDC, c: i32, lpname: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextFaceW(hdc: HDC, c: i32, lpname: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsA(hdc: HDC, lptm: *mut TEXTMETRICA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsW(hdc: HDC, lptm: *mut TEXTMETRICW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRect(hwnd: super::super::Foundation::HWND, lprect: *mut super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    pub fn GetWinMetaFileBits(hemf: HENHMETAFILE, cbdata16: u32, pdata16: *mut u8, imapmode: i32, hdcref: HDC) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDC(hwnd: super::super::Foundation::HWND) -> HDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowExtEx(hdc: HDC, lpsize: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowOrgEx(hdc: HDC, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgnBox(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWorldTransform(hdc: HDC, lpxf: *mut XFORM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GradientFill(hdc: HDC, pvertex: *const TRIVERTEX, nvertex: u32, pmesh: *const ::core::ffi::c_void, nmesh: u32, ulmode: GRADIENT_FILL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringA(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringW(hdc: HDC, hbrush: HBRUSH, lpoutputfunc: GRAYSTRINGPROC, lpdata: super::super::Foundation::LPARAM, ncount: i32, x: i32, y: i32, nwidth: i32, nheight: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InflateRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    pub fn IntersectClipRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntersectRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, berase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRect(hdc: HDC, lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsRectEmpty(lprc: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LPtoDP(hdc: HDC, lppt: *mut super::super::Foundation::POINT, c: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineDDA(xstart: i32, ystart: i32, xend: i32, yend: i32, lpproc: LINEDDAPROC, data: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineTo(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapA(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: super::super::Foundation::PSTR) -> HBITMAP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapW(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: super::super::Foundation::PWSTR) -> HBITMAP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockWindowUpdate(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapWindowPoints(hwndfrom: super::super::Foundation::HWND, hwndto: super::super::Foundation::HWND, lppoints: *mut super::super::Foundation::POINT, cpoints: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MaskBlt(hdcdest: HDC, xdest: i32, ydest: i32, width: i32, height: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32, rop: u32) -> super::super::Foundation::BOOL;
    pub fn MergeFontPackage(puchmergefontbuffer: *const u8, ulmergefontbuffersize: u32, puchfontpackagebuffer: *const u8, ulfontpackagebuffersize: u32, ppuchdestbuffer: *mut *mut u8, puldestbuffersize: *mut u32, pulbyteswritten: *mut u32, usmode: u16, lpfnallocate: CFP_ALLOCPROC, lpfnreallocate: CFP_REALLOCPROC, lpfnfree: CFP_FREEPROC, lpvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyWorldTransform(hdc: HDC, lpxf: *const XFORM, mode: MODIFY_WORLD_TRANSFORM_MODE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromPoint(pt: super::super::Foundation::POINT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromRect(lprc: *const super::super::Foundation::RECT, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromWindow(hwnd: super::super::Foundation::HWND, dwflags: MONITOR_FROM_FLAGS) -> HMONITOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveToEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    pub fn OffsetClipRgn(hdc: HDC, x: i32, y: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetRect(lprc: *mut super::super::Foundation::RECT, dx: i32, dy: i32) -> super::super::Foundation::BOOL;
    pub fn OffsetRgn(hrgn: HRGN, x: i32, y: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintDesktop(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintRgn(hdc: HDC, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PatBlt(hdc: HDC, x: i32, y: i32, w: i32, h: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    pub fn PathToRegion(hdc: HDC) -> HRGN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Pie(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFile(hdc: HDC, hmf: HENHMETAFILE, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFileRecord(hdc: HDC, pht: *const HANDLETABLE, pmr: *const ENHMETARECORD, cht: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFile(hdc: HDC, hmf: HMETAFILE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFileRecord(hdc: HDC, lphandletable: *const HANDLETABLE, lpmr: *const METARECORD, noobjs: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlgBlt(hdcdest: HDC, lppoint: *const super::super::Foundation::POINT, hdcsrc: HDC, xsrc: i32, ysrc: i32, width: i32, height: i32, hbmmask: HBITMAP, xmask: i32, ymask: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezier(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezierTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyDraw(hdc: HDC, apt: *const super::super::Foundation::POINT, aj: *const u8, cpt: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolygon(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const i32, csz: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolyline(hdc: HDC, apt: *const super::super::Foundation::POINT, asz: *const u32, csz: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutA(hdc: HDC, ppt: *const POLYTEXTA, nstrings: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutW(hdc: HDC, ppt: *const POLYTEXTW, nstrings: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polygon(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polyline(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolylineTo(hdc: HDC, apt: *const super::super::Foundation::POINT, cpt: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRect(lprc: *const super::super::Foundation::RECT, pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRegion(hrgn: HRGN, x: i32, y: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtVisible(hdc: HDC, x: i32, y: i32) -> super::super::Foundation::BOOL;
    pub fn RealizePalette(hdc: HDC) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectInRegion(hrgn: HRGN, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectVisible(hdc: HDC, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Rectangle(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RedrawWindow(hwnd: super::super::Foundation::HWND, lprcupdate: *const super::super::Foundation::RECT, hrgnupdate: HRGN, flags: REDRAW_WINDOW_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseDC(hwnd: super::super::Foundation::HWND, hdc: HDC) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontMemResourceEx(h: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceA(lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExA(name: super::super::Foundation::PSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExW(name: super::super::Foundation::PWSTR, fl: u32, pdv: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceW(lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCA(hdc: HDC, lpdm: *const DEVMODEA) -> HDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCW(hdc: HDC, lpdm: *const DEVMODEW) -> HDC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResizePalette(hpal: HPALETTE, n: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreDC(hdc: HDC, nsaveddc: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoundRect(hdc: HDC, left: i32, top: i32, right: i32, bottom: i32, width: i32, height: i32) -> super::super::Foundation::BOOL;
    pub fn SaveDC(hdc: HDC) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleViewportExtEx(hdc: HDC, xn: i32, dx: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleWindowExtEx(hdc: HDC, xn: i32, xd: i32, yn: i32, yd: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScreenToClient(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectClipPath(hdc: HDC, mode: RGN_COMBINE_MODE) -> super::super::Foundation::BOOL;
    pub fn SelectClipRgn(hdc: HDC, hrgn: HRGN) -> i32;
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectPalette(hdc: HDC, hpal: HPALETTE, bforcebkgd: super::super::Foundation::BOOL) -> HPALETTE;
    pub fn SetArcDirection(hdc: HDC, dir: ARC_DIRECTION) -> i32;
    pub fn SetBitmapBits(hbm: HBITMAP, cb: u32, pvbits: *const ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBitmapDimensionEx(hbm: HBITMAP, w: i32, h: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    pub fn SetBkColor(hdc: HDC, color: u32) -> u32;
    pub fn SetBkMode(hdc: HDC, mode: BACKGROUND_MODE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBoundsRect(hdc: HDC, lprect: *const super::super::Foundation::RECT, flags: SET_BOUNDS_RECT_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBrushOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorAdjustment(hdc: HDC, lpca: *const COLORADJUSTMENT) -> super::super::Foundation::BOOL;
    pub fn SetDCBrushColor(hdc: HDC, color: u32) -> u32;
    pub fn SetDCPenColor(hdc: HDC, color: u32) -> u32;
    pub fn SetDIBColorTable(hdc: HDC, istart: u32, centries: u32, prgbq: *const RGBQUAD) -> u32;
    pub fn SetDIBits(hdc: HDC, hbm: HBITMAP, start: u32, clines: u32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    pub fn SetDIBitsToDevice(hdc: HDC, xdest: i32, ydest: i32, w: u32, h: u32, xsrc: i32, ysrc: i32, startscan: u32, clines: u32, lpvbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, coloruse: DIB_USAGE) -> i32;
    pub fn SetEnhMetaFileBits(nsize: u32, pb: *const u8) -> HENHMETAFILE;
    pub fn SetGraphicsMode(hdc: HDC, imode: GRAPHICS_MODE) -> i32;
    pub fn SetLayout(hdc: HDC, l: DC_LAYOUT) -> u32;
    pub fn SetMapMode(hdc: HDC, imode: HDC_MAP_MODE) -> i32;
    pub fn SetMapperFlags(hdc: HDC, flags: u32) -> u32;
    pub fn SetMetaFileBitsEx(cbbuffer: u32, lpdata: *const u8) -> HMETAFILE;
    pub fn SetMetaRgn(hdc: HDC) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMiterLimit(hdc: HDC, limit: f32, old: *mut f32) -> super::super::Foundation::BOOL;
    pub fn SetPaletteEntries(hpal: HPALETTE, istart: u32, centries: u32, ppalentries: *const PALETTEENTRY) -> u32;
    pub fn SetPixel(hdc: HDC, x: i32, y: i32, color: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPixelV(hdc: HDC, x: i32, y: i32, color: u32) -> super::super::Foundation::BOOL;
    pub fn SetPolyFillMode(hdc: HDC, mode: CREATE_POLYGON_RGN_MODE) -> i32;
    pub fn SetROP2(hdc: HDC, rop2: R2_MODE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRect(lprc: *mut super::super::Foundation::RECT, xleft: i32, ytop: i32, xright: i32, ybottom: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectEmpty(lprc: *mut super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectRgn(hrgn: HRGN, left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::BOOL;
    pub fn SetStretchBltMode(hdc: HDC, mode: STRETCH_BLT_MODE) -> i32;
    pub fn SetSystemPaletteUse(hdc: HDC, r#use: SYSTEM_PALETTE_USE) -> u32;
    pub fn SetTextAlign(hdc: HDC, align: TEXT_ALIGN_OPTIONS) -> u32;
    pub fn SetTextCharacterExtra(hdc: HDC, extra: i32) -> i32;
    pub fn SetTextColor(hdc: HDC, color: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextJustification(hdc: HDC, extra: i32, count: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowExtEx(hdc: HDC, x: i32, y: i32, lpsz: *mut super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowOrgEx(hdc: HDC, x: i32, y: i32, lppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN, bredraw: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWorldTransform(hdc: HDC, lpxf: *const XFORM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StretchBlt(hdcdest: HDC, xdest: i32, ydest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xsrc: i32, ysrc: i32, wsrc: i32, hsrc: i32, rop: ROP_CODE) -> super::super::Foundation::BOOL;
    pub fn StretchDIBits(hdc: HDC, xdest: i32, ydest: i32, destwidth: i32, destheight: i32, xsrc: i32, ysrc: i32, srcwidth: i32, srcheight: i32, lpbits: *const ::core::ffi::c_void, lpbmi: *const BITMAPINFO, iusage: DIB_USAGE, rop: ROP_CODE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokeAndFillPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokePath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SubtractRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    pub fn TTCharToUnicode(hdc: HDC, puccharcodes: *const u8, ulcharcodesize: u32, pusshortcodes: *mut u16, ulshortcodesize: u32, ulflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTDeleteEmbeddedFont(hfontreference: super::super::Foundation::HANDLE, ulflags: u32, pulstatus: *mut u32) -> i32;
    pub fn TTEmbedFont(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    pub fn TTEmbedFontEx(hdc: HDC, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, pulcharcodeset: *const u32, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEmbedFontFromFileA(hdc: HDC, szfontfilename: super::super::Foundation::PSTR, usttcindex: u16, ulflags: TTEMBED_FLAGS, ulcharset: EMBED_FONT_CHARSET, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, pulstatus: *mut u32, lpfnwritetostream: WRITEEMBEDPROC, lpvwritestream: *const ::core::ffi::c_void, puscharcodeset: *const u16, uscharcodecount: u16, uslanguage: u16, pttembedinfo: *const TTEMBEDINFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEnableEmbeddingForFacename(lpszfacename: super::super::Foundation::PSTR, benable: super::super::Foundation::BOOL) -> i32;
    pub fn TTGetEmbeddedFontInfo(ulflags: TTEMBED_FLAGS, pulprivstatus: *mut u32, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut u32, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const ::core::ffi::c_void, pttloadinfo: *const TTLOADINFO) -> i32;
    pub fn TTGetEmbeddingType(hdc: HDC, pulembedtype: *mut EMBEDDED_FONT_PRIV_STATUS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTGetNewFontName(phfontreference: *const super::super::Foundation::HANDLE, wzwinfamilyname: super::super::Foundation::PWSTR, cchmaxwinname: i32, szmacfamilyname: super::super::Foundation::PSTR, cchmaxmacname: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabled(hdc: HDC, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabledForFacename(lpszfacename: super::super::Foundation::PSTR, pbenabled: *mut super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTLoadEmbeddedFont(phfontreference: *mut super::super::Foundation::HANDLE, ulflags: u32, pulprivstatus: *mut EMBEDDED_FONT_PRIV_STATUS, ulprivs: FONT_LICENSE_PRIVS, pulstatus: *mut TTLOAD_EMBEDDED_FONT_STATUS, lpfnreadfromstream: READEMBEDPROC, lpvreadstream: *const ::core::ffi::c_void, szwinfamilyname: super::super::Foundation::PWSTR, szmacfamilyname: super::super::Foundation::PSTR, pttloadinfo: *const TTLOADINFO) -> i32;
    pub fn TTRunValidationTests(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMS) -> i32;
    pub fn TTRunValidationTestsEx(hdc: HDC, ptestparam: *const TTVALIDATIONTESTSPARAMSEX) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TabbedTextOutA(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TabbedTextOutW(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PWSTR, chcount: i32, ntabpositions: i32, lpntabstoppositions: *const i32, ntaborigin: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutA(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PSTR, c: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutW(hdc: HDC, x: i32, y: i32, lpstring: super::super::Foundation::PWSTR, c: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransparentBlt(hdcdest: HDC, xorigindest: i32, yorigindest: i32, wdest: i32, hdest: i32, hdcsrc: HDC, xoriginsrc: i32, yoriginsrc: i32, wsrc: i32, hsrc: i32, crtransparent: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnionRect(lprcdst: *mut super::super::Foundation::RECT, lprcsrc1: *const super::super::Foundation::RECT, lprcsrc2: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnrealizeObject(h: HGDIOBJ) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateColors(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateWindow(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRect(hwnd: super::super::Foundation::HWND, lprect: *const super::super::Foundation::RECT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRgn(hwnd: super::super::Foundation::HWND, hrgn: HRGN) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WidenPath(hdc: HDC) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromDC(hdc: HDC) -> super::super::Foundation::HWND;
    pub fn wglSwapMultipleBuffers(param0: u32, param1: *const WGLSWAP) -> u32;
}
pub struct ABC(i32);
pub struct ABCFLOAT(i32);
pub const ABORTDOC: u32 = 2u32;
pub struct ABORTPATH(i32);
pub const ABSOLUTE: u32 = 1u32;
pub const AC_SRC_ALPHA: u32 = 1u32;
pub const AC_SRC_OVER: u32 = 0u32;
pub const ANSI_CHARSET: u32 = 0u32;
pub const ARABIC_CHARSET: u32 = 178u32;
pub struct ARC_DIRECTION(i32);
pub const ASPECT_FILTERING: u32 = 1u32;
pub struct AXESLISTA(i32);
pub struct AXESLISTW(i32);
pub struct AXISINFOA(i32);
pub struct AXISINFOW(i32);
pub struct BACKGROUND_MODE(i32);
pub const BALTIC_CHARSET: u32 = 186u32;
pub const BANDINFO: u32 = 24u32;
pub const BEGIN_PATH: u32 = 4096u32;
pub struct BITMAP(i32);
pub struct BITMAPCOREHEADER(i32);
pub struct BITMAPCOREINFO(i32);
pub struct BITMAPFILEHEADER(i32);
pub struct BITMAPINFO(i32);
pub struct BITMAPINFOHEADER(i32);
pub struct BITMAPV4HEADER(i32);
pub struct BITMAPV5HEADER(i32);
pub const BI_BITFIELDS: i32 = 3i32;
pub const BI_JPEG: i32 = 4i32;
pub const BI_PNG: i32 = 5i32;
pub const BI_RGB: i32 = 0i32;
pub const BI_RLE4: i32 = 2i32;
pub const BI_RLE8: i32 = 1i32;
pub const BKMODE_LAST: u32 = 2u32;
pub struct BLENDFUNCTION(i32);
pub const BS_DIBPATTERN: u32 = 5u32;
pub const BS_DIBPATTERN8X8: u32 = 8u32;
pub const BS_DIBPATTERNPT: u32 = 6u32;
pub const BS_HATCHED: u32 = 2u32;
pub const BS_HOLLOW: u32 = 1u32;
pub const BS_INDEXED: u32 = 4u32;
pub const BS_MONOPATTERN: u32 = 9u32;
pub const BS_NULL: u32 = 1u32;
pub const BS_PATTERN: u32 = 3u32;
pub const BS_PATTERN8X8: u32 = 7u32;
pub const BS_SOLID: u32 = 0u32;
pub const CA_LOG_FILTER: u32 = 2u32;
pub const CA_NEGATIVE: u32 = 1u32;
pub const CBM_INIT: i32 = 4i32;
pub const CCHFORMNAME: u32 = 32u32;
pub const CC_CHORD: u32 = 4u32;
pub const CC_CIRCLES: u32 = 1u32;
pub const CC_ELLIPSES: u32 = 8u32;
pub const CC_INTERIORS: u32 = 128u32;
pub const CC_NONE: u32 = 0u32;
pub const CC_PIE: u32 = 2u32;
pub const CC_ROUNDRECT: u32 = 256u32;
pub const CC_STYLED: u32 = 32u32;
pub const CC_WIDE: u32 = 16u32;
pub const CC_WIDESTYLED: u32 = 64u32;
pub struct CDS_TYPE(i32);
pub struct CFP_ALLOCPROC(i32);
pub struct CFP_FREEPROC(i32);
pub struct CFP_REALLOCPROC(i32);
pub const CHARSET_DEFAULT: u32 = 1u32;
pub const CHARSET_GLYPHIDX: u32 = 3u32;
pub const CHECKJPEGFORMAT: u32 = 4119u32;
pub const CHECKPNGFORMAT: u32 = 4120u32;
pub const CHINESEBIG5_CHARSET: u32 = 136u32;
pub struct CIEXYZ(i32);
pub struct CIEXYZTRIPLE(i32);
pub const CLEARTYPE_NATURAL_QUALITY: u32 = 6u32;
pub const CLIP_TO_PATH: u32 = 4097u32;
pub const CLOSECHANNEL: u32 = 4112u32;
pub const CLR_INVALID: u32 = 4294967295u32;
pub const CM_CMYK_COLOR: u32 = 4u32;
pub const CM_DEVICE_ICM: u32 = 1u32;
pub const CM_GAMMA_RAMP: u32 = 2u32;
pub const CM_IN_GAMUT: u32 = 0u32;
pub const CM_NONE: u32 = 0u32;
pub const CM_OUT_OF_GAMUT: u32 = 255u32;
pub struct COLORADJUSTMENT(i32);
pub struct COLORCORRECTPALETTE(i32);
pub struct COLORMATCHTOTARGET(i32);
pub const COLORMATCHTOTARGET_EMBEDED: u32 = 1u32;
pub const COMPLEXREGION: u32 = 3u32;
pub const CP_NONE: u32 = 0u32;
pub const CP_RECTANGLE: u32 = 1u32;
pub const CP_REGION: u32 = 2u32;
pub const CREATECOLORSPACE_EMBEDED: u32 = 1u32;
pub struct CREATE_FONT_PACKAGE_SUBSET_ENCODING(i32);
pub struct CREATE_FONT_PACKAGE_SUBSET_PLATFORM(i32);
pub struct CREATE_POLYGON_RGN_MODE(i32);
pub struct CreatedHDC(i32);
pub const DCBA_FACEDOWNCENTER: u32 = 257u32;
pub const DCBA_FACEDOWNLEFT: u32 = 258u32;
pub const DCBA_FACEDOWNNONE: u32 = 256u32;
pub const DCBA_FACEDOWNRIGHT: u32 = 259u32;
pub const DCBA_FACEUPCENTER: u32 = 1u32;
pub const DCBA_FACEUPLEFT: u32 = 2u32;
pub const DCBA_FACEUPNONE: u32 = 0u32;
pub const DCBA_FACEUPRIGHT: u32 = 3u32;
pub const DCTT_BITMAP: i32 = 1i32;
pub const DCTT_DOWNLOAD: i32 = 2i32;
pub const DCTT_DOWNLOAD_OUTLINE: i32 = 8i32;
pub const DCTT_SUBDEV: i32 = 4i32;
pub const DC_BINADJUST: u32 = 19u32;
pub const DC_DATATYPE_PRODUCED: u32 = 21u32;
pub const DC_EMF_COMPLIANT: u32 = 20u32;
pub struct DC_LAYOUT(i32);
pub const DC_MANUFACTURER: u32 = 23u32;
pub const DC_MODEL: u32 = 24u32;
pub const DEFAULT_CHARSET: u32 = 1u32;
pub const DEFAULT_PITCH: u32 = 0u32;
pub struct DESIGNVECTOR(i32);
pub const DEVICEDATA: u32 = 19u32;
pub const DEVICE_FONTTYPE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DEVMODEW(i32);
pub struct DFCS_STATE(i32);
pub struct DFC_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DIBSECTION(i32);
pub struct DIB_USAGE(i32);
pub struct DISPLAYCONFIG_COLOR_ENCODING(i32);
pub const DISPLAYCONFIG_MAXPATH: u32 = 1024u32;
pub const DISPLAYCONFIG_PATH_ACTIVE: u32 = 1u32;
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295u32;
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: u32 = 4u32;
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: u32 = 8u32;
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: u32 = 65535u32;
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: u32 = 29u32;
pub const DISPLAYCONFIG_SOURCE_IN_USE: u32 = 1u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: u32 = 4u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: u32 = 8u32;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: u32 = 16u32;
pub const DISPLAYCONFIG_TARGET_FORCIBLE: u32 = 2u32;
pub const DISPLAYCONFIG_TARGET_IN_USE: u32 = 1u32;
pub const DISPLAYCONFIG_TARGET_IS_HMD: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAY_DEVICEA(i32);
pub struct DISPLAY_DEVICEW(i32);
pub const DISPLAY_DEVICE_ACC_DRIVER: u32 = 64u32;
pub const DISPLAY_DEVICE_ACTIVE: u32 = 1u32;
pub const DISPLAY_DEVICE_ATTACHED: u32 = 2u32;
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: u32 = 1u32;
pub const DISPLAY_DEVICE_DISCONNECT: u32 = 33554432u32;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: u32 = 8u32;
pub const DISPLAY_DEVICE_MODESPRUNED: u32 = 134217728u32;
pub const DISPLAY_DEVICE_MULTI_DRIVER: u32 = 2u32;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: u32 = 4u32;
pub const DISPLAY_DEVICE_RDPUDD: u32 = 16777216u32;
pub const DISPLAY_DEVICE_REMOTE: u32 = 67108864u32;
pub const DISPLAY_DEVICE_REMOVABLE: u32 = 32u32;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: u32 = 2097152u32;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: u32 = 524288u32;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: u32 = 16u32;
pub struct DISP_CHANGE(i32);
pub const DI_APPBANDING: u32 = 1u32;
pub const DI_ROPS_READ_DESTINATION: u32 = 2u32;
pub const DMBIN_AUTO: u32 = 7u32;
pub const DMBIN_CASSETTE: u32 = 14u32;
pub const DMBIN_ENVELOPE: u32 = 5u32;
pub const DMBIN_ENVMANUAL: u32 = 6u32;
pub const DMBIN_FORMSOURCE: u32 = 15u32;
pub const DMBIN_LARGECAPACITY: u32 = 11u32;
pub const DMBIN_LARGEFMT: u32 = 10u32;
pub const DMBIN_LAST: u32 = 15u32;
pub const DMBIN_LOWER: u32 = 2u32;
pub const DMBIN_MANUAL: u32 = 4u32;
pub const DMBIN_MIDDLE: u32 = 3u32;
pub const DMBIN_ONLYONE: u32 = 1u32;
pub const DMBIN_SMALLFMT: u32 = 9u32;
pub const DMBIN_TRACTOR: u32 = 8u32;
pub const DMBIN_UPPER: u32 = 1u32;
pub const DMBIN_USER: u32 = 256u32;
pub const DMCOLLATE_FALSE: u32 = 0u32;
pub const DMCOLLATE_TRUE: u32 = 1u32;
pub const DMCOLOR_COLOR: u32 = 2u32;
pub const DMCOLOR_MONOCHROME: u32 = 1u32;
pub const DMDFO_CENTER: u32 = 2u32;
pub const DMDFO_DEFAULT: u32 = 0u32;
pub const DMDFO_STRETCH: u32 = 1u32;
pub const DMDISPLAYFLAGS_TEXTMODE: u32 = 4u32;
pub const DMDITHER_COARSE: u32 = 2u32;
pub const DMDITHER_ERRORDIFFUSION: u32 = 5u32;
pub const DMDITHER_FINE: u32 = 3u32;
pub const DMDITHER_GRAYSCALE: u32 = 10u32;
pub const DMDITHER_LINEART: u32 = 4u32;
pub const DMDITHER_NONE: u32 = 1u32;
pub const DMDITHER_RESERVED6: u32 = 6u32;
pub const DMDITHER_RESERVED7: u32 = 7u32;
pub const DMDITHER_RESERVED8: u32 = 8u32;
pub const DMDITHER_RESERVED9: u32 = 9u32;
pub const DMDITHER_USER: u32 = 256u32;
pub const DMDO_180: u32 = 2u32;
pub const DMDO_270: u32 = 3u32;
pub const DMDO_90: u32 = 1u32;
pub const DMDO_DEFAULT: u32 = 0u32;
pub const DMDUP_HORIZONTAL: u32 = 3u32;
pub const DMDUP_SIMPLEX: u32 = 1u32;
pub const DMDUP_VERTICAL: u32 = 2u32;
pub const DMICMMETHOD_DEVICE: u32 = 4u32;
pub const DMICMMETHOD_DRIVER: u32 = 3u32;
pub const DMICMMETHOD_NONE: u32 = 1u32;
pub const DMICMMETHOD_SYSTEM: u32 = 2u32;
pub const DMICMMETHOD_USER: u32 = 256u32;
pub const DMICM_ABS_COLORIMETRIC: u32 = 4u32;
pub const DMICM_COLORIMETRIC: u32 = 3u32;
pub const DMICM_CONTRAST: u32 = 2u32;
pub const DMICM_SATURATE: u32 = 1u32;
pub const DMICM_USER: u32 = 256u32;
pub const DMMEDIA_GLOSSY: u32 = 3u32;
pub const DMMEDIA_STANDARD: u32 = 1u32;
pub const DMMEDIA_TRANSPARENCY: u32 = 2u32;
pub const DMMEDIA_USER: u32 = 256u32;
pub const DMNUP_ONEUP: u32 = 2u32;
pub const DMNUP_SYSTEM: u32 = 1u32;
pub const DMORIENT_LANDSCAPE: u32 = 2u32;
pub const DMORIENT_PORTRAIT: u32 = 1u32;
pub const DMPAPER_10X11: u32 = 45u32;
pub const DMPAPER_10X14: u32 = 16u32;
pub const DMPAPER_11X17: u32 = 17u32;
pub const DMPAPER_12X11: u32 = 90u32;
pub const DMPAPER_15X11: u32 = 46u32;
pub const DMPAPER_9X11: u32 = 44u32;
pub const DMPAPER_A2: u32 = 66u32;
pub const DMPAPER_A3: u32 = 8u32;
pub const DMPAPER_A3_EXTRA: u32 = 63u32;
pub const DMPAPER_A3_EXTRA_TRANSVERSE: u32 = 68u32;
pub const DMPAPER_A3_ROTATED: u32 = 76u32;
pub const DMPAPER_A3_TRANSVERSE: u32 = 67u32;
pub const DMPAPER_A4: u32 = 9u32;
pub const DMPAPER_A4SMALL: u32 = 10u32;
pub const DMPAPER_A4_EXTRA: u32 = 53u32;
pub const DMPAPER_A4_PLUS: u32 = 60u32;
pub const DMPAPER_A4_ROTATED: u32 = 77u32;
pub const DMPAPER_A4_TRANSVERSE: u32 = 55u32;
pub const DMPAPER_A5: u32 = 11u32;
pub const DMPAPER_A5_EXTRA: u32 = 64u32;
pub const DMPAPER_A5_ROTATED: u32 = 78u32;
pub const DMPAPER_A5_TRANSVERSE: u32 = 61u32;
pub const DMPAPER_A6: u32 = 70u32;
pub const DMPAPER_A6_ROTATED: u32 = 83u32;
pub const DMPAPER_A_PLUS: u32 = 57u32;
pub const DMPAPER_B4: u32 = 12u32;
pub const DMPAPER_B4_JIS_ROTATED: u32 = 79u32;
pub const DMPAPER_B5: u32 = 13u32;
pub const DMPAPER_B5_EXTRA: u32 = 65u32;
pub const DMPAPER_B5_JIS_ROTATED: u32 = 80u32;
pub const DMPAPER_B5_TRANSVERSE: u32 = 62u32;
pub const DMPAPER_B6_JIS: u32 = 88u32;
pub const DMPAPER_B6_JIS_ROTATED: u32 = 89u32;
pub const DMPAPER_B_PLUS: u32 = 58u32;
pub const DMPAPER_CSHEET: u32 = 24u32;
pub const DMPAPER_DBL_JAPANESE_POSTCARD: u32 = 69u32;
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 82u32;
pub const DMPAPER_DSHEET: u32 = 25u32;
pub const DMPAPER_ENV_10: u32 = 20u32;
pub const DMPAPER_ENV_11: u32 = 21u32;
pub const DMPAPER_ENV_12: u32 = 22u32;
pub const DMPAPER_ENV_14: u32 = 23u32;
pub const DMPAPER_ENV_9: u32 = 19u32;
pub const DMPAPER_ENV_B4: u32 = 33u32;
pub const DMPAPER_ENV_B5: u32 = 34u32;
pub const DMPAPER_ENV_B6: u32 = 35u32;
pub const DMPAPER_ENV_C3: u32 = 29u32;
pub const DMPAPER_ENV_C4: u32 = 30u32;
pub const DMPAPER_ENV_C5: u32 = 28u32;
pub const DMPAPER_ENV_C6: u32 = 31u32;
pub const DMPAPER_ENV_C65: u32 = 32u32;
pub const DMPAPER_ENV_DL: u32 = 27u32;
pub const DMPAPER_ENV_INVITE: u32 = 47u32;
pub const DMPAPER_ENV_ITALY: u32 = 36u32;
pub const DMPAPER_ENV_MONARCH: u32 = 37u32;
pub const DMPAPER_ENV_PERSONAL: u32 = 38u32;
pub const DMPAPER_ESHEET: u32 = 26u32;
pub const DMPAPER_EXECUTIVE: u32 = 7u32;
pub const DMPAPER_FANFOLD_LGL_GERMAN: u32 = 41u32;
pub const DMPAPER_FANFOLD_STD_GERMAN: u32 = 40u32;
pub const DMPAPER_FANFOLD_US: u32 = 39u32;
pub const DMPAPER_FOLIO: u32 = 14u32;
pub const DMPAPER_ISO_B4: u32 = 42u32;
pub const DMPAPER_JAPANESE_POSTCARD: u32 = 43u32;
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 81u32;
pub const DMPAPER_JENV_CHOU3: u32 = 73u32;
pub const DMPAPER_JENV_CHOU3_ROTATED: u32 = 86u32;
pub const DMPAPER_JENV_CHOU4: u32 = 74u32;
pub const DMPAPER_JENV_CHOU4_ROTATED: u32 = 87u32;
pub const DMPAPER_JENV_KAKU2: u32 = 71u32;
pub const DMPAPER_JENV_KAKU2_ROTATED: u32 = 84u32;
pub const DMPAPER_JENV_KAKU3: u32 = 72u32;
pub const DMPAPER_JENV_KAKU3_ROTATED: u32 = 85u32;
pub const DMPAPER_JENV_YOU4: u32 = 91u32;
pub const DMPAPER_JENV_YOU4_ROTATED: u32 = 92u32;
pub const DMPAPER_LAST: u32 = 118u32;
pub const DMPAPER_LEDGER: u32 = 4u32;
pub const DMPAPER_LEGAL: u32 = 5u32;
pub const DMPAPER_LEGAL_EXTRA: u32 = 51u32;
pub const DMPAPER_LETTER: u32 = 1u32;
pub const DMPAPER_LETTERSMALL: u32 = 2u32;
pub const DMPAPER_LETTER_EXTRA: u32 = 50u32;
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 56u32;
pub const DMPAPER_LETTER_PLUS: u32 = 59u32;
pub const DMPAPER_LETTER_ROTATED: u32 = 75u32;
pub const DMPAPER_LETTER_TRANSVERSE: u32 = 54u32;
pub const DMPAPER_NOTE: u32 = 18u32;
pub const DMPAPER_P16K: u32 = 93u32;
pub const DMPAPER_P16K_ROTATED: u32 = 106u32;
pub const DMPAPER_P32K: u32 = 94u32;
pub const DMPAPER_P32KBIG: u32 = 95u32;
pub const DMPAPER_P32KBIG_ROTATED: u32 = 108u32;
pub const DMPAPER_P32K_ROTATED: u32 = 107u32;
pub const DMPAPER_PENV_1: u32 = 96u32;
pub const DMPAPER_PENV_10: u32 = 105u32;
pub const DMPAPER_PENV_10_ROTATED: u32 = 118u32;
pub const DMPAPER_PENV_1_ROTATED: u32 = 109u32;
pub const DMPAPER_PENV_2: u32 = 97u32;
pub const DMPAPER_PENV_2_ROTATED: u32 = 110u32;
pub const DMPAPER_PENV_3: u32 = 98u32;
pub const DMPAPER_PENV_3_ROTATED: u32 = 111u32;
pub const DMPAPER_PENV_4: u32 = 99u32;
pub const DMPAPER_PENV_4_ROTATED: u32 = 112u32;
pub const DMPAPER_PENV_5: u32 = 100u32;
pub const DMPAPER_PENV_5_ROTATED: u32 = 113u32;
pub const DMPAPER_PENV_6: u32 = 101u32;
pub const DMPAPER_PENV_6_ROTATED: u32 = 114u32;
pub const DMPAPER_PENV_7: u32 = 102u32;
pub const DMPAPER_PENV_7_ROTATED: u32 = 115u32;
pub const DMPAPER_PENV_8: u32 = 103u32;
pub const DMPAPER_PENV_8_ROTATED: u32 = 116u32;
pub const DMPAPER_PENV_9: u32 = 104u32;
pub const DMPAPER_PENV_9_ROTATED: u32 = 117u32;
pub const DMPAPER_QUARTO: u32 = 15u32;
pub const DMPAPER_RESERVED_48: u32 = 48u32;
pub const DMPAPER_RESERVED_49: u32 = 49u32;
pub const DMPAPER_STATEMENT: u32 = 6u32;
pub const DMPAPER_TABLOID: u32 = 3u32;
pub const DMPAPER_TABLOID_EXTRA: u32 = 52u32;
pub const DMPAPER_USER: u32 = 256u32;
pub const DMRES_DRAFT: i32 = -1i32;
pub const DMRES_HIGH: i32 = -4i32;
pub const DMRES_LOW: i32 = -2i32;
pub const DMRES_MEDIUM: i32 = -3i32;
pub const DMTT_BITMAP: u32 = 1u32;
pub const DMTT_DOWNLOAD: u32 = 2u32;
pub const DMTT_DOWNLOAD_OUTLINE: u32 = 4u32;
pub const DMTT_SUBDEV: u32 = 3u32;
pub const DM_BITSPERPEL: i32 = 262144i32;
pub const DM_COLLATE: i32 = 32768i32;
pub const DM_COLOR: i32 = 2048i32;
pub const DM_COPIES: i32 = 256i32;
pub const DM_DEFAULTSOURCE: i32 = 512i32;
pub const DM_DISPLAYFIXEDOUTPUT: i32 = 536870912i32;
pub const DM_DISPLAYFLAGS: i32 = 2097152i32;
pub const DM_DISPLAYFREQUENCY: i32 = 4194304i32;
pub const DM_DISPLAYORIENTATION: i32 = 128i32;
pub const DM_DITHERTYPE: i32 = 67108864i32;
pub const DM_DUPLEX: i32 = 4096i32;
pub const DM_FORMNAME: i32 = 65536i32;
pub const DM_ICMINTENT: i32 = 16777216i32;
pub const DM_ICMMETHOD: i32 = 8388608i32;
pub const DM_INTERLACED: u32 = 2u32;
pub const DM_LOGPIXELS: i32 = 131072i32;
pub const DM_MEDIATYPE: i32 = 33554432i32;
pub const DM_NUP: i32 = 64i32;
pub const DM_ORIENTATION: i32 = 1i32;
pub const DM_PANNINGHEIGHT: i32 = 268435456i32;
pub const DM_PANNINGWIDTH: i32 = 134217728i32;
pub const DM_PAPERLENGTH: i32 = 4i32;
pub const DM_PAPERSIZE: i32 = 2i32;
pub const DM_PAPERWIDTH: i32 = 8i32;
pub const DM_PELSHEIGHT: i32 = 1048576i32;
pub const DM_PELSWIDTH: i32 = 524288i32;
pub const DM_POSITION: i32 = 32i32;
pub const DM_PRINTQUALITY: i32 = 1024i32;
pub const DM_SCALE: i32 = 16i32;
pub const DM_SPECVERSION: u32 = 1025u32;
pub const DM_TTOPTION: i32 = 16384i32;
pub const DM_YRESOLUTION: i32 = 8192i32;
pub const DOWNLOADFACE: u32 = 514u32;
pub const DOWNLOADHEADER: u32 = 4111u32;
pub const DRAFTMODE: u32 = 7u32;
pub struct DRAWEDGE_FLAGS(i32);
pub const DRAWPATTERNRECT: u32 = 25u32;
pub struct DRAWSTATEPROC(i32);
pub struct DRAWSTATE_FLAGS(i32);
pub struct DRAWTEXTPARAMS(i32);
pub struct DRAW_CAPTION_FLAGS(i32);
pub struct DRAW_EDGE_FLAGS(i32);
pub struct DRAW_TEXT_FORMAT(i32);
pub const DT_CHARSTREAM: u32 = 4u32;
pub const DT_DISPFILE: u32 = 6u32;
pub const DT_METAFILE: u32 = 5u32;
pub const DT_PLOTTER: u32 = 0u32;
pub const DT_RASCAMERA: u32 = 3u32;
pub const DT_RASDISPLAY: u32 = 1u32;
pub const DT_RASPRINTER: u32 = 2u32;
pub const EASTEUROPE_CHARSET: u32 = 238u32;
pub const ELF_CULTURE_LATIN: u32 = 0u32;
pub const ELF_VENDOR_SIZE: u32 = 4u32;
pub const ELF_VERSION: u32 = 0u32;
pub struct EMBEDDED_FONT_PRIV_STATUS(i32);
pub struct EMBED_FONT_CHARSET(i32);
pub struct EMR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRALPHABLEND(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRANGLEARC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRARC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRBITBLT(i32);
pub struct EMRCREATEBRUSHINDIRECT(i32);
pub struct EMRCREATEDIBPATTERNBRUSHPT(i32);
pub struct EMRCREATEMONOBRUSH(i32);
pub struct EMRCREATEPALETTE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRCREATEPEN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRELLIPSE(i32);
pub struct EMREOF(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXCLUDECLIPRECT(i32);
pub struct EMREXTCREATEFONTINDIRECTW(i32);
pub struct EMREXTCREATEPEN(i32);
pub struct EMREXTESCAPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTFLOODFILL(i32);
pub struct EMREXTSELECTCLIPRGN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMREXTTEXTOUTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFILLPATH(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFILLRGN(i32);
pub struct EMRFORMAT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRFRAMERGN(i32);
pub struct EMRGDICOMMENT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRGLSBOUNDEDRECORD(i32);
pub struct EMRGLSRECORD(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRGRADIENTFILL(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRINVERTRGN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRLINETO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRMASKBLT(i32);
pub struct EMRMODIFYWORLDTRANSFORM(i32);
pub struct EMRNAMEDESCAPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMROFFSETCLIPRGN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPLGBLT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYDRAW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYDRAW16(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYLINE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYLINE16(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYPOLYLINE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYPOLYLINE16(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRPOLYTEXTOUTA(i32);
pub struct EMRRESIZEPALETTE(i32);
pub struct EMRRESTOREDC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRROUNDRECT(i32);
pub struct EMRSCALEVIEWPORTEXTEX(i32);
pub struct EMRSELECTCLIPPATH(i32);
pub struct EMRSELECTOBJECT(i32);
pub struct EMRSELECTPALETTE(i32);
pub struct EMRSETARCDIRECTION(i32);
pub struct EMRSETCOLORADJUSTMENT(i32);
pub struct EMRSETCOLORSPACE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETDIBITSTODEVICE(i32);
pub struct EMRSETICMPROFILE(i32);
pub struct EMRSETMAPPERFLAGS(i32);
pub struct EMRSETMITERLIMIT(i32);
pub struct EMRSETPALETTEENTRIES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETPIXELV(i32);
pub struct EMRSETTEXTCOLOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETVIEWPORTEXTEX(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSETVIEWPORTORGEX(i32);
pub struct EMRSETWORLDTRANSFORM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSTRETCHBLT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRSTRETCHDIBITS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EMRTRANSPARENTBLT(i32);
pub const EMR_ABORTPATH: u32 = 68u32;
pub const EMR_ALPHABLEND: u32 = 114u32;
pub const EMR_ANGLEARC: u32 = 41u32;
pub const EMR_ARC: u32 = 45u32;
pub const EMR_ARCTO: u32 = 55u32;
pub const EMR_BEGINPATH: u32 = 59u32;
pub const EMR_BITBLT: u32 = 76u32;
pub const EMR_CHORD: u32 = 46u32;
pub const EMR_CLOSEFIGURE: u32 = 61u32;
pub const EMR_COLORCORRECTPALETTE: u32 = 111u32;
pub const EMR_COLORMATCHTOTARGETW: u32 = 121u32;
pub const EMR_CREATEBRUSHINDIRECT: u32 = 39u32;
pub const EMR_CREATECOLORSPACE: u32 = 99u32;
pub const EMR_CREATECOLORSPACEW: u32 = 122u32;
pub const EMR_CREATEDIBPATTERNBRUSHPT: u32 = 94u32;
pub const EMR_CREATEMONOBRUSH: u32 = 93u32;
pub const EMR_CREATEPALETTE: u32 = 49u32;
pub const EMR_CREATEPEN: u32 = 38u32;
pub const EMR_DELETECOLORSPACE: u32 = 101u32;
pub const EMR_DELETEOBJECT: u32 = 40u32;
pub const EMR_ELLIPSE: u32 = 42u32;
pub const EMR_ENDPATH: u32 = 60u32;
pub const EMR_EOF: u32 = 14u32;
pub const EMR_EXCLUDECLIPRECT: u32 = 29u32;
pub const EMR_EXTCREATEFONTINDIRECTW: u32 = 82u32;
pub const EMR_EXTCREATEPEN: u32 = 95u32;
pub const EMR_EXTFLOODFILL: u32 = 53u32;
pub const EMR_EXTSELECTCLIPRGN: u32 = 75u32;
pub const EMR_EXTTEXTOUTA: u32 = 83u32;
pub const EMR_EXTTEXTOUTW: u32 = 84u32;
pub const EMR_FILLPATH: u32 = 62u32;
pub const EMR_FILLRGN: u32 = 71u32;
pub const EMR_FLATTENPATH: u32 = 65u32;
pub const EMR_FRAMERGN: u32 = 72u32;
pub const EMR_GDICOMMENT: u32 = 70u32;
pub const EMR_GLSBOUNDEDRECORD: u32 = 103u32;
pub const EMR_GLSRECORD: u32 = 102u32;
pub const EMR_GRADIENTFILL: u32 = 118u32;
pub const EMR_HEADER: u32 = 1u32;
pub const EMR_INTERSECTCLIPRECT: u32 = 30u32;
pub const EMR_INVERTRGN: u32 = 73u32;
pub const EMR_LINETO: u32 = 54u32;
pub const EMR_MASKBLT: u32 = 78u32;
pub const EMR_MAX: u32 = 122u32;
pub const EMR_MIN: u32 = 1u32;
pub const EMR_MODIFYWORLDTRANSFORM: u32 = 36u32;
pub const EMR_MOVETOEX: u32 = 27u32;
pub const EMR_OFFSETCLIPRGN: u32 = 26u32;
pub const EMR_PAINTRGN: u32 = 74u32;
pub const EMR_PIE: u32 = 47u32;
pub const EMR_PIXELFORMAT: u32 = 104u32;
pub const EMR_PLGBLT: u32 = 79u32;
pub const EMR_POLYBEZIER: u32 = 2u32;
pub const EMR_POLYBEZIER16: u32 = 85u32;
pub const EMR_POLYBEZIERTO: u32 = 5u32;
pub const EMR_POLYBEZIERTO16: u32 = 88u32;
pub const EMR_POLYDRAW: u32 = 56u32;
pub const EMR_POLYDRAW16: u32 = 92u32;
pub const EMR_POLYGON: u32 = 3u32;
pub const EMR_POLYGON16: u32 = 86u32;
pub const EMR_POLYLINE: u32 = 4u32;
pub const EMR_POLYLINE16: u32 = 87u32;
pub const EMR_POLYLINETO: u32 = 6u32;
pub const EMR_POLYLINETO16: u32 = 89u32;
pub const EMR_POLYPOLYGON: u32 = 8u32;
pub const EMR_POLYPOLYGON16: u32 = 91u32;
pub const EMR_POLYPOLYLINE: u32 = 7u32;
pub const EMR_POLYPOLYLINE16: u32 = 90u32;
pub const EMR_POLYTEXTOUTA: u32 = 96u32;
pub const EMR_POLYTEXTOUTW: u32 = 97u32;
pub const EMR_REALIZEPALETTE: u32 = 52u32;
pub const EMR_RECTANGLE: u32 = 43u32;
pub const EMR_RESERVED_105: u32 = 105u32;
pub const EMR_RESERVED_106: u32 = 106u32;
pub const EMR_RESERVED_107: u32 = 107u32;
pub const EMR_RESERVED_108: u32 = 108u32;
pub const EMR_RESERVED_109: u32 = 109u32;
pub const EMR_RESERVED_110: u32 = 110u32;
pub const EMR_RESERVED_117: u32 = 117u32;
pub const EMR_RESERVED_119: u32 = 119u32;
pub const EMR_RESERVED_120: u32 = 120u32;
pub const EMR_RESIZEPALETTE: u32 = 51u32;
pub const EMR_RESTOREDC: u32 = 34u32;
pub const EMR_ROUNDRECT: u32 = 44u32;
pub const EMR_SAVEDC: u32 = 33u32;
pub const EMR_SCALEVIEWPORTEXTEX: u32 = 31u32;
pub const EMR_SCALEWINDOWEXTEX: u32 = 32u32;
pub const EMR_SELECTCLIPPATH: u32 = 67u32;
pub const EMR_SELECTOBJECT: u32 = 37u32;
pub const EMR_SELECTPALETTE: u32 = 48u32;
pub const EMR_SETARCDIRECTION: u32 = 57u32;
pub const EMR_SETBKCOLOR: u32 = 25u32;
pub const EMR_SETBKMODE: u32 = 18u32;
pub const EMR_SETBRUSHORGEX: u32 = 13u32;
pub const EMR_SETCOLORADJUSTMENT: u32 = 23u32;
pub const EMR_SETCOLORSPACE: u32 = 100u32;
pub const EMR_SETDIBITSTODEVICE: u32 = 80u32;
pub const EMR_SETICMMODE: u32 = 98u32;
pub const EMR_SETICMPROFILEA: u32 = 112u32;
pub const EMR_SETICMPROFILEW: u32 = 113u32;
pub const EMR_SETLAYOUT: u32 = 115u32;
pub const EMR_SETMAPMODE: u32 = 17u32;
pub const EMR_SETMAPPERFLAGS: u32 = 16u32;
pub const EMR_SETMETARGN: u32 = 28u32;
pub const EMR_SETMITERLIMIT: u32 = 58u32;
pub const EMR_SETPALETTEENTRIES: u32 = 50u32;
pub const EMR_SETPIXELV: u32 = 15u32;
pub const EMR_SETPOLYFILLMODE: u32 = 19u32;
pub const EMR_SETROP2: u32 = 20u32;
pub const EMR_SETSTRETCHBLTMODE: u32 = 21u32;
pub const EMR_SETTEXTALIGN: u32 = 22u32;
pub const EMR_SETTEXTCOLOR: u32 = 24u32;
pub const EMR_SETVIEWPORTEXTEX: u32 = 11u32;
pub const EMR_SETVIEWPORTORGEX: u32 = 12u32;
pub const EMR_SETWINDOWEXTEX: u32 = 9u32;
pub const EMR_SETWINDOWORGEX: u32 = 10u32;
pub const EMR_SETWORLDTRANSFORM: u32 = 35u32;
pub const EMR_STRETCHBLT: u32 = 77u32;
pub const EMR_STRETCHDIBITS: u32 = 81u32;
pub const EMR_STROKEANDFILLPATH: u32 = 63u32;
pub const EMR_STROKEPATH: u32 = 64u32;
pub const EMR_TRANSPARENTBLT: u32 = 116u32;
pub const EMR_WIDENPATH: u32 = 66u32;
pub const ENABLEDUPLEX: u32 = 28u32;
pub const ENABLEPAIRKERNING: u32 = 769u32;
pub const ENABLERELATIVEWIDTHS: u32 = 768u32;
pub const ENCAPSULATED_POSTSCRIPT: u32 = 4116u32;
pub const ENDDOC: u32 = 11u32;
pub const END_PATH: u32 = 4098u32;
#[cfg(feature = "Win32_Foundation")]
pub struct ENHMETAHEADER(i32);
pub struct ENHMETARECORD(i32);
pub const ENHMETA_SIGNATURE: u32 = 1179469088u32;
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648u32;
pub struct ENHMFENUMPROC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTEXA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMLOGFONTEXDVA(i32);
pub struct ENUMLOGFONTEXDVW(i32);
pub struct ENUMLOGFONTEXW(i32);
pub struct ENUMLOGFONTW(i32);
pub const ENUMPAPERBINS: u32 = 31u32;
pub const ENUMPAPERMETRICS: u32 = 34u32;
pub struct ENUM_DISPLAY_SETTINGS_MODE(i32);
pub const EPSPRINTING: u32 = 33u32;
pub const EPS_SIGNATURE: u32 = 1179865157u32;
pub const ERROR: u32 = 0u32;
pub const ERR_FORMAT: u32 = 1006u32;
pub const ERR_GENERIC: u32 = 1000u32;
pub const ERR_INVALID_BASE: u32 = 1085u32;
pub const ERR_INVALID_CMAP: u32 = 1060u32;
pub const ERR_INVALID_DELTA_FORMAT: u32 = 1013u32;
pub const ERR_INVALID_EBLC: u32 = 1086u32;
pub const ERR_INVALID_GDEF: u32 = 1083u32;
pub const ERR_INVALID_GLYF: u32 = 1061u32;
pub const ERR_INVALID_GPOS: u32 = 1082u32;
pub const ERR_INVALID_GSUB: u32 = 1081u32;
pub const ERR_INVALID_HDMX: u32 = 1089u32;
pub const ERR_INVALID_HEAD: u32 = 1062u32;
pub const ERR_INVALID_HHEA: u32 = 1063u32;
pub const ERR_INVALID_HHEA_OR_VHEA: u32 = 1072u32;
pub const ERR_INVALID_HMTX: u32 = 1064u32;
pub const ERR_INVALID_HMTX_OR_VMTX: u32 = 1073u32;
pub const ERR_INVALID_JSTF: u32 = 1084u32;
pub const ERR_INVALID_LOCA: u32 = 1065u32;
pub const ERR_INVALID_LTSH: u32 = 1087u32;
pub const ERR_INVALID_MAXP: u32 = 1066u32;
pub const ERR_INVALID_MERGE_CHECKSUMS: u32 = 1011u32;
pub const ERR_INVALID_MERGE_FORMATS: u32 = 1010u32;
pub const ERR_INVALID_MERGE_NUMGLYPHS: u32 = 1012u32;
pub const ERR_INVALID_NAME: u32 = 1067u32;
pub const ERR_INVALID_OS2: u32 = 1069u32;
pub const ERR_INVALID_POST: u32 = 1068u32;
pub const ERR_INVALID_TTC_INDEX: u32 = 1015u32;
pub const ERR_INVALID_TTO: u32 = 1080u32;
pub const ERR_INVALID_VDMX: u32 = 1088u32;
pub const ERR_INVALID_VHEA: u32 = 1070u32;
pub const ERR_INVALID_VMTX: u32 = 1071u32;
pub const ERR_MEM: u32 = 1005u32;
pub const ERR_MISSING_CMAP: u32 = 1030u32;
pub const ERR_MISSING_EBDT: u32 = 1044u32;
pub const ERR_MISSING_GLYF: u32 = 1031u32;
pub const ERR_MISSING_HEAD: u32 = 1032u32;
pub const ERR_MISSING_HHEA: u32 = 1033u32;
pub const ERR_MISSING_HHEA_OR_VHEA: u32 = 1042u32;
pub const ERR_MISSING_HMTX: u32 = 1034u32;
pub const ERR_MISSING_HMTX_OR_VMTX: u32 = 1043u32;
pub const ERR_MISSING_LOCA: u32 = 1035u32;
pub const ERR_MISSING_MAXP: u32 = 1036u32;
pub const ERR_MISSING_NAME: u32 = 1037u32;
pub const ERR_MISSING_OS2: u32 = 1039u32;
pub const ERR_MISSING_POST: u32 = 1038u32;
pub const ERR_MISSING_VHEA: u32 = 1040u32;
pub const ERR_MISSING_VMTX: u32 = 1041u32;
pub const ERR_NOT_TTC: u32 = 1014u32;
pub const ERR_NO_GLYPHS: u32 = 1009u32;
pub const ERR_PARAMETER0: u32 = 1100u32;
pub const ERR_PARAMETER1: u32 = 1101u32;
pub const ERR_PARAMETER10: u32 = 1110u32;
pub const ERR_PARAMETER11: u32 = 1111u32;
pub const ERR_PARAMETER12: u32 = 1112u32;
pub const ERR_PARAMETER13: u32 = 1113u32;
pub const ERR_PARAMETER14: u32 = 1114u32;
pub const ERR_PARAMETER15: u32 = 1115u32;
pub const ERR_PARAMETER16: u32 = 1116u32;
pub const ERR_PARAMETER2: u32 = 1102u32;
pub const ERR_PARAMETER3: u32 = 1103u32;
pub const ERR_PARAMETER4: u32 = 1104u32;
pub const ERR_PARAMETER5: u32 = 1105u32;
pub const ERR_PARAMETER6: u32 = 1106u32;
pub const ERR_PARAMETER7: u32 = 1107u32;
pub const ERR_PARAMETER8: u32 = 1108u32;
pub const ERR_PARAMETER9: u32 = 1109u32;
pub const ERR_READCONTROL: u32 = 1003u32;
pub const ERR_READOUTOFBOUNDS: u32 = 1001u32;
pub const ERR_VERSION: u32 = 1008u32;
pub const ERR_WOULD_GROW: u32 = 1007u32;
pub const ERR_WRITECONTROL: u32 = 1004u32;
pub const ERR_WRITEOUTOFBOUNDS: u32 = 1002u32;
pub struct ETO_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EXTLOGFONTA(i32);
pub struct EXTLOGFONTW(i32);
pub struct EXTLOGPEN(i32);
pub struct EXTLOGPEN32(i32);
pub const EXTTEXTOUT: u32 = 512u32;
pub const EXT_DEVICE_CAPS: u32 = 4099u32;
pub struct EXT_FLOOD_FILL_TYPE(i32);
pub const E_ADDFONTFAILED: i32 = 512i32;
pub const E_API_NOTIMPL: i32 = 1i32;
pub const E_CHARCODECOUNTINVALID: i32 = 2i32;
pub const E_CHARCODESETINVALID: i32 = 3i32;
pub const E_CHARSETINVALID: i32 = 21i32;
pub const E_COULDNTCREATETEMPFILE: i32 = 513i32;
pub const E_DEVICETRUETYPEFONT: i32 = 4i32;
pub const E_ERRORACCESSINGEXCLUDELIST: i32 = 274i32;
pub const E_ERRORACCESSINGFACENAME: i32 = 13i32;
pub const E_ERRORACCESSINGFONTDATA: i32 = 12i32;
pub const E_ERRORCOMPRESSINGFONTDATA: i32 = 256i32;
pub const E_ERRORCONVERTINGCHARS: i32 = 18i32;
pub const E_ERRORCREATINGFONTFILE: i32 = 269i32;
pub const E_ERRORDECOMPRESSINGFONTDATA: i32 = 273i32;
pub const E_ERROREXPANDINGFONTDATA: i32 = 519i32;
pub const E_ERRORGETTINGDC: i32 = 520i32;
pub const E_ERRORREADINGFONTDATA: i32 = 267i32;
pub const E_ERRORUNICODECONVERSION: i32 = 17i32;
pub const E_EXCEPTION: i32 = 19i32;
pub const E_EXCEPTIONINCOMPRESSION: i32 = 522i32;
pub const E_EXCEPTIONINDECOMPRESSION: i32 = 521i32;
pub const E_FACENAMEINVALID: i32 = 275i32;
pub const E_FILE_NOT_FOUND: i32 = 23i32;
pub const E_FLAGSINVALID: i32 = 268i32;
pub const E_FONTALREADYEXISTS: i32 = 270i32;
pub const E_FONTDATAINVALID: i32 = 258i32;
pub const E_FONTFAMILYNAMENOTINFULL: i32 = 285i32;
pub const E_FONTFILECREATEFAILED: i32 = 515i32;
pub const E_FONTFILENOTFOUND: i32 = 517i32;
pub const E_FONTINSTALLFAILED: i32 = 272i32;
pub const E_FONTNAMEALREADYEXISTS: i32 = 271i32;
pub const E_FONTNOTEMBEDDABLE: i32 = 260i32;
pub const E_FONTREFERENCEINVALID: i32 = 8i32;
pub const E_FONTVARIATIONSIMULATED: i32 = 283i32;
pub const E_HDCINVALID: i32 = 6i32;
pub const E_INPUTPARAMINVALID: i32 = 25i32;
pub const E_NAMECHANGEFAILED: i32 = 259i32;
pub const E_NOFREEMEMORY: i32 = 7i32;
pub const E_NONE: i32 = 0i32;
pub const E_NOOS2: i32 = 265i32;
pub const E_NOTATRUETYPEFONT: i32 = 10i32;
pub const E_PBENABLEDINVALID: i32 = 280i32;
pub const E_PERMISSIONSINVALID: i32 = 279i32;
pub const E_PRIVSINVALID: i32 = 261i32;
pub const E_PRIVSTATUSINVALID: i32 = 278i32;
pub const E_READFROMSTREAMFAILED: i32 = 263i32;
pub const E_RESERVEDPARAMNOTNULL: i32 = 20i32;
pub const E_RESOURCEFILECREATEFAILED: i32 = 518i32;
pub const E_SAVETOSTREAMFAILED: i32 = 264i32;
pub const E_STATUSINVALID: i32 = 277i32;
pub const E_STREAMINVALID: i32 = 276i32;
pub const E_SUBSETTINGEXCEPTION: i32 = 281i32;
pub const E_SUBSETTINGFAILED: i32 = 262i32;
pub const E_SUBSTRING_TEST_FAIL: i32 = 282i32;
pub const E_T2NOFREEMEMORY: i32 = 266i32;
pub const E_TTC_INDEX_OUT_OF_RANGE: i32 = 24i32;
pub const E_WINDOWSAPI: i32 = 516i32;
pub const FEATURESETTING_CUSTPAPER: u32 = 3u32;
pub const FEATURESETTING_MIRROR: u32 = 4u32;
pub const FEATURESETTING_NEGATIVE: u32 = 5u32;
pub const FEATURESETTING_NUP: u32 = 0u32;
pub const FEATURESETTING_OUTPUT: u32 = 1u32;
pub const FEATURESETTING_PRIVATE_BEGIN: u32 = 4096u32;
pub const FEATURESETTING_PRIVATE_END: u32 = 8191u32;
pub const FEATURESETTING_PROTOCOL: u32 = 6u32;
pub const FEATURESETTING_PSLEVEL: u32 = 2u32;
pub struct FIXED(i32);
pub const FIXED_PITCH: u32 = 1u32;
pub const FLI_GLYPHS: i32 = 262144i32;
pub const FLI_MASK: u32 = 4155u32;
pub const FLUSHOUTPUT: u32 = 6u32;
pub struct FONTENUMPROCA(i32);
pub struct FONTENUMPROCW(i32);
pub const FONTMAPPER_MAX: u32 = 10u32;
pub struct FONT_CLIP_PRECISION(i32);
pub struct FONT_LICENSE_PRIVS(i32);
pub struct FONT_OUTPUT_PRECISION(i32);
pub struct FONT_PITCH_AND_FAMILY(i32);
pub struct FONT_QUALITY(i32);
pub struct FONT_RESOURCE_CHARACTERISTICS(i32);
pub const FS_ARABIC: i32 = 64i32;
pub const FS_BALTIC: i32 = 128i32;
pub const FS_CHINESESIMP: i32 = 262144i32;
pub const FS_CHINESETRAD: i32 = 1048576i32;
pub const FS_CYRILLIC: i32 = 4i32;
pub const FS_GREEK: i32 = 8i32;
pub const FS_HEBREW: i32 = 32i32;
pub const FS_JISJAPAN: i32 = 131072i32;
pub const FS_JOHAB: i32 = 2097152i32;
pub const FS_LATIN1: i32 = 1i32;
pub const FS_LATIN2: i32 = 2i32;
pub const FS_SYMBOL: i32 = -2147483648i32;
pub const FS_THAI: i32 = 65536i32;
pub const FS_TURKISH: i32 = 16i32;
pub const FS_VIETNAMESE: i32 = 256i32;
pub const FS_WANSUNG: i32 = 524288i32;
pub const FW_BLACK: u32 = 900u32;
pub const FW_BOLD: u32 = 700u32;
pub const FW_DEMIBOLD: u32 = 600u32;
pub const FW_DONTCARE: u32 = 0u32;
pub const FW_EXTRABOLD: u32 = 800u32;
pub const FW_EXTRALIGHT: u32 = 200u32;
pub const FW_HEAVY: u32 = 900u32;
pub const FW_LIGHT: u32 = 300u32;
pub const FW_MEDIUM: u32 = 500u32;
pub const FW_NORMAL: u32 = 400u32;
pub const FW_REGULAR: u32 = 400u32;
pub const FW_SEMIBOLD: u32 = 600u32;
pub const FW_THIN: u32 = 100u32;
pub const FW_ULTRABOLD: u32 = 800u32;
pub const FW_ULTRALIGHT: u32 = 200u32;
pub const GB2312_CHARSET: u32 = 134u32;
pub const GCPCLASS_ARABIC: u32 = 2u32;
pub const GCPCLASS_HEBREW: u32 = 2u32;
pub const GCPCLASS_LATIN: u32 = 1u32;
pub const GCPCLASS_LATINNUMBER: u32 = 5u32;
pub const GCPCLASS_LATINNUMERICSEPARATOR: u32 = 7u32;
pub const GCPCLASS_LATINNUMERICTERMINATOR: u32 = 6u32;
pub const GCPCLASS_LOCALNUMBER: u32 = 4u32;
pub const GCPCLASS_NEUTRAL: u32 = 3u32;
pub const GCPCLASS_NUMERICSEPARATOR: u32 = 8u32;
pub const GCPCLASS_POSTBOUNDLTR: u32 = 32u32;
pub const GCPCLASS_POSTBOUNDRTL: u32 = 16u32;
pub const GCPCLASS_PREBOUNDLTR: u32 = 128u32;
pub const GCPCLASS_PREBOUNDRTL: u32 = 64u32;
pub const GCPGLYPH_LINKAFTER: u32 = 16384u32;
pub const GCPGLYPH_LINKBEFORE: u32 = 32768u32;
pub const GCP_DBCS: u32 = 1u32;
pub const GCP_ERROR: u32 = 32768u32;
pub const GCP_JUSTIFYIN: i32 = 2097152i32;
#[cfg(feature = "Win32_Foundation")]
pub struct GCP_RESULTSA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct GCP_RESULTSW(i32);
pub const GDICOMMENT_BEGINGROUP: u32 = 2u32;
pub const GDICOMMENT_ENDGROUP: u32 = 3u32;
pub const GDICOMMENT_IDENTIFIER: u32 = 1128875079u32;
pub const GDICOMMENT_MULTIFORMATS: u32 = 1073741828u32;
pub const GDICOMMENT_UNICODE_END: u32 = 128u32;
pub const GDICOMMENT_UNICODE_STRING: u32 = 64u32;
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649u32;
pub const GDIPLUS_TS_QUERYVER: u32 = 4122u32;
pub const GDIPLUS_TS_RECORD: u32 = 4123u32;
pub const GDIREGISTERDDRAWPACKETVERSION: u32 = 1u32;
pub const GDI_ERROR: i32 = -1i32;
pub const GETCOLORTABLE: u32 = 5u32;
pub const GETDEVICEUNITS: u32 = 42u32;
pub const GETEXTENDEDTEXTMETRICS: u32 = 256u32;
pub const GETEXTENTTABLE: u32 = 257u32;
pub const GETFACENAME: u32 = 513u32;
pub const GETPAIRKERNTABLE: u32 = 258u32;
pub const GETPENWIDTH: u32 = 16u32;
pub const GETPHYSPAGESIZE: u32 = 12u32;
pub const GETPRINTINGOFFSET: u32 = 13u32;
pub const GETSCALINGFACTOR: u32 = 14u32;
pub const GETSETPAPERBINS: u32 = 29u32;
pub const GETSETPAPERMETRICS: u32 = 35u32;
pub const GETSETPRINTORIENT: u32 = 30u32;
pub const GETSETSCREENPARAMS: u32 = 3072u32;
pub const GETTECHNOLGY: u32 = 20u32;
pub const GETTECHNOLOGY: u32 = 20u32;
pub const GETTRACKKERNTABLE: u32 = 259u32;
pub const GETVECTORBRUSHSIZE: u32 = 27u32;
pub const GETVECTORPENSIZE: u32 = 26u32;
pub struct GET_CHARACTER_PLACEMENT_FLAGS(i32);
pub struct GET_DCX_FLAGS(i32);
pub struct GET_DEVICE_CAPS_INDEX(i32);
pub struct GET_GLYPH_OUTLINE_FORMAT(i32);
pub const GET_PS_FEATURESETTING: u32 = 4121u32;
pub struct GET_STOCK_OBJECT_FLAGS(i32);
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct GLYPHMETRICS(i32);
pub struct GLYPHSET(i32);
pub const GM_LAST: u32 = 2u32;
pub struct GOBJENUMPROC(i32);
pub struct GRADIENT_FILL(i32);
pub const GRADIENT_FILL_OP_FLAG: u32 = 255u32;
pub struct GRADIENT_RECT(i32);
pub struct GRADIENT_TRIANGLE(i32);
pub struct GRAPHICS_MODE(i32);
pub struct GRAYSTRINGPROC(i32);
pub const GREEK_CHARSET: u32 = 161u32;
pub const GS_8BIT_INDICES: u32 = 1u32;
pub struct HANDLETABLE(i32);
pub const HANGEUL_CHARSET: u32 = 129u32;
pub const HANGUL_CHARSET: u32 = 129u32;
pub struct HATCH_BRUSH_STYLE(i32);
pub struct HBITMAP(i32);
pub struct HBRUSH(i32);
pub struct HDC(i32);
pub struct HDC_MAP_MODE(i32);
pub const HEBREW_CHARSET: u32 = 177u32;
pub struct HENHMETAFILE(i32);
pub struct HFONT(i32);
pub struct HGDIOBJ(i32);
pub struct HMETAFILE(i32);
pub struct HMONITOR(i32);
pub struct HPALETTE(i32);
pub struct HPEN(i32);
pub struct HRGN(i32);
pub const HS_API_MAX: u32 = 12u32;
pub struct HdcMetdataEnhFileHandle(i32);
pub struct HdcMetdataFileHandle(i32);
pub const ICM_DONE_OUTSIDEDC: u32 = 4u32;
pub const ICM_OFF: u32 = 1u32;
pub const ICM_ON: u32 = 2u32;
pub const ICM_QUERY: u32 = 3u32;
pub const ILLUMINANT_A: u32 = 1u32;
pub const ILLUMINANT_B: u32 = 2u32;
pub const ILLUMINANT_C: u32 = 3u32;
pub const ILLUMINANT_D50: u32 = 4u32;
pub const ILLUMINANT_D55: u32 = 5u32;
pub const ILLUMINANT_D65: u32 = 6u32;
pub const ILLUMINANT_D75: u32 = 7u32;
pub const ILLUMINANT_DAYLIGHT: u32 = 3u32;
pub const ILLUMINANT_DEVICE_DEFAULT: u32 = 0u32;
pub const ILLUMINANT_F2: u32 = 8u32;
pub const ILLUMINANT_FLUORESCENT: u32 = 8u32;
pub const ILLUMINANT_MAX_INDEX: u32 = 8u32;
pub const ILLUMINANT_NTSC: u32 = 3u32;
pub const ILLUMINANT_TUNGSTEN: u32 = 1u32;
pub const JOHAB_CHARSET: u32 = 130u32;
pub struct KERNINGPAIR(i32);
pub const LAYOUT_BTT: u32 = 2u32;
pub const LAYOUT_VBH: u32 = 4u32;
pub const LCS_CALIBRATED_RGB: i32 = 0i32;
pub const LCS_GM_ABS_COLORIMETRIC: i32 = 8i32;
pub const LCS_GM_BUSINESS: i32 = 1i32;
pub const LCS_GM_GRAPHICS: i32 = 2i32;
pub const LCS_GM_IMAGES: i32 = 4i32;
pub const LC_INTERIORS: u32 = 128u32;
pub const LC_MARKER: u32 = 4u32;
pub const LC_NONE: u32 = 0u32;
pub const LC_POLYLINE: u32 = 2u32;
pub const LC_POLYMARKER: u32 = 8u32;
pub const LC_STYLED: u32 = 32u32;
pub const LC_WIDE: u32 = 16u32;
pub const LC_WIDESTYLED: u32 = 64u32;
pub const LF_FACESIZE: u32 = 32u32;
pub const LF_FULLFACESIZE: u32 = 64u32;
pub struct LINEDDAPROC(i32);
pub struct LOGBRUSH(i32);
pub struct LOGBRUSH32(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LOGFONTA(i32);
pub struct LOGFONTW(i32);
pub struct LOGPALETTE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LOGPEN(i32);
pub const LPD_DOUBLEBUFFER: u32 = 1u32;
pub const LPD_SHARE_ACCUM: u32 = 256u32;
pub const LPD_SHARE_DEPTH: u32 = 64u32;
pub const LPD_SHARE_STENCIL: u32 = 128u32;
pub const LPD_STEREO: u32 = 2u32;
pub const LPD_SUPPORT_GDI: u32 = 16u32;
pub const LPD_SUPPORT_OPENGL: u32 = 32u32;
pub const LPD_SWAP_COPY: u32 = 1024u32;
pub const LPD_SWAP_EXCHANGE: u32 = 512u32;
pub const LPD_TRANSPARENT: u32 = 4096u32;
pub const LPD_TYPE_COLORINDEX: u32 = 1u32;
pub const LPD_TYPE_RGBA: u32 = 0u32;
pub struct LPFNDEVCAPS(i32);
pub struct LPFNDEVMODE(i32);
pub const MAC_CHARSET: u32 = 77u32;
pub struct MAT2(i32);
pub const MAXSTRETCHBLTMODE: u32 = 4u32;
pub const METAFILE_DRIVER: u32 = 2049u32;
pub struct METAHEADER(i32);
pub struct METARECORD(i32);
pub const META_ANIMATEPALETTE: u32 = 1078u32;
pub const META_ARC: u32 = 2071u32;
pub const META_BITBLT: u32 = 2338u32;
pub const META_CHORD: u32 = 2096u32;
pub const META_CREATEBRUSHINDIRECT: u32 = 764u32;
pub const META_CREATEFONTINDIRECT: u32 = 763u32;
pub const META_CREATEPALETTE: u32 = 247u32;
pub const META_CREATEPATTERNBRUSH: u32 = 505u32;
pub const META_CREATEPENINDIRECT: u32 = 762u32;
pub const META_CREATEREGION: u32 = 1791u32;
pub const META_DELETEOBJECT: u32 = 496u32;
pub const META_DIBBITBLT: u32 = 2368u32;
pub const META_DIBCREATEPATTERNBRUSH: u32 = 322u32;
pub const META_DIBSTRETCHBLT: u32 = 2881u32;
pub const META_ELLIPSE: u32 = 1048u32;
pub const META_ESCAPE: u32 = 1574u32;
pub const META_EXCLUDECLIPRECT: u32 = 1045u32;
pub const META_EXTFLOODFILL: u32 = 1352u32;
pub const META_EXTTEXTOUT: u32 = 2610u32;
pub const META_FILLREGION: u32 = 552u32;
pub const META_FLOODFILL: u32 = 1049u32;
pub const META_FRAMEREGION: u32 = 1065u32;
pub const META_INTERSECTCLIPRECT: u32 = 1046u32;
pub const META_INVERTREGION: u32 = 298u32;
pub const META_LINETO: u32 = 531u32;
pub const META_MOVETO: u32 = 532u32;
pub const META_OFFSETCLIPRGN: u32 = 544u32;
pub const META_OFFSETVIEWPORTORG: u32 = 529u32;
pub const META_OFFSETWINDOWORG: u32 = 527u32;
pub const META_PAINTREGION: u32 = 299u32;
pub const META_PATBLT: u32 = 1565u32;
pub const META_PIE: u32 = 2074u32;
pub const META_POLYGON: u32 = 804u32;
pub const META_POLYLINE: u32 = 805u32;
pub const META_POLYPOLYGON: u32 = 1336u32;
pub const META_REALIZEPALETTE: u32 = 53u32;
pub const META_RECTANGLE: u32 = 1051u32;
pub const META_RESIZEPALETTE: u32 = 313u32;
pub const META_RESTOREDC: u32 = 295u32;
pub const META_ROUNDRECT: u32 = 1564u32;
pub const META_SAVEDC: u32 = 30u32;
pub const META_SCALEVIEWPORTEXT: u32 = 1042u32;
pub const META_SCALEWINDOWEXT: u32 = 1040u32;
pub const META_SELECTCLIPREGION: u32 = 300u32;
pub const META_SELECTOBJECT: u32 = 301u32;
pub const META_SELECTPALETTE: u32 = 564u32;
pub const META_SETBKCOLOR: u32 = 513u32;
pub const META_SETBKMODE: u32 = 258u32;
pub const META_SETDIBTODEV: u32 = 3379u32;
pub const META_SETLAYOUT: u32 = 329u32;
pub const META_SETMAPMODE: u32 = 259u32;
pub const META_SETMAPPERFLAGS: u32 = 561u32;
pub const META_SETPALENTRIES: u32 = 55u32;
pub const META_SETPIXEL: u32 = 1055u32;
pub const META_SETPOLYFILLMODE: u32 = 262u32;
pub const META_SETRELABS: u32 = 261u32;
pub const META_SETROP2: u32 = 260u32;
pub const META_SETSTRETCHBLTMODE: u32 = 263u32;
pub const META_SETTEXTALIGN: u32 = 302u32;
pub const META_SETTEXTCHAREXTRA: u32 = 264u32;
pub const META_SETTEXTCOLOR: u32 = 521u32;
pub const META_SETTEXTJUSTIFICATION: u32 = 522u32;
pub const META_SETVIEWPORTEXT: u32 = 526u32;
pub const META_SETVIEWPORTORG: u32 = 525u32;
pub const META_SETWINDOWEXT: u32 = 524u32;
pub const META_SETWINDOWORG: u32 = 523u32;
pub const META_STRETCHBLT: u32 = 2851u32;
pub const META_STRETCHDIB: u32 = 3907u32;
pub const META_TEXTOUT: u32 = 1313u32;
pub const MFCOMMENT: u32 = 15u32;
pub struct MFENUMPROC(i32);
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: u32 = 0u32;
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: u32 = 2147483647u32;
pub const MM_MAX_AXES_NAMELEN: u32 = 16u32;
pub const MM_MAX_NUMAXES: u32 = 16u32;
pub struct MODIFY_WORLD_TRANSFORM_MODE(i32);
pub struct MONITORENUMPROC(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFOEXA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MONITORINFOEXW(i32);
pub struct MONITOR_FROM_FLAGS(i32);
pub const MONO_FONT: u32 = 8u32;
pub const MOUSETRAILS: u32 = 39u32;
pub const NEWFRAME: u32 = 1u32;
pub struct NEWTEXTMETRICA(i32);
pub struct NEWTEXTMETRICW(i32);
pub const NEWTRANSPARENT: u32 = 3u32;
pub const NEXTBAND: u32 = 3u32;
pub const NTM_BOLD: i32 = 32i32;
pub const NTM_DSIG: u32 = 2097152u32;
pub const NTM_ITALIC: i32 = 1i32;
pub const NTM_MULTIPLEMASTER: u32 = 524288u32;
pub const NTM_NONNEGATIVE_AC: u32 = 65536u32;
pub const NTM_PS_OPENTYPE: u32 = 131072u32;
pub const NTM_REGULAR: i32 = 64i32;
pub const NTM_TT_OPENTYPE: u32 = 262144u32;
pub const NTM_TYPE1: u32 = 1048576u32;
pub const NULLREGION: u32 = 1u32;
pub struct OBJ_TYPE(i32);
pub const OEM_CHARSET: u32 = 255u32;
pub const OPENCHANNEL: u32 = 4110u32;
#[cfg(feature = "Win32_Foundation")]
pub struct OUTLINETEXTMETRICA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OUTLINETEXTMETRICW(i32);
pub const OUT_SCREEN_OUTLINE_PRECIS: u32 = 9u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PAINTSTRUCT(i32);
pub struct PALETTEENTRY(i32);
pub struct PANOSE(i32);
pub const PANOSE_COUNT: u32 = 10u32;
pub const PAN_ANY: u32 = 0u32;
pub const PAN_ARMSTYLE_INDEX: u32 = 6u32;
pub const PAN_BENT_ARMS_DOUBLE_SERIF: u32 = 11u32;
pub const PAN_BENT_ARMS_HORZ: u32 = 7u32;
pub const PAN_BENT_ARMS_SINGLE_SERIF: u32 = 10u32;
pub const PAN_BENT_ARMS_VERT: u32 = 9u32;
pub const PAN_BENT_ARMS_WEDGE: u32 = 8u32;
pub const PAN_CONTRAST_HIGH: u32 = 8u32;
pub const PAN_CONTRAST_INDEX: u32 = 4u32;
pub const PAN_CONTRAST_LOW: u32 = 4u32;
pub const PAN_CONTRAST_MEDIUM: u32 = 6u32;
pub const PAN_CONTRAST_MEDIUM_HIGH: u32 = 7u32;
pub const PAN_CONTRAST_MEDIUM_LOW: u32 = 5u32;
pub const PAN_CONTRAST_NONE: u32 = 2u32;
pub const PAN_CONTRAST_VERY_HIGH: u32 = 9u32;
pub const PAN_CONTRAST_VERY_LOW: u32 = 3u32;
pub const PAN_CULTURE_LATIN: u32 = 0u32;
pub const PAN_FAMILYTYPE_INDEX: u32 = 0u32;
pub const PAN_FAMILY_DECORATIVE: u32 = 4u32;
pub const PAN_FAMILY_PICTORIAL: u32 = 5u32;
pub const PAN_FAMILY_SCRIPT: u32 = 3u32;
pub const PAN_FAMILY_TEXT_DISPLAY: u32 = 2u32;
pub const PAN_LETTERFORM_INDEX: u32 = 7u32;
pub const PAN_LETT_NORMAL_BOXED: u32 = 4u32;
pub const PAN_LETT_NORMAL_CONTACT: u32 = 2u32;
pub const PAN_LETT_NORMAL_FLATTENED: u32 = 5u32;
pub const PAN_LETT_NORMAL_OFF_CENTER: u32 = 7u32;
pub const PAN_LETT_NORMAL_ROUNDED: u32 = 6u32;
pub const PAN_LETT_NORMAL_SQUARE: u32 = 8u32;
pub const PAN_LETT_NORMAL_WEIGHTED: u32 = 3u32;
pub const PAN_LETT_OBLIQUE_BOXED: u32 = 11u32;
pub const PAN_LETT_OBLIQUE_CONTACT: u32 = 9u32;
pub const PAN_LETT_OBLIQUE_FLATTENED: u32 = 12u32;
pub const PAN_LETT_OBLIQUE_OFF_CENTER: u32 = 14u32;
pub const PAN_LETT_OBLIQUE_ROUNDED: u32 = 13u32;
pub const PAN_LETT_OBLIQUE_SQUARE: u32 = 15u32;
pub const PAN_LETT_OBLIQUE_WEIGHTED: u32 = 10u32;
pub const PAN_MIDLINE_CONSTANT_POINTED: u32 = 9u32;
pub const PAN_MIDLINE_CONSTANT_SERIFED: u32 = 10u32;
pub const PAN_MIDLINE_CONSTANT_TRIMMED: u32 = 8u32;
pub const PAN_MIDLINE_HIGH_POINTED: u32 = 6u32;
pub const PAN_MIDLINE_HIGH_SERIFED: u32 = 7u32;
pub const PAN_MIDLINE_HIGH_TRIMMED: u32 = 5u32;
pub const PAN_MIDLINE_INDEX: u32 = 8u32;
pub const PAN_MIDLINE_LOW_POINTED: u32 = 12u32;
pub const PAN_MIDLINE_LOW_SERIFED: u32 = 13u32;
pub const PAN_MIDLINE_LOW_TRIMMED: u32 = 11u32;
pub const PAN_MIDLINE_STANDARD_POINTED: u32 = 3u32;
pub const PAN_MIDLINE_STANDARD_SERIFED: u32 = 4u32;
pub const PAN_MIDLINE_STANDARD_TRIMMED: u32 = 2u32;
pub const PAN_NO_FIT: u32 = 1u32;
pub const PAN_PROPORTION_INDEX: u32 = 3u32;
pub const PAN_PROP_CONDENSED: u32 = 6u32;
pub const PAN_PROP_EVEN_WIDTH: u32 = 4u32;
pub const PAN_PROP_EXPANDED: u32 = 5u32;
pub const PAN_PROP_MODERN: u32 = 3u32;
pub const PAN_PROP_MONOSPACED: u32 = 9u32;
pub const PAN_PROP_OLD_STYLE: u32 = 2u32;
pub const PAN_PROP_VERY_CONDENSED: u32 = 8u32;
pub const PAN_PROP_VERY_EXPANDED: u32 = 7u32;
pub const PAN_SERIFSTYLE_INDEX: u32 = 1u32;
pub const PAN_SERIF_BONE: u32 = 8u32;
pub const PAN_SERIF_COVE: u32 = 2u32;
pub const PAN_SERIF_EXAGGERATED: u32 = 9u32;
pub const PAN_SERIF_FLARED: u32 = 14u32;
pub const PAN_SERIF_NORMAL_SANS: u32 = 11u32;
pub const PAN_SERIF_OBTUSE_COVE: u32 = 3u32;
pub const PAN_SERIF_OBTUSE_SANS: u32 = 12u32;
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: u32 = 5u32;
pub const PAN_SERIF_PERP_SANS: u32 = 13u32;
pub const PAN_SERIF_ROUNDED: u32 = 15u32;
pub const PAN_SERIF_SQUARE: u32 = 6u32;
pub const PAN_SERIF_SQUARE_COVE: u32 = 4u32;
pub const PAN_SERIF_THIN: u32 = 7u32;
pub const PAN_SERIF_TRIANGLE: u32 = 10u32;
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: u32 = 6u32;
pub const PAN_STRAIGHT_ARMS_HORZ: u32 = 2u32;
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: u32 = 5u32;
pub const PAN_STRAIGHT_ARMS_VERT: u32 = 4u32;
pub const PAN_STRAIGHT_ARMS_WEDGE: u32 = 3u32;
pub const PAN_STROKEVARIATION_INDEX: u32 = 5u32;
pub const PAN_STROKE_GRADUAL_DIAG: u32 = 2u32;
pub const PAN_STROKE_GRADUAL_HORZ: u32 = 5u32;
pub const PAN_STROKE_GRADUAL_TRAN: u32 = 3u32;
pub const PAN_STROKE_GRADUAL_VERT: u32 = 4u32;
pub const PAN_STROKE_INSTANT_VERT: u32 = 8u32;
pub const PAN_STROKE_RAPID_HORZ: u32 = 7u32;
pub const PAN_STROKE_RAPID_VERT: u32 = 6u32;
pub const PAN_WEIGHT_BLACK: u32 = 10u32;
pub const PAN_WEIGHT_BOLD: u32 = 8u32;
pub const PAN_WEIGHT_BOOK: u32 = 5u32;
pub const PAN_WEIGHT_DEMI: u32 = 7u32;
pub const PAN_WEIGHT_HEAVY: u32 = 9u32;
pub const PAN_WEIGHT_INDEX: u32 = 2u32;
pub const PAN_WEIGHT_LIGHT: u32 = 3u32;
pub const PAN_WEIGHT_MEDIUM: u32 = 6u32;
pub const PAN_WEIGHT_NORD: u32 = 11u32;
pub const PAN_WEIGHT_THIN: u32 = 4u32;
pub const PAN_WEIGHT_VERY_LIGHT: u32 = 2u32;
pub const PAN_XHEIGHT_CONSTANT_LARGE: u32 = 4u32;
pub const PAN_XHEIGHT_CONSTANT_SMALL: u32 = 2u32;
pub const PAN_XHEIGHT_CONSTANT_STD: u32 = 3u32;
pub const PAN_XHEIGHT_DUCKING_LARGE: u32 = 7u32;
pub const PAN_XHEIGHT_DUCKING_SMALL: u32 = 5u32;
pub const PAN_XHEIGHT_DUCKING_STD: u32 = 6u32;
pub const PAN_XHEIGHT_INDEX: u32 = 9u32;
pub const PASSTHROUGH: u32 = 19u32;
pub const PC_EXPLICIT: u32 = 2u32;
pub const PC_INTERIORS: u32 = 128u32;
pub const PC_NOCOLLAPSE: u32 = 4u32;
pub const PC_NONE: u32 = 0u32;
pub const PC_PATHS: u32 = 512u32;
pub const PC_POLYGON: u32 = 1u32;
pub const PC_POLYPOLYGON: u32 = 256u32;
pub const PC_RECTANGLE: u32 = 2u32;
pub const PC_RESERVED: u32 = 1u32;
pub const PC_SCANLINE: u32 = 8u32;
pub const PC_STYLED: u32 = 32u32;
pub const PC_TRAPEZOID: u32 = 4u32;
pub const PC_WIDE: u32 = 16u32;
pub const PC_WIDESTYLED: u32 = 64u32;
pub const PC_WINDPOLYGON: u32 = 4u32;
pub struct PELARRAY(i32);
pub struct PEN_STYLE(i32);
pub const PFD_DEPTH_DONTCARE: u32 = 536870912u32;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 16384u32;
pub const PFD_DOUBLEBUFFER: u32 = 1u32;
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 1073741824u32;
pub const PFD_DRAW_TO_BITMAP: u32 = 8u32;
pub const PFD_DRAW_TO_WINDOW: u32 = 4u32;
pub const PFD_GENERIC_ACCELERATED: u32 = 4096u32;
pub const PFD_GENERIC_FORMAT: u32 = 64u32;
pub const PFD_MAIN_PLANE: u32 = 0u32;
pub const PFD_NEED_PALETTE: u32 = 128u32;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 256u32;
pub const PFD_OVERLAY_PLANE: u32 = 1u32;
pub const PFD_STEREO: u32 = 2u32;
pub const PFD_STEREO_DONTCARE: u32 = 2147483648u32;
pub const PFD_SUPPORT_COMPOSITION: u32 = 32768u32;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 8192u32;
pub const PFD_SUPPORT_GDI: u32 = 16u32;
pub const PFD_SUPPORT_OPENGL: u32 = 32u32;
pub const PFD_SWAP_COPY: u32 = 1024u32;
pub const PFD_SWAP_EXCHANGE: u32 = 512u32;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 2048u32;
pub const PFD_TYPE_COLORINDEX: u32 = 1u32;
pub const PFD_TYPE_RGBA: u32 = 0u32;
pub const PFD_UNDERLAY_PLANE: i32 = -1i32;
pub struct POINTFX(i32);
pub const POLYFILL_LAST: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct POLYTEXTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct POLYTEXTW(i32);
pub const POSTSCRIPT_DATA: u32 = 37u32;
pub const POSTSCRIPT_IDENTIFY: u32 = 4117u32;
pub const POSTSCRIPT_IGNORE: u32 = 38u32;
pub const POSTSCRIPT_INJECTION: u32 = 4118u32;
pub const POSTSCRIPT_PASSTHROUGH: u32 = 4115u32;
pub const PRINTRATEUNIT_CPS: u32 = 2u32;
pub const PRINTRATEUNIT_IPM: u32 = 4u32;
pub const PRINTRATEUNIT_LPM: u32 = 3u32;
pub const PRINTRATEUNIT_PPM: u32 = 1u32;
pub const PR_JOBSTATUS: u32 = 0u32;
pub const PSIDENT_GDICENTRIC: u32 = 0u32;
pub const PSIDENT_PSCENTRIC: u32 = 1u32;
pub const PSINJECT_DLFONT: u32 = 3722304989u32;
pub const PSPROTOCOL_ASCII: u32 = 0u32;
pub const PSPROTOCOL_BCP: u32 = 1u32;
pub const PSPROTOCOL_BINARY: u32 = 3u32;
pub const PSPROTOCOL_TBCP: u32 = 2u32;
pub const PT_BEZIERTO: u32 = 4u32;
pub const PT_CLOSEFIGURE: u32 = 1u32;
pub const PT_LINETO: u32 = 2u32;
pub const PT_MOVETO: u32 = 6u32;
pub const QDC_ALL_PATHS: u32 = 1u32;
pub const QDC_DATABASE_CURRENT: u32 = 4u32;
pub const QDC_INCLUDE_HMD: u32 = 32u32;
pub const QDC_ONLY_ACTIVE_PATHS: u32 = 2u32;
pub const QDC_VIRTUAL_MODE_AWARE: u32 = 16u32;
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 64u32;
pub const QDI_DIBTOSCREEN: u32 = 4u32;
pub const QDI_GETDIBITS: u32 = 2u32;
pub const QDI_SETDIBITS: u32 = 1u32;
pub const QDI_STRETCHDIB: u32 = 8u32;
pub const QUERYDIBSUPPORT: u32 = 3073u32;
pub const QUERYESCSUPPORT: u32 = 8u32;
pub const QUERYROPSUPPORT: u32 = 40u32;
pub struct R2_MODE(i32);
pub struct RASTERIZER_STATUS(i32);
pub const RASTER_FONTTYPE: u32 = 1u32;
pub const RC_BANDING: u32 = 2u32;
pub const RC_BIGFONT: u32 = 1024u32;
pub const RC_BITBLT: u32 = 1u32;
pub const RC_BITMAP64: u32 = 8u32;
pub const RC_DEVBITS: u32 = 32768u32;
pub const RC_DIBTODEV: u32 = 512u32;
pub const RC_DI_BITMAP: u32 = 128u32;
pub const RC_FLOODFILL: u32 = 4096u32;
pub const RC_GDI20_OUTPUT: u32 = 16u32;
pub const RC_GDI20_STATE: u32 = 32u32;
pub const RC_OP_DX_OUTPUT: u32 = 16384u32;
pub const RC_PALETTE: u32 = 256u32;
pub const RC_SAVEBITMAP: u32 = 64u32;
pub const RC_SCALING: u32 = 4u32;
pub const RC_STRETCHBLT: u32 = 2048u32;
pub const RC_STRETCHDIB: u32 = 8192u32;
pub const RDH_RECTANGLES: u32 = 1u32;
pub struct READEMBEDPROC(i32);
pub struct REDRAW_WINDOW_FLAGS(i32);
pub const RELATIVE: u32 = 2u32;
pub const RESTORE_CTM: u32 = 4100u32;
pub struct RGBQUAD(i32);
pub struct RGBTRIPLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RGNDATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RGNDATAHEADER(i32);
pub struct RGN_COMBINE_MODE(i32);
pub const RGN_ERROR: u32 = 0u32;
pub struct ROP_CODE(i32);
pub const RUSSIAN_CHARSET: u32 = 204u32;
pub const SAVE_CTM: u32 = 4101u32;
pub const SB_CONST_ALPHA: u32 = 1u32;
pub const SB_GRAD_RECT: u32 = 16u32;
pub const SB_GRAD_TRI: u32 = 32u32;
pub const SB_NONE: u32 = 0u32;
pub const SB_PIXEL_ALPHA: u32 = 2u32;
pub const SB_PREMULT_ALPHA: u32 = 4u32;
pub const SC_SCREENSAVE: u32 = 61760u32;
pub const SDC_ALLOW_CHANGES: u32 = 1024u32;
pub const SDC_ALLOW_PATH_ORDER_CHANGES: u32 = 8192u32;
pub const SDC_APPLY: u32 = 128u32;
pub const SDC_FORCE_MODE_ENUMERATION: u32 = 4096u32;
pub const SDC_NO_OPTIMIZATION: u32 = 256u32;
pub const SDC_PATH_PERSIST_IF_REQUIRED: u32 = 2048u32;
pub const SDC_SAVE_TO_DATABASE: u32 = 512u32;
pub const SDC_TOPOLOGY_CLONE: u32 = 2u32;
pub const SDC_TOPOLOGY_EXTEND: u32 = 4u32;
pub const SDC_TOPOLOGY_EXTERNAL: u32 = 8u32;
pub const SDC_TOPOLOGY_INTERNAL: u32 = 1u32;
pub const SDC_TOPOLOGY_SUPPLIED: u32 = 16u32;
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: u32 = 32u32;
pub const SDC_VALIDATE: u32 = 64u32;
pub const SDC_VIRTUAL_MODE_AWARE: u32 = 32768u32;
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 131072u32;
pub const SELECTDIB: u32 = 41u32;
pub const SELECTPAPERSOURCE: u32 = 18u32;
pub const SETABORTPROC: u32 = 9u32;
pub const SETALLJUSTVALUES: u32 = 771u32;
pub const SETCHARSET: u32 = 772u32;
pub const SETCOLORTABLE: u32 = 4u32;
pub const SETCOPYCOUNT: u32 = 17u32;
pub const SETDIBSCALING: u32 = 32u32;
pub const SETICMPROFILE_EMBEDED: u32 = 1u32;
pub const SETKERNTRACK: u32 = 770u32;
pub const SETLINECAP: u32 = 21u32;
pub const SETLINEJOIN: u32 = 22u32;
pub const SETMITERLIMIT: u32 = 23u32;
pub const SET_ARC_DIRECTION: u32 = 4102u32;
pub const SET_BACKGROUND_COLOR: u32 = 4103u32;
pub const SET_BOUNDS: u32 = 4109u32;
pub struct SET_BOUNDS_RECT_FLAGS(i32);
pub const SET_CLIP_BOX: u32 = 4108u32;
pub const SET_MIRROR_MODE: u32 = 4110u32;
pub const SET_POLY_MODE: u32 = 4104u32;
pub const SET_SCREEN_ANGLE: u32 = 4105u32;
pub const SET_SPREAD: u32 = 4106u32;
pub const SHIFTJIS_CHARSET: u32 = 128u32;
pub const SIMPLEREGION: u32 = 2u32;
pub const SPCLPASSTHROUGH2: u32 = 4568u32;
pub const SP_APPABORT: i32 = -2i32;
pub const SP_ERROR: i32 = -1i32;
pub const SP_NOTREPORTED: u32 = 16384u32;
pub const SP_OUTOFDISK: i32 = -4i32;
pub const SP_OUTOFMEMORY: i32 = -5i32;
pub const SP_USERABORT: i32 = -3i32;
pub const STARTDOC: u32 = 10u32;
pub const STOCK_LAST: u32 = 19u32;
pub const STRETCHBLT: u32 = 2048u32;
pub struct STRETCH_BLT_MODE(i32);
pub const SYMBOL_CHARSET: u32 = 2u32;
pub const SYSPAL_ERROR: u32 = 0u32;
pub const SYSRGN: u32 = 4u32;
pub struct SYSTEM_PALETTE_USE(i32);
pub const TC_CP_STROKE: u32 = 4u32;
pub const TC_CR_90: u32 = 8u32;
pub const TC_CR_ANY: u32 = 16u32;
pub const TC_EA_DOUBLE: u32 = 512u32;
pub const TC_IA_ABLE: u32 = 1024u32;
pub const TC_OP_CHARACTER: u32 = 1u32;
pub const TC_OP_STROKE: u32 = 2u32;
pub const TC_RA_ABLE: u32 = 8192u32;
pub const TC_RESERVED: u32 = 32768u32;
pub const TC_SA_CONTIN: u32 = 256u32;
pub const TC_SA_DOUBLE: u32 = 64u32;
pub const TC_SA_INTEGER: u32 = 128u32;
pub const TC_SCROLLBLT: u32 = 65536u32;
pub const TC_SF_X_YINDEP: u32 = 32u32;
pub const TC_SO_ABLE: u32 = 4096u32;
pub const TC_UA_ABLE: u32 = 2048u32;
pub const TC_VA_ABLE: u32 = 16384u32;
pub struct TEXTMETRICA(i32);
pub struct TEXTMETRICW(i32);
pub struct TEXT_ALIGN_OPTIONS(i32);
pub const THAI_CHARSET: u32 = 222u32;
pub const TMPF_DEVICE: u32 = 8u32;
pub const TMPF_FIXED_PITCH: u32 = 1u32;
pub const TMPF_TRUETYPE: u32 = 4u32;
pub const TMPF_VECTOR: u32 = 2u32;
pub const TRANSFORM_CTM: u32 = 4107u32;
pub struct TRIVERTEX(i32);
pub const TRUETYPE_FONTTYPE: u32 = 4u32;
pub const TTDELETE_DONTREMOVEFONT: u32 = 1u32;
pub struct TTEMBEDINFO(i32);
pub const TTEMBED_EUDCEMBEDDED: u32 = 2u32;
pub const TTEMBED_FAILIFVARIATIONSIMULATED: u32 = 16u32;
pub struct TTEMBED_FLAGS(i32);
pub const TTEMBED_SUBSETCANCEL: u32 = 4u32;
pub const TTEMBED_VARIATIONSIMULATED: u32 = 1u32;
pub const TTEMBED_WEBOBJECT: u32 = 128u32;
pub const TTEMBED_XORENCRYPTDATA: u32 = 268435456u32;
pub const TTFCFP_APPLE_PLATFORMID: u32 = 1u32;
pub const TTFCFP_DELTA: u32 = 2u32;
pub const TTFCFP_DONT_CARE: u32 = 65535u32;
pub const TTFCFP_FLAGS_COMPRESS: u32 = 2u32;
pub const TTFCFP_FLAGS_GLYPHLIST: u32 = 8u32;
pub const TTFCFP_FLAGS_SUBSET: u32 = 1u32;
pub const TTFCFP_FLAGS_TTC: u32 = 4u32;
pub const TTFCFP_LANG_KEEP_ALL: u32 = 0u32;
pub const TTFCFP_MS_PLATFORMID: u32 = 3u32;
pub const TTFCFP_SUBSET: u32 = 0u32;
pub const TTFCFP_SUBSET1: u32 = 1u32;
pub const TTFMFP_DELTA: u32 = 2u32;
pub const TTFMFP_SUBSET: u32 = 0u32;
pub const TTFMFP_SUBSET1: u32 = 1u32;
pub struct TTLOADINFO(i32);
pub struct TTLOAD_EMBEDDED_FONT_STATUS(i32);
pub const TTLOAD_EUDC_OVERWRITE: u32 = 2u32;
pub const TTLOAD_EUDC_SET: u32 = 4u32;
pub const TTLOAD_PRIVATE: u32 = 1u32;
pub struct TTPOLYCURVE(i32);
pub struct TTPOLYGONHEADER(i32);
pub struct TTVALIDATIONTESTSPARAMS(i32);
pub struct TTVALIDATIONTESTSPARAMSEX(i32);
pub const TT_AVAILABLE: u32 = 1u32;
pub const TT_ENABLED: u32 = 2u32;
pub const TT_POLYGON_TYPE: u32 = 24u32;
pub const TT_PRIM_CSPLINE: u32 = 3u32;
pub const TT_PRIM_LINE: u32 = 1u32;
pub const TT_PRIM_QSPLINE: u32 = 2u32;
pub const TURKISH_CHARSET: u32 = 162u32;
pub const VARIABLE_PITCH: u32 = 2u32;
pub const VIETNAMESE_CHARSET: u32 = 163u32;
pub struct WCRANGE(i32);
pub struct WGLSWAP(i32);
pub const WGL_FONT_LINES: u32 = 0u32;
pub const WGL_FONT_POLYGONS: u32 = 1u32;
pub const WGL_SWAPMULTIPLE_MAX: u32 = 16u32;
pub const WGL_SWAP_MAIN_PLANE: u32 = 1u32;
pub const WGL_SWAP_OVERLAY1: u32 = 2u32;
pub const WGL_SWAP_OVERLAY10: u32 = 1024u32;
pub const WGL_SWAP_OVERLAY11: u32 = 2048u32;
pub const WGL_SWAP_OVERLAY12: u32 = 4096u32;
pub const WGL_SWAP_OVERLAY13: u32 = 8192u32;
pub const WGL_SWAP_OVERLAY14: u32 = 16384u32;
pub const WGL_SWAP_OVERLAY15: u32 = 32768u32;
pub const WGL_SWAP_OVERLAY2: u32 = 4u32;
pub const WGL_SWAP_OVERLAY3: u32 = 8u32;
pub const WGL_SWAP_OVERLAY4: u32 = 16u32;
pub const WGL_SWAP_OVERLAY5: u32 = 32u32;
pub const WGL_SWAP_OVERLAY6: u32 = 64u32;
pub const WGL_SWAP_OVERLAY7: u32 = 128u32;
pub const WGL_SWAP_OVERLAY8: u32 = 256u32;
pub const WGL_SWAP_OVERLAY9: u32 = 512u32;
pub const WGL_SWAP_UNDERLAY1: u32 = 65536u32;
pub const WGL_SWAP_UNDERLAY10: u32 = 33554432u32;
pub const WGL_SWAP_UNDERLAY11: u32 = 67108864u32;
pub const WGL_SWAP_UNDERLAY12: u32 = 134217728u32;
pub const WGL_SWAP_UNDERLAY13: u32 = 268435456u32;
pub const WGL_SWAP_UNDERLAY14: u32 = 536870912u32;
pub const WGL_SWAP_UNDERLAY15: u32 = 1073741824u32;
pub const WGL_SWAP_UNDERLAY2: u32 = 131072u32;
pub const WGL_SWAP_UNDERLAY3: u32 = 262144u32;
pub const WGL_SWAP_UNDERLAY4: u32 = 524288u32;
pub const WGL_SWAP_UNDERLAY5: u32 = 1048576u32;
pub const WGL_SWAP_UNDERLAY6: u32 = 2097152u32;
pub const WGL_SWAP_UNDERLAY7: u32 = 4194304u32;
pub const WGL_SWAP_UNDERLAY8: u32 = 8388608u32;
pub const WGL_SWAP_UNDERLAY9: u32 = 16777216u32;
pub struct WRITEEMBEDPROC(i32);
pub struct XFORM(i32);
