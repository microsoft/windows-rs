#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn AbortDoc(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn AbortPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("gdi32.dll" "system" fn AddFontMemResourceEx(pfileview : *const core::ffi::c_void, cjsize : u32, pvresrved : *const core::ffi::c_void, pnumfonts : *const u32) -> super::HANDLE);
windows_link::link!("gdi32.dll" "system" fn AddFontResourceA(param0 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("gdi32.dll" "system" fn AddFontResourceExA(name : windows_sys::core::PCSTR, fl : u32, res : *const core::ffi::c_void) -> i32);
windows_link::link!("gdi32.dll" "system" fn AddFontResourceExW(name : windows_sys::core::PCWSTR, fl : u32, res : *const core::ffi::c_void) -> i32);
windows_link::link!("gdi32.dll" "system" fn AddFontResourceW(param0 : windows_sys::core::PCWSTR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("msimg32.dll" "system" fn AlphaBlend(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, ftn : BLENDFUNCTION) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn AngleArc(hdc : super::HDC, x : i32, y : i32, r : u32, startangle : f32, sweepangle : f32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn AnimatePalette(hpal : super::HPALETTE, istartindex : u32, centries : u32, ppe : *const PALETTEENTRY) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Arc(hdc : super::HDC, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ArcTo(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32, xr1 : i32, yr1 : i32, xr2 : i32, yr2 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn BeginPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn BitBlt(hdc : super::HDC, x : i32, y : i32, cx : i32, cy : i32, hdcsrc : super::HDC, x1 : i32, y1 : i32, rop : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CancelDC(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CheckColorsInGamut(hdc : super::HDC, lprgbtriple : *const RGBTRIPLE, dlpbuffer : *mut core::ffi::c_void, ncount : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ChoosePixelFormat(hdc : super::HDC, ppfd : *const PIXELFORMATDESCRIPTOR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Chord(hdc : super::HDC, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CloseEnhMetaFile(hdc : super::HDC) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CloseFigure(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CloseMetaFile(hdc : super::HDC) -> super::HMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ColorCorrectPalette(hdc : super::HDC, hpal : super::HPALETTE, defirst : u32, num : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ColorMatchToTarget(hdc : super::HDC, hdctarget : super::HDC, action : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CombineRgn(hrgndst : super::HRGN, hrgnsrc1 : super::HRGN, hrgnsrc2 : super::HRGN, imode : i32) -> i32);
windows_link::link!("gdi32.dll" "system" fn CombineTransform(lpxfout : *mut XFORM, lpxf1 : *const XFORM, lpxf2 : *const XFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CopyEnhMetaFileA(henh : super::HENHMETAFILE, lpfilename : windows_sys::core::PCSTR) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CopyEnhMetaFileW(henh : super::HENHMETAFILE, lpfilename : windows_sys::core::PCWSTR) -> super::HENHMETAFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CopyMetaFileA(param0 : super::HMETAFILE, param1 : windows_sys::core::PCSTR) -> super::HMETAFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CopyMetaFileW(param0 : super::HMETAFILE, param1 : windows_sys::core::PCWSTR) -> super::HMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateBitmap(nwidth : i32, nheight : i32, nplanes : u32, nbitcount : u32, lpbits : *const core::ffi::c_void) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateBitmapIndirect(pbm : *const BITMAP) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateBrushIndirect(plbrush : *const LOGBRUSH) -> super::HBRUSH);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateColorSpaceA(lplcs : *const LOGCOLORSPACEA) -> super::HCOLORSPACE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateColorSpaceW(lplcs : *const LOGCOLORSPACEW) -> super::HCOLORSPACE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleBitmap(hdc : super::HDC, cx : i32, cy : i32) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleDC(hdc : super::HDC) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateDCA(pwszdriver : windows_sys::core::PCSTR, pwszdevice : windows_sys::core::PCSTR, pszport : windows_sys::core::PCSTR, pdm : *const DEVMODEA) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateDCW(pwszdriver : windows_sys::core::PCWSTR, pwszdevice : windows_sys::core::PCWSTR, pszport : windows_sys::core::PCWSTR, pdm : *const DEVMODEW) -> super::HDC);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateDIBPatternBrush(h : super::HGLOBAL, iusage : u32) -> super::HBRUSH);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateDIBPatternBrushPt(lppackeddib : *const core::ffi::c_void, iusage : u32) -> super::HBRUSH);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateDIBSection(hdc : super::HDC, pbmi : *const BITMAPINFO, usage : u32, ppvbits : *mut *mut core::ffi::c_void, hsection : super::HANDLE, offset : u32) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateDIBitmap(hdc : super::HDC, pbmih : *const BITMAPINFOHEADER, flinit : u32, pjbits : *const core::ffi::c_void, pbmi : *const BITMAPINFO, iusage : u32) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateDiscardableBitmap(hdc : super::HDC, cx : i32, cy : i32) -> super::HBITMAP);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CreateEllipticRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> super::HRGN);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateEllipticRgnIndirect(lprect : *const super::RECT) -> super::HRGN);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateEnhMetaFileA(hdc : super::HDC, lpfilename : windows_sys::core::PCSTR, lprc : *const super::RECT, lpdesc : windows_sys::core::PCSTR) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateEnhMetaFileW(hdc : super::HDC, lpfilename : windows_sys::core::PCWSTR, lprc : *const super::RECT, lpdesc : windows_sys::core::PCWSTR) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateFontA(cheight : i32, cwidth : i32, cescapement : i32, corientation : i32, cweight : i32, bitalic : u32, bunderline : u32, bstrikeout : u32, icharset : u32, ioutprecision : u32, iclipprecision : u32, iquality : u32, ipitchandfamily : u32, pszfacename : windows_sys::core::PCSTR) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectA(lplf : *const LOGFONTA) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectExA(param0 : *const ENUMLOGFONTEXDVA) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectExW(param0 : *const ENUMLOGFONTEXDVW) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectW(lplf : *const LOGFONTW) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateFontW(cheight : i32, cwidth : i32, cescapement : i32, corientation : i32, cweight : i32, bitalic : u32, bunderline : u32, bstrikeout : u32, icharset : u32, ioutprecision : u32, iclipprecision : u32, iquality : u32, ipitchandfamily : u32, pszfacename : windows_sys::core::PCWSTR) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateHalftonePalette(hdc : super::HDC) -> super::HPALETTE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateHatchBrush(ihatch : i32, color : super::COLORREF) -> super::HBRUSH);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateICA(pszdriver : windows_sys::core::PCSTR, pszdevice : windows_sys::core::PCSTR, pszport : windows_sys::core::PCSTR, pdm : *const DEVMODEA) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateICW(pszdriver : windows_sys::core::PCWSTR, pszdevice : windows_sys::core::PCWSTR, pszport : windows_sys::core::PCWSTR, pdm : *const DEVMODEW) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateMetaFileA(pszfile : windows_sys::core::PCSTR) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateMetaFileW(pszfile : windows_sys::core::PCWSTR) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreatePalette(plpal : *const LOGPALETTE) -> super::HPALETTE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreatePatternBrush(hbm : super::HBITMAP) -> super::HBRUSH);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreatePen(istyle : i32, cwidth : i32, color : super::COLORREF) -> super::HPEN);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreatePenIndirect(plpen : *const LOGPEN) -> super::HPEN);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreatePolyPolygonRgn(pptl : *const super::POINT, pc : *const i32, cpoly : i32, imode : i32) -> super::HRGN);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreatePolygonRgn(pptl : *const super::POINT, cpoint : i32, imode : i32) -> super::HRGN);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CreateRectRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> super::HRGN);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateRectRgnIndirect(lprect : *const super::RECT) -> super::HRGN);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CreateRoundRectRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32, w : i32, h : i32) -> super::HRGN);
windows_link::link!("gdi32.dll" "system" fn CreateScalableFontResourceA(fdwhidden : u32, lpszfont : windows_sys::core::PCSTR, lpszfile : windows_sys::core::PCSTR, lpszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn CreateScalableFontResourceW(fdwhidden : u32, lpszfont : windows_sys::core::PCWSTR, lpszfile : windows_sys::core::PCWSTR, lpszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateSolidBrush(color : super::COLORREF) -> super::HBRUSH);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DPtoLP(hdc : super::HDC, lppt : *mut super::POINT, c : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DeleteColorSpace(hcs : super::HCOLORSPACE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DeleteDC(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DeleteEnhMetaFile(hmf : super::HENHMETAFILE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn DeleteMetaFile(hmf : super::HMETAFILE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DeleteObject(ho : super::HGDIOBJ) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DescribePixelFormat(hdc : super::HDC, ipixelformat : i32, nbytes : u32, ppfd : *mut PIXELFORMATDESCRIPTOR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn DeviceCapabilitiesA(pdevice : windows_sys::core::PCSTR, pport : windows_sys::core::PCSTR, fwcapability : u16, poutput : windows_sys::core::PSTR, pdevmode : *const DEVMODEA) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("winspool.drv" "system" fn DeviceCapabilitiesW(pdevice : windows_sys::core::PCWSTR, pport : windows_sys::core::PCWSTR, fwcapability : u16, poutput : windows_sys::core::PWSTR, pdevmode : *const DEVMODEW) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn DrawEscape(hdc : super::HDC, iescape : i32, cjin : i32, lpin : windows_sys::core::PCSTR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Ellipse(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn EndDoc(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn EndPage(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn EndPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumEnhMetaFile(hdc : super::HDC, hmf : super::HENHMETAFILE, proc : ENHMFENUMPROC, param : *const core::ffi::c_void, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesA(hdc : super::HDC, lplogfont : windows_sys::core::PCSTR, lpproc : FONTENUMPROCA, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesExA(hdc : super::HDC, lplogfont : *const LOGFONTA, lpproc : FONTENUMPROCA, lparam : super::LPARAM, dwflags : u32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesExW(hdc : super::HDC, lplogfont : *const LOGFONTW, lpproc : FONTENUMPROCW, lparam : super::LPARAM, dwflags : u32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesW(hdc : super::HDC, lplogfont : windows_sys::core::PCWSTR, lpproc : FONTENUMPROCW, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontsA(hdc : super::HDC, lplogfont : windows_sys::core::PCSTR, lpproc : FONTENUMPROCA, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontsW(hdc : super::HDC, lplogfont : windows_sys::core::PCWSTR, lpproc : FONTENUMPROCW, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumICMProfilesA(hdc : super::HDC, proc : ICMENUMPROCA, param : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumICMProfilesW(hdc : super::HDC, proc : ICMENUMPROCW, param : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumMetaFile(hdc : super::HDC, hmf : super::HMETAFILE, proc : MFENUMPROC, param : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumObjects(hdc : super::HDC, ntype : i32, lpfunc : GOBJENUMPROC, lparam : super::LPARAM) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn EqualRgn(hrgn1 : super::HRGN, hrgn2 : super::HRGN) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Escape(hdc : super::HDC, iescape : i32, cjin : i32, pvin : windows_sys::core::PCSTR, pvout : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExcludeClipRect(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExtCreatePen(ipenstyle : u32, cwidth : u32, plbrush : *const LOGBRUSH, cstyle : u32, pstyle : *const u32) -> super::HPEN);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtCreateRegion(lpx : *const XFORM, ncount : u32, lpdata : *const RGNDATA) -> super::HRGN);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExtEscape(hdc : super::HDC, iescape : i32, cjinput : i32, lpindata : windows_sys::core::PCSTR, cjoutput : i32, lpoutdata : windows_sys::core::PSTR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExtFloodFill(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF, r#type : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtSelectClipRgn(hdc : super::HDC, hrgn : super::HRGN, mode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExtTextOutA(hdc : super::HDC, x : i32, y : i32, options : u32, lprect : *const super::RECT, lpstring : windows_sys::core::PCSTR, c : u32, lpdx : *const i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExtTextOutW(hdc : super::HDC, x : i32, y : i32, options : u32, lprect : *const super::RECT, lpstring : windows_sys::core::PCWSTR, c : u32, lpdx : *const i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FillPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn FillRgn(hdc : super::HDC, hrgn : super::HRGN, hbr : super::HBRUSH) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FixBrushOrgEx(hdc : super::HDC, x : i32, y : i32, ptl : *const super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FlattenPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FloodFill(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn FrameRgn(hdc : super::HDC, hrgn : super::HRGN, hbr : super::HBRUSH, w : i32, h : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GdiAlphaBlend(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, ftn : BLENDFUNCTION) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GdiComment(hdc : super::HDC, nsize : u32, lpdata : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiFlush() -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiGetBatchLimit() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GdiGradientFill(hdc : super::HDC, pvertex : *const TRIVERTEX, nvertex : u32, pmesh : *const core::ffi::c_void, ncount : u32, ulmode : u32) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiSetBatchLimit(dw : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GdiTransparentBlt(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, crtransparent : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetArcDirection(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetAspectRatioFilterEx(hdc : super::HDC, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBitmapBits(hbit : super::HBITMAP, cb : i32, lpvbits : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBitmapDimensionEx(hbit : super::HBITMAP, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBkColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBkMode(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBoundsRect(hdc : super::HDC, lprect : *mut super::RECT, flags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBrushOrgEx(hdc : super::HDC, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsA(hdc : super::HDC, wfirst : u32, wlast : u32, lpabc : *mut ABC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsFloatA(hdc : super::HDC, ifirst : u32, ilast : u32, lpabc : *mut ABCFLOAT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsFloatW(hdc : super::HDC, ifirst : u32, ilast : u32, lpabc : *mut ABCFLOAT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsI(hdc : super::HDC, gifirst : u32, cgi : u32, pgi : *const u16, pabc : *mut ABC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsW(hdc : super::HDC, wfirst : u32, wlast : u32, lpabc : *mut ABC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidth32A(hdc : super::HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidth32W(hdc : super::HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthA(hdc : super::HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthFloatA(hdc : super::HDC, ifirst : u32, ilast : u32, lpbuffer : *mut f32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthFloatW(hdc : super::HDC, ifirst : u32, ilast : u32, lpbuffer : *mut f32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthI(hdc : super::HDC, gifirst : u32, cgi : u32, pgi : *const u16, piwidths : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthW(hdc : super::HDC, ifirst : u32, ilast : u32, lpbuffer : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharacterPlacementA(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, ncount : i32, nmexextent : i32, lpresults : *mut GCP_RESULTSA, dwflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCharacterPlacementW(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, ncount : i32, nmexextent : i32, lpresults : *mut GCP_RESULTSW, dwflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetClipBox(hdc : super::HDC, lprect : *mut super::RECT) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetClipRgn(hdc : super::HDC, hrgn : super::HRGN) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetColorAdjustment(hdc : super::HDC, lpca : *mut COLORADJUSTMENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetColorSpace(hdc : super::HDC) -> super::HCOLORSPACE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCurrentObject(hdc : super::HDC, r#type : u32) -> super::HGDIOBJ);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetCurrentPositionEx(hdc : super::HDC, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDCBrushColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDCOrgEx(hdc : super::HDC, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDCPenColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDIBColorTable(hdc : super::HDC, istart : u32, centries : u32, prgbq : *mut RGBQUAD) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDIBits(hdc : super::HDC, hbm : super::HBITMAP, start : u32, clines : u32, lpvbits : *mut core::ffi::c_void, lpbmi : *mut BITMAPINFO, usage : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDeviceCaps(hdc : super::HDC, index : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDeviceGammaRamp(hdc : super::HDC, lpramp : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileA(lpname : windows_sys::core::PCSTR) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileBits(hemf : super::HENHMETAFILE, nsize : u32, lpdata : *mut u8) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileDescriptionA(hemf : super::HENHMETAFILE, cchbuffer : u32, lpdescription : windows_sys::core::PSTR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileDescriptionW(hemf : super::HENHMETAFILE, cchbuffer : u32, lpdescription : windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileHeader(hemf : super::HENHMETAFILE, nsize : u32, lpenhmetaheader : *mut ENHMETAHEADER) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFilePaletteEntries(hemf : super::HENHMETAFILE, nnumentries : u32, lppaletteentries : *mut PALETTEENTRY) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFilePixelFormat(hemf : super::HENHMETAFILE, cbbuffer : u32, ppfd : *mut PIXELFORMATDESCRIPTOR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileW(lpname : windows_sys::core::PCWSTR) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetFontData(hdc : super::HDC, dwtable : u32, dwoffset : u32, pvbuffer : *mut core::ffi::c_void, cjbuffer : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetFontLanguageInfo(hdc : super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetFontUnicodeRanges(hdc : super::HDC, lpgs : *mut GLYPHSET) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetGlyphIndicesA(hdc : super::HDC, lpstr : windows_sys::core::PCSTR, c : i32, pgi : *mut u16, fl : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetGlyphIndicesW(hdc : super::HDC, lpstr : windows_sys::core::PCWSTR, c : i32, pgi : *mut u16, fl : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetGlyphOutlineA(hdc : super::HDC, uchar : u32, fuformat : u32, lpgm : *mut GLYPHMETRICS, cjbuffer : u32, pvbuffer : *mut core::ffi::c_void, lpmat2 : *const MAT2) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetGlyphOutlineW(hdc : super::HDC, uchar : u32, fuformat : u32, lpgm : *mut GLYPHMETRICS, cjbuffer : u32, pvbuffer : *mut core::ffi::c_void, lpmat2 : *const MAT2) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetGraphicsMode(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetICMProfileA(hdc : super::HDC, pbufsize : *mut u32, pszfilename : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetICMProfileW(hdc : super::HDC, pbufsize : *mut u32, pszfilename : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetKerningPairsA(hdc : super::HDC, npairs : u32, lpkernpair : *mut KERNINGPAIR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetKerningPairsW(hdc : super::HDC, npairs : u32, lpkernpair : *mut KERNINGPAIR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetLayout(hdc : super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetLogColorSpaceA(hcolorspace : super::HCOLORSPACE, lpbuffer : *mut LOGCOLORSPACEA, nsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetLogColorSpaceW(hcolorspace : super::HCOLORSPACE, lpbuffer : *mut LOGCOLORSPACEW, nsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetMapMode(hdc : super::HDC) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn GetMetaFileA(lpname : windows_sys::core::PCSTR) -> super::HMETAFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn GetMetaFileBitsEx(hmf : super::HMETAFILE, cbbuffer : u32, lpdata : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn GetMetaFileW(lpname : windows_sys::core::PCWSTR) -> super::HMETAFILE);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetMetaRgn(hdc : super::HDC, hrgn : super::HRGN) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetMiterLimit(hdc : super::HDC, plimit : *mut f32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetNearestColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetNearestPaletteIndex(h : super::HPALETTE, color : super::COLORREF) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("gdi32.dll" "system" fn GetObjectA(h : super::HANDLE, c : i32, pv : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetObjectType(h : super::HGDIOBJ) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("gdi32.dll" "system" fn GetObjectW(h : super::HANDLE, c : i32, pv : *mut core::ffi::c_void) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetOutlineTextMetricsA(hdc : super::HDC, cjcopy : u32, potm : *mut OUTLINETEXTMETRICA) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetOutlineTextMetricsW(hdc : super::HDC, cjcopy : u32, potm : *mut OUTLINETEXTMETRICW) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPaletteEntries(hpal : super::HPALETTE, istart : u32, centries : u32, ppalentries : *mut PALETTEENTRY) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPath(hdc : super::HDC, apt : *mut super::POINT, aj : *mut u8, cpt : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPixel(hdc : super::HDC, x : i32, y : i32) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPixelFormat(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPolyFillMode(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetROP2(hdc : super::HDC) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetRandomRgn(hdc : super::HDC, hrgn : super::HRGN, i : i32) -> i32);
windows_link::link!("gdi32.dll" "system" fn GetRasterizerCaps(lpraststat : *mut RASTERIZER_STATUS, cjbytes : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetRegionData(hrgn : super::HRGN, ncount : u32, lprgndata : *mut RGNDATA) -> u32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetRgnBox(hrgn : super::HRGN, lprc : *mut super::RECT) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetStockObject(i : i32) -> super::HGDIOBJ);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetStretchBltMode(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetSystemPaletteEntries(hdc : super::HDC, istart : u32, centries : u32, ppalentries : *mut PALETTEENTRY) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetSystemPaletteUse(hdc : super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextAlign(hdc : super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextCharacterExtra(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextCharset(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextCharsetInfo(hdc : super::HDC, lpsig : *mut FONTSIGNATURE, dwflags : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointA(hdc : super::HDC, lpszstring : windows_sys::core::PCSTR, cchstring : i32, nmaxextent : i32, lpnfit : *mut i32, lpndx : *mut i32, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointI(hdc : super::HDC, lpwszstring : *const u16, cwchstring : i32, nmaxextent : i32, lpnfit : *mut i32, lpndx : *mut i32, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointW(hdc : super::HDC, lpszstring : windows_sys::core::PCWSTR, cchstring : i32, nmaxextent : i32, lpnfit : *mut i32, lpndx : *mut i32, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPoint32A(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, c : i32, psizl : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPoint32W(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, c : i32, psizl : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointA(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, c : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointI(hdc : super::HDC, pgiin : *const u16, cgi : i32, psize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointW(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, c : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextFaceA(hdc : super::HDC, c : i32, lpname : windows_sys::core::PSTR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextFaceW(hdc : super::HDC, c : i32, lpname : windows_sys::core::PWSTR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextMetricsA(hdc : super::HDC, lptm : *mut TEXTMETRICA) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextMetricsW(hdc : super::HDC, lptm : *mut TEXTMETRICW) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetViewportExtEx(hdc : super::HDC, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetViewportOrgEx(hdc : super::HDC, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetWinMetaFileBits(hemf : super::HENHMETAFILE, cbdata16 : u32, pdata16 : *mut u8, imapmode : i32, hdcref : super::HDC) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetWindowExtEx(hdc : super::HDC, lpsize : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetWindowOrgEx(hdc : super::HDC, lppoint : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetWorldTransform(hdc : super::HDC, lpxf : *mut XFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("msimg32.dll" "system" fn GradientFill(hdc : super::HDC, pvertex : *const TRIVERTEX, nvertex : u32, pmesh : *const core::ffi::c_void, nmesh : u32, ulmode : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn IntersectClipRect(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn InvertRgn(hdc : super::HDC, hrgn : super::HRGN) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn LPtoDP(hdc : super::HDC, lppt : *mut super::POINT, c : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn LineDDA(xstart : i32, ystart : i32, xend : i32, yend : i32, lpproc : LINEDDAPROC, data : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn LineTo(hdc : super::HDC, x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn MaskBlt(hdcdest : super::HDC, xdest : i32, ydest : i32, width : i32, height : i32, hdcsrc : super::HDC, xsrc : i32, ysrc : i32, hbmmask : super::HBITMAP, xmask : i32, ymask : i32, rop : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ModifyWorldTransform(hdc : super::HDC, lpxf : *const XFORM, mode : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn MoveToEx(hdc : super::HDC, x : i32, y : i32, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn OffsetClipRgn(hdc : super::HDC, x : i32, y : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn OffsetRgn(hrgn : super::HRGN, x : i32, y : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn OffsetViewportOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn OffsetWindowOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PaintRgn(hdc : super::HDC, hrgn : super::HRGN) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PatBlt(hdc : super::HDC, x : i32, y : i32, w : i32, h : i32, rop : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PathToRegion(hdc : super::HDC) -> super::HRGN);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Pie(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32, xr1 : i32, yr1 : i32, xr2 : i32, yr2 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PlayEnhMetaFile(hdc : super::HDC, hmf : super::HENHMETAFILE, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PlayEnhMetaFileRecord(hdc : super::HDC, pht : *const HANDLETABLE, pmr : *const ENHMETARECORD, cht : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PlayMetaFile(hdc : super::HDC, hmf : super::HMETAFILE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PlayMetaFileRecord(hdc : super::HDC, lphandletable : *const HANDLETABLE, lpmr : *const METARECORD, noobjs : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PlgBlt(hdcdest : super::HDC, lppoint : *const super::POINT, hdcsrc : super::HDC, xsrc : i32, ysrc : i32, width : i32, height : i32, hbmmask : super::HBITMAP, xmask : i32, ymask : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyBezier(hdc : super::HDC, apt : *const super::POINT, cpt : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyBezierTo(hdc : super::HDC, apt : *const super::POINT, cpt : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyDraw(hdc : super::HDC, apt : *const super::POINT, aj : *const u8, cpt : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyPolygon(hdc : super::HDC, apt : *const super::POINT, asz : *const i32, csz : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyPolyline(hdc : super::HDC, apt : *const super::POINT, asz : *const u32, csz : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyTextOutA(hdc : super::HDC, ppt : *const POLYTEXTA, nstrings : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolyTextOutW(hdc : super::HDC, ppt : *const POLYTEXTW, nstrings : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Polygon(hdc : super::HDC, apt : *const super::POINT, cpt : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Polyline(hdc : super::HDC, apt : *const super::POINT, cpt : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PolylineTo(hdc : super::HDC, apt : *const super::POINT, cpt : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn PtInRegion(hrgn : super::HRGN, x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PtVisible(hdc : super::HDC, x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn RealizePalette(hdc : super::HDC) -> u32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn RectInRegion(hrgn : super::HRGN, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn RectVisible(hdc : super::HDC, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Rectangle(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("gdi32.dll" "system" fn RemoveFontMemResourceEx(h : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceA(lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceExA(name : windows_sys::core::PCSTR, fl : u32, pdv : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceExW(name : windows_sys::core::PCWSTR, fl : u32, pdv : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ResetDCA(hdc : super::HDC, lpdm : *const DEVMODEA) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ResetDCW(hdc : super::HDC, lpdm : *const DEVMODEW) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ResizePalette(hpal : super::HPALETTE, n : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn RestoreDC(hdc : super::HDC, nsaveddc : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn RoundRect(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32, width : i32, height : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SaveDC(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ScaleViewportExtEx(hdc : super::HDC, xn : i32, dx : i32, yn : i32, yd : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ScaleWindowExtEx(hdc : super::HDC, xn : i32, xd : i32, yn : i32, yd : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SelectClipPath(hdc : super::HDC, mode : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SelectClipRgn(hdc : super::HDC, hrgn : super::HRGN) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SelectObject(hdc : super::HDC, h : super::HGDIOBJ) -> super::HGDIOBJ);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SelectPalette(hdc : super::HDC, hpal : super::HPALETTE, bforcebkgd : windows_sys::core::BOOL) -> super::HPALETTE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetAbortProc(hdc : super::HDC, proc : ABORTPROC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetArcDirection(hdc : super::HDC, dir : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBitmapBits(hbm : super::HBITMAP, cb : u32, pvbits : *const core::ffi::c_void) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBitmapDimensionEx(hbm : super::HBITMAP, w : i32, h : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBkColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBkMode(hdc : super::HDC, mode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBoundsRect(hdc : super::HDC, lprect : *const super::RECT, flags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBrushOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetColorAdjustment(hdc : super::HDC, lpca : *const COLORADJUSTMENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetColorSpace(hdc : super::HDC, hcs : super::HCOLORSPACE) -> super::HCOLORSPACE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDCBrushColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDCPenColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDIBColorTable(hdc : super::HDC, istart : u32, centries : u32, prgbq : *const RGBQUAD) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDIBits(hdc : super::HDC, hbm : super::HBITMAP, start : u32, clines : u32, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, coloruse : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDIBitsToDevice(hdc : super::HDC, xdest : i32, ydest : i32, w : u32, h : u32, xsrc : i32, ysrc : i32, startscan : u32, clines : u32, lpvbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, coloruse : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDeviceGammaRamp(hdc : super::HDC, lpramp : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetEnhMetaFileBits(nsize : u32, pb : *const u8) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetGraphicsMode(hdc : super::HDC, imode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetICMMode(hdc : super::HDC, mode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetICMProfileA(hdc : super::HDC, lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetICMProfileW(hdc : super::HDC, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetLayout(hdc : super::HDC, l : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetMapMode(hdc : super::HDC, imode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetMapperFlags(hdc : super::HDC, flags : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn SetMetaFileBitsEx(cbbuffer : u32, lpdata : *const u8) -> super::HMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetMetaRgn(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetMiterLimit(hdc : super::HDC, limit : f32, old : *mut f32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetPaletteEntries(hpal : super::HPALETTE, istart : u32, centries : u32, ppalentries : *const PALETTEENTRY) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetPixel(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetPixelFormat(hdc : super::HDC, format : i32, ppfd : *const PIXELFORMATDESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetPixelV(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetPolyFillMode(hdc : super::HDC, mode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetROP2(hdc : super::HDC, rop2 : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn SetRectRgn(hrgn : super::HRGN, left : i32, top : i32, right : i32, bottom : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetStretchBltMode(hdc : super::HDC, mode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetSystemPaletteUse(hdc : super::HDC, r#use : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextAlign(hdc : super::HDC, align : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextCharacterExtra(hdc : super::HDC, extra : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextJustification(hdc : super::HDC, extra : i32, count : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetViewportExtEx(hdc : super::HDC, x : i32, y : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetViewportOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetWinMetaFileBits(nsize : u32, lpmeta16data : *const u8, hdcref : super::HDC, lpmfp : *const METAFILEPICT) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetWindowExtEx(hdc : super::HDC, x : i32, y : i32, lpsz : *mut super::SIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetWindowOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : *mut super::POINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetWorldTransform(hdc : super::HDC, lpxf : *const XFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StartDocA(hdc : super::HDC, lpdi : *const DOCINFOA) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StartDocW(hdc : super::HDC, lpdi : *const DOCINFOW) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StartPage(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StretchBlt(hdcdest : super::HDC, xdest : i32, ydest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xsrc : i32, ysrc : i32, wsrc : i32, hsrc : i32, rop : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StretchDIBits(hdc : super::HDC, xdest : i32, ydest : i32, destwidth : i32, destheight : i32, xsrc : i32, ysrc : i32, srcwidth : i32, srcheight : i32, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, iusage : u32, rop : u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StrokeAndFillPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StrokePath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SwapBuffers(param0 : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn TextOutA(hdc : super::HDC, x : i32, y : i32, lpstring : windows_sys::core::PCSTR, c : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn TextOutW(hdc : super::HDC, x : i32, y : i32, lpstring : windows_sys::core::PCWSTR, c : i32) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn TranslateCharsetInfo(lpsrc : *mut u32, lpcs : *mut CHARSETINFO, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("msimg32.dll" "system" fn TransparentBlt(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, crtransparent : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn UnrealizeObject(h : super::HGDIOBJ) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn UpdateColors(hdc : super::HDC) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn UpdateICMRegKeyA(reserved : u32, lpszcmid : windows_sys::core::PCSTR, lpszfilename : windows_sys::core::PCSTR, command : u32) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn UpdateICMRegKeyW(reserved : u32, lpszcmid : windows_sys::core::PCWSTR, lpszfilename : windows_sys::core::PCWSTR, command : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn WidenPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglCopyContext(param0 : super::HGLRC, param1 : super::HGLRC, param2 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglCreateContext(param0 : super::HDC) -> super::HGLRC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglCreateLayerContext(param0 : super::HDC, param1 : i32) -> super::HGLRC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglDeleteContext(param0 : super::HGLRC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglDescribeLayerPlane(param0 : super::HDC, param1 : i32, param2 : i32, param3 : u32, param4 : *mut LAYERPLANEDESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglGetCurrentContext() -> super::HGLRC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglGetCurrentDC() -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglGetLayerPaletteEntries(param0 : super::HDC, param1 : i32, param2 : i32, param3 : i32, param4 : *mut super::COLORREF) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("opengl32.dll" "system" fn wglGetProcAddress(param0 : windows_sys::core::PCSTR) -> super::PROC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglMakeCurrent(param0 : super::HDC, param1 : super::HGLRC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglRealizeLayerPalette(param0 : super::HDC, param1 : i32, param2 : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglSetLayerPaletteEntries(param0 : super::HDC, param1 : i32, param2 : i32, param3 : i32, param4 : *const super::COLORREF) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglShareLists(param0 : super::HGLRC, param1 : super::HGLRC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglSwapLayerBuffers(param0 : super::HDC, param1 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglSwapMultipleBuffers(param0 : u32, param1 : *const WGLSWAP) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglUseFontBitmapsA(param0 : super::HDC, param1 : u32, param2 : u32, param3 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglUseFontBitmapsW(param0 : super::HDC, param1 : u32, param2 : u32, param3 : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglUseFontOutlinesA(param0 : super::HDC, param1 : u32, param2 : u32, param3 : u32, param4 : f32, param5 : f32, param6 : i32, param7 : *mut GLYPHMETRICSFLOAT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglUseFontOutlinesW(param0 : super::HDC, param1 : u32, param2 : u32, param3 : u32, param4 : f32, param5 : f32, param6 : i32, param7 : *mut GLYPHMETRICSFLOAT) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ABC {
    pub abcA: i32,
    pub abcB: u32,
    pub abcC: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ABCFLOAT {
    pub abcfA: f32,
    pub abcfB: f32,
    pub abcfC: f32,
}
pub const ABORTDOC: u32 = 2;
#[cfg(feature = "windef")]
pub type ABORTPROC = Option<unsafe extern "system" fn(param0: super::HDC, param1: i32) -> windows_sys::core::BOOL>;
pub const ABSOLUTE: u32 = 1;
pub const AC_SRC_ALPHA: u32 = 1;
pub const AC_SRC_OVER: u32 = 0;
pub const AD_CLOCKWISE: u32 = 2;
pub const AD_COUNTERCLOCKWISE: u32 = 1;
pub const ALTERNATE: u32 = 1;
pub const ANSI_CHARSET: u32 = 0;
pub const ANSI_FIXED_FONT: u32 = 11;
pub const ANSI_VAR_FONT: u32 = 12;
pub const ANTIALIASED_QUALITY: u32 = 4;
pub const ARABIC_CHARSET: u32 = 178;
pub const ASPECTX: u32 = 40;
pub const ASPECTXY: u32 = 44;
pub const ASPECTY: u32 = 42;
pub const ASPECT_FILTERING: u32 = 1;
pub type AXESLIST = AXESLISTA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AXESLISTA {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOA; 16],
}
impl Default for AXESLISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AXESLISTW {
    pub axlReserved: u32,
    pub axlNumAxes: u32,
    pub axlAxisInfo: [AXISINFOW; 16],
}
impl Default for AXESLISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AXISINFO = AXISINFOA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AXISINFOA {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u8; 16],
}
impl Default for AXISINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AXISINFOW {
    pub axMinValue: i32,
    pub axMaxValue: i32,
    pub axAxisName: [u16; 16],
}
impl Default for AXISINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BALTIC_CHARSET: u32 = 186;
pub const BANDINFO: u32 = 24;
pub type BCHAR = u8;
pub const BEGIN_PATH: u32 = 4096;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub bmBits: *mut core::ffi::c_void,
}
impl Default for BITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BITMAPCOREHEADER {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPCOREINFO {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
impl Default for BITMAPCOREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct BITMAPFILEHEADER {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
impl Default for BITMAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const BITSPIXEL: u32 = 12;
pub const BI_BITFIELDS: u32 = 3;
pub const BI_JPEG: u32 = 4;
pub const BI_PNG: u32 = 5;
pub const BI_RGB: u32 = 0;
pub const BI_RLE4: u32 = 2;
pub const BI_RLE8: u32 = 1;
pub const BKMODE_LAST: u32 = 2;
pub const BLACKNESS: u32 = 66;
pub const BLACKONWHITE: u32 = 1;
pub const BLACK_BRUSH: u32 = 4;
pub const BLACK_PEN: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BLENDFUNCTION {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
}
pub const BLTALIGNMENT: u32 = 119;
pub const BS_DIBPATTERN: u32 = 5;
pub const BS_DIBPATTERN8X8: u32 = 8;
pub const BS_DIBPATTERNPT: u32 = 6;
pub const BS_HATCHED: u32 = 2;
pub const BS_HOLLOW: u32 = 1;
pub const BS_INDEXED: u32 = 4;
pub const BS_MONOPATTERN: u32 = 9;
pub const BS_NULL: u32 = 1;
pub const BS_PATTERN: u32 = 3;
pub const BS_PATTERN8X8: u32 = 7;
pub const BS_SOLID: u32 = 0;
pub const CAPTUREBLT: u32 = 1073741824;
pub const CA_LOG_FILTER: u32 = 2;
pub const CA_NEGATIVE: u32 = 1;
pub const CBM_INIT: u32 = 4;
pub const CCHDEVICENAME: u32 = 32;
pub const CCHFORMNAME: u32 = 32;
pub const CC_CHORD: u32 = 4;
pub const CC_CIRCLES: u32 = 1;
pub const CC_ELLIPSES: u32 = 8;
pub const CC_INTERIORS: u32 = 128;
pub const CC_NONE: u32 = 0;
pub const CC_PIE: u32 = 2;
pub const CC_ROUNDRECT: u32 = 256;
pub const CC_STYLED: u32 = 32;
pub const CC_WIDE: u32 = 16;
pub const CC_WIDESTYLED: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHARSETINFO {
    pub ciCharset: u32,
    pub ciACP: u32,
    pub fs: FONTSIGNATURE,
}
pub const CHECKJPEGFORMAT: u32 = 4119;
pub const CHECKPNGFORMAT: u32 = 4120;
pub const CHINESEBIG5_CHARSET: u32 = 136;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CIEXYZ {
    pub ciexyzX: FXPT2DOT30,
    pub ciexyzY: FXPT2DOT30,
    pub ciexyzZ: FXPT2DOT30,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
pub const CLEARTYPE_NATURAL_QUALITY: u32 = 6;
pub const CLEARTYPE_QUALITY: u32 = 5;
pub const CLIPCAPS: u32 = 36;
pub const CLIP_CHARACTER_PRECIS: u32 = 1;
pub const CLIP_DEFAULT_PRECIS: u32 = 0;
pub const CLIP_DFA_DISABLE: u32 = 64;
pub const CLIP_EMBEDDED: u32 = 128;
pub const CLIP_LH_ANGLES: u32 = 16;
pub const CLIP_MASK: u32 = 15;
pub const CLIP_STROKE_PRECIS: u32 = 2;
pub const CLIP_TO_PATH: u32 = 4097;
pub const CLIP_TT_ALWAYS: u32 = 32;
pub const CLOSECHANNEL: u32 = 4112;
pub const CLR_INVALID: u32 = 4294967295;
pub const CM_CMYK_COLOR: u32 = 4;
pub const CM_DEVICE_ICM: u32 = 1;
pub const CM_GAMMA_RAMP: u32 = 2;
pub const CM_IN_GAMUT: u32 = 0;
pub const CM_NONE: u32 = 0;
pub const CM_OUT_OF_GAMUT: u32 = 255;
pub type COLOR16 = u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const COLORMATCHTOTARGET_EMBEDED: u32 = 1;
pub const COLORMGMTCAPS: u32 = 121;
pub const COLORONCOLOR: u32 = 3;
pub const COLORRES: u32 = 108;
pub const COLOR_ADJ_MAX: i16 = 100;
pub const COLOR_ADJ_MIN: i16 = -100;
pub const COMPLEXREGION: u32 = 3;
pub const CP_NONE: u32 = 0;
pub const CP_RECTANGLE: u32 = 1;
pub const CP_REGION: u32 = 2;
pub const CREATECOLORSPACE_EMBEDED: u32 = 1;
pub const CS_DELETE_TRANSFORM: u32 = 3;
pub const CS_DISABLE: u32 = 2;
pub const CS_ENABLE: u32 = 1;
pub const CURVECAPS: u32 = 28;
pub const DCBA_FACEDOWNCENTER: u32 = 257;
pub const DCBA_FACEDOWNLEFT: u32 = 258;
pub const DCBA_FACEDOWNNONE: u32 = 256;
pub const DCBA_FACEDOWNRIGHT: u32 = 259;
pub const DCBA_FACEUPCENTER: u32 = 1;
pub const DCBA_FACEUPLEFT: u32 = 2;
pub const DCBA_FACEUPNONE: u32 = 0;
pub const DCBA_FACEUPRIGHT: u32 = 3;
pub const DCB_ACCUMULATE: u32 = 2;
pub const DCB_DIRTY: u32 = 2;
pub const DCB_DISABLE: u32 = 8;
pub const DCB_ENABLE: u32 = 4;
pub const DCB_RESET: u32 = 1;
pub const DCB_SET: u32 = 3;
pub const DCTT_BITMAP: u32 = 1;
pub const DCTT_DOWNLOAD: u32 = 2;
pub const DCTT_DOWNLOAD_OUTLINE: u32 = 8;
pub const DCTT_SUBDEV: u32 = 4;
pub const DC_BINADJUST: u32 = 19;
pub const DC_BRUSH: u32 = 18;
pub const DC_COLLATE: u32 = 22;
pub const DC_COLORDEVICE: u32 = 32;
pub const DC_DATATYPE_PRODUCED: u32 = 21;
pub const DC_EMF_COMPLIANT: u32 = 20;
pub const DC_MANUFACTURER: u32 = 23;
pub const DC_MEDIAREADY: u32 = 29;
pub const DC_MEDIATYPENAMES: u32 = 34;
pub const DC_MEDIATYPES: u32 = 35;
pub const DC_MODEL: u32 = 24;
pub const DC_NUP: u32 = 33;
pub const DC_PEN: u32 = 19;
pub const DC_PERSONALITY: u32 = 25;
pub const DC_PRINTERMEM: u32 = 28;
pub const DC_PRINTRATE: u32 = 26;
pub const DC_PRINTRATEPPM: u32 = 31;
pub const DC_PRINTRATEUNIT: u32 = 27;
pub const DC_STAPLE: u32 = 30;
pub const DEFAULT_CHARSET: u32 = 1;
pub const DEFAULT_GUI_FONT: u32 = 17;
pub const DEFAULT_PALETTE: u32 = 15;
pub const DEFAULT_PITCH: u32 = 0;
pub const DEFAULT_QUALITY: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DESIGNVECTOR {
    pub dvReserved: u32,
    pub dvNumAxes: u32,
    pub dvValues: [i32; 16],
}
impl Default for DESIGNVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DESKTOPHORZRES: u32 = 118;
pub const DESKTOPVERTRES: u32 = 117;
pub const DEVICEDATA: u32 = 19;
pub const DEVICE_DEFAULT_FONT: u32 = 14;
pub const DEVICE_FONTTYPE: u32 = 2;
#[cfg(feature = "windef")]
pub type DEVMODE = DEVMODEA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DEVMODEA {
    pub dmDeviceName: [u8; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous: DEVMODEA_0,
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
#[cfg(feature = "windef")]
impl Default for DEVMODEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union DEVMODEA_0 {
    pub Anonymous: DEVMODEA_0_0,
    pub Anonymous2: DEVMODEA_0_1,
}
#[cfg(feature = "windef")]
impl Default for DEVMODEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DEVMODEA_0_1 {
    pub dmPosition: super::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union DEVMODEA_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
#[cfg(feature = "windef")]
impl Default for DEVMODEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct DEVMODEW {
    pub dmDeviceName: [u16; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: u32,
    pub Anonymous: DEVMODEW_0,
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
#[cfg(feature = "windef")]
impl Default for DEVMODEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union DEVMODEW_0 {
    pub Anonymous: DEVMODEW_0_0,
    pub Anonymous2: DEVMODEW_0_1,
}
#[cfg(feature = "windef")]
impl Default for DEVMODEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DEVMODEW_0_1 {
    pub dmPosition: super::POINTL,
    pub dmDisplayOrientation: u32,
    pub dmDisplayFixedOutput: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union DEVMODEW_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
#[cfg(feature = "windef")]
impl Default for DEVMODEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DIBSECTION {
    pub dsBm: BITMAP,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [u32; 3],
    pub dshSection: super::HANDLE,
    pub dsOffset: u32,
}
#[cfg(feature = "winnt")]
impl Default for DIBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIB_PAL_COLORS: u32 = 1;
pub const DIB_RGB_COLORS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_2DREGION {
    pub cx: u32,
    pub cy: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_ADAPTER_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub adapterDevicePath: [u16; 128],
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_ADAPTER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DISPLAYCONFIG_ADVANCED_COLOR_MODE = i32;
pub const DISPLAYCONFIG_ADVANCED_COLOR_MODE_HDR: DISPLAYCONFIG_ADVANCED_COLOR_MODE = 2;
pub const DISPLAYCONFIG_ADVANCED_COLOR_MODE_SDR: DISPLAYCONFIG_ADVANCED_COLOR_MODE = 0;
pub const DISPLAYCONFIG_ADVANCED_COLOR_MODE_WCG: DISPLAYCONFIG_ADVANCED_COLOR_MODE = 1;
pub type DISPLAYCONFIG_COLOR_ENCODING = i32;
pub const DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32: DISPLAYCONFIG_COLOR_ENCODING = -1;
pub const DISPLAYCONFIG_COLOR_ENCODING_INTENSITY: DISPLAYCONFIG_COLOR_ENCODING = 4;
pub const DISPLAYCONFIG_COLOR_ENCODING_RGB: DISPLAYCONFIG_COLOR_ENCODING = 0;
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR420: DISPLAYCONFIG_COLOR_ENCODING = 3;
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR422: DISPLAYCONFIG_COLOR_ENCODING = 2;
pub const DISPLAYCONFIG_COLOR_ENCODING_YCBCR444: DISPLAYCONFIG_COLOR_ENCODING = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    pub PathSourceSize: super::POINTL,
    pub DesktopImageRegion: super::RECTL,
    pub DesktopImageClip: super::RECTL,
}
pub const DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32: DISPLAYCONFIG_DEVICE_INFO_TYPE = -1;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = 4;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO: DISPLAYCONFIG_DEVICE_INFO_TYPE = 9;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2: DISPLAYCONFIG_DEVICE_INFO_TYPE = 15;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = 12;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL: DISPLAYCONFIG_DEVICE_INFO_TYPE = 11;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = 1;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = 7;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE = 6;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = 2;
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE: DISPLAYCONFIG_DEVICE_INFO_TYPE = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    pub r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
    pub size: u32,
    pub adapterId: super::LUID,
    pub id: u32,
}
pub const DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE = 10;
pub const DISPLAYCONFIG_DEVICE_INFO_SET_HDR_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE = 16;
pub const DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = 13;
pub const DISPLAYCONFIG_DEVICE_INFO_SET_RESERVED1: DISPLAYCONFIG_DEVICE_INFO_TYPE = 14;
pub const DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = 8;
pub const DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE: DISPLAYCONFIG_DEVICE_INFO_TYPE = 5;
pub const DISPLAYCONFIG_DEVICE_INFO_SET_WCG_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE = 17;
pub type DISPLAYCONFIG_DEVICE_INFO_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0,
    pub colorEncoding: DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2 {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0,
    pub colorEncoding: DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: u32,
    pub activeColorMode: DISPLAYCONFIG_ADVANCED_COLOR_MODE,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_MAXPATH: u32 = 1024;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_MODE_INFO {
    pub infoType: DISPLAYCONFIG_MODE_INFO_TYPE,
    pub id: u32,
    pub adapterId: super::LUID,
    pub Anonymous: DISPLAYCONFIG_MODE_INFO_0,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_MODE_INFO_0 {
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
    pub sourceMode: DISPLAYCONFIG_SOURCE_MODE,
    pub desktopImageInfo: DISPLAYCONFIG_DESKTOP_IMAGE_INFO,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for DISPLAYCONFIG_MODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DISPLAYCONFIG_MODE_INFO_TYPE = i32;
pub const DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE: DISPLAYCONFIG_MODE_INFO_TYPE = 3;
pub const DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32: DISPLAYCONFIG_MODE_INFO_TYPE = -1;
pub const DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE: DISPLAYCONFIG_MODE_INFO_TYPE = 1;
pub const DISPLAYCONFIG_MODE_INFO_TYPE_TARGET: DISPLAYCONFIG_MODE_INFO_TYPE = 2;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 3;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 2;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 11;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 10;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_USB_TUNNEL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 18;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 4;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 8;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_FORCE_UINT32: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = -1;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 0;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 5;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_VIRTUAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 17;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 16;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = -2147483648;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 6;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 15;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = -1;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 9;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 14;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 1;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 13;
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = 12;
pub const DISPLAYCONFIG_PATH_ACTIVE: u32 = 1;
pub const DISPLAYCONFIG_PATH_BOOST_REFRESH_RATE: u32 = 16;
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: u32 = 65535;
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: u32 = 65535;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_INFO {
    pub sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    pub flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295;
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: u32 = 4;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    pub adapterId: super::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0,
    pub statusFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: u32 = 65535;
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: u32 = 8;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO {
    pub adapterId: super::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub rotation: DISPLAYCONFIG_ROTATION,
    pub scaling: DISPLAYCONFIG_SCALING,
    pub refreshRate: DISPLAYCONFIG_RATIONAL,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
    pub targetAvailable: windows_sys::core::BOOL,
    pub statusFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: u32 = 65535;
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: u32 = 29;
pub type DISPLAYCONFIG_PIXELFORMAT = i32;
pub const DISPLAYCONFIG_PIXELFORMAT_16BPP: DISPLAYCONFIG_PIXELFORMAT = 2;
pub const DISPLAYCONFIG_PIXELFORMAT_24BPP: DISPLAYCONFIG_PIXELFORMAT = 3;
pub const DISPLAYCONFIG_PIXELFORMAT_32BPP: DISPLAYCONFIG_PIXELFORMAT = 4;
pub const DISPLAYCONFIG_PIXELFORMAT_8BPP: DISPLAYCONFIG_PIXELFORMAT = 1;
pub const DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32: DISPLAYCONFIG_PIXELFORMAT = -1;
pub const DISPLAYCONFIG_PIXELFORMAT_NONGDI: DISPLAYCONFIG_PIXELFORMAT = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub type DISPLAYCONFIG_ROTATION = i32;
pub const DISPLAYCONFIG_ROTATION_FORCE_UINT32: DISPLAYCONFIG_ROTATION = -1;
pub const DISPLAYCONFIG_ROTATION_IDENTITY: DISPLAYCONFIG_ROTATION = 1;
pub const DISPLAYCONFIG_ROTATION_ROTATE180: DISPLAYCONFIG_ROTATION = 3;
pub const DISPLAYCONFIG_ROTATION_ROTATE270: DISPLAYCONFIG_ROTATION = 4;
pub const DISPLAYCONFIG_ROTATION_ROTATE90: DISPLAYCONFIG_ROTATION = 2;
pub type DISPLAYCONFIG_SCALING = i32;
pub const DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX: DISPLAYCONFIG_SCALING = 4;
pub const DISPLAYCONFIG_SCALING_CENTERED: DISPLAYCONFIG_SCALING = 2;
pub const DISPLAYCONFIG_SCALING_CUSTOM: DISPLAYCONFIG_SCALING = 5;
pub const DISPLAYCONFIG_SCALING_FORCE_UINT32: DISPLAYCONFIG_SCALING = -1;
pub const DISPLAYCONFIG_SCALING_IDENTITY: DISPLAYCONFIG_SCALING = 1;
pub const DISPLAYCONFIG_SCALING_PREFERRED: DISPLAYCONFIG_SCALING = 128;
pub const DISPLAYCONFIG_SCALING_STRETCHED: DISPLAYCONFIG_SCALING = 3;
pub type DISPLAYCONFIG_SCANLINE_ORDERING = i32;
pub const DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32: DISPLAYCONFIG_SCANLINE_ORDERING = -1;
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED: DISPLAYCONFIG_SCANLINE_ORDERING = 2;
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = 3;
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = 2;
pub const DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE: DISPLAYCONFIG_SCANLINE_ORDERING = 1;
pub const DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED: DISPLAYCONFIG_SCANLINE_ORDERING = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub SDRWhiteLevel: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_HDR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_HDR_STATE_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_HDR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_HDR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_HDR_STATE_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_HDR_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_HDR_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0,
    pub specializationType: windows_sys::core::GUID,
    pub specializationSubType: windows_sys::core::GUID,
    pub specializationApplicationName: [u16; 128],
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_WCG_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_WCG_STATE_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_WCG_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_WCG_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_WCG_STATE_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SET_WCG_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_WCG_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub viewGdiDeviceName: [u16; 32],
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAYCONFIG_SOURCE_IN_USE: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SOURCE_MODE {
    pub width: u32,
    pub height: u32,
    pub pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    pub position: super::POINTL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0,
    pub value: u32,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub baseOutputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub edidManufactureId: u16,
    pub edidProductCodeId: u16,
    pub connectorInstance: u32,
    pub monitorFriendlyDeviceName: [u16; 64],
    pub monitorDevicePath: [u16; 128],
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0,
}
impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0,
    pub value: u32,
}
impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: u32 = 4;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: u32 = 8;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: u32 = 16;
pub const DISPLAYCONFIG_TARGET_FORCIBLE: u32 = 2;
pub const DISPLAYCONFIG_TARGET_IN_USE: u32 = 1;
pub const DISPLAYCONFIG_TARGET_IS_HMD: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_MODE {
    pub targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
}
impl Default for DISPLAYCONFIG_TARGET_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub width: u32,
    pub height: u32,
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
}
#[cfg(feature = "winnt")]
impl Default for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAYCONFIG_TOPOLOGY_CLONE: DISPLAYCONFIG_TOPOLOGY_ID = 2;
pub const DISPLAYCONFIG_TOPOLOGY_EXTEND: DISPLAYCONFIG_TOPOLOGY_ID = 4;
pub const DISPLAYCONFIG_TOPOLOGY_EXTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = 8;
pub const DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32: DISPLAYCONFIG_TOPOLOGY_ID = -1;
pub type DISPLAYCONFIG_TOPOLOGY_ID = i32;
pub const DISPLAYCONFIG_TOPOLOGY_INTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = 1;
pub type DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    pub pixelRate: u64,
    pub hSyncFreq: DISPLAYCONFIG_RATIONAL,
    pub vSyncFreq: DISPLAYCONFIG_RATIONAL,
    pub activeSize: DISPLAYCONFIG_2DREGION,
    pub totalSize: DISPLAYCONFIG_2DREGION,
    pub Anonymous: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
}
impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    pub AdditionalSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0,
    pub videoStandard: u32,
}
impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: u32,
}
pub type DISPLAY_DEVICE = DISPLAY_DEVICEA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAY_DEVICEA {
    pub cb: u32,
    pub DeviceName: [i8; 32],
    pub DeviceString: [i8; 128],
    pub StateFlags: u32,
    pub DeviceID: [i8; 128],
    pub DeviceKey: [i8; 128],
}
impl Default for DISPLAY_DEVICEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAY_DEVICEW {
    pub cb: u32,
    pub DeviceName: [u16; 32],
    pub DeviceString: [u16; 128],
    pub StateFlags: u32,
    pub DeviceID: [u16; 128],
    pub DeviceKey: [u16; 128],
}
impl Default for DISPLAY_DEVICEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAY_DEVICE_ACC_DRIVER: u32 = 64;
pub const DISPLAY_DEVICE_ACTIVE: u32 = 1;
pub const DISPLAY_DEVICE_ATTACHED: u32 = 2;
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: u32 = 1;
pub const DISPLAY_DEVICE_DISCONNECT: u32 = 33554432;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: u32 = 8;
pub const DISPLAY_DEVICE_MODESPRUNED: u32 = 134217728;
pub const DISPLAY_DEVICE_MULTI_DRIVER: u32 = 2;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: u32 = 4;
pub const DISPLAY_DEVICE_RDPUDD: u32 = 16777216;
pub const DISPLAY_DEVICE_REMOTE: u32 = 67108864;
pub const DISPLAY_DEVICE_REMOVABLE: u32 = 32;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: u32 = 2097152;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: u32 = 524288;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: u32 = 16;
pub const DI_APPBANDING: u32 = 1;
pub const DI_ROPS_READ_DESTINATION: u32 = 2;
pub const DKGRAY_BRUSH: u32 = 3;
pub const DMBIN_AUTO: u32 = 7;
pub const DMBIN_CASSETTE: u32 = 14;
pub const DMBIN_ENVELOPE: u32 = 5;
pub const DMBIN_ENVMANUAL: u32 = 6;
pub const DMBIN_FIRST: u32 = 1;
pub const DMBIN_FORMSOURCE: u32 = 15;
pub const DMBIN_LARGECAPACITY: u32 = 11;
pub const DMBIN_LARGEFMT: u32 = 10;
pub const DMBIN_LAST: u32 = 15;
pub const DMBIN_LOWER: u32 = 2;
pub const DMBIN_MANUAL: u32 = 4;
pub const DMBIN_MIDDLE: u32 = 3;
pub const DMBIN_ONLYONE: u32 = 1;
pub const DMBIN_SMALLFMT: u32 = 9;
pub const DMBIN_TRACTOR: u32 = 8;
pub const DMBIN_UPPER: u32 = 1;
pub const DMBIN_USER: u32 = 256;
pub const DMCOLLATE_FALSE: u32 = 0;
pub const DMCOLLATE_TRUE: u32 = 1;
pub const DMCOLOR_COLOR: u32 = 2;
pub const DMCOLOR_MONOCHROME: u32 = 1;
pub const DMDFO_CENTER: u32 = 2;
pub const DMDFO_DEFAULT: u32 = 0;
pub const DMDFO_STRETCH: u32 = 1;
pub const DMDISPLAYFLAGS_TEXTMODE: u32 = 4;
pub const DMDITHER_COARSE: u32 = 2;
pub const DMDITHER_ERRORDIFFUSION: u32 = 5;
pub const DMDITHER_FINE: u32 = 3;
pub const DMDITHER_GRAYSCALE: u32 = 10;
pub const DMDITHER_LINEART: u32 = 4;
pub const DMDITHER_NONE: u32 = 1;
pub const DMDITHER_RESERVED6: u32 = 6;
pub const DMDITHER_RESERVED7: u32 = 7;
pub const DMDITHER_RESERVED8: u32 = 8;
pub const DMDITHER_RESERVED9: u32 = 9;
pub const DMDITHER_USER: u32 = 256;
pub const DMDO_180: u32 = 2;
pub const DMDO_270: u32 = 3;
pub const DMDO_90: u32 = 1;
pub const DMDO_DEFAULT: u32 = 0;
pub const DMDUP_HORIZONTAL: u32 = 3;
pub const DMDUP_SIMPLEX: u32 = 1;
pub const DMDUP_VERTICAL: u32 = 2;
pub const DMICMMETHOD_DEVICE: u32 = 4;
pub const DMICMMETHOD_DRIVER: u32 = 3;
pub const DMICMMETHOD_NONE: u32 = 1;
pub const DMICMMETHOD_SYSTEM: u32 = 2;
pub const DMICMMETHOD_USER: u32 = 256;
pub const DMICM_ABS_COLORIMETRIC: u32 = 4;
pub const DMICM_COLORIMETRIC: u32 = 3;
pub const DMICM_CONTRAST: u32 = 2;
pub const DMICM_SATURATE: u32 = 1;
pub const DMICM_USER: u32 = 256;
pub const DMMEDIA_GLOSSY: u32 = 3;
pub const DMMEDIA_STANDARD: u32 = 1;
pub const DMMEDIA_TRANSPARENCY: u32 = 2;
pub const DMMEDIA_USER: u32 = 256;
pub const DMNUP_ONEUP: u32 = 2;
pub const DMNUP_SYSTEM: u32 = 1;
pub const DMORIENT_LANDSCAPE: u32 = 2;
pub const DMORIENT_PORTRAIT: u32 = 1;
pub const DMPAPER_10X11: u32 = 45;
pub const DMPAPER_10X14: u32 = 16;
pub const DMPAPER_11X17: u32 = 17;
pub const DMPAPER_12X11: u32 = 90;
pub const DMPAPER_15X11: u32 = 46;
pub const DMPAPER_9X11: u32 = 44;
pub const DMPAPER_A2: u32 = 66;
pub const DMPAPER_A3: u32 = 8;
pub const DMPAPER_A3_EXTRA: u32 = 63;
pub const DMPAPER_A3_EXTRA_TRANSVERSE: u32 = 68;
pub const DMPAPER_A3_ROTATED: u32 = 76;
pub const DMPAPER_A3_TRANSVERSE: u32 = 67;
pub const DMPAPER_A4: u32 = 9;
pub const DMPAPER_A4SMALL: u32 = 10;
pub const DMPAPER_A4_EXTRA: u32 = 53;
pub const DMPAPER_A4_PLUS: u32 = 60;
pub const DMPAPER_A4_ROTATED: u32 = 77;
pub const DMPAPER_A4_TRANSVERSE: u32 = 55;
pub const DMPAPER_A5: u32 = 11;
pub const DMPAPER_A5_EXTRA: u32 = 64;
pub const DMPAPER_A5_ROTATED: u32 = 78;
pub const DMPAPER_A5_TRANSVERSE: u32 = 61;
pub const DMPAPER_A6: u32 = 70;
pub const DMPAPER_A6_ROTATED: u32 = 83;
pub const DMPAPER_A_PLUS: u32 = 57;
pub const DMPAPER_B4: u32 = 12;
pub const DMPAPER_B4_JIS_ROTATED: u32 = 79;
pub const DMPAPER_B5: u32 = 13;
pub const DMPAPER_B5_EXTRA: u32 = 65;
pub const DMPAPER_B5_JIS_ROTATED: u32 = 80;
pub const DMPAPER_B5_TRANSVERSE: u32 = 62;
pub const DMPAPER_B6_JIS: u32 = 88;
pub const DMPAPER_B6_JIS_ROTATED: u32 = 89;
pub const DMPAPER_B_PLUS: u32 = 58;
pub const DMPAPER_CSHEET: u32 = 24;
pub const DMPAPER_DBL_JAPANESE_POSTCARD: u32 = 69;
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: u32 = 82;
pub const DMPAPER_DSHEET: u32 = 25;
pub const DMPAPER_ENV_10: u32 = 20;
pub const DMPAPER_ENV_11: u32 = 21;
pub const DMPAPER_ENV_12: u32 = 22;
pub const DMPAPER_ENV_14: u32 = 23;
pub const DMPAPER_ENV_9: u32 = 19;
pub const DMPAPER_ENV_B4: u32 = 33;
pub const DMPAPER_ENV_B5: u32 = 34;
pub const DMPAPER_ENV_B6: u32 = 35;
pub const DMPAPER_ENV_C3: u32 = 29;
pub const DMPAPER_ENV_C4: u32 = 30;
pub const DMPAPER_ENV_C5: u32 = 28;
pub const DMPAPER_ENV_C6: u32 = 31;
pub const DMPAPER_ENV_C65: u32 = 32;
pub const DMPAPER_ENV_DL: u32 = 27;
pub const DMPAPER_ENV_INVITE: u32 = 47;
pub const DMPAPER_ENV_ITALY: u32 = 36;
pub const DMPAPER_ENV_MONARCH: u32 = 37;
pub const DMPAPER_ENV_PERSONAL: u32 = 38;
pub const DMPAPER_ESHEET: u32 = 26;
pub const DMPAPER_EXECUTIVE: u32 = 7;
pub const DMPAPER_FANFOLD_LGL_GERMAN: u32 = 41;
pub const DMPAPER_FANFOLD_STD_GERMAN: u32 = 40;
pub const DMPAPER_FANFOLD_US: u32 = 39;
pub const DMPAPER_FIRST: u32 = 1;
pub const DMPAPER_FOLIO: u32 = 14;
pub const DMPAPER_ISO_B4: u32 = 42;
pub const DMPAPER_JAPANESE_POSTCARD: u32 = 43;
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: u32 = 81;
pub const DMPAPER_JENV_CHOU3: u32 = 73;
pub const DMPAPER_JENV_CHOU3_ROTATED: u32 = 86;
pub const DMPAPER_JENV_CHOU4: u32 = 74;
pub const DMPAPER_JENV_CHOU4_ROTATED: u32 = 87;
pub const DMPAPER_JENV_KAKU2: u32 = 71;
pub const DMPAPER_JENV_KAKU2_ROTATED: u32 = 84;
pub const DMPAPER_JENV_KAKU3: u32 = 72;
pub const DMPAPER_JENV_KAKU3_ROTATED: u32 = 85;
pub const DMPAPER_JENV_YOU4: u32 = 91;
pub const DMPAPER_JENV_YOU4_ROTATED: u32 = 92;
pub const DMPAPER_LAST: u32 = 118;
pub const DMPAPER_LEDGER: u32 = 4;
pub const DMPAPER_LEGAL: u32 = 5;
pub const DMPAPER_LEGAL_EXTRA: u32 = 51;
pub const DMPAPER_LETTER: u32 = 1;
pub const DMPAPER_LETTERSMALL: u32 = 2;
pub const DMPAPER_LETTER_EXTRA: u32 = 50;
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: u32 = 56;
pub const DMPAPER_LETTER_PLUS: u32 = 59;
pub const DMPAPER_LETTER_ROTATED: u32 = 75;
pub const DMPAPER_LETTER_TRANSVERSE: u32 = 54;
pub const DMPAPER_NOTE: u32 = 18;
pub const DMPAPER_P16K: u32 = 93;
pub const DMPAPER_P16K_ROTATED: u32 = 106;
pub const DMPAPER_P32K: u32 = 94;
pub const DMPAPER_P32KBIG: u32 = 95;
pub const DMPAPER_P32KBIG_ROTATED: u32 = 108;
pub const DMPAPER_P32K_ROTATED: u32 = 107;
pub const DMPAPER_PENV_1: u32 = 96;
pub const DMPAPER_PENV_10: u32 = 105;
pub const DMPAPER_PENV_10_ROTATED: u32 = 118;
pub const DMPAPER_PENV_1_ROTATED: u32 = 109;
pub const DMPAPER_PENV_2: u32 = 97;
pub const DMPAPER_PENV_2_ROTATED: u32 = 110;
pub const DMPAPER_PENV_3: u32 = 98;
pub const DMPAPER_PENV_3_ROTATED: u32 = 111;
pub const DMPAPER_PENV_4: u32 = 99;
pub const DMPAPER_PENV_4_ROTATED: u32 = 112;
pub const DMPAPER_PENV_5: u32 = 100;
pub const DMPAPER_PENV_5_ROTATED: u32 = 113;
pub const DMPAPER_PENV_6: u32 = 101;
pub const DMPAPER_PENV_6_ROTATED: u32 = 114;
pub const DMPAPER_PENV_7: u32 = 102;
pub const DMPAPER_PENV_7_ROTATED: u32 = 115;
pub const DMPAPER_PENV_8: u32 = 103;
pub const DMPAPER_PENV_8_ROTATED: u32 = 116;
pub const DMPAPER_PENV_9: u32 = 104;
pub const DMPAPER_PENV_9_ROTATED: u32 = 117;
pub const DMPAPER_QUARTO: u32 = 15;
pub const DMPAPER_RESERVED_48: u32 = 48;
pub const DMPAPER_RESERVED_49: u32 = 49;
pub const DMPAPER_STATEMENT: u32 = 6;
pub const DMPAPER_TABLOID: u32 = 3;
pub const DMPAPER_TABLOID_EXTRA: u32 = 52;
pub const DMPAPER_USER: u32 = 256;
pub const DMRES_DRAFT: i32 = -1;
pub const DMRES_HIGH: i32 = -4;
pub const DMRES_LOW: i32 = -2;
pub const DMRES_MEDIUM: i32 = -3;
pub const DMTT_BITMAP: u32 = 1;
pub const DMTT_DOWNLOAD: u32 = 2;
pub const DMTT_DOWNLOAD_OUTLINE: u32 = 4;
pub const DMTT_SUBDEV: u32 = 3;
pub const DM_BITSPERPEL: u32 = 262144;
pub const DM_COLLATE: u32 = 32768;
pub const DM_COLOR: u32 = 2048;
pub const DM_COPIES: u32 = 256;
pub const DM_DEFAULTSOURCE: u32 = 512;
pub const DM_DISPLAYFIXEDOUTPUT: u32 = 536870912;
pub const DM_DISPLAYFLAGS: u32 = 2097152;
pub const DM_DISPLAYFREQUENCY: u32 = 4194304;
pub const DM_DISPLAYORIENTATION: u32 = 128;
pub const DM_DITHERTYPE: u32 = 67108864;
pub const DM_DUPLEX: u32 = 4096;
pub const DM_FORMNAME: u32 = 65536;
pub const DM_ICMINTENT: u32 = 16777216;
pub const DM_ICMMETHOD: u32 = 8388608;
pub const DM_INTERLACED: u32 = 2;
pub const DM_LOGPIXELS: u32 = 131072;
pub const DM_MEDIATYPE: u32 = 33554432;
pub const DM_NUP: u32 = 64;
pub const DM_ORIENTATION: u32 = 1;
pub const DM_PANNINGHEIGHT: u32 = 268435456;
pub const DM_PANNINGWIDTH: u32 = 134217728;
pub const DM_PAPERLENGTH: u32 = 4;
pub const DM_PAPERSIZE: u32 = 2;
pub const DM_PAPERWIDTH: u32 = 8;
pub const DM_PELSHEIGHT: u32 = 1048576;
pub const DM_PELSWIDTH: u32 = 524288;
pub const DM_POSITION: u32 = 32;
pub const DM_PRINTQUALITY: u32 = 1024;
pub const DM_SCALE: u32 = 16;
pub const DM_SPECVERSION: u32 = 1025;
pub const DM_TTOPTION: u32 = 16384;
pub const DM_YRESOLUTION: u32 = 8192;
pub type DOCINFO = DOCINFOA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: windows_sys::core::PCSTR,
    pub lpszOutput: windows_sys::core::PCSTR,
    pub lpszDatatype: windows_sys::core::PCSTR,
    pub fwType: u32,
}
impl Default for DOCINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: windows_sys::core::PCWSTR,
    pub lpszOutput: windows_sys::core::PCWSTR,
    pub lpszDatatype: windows_sys::core::PCWSTR,
    pub fwType: u32,
}
impl Default for DOCINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOWNLOADFACE: u32 = 514;
pub const DOWNLOADHEADER: u32 = 4111;
pub const DRAFTMODE: u32 = 7;
pub const DRAFT_QUALITY: u32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DRAWPATRECT {
    pub ptPosition: super::POINT,
    pub ptSize: super::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
pub const DRAWPATTERNRECT: u32 = 25;
pub const DRIVERVERSION: u32 = 0;
pub const DSTINVERT: u32 = 5570569;
pub const DT_CHARSTREAM: u32 = 4;
pub const DT_DISPFILE: u32 = 6;
pub const DT_METAFILE: u32 = 5;
pub const DT_PLOTTER: u32 = 0;
pub const DT_RASCAMERA: u32 = 3;
pub const DT_RASDISPLAY: u32 = 1;
pub const DT_RASPRINTER: u32 = 2;
pub const EASTEUROPE_CHARSET: u32 = 238;
pub const ELF_CULTURE_LATIN: u32 = 0;
pub const ELF_VENDOR_SIZE: u32 = 4;
pub const ELF_VERSION: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMR {
    pub iType: u32,
    pub nSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRABORTPATH {
    pub emr: EMR,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRALPHABLEND {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRANGLEARC {
    pub emr: EMR,
    pub ptlCenter: super::POINTL,
    pub nRadius: u32,
    pub eStartAngle: f32,
    pub eSweepAngle: f32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRARC {
    pub emr: EMR,
    pub rclBox: super::RECTL,
    pub ptlStart: super::POINTL,
    pub ptlEnd: super::POINTL,
}
#[cfg(feature = "windef")]
pub type EMRARCTO = EMRARC;
pub type EMRBEGINPATH = EMRABORTPATH;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRBITBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
}
#[cfg(feature = "windef")]
pub type EMRCHORD = EMRARC;
pub type EMRCLOSEFIGURE = EMRABORTPATH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRCOLORCORRECTPALETTE {
    pub emr: EMR,
    pub ihPalette: u32,
    pub nFirstEntry: u32,
    pub nPalEntries: u32,
    pub nReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRCOLORMATCHTOTARGET {
    pub emr: EMR,
    pub dwAction: u32,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRCOLORMATCHTOTARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEBRUSHINDIRECT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub lb: LOGBRUSH32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATECOLORSPACE {
    pub emr: EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRCREATECOLORSPACEW {
    pub emr: EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEW,
    pub dwFlags: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRCREATECOLORSPACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEDIBPATTERNBRUSHPT {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEMONOBRUSH {
    pub emr: EMR,
    pub ihBrush: u32,
    pub iUsage: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub lgpl: LOGPALETTE,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub lopn: LOGPEN,
}
pub type EMRDELETECOLORSPACE = EMRSETCOLORSPACE;
pub type EMRDELETEOBJECT = EMRSELECTOBJECT;
pub type EMRDRAWESCAPE = EMREXTESCAPE;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRELLIPSE {
    pub emr: EMR,
    pub rclBox: super::RECTL,
}
pub type EMRENDPATH = EMRABORTPATH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMREOF {
    pub emr: EMR,
    pub nPalEntries: u32,
    pub offPalEntries: u32,
    pub nSizeLast: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMREXCLUDECLIPRECT {
    pub emr: EMR,
    pub rclClip: super::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMREXTCREATEFONTINDIRECTW {
    pub emr: EMR,
    pub ihFont: u32,
    pub elfw: EXTLOGFONTW,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMREXTCREATEPEN {
    pub emr: EMR,
    pub ihPen: u32,
    pub offBmi: u32,
    pub cbBmi: u32,
    pub offBits: u32,
    pub cbBits: u32,
    pub elp: EXTLOGPEN32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMREXTESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl Default for EMREXTESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMREXTFLOODFILL {
    pub emr: EMR,
    pub ptlStart: super::POINTL,
    pub crColor: super::COLORREF,
    pub iMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMREXTSELECTCLIPRGN {
    pub emr: EMR,
    pub cbRgnData: u32,
    pub iMode: u32,
    pub RgnData: [u8; 1],
}
impl Default for EMREXTSELECTCLIPRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMREXTTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub emrtext: EMRTEXT,
}
#[cfg(feature = "windef")]
pub type EMREXTTEXTOUTW = EMREXTTEXTOUTA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRFILLPATH {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRFILLRGN {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRFILLRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EMRFLATTENPATH = EMRABORTPATH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRFORMAT {
    pub dSignature: u32,
    pub nVersion: u32,
    pub cbData: u32,
    pub offData: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRFRAMERGN {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbRgnData: u32,
    pub ihBrush: u32,
    pub szlStroke: super::SIZEL,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRFRAMERGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRGDICOMMENT {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRGDICOMMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRGLSBOUNDEDRECORD {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbData: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRGLSBOUNDEDRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRGLSRECORD {
    pub emr: EMR,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRGLSRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRGRADIENTFILL {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub nVer: u32,
    pub nTri: u32,
    pub ulMode: u32,
    pub Ver: [TRIVERTEX; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRGRADIENTFILL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type EMRINTERSECTCLIPRECT = EMREXCLUDECLIPRECT;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRINVERTRGN {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbRgnData: u32,
    pub RgnData: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRINVERTRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRLINETO {
    pub emr: EMR,
    pub ptl: super::POINTL,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRMASKBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRMODIFYWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
    pub iMode: u32,
}
#[cfg(feature = "windef")]
pub type EMRMOVETOEX = EMRLINETO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRNAMEDESCAPE {
    pub emr: EMR,
    pub iEscape: i32,
    pub cbDriver: i32,
    pub cbEscData: i32,
    pub EscData: [u8; 1],
}
impl Default for EMRNAMEDESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMROFFSETCLIPRGN {
    pub emr: EMR,
    pub ptlOffset: super::POINTL,
}
#[cfg(feature = "windef")]
pub type EMRPAINTRGN = EMRINVERTRGN;
#[cfg(feature = "windef")]
pub type EMRPIE = EMRARC;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRPIXELFORMAT {
    pub emr: EMR,
    pub pfd: PIXELFORMATDESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPLGBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub aptlDest: [super::POINTL; 3],
    pub xSrc: i32,
    pub ySrc: i32,
    pub cxSrc: i32,
    pub cySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
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
#[cfg(feature = "windef")]
impl Default for EMRPLGBLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type EMRPOLYBEZIER = EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type EMRPOLYBEZIER16 = EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type EMRPOLYBEZIERTO = EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type EMRPOLYBEZIERTO16 = EMRPOLYLINE16;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYDRAW {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cptl: u32,
    pub aptl: [super::POINTL; 1],
    pub abTypes: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYDRAW16 {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cpts: u32,
    pub apts: [super::POINTS; 1],
    pub abTypes: [u8; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYDRAW16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type EMRPOLYGON = EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type EMRPOLYGON16 = EMRPOLYLINE16;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cptl: u32,
    pub aptl: [super::POINTL; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cpts: u32,
    pub apts: [super::POINTS; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYLINE16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type EMRPOLYLINETO = EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type EMRPOLYLINETO16 = EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type EMRPOLYPOLYGON = EMRPOLYPOLYLINE;
#[cfg(feature = "windef")]
pub type EMRPOLYPOLYGON16 = EMRPOLYPOLYLINE16;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub nPolys: u32,
    pub cptl: u32,
    pub aPolyCounts: [u32; 1],
    pub aptl: [super::POINTL; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYPOLYLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub nPolys: u32,
    pub cpts: u32,
    pub aPolyCounts: [u32; 1],
    pub apts: [super::POINTS; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYPOLYLINE16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EMRPOLYTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub iGraphicsMode: u32,
    pub exScale: f32,
    pub eyScale: f32,
    pub cStrings: i32,
    pub aemrtext: [EMRTEXT; 1],
}
#[cfg(feature = "windef")]
impl Default for EMRPOLYTEXTOUTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
pub type EMRPOLYTEXTOUTW = EMRPOLYTEXTOUTA;
pub type EMRREALIZEPALETTE = EMRABORTPATH;
#[cfg(feature = "windef")]
pub type EMRRECTANGLE = EMRELLIPSE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRRESIZEPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
    pub cEntries: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRRESTOREDC {
    pub emr: EMR,
    pub iRelative: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRROUNDRECT {
    pub emr: EMR,
    pub rclBox: super::RECTL,
    pub szlCorner: super::SIZEL,
}
pub type EMRSAVEDC = EMRABORTPATH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSCALEVIEWPORTEXTEX {
    pub emr: EMR,
    pub xNum: i32,
    pub xDenom: i32,
    pub yNum: i32,
    pub yDenom: i32,
}
pub type EMRSCALEWINDOWEXTEX = EMRSCALEVIEWPORTEXTEX;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSELECTCLIPPATH {
    pub emr: EMR,
    pub iMode: u32,
}
pub type EMRSELECTCOLORSPACE = EMRSETCOLORSPACE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSELECTOBJECT {
    pub emr: EMR,
    pub ihObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSELECTPALETTE {
    pub emr: EMR,
    pub ihPal: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSETARCDIRECTION {
    pub emr: EMR,
    pub iArcDirection: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETBKCOLOR {
    pub emr: EMR,
    pub crColor: super::COLORREF,
}
pub type EMRSETBKMODE = EMRSELECTCLIPPATH;
#[cfg(feature = "windef")]
pub type EMRSETBRUSHORGEX = EMRSETVIEWPORTORGEX;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSETCOLORADJUSTMENT {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSETCOLORSPACE {
    pub emr: EMR,
    pub ihCS: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETDIBITSTODEVICE {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
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
pub type EMRSETICMMODE = EMRSELECTCLIPPATH;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRSETICMPROFILE {
    pub emr: EMR,
    pub dwFlags: u32,
    pub cbName: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
impl Default for EMRSETICMPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EMRSETICMPROFILEA = EMRSETICMPROFILE;
pub type EMRSETICMPROFILEW = EMRSETICMPROFILE;
pub type EMRSETLAYOUT = EMRSELECTCLIPPATH;
pub type EMRSETMAPMODE = EMRSELECTCLIPPATH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSETMAPPERFLAGS {
    pub emr: EMR,
    pub dwFlags: u32,
}
pub type EMRSETMETARGN = EMRABORTPATH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSETMITERLIMIT {
    pub emr: EMR,
    pub eMiterLimit: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EMRSETPALETTEENTRIES {
    pub emr: EMR,
    pub ihPal: u32,
    pub iStart: u32,
    pub cEntries: u32,
    pub aPalEntries: [PALETTEENTRY; 1],
}
impl Default for EMRSETPALETTEENTRIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETPIXELV {
    pub emr: EMR,
    pub ptlPixel: super::POINTL,
    pub crColor: super::COLORREF,
}
pub type EMRSETPOLYFILLMODE = EMRSELECTCLIPPATH;
pub type EMRSETROP2 = EMRSELECTCLIPPATH;
pub type EMRSETSTRETCHBLTMODE = EMRSELECTCLIPPATH;
pub type EMRSETTEXTALIGN = EMRSELECTCLIPPATH;
#[cfg(feature = "windef")]
pub type EMRSETTEXTCOLOR = EMRSETBKCOLOR;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETVIEWPORTEXTEX {
    pub emr: EMR,
    pub szlExtent: super::SIZEL,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETVIEWPORTORGEX {
    pub emr: EMR,
    pub ptlOrigin: super::POINTL,
}
#[cfg(feature = "windef")]
pub type EMRSETWINDOWEXTEX = EMRSETVIEWPORTEXTEX;
#[cfg(feature = "windef")]
pub type EMRSETWINDOWORGEX = EMRSETVIEWPORTORGEX;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EMRSETWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSTRETCHBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRSTRETCHDIBITS {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
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
#[cfg(feature = "windef")]
pub type EMRSTROKEANDFILLPATH = EMRFILLPATH;
#[cfg(feature = "windef")]
pub type EMRSTROKEPATH = EMRFILLPATH;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRTEXT {
    pub ptlReference: super::POINTL,
    pub nChars: u32,
    pub offString: u32,
    pub fOptions: u32,
    pub rcl: super::RECTL,
    pub offDx: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EMRTRANSPARENTBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: i32,
    pub yDest: i32,
    pub cxDest: i32,
    pub cyDest: i32,
    pub dwRop: u32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: u32,
    pub offBmiSrc: u32,
    pub cbBmiSrc: u32,
    pub offBitsSrc: u32,
    pub cbBitsSrc: u32,
    pub cxSrc: i32,
    pub cySrc: i32,
}
pub type EMRWIDENPATH = EMRABORTPATH;
pub const EMR_ABORTPATH: u32 = 68;
pub const EMR_ALPHABLEND: u32 = 114;
pub const EMR_ANGLEARC: u32 = 41;
pub const EMR_ARC: u32 = 45;
pub const EMR_ARCTO: u32 = 55;
pub const EMR_BEGINPATH: u32 = 59;
pub const EMR_BITBLT: u32 = 76;
pub const EMR_CHORD: u32 = 46;
pub const EMR_CLOSEFIGURE: u32 = 61;
pub const EMR_COLORCORRECTPALETTE: u32 = 111;
pub const EMR_COLORMATCHTOTARGETW: u32 = 121;
pub const EMR_CREATEBRUSHINDIRECT: u32 = 39;
pub const EMR_CREATECOLORSPACE: u32 = 99;
pub const EMR_CREATECOLORSPACEW: u32 = 122;
pub const EMR_CREATEDIBPATTERNBRUSHPT: u32 = 94;
pub const EMR_CREATEMONOBRUSH: u32 = 93;
pub const EMR_CREATEPALETTE: u32 = 49;
pub const EMR_CREATEPEN: u32 = 38;
pub const EMR_DELETECOLORSPACE: u32 = 101;
pub const EMR_DELETEOBJECT: u32 = 40;
pub const EMR_ELLIPSE: u32 = 42;
pub const EMR_ENDPATH: u32 = 60;
pub const EMR_EOF: u32 = 14;
pub const EMR_EXCLUDECLIPRECT: u32 = 29;
pub const EMR_EXTCREATEFONTINDIRECTW: u32 = 82;
pub const EMR_EXTCREATEPEN: u32 = 95;
pub const EMR_EXTFLOODFILL: u32 = 53;
pub const EMR_EXTSELECTCLIPRGN: u32 = 75;
pub const EMR_EXTTEXTOUTA: u32 = 83;
pub const EMR_EXTTEXTOUTW: u32 = 84;
pub const EMR_FILLPATH: u32 = 62;
pub const EMR_FILLRGN: u32 = 71;
pub const EMR_FLATTENPATH: u32 = 65;
pub const EMR_FRAMERGN: u32 = 72;
pub const EMR_GDICOMMENT: u32 = 70;
pub const EMR_GLSBOUNDEDRECORD: u32 = 103;
pub const EMR_GLSRECORD: u32 = 102;
pub const EMR_GRADIENTFILL: u32 = 118;
pub const EMR_HEADER: u32 = 1;
pub const EMR_INTERSECTCLIPRECT: u32 = 30;
pub const EMR_INVERTRGN: u32 = 73;
pub const EMR_LINETO: u32 = 54;
pub const EMR_MASKBLT: u32 = 78;
pub const EMR_MAX: u32 = 122;
pub const EMR_MIN: u32 = 1;
pub const EMR_MODIFYWORLDTRANSFORM: u32 = 36;
pub const EMR_MOVETOEX: u32 = 27;
pub const EMR_OFFSETCLIPRGN: u32 = 26;
pub const EMR_PAINTRGN: u32 = 74;
pub const EMR_PIE: u32 = 47;
pub const EMR_PIXELFORMAT: u32 = 104;
pub const EMR_PLGBLT: u32 = 79;
pub const EMR_POLYBEZIER: u32 = 2;
pub const EMR_POLYBEZIER16: u32 = 85;
pub const EMR_POLYBEZIERTO: u32 = 5;
pub const EMR_POLYBEZIERTO16: u32 = 88;
pub const EMR_POLYDRAW: u32 = 56;
pub const EMR_POLYDRAW16: u32 = 92;
pub const EMR_POLYGON: u32 = 3;
pub const EMR_POLYGON16: u32 = 86;
pub const EMR_POLYLINE: u32 = 4;
pub const EMR_POLYLINE16: u32 = 87;
pub const EMR_POLYLINETO: u32 = 6;
pub const EMR_POLYLINETO16: u32 = 89;
pub const EMR_POLYPOLYGON: u32 = 8;
pub const EMR_POLYPOLYGON16: u32 = 91;
pub const EMR_POLYPOLYLINE: u32 = 7;
pub const EMR_POLYPOLYLINE16: u32 = 90;
pub const EMR_POLYTEXTOUTA: u32 = 96;
pub const EMR_POLYTEXTOUTW: u32 = 97;
pub const EMR_REALIZEPALETTE: u32 = 52;
pub const EMR_RECTANGLE: u32 = 43;
pub const EMR_RESERVED_105: u32 = 105;
pub const EMR_RESERVED_106: u32 = 106;
pub const EMR_RESERVED_107: u32 = 107;
pub const EMR_RESERVED_108: u32 = 108;
pub const EMR_RESERVED_109: u32 = 109;
pub const EMR_RESERVED_110: u32 = 110;
pub const EMR_RESERVED_117: u32 = 117;
pub const EMR_RESERVED_119: u32 = 119;
pub const EMR_RESERVED_120: u32 = 120;
pub const EMR_RESIZEPALETTE: u32 = 51;
pub const EMR_RESTOREDC: u32 = 34;
pub const EMR_ROUNDRECT: u32 = 44;
pub const EMR_SAVEDC: u32 = 33;
pub const EMR_SCALEVIEWPORTEXTEX: u32 = 31;
pub const EMR_SCALEWINDOWEXTEX: u32 = 32;
pub const EMR_SELECTCLIPPATH: u32 = 67;
pub const EMR_SELECTOBJECT: u32 = 37;
pub const EMR_SELECTPALETTE: u32 = 48;
pub const EMR_SETARCDIRECTION: u32 = 57;
pub const EMR_SETBKCOLOR: u32 = 25;
pub const EMR_SETBKMODE: u32 = 18;
pub const EMR_SETBRUSHORGEX: u32 = 13;
pub const EMR_SETCOLORADJUSTMENT: u32 = 23;
pub const EMR_SETCOLORSPACE: u32 = 100;
pub const EMR_SETDIBITSTODEVICE: u32 = 80;
pub const EMR_SETICMMODE: u32 = 98;
pub const EMR_SETICMPROFILEA: u32 = 112;
pub const EMR_SETICMPROFILEW: u32 = 113;
pub const EMR_SETLAYOUT: u32 = 115;
pub const EMR_SETMAPMODE: u32 = 17;
pub const EMR_SETMAPPERFLAGS: u32 = 16;
pub const EMR_SETMETARGN: u32 = 28;
pub const EMR_SETMITERLIMIT: u32 = 58;
pub const EMR_SETPALETTEENTRIES: u32 = 50;
pub const EMR_SETPIXELV: u32 = 15;
pub const EMR_SETPOLYFILLMODE: u32 = 19;
pub const EMR_SETROP2: u32 = 20;
pub const EMR_SETSTRETCHBLTMODE: u32 = 21;
pub const EMR_SETTEXTALIGN: u32 = 22;
pub const EMR_SETTEXTCOLOR: u32 = 24;
pub const EMR_SETVIEWPORTEXTEX: u32 = 11;
pub const EMR_SETVIEWPORTORGEX: u32 = 12;
pub const EMR_SETWINDOWEXTEX: u32 = 9;
pub const EMR_SETWINDOWORGEX: u32 = 10;
pub const EMR_SETWORLDTRANSFORM: u32 = 35;
pub const EMR_STRETCHBLT: u32 = 77;
pub const EMR_STRETCHDIBITS: u32 = 81;
pub const EMR_STROKEANDFILLPATH: u32 = 63;
pub const EMR_STROKEPATH: u32 = 64;
pub const EMR_TRANSPARENTBLT: u32 = 116;
pub const EMR_WIDENPATH: u32 = 66;
pub const ENABLEDUPLEX: u32 = 28;
pub const ENABLEPAIRKERNING: u32 = 769;
pub const ENABLERELATIVEWIDTHS: u32 = 768;
pub const ENCAPSULATED_POSTSCRIPT: u32 = 4116;
pub const ENDDOC: u32 = 11;
pub const END_PATH: u32 = 4098;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct ENHMETAHEADER {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::RECTL,
    pub rclFrame: super::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::SIZEL,
    pub szlMillimeters: super::SIZEL,
    pub cbPixelFormat: u32,
    pub offPixelFormat: u32,
    pub bOpenGL: u32,
    pub szlMicrometers: super::SIZEL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENHMETARECORD {
    pub iType: u32,
    pub nSize: u32,
    pub dParm: [u32; 1],
}
impl Default for ENHMETARECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENHMETA_SIGNATURE: u32 = 1179469088;
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ENHMFENUMPROC = Option<unsafe extern "system" fn(hdc: super::HDC, lpht: *const HANDLETABLE, lpmr: *const ENHMETARECORD, nhandles: i32, data: super::LPARAM) -> i32>;
pub type ENUMLOGFONT = ENUMLOGFONTA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
}
impl Default for ENUMLOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ENUMLOGFONTEX = ENUMLOGFONTEXA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [u8; 64],
    pub elfStyle: [u8; 32],
    pub elfScript: [u8; 32],
}
impl Default for ENUMLOGFONTEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ENUMLOGFONTEXDV = ENUMLOGFONTEXDVA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
    pub elfScript: [u16; 32],
}
impl Default for ENUMLOGFONTEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [u16; 64],
    pub elfStyle: [u16; 32],
}
impl Default for ENUMLOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENUMPAPERBINS: u32 = 31;
pub const ENUMPAPERMETRICS: u32 = 34;
pub type ENUMTEXTMETRIC = ENUMTEXTMETRICA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENUMTEXTMETRICA {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXA,
    pub etmAxesList: AXESLISTA,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENUMTEXTMETRICW {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXW,
    pub etmAxesList: AXESLISTW,
}
pub const EPSPRINTING: u32 = 33;
pub const EPS_SIGNATURE: u32 = 1179865157;
pub const ERROR: u32 = 0;
pub const ETO_CLIPPED: u32 = 4;
pub const ETO_GLYPH_INDEX: u32 = 16;
pub const ETO_IGNORELANGUAGE: u32 = 4096;
pub const ETO_NUMERICSLATIN: u32 = 2048;
pub const ETO_NUMERICSLOCAL: u32 = 1024;
pub const ETO_OPAQUE: u32 = 2;
pub const ETO_PDY: u32 = 8192;
pub const ETO_REVERSE_INDEX_MAP: u32 = 65536;
pub const ETO_RTLREADING: u32 = 128;
pub type EXTLOGFONT = EXTLOGFONTA;
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for EXTLOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for EXTLOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EXTLOGPEN {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: super::COLORREF,
    pub elpHatch: usize,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
#[cfg(feature = "windef")]
impl Default for EXTLOGPEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct EXTLOGPEN32 {
    pub elpPenStyle: u32,
    pub elpWidth: u32,
    pub elpBrushStyle: u32,
    pub elpColor: super::COLORREF,
    pub elpHatch: u32,
    pub elpNumEntries: u32,
    pub elpStyleEntry: [u32; 1],
}
#[cfg(feature = "windef")]
impl Default for EXTLOGPEN32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXTTEXTOUT: u32 = 512;
pub const EXT_DEVICE_CAPS: u32 = 4099;
pub const FEATURESETTING_CUSTPAPER: u32 = 3;
pub const FEATURESETTING_MIRROR: u32 = 4;
pub const FEATURESETTING_NEGATIVE: u32 = 5;
pub const FEATURESETTING_NUP: u32 = 0;
pub const FEATURESETTING_OUTPUT: u32 = 1;
pub const FEATURESETTING_PRIVATE_BEGIN: u32 = 4096;
pub const FEATURESETTING_PRIVATE_END: u32 = 8191;
pub const FEATURESETTING_PROTOCOL: u32 = 6;
pub const FEATURESETTING_PSLEVEL: u32 = 2;
pub const FF_DECORATIVE: u32 = 80;
pub const FF_DONTCARE: u32 = 0;
pub const FF_MODERN: u32 = 48;
pub const FF_ROMAN: u32 = 16;
pub const FF_SCRIPT: u32 = 64;
pub const FF_SWISS: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FIXED {
    pub fract: u16,
    pub value: i16,
}
pub const FIXED_PITCH: u32 = 1;
pub const FLI_GLYPHS: u32 = 262144;
pub const FLI_MASK: u32 = 4155;
pub const FLOODFILLBORDER: u32 = 0;
pub const FLOODFILLSURFACE: u32 = 1;
pub const FLUSHOUTPUT: u32 = 6;
#[cfg(feature = "minwindef")]
pub type FONTENUMPROC = FONTENUMPROCA;
#[cfg(feature = "minwindef")]
pub type FONTENUMPROCA = OLDFONTENUMPROCA;
#[cfg(feature = "minwindef")]
pub type FONTENUMPROCW = OLDFONTENUMPROCW;
pub const FONTMAPPER_MAX: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FONTSIGNATURE {
    pub fsUsb: [u32; 4],
    pub fsCsb: [u32; 2],
}
impl Default for FONTSIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FR_NOT_ENUM: u32 = 32;
pub const FR_PRIVATE: u32 = 16;
pub const FS_ARABIC: u32 = 64;
pub const FS_BALTIC: u32 = 128;
pub const FS_CHINESESIMP: u32 = 262144;
pub const FS_CHINESETRAD: u32 = 1048576;
pub const FS_CYRILLIC: u32 = 4;
pub const FS_GREEK: u32 = 8;
pub const FS_HEBREW: u32 = 32;
pub const FS_JISJAPAN: u32 = 131072;
pub const FS_JOHAB: u32 = 2097152;
pub const FS_LATIN1: u32 = 1;
pub const FS_LATIN2: u32 = 2;
pub const FS_SYMBOL: u32 = 2147483648;
pub const FS_THAI: u32 = 65536;
pub const FS_TURKISH: u32 = 16;
pub const FS_VIETNAMESE: u32 = 256;
pub const FS_WANSUNG: u32 = 524288;
pub const FW_BLACK: u32 = 900;
pub const FW_BOLD: u32 = 700;
pub const FW_DEMIBOLD: u32 = 600;
pub const FW_DONTCARE: u32 = 0;
pub const FW_EXTRABOLD: u32 = 800;
pub const FW_EXTRALIGHT: u32 = 200;
pub const FW_HEAVY: u32 = 900;
pub const FW_LIGHT: u32 = 300;
pub const FW_MEDIUM: u32 = 500;
pub const FW_NORMAL: u32 = 400;
pub const FW_REGULAR: u32 = 400;
pub const FW_SEMIBOLD: u32 = 600;
pub const FW_THIN: u32 = 100;
pub const FW_ULTRABOLD: u32 = 800;
pub const FW_ULTRALIGHT: u32 = 200;
pub type FXPT16DOT16 = i32;
pub type FXPT2DOT30 = i32;
pub const GB2312_CHARSET: u32 = 134;
pub const GCPCLASS_ARABIC: u32 = 2;
pub const GCPCLASS_HEBREW: u32 = 2;
pub const GCPCLASS_LATIN: u32 = 1;
pub const GCPCLASS_LATINNUMBER: u32 = 5;
pub const GCPCLASS_LATINNUMERICSEPARATOR: u32 = 7;
pub const GCPCLASS_LATINNUMERICTERMINATOR: u32 = 6;
pub const GCPCLASS_LOCALNUMBER: u32 = 4;
pub const GCPCLASS_NEUTRAL: u32 = 3;
pub const GCPCLASS_NUMERICSEPARATOR: u32 = 8;
pub const GCPCLASS_POSTBOUNDLTR: u32 = 32;
pub const GCPCLASS_POSTBOUNDRTL: u32 = 16;
pub const GCPCLASS_PREBOUNDLTR: u32 = 128;
pub const GCPCLASS_PREBOUNDRTL: u32 = 64;
pub const GCPGLYPH_LINKAFTER: u32 = 16384;
pub const GCPGLYPH_LINKBEFORE: u32 = 32768;
pub const GCP_CLASSIN: u32 = 524288;
pub const GCP_DBCS: u32 = 1;
pub const GCP_DIACRITIC: u32 = 256;
pub const GCP_DISPLAYZWG: u32 = 4194304;
pub const GCP_ERROR: u32 = 32768;
pub const GCP_GLYPHSHAPE: u32 = 16;
pub const GCP_JUSTIFY: u32 = 65536;
pub const GCP_JUSTIFYIN: u32 = 2097152;
pub const GCP_KASHIDA: u32 = 1024;
pub const GCP_LIGATE: u32 = 32;
pub const GCP_MAXEXTENT: u32 = 1048576;
pub const GCP_NEUTRALOVERRIDE: u32 = 33554432;
pub const GCP_NUMERICOVERRIDE: u32 = 16777216;
pub const GCP_NUMERICSLATIN: u32 = 67108864;
pub const GCP_NUMERICSLOCAL: u32 = 134217728;
pub const GCP_REORDER: u32 = 2;
pub type GCP_RESULTS = GCP_RESULTSA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GCP_RESULTSA {
    pub lStructSize: u32,
    pub lpOutString: windows_sys::core::PSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: windows_sys::core::PSTR,
    pub lpGlyphs: windows_sys::core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl Default for GCP_RESULTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GCP_RESULTSW {
    pub lStructSize: u32,
    pub lpOutString: windows_sys::core::PWSTR,
    pub lpOrder: *mut u32,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: windows_sys::core::PSTR,
    pub lpGlyphs: windows_sys::core::PWSTR,
    pub nGlyphs: u32,
    pub nMaxFit: i32,
}
impl Default for GCP_RESULTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GCP_SYMSWAPOFF: u32 = 8388608;
pub const GCP_USEKERNING: u32 = 8;
pub const GDICOMMENT_BEGINGROUP: u32 = 2;
pub const GDICOMMENT_ENDGROUP: u32 = 3;
pub const GDICOMMENT_IDENTIFIER: u32 = 1128875079;
pub const GDICOMMENT_MULTIFORMATS: u32 = 1073741828;
pub const GDICOMMENT_UNICODE_END: u32 = 128;
pub const GDICOMMENT_UNICODE_STRING: u32 = 64;
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649;
pub const GDIPLUS_TS_QUERYVER: u32 = 4122;
pub const GDIPLUS_TS_RECORD: u32 = 4123;
pub const GDI_ERROR: u32 = 4294967295;
pub const GDI_MAX_OBJ_TYPE: u32 = 14;
pub const GDI_MIN_OBJ_TYPE: u32 = 1;
pub const GDI_OBJ_LAST: u32 = 14;
pub const GETCOLORTABLE: u32 = 5;
pub const GETDEVICEUNITS: u32 = 42;
pub const GETEXTENDEDTEXTMETRICS: u32 = 256;
pub const GETEXTENTTABLE: u32 = 257;
pub const GETFACENAME: u32 = 513;
pub const GETPAIRKERNTABLE: u32 = 258;
pub const GETPENWIDTH: u32 = 16;
pub const GETPHYSPAGESIZE: u32 = 12;
pub const GETPRINTINGOFFSET: u32 = 13;
pub const GETSCALINGFACTOR: u32 = 14;
pub const GETSETPAPERBINS: u32 = 29;
pub const GETSETPAPERMETRICS: u32 = 35;
pub const GETSETPRINTORIENT: u32 = 30;
pub const GETSETSCREENPARAMS: u32 = 3072;
pub const GETTECHNOLGY: u32 = 20;
pub const GETTECHNOLOGY: u32 = 20;
pub const GETTRACKKERNTABLE: u32 = 259;
pub const GETVECTORBRUSHSIZE: u32 = 27;
pub const GETVECTORPENSIZE: u32 = 26;
pub const GET_PS_FEATURESETTING: u32 = 4121;
pub const GGI_MARK_NONEXISTING_GLYPHS: u32 = 1;
pub const GGO_BEZIER: u32 = 3;
pub const GGO_BITMAP: u32 = 1;
pub const GGO_GLYPH_INDEX: u32 = 128;
pub const GGO_GRAY2_BITMAP: u32 = 4;
pub const GGO_GRAY4_BITMAP: u32 = 5;
pub const GGO_GRAY8_BITMAP: u32 = 6;
pub const GGO_METRICS: u32 = 0;
pub const GGO_NATIVE: u32 = 2;
pub const GGO_UNHINTED: u32 = 256;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: u32,
    pub gmBlackBoxY: u32,
    pub gmptGlyphOrigin: super::POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: f32,
    pub gmfBlackBoxY: f32,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: f32,
    pub gmfCellIncY: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLYPHSET {
    pub cbThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRanges: u32,
    pub ranges: [WCRANGE; 1],
}
impl Default for GLYPHSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GM_ADVANCED: u32 = 2;
pub const GM_COMPATIBLE: u32 = 1;
pub const GM_LAST: u32 = 2;
#[cfg(feature = "minwindef")]
pub type GOBJENUMPROC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::LPARAM) -> i32>;
pub const GRADIENT_FILL_OP_FLAG: u32 = 255;
pub const GRADIENT_FILL_RECT_H: u32 = 0;
pub const GRADIENT_FILL_RECT_V: u32 = 1;
pub const GRADIENT_FILL_TRIANGLE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GRADIENT_RECT {
    pub UpperLeft: u32,
    pub LowerRight: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GRADIENT_TRIANGLE {
    pub Vertex1: u32,
    pub Vertex2: u32,
    pub Vertex3: u32,
}
pub const GRAY_BRUSH: u32 = 2;
pub const GREEK_CHARSET: u32 = 161;
pub const GS_8BIT_INDICES: u32 = 1;
pub const HALFTONE: u32 = 4;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct HANDLETABLE {
    pub objectHandle: [super::HGDIOBJ; 1],
}
#[cfg(feature = "windef")]
impl Default for HANDLETABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HANGEUL_CHARSET: u32 = 129;
pub const HANGUL_CHARSET: u32 = 129;
pub const HEBREW_CHARSET: u32 = 177;
pub const HOLLOW_BRUSH: u32 = 5;
pub const HORZRES: u32 = 8;
pub const HORZSIZE: u32 = 4;
pub const HS_API_MAX: u32 = 12;
pub const HS_BDIAGONAL: u32 = 3;
pub const HS_CROSS: u32 = 4;
pub const HS_DIAGCROSS: u32 = 5;
pub const HS_FDIAGONAL: u32 = 2;
pub const HS_HORIZONTAL: u32 = 0;
pub const HS_VERTICAL: u32 = 1;
#[cfg(feature = "minwindef")]
pub type ICMENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: super::LPARAM) -> i32>;
#[cfg(feature = "minwindef")]
pub type ICMENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::LPARAM) -> i32>;
pub const ICM_ADDPROFILE: u32 = 1;
pub const ICM_DELETEPROFILE: u32 = 2;
pub const ICM_DONE_OUTSIDEDC: u32 = 4;
pub const ICM_OFF: u32 = 1;
pub const ICM_ON: u32 = 2;
pub const ICM_QUERY: u32 = 3;
pub const ICM_QUERYMATCH: u32 = 7;
pub const ICM_QUERYPROFILE: u32 = 3;
pub const ICM_REGISTERICMATCHER: u32 = 5;
pub const ICM_SETDEFAULTPROFILE: u32 = 4;
pub const ICM_UNREGISTERICMATCHER: u32 = 6;
pub const ILLUMINANT_A: u32 = 1;
pub const ILLUMINANT_B: u32 = 2;
pub const ILLUMINANT_C: u32 = 3;
pub const ILLUMINANT_D50: u32 = 4;
pub const ILLUMINANT_D55: u32 = 5;
pub const ILLUMINANT_D65: u32 = 6;
pub const ILLUMINANT_D75: u32 = 7;
pub const ILLUMINANT_DAYLIGHT: u32 = 3;
pub const ILLUMINANT_DEVICE_DEFAULT: u32 = 0;
pub const ILLUMINANT_F2: u32 = 8;
pub const ILLUMINANT_FLUORESCENT: u32 = 8;
pub const ILLUMINANT_MAX_INDEX: u32 = 8;
pub const ILLUMINANT_NTSC: u32 = 3;
pub const ILLUMINANT_TUNGSTEN: u32 = 1;
pub const JOHAB_CHARSET: u32 = 130;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERNINGPAIR {
    pub wFirst: u16,
    pub wSecond: u16,
    pub iKernAmount: i32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerPlane: u8,
    pub bReserved: u8,
    pub crTransparent: super::COLORREF,
}
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: u32 = 8;
pub const LAYOUT_BTT: u32 = 2;
pub const LAYOUT_ORIENTATIONMASK: u32 = 7;
pub const LAYOUT_RTL: u32 = 1;
pub const LAYOUT_VBH: u32 = 4;
pub type LCSCSTYPE = i32;
pub type LCSGAMUTMATCH = i32;
pub const LCS_CALIBRATED_RGB: u32 = 0;
pub const LCS_GM_ABS_COLORIMETRIC: u32 = 8;
pub const LCS_GM_BUSINESS: u32 = 1;
pub const LCS_GM_GRAPHICS: u32 = 2;
pub const LCS_GM_IMAGES: u32 = 4;
pub const LCS_SIGNATURE: u32 = 1347637059;
pub const LCS_WINDOWS_COLOR_SPACE: u32 = 1466527264;
pub const LCS_sRGB: u32 = 1934772034;
pub const LC_INTERIORS: u32 = 128;
pub const LC_MARKER: u32 = 4;
pub const LC_NONE: u32 = 0;
pub const LC_POLYLINE: u32 = 2;
pub const LC_POLYMARKER: u32 = 8;
pub const LC_STYLED: u32 = 32;
pub const LC_WIDE: u32 = 16;
pub const LC_WIDESTYLED: u32 = 64;
pub const LF_FACESIZE: u32 = 32;
pub const LF_FULLFACESIZE: u32 = 64;
pub const LINECAPS: u32 = 30;
#[cfg(feature = "minwindef")]
pub type LINEDDAPROC = Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: super::LPARAM)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOCALESIGNATURE {
    pub lsUsb: [u32; 4],
    pub lsCsbDefault: [u32; 2],
    pub lsCsbSupported: [u32; 2],
}
impl Default for LOCALESIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct LOGBRUSH {
    pub lbStyle: u32,
    pub lbColor: super::COLORREF,
    pub lbHatch: usize,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct LOGBRUSH32 {
    pub lbStyle: u32,
    pub lbColor: super::COLORREF,
    pub lbHatch: u32,
}
pub type LOGCOLORSPACE = LOGCOLORSPACEA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: LCSGAMUTMATCH,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [i8; 260],
}
impl Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: LCSGAMUTMATCH,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [u16; 260],
}
impl Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LOGFONT = LOGFONTA;
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub lfFaceName: [i8; 32],
}
impl Default for LOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for LOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOGPALETTE {
    pub palVersion: u16,
    pub palNumEntries: u16,
    pub palPalEntry: [PALETTEENTRY; 1],
}
impl Default for LOGPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct LOGPEN {
    pub lopnStyle: u32,
    pub lopnWidth: super::POINT,
    pub lopnColor: super::COLORREF,
}
pub const LOGPIXELSX: u32 = 88;
pub const LOGPIXELSY: u32 = 90;
pub type LPABC = *mut ABC;
pub type LPABCFLOAT = *mut ABCFLOAT;
pub type LPAXESLIST = LPAXESLISTA;
pub type LPAXESLISTA = *mut AXESLISTA;
pub type LPAXESLISTW = *mut AXESLISTW;
pub type LPAXISINFO = LPAXISINFOA;
pub type LPAXISINFOA = *mut AXISINFOA;
pub type LPAXISINFOW = *mut AXISINFOW;
pub type LPBITMAP = *mut BITMAP;
pub type LPBITMAPCOREHEADER = *mut BITMAPCOREHEADER;
pub type LPBITMAPCOREINFO = *mut BITMAPCOREINFO;
pub type LPBITMAPFILEHEADER = *mut BITMAPFILEHEADER;
pub type LPBITMAPINFO = *mut BITMAPINFO;
pub type LPBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
pub type LPBITMAPV4HEADER = *mut BITMAPV4HEADER;
pub type LPBITMAPV5HEADER = *mut BITMAPV5HEADER;
pub type LPCHARSETINFO = *mut CHARSETINFO;
pub type LPCIEXYZ = *mut CIEXYZ;
pub type LPCIEXYZTRIPLE = *mut CIEXYZTRIPLE;
pub type LPCOLORADJUSTMENT = *mut COLORADJUSTMENT;
pub type LPDESIGNVECTOR = *mut DESIGNVECTOR;
#[cfg(feature = "windef")]
pub type LPDEVMODE = LPDEVMODEA;
#[cfg(feature = "windef")]
pub type LPDEVMODEA = *mut DEVMODEA;
#[cfg(feature = "windef")]
pub type LPDEVMODEW = *mut DEVMODEW;
#[cfg(feature = "winnt")]
pub type LPDIBSECTION = *mut DIBSECTION;
pub type LPDISPLAY_DEVICE = LPDISPLAY_DEVICEA;
pub type LPDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub type LPDOCINFO = LPDOCINFOA;
pub type LPDOCINFOA = *mut DOCINFOA;
pub type LPDOCINFOW = *mut DOCINFOW;
pub const LPD_DOUBLEBUFFER: u32 = 1;
pub const LPD_SHARE_ACCUM: u32 = 256;
pub const LPD_SHARE_DEPTH: u32 = 64;
pub const LPD_SHARE_STENCIL: u32 = 128;
pub const LPD_STEREO: u32 = 2;
pub const LPD_SUPPORT_GDI: u32 = 16;
pub const LPD_SUPPORT_OPENGL: u32 = 32;
pub const LPD_SWAP_COPY: u32 = 1024;
pub const LPD_SWAP_EXCHANGE: u32 = 512;
pub const LPD_TRANSPARENT: u32 = 4096;
pub const LPD_TYPE_COLORINDEX: u32 = 1;
pub const LPD_TYPE_RGBA: u32 = 0;
#[cfg(feature = "windef")]
pub type LPENHMETAHEADER = *mut ENHMETAHEADER;
pub type LPENHMETARECORD = *mut ENHMETARECORD;
pub type LPENUMLOGFONT = LPENUMLOGFONTA;
pub type LPENUMLOGFONTA = *mut ENUMLOGFONTA;
pub type LPENUMLOGFONTEX = LPENUMLOGFONTEXA;
pub type LPENUMLOGFONTEXA = *mut ENUMLOGFONTEXA;
pub type LPENUMLOGFONTEXDV = LPENUMLOGFONTEXDVA;
pub type LPENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
pub type LPENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
pub type LPENUMLOGFONTEXW = *mut ENUMLOGFONTEXW;
pub type LPENUMLOGFONTW = *mut ENUMLOGFONTW;
pub type LPENUMTEXTMETRIC = LPENUMTEXTMETRICA;
pub type LPENUMTEXTMETRICA = *mut ENUMTEXTMETRICA;
pub type LPENUMTEXTMETRICW = *mut ENUMTEXTMETRICW;
pub type LPEXTLOGFONT = LPEXTLOGFONTA;
pub type LPEXTLOGFONTA = *mut EXTLOGFONTA;
pub type LPEXTLOGFONTW = *mut EXTLOGFONTW;
#[cfg(feature = "windef")]
pub type LPEXTLOGPEN = *mut EXTLOGPEN;
#[cfg(feature = "windef")]
pub type LPEXTLOGPEN32 = *mut EXTLOGPEN32;
#[cfg(feature = "windef")]
pub type LPFNDEVCAPS = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: u32, param3: windows_sys::core::PCSTR, param4: LPDEVMODE) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFNDEVMODE = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::HMODULE, param2: LPDEVMODE, param3: windows_sys::core::PCSTR, param4: windows_sys::core::PCSTR, param5: LPDEVMODE, param6: windows_sys::core::PCSTR, param7: u32) -> u32>;
pub type LPFONTSIGNATURE = *mut FONTSIGNATURE;
pub type LPFXPT16DOT16 = *mut i32;
pub type LPFXPT2DOT30 = *mut i32;
pub type LPGCP_RESULTS = LPGCP_RESULTSA;
pub type LPGCP_RESULTSA = *mut GCP_RESULTSA;
pub type LPGCP_RESULTSW = *mut GCP_RESULTSW;
#[cfg(feature = "windef")]
pub type LPGLYPHMETRICS = *mut GLYPHMETRICS;
pub type LPGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
pub type LPGLYPHSET = *mut GLYPHSET;
pub type LPGRADIENT_RECT = *mut GRADIENT_RECT;
pub type LPGRADIENT_TRIANGLE = *mut GRADIENT_TRIANGLE;
#[cfg(feature = "windef")]
pub type LPHANDLETABLE = *mut HANDLETABLE;
pub type LPKERNINGPAIR = *mut KERNINGPAIR;
#[cfg(feature = "windef")]
pub type LPLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
pub type LPLOCALESIGNATURE = *mut LOCALESIGNATURE;
#[cfg(feature = "windef")]
pub type LPLOGBRUSH = *mut LOGBRUSH;
#[cfg(feature = "windef")]
pub type LPLOGBRUSH32 = *mut LOGBRUSH32;
pub type LPLOGCOLORSPACE = LPLOGCOLORSPACEA;
pub type LPLOGCOLORSPACEA = *mut LOGCOLORSPACEA;
pub type LPLOGCOLORSPACEW = *mut LOGCOLORSPACEW;
pub type LPLOGFONT = LPLOGFONTA;
pub type LPLOGFONTA = *mut LOGFONTA;
pub type LPLOGFONTW = *mut LOGFONTW;
pub type LPLOGPALETTE = *mut LOGPALETTE;
#[cfg(feature = "windef")]
pub type LPLOGPEN = *mut LOGPEN;
pub type LPMAT2 = *mut MAT2;
#[cfg(feature = "minwindef")]
pub type LPMETAFILEPICT = *mut METAFILEPICT;
pub type LPMETAHEADER = *mut METAHEADER;
pub type LPMETARECORD = *mut METARECORD;
pub type LPNEWTEXTMETRIC = LPNEWTEXTMETRICA;
pub type LPNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
pub type LPNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
#[cfg(feature = "windef")]
pub type LPOUTLINETEXTMETRIC = LPOUTLINETEXTMETRICA;
#[cfg(feature = "windef")]
pub type LPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
#[cfg(feature = "windef")]
pub type LPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
pub type LPPALETTEENTRY = *mut PALETTEENTRY;
pub type LPPANOSE = *mut PANOSE;
#[cfg(feature = "windef")]
pub type LPPATTERN = *mut PATTERN;
pub type LPPELARRAY = *mut PELARRAY;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPOINTFX = *mut POINTFX;
#[cfg(feature = "windef")]
pub type LPPOLYTEXT = LPPOLYTEXTA;
#[cfg(feature = "windef")]
pub type LPPOLYTEXTA = *mut POLYTEXTA;
#[cfg(feature = "windef")]
pub type LPPOLYTEXTW = *mut POLYTEXTW;
pub type LPRASTERIZER_STATUS = *mut RASTERIZER_STATUS;
pub type LPRGBQUAD = *mut RGBQUAD;
pub type LPRGBTRIPLE = *mut RGBTRIPLE;
#[cfg(feature = "windef")]
pub type LPRGNDATA = *mut RGNDATA;
pub type LPTEXTMETRIC = LPTEXTMETRICA;
pub type LPTEXTMETRICA = *mut TEXTMETRICA;
pub type LPTEXTMETRICW = *mut TEXTMETRICW;
pub type LPTRIVERTEX = *mut TRIVERTEX;
pub type LPTTPOLYCURVE = *mut TTPOLYCURVE;
pub type LPTTPOLYGONHEADER = *mut TTPOLYGONHEADER;
pub type LPWCRANGE = *mut WCRANGE;
#[cfg(feature = "windef")]
pub type LPWGLSWAP = *mut WGLSWAP;
pub type LPXFORM = *mut XFORM;
pub const LTGRAY_BRUSH: u32 = 1;
pub const MAC_CHARSET: u32 = 77;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
pub const MAXSTRETCHBLTMODE: u32 = 4;
pub const MERGECOPY: u32 = 12583114;
pub const MERGEPAINT: u32 = 12255782;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct METAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: super::HMETAFILE,
}
#[cfg(feature = "minwindef")]
impl Default for METAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const METAFILE_DRIVER: u32 = 2049;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct METAHEADER {
    pub mtType: u16,
    pub mtHeaderSize: u16,
    pub mtVersion: u16,
    pub mtSize: u32,
    pub mtNoObjects: u16,
    pub mtMaxRecord: u32,
    pub mtNoParameters: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct METARECORD {
    pub rdSize: u32,
    pub rdFunction: u16,
    pub rdParm: [u16; 1],
}
impl Default for METARECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const META_ANIMATEPALETTE: u32 = 1078;
pub const META_ARC: u32 = 2071;
pub const META_BITBLT: u32 = 2338;
pub const META_CHORD: u32 = 2096;
pub const META_CREATEBRUSHINDIRECT: u32 = 764;
pub const META_CREATEFONTINDIRECT: u32 = 763;
pub const META_CREATEPALETTE: u32 = 247;
pub const META_CREATEPATTERNBRUSH: u32 = 505;
pub const META_CREATEPENINDIRECT: u32 = 762;
pub const META_CREATEREGION: u32 = 1791;
pub const META_DELETEOBJECT: u32 = 496;
pub const META_DIBBITBLT: u32 = 2368;
pub const META_DIBCREATEPATTERNBRUSH: u32 = 322;
pub const META_DIBSTRETCHBLT: u32 = 2881;
pub const META_ELLIPSE: u32 = 1048;
pub const META_ESCAPE: u32 = 1574;
pub const META_EXCLUDECLIPRECT: u32 = 1045;
pub const META_EXTFLOODFILL: u32 = 1352;
pub const META_EXTTEXTOUT: u32 = 2610;
pub const META_FILLREGION: u32 = 552;
pub const META_FLOODFILL: u32 = 1049;
pub const META_FRAMEREGION: u32 = 1065;
pub const META_INTERSECTCLIPRECT: u32 = 1046;
pub const META_INVERTREGION: u32 = 298;
pub const META_LINETO: u32 = 531;
pub const META_MOVETO: u32 = 532;
pub const META_OFFSETCLIPRGN: u32 = 544;
pub const META_OFFSETVIEWPORTORG: u32 = 529;
pub const META_OFFSETWINDOWORG: u32 = 527;
pub const META_PAINTREGION: u32 = 299;
pub const META_PATBLT: u32 = 1565;
pub const META_PIE: u32 = 2074;
pub const META_POLYGON: u32 = 804;
pub const META_POLYLINE: u32 = 805;
pub const META_POLYPOLYGON: u32 = 1336;
pub const META_REALIZEPALETTE: u32 = 53;
pub const META_RECTANGLE: u32 = 1051;
pub const META_RESIZEPALETTE: u32 = 313;
pub const META_RESTOREDC: u32 = 295;
pub const META_ROUNDRECT: u32 = 1564;
pub const META_SAVEDC: u32 = 30;
pub const META_SCALEVIEWPORTEXT: u32 = 1042;
pub const META_SCALEWINDOWEXT: u32 = 1040;
pub const META_SELECTCLIPREGION: u32 = 300;
pub const META_SELECTOBJECT: u32 = 301;
pub const META_SELECTPALETTE: u32 = 564;
pub const META_SETBKCOLOR: u32 = 513;
pub const META_SETBKMODE: u32 = 258;
pub const META_SETDIBTODEV: u32 = 3379;
pub const META_SETLAYOUT: u32 = 329;
pub const META_SETMAPMODE: u32 = 259;
pub const META_SETMAPPERFLAGS: u32 = 561;
pub const META_SETPALENTRIES: u32 = 55;
pub const META_SETPIXEL: u32 = 1055;
pub const META_SETPOLYFILLMODE: u32 = 262;
pub const META_SETRELABS: u32 = 261;
pub const META_SETROP2: u32 = 260;
pub const META_SETSTRETCHBLTMODE: u32 = 263;
pub const META_SETTEXTALIGN: u32 = 302;
pub const META_SETTEXTCHAREXTRA: u32 = 264;
pub const META_SETTEXTCOLOR: u32 = 521;
pub const META_SETTEXTJUSTIFICATION: u32 = 522;
pub const META_SETVIEWPORTEXT: u32 = 526;
pub const META_SETVIEWPORTORG: u32 = 525;
pub const META_SETWINDOWEXT: u32 = 524;
pub const META_SETWINDOWORG: u32 = 523;
pub const META_STRETCHBLT: u32 = 2851;
pub const META_STRETCHDIB: u32 = 3907;
pub const META_TEXTOUT: u32 = 1313;
pub const MFCOMMENT: u32 = 15;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type MFENUMPROC = Option<unsafe extern "system" fn(hdc: super::HDC, lpht: *const HANDLETABLE, lpmr: *const METARECORD, nobj: i32, param: super::LPARAM) -> i32>;
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: u32 = 0;
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: u32 = 2147483647;
pub const MM_ANISOTROPIC: u32 = 8;
pub const MM_HIENGLISH: u32 = 5;
pub const MM_HIMETRIC: u32 = 3;
pub const MM_ISOTROPIC: u32 = 7;
pub const MM_LOENGLISH: u32 = 4;
pub const MM_LOMETRIC: u32 = 2;
pub const MM_MAX: u32 = 8;
pub const MM_MAX_AXES_NAMELEN: u32 = 16;
pub const MM_MAX_FIXEDSCALE: u32 = 6;
pub const MM_MAX_NUMAXES: u32 = 16;
pub const MM_MIN: u32 = 1;
pub const MM_TEXT: u32 = 1;
pub const MM_TWIPS: u32 = 6;
pub const MONO_FONT: u32 = 8;
pub const MOUSETRAILS: u32 = 39;
pub const MWT_IDENTITY: u32 = 1;
pub const MWT_LEFTMULTIPLY: u32 = 2;
pub const MWT_MAX: u32 = 3;
pub const MWT_MIN: u32 = 1;
pub const MWT_RIGHTMULTIPLY: u32 = 3;
pub const NEWFRAME: u32 = 1;
pub type NEWTEXTMETRIC = NEWTEXTMETRICA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type NEWTEXTMETRICEX = NEWTEXTMETRICEXA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NEWTEXTMETRICEXA {
    pub ntmTm: NEWTEXTMETRICA,
    pub ntmFontSig: FONTSIGNATURE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NEWTEXTMETRICEXW {
    pub ntmTm: NEWTEXTMETRICW,
    pub ntmFontSig: FONTSIGNATURE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const NEXTBAND: u32 = 3;
pub const NOMIRRORBITMAP: u32 = 2147483648;
pub const NONANTIALIASED_QUALITY: u32 = 3;
pub const NOTSRCCOPY: u32 = 3342344;
pub const NOTSRCERASE: u32 = 1114278;
pub type NPABC = *mut ABC;
pub type NPABCFLOAT = *mut ABCFLOAT;
pub type NPBITMAP = *mut BITMAP;
pub type NPCHARSETINFO = *mut CHARSETINFO;
#[cfg(feature = "windef")]
pub type NPDEVMODE = NPDEVMODEA;
#[cfg(feature = "windef")]
pub type NPDEVMODEA = *mut DEVMODEA;
#[cfg(feature = "windef")]
pub type NPDEVMODEW = *mut DEVMODEW;
pub type NPEXTLOGFONT = NPEXTLOGFONTA;
pub type NPEXTLOGFONTA = *mut EXTLOGFONTA;
pub type NPEXTLOGFONTW = *mut EXTLOGFONTW;
#[cfg(feature = "windef")]
pub type NPEXTLOGPEN = *mut EXTLOGPEN;
#[cfg(feature = "windef")]
pub type NPEXTLOGPEN32 = *mut EXTLOGPEN32;
#[cfg(feature = "windef")]
pub type NPLOGBRUSH = *mut LOGBRUSH;
#[cfg(feature = "windef")]
pub type NPLOGBRUSH32 = *mut LOGBRUSH32;
pub type NPLOGFONT = NPLOGFONTA;
pub type NPLOGFONTA = *mut LOGFONTA;
pub type NPLOGFONTW = *mut LOGFONTW;
pub type NPLOGPALETTE = *mut LOGPALETTE;
#[cfg(feature = "windef")]
pub type NPLOGPEN = *mut LOGPEN;
pub type NPNEWTEXTMETRIC = NPNEWTEXTMETRICA;
pub type NPNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
pub type NPNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
#[cfg(feature = "windef")]
pub type NPOUTLINETEXTMETRIC = NPOUTLINETEXTMETRICA;
#[cfg(feature = "windef")]
pub type NPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
#[cfg(feature = "windef")]
pub type NPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
#[cfg(feature = "windef")]
pub type NPPATTERN = *mut PATTERN;
pub type NPPELARRAY = *mut PELARRAY;
#[cfg(feature = "windef")]
pub type NPPOLYTEXT = NPPOLYTEXTA;
#[cfg(feature = "windef")]
pub type NPPOLYTEXTA = *mut POLYTEXTA;
#[cfg(feature = "windef")]
pub type NPPOLYTEXTW = *mut POLYTEXTW;
pub type NPRGBTRIPLE = *mut RGBTRIPLE;
#[cfg(feature = "windef")]
pub type NPRGNDATA = *mut RGNDATA;
pub type NPTEXTMETRIC = NPTEXTMETRICA;
pub type NPTEXTMETRICA = *mut TEXTMETRICA;
pub type NPTEXTMETRICW = *mut TEXTMETRICW;
pub const NTM_BOLD: u32 = 32;
pub const NTM_DSIG: u32 = 2097152;
pub const NTM_ITALIC: u32 = 1;
pub const NTM_MULTIPLEMASTER: u32 = 524288;
pub const NTM_NONNEGATIVE_AC: u32 = 65536;
pub const NTM_PS_OPENTYPE: u32 = 131072;
pub const NTM_REGULAR: u32 = 64;
pub const NTM_TT_OPENTYPE: u32 = 262144;
pub const NTM_TYPE1: u32 = 1048576;
pub const NULLREGION: u32 = 1;
pub const NULL_BRUSH: u32 = 5;
pub const NULL_PEN: u32 = 8;
pub const NUMBRUSHES: u32 = 16;
pub const NUMCOLORS: u32 = 24;
pub const NUMFONTS: u32 = 22;
pub const NUMMARKERS: u32 = 20;
pub const NUMPENS: u32 = 18;
pub const NUMRESERVED: u32 = 106;
pub const OBJ_BITMAP: u32 = 7;
pub const OBJ_BRUSH: u32 = 2;
pub const OBJ_COLORSPACE: u32 = 14;
pub const OBJ_DC: u32 = 3;
pub const OBJ_ENHMETADC: u32 = 12;
pub const OBJ_ENHMETAFILE: u32 = 13;
pub const OBJ_EXTPEN: u32 = 11;
pub const OBJ_FONT: u32 = 6;
pub const OBJ_MEMDC: u32 = 10;
pub const OBJ_METADC: u32 = 4;
pub const OBJ_METAFILE: u32 = 9;
pub const OBJ_PAL: u32 = 5;
pub const OBJ_PEN: u32 = 1;
pub const OBJ_REGION: u32 = 8;
pub const OEM_CHARSET: u32 = 255;
pub const OEM_FIXED_FONT: u32 = 10;
#[cfg(feature = "minwindef")]
pub type OLDFONTENUMPROCA = Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: u32, param3: super::LPARAM) -> i32>;
#[cfg(feature = "minwindef")]
pub type OLDFONTENUMPROCW = Option<unsafe extern "system" fn(param0: *const LOGFONTW, param1: *const TEXTMETRICW, param2: u32, param3: super::LPARAM) -> i32>;
pub const OPAQUE: u32 = 2;
pub const OPENCHANNEL: u32 = 4110;
#[cfg(feature = "windef")]
pub type OUTLINETEXTMETRIC = OUTLINETEXTMETRICA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
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
    pub otmrcFontBox: super::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::POINT,
    pub otmptSubscriptOffset: super::POINT,
    pub otmptSuperscriptSize: super::POINT,
    pub otmptSuperscriptOffset: super::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: windows_sys::core::PSTR,
    pub otmpFaceName: windows_sys::core::PSTR,
    pub otmpStyleName: windows_sys::core::PSTR,
    pub otmpFullName: windows_sys::core::PSTR,
}
#[cfg(feature = "windef")]
impl Default for OUTLINETEXTMETRICA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
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
    pub otmrcFontBox: super::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: u32,
    pub otmusMinimumPPEM: u32,
    pub otmptSubscriptSize: super::POINT,
    pub otmptSubscriptOffset: super::POINT,
    pub otmptSuperscriptSize: super::POINT,
    pub otmptSuperscriptOffset: super::POINT,
    pub otmsStrikeoutSize: u32,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: windows_sys::core::PSTR,
    pub otmpFaceName: windows_sys::core::PSTR,
    pub otmpStyleName: windows_sys::core::PSTR,
    pub otmpFullName: windows_sys::core::PSTR,
}
#[cfg(feature = "windef")]
impl Default for OUTLINETEXTMETRICW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OUT_CHARACTER_PRECIS: u32 = 2;
pub const OUT_DEFAULT_PRECIS: u32 = 0;
pub const OUT_DEVICE_PRECIS: u32 = 5;
pub const OUT_OUTLINE_PRECIS: u32 = 8;
pub const OUT_PS_ONLY_PRECIS: u32 = 10;
pub const OUT_RASTER_PRECIS: u32 = 6;
pub const OUT_SCREEN_OUTLINE_PRECIS: u32 = 9;
pub const OUT_STRING_PRECIS: u32 = 1;
pub const OUT_STROKE_PRECIS: u32 = 3;
pub const OUT_TT_ONLY_PRECIS: u32 = 7;
pub const OUT_TT_PRECIS: u32 = 4;
pub type PABC = *mut ABC;
pub type PABCFLOAT = *mut ABCFLOAT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const PANOSE_COUNT: u32 = 10;
pub const PAN_ANY: u32 = 0;
pub const PAN_ARMSTYLE_INDEX: u32 = 6;
pub const PAN_BENT_ARMS_DOUBLE_SERIF: u32 = 11;
pub const PAN_BENT_ARMS_HORZ: u32 = 7;
pub const PAN_BENT_ARMS_SINGLE_SERIF: u32 = 10;
pub const PAN_BENT_ARMS_VERT: u32 = 9;
pub const PAN_BENT_ARMS_WEDGE: u32 = 8;
pub const PAN_CONTRAST_HIGH: u32 = 8;
pub const PAN_CONTRAST_INDEX: u32 = 4;
pub const PAN_CONTRAST_LOW: u32 = 4;
pub const PAN_CONTRAST_MEDIUM: u32 = 6;
pub const PAN_CONTRAST_MEDIUM_HIGH: u32 = 7;
pub const PAN_CONTRAST_MEDIUM_LOW: u32 = 5;
pub const PAN_CONTRAST_NONE: u32 = 2;
pub const PAN_CONTRAST_VERY_HIGH: u32 = 9;
pub const PAN_CONTRAST_VERY_LOW: u32 = 3;
pub const PAN_CULTURE_LATIN: u32 = 0;
pub const PAN_FAMILYTYPE_INDEX: u32 = 0;
pub const PAN_FAMILY_DECORATIVE: u32 = 4;
pub const PAN_FAMILY_PICTORIAL: u32 = 5;
pub const PAN_FAMILY_SCRIPT: u32 = 3;
pub const PAN_FAMILY_TEXT_DISPLAY: u32 = 2;
pub const PAN_LETTERFORM_INDEX: u32 = 7;
pub const PAN_LETT_NORMAL_BOXED: u32 = 4;
pub const PAN_LETT_NORMAL_CONTACT: u32 = 2;
pub const PAN_LETT_NORMAL_FLATTENED: u32 = 5;
pub const PAN_LETT_NORMAL_OFF_CENTER: u32 = 7;
pub const PAN_LETT_NORMAL_ROUNDED: u32 = 6;
pub const PAN_LETT_NORMAL_SQUARE: u32 = 8;
pub const PAN_LETT_NORMAL_WEIGHTED: u32 = 3;
pub const PAN_LETT_OBLIQUE_BOXED: u32 = 11;
pub const PAN_LETT_OBLIQUE_CONTACT: u32 = 9;
pub const PAN_LETT_OBLIQUE_FLATTENED: u32 = 12;
pub const PAN_LETT_OBLIQUE_OFF_CENTER: u32 = 14;
pub const PAN_LETT_OBLIQUE_ROUNDED: u32 = 13;
pub const PAN_LETT_OBLIQUE_SQUARE: u32 = 15;
pub const PAN_LETT_OBLIQUE_WEIGHTED: u32 = 10;
pub const PAN_MIDLINE_CONSTANT_POINTED: u32 = 9;
pub const PAN_MIDLINE_CONSTANT_SERIFED: u32 = 10;
pub const PAN_MIDLINE_CONSTANT_TRIMMED: u32 = 8;
pub const PAN_MIDLINE_HIGH_POINTED: u32 = 6;
pub const PAN_MIDLINE_HIGH_SERIFED: u32 = 7;
pub const PAN_MIDLINE_HIGH_TRIMMED: u32 = 5;
pub const PAN_MIDLINE_INDEX: u32 = 8;
pub const PAN_MIDLINE_LOW_POINTED: u32 = 12;
pub const PAN_MIDLINE_LOW_SERIFED: u32 = 13;
pub const PAN_MIDLINE_LOW_TRIMMED: u32 = 11;
pub const PAN_MIDLINE_STANDARD_POINTED: u32 = 3;
pub const PAN_MIDLINE_STANDARD_SERIFED: u32 = 4;
pub const PAN_MIDLINE_STANDARD_TRIMMED: u32 = 2;
pub const PAN_NO_FIT: u32 = 1;
pub const PAN_PROPORTION_INDEX: u32 = 3;
pub const PAN_PROP_CONDENSED: u32 = 6;
pub const PAN_PROP_EVEN_WIDTH: u32 = 4;
pub const PAN_PROP_EXPANDED: u32 = 5;
pub const PAN_PROP_MODERN: u32 = 3;
pub const PAN_PROP_MONOSPACED: u32 = 9;
pub const PAN_PROP_OLD_STYLE: u32 = 2;
pub const PAN_PROP_VERY_CONDENSED: u32 = 8;
pub const PAN_PROP_VERY_EXPANDED: u32 = 7;
pub const PAN_SERIFSTYLE_INDEX: u32 = 1;
pub const PAN_SERIF_BONE: u32 = 8;
pub const PAN_SERIF_COVE: u32 = 2;
pub const PAN_SERIF_EXAGGERATED: u32 = 9;
pub const PAN_SERIF_FLARED: u32 = 14;
pub const PAN_SERIF_NORMAL_SANS: u32 = 11;
pub const PAN_SERIF_OBTUSE_COVE: u32 = 3;
pub const PAN_SERIF_OBTUSE_SANS: u32 = 12;
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: u32 = 5;
pub const PAN_SERIF_PERP_SANS: u32 = 13;
pub const PAN_SERIF_ROUNDED: u32 = 15;
pub const PAN_SERIF_SQUARE: u32 = 6;
pub const PAN_SERIF_SQUARE_COVE: u32 = 4;
pub const PAN_SERIF_THIN: u32 = 7;
pub const PAN_SERIF_TRIANGLE: u32 = 10;
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: u32 = 6;
pub const PAN_STRAIGHT_ARMS_HORZ: u32 = 2;
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: u32 = 5;
pub const PAN_STRAIGHT_ARMS_VERT: u32 = 4;
pub const PAN_STRAIGHT_ARMS_WEDGE: u32 = 3;
pub const PAN_STROKEVARIATION_INDEX: u32 = 5;
pub const PAN_STROKE_GRADUAL_DIAG: u32 = 2;
pub const PAN_STROKE_GRADUAL_HORZ: u32 = 5;
pub const PAN_STROKE_GRADUAL_TRAN: u32 = 3;
pub const PAN_STROKE_GRADUAL_VERT: u32 = 4;
pub const PAN_STROKE_INSTANT_VERT: u32 = 8;
pub const PAN_STROKE_RAPID_HORZ: u32 = 7;
pub const PAN_STROKE_RAPID_VERT: u32 = 6;
pub const PAN_WEIGHT_BLACK: u32 = 10;
pub const PAN_WEIGHT_BOLD: u32 = 8;
pub const PAN_WEIGHT_BOOK: u32 = 5;
pub const PAN_WEIGHT_DEMI: u32 = 7;
pub const PAN_WEIGHT_HEAVY: u32 = 9;
pub const PAN_WEIGHT_INDEX: u32 = 2;
pub const PAN_WEIGHT_LIGHT: u32 = 3;
pub const PAN_WEIGHT_MEDIUM: u32 = 6;
pub const PAN_WEIGHT_NORD: u32 = 11;
pub const PAN_WEIGHT_THIN: u32 = 4;
pub const PAN_WEIGHT_VERY_LIGHT: u32 = 2;
pub const PAN_XHEIGHT_CONSTANT_LARGE: u32 = 4;
pub const PAN_XHEIGHT_CONSTANT_SMALL: u32 = 2;
pub const PAN_XHEIGHT_CONSTANT_STD: u32 = 3;
pub const PAN_XHEIGHT_DUCKING_LARGE: u32 = 7;
pub const PAN_XHEIGHT_DUCKING_SMALL: u32 = 5;
pub const PAN_XHEIGHT_DUCKING_STD: u32 = 6;
pub const PAN_XHEIGHT_INDEX: u32 = 9;
pub const PASSTHROUGH: u32 = 19;
pub const PATCOPY: u32 = 15728673;
pub const PATINVERT: u32 = 5898313;
pub const PATPAINT: u32 = 16452105;
#[cfg(feature = "windef")]
pub type PATTERN = LOGBRUSH;
pub type PAXESLIST = PAXESLISTA;
pub type PAXESLISTA = *mut AXESLISTA;
pub type PAXESLISTW = *mut AXESLISTW;
pub type PAXISINFO = PAXISINFOA;
pub type PAXISINFOA = *mut AXISINFOA;
pub type PAXISINFOW = *mut AXISINFOW;
pub type PBITMAP = *mut BITMAP;
pub type PBITMAPCOREHEADER = *mut BITMAPCOREHEADER;
pub type PBITMAPCOREINFO = *mut BITMAPCOREINFO;
pub type PBITMAPFILEHEADER = *mut BITMAPFILEHEADER;
pub type PBITMAPINFO = *mut BITMAPINFO;
pub type PBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
pub type PBITMAPV4HEADER = *mut BITMAPV4HEADER;
pub type PBITMAPV5HEADER = *mut BITMAPV5HEADER;
pub type PBLENDFUNCTION = *mut BLENDFUNCTION;
pub type PCHARSETINFO = *mut CHARSETINFO;
pub type PCOLORADJUSTMENT = *mut COLORADJUSTMENT;
pub const PC_EXPLICIT: u32 = 2;
pub const PC_INTERIORS: u32 = 128;
pub const PC_NOCOLLAPSE: u32 = 4;
pub const PC_NONE: u32 = 0;
pub const PC_PATHS: u32 = 512;
pub const PC_POLYGON: u32 = 1;
pub const PC_POLYPOLYGON: u32 = 256;
pub const PC_RECTANGLE: u32 = 2;
pub const PC_RESERVED: u32 = 1;
pub const PC_SCANLINE: u32 = 8;
pub const PC_STYLED: u32 = 32;
pub const PC_TRAPEZOID: u32 = 4;
pub const PC_WIDE: u32 = 16;
pub const PC_WIDESTYLED: u32 = 64;
pub const PC_WINDPOLYGON: u32 = 4;
pub type PDESIGNVECTOR = *mut DESIGNVECTOR;
pub const PDEVICESIZE: u32 = 26;
#[cfg(feature = "windef")]
pub type PDEVMODE = PDEVMODEA;
#[cfg(feature = "windef")]
pub type PDEVMODEA = *mut DEVMODEA;
#[cfg(feature = "windef")]
pub type PDEVMODEW = *mut DEVMODEW;
#[cfg(feature = "winnt")]
pub type PDIBSECTION = *mut DIBSECTION;
pub type PDISPLAY_DEVICE = PDISPLAY_DEVICEA;
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[cfg(feature = "windef")]
pub type PDRAWPATRECT = *mut DRAWPATRECT;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PELARRAY {
    pub paXCount: i32,
    pub paYCount: i32,
    pub paXExt: i32,
    pub paYExt: i32,
    pub paRGBs: u8,
}
pub type PEMR = *mut EMR;
pub type PEMRABORTPATH = *mut EMRABORTPATH;
#[cfg(feature = "windef")]
pub type PEMRALPHABLEND = *mut EMRALPHABLEND;
#[cfg(feature = "windef")]
pub type PEMRANGLEARC = *mut EMRANGLEARC;
#[cfg(feature = "windef")]
pub type PEMRARC = *mut EMRARC;
#[cfg(feature = "windef")]
pub type PEMRARCTO = *mut EMRARC;
pub type PEMRBEGINPATH = *mut EMRABORTPATH;
#[cfg(feature = "windef")]
pub type PEMRBITBLT = *mut EMRBITBLT;
#[cfg(feature = "windef")]
pub type PEMRCHORD = *mut EMRARC;
pub type PEMRCLOSEFIGURE = *mut EMRABORTPATH;
pub type PEMRCOLORCORRECTPALETTE = *mut EMRCOLORCORRECTPALETTE;
pub type PEMRCOLORMATCHTOTARGET = *mut EMRCOLORMATCHTOTARGET;
#[cfg(feature = "windef")]
pub type PEMRCREATEBRUSHINDIRECT = *mut EMRCREATEBRUSHINDIRECT;
pub type PEMRCREATECOLORSPACE = *mut EMRCREATECOLORSPACE;
pub type PEMRCREATECOLORSPACEW = *mut EMRCREATECOLORSPACEW;
pub type PEMRCREATEDIBPATTERNBRUSHPT = *mut EMRCREATEDIBPATTERNBRUSHPT;
pub type PEMRCREATEMONOBRUSH = *mut EMRCREATEMONOBRUSH;
pub type PEMRCREATEPALETTE = *mut EMRCREATEPALETTE;
#[cfg(feature = "windef")]
pub type PEMRCREATEPEN = *mut EMRCREATEPEN;
pub type PEMRDELETECOLORSPACE = *mut EMRSETCOLORSPACE;
pub type PEMRDELETEOBJECT = *mut EMRSELECTOBJECT;
pub type PEMRDRAWESCAPE = *mut EMREXTESCAPE;
#[cfg(feature = "windef")]
pub type PEMRELLIPSE = *mut EMRELLIPSE;
pub type PEMRENDPATH = *mut EMRABORTPATH;
pub type PEMREOF = *mut EMREOF;
#[cfg(feature = "windef")]
pub type PEMREXCLUDECLIPRECT = *mut EMREXCLUDECLIPRECT;
pub type PEMREXTCREATEFONTINDIRECTW = *mut EMREXTCREATEFONTINDIRECTW;
#[cfg(feature = "windef")]
pub type PEMREXTCREATEPEN = *mut EMREXTCREATEPEN;
pub type PEMREXTESCAPE = *mut EMREXTESCAPE;
#[cfg(feature = "windef")]
pub type PEMREXTFLOODFILL = *mut EMREXTFLOODFILL;
pub type PEMREXTSELECTCLIPRGN = *mut EMREXTSELECTCLIPRGN;
#[cfg(feature = "windef")]
pub type PEMREXTTEXTOUTA = *mut EMREXTTEXTOUTA;
#[cfg(feature = "windef")]
pub type PEMREXTTEXTOUTW = *mut EMREXTTEXTOUTA;
#[cfg(feature = "windef")]
pub type PEMRFILLPATH = *mut EMRFILLPATH;
#[cfg(feature = "windef")]
pub type PEMRFILLRGN = *mut EMRFILLRGN;
pub type PEMRFLATTENPATH = *mut EMRABORTPATH;
pub type PEMRFORMAT = *mut EMRFORMAT;
#[cfg(feature = "windef")]
pub type PEMRFRAMERGN = *mut EMRFRAMERGN;
pub type PEMRGDICOMMENT = *mut EMRGDICOMMENT;
#[cfg(feature = "windef")]
pub type PEMRGLSBOUNDEDRECORD = *mut EMRGLSBOUNDEDRECORD;
pub type PEMRGLSRECORD = *mut EMRGLSRECORD;
#[cfg(feature = "windef")]
pub type PEMRGRADIENTFILL = *mut EMRGRADIENTFILL;
#[cfg(feature = "windef")]
pub type PEMRINTERSECTCLIPRECT = *mut EMREXCLUDECLIPRECT;
#[cfg(feature = "windef")]
pub type PEMRINVERTRGN = *mut EMRINVERTRGN;
#[cfg(feature = "windef")]
pub type PEMRLINETO = *mut EMRLINETO;
#[cfg(feature = "windef")]
pub type PEMRMASKBLT = *mut EMRMASKBLT;
pub type PEMRMODIFYWORLDTRANSFORM = *mut EMRMODIFYWORLDTRANSFORM;
#[cfg(feature = "windef")]
pub type PEMRMOVETOEX = *mut EMRLINETO;
pub type PEMRNAMEDESCAPE = *mut EMRNAMEDESCAPE;
#[cfg(feature = "windef")]
pub type PEMROFFSETCLIPRGN = *mut EMROFFSETCLIPRGN;
#[cfg(feature = "windef")]
pub type PEMRPAINTRGN = *mut EMRINVERTRGN;
#[cfg(feature = "windef")]
pub type PEMRPIE = *mut EMRARC;
pub type PEMRPIXELFORMAT = *mut EMRPIXELFORMAT;
#[cfg(feature = "windef")]
pub type PEMRPLGBLT = *mut EMRPLGBLT;
#[cfg(feature = "windef")]
pub type PEMRPOLYBEZIER = *mut EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYBEZIER16 = *mut EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYBEZIERTO = *mut EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYBEZIERTO16 = *mut EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYDRAW = *mut EMRPOLYDRAW;
#[cfg(feature = "windef")]
pub type PEMRPOLYDRAW16 = *mut EMRPOLYDRAW16;
#[cfg(feature = "windef")]
pub type PEMRPOLYGON = *mut EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYGON16 = *mut EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYLINE = *mut EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYLINE16 = *mut EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYLINETO = *mut EMRPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYLINETO16 = *mut EMRPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYPOLYGON = *mut EMRPOLYPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYPOLYGON16 = *mut EMRPOLYPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYPOLYLINE = *mut EMRPOLYPOLYLINE;
#[cfg(feature = "windef")]
pub type PEMRPOLYPOLYLINE16 = *mut EMRPOLYPOLYLINE16;
#[cfg(feature = "windef")]
pub type PEMRPOLYTEXTOUTA = *mut EMRPOLYTEXTOUTA;
#[cfg(feature = "windef")]
pub type PEMRPOLYTEXTOUTW = *mut EMRPOLYTEXTOUTA;
pub type PEMRREALIZEPALETTE = *mut EMRABORTPATH;
#[cfg(feature = "windef")]
pub type PEMRRECTANGLE = *mut EMRELLIPSE;
pub type PEMRRESIZEPALETTE = *mut EMRRESIZEPALETTE;
pub type PEMRRESTOREDC = *mut EMRRESTOREDC;
#[cfg(feature = "windef")]
pub type PEMRROUNDRECT = *mut EMRROUNDRECT;
pub type PEMRSAVEDC = *mut EMRABORTPATH;
pub type PEMRSCALEVIEWPORTEXTEX = *mut EMRSCALEVIEWPORTEXTEX;
pub type PEMRSCALEWINDOWEXTEX = *mut EMRSCALEVIEWPORTEXTEX;
pub type PEMRSELECTCLIPPATH = *mut EMRSELECTCLIPPATH;
pub type PEMRSELECTCOLORSPACE = *mut EMRSETCOLORSPACE;
pub type PEMRSELECTOBJECT = *mut EMRSELECTOBJECT;
pub type PEMRSELECTPALETTE = *mut EMRSELECTPALETTE;
pub type PEMRSETARCDIRECTION = *mut EMRSETARCDIRECTION;
#[cfg(feature = "windef")]
pub type PEMRSETBKCOLOR = *mut EMRSETBKCOLOR;
pub type PEMRSETBKMODE = *mut EMRSELECTCLIPPATH;
#[cfg(feature = "windef")]
pub type PEMRSETBRUSHORGEX = *mut EMRSETVIEWPORTORGEX;
pub type PEMRSETCOLORADJUSTMENT = *mut EMRSETCOLORADJUSTMENT;
pub type PEMRSETCOLORSPACE = *mut EMRSETCOLORSPACE;
#[cfg(feature = "windef")]
pub type PEMRSETDIBITSTODEVICE = *mut EMRSETDIBITSTODEVICE;
pub type PEMRSETICMMODE = *mut EMRSELECTCLIPPATH;
pub type PEMRSETICMPROFILE = *mut EMRSETICMPROFILE;
pub type PEMRSETICMPROFILEA = *mut EMRSETICMPROFILE;
pub type PEMRSETICMPROFILEW = *mut EMRSETICMPROFILE;
pub type PEMRSETLAYOUT = *mut EMRSELECTCLIPPATH;
pub type PEMRSETMAPMODE = *mut EMRSELECTCLIPPATH;
pub type PEMRSETMAPPERFLAGS = *mut EMRSETMAPPERFLAGS;
pub type PEMRSETMETARGN = *mut EMRABORTPATH;
pub type PEMRSETMITERLIMIT = *mut EMRSETMITERLIMIT;
pub type PEMRSETPALETTEENTRIES = *mut EMRSETPALETTEENTRIES;
#[cfg(feature = "windef")]
pub type PEMRSETPIXELV = *mut EMRSETPIXELV;
pub type PEMRSETPOLYFILLMODE = *mut EMRSELECTCLIPPATH;
pub type PEMRSETROP2 = *mut EMRSELECTCLIPPATH;
pub type PEMRSETSTRETCHBLTMODE = *mut EMRSELECTCLIPPATH;
pub type PEMRSETTEXTALIGN = *mut EMRSELECTCLIPPATH;
#[cfg(feature = "windef")]
pub type PEMRSETTEXTCOLOR = *mut EMRSETBKCOLOR;
#[cfg(feature = "windef")]
pub type PEMRSETVIEWPORTEXTEX = *mut EMRSETVIEWPORTEXTEX;
#[cfg(feature = "windef")]
pub type PEMRSETVIEWPORTORGEX = *mut EMRSETVIEWPORTORGEX;
#[cfg(feature = "windef")]
pub type PEMRSETWINDOWEXTEX = *mut EMRSETVIEWPORTEXTEX;
#[cfg(feature = "windef")]
pub type PEMRSETWINDOWORGEX = *mut EMRSETVIEWPORTORGEX;
pub type PEMRSETWORLDTRANSFORM = *mut EMRSETWORLDTRANSFORM;
#[cfg(feature = "windef")]
pub type PEMRSTRETCHBLT = *mut EMRSTRETCHBLT;
#[cfg(feature = "windef")]
pub type PEMRSTRETCHDIBITS = *mut EMRSTRETCHDIBITS;
#[cfg(feature = "windef")]
pub type PEMRSTROKEANDFILLPATH = *mut EMRFILLPATH;
#[cfg(feature = "windef")]
pub type PEMRSTROKEPATH = *mut EMRFILLPATH;
#[cfg(feature = "windef")]
pub type PEMRTEXT = *mut EMRTEXT;
#[cfg(feature = "windef")]
pub type PEMRTRANSPARENTBLT = *mut EMRTRANSPARENTBLT;
pub type PEMRWIDENPATH = *mut EMRABORTPATH;
#[cfg(feature = "windef")]
pub type PENHMETAHEADER = *mut ENHMETAHEADER;
pub type PENHMETARECORD = *mut ENHMETARECORD;
pub type PENUMLOGFONTEXDV = PENUMLOGFONTEXDVA;
pub type PENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
pub type PENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
pub type PENUMTEXTMETRIC = PENUMTEXTMETRICA;
pub type PENUMTEXTMETRICA = *mut ENUMTEXTMETRICA;
pub type PENUMTEXTMETRICW = *mut ENUMTEXTMETRICW;
pub type PEXTLOGFONT = PEXTLOGFONTA;
pub type PEXTLOGFONTA = *mut EXTLOGFONTA;
pub type PEXTLOGFONTW = *mut EXTLOGFONTW;
#[cfg(feature = "windef")]
pub type PEXTLOGPEN = *mut EXTLOGPEN;
#[cfg(feature = "windef")]
pub type PEXTLOGPEN32 = *mut EXTLOGPEN32;
pub const PFD_DEPTH_DONTCARE: u32 = 536870912;
pub const PFD_DIRECT3D_ACCELERATED: u32 = 16384;
pub const PFD_DOUBLEBUFFER: u32 = 1;
pub const PFD_DOUBLEBUFFER_DONTCARE: u32 = 1073741824;
pub const PFD_DRAW_TO_BITMAP: u32 = 8;
pub const PFD_DRAW_TO_WINDOW: u32 = 4;
pub const PFD_GENERIC_ACCELERATED: u32 = 4096;
pub const PFD_GENERIC_FORMAT: u32 = 64;
pub const PFD_MAIN_PLANE: u32 = 0;
pub const PFD_NEED_PALETTE: u32 = 128;
pub const PFD_NEED_SYSTEM_PALETTE: u32 = 256;
pub const PFD_OVERLAY_PLANE: u32 = 1;
pub const PFD_STEREO: u32 = 2;
pub const PFD_STEREO_DONTCARE: u32 = 2147483648;
pub const PFD_SUPPORT_COMPOSITION: u32 = 32768;
pub const PFD_SUPPORT_DIRECTDRAW: u32 = 8192;
pub const PFD_SUPPORT_GDI: u32 = 16;
pub const PFD_SUPPORT_OPENGL: u32 = 32;
pub const PFD_SWAP_COPY: u32 = 1024;
pub const PFD_SWAP_EXCHANGE: u32 = 512;
pub const PFD_SWAP_LAYER_BUFFERS: u32 = 2048;
pub const PFD_TYPE_COLORINDEX: u32 = 1;
pub const PFD_TYPE_RGBA: u32 = 0;
pub const PFD_UNDERLAY_PLANE: i32 = -1;
pub type PFONTSIGNATURE = *mut FONTSIGNATURE;
pub type PGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
pub type PGLYPHSET = *mut GLYPHSET;
pub type PGRADIENT_RECT = *mut GRADIENT_RECT;
pub type PGRADIENT_TRIANGLE = *mut GRADIENT_TRIANGLE;
#[cfg(feature = "windef")]
pub type PHANDLETABLE = *mut HANDLETABLE;
pub const PHYSICALHEIGHT: u32 = 111;
pub const PHYSICALOFFSETX: u32 = 112;
pub const PHYSICALOFFSETY: u32 = 113;
pub const PHYSICALWIDTH: u32 = 110;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: u8,
    pub bReserved: u8,
    pub dwLayerMask: u32,
    pub dwVisibleMask: u32,
    pub dwDamageMask: u32,
}
pub const PLANES: u32 = 14;
#[cfg(feature = "windef")]
pub type PLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
pub type PLOCALESIGNATURE = *mut LOCALESIGNATURE;
#[cfg(feature = "windef")]
pub type PLOGBRUSH = *mut LOGBRUSH;
#[cfg(feature = "windef")]
pub type PLOGBRUSH32 = *mut LOGBRUSH32;
pub type PLOGFONT = PLOGFONTA;
pub type PLOGFONTA = *mut LOGFONTA;
pub type PLOGFONTW = *mut LOGFONTW;
pub type PLOGPALETTE = *mut LOGPALETTE;
#[cfg(feature = "windef")]
pub type PLOGPEN = *mut LOGPEN;
pub type PMETAHEADER = *mut METAHEADER;
pub type PMETARECORD = *mut METARECORD;
pub type PNEWTEXTMETRIC = PNEWTEXTMETRICA;
pub type PNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
pub type PNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTFLOAT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTFX {
    pub x: FIXED,
    pub y: FIXED,
}
pub const POLYFILL_LAST: u32 = 2;
pub const POLYGONALCAPS: u32 = 32;
#[cfg(feature = "windef")]
pub type POLYTEXT = POLYTEXTA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct POLYTEXTA {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: windows_sys::core::PCSTR,
    pub uiFlags: u32,
    pub rcl: super::RECT,
    pub pdx: *mut i32,
}
#[cfg(feature = "windef")]
impl Default for POLYTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct POLYTEXTW {
    pub x: i32,
    pub y: i32,
    pub n: u32,
    pub lpstr: windows_sys::core::PCWSTR,
    pub uiFlags: u32,
    pub rcl: super::RECT,
    pub pdx: *mut i32,
}
#[cfg(feature = "windef")]
impl Default for POLYTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POSTSCRIPT_DATA: u32 = 37;
pub const POSTSCRIPT_IDENTIFY: u32 = 4117;
pub const POSTSCRIPT_IGNORE: u32 = 38;
pub const POSTSCRIPT_INJECTION: u32 = 4118;
pub const POSTSCRIPT_PASSTHROUGH: u32 = 4115;
#[cfg(feature = "windef")]
pub type POUTLINETEXTMETRIC = POUTLINETEXTMETRICA;
#[cfg(feature = "windef")]
pub type POUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
#[cfg(feature = "windef")]
pub type POUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
pub type PPALETTEENTRY = *mut PALETTEENTRY;
#[cfg(feature = "windef")]
pub type PPATTERN = *mut PATTERN;
pub type PPELARRAY = *mut PELARRAY;
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type PPOINTFLOAT = *mut POINTFLOAT;
#[cfg(feature = "windef")]
pub type PPOLYTEXT = PPOLYTEXTA;
#[cfg(feature = "windef")]
pub type PPOLYTEXTA = *mut POLYTEXTA;
#[cfg(feature = "windef")]
pub type PPOLYTEXTW = *mut POLYTEXTW;
pub type PPSFEATURE_CUSTPAPER = *mut PSFEATURE_CUSTPAPER;
pub type PPSFEATURE_OUTPUT = *mut PSFEATURE_OUTPUT;
pub type PPSINJECTDATA = *mut PSINJECTDATA;
pub type PRGBTRIPLE = *mut RGBTRIPLE;
#[cfg(feature = "windef")]
pub type PRGNDATA = *mut RGNDATA;
#[cfg(feature = "windef")]
pub type PRGNDATAHEADER = *mut RGNDATAHEADER;
pub const PRINTRATEUNIT_CPS: u32 = 2;
pub const PRINTRATEUNIT_IPM: u32 = 4;
pub const PRINTRATEUNIT_LPM: u32 = 3;
pub const PRINTRATEUNIT_PPM: u32 = 1;
pub const PROFILE_EMBEDDED: u32 = 1296188740;
pub const PROFILE_LINKED: u32 = 1279872587;
pub const PROOF_QUALITY: u32 = 2;
pub const PR_JOBSTATUS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: windows_sys::core::BOOL,
    pub bSetPageDevice: windows_sys::core::BOOL,
}
pub const PSIDENT_GDICENTRIC: u32 = 0;
pub const PSIDENT_PSCENTRIC: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: u16,
    pub PageNumber: u16,
}
pub const PSINJECT_BEGINDEFAULTS: u32 = 12;
pub const PSINJECT_BEGINPAGESETUP: u32 = 101;
pub const PSINJECT_BEGINPROLOG: u32 = 14;
pub const PSINJECT_BEGINSETUP: u32 = 16;
pub const PSINJECT_BEGINSTREAM: u32 = 1;
pub const PSINJECT_BOUNDINGBOX: u32 = 9;
pub const PSINJECT_COMMENTS: u32 = 11;
pub const PSINJECT_DLFONT: u32 = 3722304989;
pub const PSINJECT_DOCNEEDEDRES: u32 = 5;
pub const PSINJECT_DOCSUPPLIEDRES: u32 = 6;
pub const PSINJECT_DOCUMENTPROCESSCOLORS: u32 = 10;
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: u32 = 21;
pub const PSINJECT_ENDDEFAULTS: u32 = 13;
pub const PSINJECT_ENDPAGECOMMENTS: u32 = 107;
pub const PSINJECT_ENDPAGESETUP: u32 = 102;
pub const PSINJECT_ENDPROLOG: u32 = 15;
pub const PSINJECT_ENDSETUP: u32 = 17;
pub const PSINJECT_ENDSTREAM: u32 = 20;
pub const PSINJECT_EOF: u32 = 19;
pub const PSINJECT_ORIENTATION: u32 = 8;
pub const PSINJECT_PAGEBBOX: u32 = 106;
pub const PSINJECT_PAGENUMBER: u32 = 100;
pub const PSINJECT_PAGEORDER: u32 = 7;
pub const PSINJECT_PAGES: u32 = 4;
pub const PSINJECT_PAGESATEND: u32 = 3;
pub const PSINJECT_PAGETRAILER: u32 = 103;
pub const PSINJECT_PLATECOLOR: u32 = 104;
pub const PSINJECT_PSADOBE: u32 = 2;
pub const PSINJECT_SHOWPAGE: u32 = 105;
pub const PSINJECT_TRAILER: u32 = 18;
pub const PSINJECT_VMRESTORE: u32 = 201;
pub const PSINJECT_VMSAVE: u32 = 200;
pub const PSPROTOCOL_ASCII: u32 = 0;
pub const PSPROTOCOL_BCP: u32 = 1;
pub const PSPROTOCOL_BINARY: u32 = 3;
pub const PSPROTOCOL_TBCP: u32 = 2;
pub const PS_ALTERNATE: u32 = 8;
pub const PS_COSMETIC: u32 = 0;
pub const PS_DASH: u32 = 1;
pub const PS_DASHDOT: u32 = 3;
pub const PS_DASHDOTDOT: u32 = 4;
pub const PS_DOT: u32 = 2;
pub const PS_ENDCAP_FLAT: u32 = 512;
pub const PS_ENDCAP_MASK: u32 = 3840;
pub const PS_ENDCAP_ROUND: u32 = 0;
pub const PS_ENDCAP_SQUARE: u32 = 256;
pub const PS_GEOMETRIC: u32 = 65536;
pub const PS_INSIDEFRAME: u32 = 6;
pub const PS_JOIN_BEVEL: u32 = 4096;
pub const PS_JOIN_MASK: u32 = 61440;
pub const PS_JOIN_MITER: u32 = 8192;
pub const PS_JOIN_ROUND: u32 = 0;
pub const PS_NULL: u32 = 5;
pub const PS_SOLID: u32 = 0;
pub const PS_STYLE_MASK: u32 = 15;
pub const PS_TYPE_MASK: u32 = 983040;
pub const PS_USERSTYLE: u32 = 7;
pub type PTEXTMETRIC = PTEXTMETRICA;
pub type PTEXTMETRICA = *mut TEXTMETRICA;
pub type PTEXTMETRICW = *mut TEXTMETRICW;
pub type PTRIVERTEX = *mut TRIVERTEX;
pub const PT_BEZIERTO: u32 = 4;
pub const PT_CLOSEFIGURE: u32 = 1;
pub const PT_LINETO: u32 = 2;
pub const PT_MOVETO: u32 = 6;
pub type PWCRANGE = *mut WCRANGE;
#[cfg(feature = "windef")]
pub type PWGLSWAP = *mut WGLSWAP;
pub type PXFORM = *mut XFORM;
pub const QDC_ALL_PATHS: u32 = 1;
pub const QDC_DATABASE_CURRENT: u32 = 4;
pub const QDC_INCLUDE_HMD: u32 = 32;
pub const QDC_ONLY_ACTIVE_PATHS: u32 = 2;
pub const QDC_VIRTUAL_MODE_AWARE: u32 = 16;
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 64;
pub const QDI_DIBTOSCREEN: u32 = 4;
pub const QDI_GETDIBITS: u32 = 2;
pub const QDI_SETDIBITS: u32 = 1;
pub const QDI_STRETCHDIB: u32 = 8;
pub const QUERYDIBSUPPORT: u32 = 3073;
pub const QUERYESCSUPPORT: u32 = 8;
pub const R2_BLACK: u32 = 1;
pub const R2_COPYPEN: u32 = 13;
pub const R2_LAST: u32 = 16;
pub const R2_MASKNOTPEN: u32 = 3;
pub const R2_MASKPEN: u32 = 9;
pub const R2_MASKPENNOT: u32 = 5;
pub const R2_MERGENOTPEN: u32 = 12;
pub const R2_MERGEPEN: u32 = 15;
pub const R2_MERGEPENNOT: u32 = 14;
pub const R2_NOP: u32 = 11;
pub const R2_NOT: u32 = 6;
pub const R2_NOTCOPYPEN: u32 = 4;
pub const R2_NOTMASKPEN: u32 = 8;
pub const R2_NOTMERGEPEN: u32 = 2;
pub const R2_NOTXORPEN: u32 = 10;
pub const R2_WHITE: u32 = 16;
pub const R2_XORPEN: u32 = 7;
pub const RASTERCAPS: u32 = 38;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RASTERIZER_STATUS {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
pub const RASTER_FONTTYPE: u32 = 1;
pub const RC_BANDING: u32 = 2;
pub const RC_BIGFONT: u32 = 1024;
pub const RC_BITBLT: u32 = 1;
pub const RC_BITMAP64: u32 = 8;
pub const RC_DEVBITS: u32 = 32768;
pub const RC_DIBTODEV: u32 = 512;
pub const RC_DI_BITMAP: u32 = 128;
pub const RC_FLOODFILL: u32 = 4096;
pub const RC_GDI20_OUTPUT: u32 = 16;
pub const RC_GDI20_STATE: u32 = 32;
pub const RC_OP_DX_OUTPUT: u32 = 16384;
pub const RC_PALETTE: u32 = 256;
pub const RC_SAVEBITMAP: u32 = 64;
pub const RC_SCALING: u32 = 4;
pub const RC_STRETCHBLT: u32 = 2048;
pub const RC_STRETCHDIB: u32 = 8192;
pub const RDH_RECTANGLES: u32 = 1;
pub const REFERENCE_BLACK_MAX: u16 = 4000;
pub const REFERENCE_BLACK_MIN: u16 = 0;
pub const REFERENCE_WHITE_MAX: u16 = 10000;
pub const REFERENCE_WHITE_MIN: u16 = 6000;
pub const RELATIVE: u32 = 2;
pub const RESTORE_CTM: u32 = 4100;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RGBQUAD {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbReserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RGBTRIPLE {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}
pub const RGB_GAMMA_MAX: u16 = 65000;
pub const RGB_GAMMA_MIN: u16 = 1344;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [i8; 1],
}
#[cfg(feature = "windef")]
impl Default for RGNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct RGNDATAHEADER {
    pub dwSize: u32,
    pub iType: u32,
    pub nCount: u32,
    pub nRgnSize: u32,
    pub rcBound: super::RECT,
}
pub const RGN_AND: u32 = 1;
pub const RGN_COPY: u32 = 5;
pub const RGN_DIFF: u32 = 4;
pub const RGN_ERROR: u32 = 0;
pub const RGN_MAX: u32 = 5;
pub const RGN_MIN: u32 = 1;
pub const RGN_OR: u32 = 2;
pub const RGN_XOR: u32 = 3;
pub const RUSSIAN_CHARSET: u32 = 204;
pub const SAVE_CTM: u32 = 4101;
pub const SB_CONST_ALPHA: u32 = 1;
pub const SB_GRAD_RECT: u32 = 16;
pub const SB_GRAD_TRI: u32 = 32;
pub const SB_NONE: u32 = 0;
pub const SB_PIXEL_ALPHA: u32 = 2;
pub const SB_PREMULT_ALPHA: u32 = 4;
pub const SCALINGFACTORX: u32 = 114;
pub const SCALINGFACTORY: u32 = 115;
pub const SDC_ALLOW_CHANGES: u32 = 1024;
pub const SDC_ALLOW_PATH_ORDER_CHANGES: u32 = 8192;
pub const SDC_APPLY: u32 = 128;
pub const SDC_FORCE_MODE_ENUMERATION: u32 = 4096;
pub const SDC_NO_OPTIMIZATION: u32 = 256;
pub const SDC_PATH_PERSIST_IF_REQUIRED: u32 = 2048;
pub const SDC_SAVE_TO_DATABASE: u32 = 512;
pub const SDC_TOPOLOGY_CLONE: u32 = 2;
pub const SDC_TOPOLOGY_EXTEND: u32 = 4;
pub const SDC_TOPOLOGY_EXTERNAL: u32 = 8;
pub const SDC_TOPOLOGY_INTERNAL: u32 = 1;
pub const SDC_TOPOLOGY_SUPPLIED: u32 = 16;
pub const SDC_USE_DATABASE_CURRENT: u32 = 15;
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: u32 = 32;
pub const SDC_VALIDATE: u32 = 64;
pub const SDC_VIRTUAL_MODE_AWARE: u32 = 32768;
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: u32 = 131072;
pub const SELECTPAPERSOURCE: u32 = 18;
pub const SETABORTPROC: u32 = 9;
pub const SETALLJUSTVALUES: u32 = 771;
pub const SETCHARSET: u32 = 772;
pub const SETCOLORTABLE: u32 = 4;
pub const SETCOPYCOUNT: u32 = 17;
pub const SETDIBSCALING: u32 = 32;
pub const SETICMPROFILE_EMBEDED: u32 = 1;
pub const SETKERNTRACK: u32 = 770;
pub const SETLINECAP: u32 = 21;
pub const SETLINEJOIN: u32 = 22;
pub const SETMITERLIMIT: u32 = 23;
pub const SET_ARC_DIRECTION: u32 = 4102;
pub const SET_BACKGROUND_COLOR: u32 = 4103;
pub const SET_BOUNDS: u32 = 4109;
pub const SET_CLIP_BOX: u32 = 4108;
pub const SET_MIRROR_MODE: u32 = 4110;
pub const SET_POLY_MODE: u32 = 4104;
pub const SET_SCREEN_ANGLE: u32 = 4105;
pub const SET_SPREAD: u32 = 4106;
pub const SHADEBLENDCAPS: u32 = 120;
pub const SHIFTJIS_CHARSET: u32 = 128;
pub const SIMPLEREGION: u32 = 2;
pub const SIZEPALETTE: u32 = 104;
pub const SPCLPASSTHROUGH2: u32 = 4568;
pub const SP_APPABORT: i32 = -2;
pub const SP_ERROR: i32 = -1;
pub const SP_NOTREPORTED: u32 = 16384;
pub const SP_OUTOFDISK: i32 = -4;
pub const SP_OUTOFMEMORY: i32 = -5;
pub const SP_USERABORT: i32 = -3;
pub const SRCAND: u32 = 8913094;
pub const SRCCOPY: u32 = 13369376;
pub const SRCERASE: u32 = 4457256;
pub const SRCINVERT: u32 = 6684742;
pub const SRCPAINT: u32 = 15597702;
pub const STAMP_AXESLIST: u32 = 134245473;
pub const STAMP_CFF2: u32 = 134248035;
pub const STAMP_DESIGNVECTOR: u32 = 134248036;
pub const STAMP_TRUETYPE_VARIATION: u32 = 134248052;
pub const STARTDOC: u32 = 10;
pub const STOCK_LAST: u32 = 19;
pub const STRETCHBLT: u32 = 2048;
pub const STRETCH_ANDSCANS: u32 = 1;
pub const STRETCH_DELETESCANS: u32 = 3;
pub const STRETCH_HALFTONE: u32 = 4;
pub const STRETCH_ORSCANS: u32 = 2;
pub const SYMBOL_CHARSET: u32 = 2;
pub const SYSPAL_ERROR: u32 = 0;
pub const SYSPAL_NOSTATIC: u32 = 2;
pub const SYSPAL_NOSTATIC256: u32 = 3;
pub const SYSPAL_STATIC: u32 = 1;
pub const SYSRGN: u32 = 4;
pub const SYSTEM_FIXED_FONT: u32 = 16;
pub const SYSTEM_FONT: u32 = 13;
pub const TA_BASELINE: u32 = 24;
pub const TA_BOTTOM: u32 = 8;
pub const TA_CENTER: u32 = 6;
pub const TA_LEFT: u32 = 0;
pub const TA_MASK: u32 = 287;
pub const TA_NOUPDATECP: u32 = 0;
pub const TA_RIGHT: u32 = 2;
pub const TA_RTLREADING: u32 = 256;
pub const TA_TOP: u32 = 0;
pub const TA_UPDATECP: u32 = 1;
pub const TCI_SRCCHARSET: u32 = 1;
pub const TCI_SRCCODEPAGE: u32 = 2;
pub const TCI_SRCFONTSIG: u32 = 3;
pub const TCI_SRCLOCALE: u32 = 4096;
pub const TC_CP_STROKE: u32 = 4;
pub const TC_CR_90: u32 = 8;
pub const TC_CR_ANY: u32 = 16;
pub const TC_EA_DOUBLE: u32 = 512;
pub const TC_IA_ABLE: u32 = 1024;
pub const TC_OP_CHARACTER: u32 = 1;
pub const TC_OP_STROKE: u32 = 2;
pub const TC_RA_ABLE: u32 = 8192;
pub const TC_RESERVED: u32 = 32768;
pub const TC_SA_CONTIN: u32 = 256;
pub const TC_SA_DOUBLE: u32 = 64;
pub const TC_SA_INTEGER: u32 = 128;
pub const TC_SCROLLBLT: u32 = 65536;
pub const TC_SF_X_YINDEP: u32 = 32;
pub const TC_SO_ABLE: u32 = 4096;
pub const TC_UA_ABLE: u32 = 2048;
pub const TC_VA_ABLE: u32 = 16384;
pub const TECHNOLOGY: u32 = 2;
pub const TEXTCAPS: u32 = 34;
pub type TEXTMETRIC = TEXTMETRICA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const THAI_CHARSET: u32 = 222;
pub const TMPF_DEVICE: u32 = 8;
pub const TMPF_FIXED_PITCH: u32 = 1;
pub const TMPF_TRUETYPE: u32 = 4;
pub const TMPF_VECTOR: u32 = 2;
pub const TRANSFORM_CTM: u32 = 4107;
pub const TRANSPARENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRIVERTEX {
    pub x: i32,
    pub y: i32,
    pub Red: COLOR16,
    pub Green: COLOR16,
    pub Blue: COLOR16,
    pub Alpha: COLOR16,
}
pub const TRUETYPE_FONTTYPE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TTPOLYCURVE {
    pub wType: u16,
    pub cpfx: u16,
    pub apfx: [POINTFX; 1],
}
impl Default for TTPOLYCURVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TTPOLYGONHEADER {
    pub cb: u32,
    pub dwType: u32,
    pub pfxStart: POINTFX,
}
pub const TT_AVAILABLE: u32 = 1;
pub const TT_ENABLED: u32 = 2;
pub const TT_POLYGON_TYPE: u32 = 24;
pub const TT_PRIM_CSPLINE: u32 = 3;
pub const TT_PRIM_LINE: u32 = 1;
pub const TT_PRIM_QSPLINE: u32 = 2;
pub const TURKISH_CHARSET: u32 = 162;
pub const VARIABLE_PITCH: u32 = 2;
pub const VERTRES: u32 = 10;
pub const VERTSIZE: u32 = 6;
pub const VIETNAMESE_CHARSET: u32 = 163;
pub const VREFRESH: u32 = 116;
pub const VTA_BASELINE: u32 = 24;
pub const VTA_BOTTOM: u32 = 2;
pub const VTA_CENTER: u32 = 6;
pub const VTA_LEFT: u32 = 8;
pub const VTA_RIGHT: u32 = 0;
pub const VTA_TOP: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WCRANGE {
    pub wcLow: u16,
    pub cGlyphs: u16,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct WGLSWAP {
    pub hdc: super::HDC,
    pub uiFlags: u32,
}
#[cfg(feature = "windef")]
impl Default for WGLSWAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WGL_FONT_LINES: u32 = 0;
pub const WGL_FONT_POLYGONS: u32 = 1;
pub const WGL_SWAPMULTIPLE_MAX: u32 = 16;
pub const WGL_SWAP_MAIN_PLANE: u32 = 1;
pub const WGL_SWAP_OVERLAY1: u32 = 2;
pub const WGL_SWAP_OVERLAY10: u32 = 1024;
pub const WGL_SWAP_OVERLAY11: u32 = 2048;
pub const WGL_SWAP_OVERLAY12: u32 = 4096;
pub const WGL_SWAP_OVERLAY13: u32 = 8192;
pub const WGL_SWAP_OVERLAY14: u32 = 16384;
pub const WGL_SWAP_OVERLAY15: u32 = 32768;
pub const WGL_SWAP_OVERLAY2: u32 = 4;
pub const WGL_SWAP_OVERLAY3: u32 = 8;
pub const WGL_SWAP_OVERLAY4: u32 = 16;
pub const WGL_SWAP_OVERLAY5: u32 = 32;
pub const WGL_SWAP_OVERLAY6: u32 = 64;
pub const WGL_SWAP_OVERLAY7: u32 = 128;
pub const WGL_SWAP_OVERLAY8: u32 = 256;
pub const WGL_SWAP_OVERLAY9: u32 = 512;
pub const WGL_SWAP_UNDERLAY1: u32 = 65536;
pub const WGL_SWAP_UNDERLAY10: u32 = 33554432;
pub const WGL_SWAP_UNDERLAY11: u32 = 67108864;
pub const WGL_SWAP_UNDERLAY12: u32 = 134217728;
pub const WGL_SWAP_UNDERLAY13: u32 = 268435456;
pub const WGL_SWAP_UNDERLAY14: u32 = 536870912;
pub const WGL_SWAP_UNDERLAY15: u32 = 1073741824;
pub const WGL_SWAP_UNDERLAY2: u32 = 131072;
pub const WGL_SWAP_UNDERLAY3: u32 = 262144;
pub const WGL_SWAP_UNDERLAY4: u32 = 524288;
pub const WGL_SWAP_UNDERLAY5: u32 = 1048576;
pub const WGL_SWAP_UNDERLAY6: u32 = 2097152;
pub const WGL_SWAP_UNDERLAY7: u32 = 4194304;
pub const WGL_SWAP_UNDERLAY8: u32 = 8388608;
pub const WGL_SWAP_UNDERLAY9: u32 = 16777216;
pub const WHITENESS: u32 = 16711778;
pub const WHITEONBLACK: u32 = 2;
pub const WHITE_BRUSH: u32 = 0;
pub const WHITE_PEN: u32 = 6;
pub const WINDING: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
