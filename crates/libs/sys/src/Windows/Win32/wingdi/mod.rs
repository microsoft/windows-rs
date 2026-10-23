#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn AbortDoc(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn AbortPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn AddFontMemResourceEx(pfileview : super::PVOID, cjsize : super::DWORD, pvresrved : super::PVOID, pnumfonts : *const super::DWORD) -> super::HANDLE);
windows_link::link!("gdi32.dll" "system" fn AddFontResourceA(param0 : windows_sys::core::PCSTR) -> i32);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn AddFontResourceExA(name : windows_sys::core::PCSTR, fl : super::DWORD, res : super::PVOID) -> i32);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn AddFontResourceExW(name : windows_sys::core::PCWSTR, fl : super::DWORD, res : super::PVOID) -> i32);
windows_link::link!("gdi32.dll" "system" fn AddFontResourceW(param0 : windows_sys::core::PCWSTR) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("msimg32.dll" "system" fn AlphaBlend(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, ftn : BLENDFUNCTION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn AngleArc(hdc : super::HDC, x : i32, y : i32, r : super::DWORD, startangle : super::FLOAT, sweepangle : super::FLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn AnimatePalette(hpal : super::HPALETTE, istartindex : super::UINT, centries : super::UINT, ppe : *const PALETTEENTRY) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Arc(hdc : super::HDC, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ArcTo(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32, xr1 : i32, yr1 : i32, xr2 : i32, yr2 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn BeginPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn BitBlt(hdc : super::HDC, x : i32, y : i32, cx : i32, cy : i32, hdcsrc : super::HDC, x1 : i32, y1 : i32, rop : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CancelDC(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CheckColorsInGamut(hdc : super::HDC, lprgbtriple : LPRGBTRIPLE, dlpbuffer : super::LPVOID, ncount : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ChoosePixelFormat(hdc : super::HDC, ppfd : *const PIXELFORMATDESCRIPTOR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Chord(hdc : super::HDC, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CloseEnhMetaFile(hdc : super::HDC) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CloseFigure(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CloseMetaFile(hdc : super::HDC) -> super::HMETAFILE);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ColorCorrectPalette(hdc : super::HDC, hpal : super::HPALETTE, defirst : super::DWORD, num : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ColorMatchToTarget(hdc : super::HDC, hdctarget : super::HDC, action : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CombineRgn(hrgndst : super::HRGN, hrgnsrc1 : super::HRGN, hrgnsrc2 : super::HRGN, imode : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CombineTransform(lpxfout : LPXFORM, lpxf1 : *const XFORM, lpxf2 : *const XFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CopyEnhMetaFileA(henh : super::HENHMETAFILE, lpfilename : windows_sys::core::PCSTR) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CopyEnhMetaFileW(henh : super::HENHMETAFILE, lpfilename : windows_sys::core::PCWSTR) -> super::HENHMETAFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CopyMetaFileA(param0 : super::HMETAFILE, param1 : windows_sys::core::PCSTR) -> super::HMETAFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CopyMetaFileW(param0 : super::HMETAFILE, param1 : windows_sys::core::PCWSTR) -> super::HMETAFILE);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateBitmap(nwidth : i32, nheight : i32, nplanes : super::UINT, nbitcount : super::UINT, lpbits : *const core::ffi::c_void) -> super::HBITMAP);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateBitmapIndirect(pbm : *const BITMAP) -> super::HBITMAP);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateBrushIndirect(plbrush : *const LOGBRUSH) -> super::HBRUSH);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateColorSpaceA(lplcs : LPLOGCOLORSPACEA) -> super::HCOLORSPACE);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateColorSpaceW(lplcs : LPLOGCOLORSPACEW) -> super::HCOLORSPACE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleBitmap(hdc : super::HDC, cx : i32, cy : i32) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateCompatibleDC(hdc : super::HDC) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateDCA(pwszdriver : windows_sys::core::PCSTR, pwszdevice : windows_sys::core::PCSTR, pszport : windows_sys::core::PCSTR, pdm : *const DEVMODEA) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateDCW(pwszdriver : windows_sys::core::PCWSTR, pwszdevice : windows_sys::core::PCWSTR, pszport : windows_sys::core::PCWSTR, pdm : *const DEVMODEW) -> super::HDC);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateDIBPatternBrush(h : super::HGLOBAL, iusage : super::UINT) -> super::HBRUSH);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateDIBPatternBrushPt(lppackeddib : *const core::ffi::c_void, iusage : super::UINT) -> super::HBRUSH);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateDIBSection(hdc : super::HDC, pbmi : *const BITMAPINFO, usage : super::UINT, ppvbits : *mut *mut core::ffi::c_void, hsection : super::HANDLE, offset : super::DWORD) -> super::HBITMAP);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateDIBitmap(hdc : super::HDC, pbmih : *const BITMAPINFOHEADER, flinit : super::DWORD, pjbits : *const core::ffi::c_void, pbmi : *const BITMAPINFO, iusage : super::UINT) -> super::HBITMAP);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateDiscardableBitmap(hdc : super::HDC, cx : i32, cy : i32) -> super::HBITMAP);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CreateEllipticRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> super::HRGN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateEllipticRgnIndirect(lprect : *const super::RECT) -> super::HRGN);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateEnhMetaFileA(hdc : super::HDC, lpfilename : windows_sys::core::PCSTR, lprc : *const super::RECT, lpdesc : windows_sys::core::PCSTR) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateEnhMetaFileW(hdc : super::HDC, lpfilename : windows_sys::core::PCWSTR, lprc : *const super::RECT, lpdesc : windows_sys::core::PCWSTR) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateFontA(cheight : i32, cwidth : i32, cescapement : i32, corientation : i32, cweight : i32, bitalic : super::DWORD, bunderline : super::DWORD, bstrikeout : super::DWORD, icharset : super::DWORD, ioutprecision : super::DWORD, iclipprecision : super::DWORD, iquality : super::DWORD, ipitchandfamily : super::DWORD, pszfacename : windows_sys::core::PCSTR) -> super::HFONT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectA(lplf : *const LOGFONTA) -> super::HFONT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectExA(param0 : *const ENUMLOGFONTEXDVA) -> super::HFONT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectExW(param0 : *const ENUMLOGFONTEXDVW) -> super::HFONT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateFontIndirectW(lplf : *const LOGFONTW) -> super::HFONT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateFontW(cheight : i32, cwidth : i32, cescapement : i32, corientation : i32, cweight : i32, bitalic : super::DWORD, bunderline : super::DWORD, bstrikeout : super::DWORD, icharset : super::DWORD, ioutprecision : super::DWORD, iclipprecision : super::DWORD, iquality : super::DWORD, ipitchandfamily : super::DWORD, pszfacename : windows_sys::core::PCWSTR) -> super::HFONT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateHalftonePalette(hdc : super::HDC) -> super::HPALETTE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateHatchBrush(ihatch : i32, color : super::COLORREF) -> super::HBRUSH);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateICA(pszdriver : windows_sys::core::PCSTR, pszdevice : windows_sys::core::PCSTR, pszport : windows_sys::core::PCSTR, pdm : *const DEVMODEA) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn CreateICW(pszdriver : windows_sys::core::PCWSTR, pszdevice : windows_sys::core::PCWSTR, pszport : windows_sys::core::PCWSTR, pdm : *const DEVMODEW) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateMetaFileA(pszfile : windows_sys::core::PCSTR) -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateMetaFileW(pszfile : windows_sys::core::PCWSTR) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreatePalette(plpal : *const LOGPALETTE) -> super::HPALETTE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreatePatternBrush(hbm : super::HBITMAP) -> super::HBRUSH);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreatePen(istyle : i32, cwidth : i32, color : super::COLORREF) -> super::HPEN);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreatePenIndirect(plpen : *const LOGPEN) -> super::HPEN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreatePolyPolygonRgn(pptl : *const super::POINT, pc : *const super::INT, cpoly : i32, imode : i32) -> super::HRGN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreatePolygonRgn(pptl : *const super::POINT, cpoint : i32, imode : i32) -> super::HRGN);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CreateRectRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> super::HRGN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn CreateRectRgnIndirect(lprect : *const super::RECT) -> super::HRGN);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn CreateRoundRectRgn(x1 : i32, y1 : i32, x2 : i32, y2 : i32, w : i32, h : i32) -> super::HRGN);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn CreateScalableFontResourceA(fdwhidden : super::DWORD, lpszfont : windows_sys::core::PCSTR, lpszfile : windows_sys::core::PCSTR, lpszpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn CreateScalableFontResourceW(fdwhidden : super::DWORD, lpszfont : windows_sys::core::PCWSTR, lpszfile : windows_sys::core::PCWSTR, lpszpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn CreateSolidBrush(color : super::COLORREF) -> super::HBRUSH);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn DPtoLP(hdc : super::HDC, lppt : super::LPPOINT, c : i32) -> windows_sys::core::BOOL);
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
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn DescribePixelFormat(hdc : super::HDC, ipixelformat : i32, nbytes : super::UINT, ppfd : LPPIXELFORMATDESCRIPTOR) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("winspool.drv" "system" fn DeviceCapabilitiesA(pdevice : windows_sys::core::PCSTR, pport : windows_sys::core::PCSTR, fwcapability : super::WORD, poutput : windows_sys::core::PSTR, pdevmode : *const DEVMODEA) -> i32);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("winspool.drv" "system" fn DeviceCapabilitiesW(pdevice : windows_sys::core::PCWSTR, pport : windows_sys::core::PCWSTR, fwcapability : super::WORD, poutput : windows_sys::core::PWSTR, pdevmode : *const DEVMODEW) -> i32);
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumEnhMetaFile(hdc : super::HDC, hmf : super::HENHMETAFILE, proc : ENHMFENUMPROC, param : super::LPVOID, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesA(hdc : super::HDC, lplogfont : windows_sys::core::PCSTR, lpproc : FONTENUMPROCA, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesExA(hdc : super::HDC, lplogfont : LPLOGFONTA, lpproc : FONTENUMPROCA, lparam : super::LPARAM, dwflags : super::DWORD) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesExW(hdc : super::HDC, lplogfont : LPLOGFONTW, lpproc : FONTENUMPROCW, lparam : super::LPARAM, dwflags : super::DWORD) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontFamiliesW(hdc : super::HDC, lplogfont : windows_sys::core::PCWSTR, lpproc : FONTENUMPROCW, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontsA(hdc : super::HDC, lplogfont : windows_sys::core::PCSTR, lpproc : FONTENUMPROCA, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn EnumFontsW(hdc : super::HDC, lplogfont : windows_sys::core::PCWSTR, lpproc : FONTENUMPROCW, lparam : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumICMProfilesA(hdc : super::HDC, proc : ICMENUMPROCA, param : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumICMProfilesW(hdc : super::HDC, proc : ICMENUMPROCW, param : super::LPARAM) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumMetaFile(hdc : super::HDC, hmf : super::HMETAFILE, proc : MFENUMPROC, param : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn EnumObjects(hdc : super::HDC, ntype : i32, lpfunc : GOBJENUMPROC, lparam : super::LPARAM) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn EqualRgn(hrgn1 : super::HRGN, hrgn2 : super::HRGN) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn Escape(hdc : super::HDC, iescape : i32, cjin : i32, pvin : windows_sys::core::PCSTR, pvout : super::LPVOID) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExcludeClipRect(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtCreatePen(ipenstyle : super::DWORD, cwidth : super::DWORD, plbrush : *const LOGBRUSH, cstyle : super::DWORD, pstyle : *const super::DWORD) -> super::HPEN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtCreateRegion(lpx : *const XFORM, ncount : super::DWORD, lpdata : *const RGNDATA) -> super::HRGN);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn ExtEscape(hdc : super::HDC, iescape : i32, cjinput : i32, lpindata : windows_sys::core::PCSTR, cjoutput : i32, lpoutdata : windows_sys::core::PSTR) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtFloodFill(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF, r#type : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtSelectClipRgn(hdc : super::HDC, hrgn : super::HRGN, mode : i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtTextOutA(hdc : super::HDC, x : i32, y : i32, options : super::UINT, lprect : *const super::RECT, lpstring : windows_sys::core::PCSTR, c : super::UINT, lpdx : *const super::INT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ExtTextOutW(hdc : super::HDC, x : i32, y : i32, options : super::UINT, lprect : *const super::RECT, lpstring : windows_sys::core::PCWSTR, c : super::UINT, lpdx : *const super::INT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FillPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn FillRgn(hdc : super::HDC, hrgn : super::HRGN, hbr : super::HBRUSH) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn FixBrushOrgEx(hdc : super::HDC, x : i32, y : i32, ptl : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FlattenPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn FloodFill(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn FrameRgn(hdc : super::HDC, hrgn : super::HRGN, hbr : super::HBRUSH, w : i32, h : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GdiAlphaBlend(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, ftn : BLENDFUNCTION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GdiComment(hdc : super::HDC, nsize : super::UINT, lpdata : *const super::BYTE) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn GdiFlush() -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn GdiGetBatchLimit() -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GdiGradientFill(hdc : super::HDC, pvertex : PTRIVERTEX, nvertex : super::ULONG, pmesh : super::PVOID, ncount : super::ULONG, ulmode : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn GdiSetBatchLimit(dw : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GdiTransparentBlt(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, crtransparent : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetArcDirection(hdc : super::HDC) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetAspectRatioFilterEx(hdc : super::HDC, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetBitmapBits(hbit : super::HBITMAP, cb : super::LONG, lpvbits : super::LPVOID) -> super::LONG);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetBitmapDimensionEx(hbit : super::HBITMAP, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBkColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetBkMode(hdc : super::HDC) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetBoundsRect(hdc : super::HDC, lprect : super::LPRECT, flags : super::UINT) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetBrushOrgEx(hdc : super::HDC, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsA(hdc : super::HDC, wfirst : super::UINT, wlast : super::UINT, lpabc : LPABC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsFloatA(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpabc : LPABCFLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsFloatW(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpabc : LPABCFLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsI(hdc : super::HDC, gifirst : super::UINT, cgi : super::UINT, pgi : super::LPWORD, pabc : LPABC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharABCWidthsW(hdc : super::HDC, wfirst : super::UINT, wlast : super::UINT, lpabc : LPABC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidth32A(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpbuffer : super::LPINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidth32W(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpbuffer : super::LPINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthA(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpbuffer : super::LPINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthFloatA(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpbuffer : super::PFLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthFloatW(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpbuffer : super::PFLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthI(hdc : super::HDC, gifirst : super::UINT, cgi : super::UINT, pgi : super::LPWORD, piwidths : super::LPINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharWidthW(hdc : super::HDC, ifirst : super::UINT, ilast : super::UINT, lpbuffer : super::LPINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharacterPlacementA(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, ncount : i32, nmexextent : i32, lpresults : LPGCP_RESULTSA, dwflags : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCharacterPlacementW(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, ncount : i32, nmexextent : i32, lpresults : LPGCP_RESULTSW, dwflags : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetClipBox(hdc : super::HDC, lprect : super::LPRECT) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetClipRgn(hdc : super::HDC, hrgn : super::HRGN) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetColorAdjustment(hdc : super::HDC, lpca : LPCOLORADJUSTMENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetColorSpace(hdc : super::HDC) -> super::HCOLORSPACE);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCurrentObject(hdc : super::HDC, r#type : super::UINT) -> super::HGDIOBJ);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetCurrentPositionEx(hdc : super::HDC, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDCBrushColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetDCOrgEx(hdc : super::HDC, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDCPenColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetDIBColorTable(hdc : super::HDC, istart : super::UINT, centries : super::UINT, prgbq : *mut RGBQUAD) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetDIBits(hdc : super::HDC, hbm : super::HBITMAP, start : super::UINT, clines : super::UINT, lpvbits : super::LPVOID, lpbmi : LPBITMAPINFO, usage : super::UINT) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetDeviceCaps(hdc : super::HDC, index : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetDeviceGammaRamp(hdc : super::HDC, lpramp : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileA(lpname : windows_sys::core::PCSTR) -> super::HENHMETAFILE);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileBits(hemf : super::HENHMETAFILE, nsize : super::UINT, lpdata : super::LPBYTE) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileDescriptionA(hemf : super::HENHMETAFILE, cchbuffer : super::UINT, lpdescription : windows_sys::core::PSTR) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileDescriptionW(hemf : super::HENHMETAFILE, cchbuffer : super::UINT, lpdescription : windows_sys::core::PWSTR) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileHeader(hemf : super::HENHMETAFILE, nsize : super::UINT, lpenhmetaheader : LPENHMETAHEADER) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFilePaletteEntries(hemf : super::HENHMETAFILE, nnumentries : super::UINT, lppaletteentries : LPPALETTEENTRY) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFilePixelFormat(hemf : super::HENHMETAFILE, cbbuffer : super::UINT, ppfd : *mut PIXELFORMATDESCRIPTOR) -> super::UINT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetEnhMetaFileW(lpname : windows_sys::core::PCWSTR) -> super::HENHMETAFILE);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetFontData(hdc : super::HDC, dwtable : super::DWORD, dwoffset : super::DWORD, pvbuffer : super::PVOID, cjbuffer : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetFontLanguageInfo(hdc : super::HDC) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetFontUnicodeRanges(hdc : super::HDC, lpgs : LPGLYPHSET) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetGlyphIndicesA(hdc : super::HDC, lpstr : windows_sys::core::PCSTR, c : i32, pgi : super::LPWORD, fl : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetGlyphIndicesW(hdc : super::HDC, lpstr : windows_sys::core::PCWSTR, c : i32, pgi : super::LPWORD, fl : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetGlyphOutlineA(hdc : super::HDC, uchar : super::UINT, fuformat : super::UINT, lpgm : LPGLYPHMETRICS, cjbuffer : super::DWORD, pvbuffer : super::LPVOID, lpmat2 : *const MAT2) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetGlyphOutlineW(hdc : super::HDC, uchar : super::UINT, fuformat : super::UINT, lpgm : LPGLYPHMETRICS, cjbuffer : super::DWORD, pvbuffer : super::LPVOID, lpmat2 : *const MAT2) -> super::DWORD);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetGraphicsMode(hdc : super::HDC) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetICMProfileA(hdc : super::HDC, pbufsize : super::LPDWORD, pszfilename : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetICMProfileW(hdc : super::HDC, pbufsize : super::LPDWORD, pszfilename : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetKerningPairsA(hdc : super::HDC, npairs : super::DWORD, lpkernpair : LPKERNINGPAIR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetKerningPairsW(hdc : super::HDC, npairs : super::DWORD, lpkernpair : LPKERNINGPAIR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetLayout(hdc : super::HDC) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetLogColorSpaceA(hcolorspace : super::HCOLORSPACE, lpbuffer : LPLOGCOLORSPACEA, nsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetLogColorSpaceW(hcolorspace : super::HCOLORSPACE, lpbuffer : LPLOGCOLORSPACEW, nsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetMapMode(hdc : super::HDC) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn GetMetaFileA(lpname : windows_sys::core::PCSTR) -> super::HMETAFILE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("gdi32.dll" "system" fn GetMetaFileBitsEx(hmf : super::HMETAFILE, cbbuffer : super::UINT, lpdata : super::LPVOID) -> super::UINT);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn GetMetaFileW(lpname : windows_sys::core::PCWSTR) -> super::HMETAFILE);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetMetaRgn(hdc : super::HDC, hrgn : super::HRGN) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetMiterLimit(hdc : super::HDC, plimit : super::PFLOAT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetNearestColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetNearestPaletteIndex(h : super::HPALETTE, color : super::COLORREF) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetObjectA(h : super::HANDLE, c : i32, pv : super::LPVOID) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetObjectType(h : super::HGDIOBJ) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetObjectW(h : super::HANDLE, c : i32, pv : super::LPVOID) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetOutlineTextMetricsA(hdc : super::HDC, cjcopy : super::UINT, potm : LPOUTLINETEXTMETRICA) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetOutlineTextMetricsW(hdc : super::HDC, cjcopy : super::UINT, potm : LPOUTLINETEXTMETRICW) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetPaletteEntries(hpal : super::HPALETTE, istart : super::UINT, centries : super::UINT, ppalentries : LPPALETTEENTRY) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetPath(hdc : super::HDC, apt : super::LPPOINT, aj : super::LPBYTE, cpt : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPixel(hdc : super::HDC, x : i32, y : i32) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPixelFormat(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetPolyFillMode(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetROP2(hdc : super::HDC) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetRandomRgn(hdc : super::HDC, hrgn : super::HRGN, i : super::INT) -> i32);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn GetRasterizerCaps(lpraststat : LPRASTERIZER_STATUS, cjbytes : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetRegionData(hrgn : super::HRGN, ncount : super::DWORD, lprgndata : LPRGNDATA) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetRgnBox(hrgn : super::HRGN, lprc : super::LPRECT) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetStockObject(i : i32) -> super::HGDIOBJ);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetStretchBltMode(hdc : super::HDC) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetSystemPaletteEntries(hdc : super::HDC, istart : super::UINT, centries : super::UINT, ppalentries : LPPALETTEENTRY) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetSystemPaletteUse(hdc : super::HDC) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextAlign(hdc : super::HDC) -> super::UINT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextCharacterExtra(hdc : super::HDC) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextCharset(hdc : super::HDC) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextCharsetInfo(hdc : super::HDC, lpsig : LPFONTSIGNATURE, dwflags : super::DWORD) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextColor(hdc : super::HDC) -> super::COLORREF);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointA(hdc : super::HDC, lpszstring : windows_sys::core::PCSTR, cchstring : i32, nmaxextent : i32, lpnfit : super::LPINT, lpndx : super::LPINT, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointI(hdc : super::HDC, lpwszstring : super::LPWORD, cwchstring : i32, nmaxextent : i32, lpnfit : super::LPINT, lpndx : super::LPINT, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentExPointW(hdc : super::HDC, lpszstring : windows_sys::core::PCWSTR, cchstring : i32, nmaxextent : i32, lpnfit : super::LPINT, lpndx : super::LPINT, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPoint32A(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, c : i32, psizl : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPoint32W(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, c : i32, psizl : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointA(hdc : super::HDC, lpstring : windows_sys::core::PCSTR, c : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointI(hdc : super::HDC, pgiin : super::LPWORD, cgi : i32, psize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextExtentPointW(hdc : super::HDC, lpstring : windows_sys::core::PCWSTR, c : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextFaceA(hdc : super::HDC, c : i32, lpname : windows_sys::core::PSTR) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn GetTextFaceW(hdc : super::HDC, c : i32, lpname : windows_sys::core::PWSTR) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetTextMetricsA(hdc : super::HDC, lptm : LPTEXTMETRICA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn GetTextMetricsW(hdc : super::HDC, lptm : LPTEXTMETRICW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetViewportExtEx(hdc : super::HDC, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetViewportOrgEx(hdc : super::HDC, lppoint : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetWinMetaFileBits(hemf : super::HENHMETAFILE, cbdata16 : super::UINT, pdata16 : super::LPBYTE, imapmode : super::INT, hdcref : super::HDC) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetWindowExtEx(hdc : super::HDC, lpsize : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetWindowOrgEx(hdc : super::HDC, lppoint : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn GetWorldTransform(hdc : super::HDC, lpxf : LPXFORM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("msimg32.dll" "system" fn GradientFill(hdc : super::HDC, pvertex : PTRIVERTEX, nvertex : super::ULONG, pmesh : super::PVOID, nmesh : super::ULONG, ulmode : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn IntersectClipRect(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn InvertRgn(hdc : super::HDC, hrgn : super::HRGN) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn LPtoDP(hdc : super::HDC, lppt : super::LPPOINT, c : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn LineDDA(xstart : i32, ystart : i32, xend : i32, yend : i32, lpproc : LINEDDAPROC, data : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn LineTo(hdc : super::HDC, x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn MaskBlt(hdcdest : super::HDC, xdest : i32, ydest : i32, width : i32, height : i32, hdcsrc : super::HDC, xsrc : i32, ysrc : i32, hbmmask : super::HBITMAP, xmask : i32, ymask : i32, rop : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ModifyWorldTransform(hdc : super::HDC, lpxf : *const XFORM, mode : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn MoveToEx(hdc : super::HDC, x : i32, y : i32, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn OffsetClipRgn(hdc : super::HDC, x : i32, y : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn OffsetRgn(hrgn : super::HRGN, x : i32, y : i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn OffsetViewportOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn OffsetWindowOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PaintRgn(hdc : super::HDC, hrgn : super::HRGN) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PatBlt(hdc : super::HDC, x : i32, y : i32, w : i32, h : i32, rop : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PathToRegion(hdc : super::HDC) -> super::HRGN);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Pie(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32, xr1 : i32, yr1 : i32, xr2 : i32, yr2 : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PlayEnhMetaFile(hdc : super::HDC, hmf : super::HENHMETAFILE, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PlayEnhMetaFileRecord(hdc : super::HDC, pht : LPHANDLETABLE, pmr : *const ENHMETARECORD, cht : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PlayMetaFile(hdc : super::HDC, hmf : super::HMETAFILE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PlgBlt(hdcdest : super::HDC, lppoint : *const super::POINT, hdcsrc : super::HDC, xsrc : i32, ysrc : i32, width : i32, height : i32, hbmmask : super::HBITMAP, xmask : i32, ymask : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyBezier(hdc : super::HDC, apt : *const super::POINT, cpt : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyBezierTo(hdc : super::HDC, apt : *const super::POINT, cpt : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyDraw(hdc : super::HDC, apt : *const super::POINT, aj : *const super::BYTE, cpt : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyPolygon(hdc : super::HDC, apt : *const super::POINT, asz : *const super::INT, csz : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyPolyline(hdc : super::HDC, apt : *const super::POINT, asz : *const super::DWORD, csz : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyTextOutA(hdc : super::HDC, ppt : *const POLYTEXTA, nstrings : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolyTextOutW(hdc : super::HDC, ppt : *const POLYTEXTW, nstrings : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn Polygon(hdc : super::HDC, apt : *const super::POINT, cpt : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn Polyline(hdc : super::HDC, apt : *const super::POINT, cpt : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn PolylineTo(hdc : super::HDC, apt : *const super::POINT, cpt : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("gdi32.dll" "system" fn PtInRegion(hrgn : super::HRGN, x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn PtVisible(hdc : super::HDC, x : i32, y : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn RealizePalette(hdc : super::HDC) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn RectInRegion(hrgn : super::HRGN, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn RectVisible(hdc : super::HDC, lprect : *const super::RECT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn Rectangle(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("gdi32.dll" "system" fn RemoveFontMemResourceEx(h : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceA(lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceExA(name : windows_sys::core::PCSTR, fl : super::DWORD, pdv : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceExW(name : windows_sys::core::PCWSTR, fl : super::DWORD, pdv : super::PVOID) -> windows_sys::core::BOOL);
windows_link::link!("gdi32.dll" "system" fn RemoveFontResourceW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ResetDCA(hdc : super::HDC, lpdm : *const DEVMODEA) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn ResetDCW(hdc : super::HDC, lpdm : *const DEVMODEW) -> super::HDC);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ResizePalette(hpal : super::HPALETTE, n : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn RestoreDC(hdc : super::HDC, nsaveddc : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn RoundRect(hdc : super::HDC, left : i32, top : i32, right : i32, bottom : i32, width : i32, height : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SaveDC(hdc : super::HDC) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ScaleViewportExtEx(hdc : super::HDC, xn : i32, dx : i32, yn : i32, yd : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn ScaleWindowExtEx(hdc : super::HDC, xn : i32, xd : i32, yn : i32, yd : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
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
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetBitmapBits(hbm : super::HBITMAP, cb : super::DWORD, pvbits : *const core::ffi::c_void) -> super::LONG);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetBitmapDimensionEx(hbm : super::HBITMAP, w : i32, h : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBkColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetBkMode(hdc : super::HDC, mode : i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetBoundsRect(hdc : super::HDC, lprect : *const super::RECT, flags : super::UINT) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetBrushOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetColorAdjustment(hdc : super::HDC, lpca : *const COLORADJUSTMENT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetColorSpace(hdc : super::HDC, hcs : super::HCOLORSPACE) -> super::HCOLORSPACE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDCBrushColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetDCPenColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetDIBColorTable(hdc : super::HDC, istart : super::UINT, centries : super::UINT, prgbq : *const RGBQUAD) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetDIBits(hdc : super::HDC, hbm : super::HBITMAP, start : super::UINT, clines : super::UINT, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, coloruse : super::UINT) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetDIBitsToDevice(hdc : super::HDC, xdest : i32, ydest : i32, w : super::DWORD, h : super::DWORD, xsrc : i32, ysrc : i32, startscan : super::UINT, clines : super::UINT, lpvbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, coloruse : super::UINT) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetDeviceGammaRamp(hdc : super::HDC, lpramp : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetEnhMetaFileBits(nsize : super::UINT, pb : *const super::BYTE) -> super::HENHMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetGraphicsMode(hdc : super::HDC, imode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetICMMode(hdc : super::HDC, mode : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetICMProfileA(hdc : super::HDC, lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetICMProfileW(hdc : super::HDC, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetLayout(hdc : super::HDC, l : super::DWORD) -> super::DWORD);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetMapMode(hdc : super::HDC, imode : i32) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetMapperFlags(hdc : super::HDC, flags : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("gdi32.dll" "system" fn SetMetaFileBitsEx(cbbuffer : super::UINT, lpdata : *const super::BYTE) -> super::HMETAFILE);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetMetaRgn(hdc : super::HDC) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetMiterLimit(hdc : super::HDC, limit : super::FLOAT, old : super::PFLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetPaletteEntries(hpal : super::HPALETTE, istart : super::UINT, centries : super::UINT, ppalentries : *const PALETTEENTRY) -> super::UINT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetPixel(hdc : super::HDC, x : i32, y : i32, color : super::COLORREF) -> super::COLORREF);
#[cfg(all(feature = "shared", feature = "windef"))]
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
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetSystemPaletteUse(hdc : super::HDC, r#use : super::UINT) -> super::UINT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetTextAlign(hdc : super::HDC, align : super::UINT) -> super::UINT);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextCharacterExtra(hdc : super::HDC, extra : i32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextColor(hdc : super::HDC, color : super::COLORREF) -> super::COLORREF);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn SetTextJustification(hdc : super::HDC, extra : i32, count : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetViewportExtEx(hdc : super::HDC, x : i32, y : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetViewportOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetWinMetaFileBits(nsize : super::UINT, lpmeta16data : *const super::BYTE, hdcref : super::HDC, lpmfp : *const METAFILEPICT) -> super::HENHMETAFILE);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetWindowExtEx(hdc : super::HDC, x : i32, y : i32, lpsz : super::LPSIZE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetWindowOrgEx(hdc : super::HDC, x : i32, y : i32, lppt : super::LPPOINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn SetWorldTransform(hdc : super::HDC, lpxf : *const XFORM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn StartDocA(hdc : super::HDC, lpdi : *const DOCINFOA) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn StartDocW(hdc : super::HDC, lpdi : *const DOCINFOW) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn StartPage(hdc : super::HDC) -> i32);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn StretchBlt(hdcdest : super::HDC, xdest : i32, ydest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xsrc : i32, ysrc : i32, wsrc : i32, hsrc : i32, rop : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn StretchDIBits(hdc : super::HDC, xdest : i32, ydest : i32, destwidth : i32, destheight : i32, xsrc : i32, ysrc : i32, srcwidth : i32, srcheight : i32, lpbits : *const core::ffi::c_void, lpbmi : *const BITMAPINFO, iusage : super::UINT, rop : super::DWORD) -> i32);
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
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn TranslateCharsetInfo(lpsrc : *mut super::DWORD, lpcs : LPCHARSETINFO, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("msimg32.dll" "system" fn TransparentBlt(hdcdest : super::HDC, xorigindest : i32, yorigindest : i32, wdest : i32, hdest : i32, hdcsrc : super::HDC, xoriginsrc : i32, yoriginsrc : i32, wsrc : i32, hsrc : i32, crtransparent : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn UnrealizeObject(h : super::HGDIOBJ) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn UpdateColors(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn UpdateICMRegKeyA(reserved : super::DWORD, lpszcmid : windows_sys::core::PCSTR, lpszfilename : windows_sys::core::PCSTR, command : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("gdi32.dll" "system" fn UpdateICMRegKeyW(reserved : super::DWORD, lpszcmid : windows_sys::core::PCWSTR, lpszfilename : windows_sys::core::PCWSTR, command : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("gdi32.dll" "system" fn WidenPath(hdc : super::HDC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglCopyContext(param0 : super::HGLRC, param1 : super::HGLRC, param2 : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglCreateContext(param0 : super::HDC) -> super::HGLRC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglCreateLayerContext(param0 : super::HDC, param1 : i32) -> super::HGLRC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglDeleteContext(param0 : super::HGLRC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglDescribeLayerPlane(param0 : super::HDC, param1 : i32, param2 : i32, param3 : super::UINT, param4 : LPLAYERPLANEDESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglGetCurrentContext() -> super::HGLRC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglGetCurrentDC() -> super::HDC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglGetLayerPaletteEntries(param0 : super::HDC, param1 : i32, param2 : i32, param3 : i32, param4 : *mut super::COLORREF) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("opengl32.dll" "system" fn wglGetProcAddress(param0 : windows_sys::core::PCSTR) -> super::PROC);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglMakeCurrent(param0 : super::HDC, param1 : super::HGLRC) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglRealizeLayerPalette(param0 : super::HDC, param1 : i32, param2 : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglSetLayerPaletteEntries(param0 : super::HDC, param1 : i32, param2 : i32, param3 : i32, param4 : *const super::COLORREF) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("opengl32.dll" "system" fn wglShareLists(param0 : super::HGLRC, param1 : super::HGLRC) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglSwapLayerBuffers(param0 : super::HDC, param1 : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglSwapMultipleBuffers(param0 : super::UINT, param1 : *const WGLSWAP) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglUseFontBitmapsA(param0 : super::HDC, param1 : super::DWORD, param2 : super::DWORD, param3 : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglUseFontBitmapsW(param0 : super::HDC, param1 : super::DWORD, param2 : super::DWORD, param3 : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglUseFontOutlinesA(param0 : super::HDC, param1 : super::DWORD, param2 : super::DWORD, param3 : super::DWORD, param4 : super::FLOAT, param5 : super::FLOAT, param6 : i32, param7 : LPGLYPHMETRICSFLOAT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("opengl32.dll" "system" fn wglUseFontOutlinesW(param0 : super::HDC, param1 : super::DWORD, param2 : super::DWORD, param3 : super::DWORD, param4 : super::FLOAT, param5 : super::FLOAT, param6 : i32, param7 : LPGLYPHMETRICSFLOAT) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ABC {
    pub abcA: i32,
    pub abcB: super::UINT,
    pub abcC: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct ABCFLOAT {
    pub abcfA: super::FLOAT,
    pub abcfB: super::FLOAT,
    pub abcfC: super::FLOAT,
}
pub const ABORTDOC: i32 = 2;
#[cfg(feature = "windef")]
pub type ABORTPROC = Option<unsafe extern "system" fn(param0: super::HDC, param1: i32) -> windows_sys::core::BOOL>;
pub const ABSOLUTE: i32 = 1;
pub const AC_SRC_ALPHA: i32 = 1;
pub const AC_SRC_OVER: i32 = 0;
pub const AD_CLOCKWISE: i32 = 2;
pub const AD_COUNTERCLOCKWISE: i32 = 1;
pub const ALTERNATE: i32 = 1;
pub const ANSI_CHARSET: i32 = 0;
pub const ANSI_FIXED_FONT: i32 = 11;
pub const ANSI_VAR_FONT: i32 = 12;
pub const ANTIALIASED_QUALITY: i32 = 4;
pub const ARABIC_CHARSET: i32 = 178;
pub const ASPECTX: i32 = 40;
pub const ASPECTXY: i32 = 44;
pub const ASPECTY: i32 = 42;
pub const ASPECT_FILTERING: i32 = 1;
#[cfg(feature = "shared")]
pub type AXESLIST = AXESLISTA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct AXESLISTA {
    pub axlReserved: super::DWORD,
    pub axlNumAxes: super::DWORD,
    pub axlAxisInfo: [AXISINFOA; 16],
}
#[cfg(feature = "shared")]
impl Default for AXESLISTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AXESLISTW {
    pub axlReserved: super::DWORD,
    pub axlNumAxes: super::DWORD,
    pub axlAxisInfo: [AXISINFOW; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AXESLISTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type AXISINFO = AXISINFOA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct AXISINFOA {
    pub axMinValue: super::LONG,
    pub axMaxValue: super::LONG,
    pub axAxisName: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for AXISINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AXISINFOW {
    pub axMinValue: super::LONG,
    pub axMaxValue: super::LONG,
    pub axAxisName: [super::WCHAR; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for AXISINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BALTIC_CHARSET: i32 = 186;
pub const BANDINFO: i32 = 24;
pub type BCHAR = u8;
pub const BEGIN_PATH: i32 = 4096;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BITMAP {
    pub bmType: super::LONG,
    pub bmWidth: super::LONG,
    pub bmHeight: super::LONG,
    pub bmWidthBytes: super::LONG,
    pub bmPlanes: super::WORD,
    pub bmBitsPixel: super::WORD,
    pub bmBits: super::LPVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BITMAPCOREHEADER {
    pub bcSize: super::DWORD,
    pub bcWidth: super::WORD,
    pub bcHeight: super::WORD,
    pub bcPlanes: super::WORD,
    pub bcBitCount: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BITMAPCOREINFO {
    pub bmciHeader: BITMAPCOREHEADER,
    pub bmciColors: [RGBTRIPLE; 1],
}
#[cfg(feature = "shared")]
impl Default for BITMAPCOREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BITMAPFILEHEADER {
    pub bfType: super::WORD,
    pub bfSize: super::DWORD,
    pub bfReserved1: super::WORD,
    pub bfReserved2: super::WORD,
    pub bfOffBits: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 1],
}
#[cfg(feature = "shared")]
impl Default for BITMAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BITMAPINFOHEADER {
    pub biSize: super::DWORD,
    pub biWidth: super::LONG,
    pub biHeight: super::LONG,
    pub biPlanes: super::WORD,
    pub biBitCount: super::WORD,
    pub biCompression: super::DWORD,
    pub biSizeImage: super::DWORD,
    pub biXPelsPerMeter: super::LONG,
    pub biYPelsPerMeter: super::LONG,
    pub biClrUsed: super::DWORD,
    pub biClrImportant: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BITMAPV4HEADER {
    pub bV4Size: super::DWORD,
    pub bV4Width: super::LONG,
    pub bV4Height: super::LONG,
    pub bV4Planes: super::WORD,
    pub bV4BitCount: super::WORD,
    pub bV4V4Compression: super::DWORD,
    pub bV4SizeImage: super::DWORD,
    pub bV4XPelsPerMeter: super::LONG,
    pub bV4YPelsPerMeter: super::LONG,
    pub bV4ClrUsed: super::DWORD,
    pub bV4ClrImportant: super::DWORD,
    pub bV4RedMask: super::DWORD,
    pub bV4GreenMask: super::DWORD,
    pub bV4BlueMask: super::DWORD,
    pub bV4AlphaMask: super::DWORD,
    pub bV4CSType: super::DWORD,
    pub bV4Endpoints: CIEXYZTRIPLE,
    pub bV4GammaRed: super::DWORD,
    pub bV4GammaGreen: super::DWORD,
    pub bV4GammaBlue: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BITMAPV5HEADER {
    pub bV5Size: super::DWORD,
    pub bV5Width: super::LONG,
    pub bV5Height: super::LONG,
    pub bV5Planes: super::WORD,
    pub bV5BitCount: super::WORD,
    pub bV5Compression: super::DWORD,
    pub bV5SizeImage: super::DWORD,
    pub bV5XPelsPerMeter: super::LONG,
    pub bV5YPelsPerMeter: super::LONG,
    pub bV5ClrUsed: super::DWORD,
    pub bV5ClrImportant: super::DWORD,
    pub bV5RedMask: super::DWORD,
    pub bV5GreenMask: super::DWORD,
    pub bV5BlueMask: super::DWORD,
    pub bV5AlphaMask: super::DWORD,
    pub bV5CSType: super::DWORD,
    pub bV5Endpoints: CIEXYZTRIPLE,
    pub bV5GammaRed: super::DWORD,
    pub bV5GammaGreen: super::DWORD,
    pub bV5GammaBlue: super::DWORD,
    pub bV5Intent: super::DWORD,
    pub bV5ProfileData: super::DWORD,
    pub bV5ProfileSize: super::DWORD,
    pub bV5Reserved: super::DWORD,
}
pub const BITSPIXEL: i32 = 12;
pub const BI_BITFIELDS: i32 = 3;
pub const BI_JPEG: i32 = 4;
pub const BI_PNG: i32 = 5;
pub const BI_RGB: i32 = 0;
pub const BI_RLE4: i32 = 2;
pub const BI_RLE8: i32 = 1;
pub const BKMODE_LAST: i32 = 2;
#[cfg(feature = "shared")]
pub const BLACKNESS: super::DWORD = 66;
pub const BLACKONWHITE: i32 = 1;
pub const BLACK_BRUSH: i32 = 4;
pub const BLACK_PEN: i32 = 7;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BLENDFUNCTION {
    pub BlendOp: super::BYTE,
    pub BlendFlags: super::BYTE,
    pub SourceConstantAlpha: super::BYTE,
    pub AlphaFormat: super::BYTE,
}
pub const BLTALIGNMENT: i32 = 119;
pub const BS_DIBPATTERN: i32 = 5;
pub const BS_DIBPATTERN8X8: i32 = 8;
pub const BS_DIBPATTERNPT: i32 = 6;
pub const BS_HATCHED: i32 = 2;
pub const BS_HOLLOW: i32 = 1;
pub const BS_INDEXED: i32 = 4;
pub const BS_MONOPATTERN: i32 = 9;
pub const BS_NULL: i32 = 1;
pub const BS_PATTERN: i32 = 3;
pub const BS_PATTERN8X8: i32 = 7;
pub const BS_SOLID: i32 = 0;
#[cfg(feature = "shared")]
pub const CAPTUREBLT: super::DWORD = 1073741824;
pub const CA_LOG_FILTER: i32 = 2;
pub const CA_NEGATIVE: i32 = 1;
pub const CBM_INIT: i32 = 4;
pub const CCHDEVICENAME: i32 = 32;
pub const CCHFORMNAME: i32 = 32;
pub const CC_CHORD: i32 = 4;
pub const CC_CIRCLES: i32 = 1;
pub const CC_ELLIPSES: i32 = 8;
pub const CC_INTERIORS: i32 = 128;
pub const CC_NONE: i32 = 0;
pub const CC_PIE: i32 = 2;
pub const CC_ROUNDRECT: i32 = 256;
pub const CC_STYLED: i32 = 32;
pub const CC_WIDE: i32 = 16;
pub const CC_WIDESTYLED: i32 = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CHARSETINFO {
    pub ciCharset: super::UINT,
    pub ciACP: super::UINT,
    pub fs: FONTSIGNATURE,
}
pub const CHECKJPEGFORMAT: i32 = 4119;
pub const CHECKPNGFORMAT: i32 = 4120;
pub const CHINESEBIG5_CHARSET: i32 = 136;
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
pub const CLEARTYPE_NATURAL_QUALITY: i32 = 6;
pub const CLEARTYPE_QUALITY: i32 = 5;
pub const CLIPCAPS: i32 = 36;
pub const CLIP_CHARACTER_PRECIS: i32 = 1;
pub const CLIP_DEFAULT_PRECIS: i32 = 0;
pub const CLIP_DFA_DISABLE: i32 = 64;
pub const CLIP_EMBEDDED: i32 = 128;
pub const CLIP_LH_ANGLES: i32 = 16;
pub const CLIP_MASK: i32 = 15;
pub const CLIP_STROKE_PRECIS: i32 = 2;
pub const CLIP_TO_PATH: i32 = 4097;
pub const CLIP_TT_ALWAYS: i32 = 32;
pub const CLOSECHANNEL: i32 = 4112;
pub const CLR_INVALID: u32 = 4294967295;
pub const CM_CMYK_COLOR: i32 = 4;
pub const CM_DEVICE_ICM: i32 = 1;
pub const CM_GAMMA_RAMP: i32 = 2;
pub const CM_IN_GAMUT: i32 = 0;
pub const CM_NONE: i32 = 0;
pub const CM_OUT_OF_GAMUT: i32 = 255;
pub type COLOR16 = u16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct COLORADJUSTMENT {
    pub caSize: super::WORD,
    pub caFlags: super::WORD,
    pub caIlluminantIndex: super::WORD,
    pub caRedGamma: super::WORD,
    pub caGreenGamma: super::WORD,
    pub caBlueGamma: super::WORD,
    pub caReferenceBlack: super::WORD,
    pub caReferenceWhite: super::WORD,
    pub caContrast: super::SHORT,
    pub caBrightness: super::SHORT,
    pub caColorfulness: super::SHORT,
    pub caRedGreenTint: super::SHORT,
}
pub const COLORMATCHTOTARGET_EMBEDED: i32 = 1;
pub const COLORMGMTCAPS: i32 = 121;
pub const COLORONCOLOR: i32 = 3;
pub const COLORRES: i32 = 108;
#[cfg(feature = "shared")]
pub const COLOR_ADJ_MAX: super::SHORT = 100;
#[cfg(feature = "shared")]
pub const COLOR_ADJ_MIN: super::SHORT = -100;
pub const COMPLEXREGION: i32 = 3;
pub const CP_NONE: i32 = 0;
pub const CP_RECTANGLE: i32 = 1;
pub const CP_REGION: i32 = 2;
pub const CREATECOLORSPACE_EMBEDED: i32 = 1;
pub const CS_DELETE_TRANSFORM: i32 = 3;
pub const CS_DISABLE: i32 = 2;
pub const CS_ENABLE: i32 = 1;
pub const CURVECAPS: i32 = 28;
pub const DCBA_FACEDOWNCENTER: i32 = 257;
pub const DCBA_FACEDOWNLEFT: i32 = 258;
pub const DCBA_FACEDOWNNONE: i32 = 256;
pub const DCBA_FACEDOWNRIGHT: i32 = 259;
pub const DCBA_FACEUPCENTER: i32 = 1;
pub const DCBA_FACEUPLEFT: i32 = 2;
pub const DCBA_FACEUPNONE: i32 = 0;
pub const DCBA_FACEUPRIGHT: i32 = 3;
pub const DCB_ACCUMULATE: i32 = 2;
pub const DCB_DIRTY: i32 = 2;
pub const DCB_DISABLE: i32 = 8;
pub const DCB_ENABLE: i32 = 4;
pub const DCB_RESET: i32 = 1;
pub const DCB_SET: i32 = 3;
pub const DCTT_BITMAP: i32 = 1;
pub const DCTT_DOWNLOAD: i32 = 2;
pub const DCTT_DOWNLOAD_OUTLINE: i32 = 8;
pub const DCTT_SUBDEV: i32 = 4;
pub const DC_BINADJUST: i32 = 19;
pub const DC_BINNAMES: i32 = 12;
pub const DC_BINS: i32 = 6;
pub const DC_BRUSH: i32 = 18;
pub const DC_COLLATE: i32 = 22;
pub const DC_COLORDEVICE: i32 = 32;
pub const DC_COPIES: i32 = 18;
pub const DC_DATATYPE_PRODUCED: i32 = 21;
pub const DC_DRIVER: i32 = 11;
pub const DC_DUPLEX: i32 = 7;
pub const DC_EMF_COMPLIANT: i32 = 20;
pub const DC_ENUMRESOLUTIONS: i32 = 13;
pub const DC_EXTRA: i32 = 9;
pub const DC_FIELDS: i32 = 1;
pub const DC_FILEDEPENDENCIES: i32 = 14;
pub const DC_MANUFACTURER: i32 = 23;
pub const DC_MAXEXTENT: i32 = 5;
pub const DC_MEDIAREADY: i32 = 29;
pub const DC_MEDIATYPENAMES: i32 = 34;
pub const DC_MEDIATYPES: i32 = 35;
pub const DC_MINEXTENT: i32 = 4;
pub const DC_MODEL: i32 = 24;
pub const DC_NUP: i32 = 33;
pub const DC_ORIENTATION: i32 = 17;
pub const DC_PAPERNAMES: i32 = 16;
pub const DC_PAPERS: i32 = 2;
pub const DC_PAPERSIZE: i32 = 3;
pub const DC_PEN: i32 = 19;
pub const DC_PERSONALITY: i32 = 25;
pub const DC_PRINTERMEM: i32 = 28;
pub const DC_PRINTRATE: i32 = 26;
pub const DC_PRINTRATEPPM: i32 = 31;
pub const DC_PRINTRATEUNIT: i32 = 27;
pub const DC_SIZE: i32 = 8;
pub const DC_STAPLE: i32 = 30;
pub const DC_TRUETYPE: i32 = 15;
pub const DC_VERSION: i32 = 10;
pub const DEFAULT_CHARSET: i32 = 1;
pub const DEFAULT_GUI_FONT: i32 = 17;
pub const DEFAULT_PALETTE: i32 = 15;
pub const DEFAULT_PITCH: i32 = 0;
pub const DEFAULT_QUALITY: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DESIGNVECTOR {
    pub dvReserved: super::DWORD,
    pub dvNumAxes: super::DWORD,
    pub dvValues: [super::LONG; 16],
}
#[cfg(feature = "shared")]
impl Default for DESIGNVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DESKTOPHORZRES: i32 = 118;
pub const DESKTOPVERTRES: i32 = 117;
pub const DEVICEDATA: i32 = 19;
pub const DEVICE_DEFAULT_FONT: i32 = 14;
pub const DEVICE_FONTTYPE: i32 = 2;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type DEVMODE = DEVMODEA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DEVMODEA {
    pub dmDeviceName: [super::BYTE; 32],
    pub dmSpecVersion: super::WORD,
    pub dmDriverVersion: super::WORD,
    pub dmSize: super::WORD,
    pub dmDriverExtra: super::WORD,
    pub dmFields: super::DWORD,
    pub Anonymous: DEVMODEA_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [super::BYTE; 32],
    pub dmLogPixels: super::WORD,
    pub dmBitsPerPel: super::DWORD,
    pub dmPelsWidth: super::DWORD,
    pub dmPelsHeight: super::DWORD,
    pub Anonymous2: DEVMODEA_1,
    pub dmDisplayFrequency: super::DWORD,
    pub dmICMMethod: super::DWORD,
    pub dmICMIntent: super::DWORD,
    pub dmMediaType: super::DWORD,
    pub dmDitherType: super::DWORD,
    pub dmReserved1: super::DWORD,
    pub dmReserved2: super::DWORD,
    pub dmPanningWidth: super::DWORD,
    pub dmPanningHeight: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for DEVMODEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub union DEVMODEA_0 {
    pub Anonymous: DEVMODEA_0_0,
    pub Anonymous2: DEVMODEA_0_1,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for DEVMODEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
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
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DEVMODEA_0_1 {
    pub dmPosition: super::POINTL,
    pub dmDisplayOrientation: super::DWORD,
    pub dmDisplayFixedOutput: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub union DEVMODEA_1 {
    pub dmDisplayFlags: super::DWORD,
    pub dmNup: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for DEVMODEA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEVMODEW {
    pub dmDeviceName: [super::WCHAR; 32],
    pub dmSpecVersion: super::WORD,
    pub dmDriverVersion: super::WORD,
    pub dmSize: super::WORD,
    pub dmDriverExtra: super::WORD,
    pub dmFields: super::DWORD,
    pub Anonymous: DEVMODEW_0,
    pub dmColor: i16,
    pub dmDuplex: i16,
    pub dmYResolution: i16,
    pub dmTTOption: i16,
    pub dmCollate: i16,
    pub dmFormName: [super::WCHAR; 32],
    pub dmLogPixels: super::WORD,
    pub dmBitsPerPel: super::DWORD,
    pub dmPelsWidth: super::DWORD,
    pub dmPelsHeight: super::DWORD,
    pub Anonymous2: DEVMODEW_1,
    pub dmDisplayFrequency: super::DWORD,
    pub dmICMMethod: super::DWORD,
    pub dmICMIntent: super::DWORD,
    pub dmMediaType: super::DWORD,
    pub dmDitherType: super::DWORD,
    pub dmReserved1: super::DWORD,
    pub dmReserved2: super::DWORD,
    pub dmPanningWidth: super::DWORD,
    pub dmPanningHeight: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for DEVMODEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DEVMODEW_0 {
    pub Anonymous: DEVMODEW_0_0,
    pub Anonymous2: DEVMODEW_0_1,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for DEVMODEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DEVMODEW_0_1 {
    pub dmPosition: super::POINTL,
    pub dmDisplayOrientation: super::DWORD,
    pub dmDisplayFixedOutput: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DEVMODEW_1 {
    pub dmDisplayFlags: super::DWORD,
    pub dmNup: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for DEVMODEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DIBSECTION {
    pub dsBm: BITMAP,
    pub dsBmih: BITMAPINFOHEADER,
    pub dsBitfields: [super::DWORD; 3],
    pub dshSection: super::HANDLE,
    pub dsOffset: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for DIBSECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIB_PAL_COLORS: i32 = 1;
pub const DIB_RGB_COLORS: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_2DREGION {
    pub cx: super::UINT32,
    pub cy: super::UINT32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_ADAPTER_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub adapterDevicePath: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "windef"))]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    pub r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
    pub size: super::UINT32,
    pub adapterId: super::LUID,
    pub id: super::UINT32,
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0,
    pub colorEncoding: DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2 {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0,
    pub colorEncoding: DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: super::UINT32,
    pub activeColorMode: DISPLAYCONFIG_ADVANCED_COLOR_MODE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_MAXPATH: i32 = 1024;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_MODE_INFO {
    pub infoType: DISPLAYCONFIG_MODE_INFO_TYPE,
    pub id: super::UINT32,
    pub adapterId: super::LUID,
    pub Anonymous: DISPLAYCONFIG_MODE_INFO_0,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_MODE_INFO_0 {
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
    pub sourceMode: DISPLAYCONFIG_SOURCE_MODE,
    pub desktopImageInfo: DISPLAYCONFIG_DESKTOP_IMAGE_INFO,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
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
pub const DISPLAYCONFIG_PATH_ACTIVE: i32 = 1;
pub const DISPLAYCONFIG_PATH_BOOST_REFRESH_RATE: i32 = 16;
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: i32 = 65535;
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: i32 = 65535;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_INFO {
    pub sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    pub flags: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: u32 = 4294967295;
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    pub adapterId: super::LUID,
    pub id: super::UINT32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0,
    pub statusFlags: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    pub modeInfoIdx: super::UINT32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: i32 = 65535;
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO {
    pub adapterId: super::LUID,
    pub id: super::UINT32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub rotation: DISPLAYCONFIG_ROTATION,
    pub scaling: DISPLAYCONFIG_SCALING,
    pub refreshRate: DISPLAYCONFIG_RATIONAL,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
    pub targetAvailable: windows_sys::core::BOOL,
    pub statusFlags: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    pub modeInfoIdx: super::UINT32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: i32 = 65535;
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: i32 = 29;
pub type DISPLAYCONFIG_PIXELFORMAT = i32;
pub const DISPLAYCONFIG_PIXELFORMAT_16BPP: DISPLAYCONFIG_PIXELFORMAT = 2;
pub const DISPLAYCONFIG_PIXELFORMAT_24BPP: DISPLAYCONFIG_PIXELFORMAT = 3;
pub const DISPLAYCONFIG_PIXELFORMAT_32BPP: DISPLAYCONFIG_PIXELFORMAT = 4;
pub const DISPLAYCONFIG_PIXELFORMAT_8BPP: DISPLAYCONFIG_PIXELFORMAT = 1;
pub const DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32: DISPLAYCONFIG_PIXELFORMAT = -1;
pub const DISPLAYCONFIG_PIXELFORMAT_NONGDI: DISPLAYCONFIG_PIXELFORMAT = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_RATIONAL {
    pub Numerator: super::UINT32,
    pub Denominator: super::UINT32,
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub SDRWhiteLevel: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_HDR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_HDR_STATE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_HDR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_HDR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_HDR_STATE_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_HDR_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_HDR_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0,
    pub specializationType: windows_sys::core::GUID,
    pub specializationSubType: windows_sys::core::GUID,
    pub specializationApplicationName: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_WCG_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_WCG_STATE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_WCG_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_WCG_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_WCG_STATE_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SET_WCG_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SET_WCG_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub viewGdiDeviceName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAYCONFIG_SOURCE_IN_USE: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SOURCE_MODE {
    pub width: super::UINT32,
    pub height: super::UINT32,
    pub pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    pub position: super::POINTL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0,
    pub value: super::UINT32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub baseOutputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub edidManufactureId: super::UINT16,
    pub edidProductCodeId: super::UINT16,
    pub connectorInstance: super::UINT32,
    pub monitorFriendlyDeviceName: [super::WCHAR; 64],
    pub monitorDevicePath: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0,
}
#[cfg(feature = "shared")]
impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0,
    pub value: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: i32 = 4;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: i32 = 8;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: i32 = 16;
pub const DISPLAYCONFIG_TARGET_FORCIBLE: i32 = 2;
pub const DISPLAYCONFIG_TARGET_IN_USE: i32 = 1;
pub const DISPLAYCONFIG_TARGET_IS_HMD: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_MODE {
    pub targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
}
#[cfg(feature = "shared")]
impl Default for DISPLAYCONFIG_TARGET_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub width: super::UINT32,
    pub height: super::UINT32,
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    pub pixelRate: super::UINT64,
    pub hSyncFreq: DISPLAYCONFIG_RATIONAL,
    pub vSyncFreq: DISPLAYCONFIG_RATIONAL,
    pub activeSize: DISPLAYCONFIG_2DREGION,
    pub totalSize: DISPLAYCONFIG_2DREGION,
    pub Anonymous: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
}
#[cfg(feature = "shared")]
impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    pub AdditionalSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0,
    pub videoStandard: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
pub type DISPLAY_DEVICE = DISPLAY_DEVICEA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISPLAY_DEVICEA {
    pub cb: super::DWORD,
    pub DeviceName: [super::CHAR; 32],
    pub DeviceString: [super::CHAR; 128],
    pub StateFlags: super::DWORD,
    pub DeviceID: [super::CHAR; 128],
    pub DeviceKey: [super::CHAR; 128],
}
#[cfg(feature = "shared")]
impl Default for DISPLAY_DEVICEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DISPLAY_DEVICEW {
    pub cb: super::DWORD,
    pub DeviceName: [super::WCHAR; 32],
    pub DeviceString: [super::WCHAR; 128],
    pub StateFlags: super::DWORD,
    pub DeviceID: [super::WCHAR; 128],
    pub DeviceKey: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DISPLAY_DEVICEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAY_DEVICE_ACC_DRIVER: i32 = 64;
pub const DISPLAY_DEVICE_ACTIVE: i32 = 1;
pub const DISPLAY_DEVICE_ATTACHED: i32 = 2;
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: i32 = 1;
pub const DISPLAY_DEVICE_DISCONNECT: i32 = 33554432;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: i32 = 8;
pub const DISPLAY_DEVICE_MODESPRUNED: i32 = 134217728;
pub const DISPLAY_DEVICE_MULTI_DRIVER: i32 = 2;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: i32 = 4;
pub const DISPLAY_DEVICE_RDPUDD: i32 = 16777216;
pub const DISPLAY_DEVICE_REMOTE: i32 = 67108864;
pub const DISPLAY_DEVICE_REMOVABLE: i32 = 32;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: i32 = 2097152;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: i32 = 524288;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: i32 = 16;
pub const DI_APPBANDING: i32 = 1;
pub const DI_ROPS_READ_DESTINATION: i32 = 2;
pub const DKGRAY_BRUSH: i32 = 3;
pub const DMBIN_AUTO: i32 = 7;
pub const DMBIN_CASSETTE: i32 = 14;
pub const DMBIN_ENVELOPE: i32 = 5;
pub const DMBIN_ENVMANUAL: i32 = 6;
pub const DMBIN_FIRST: i32 = 1;
pub const DMBIN_FORMSOURCE: i32 = 15;
pub const DMBIN_LARGECAPACITY: i32 = 11;
pub const DMBIN_LARGEFMT: i32 = 10;
pub const DMBIN_LAST: i32 = 15;
pub const DMBIN_LOWER: i32 = 2;
pub const DMBIN_MANUAL: i32 = 4;
pub const DMBIN_MIDDLE: i32 = 3;
pub const DMBIN_ONLYONE: i32 = 1;
pub const DMBIN_SMALLFMT: i32 = 9;
pub const DMBIN_TRACTOR: i32 = 8;
pub const DMBIN_UPPER: i32 = 1;
pub const DMBIN_USER: i32 = 256;
pub const DMCOLLATE_FALSE: i32 = 0;
pub const DMCOLLATE_TRUE: i32 = 1;
pub const DMCOLOR_COLOR: i32 = 2;
pub const DMCOLOR_MONOCHROME: i32 = 1;
pub const DMDFO_CENTER: i32 = 2;
pub const DMDFO_DEFAULT: i32 = 0;
pub const DMDFO_STRETCH: i32 = 1;
pub const DMDISPLAYFLAGS_TEXTMODE: i32 = 4;
pub const DMDITHER_COARSE: i32 = 2;
pub const DMDITHER_ERRORDIFFUSION: i32 = 5;
pub const DMDITHER_FINE: i32 = 3;
pub const DMDITHER_GRAYSCALE: i32 = 10;
pub const DMDITHER_LINEART: i32 = 4;
pub const DMDITHER_NONE: i32 = 1;
pub const DMDITHER_RESERVED6: i32 = 6;
pub const DMDITHER_RESERVED7: i32 = 7;
pub const DMDITHER_RESERVED8: i32 = 8;
pub const DMDITHER_RESERVED9: i32 = 9;
pub const DMDITHER_USER: i32 = 256;
pub const DMDO_180: i32 = 2;
pub const DMDO_270: i32 = 3;
pub const DMDO_90: i32 = 1;
pub const DMDO_DEFAULT: i32 = 0;
pub const DMDUP_HORIZONTAL: i32 = 3;
pub const DMDUP_SIMPLEX: i32 = 1;
pub const DMDUP_VERTICAL: i32 = 2;
pub const DMICMMETHOD_DEVICE: i32 = 4;
pub const DMICMMETHOD_DRIVER: i32 = 3;
pub const DMICMMETHOD_NONE: i32 = 1;
pub const DMICMMETHOD_SYSTEM: i32 = 2;
pub const DMICMMETHOD_USER: i32 = 256;
pub const DMICM_ABS_COLORIMETRIC: i32 = 4;
pub const DMICM_COLORIMETRIC: i32 = 3;
pub const DMICM_CONTRAST: i32 = 2;
pub const DMICM_SATURATE: i32 = 1;
pub const DMICM_USER: i32 = 256;
pub const DMMEDIA_GLOSSY: i32 = 3;
pub const DMMEDIA_STANDARD: i32 = 1;
pub const DMMEDIA_TRANSPARENCY: i32 = 2;
pub const DMMEDIA_USER: i32 = 256;
pub const DMNUP_ONEUP: i32 = 2;
pub const DMNUP_SYSTEM: i32 = 1;
pub const DMORIENT_LANDSCAPE: i32 = 2;
pub const DMORIENT_PORTRAIT: i32 = 1;
pub const DMPAPER_10X11: i32 = 45;
pub const DMPAPER_10X14: i32 = 16;
pub const DMPAPER_11X17: i32 = 17;
pub const DMPAPER_12X11: i32 = 90;
pub const DMPAPER_15X11: i32 = 46;
pub const DMPAPER_9X11: i32 = 44;
pub const DMPAPER_A2: i32 = 66;
pub const DMPAPER_A3: i32 = 8;
pub const DMPAPER_A3_EXTRA: i32 = 63;
pub const DMPAPER_A3_EXTRA_TRANSVERSE: i32 = 68;
pub const DMPAPER_A3_ROTATED: i32 = 76;
pub const DMPAPER_A3_TRANSVERSE: i32 = 67;
pub const DMPAPER_A4: i32 = 9;
pub const DMPAPER_A4SMALL: i32 = 10;
pub const DMPAPER_A4_EXTRA: i32 = 53;
pub const DMPAPER_A4_PLUS: i32 = 60;
pub const DMPAPER_A4_ROTATED: i32 = 77;
pub const DMPAPER_A4_TRANSVERSE: i32 = 55;
pub const DMPAPER_A5: i32 = 11;
pub const DMPAPER_A5_EXTRA: i32 = 64;
pub const DMPAPER_A5_ROTATED: i32 = 78;
pub const DMPAPER_A5_TRANSVERSE: i32 = 61;
pub const DMPAPER_A6: i32 = 70;
pub const DMPAPER_A6_ROTATED: i32 = 83;
pub const DMPAPER_A_PLUS: i32 = 57;
pub const DMPAPER_B4: i32 = 12;
pub const DMPAPER_B4_JIS_ROTATED: i32 = 79;
pub const DMPAPER_B5: i32 = 13;
pub const DMPAPER_B5_EXTRA: i32 = 65;
pub const DMPAPER_B5_JIS_ROTATED: i32 = 80;
pub const DMPAPER_B5_TRANSVERSE: i32 = 62;
pub const DMPAPER_B6_JIS: i32 = 88;
pub const DMPAPER_B6_JIS_ROTATED: i32 = 89;
pub const DMPAPER_B_PLUS: i32 = 58;
pub const DMPAPER_CSHEET: i32 = 24;
pub const DMPAPER_DBL_JAPANESE_POSTCARD: i32 = 69;
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: i32 = 82;
pub const DMPAPER_DSHEET: i32 = 25;
pub const DMPAPER_ENV_10: i32 = 20;
pub const DMPAPER_ENV_11: i32 = 21;
pub const DMPAPER_ENV_12: i32 = 22;
pub const DMPAPER_ENV_14: i32 = 23;
pub const DMPAPER_ENV_9: i32 = 19;
pub const DMPAPER_ENV_B4: i32 = 33;
pub const DMPAPER_ENV_B5: i32 = 34;
pub const DMPAPER_ENV_B6: i32 = 35;
pub const DMPAPER_ENV_C3: i32 = 29;
pub const DMPAPER_ENV_C4: i32 = 30;
pub const DMPAPER_ENV_C5: i32 = 28;
pub const DMPAPER_ENV_C6: i32 = 31;
pub const DMPAPER_ENV_C65: i32 = 32;
pub const DMPAPER_ENV_DL: i32 = 27;
pub const DMPAPER_ENV_INVITE: i32 = 47;
pub const DMPAPER_ENV_ITALY: i32 = 36;
pub const DMPAPER_ENV_MONARCH: i32 = 37;
pub const DMPAPER_ENV_PERSONAL: i32 = 38;
pub const DMPAPER_ESHEET: i32 = 26;
pub const DMPAPER_EXECUTIVE: i32 = 7;
pub const DMPAPER_FANFOLD_LGL_GERMAN: i32 = 41;
pub const DMPAPER_FANFOLD_STD_GERMAN: i32 = 40;
pub const DMPAPER_FANFOLD_US: i32 = 39;
pub const DMPAPER_FIRST: i32 = 1;
pub const DMPAPER_FOLIO: i32 = 14;
pub const DMPAPER_ISO_B4: i32 = 42;
pub const DMPAPER_JAPANESE_POSTCARD: i32 = 43;
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: i32 = 81;
pub const DMPAPER_JENV_CHOU3: i32 = 73;
pub const DMPAPER_JENV_CHOU3_ROTATED: i32 = 86;
pub const DMPAPER_JENV_CHOU4: i32 = 74;
pub const DMPAPER_JENV_CHOU4_ROTATED: i32 = 87;
pub const DMPAPER_JENV_KAKU2: i32 = 71;
pub const DMPAPER_JENV_KAKU2_ROTATED: i32 = 84;
pub const DMPAPER_JENV_KAKU3: i32 = 72;
pub const DMPAPER_JENV_KAKU3_ROTATED: i32 = 85;
pub const DMPAPER_JENV_YOU4: i32 = 91;
pub const DMPAPER_JENV_YOU4_ROTATED: i32 = 92;
pub const DMPAPER_LAST: i32 = 118;
pub const DMPAPER_LEDGER: i32 = 4;
pub const DMPAPER_LEGAL: i32 = 5;
pub const DMPAPER_LEGAL_EXTRA: i32 = 51;
pub const DMPAPER_LETTER: i32 = 1;
pub const DMPAPER_LETTERSMALL: i32 = 2;
pub const DMPAPER_LETTER_EXTRA: i32 = 50;
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: i32 = 56;
pub const DMPAPER_LETTER_PLUS: i32 = 59;
pub const DMPAPER_LETTER_ROTATED: i32 = 75;
pub const DMPAPER_LETTER_TRANSVERSE: i32 = 54;
pub const DMPAPER_NOTE: i32 = 18;
pub const DMPAPER_P16K: i32 = 93;
pub const DMPAPER_P16K_ROTATED: i32 = 106;
pub const DMPAPER_P32K: i32 = 94;
pub const DMPAPER_P32KBIG: i32 = 95;
pub const DMPAPER_P32KBIG_ROTATED: i32 = 108;
pub const DMPAPER_P32K_ROTATED: i32 = 107;
pub const DMPAPER_PENV_1: i32 = 96;
pub const DMPAPER_PENV_10: i32 = 105;
pub const DMPAPER_PENV_10_ROTATED: i32 = 118;
pub const DMPAPER_PENV_1_ROTATED: i32 = 109;
pub const DMPAPER_PENV_2: i32 = 97;
pub const DMPAPER_PENV_2_ROTATED: i32 = 110;
pub const DMPAPER_PENV_3: i32 = 98;
pub const DMPAPER_PENV_3_ROTATED: i32 = 111;
pub const DMPAPER_PENV_4: i32 = 99;
pub const DMPAPER_PENV_4_ROTATED: i32 = 112;
pub const DMPAPER_PENV_5: i32 = 100;
pub const DMPAPER_PENV_5_ROTATED: i32 = 113;
pub const DMPAPER_PENV_6: i32 = 101;
pub const DMPAPER_PENV_6_ROTATED: i32 = 114;
pub const DMPAPER_PENV_7: i32 = 102;
pub const DMPAPER_PENV_7_ROTATED: i32 = 115;
pub const DMPAPER_PENV_8: i32 = 103;
pub const DMPAPER_PENV_8_ROTATED: i32 = 116;
pub const DMPAPER_PENV_9: i32 = 104;
pub const DMPAPER_PENV_9_ROTATED: i32 = 117;
pub const DMPAPER_QUARTO: i32 = 15;
pub const DMPAPER_RESERVED_48: i32 = 48;
pub const DMPAPER_RESERVED_49: i32 = 49;
pub const DMPAPER_STATEMENT: i32 = 6;
pub const DMPAPER_TABLOID: i32 = 3;
pub const DMPAPER_TABLOID_EXTRA: i32 = 52;
pub const DMPAPER_USER: i32 = 256;
pub const DMRES_DRAFT: i32 = -1;
pub const DMRES_HIGH: i32 = -4;
pub const DMRES_LOW: i32 = -2;
pub const DMRES_MEDIUM: i32 = -3;
pub const DMTT_BITMAP: i32 = 1;
pub const DMTT_DOWNLOAD: i32 = 2;
pub const DMTT_DOWNLOAD_OUTLINE: i32 = 4;
pub const DMTT_SUBDEV: i32 = 3;
pub const DM_BITSPERPEL: i32 = 262144;
pub const DM_COLLATE: i32 = 32768;
pub const DM_COLOR: i32 = 2048;
pub const DM_COPIES: i32 = 256;
pub const DM_COPY: i32 = 2;
pub const DM_DEFAULTSOURCE: i32 = 512;
pub const DM_DISPLAYFIXEDOUTPUT: i32 = 536870912;
pub const DM_DISPLAYFLAGS: i32 = 2097152;
pub const DM_DISPLAYFREQUENCY: i32 = 4194304;
pub const DM_DISPLAYORIENTATION: i32 = 128;
pub const DM_DITHERTYPE: i32 = 67108864;
pub const DM_DUPLEX: i32 = 4096;
pub const DM_FORMNAME: i32 = 65536;
pub const DM_ICMINTENT: i32 = 16777216;
pub const DM_ICMMETHOD: i32 = 8388608;
pub const DM_INTERLACED: i32 = 2;
pub const DM_IN_BUFFER: i32 = 8;
pub const DM_IN_PROMPT: i32 = 4;
pub const DM_LOGPIXELS: i32 = 131072;
pub const DM_MEDIATYPE: i32 = 33554432;
pub const DM_MODIFY: i32 = 8;
pub const DM_NUP: i32 = 64;
pub const DM_ORIENTATION: i32 = 1;
pub const DM_OUT_BUFFER: i32 = 2;
pub const DM_OUT_DEFAULT: i32 = 1;
pub const DM_PANNINGHEIGHT: i32 = 268435456;
pub const DM_PANNINGWIDTH: i32 = 134217728;
pub const DM_PAPERLENGTH: i32 = 4;
pub const DM_PAPERSIZE: i32 = 2;
pub const DM_PAPERWIDTH: i32 = 8;
pub const DM_PELSHEIGHT: i32 = 1048576;
pub const DM_PELSWIDTH: i32 = 524288;
pub const DM_POSITION: i32 = 32;
pub const DM_PRINTQUALITY: i32 = 1024;
pub const DM_PROMPT: i32 = 4;
pub const DM_SCALE: i32 = 16;
pub const DM_SPECVERSION: i32 = 1025;
pub const DM_TTOPTION: i32 = 16384;
pub const DM_UPDATE: i32 = 1;
pub const DM_YRESOLUTION: i32 = 8192;
#[cfg(feature = "shared")]
pub type DOCINFO = DOCINFOA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: windows_sys::core::PCSTR,
    pub lpszOutput: windows_sys::core::PCSTR,
    pub lpszDatatype: windows_sys::core::PCSTR,
    pub fwType: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: windows_sys::core::PCWSTR,
    pub lpszOutput: windows_sys::core::PCWSTR,
    pub lpszDatatype: windows_sys::core::PCWSTR,
    pub fwType: super::DWORD,
}
pub const DOWNLOADFACE: i32 = 514;
pub const DOWNLOADHEADER: i32 = 4111;
pub const DRAFTMODE: i32 = 7;
pub const DRAFT_QUALITY: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct DRAWPATRECT {
    pub ptPosition: super::POINT,
    pub ptSize: super::POINT,
    pub wStyle: super::WORD,
    pub wPattern: super::WORD,
}
pub const DRAWPATTERNRECT: i32 = 25;
pub const DRIVERVERSION: i32 = 0;
#[cfg(feature = "shared")]
pub const DSTINVERT: super::DWORD = 5570569;
pub const DT_CHARSTREAM: i32 = 4;
pub const DT_DISPFILE: i32 = 6;
pub const DT_METAFILE: i32 = 5;
pub const DT_PLOTTER: i32 = 0;
pub const DT_RASCAMERA: i32 = 3;
pub const DT_RASDISPLAY: i32 = 1;
pub const DT_RASPRINTER: i32 = 2;
pub const EASTEUROPE_CHARSET: i32 = 238;
pub const ELF_CULTURE_LATIN: i32 = 0;
pub const ELF_VENDOR_SIZE: i32 = 4;
pub const ELF_VERSION: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMR {
    pub iType: super::DWORD,
    pub nSize: super::DWORD,
}
#[cfg(feature = "shared")]
pub type EMRABORTPATH = tagABORTPATH;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRALPHABLEND {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub cxDest: super::LONG,
    pub cyDest: super::LONG,
    pub dwRop: super::DWORD,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: super::DWORD,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub cxSrc: super::LONG,
    pub cySrc: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRANGLEARC {
    pub emr: EMR,
    pub ptlCenter: super::POINTL,
    pub nRadius: super::DWORD,
    pub eStartAngle: super::FLOAT,
    pub eSweepAngle: super::FLOAT,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRARC = tagEMRARC;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRARCTO = tagEMRARC;
#[cfg(feature = "shared")]
pub type EMRBEGINPATH = tagABORTPATH;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRBITBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub cxDest: super::LONG,
    pub cyDest: super::LONG,
    pub dwRop: super::DWORD,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: super::DWORD,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRCHORD = tagEMRARC;
#[cfg(feature = "shared")]
pub type EMRCLOSEFIGURE = tagABORTPATH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRCOLORCORRECTPALETTE {
    pub emr: EMR,
    pub ihPalette: super::DWORD,
    pub nFirstEntry: super::DWORD,
    pub nPalEntries: super::DWORD,
    pub nReserved: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EMRCOLORMATCHTOTARGET {
    pub emr: EMR,
    pub dwAction: super::DWORD,
    pub dwFlags: super::DWORD,
    pub cbName: super::DWORD,
    pub cbData: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for EMRCOLORMATCHTOTARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEBRUSHINDIRECT {
    pub emr: EMR,
    pub ihBrush: super::DWORD,
    pub lb: LOGBRUSH32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATECOLORSPACE {
    pub emr: EMR,
    pub ihCS: super::DWORD,
    pub lcs: LOGCOLORSPACEA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EMRCREATECOLORSPACEW {
    pub emr: EMR,
    pub ihCS: super::DWORD,
    pub lcs: LOGCOLORSPACEW,
    pub dwFlags: super::DWORD,
    pub cbData: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EMRCREATECOLORSPACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEDIBPATTERNBRUSHPT {
    pub emr: EMR,
    pub ihBrush: super::DWORD,
    pub iUsage: super::DWORD,
    pub offBmi: super::DWORD,
    pub cbBmi: super::DWORD,
    pub offBits: super::DWORD,
    pub cbBits: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEMONOBRUSH {
    pub emr: EMR,
    pub ihBrush: super::DWORD,
    pub iUsage: super::DWORD,
    pub offBmi: super::DWORD,
    pub cbBmi: super::DWORD,
    pub offBits: super::DWORD,
    pub cbBits: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEPALETTE {
    pub emr: EMR,
    pub ihPal: super::DWORD,
    pub lgpl: LOGPALETTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRCREATEPEN {
    pub emr: EMR,
    pub ihPen: super::DWORD,
    pub lopn: LOGPEN,
}
#[cfg(feature = "shared")]
pub type EMRDELETECOLORSPACE = tagEMRSETCOLORSPACE;
#[cfg(feature = "shared")]
pub type EMRDELETEOBJECT = tagEMRSELECTOBJECT;
#[cfg(feature = "shared")]
pub type EMRDRAWESCAPE = tagEMREXTESCAPE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRELLIPSE = tagEMRELLIPSE;
#[cfg(feature = "shared")]
pub type EMRENDPATH = tagABORTPATH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMREOF {
    pub emr: EMR,
    pub nPalEntries: super::DWORD,
    pub offPalEntries: super::DWORD,
    pub nSizeLast: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMREXCLUDECLIPRECT = tagEMREXCLUDECLIPRECT;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct EMREXTCREATEFONTINDIRECTW {
    pub emr: EMR,
    pub ihFont: super::DWORD,
    pub elfw: EXTLOGFONTW,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMREXTCREATEPEN {
    pub emr: EMR,
    pub ihPen: super::DWORD,
    pub offBmi: super::DWORD,
    pub cbBmi: super::DWORD,
    pub offBits: super::DWORD,
    pub cbBits: super::DWORD,
    pub elp: EXTLOGPEN32,
}
#[cfg(feature = "shared")]
pub type EMREXTESCAPE = tagEMREXTESCAPE;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMREXTFLOODFILL {
    pub emr: EMR,
    pub ptlStart: super::POINTL,
    pub crColor: super::COLORREF,
    pub iMode: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EMREXTSELECTCLIPRGN {
    pub emr: EMR,
    pub cbRgnData: super::DWORD,
    pub iMode: super::DWORD,
    pub RgnData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for EMREXTSELECTCLIPRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type EMREXTTEXTOUTA = tagEMREXTTEXTOUTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type EMREXTTEXTOUTW = tagEMREXTTEXTOUTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRFILLPATH = tagEMRFILLPATH;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRFILLRGN {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbRgnData: super::DWORD,
    pub ihBrush: super::DWORD,
    pub RgnData: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EMRFILLRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type EMRFLATTENPATH = tagABORTPATH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRFORMAT {
    pub dSignature: super::DWORD,
    pub nVersion: super::DWORD,
    pub cbData: super::DWORD,
    pub offData: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRFRAMERGN {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbRgnData: super::DWORD,
    pub ihBrush: super::DWORD,
    pub szlStroke: super::SIZEL,
    pub RgnData: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EMRFRAMERGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EMRGDICOMMENT {
    pub emr: EMR,
    pub cbData: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for EMRGDICOMMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRGLSBOUNDEDRECORD {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbData: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EMRGLSBOUNDEDRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EMRGLSRECORD {
    pub emr: EMR,
    pub cbData: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for EMRGLSRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRGRADIENTFILL {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub nVer: super::DWORD,
    pub nTri: super::DWORD,
    pub ulMode: super::ULONG,
    pub Ver: [TRIVERTEX; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EMRGRADIENTFILL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRINTERSECTCLIPRECT = tagEMREXCLUDECLIPRECT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRINVERTRGN = tagEMRINVERTRGN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRLINETO = tagEMRLINETO;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRMASKBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub cxDest: super::LONG,
    pub cyDest: super::LONG,
    pub dwRop: super::DWORD,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: super::DWORD,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub xMask: super::LONG,
    pub yMask: super::LONG,
    pub iUsageMask: super::DWORD,
    pub offBmiMask: super::DWORD,
    pub cbBmiMask: super::DWORD,
    pub offBitsMask: super::DWORD,
    pub cbBitsMask: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct EMRMODIFYWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
    pub iMode: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRMOVETOEX = tagEMRLINETO;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EMRNAMEDESCAPE {
    pub emr: EMR,
    pub iEscape: super::INT,
    pub cbDriver: super::INT,
    pub cbEscData: super::INT,
    pub EscData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for EMRNAMEDESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMROFFSETCLIPRGN {
    pub emr: EMR,
    pub ptlOffset: super::POINTL,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPAINTRGN = tagEMRINVERTRGN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPIE = tagEMRARC;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRPIXELFORMAT {
    pub emr: EMR,
    pub pfd: PIXELFORMATDESCRIPTOR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRPLGBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub aptlDest: [super::POINTL; 3],
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub cxSrc: super::LONG,
    pub cySrc: super::LONG,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: super::DWORD,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub xMask: super::LONG,
    pub yMask: super::LONG,
    pub iUsageMask: super::DWORD,
    pub offBmiMask: super::DWORD,
    pub cbBmiMask: super::DWORD,
    pub offBitsMask: super::DWORD,
    pub cbBitsMask: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl Default for EMRPLGBLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYBEZIER = tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYBEZIER16 = tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYBEZIERTO = tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYBEZIERTO16 = tagEMRPOLYLINE16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRPOLYDRAW {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cptl: super::DWORD,
    pub aptl: [super::POINTL; 1],
    pub abTypes: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EMRPOLYDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EMRPOLYDRAW16 {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cpts: super::DWORD,
    pub apts: [super::POINTS; 1],
    pub abTypes: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EMRPOLYDRAW16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYGON = tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYGON16 = tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYLINE = tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYLINE16 = tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYLINETO = tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYLINETO16 = tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYPOLYGON = tagEMRPOLYPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYPOLYGON16 = tagEMRPOLYPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYPOLYLINE = tagEMRPOLYPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRPOLYPOLYLINE16 = tagEMRPOLYPOLYLINE16;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type EMRPOLYTEXTOUTA = tagEMRPOLYTEXTOUTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type EMRPOLYTEXTOUTW = tagEMRPOLYTEXTOUTA;
#[cfg(feature = "shared")]
pub type EMRREALIZEPALETTE = tagABORTPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRRECTANGLE = tagEMRELLIPSE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRRESIZEPALETTE {
    pub emr: EMR,
    pub ihPal: super::DWORD,
    pub cEntries: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRRESTOREDC {
    pub emr: EMR,
    pub iRelative: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRROUNDRECT {
    pub emr: EMR,
    pub rclBox: super::RECTL,
    pub szlCorner: super::SIZEL,
}
#[cfg(feature = "shared")]
pub type EMRSAVEDC = tagABORTPATH;
#[cfg(feature = "shared")]
pub type EMRSCALEVIEWPORTEXTEX = tagEMRSCALEVIEWPORTEXTEX;
#[cfg(feature = "shared")]
pub type EMRSCALEWINDOWEXTEX = tagEMRSCALEVIEWPORTEXTEX;
#[cfg(feature = "shared")]
pub type EMRSELECTCLIPPATH = tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type EMRSELECTCOLORSPACE = tagEMRSETCOLORSPACE;
#[cfg(feature = "shared")]
pub type EMRSELECTOBJECT = tagEMRSELECTOBJECT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRSELECTPALETTE {
    pub emr: EMR,
    pub ihPal: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETARCDIRECTION {
    pub emr: EMR,
    pub iArcDirection: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETBKCOLOR = tagEMRSETTEXTCOLOR;
#[cfg(feature = "shared")]
pub type EMRSETBKMODE = tagEMRSELECTCLIPPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETBRUSHORGEX = tagEMRSETVIEWPORTORGEX;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETCOLORADJUSTMENT {
    pub emr: EMR,
    pub ColorAdjustment: COLORADJUSTMENT,
}
#[cfg(feature = "shared")]
pub type EMRSETCOLORSPACE = tagEMRSETCOLORSPACE;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRSETDIBITSTODEVICE {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub cxSrc: super::LONG,
    pub cySrc: super::LONG,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub iUsageSrc: super::DWORD,
    pub iStartScan: super::DWORD,
    pub cScans: super::DWORD,
}
#[cfg(feature = "shared")]
pub type EMRSETICMMODE = tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type EMRSETICMPROFILE = tagEMRSETICMPROFILE;
#[cfg(feature = "shared")]
pub type EMRSETICMPROFILEA = tagEMRSETICMPROFILE;
#[cfg(feature = "shared")]
pub type EMRSETICMPROFILEW = tagEMRSETICMPROFILE;
#[cfg(feature = "shared")]
pub type EMRSETLAYOUT = tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type EMRSETMAPMODE = tagEMRSELECTCLIPPATH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EMRSETMAPPERFLAGS {
    pub emr: EMR,
    pub dwFlags: super::DWORD,
}
#[cfg(feature = "shared")]
pub type EMRSETMETARGN = tagABORTPATH;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct EMRSETMITERLIMIT {
    pub emr: EMR,
    pub eMiterLimit: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EMRSETPALETTEENTRIES {
    pub emr: EMR,
    pub ihPal: super::DWORD,
    pub iStart: super::DWORD,
    pub cEntries: super::DWORD,
    pub aPalEntries: [PALETTEENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for EMRSETPALETTEENTRIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRSETPIXELV {
    pub emr: EMR,
    pub ptlPixel: super::POINTL,
    pub crColor: super::COLORREF,
}
#[cfg(feature = "shared")]
pub type EMRSETPOLYFILLMODE = tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type EMRSETROP2 = tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type EMRSETSTRETCHBLTMODE = tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type EMRSETTEXTALIGN = tagEMRSELECTCLIPPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETTEXTCOLOR = tagEMRSETTEXTCOLOR;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETVIEWPORTEXTEX = tagEMRSETVIEWPORTEXTEX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETVIEWPORTORGEX = tagEMRSETVIEWPORTORGEX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETWINDOWEXTEX = tagEMRSETVIEWPORTEXTEX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSETWINDOWORGEX = tagEMRSETVIEWPORTORGEX;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct EMRSETWORLDTRANSFORM {
    pub emr: EMR,
    pub xform: XFORM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRSTRETCHBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub cxDest: super::LONG,
    pub cyDest: super::LONG,
    pub dwRop: super::DWORD,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: super::DWORD,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub cxSrc: super::LONG,
    pub cySrc: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRSTRETCHDIBITS {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub cxSrc: super::LONG,
    pub cySrc: super::LONG,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub iUsageSrc: super::DWORD,
    pub dwRop: super::DWORD,
    pub cxDest: super::LONG,
    pub cyDest: super::LONG,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSTROKEANDFILLPATH = tagEMRFILLPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type EMRSTROKEPATH = tagEMRFILLPATH;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRTEXT {
    pub ptlReference: super::POINTL,
    pub nChars: super::DWORD,
    pub offString: super::DWORD,
    pub fOptions: super::DWORD,
    pub rcl: super::RECTL,
    pub offDx: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct EMRTRANSPARENTBLT {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub xDest: super::LONG,
    pub yDest: super::LONG,
    pub cxDest: super::LONG,
    pub cyDest: super::LONG,
    pub dwRop: super::DWORD,
    pub xSrc: super::LONG,
    pub ySrc: super::LONG,
    pub xformSrc: XFORM,
    pub crBkColorSrc: super::COLORREF,
    pub iUsageSrc: super::DWORD,
    pub offBmiSrc: super::DWORD,
    pub cbBmiSrc: super::DWORD,
    pub offBitsSrc: super::DWORD,
    pub cbBitsSrc: super::DWORD,
    pub cxSrc: super::LONG,
    pub cySrc: super::LONG,
}
#[cfg(feature = "shared")]
pub type EMRWIDENPATH = tagABORTPATH;
pub const EMR_ABORTPATH: i32 = 68;
pub const EMR_ALPHABLEND: i32 = 114;
pub const EMR_ANGLEARC: i32 = 41;
pub const EMR_ARC: i32 = 45;
pub const EMR_ARCTO: i32 = 55;
pub const EMR_BEGINPATH: i32 = 59;
pub const EMR_BITBLT: i32 = 76;
pub const EMR_CHORD: i32 = 46;
pub const EMR_CLOSEFIGURE: i32 = 61;
pub const EMR_COLORCORRECTPALETTE: i32 = 111;
pub const EMR_COLORMATCHTOTARGETW: i32 = 121;
pub const EMR_CREATEBRUSHINDIRECT: i32 = 39;
pub const EMR_CREATECOLORSPACE: i32 = 99;
pub const EMR_CREATECOLORSPACEW: i32 = 122;
pub const EMR_CREATEDIBPATTERNBRUSHPT: i32 = 94;
pub const EMR_CREATEMONOBRUSH: i32 = 93;
pub const EMR_CREATEPALETTE: i32 = 49;
pub const EMR_CREATEPEN: i32 = 38;
pub const EMR_DELETECOLORSPACE: i32 = 101;
pub const EMR_DELETEOBJECT: i32 = 40;
pub const EMR_ELLIPSE: i32 = 42;
pub const EMR_ENDPATH: i32 = 60;
pub const EMR_EOF: i32 = 14;
pub const EMR_EXCLUDECLIPRECT: i32 = 29;
pub const EMR_EXTCREATEFONTINDIRECTW: i32 = 82;
pub const EMR_EXTCREATEPEN: i32 = 95;
pub const EMR_EXTFLOODFILL: i32 = 53;
pub const EMR_EXTSELECTCLIPRGN: i32 = 75;
pub const EMR_EXTTEXTOUTA: i32 = 83;
pub const EMR_EXTTEXTOUTW: i32 = 84;
pub const EMR_FILLPATH: i32 = 62;
pub const EMR_FILLRGN: i32 = 71;
pub const EMR_FLATTENPATH: i32 = 65;
pub const EMR_FRAMERGN: i32 = 72;
pub const EMR_GDICOMMENT: i32 = 70;
pub const EMR_GLSBOUNDEDRECORD: i32 = 103;
pub const EMR_GLSRECORD: i32 = 102;
pub const EMR_GRADIENTFILL: i32 = 118;
pub const EMR_HEADER: i32 = 1;
pub const EMR_INTERSECTCLIPRECT: i32 = 30;
pub const EMR_INVERTRGN: i32 = 73;
pub const EMR_LINETO: i32 = 54;
pub const EMR_MASKBLT: i32 = 78;
pub const EMR_MAX: i32 = 122;
pub const EMR_MIN: i32 = 1;
pub const EMR_MODIFYWORLDTRANSFORM: i32 = 36;
pub const EMR_MOVETOEX: i32 = 27;
pub const EMR_OFFSETCLIPRGN: i32 = 26;
pub const EMR_PAINTRGN: i32 = 74;
pub const EMR_PIE: i32 = 47;
pub const EMR_PIXELFORMAT: i32 = 104;
pub const EMR_PLGBLT: i32 = 79;
pub const EMR_POLYBEZIER: i32 = 2;
pub const EMR_POLYBEZIER16: i32 = 85;
pub const EMR_POLYBEZIERTO: i32 = 5;
pub const EMR_POLYBEZIERTO16: i32 = 88;
pub const EMR_POLYDRAW: i32 = 56;
pub const EMR_POLYDRAW16: i32 = 92;
pub const EMR_POLYGON: i32 = 3;
pub const EMR_POLYGON16: i32 = 86;
pub const EMR_POLYLINE: i32 = 4;
pub const EMR_POLYLINE16: i32 = 87;
pub const EMR_POLYLINETO: i32 = 6;
pub const EMR_POLYLINETO16: i32 = 89;
pub const EMR_POLYPOLYGON: i32 = 8;
pub const EMR_POLYPOLYGON16: i32 = 91;
pub const EMR_POLYPOLYLINE: i32 = 7;
pub const EMR_POLYPOLYLINE16: i32 = 90;
pub const EMR_POLYTEXTOUTA: i32 = 96;
pub const EMR_POLYTEXTOUTW: i32 = 97;
pub const EMR_REALIZEPALETTE: i32 = 52;
pub const EMR_RECTANGLE: i32 = 43;
pub const EMR_RESERVED_105: i32 = 105;
pub const EMR_RESERVED_106: i32 = 106;
pub const EMR_RESERVED_107: i32 = 107;
pub const EMR_RESERVED_108: i32 = 108;
pub const EMR_RESERVED_109: i32 = 109;
pub const EMR_RESERVED_110: i32 = 110;
pub const EMR_RESERVED_117: i32 = 117;
pub const EMR_RESERVED_119: i32 = 119;
pub const EMR_RESERVED_120: i32 = 120;
pub const EMR_RESIZEPALETTE: i32 = 51;
pub const EMR_RESTOREDC: i32 = 34;
pub const EMR_ROUNDRECT: i32 = 44;
pub const EMR_SAVEDC: i32 = 33;
pub const EMR_SCALEVIEWPORTEXTEX: i32 = 31;
pub const EMR_SCALEWINDOWEXTEX: i32 = 32;
pub const EMR_SELECTCLIPPATH: i32 = 67;
pub const EMR_SELECTOBJECT: i32 = 37;
pub const EMR_SELECTPALETTE: i32 = 48;
pub const EMR_SETARCDIRECTION: i32 = 57;
pub const EMR_SETBKCOLOR: i32 = 25;
pub const EMR_SETBKMODE: i32 = 18;
pub const EMR_SETBRUSHORGEX: i32 = 13;
pub const EMR_SETCOLORADJUSTMENT: i32 = 23;
pub const EMR_SETCOLORSPACE: i32 = 100;
pub const EMR_SETDIBITSTODEVICE: i32 = 80;
pub const EMR_SETICMMODE: i32 = 98;
pub const EMR_SETICMPROFILEA: i32 = 112;
pub const EMR_SETICMPROFILEW: i32 = 113;
pub const EMR_SETLAYOUT: i32 = 115;
pub const EMR_SETMAPMODE: i32 = 17;
pub const EMR_SETMAPPERFLAGS: i32 = 16;
pub const EMR_SETMETARGN: i32 = 28;
pub const EMR_SETMITERLIMIT: i32 = 58;
pub const EMR_SETPALETTEENTRIES: i32 = 50;
pub const EMR_SETPIXELV: i32 = 15;
pub const EMR_SETPOLYFILLMODE: i32 = 19;
pub const EMR_SETROP2: i32 = 20;
pub const EMR_SETSTRETCHBLTMODE: i32 = 21;
pub const EMR_SETTEXTALIGN: i32 = 22;
pub const EMR_SETTEXTCOLOR: i32 = 24;
pub const EMR_SETVIEWPORTEXTEX: i32 = 11;
pub const EMR_SETVIEWPORTORGEX: i32 = 12;
pub const EMR_SETWINDOWEXTEX: i32 = 9;
pub const EMR_SETWINDOWORGEX: i32 = 10;
pub const EMR_SETWORLDTRANSFORM: i32 = 35;
pub const EMR_STRETCHBLT: i32 = 77;
pub const EMR_STRETCHDIBITS: i32 = 81;
pub const EMR_STROKEANDFILLPATH: i32 = 63;
pub const EMR_STROKEPATH: i32 = 64;
pub const EMR_TRANSPARENTBLT: i32 = 116;
pub const EMR_WIDENPATH: i32 = 66;
pub const ENABLEDUPLEX: i32 = 28;
pub const ENABLEPAIRKERNING: i32 = 769;
pub const ENABLERELATIVEWIDTHS: i32 = 768;
pub const ENCAPSULATED_POSTSCRIPT: i32 = 4116;
pub const ENDDOC: i32 = 11;
pub const END_PATH: i32 = 4098;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct ENHMETAHEADER {
    pub iType: super::DWORD,
    pub nSize: super::DWORD,
    pub rclBounds: super::RECTL,
    pub rclFrame: super::RECTL,
    pub dSignature: super::DWORD,
    pub nVersion: super::DWORD,
    pub nBytes: super::DWORD,
    pub nRecords: super::DWORD,
    pub nHandles: super::WORD,
    pub sReserved: super::WORD,
    pub nDescription: super::DWORD,
    pub offDescription: super::DWORD,
    pub nPalEntries: super::DWORD,
    pub szlDevice: super::SIZEL,
    pub szlMillimeters: super::SIZEL,
    pub cbPixelFormat: super::DWORD,
    pub offPixelFormat: super::DWORD,
    pub bOpenGL: super::DWORD,
    pub szlMicrometers: super::SIZEL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENHMETARECORD {
    pub iType: super::DWORD,
    pub nSize: super::DWORD,
    pub dParm: [super::DWORD; 1],
}
#[cfg(feature = "shared")]
impl Default for ENHMETARECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENHMETA_SIGNATURE: i32 = 1179469088;
pub const ENHMETA_STOCK_OBJECT: u32 = 2147483648;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type ENHMFENUMPROC = Option<unsafe extern "system" fn(hdc: super::HDC, lpht: *const HANDLETABLE, lpmr: *const ENHMETARECORD, nhandles: i32, data: super::LPARAM) -> i32>;
#[cfg(feature = "shared")]
pub type ENUMLOGFONT = ENUMLOGFONTA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [super::BYTE; 64],
    pub elfStyle: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl Default for ENUMLOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type ENUMLOGFONTEX = ENUMLOGFONTEXA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTEXA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [super::BYTE; 64],
    pub elfStyle: [super::BYTE; 32],
    pub elfScript: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl Default for ENUMLOGFONTEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type ENUMLOGFONTEXDV = ENUMLOGFONTEXDVA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUMLOGFONTEXDVA {
    pub elfEnumLogfontEx: ENUMLOGFONTEXA,
    pub elfDesignVector: DESIGNVECTOR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENUMLOGFONTEXDVW {
    pub elfEnumLogfontEx: ENUMLOGFONTEXW,
    pub elfDesignVector: DESIGNVECTOR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTEXW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [super::WCHAR; 64],
    pub elfStyle: [super::WCHAR; 32],
    pub elfScript: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ENUMLOGFONTEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ENUMLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [super::WCHAR; 64],
    pub elfStyle: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ENUMLOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENUMPAPERBINS: i32 = 31;
pub const ENUMPAPERMETRICS: i32 = 34;
#[cfg(feature = "shared")]
pub type ENUMTEXTMETRIC = ENUMTEXTMETRICA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUMTEXTMETRICA {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXA,
    pub etmAxesList: AXESLISTA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENUMTEXTMETRICW {
    pub etmNewTextMetricEx: NEWTEXTMETRICEXW,
    pub etmAxesList: AXESLISTW,
}
pub const EPSPRINTING: i32 = 33;
pub const EPS_SIGNATURE: i32 = 1179865157;
pub const ERROR: i32 = 0;
pub const ETO_CLIPPED: i32 = 4;
pub const ETO_GLYPH_INDEX: i32 = 16;
pub const ETO_IGNORELANGUAGE: i32 = 4096;
pub const ETO_NUMERICSLATIN: i32 = 2048;
pub const ETO_NUMERICSLOCAL: i32 = 1024;
pub const ETO_OPAQUE: i32 = 2;
pub const ETO_PDY: i32 = 8192;
pub const ETO_REVERSE_INDEX_MAP: i32 = 65536;
pub const ETO_RTLREADING: i32 = 128;
#[cfg(feature = "shared")]
pub type EXTLOGFONT = EXTLOGFONTA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EXTLOGFONTA {
    pub elfLogFont: LOGFONTA,
    pub elfFullName: [super::BYTE; 64],
    pub elfStyle: [super::BYTE; 32],
    pub elfVersion: super::DWORD,
    pub elfStyleSize: super::DWORD,
    pub elfMatch: super::DWORD,
    pub elfReserved: super::DWORD,
    pub elfVendorId: [super::BYTE; 4],
    pub elfCulture: super::DWORD,
    pub elfPanose: PANOSE,
}
#[cfg(feature = "shared")]
impl Default for EXTLOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct EXTLOGFONTW {
    pub elfLogFont: LOGFONTW,
    pub elfFullName: [super::WCHAR; 64],
    pub elfStyle: [super::WCHAR; 32],
    pub elfVersion: super::DWORD,
    pub elfStyleSize: super::DWORD,
    pub elfMatch: super::DWORD,
    pub elfReserved: super::DWORD,
    pub elfVendorId: [super::BYTE; 4],
    pub elfCulture: super::DWORD,
    pub elfPanose: PANOSE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for EXTLOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EXTLOGPEN {
    pub elpPenStyle: super::DWORD,
    pub elpWidth: super::DWORD,
    pub elpBrushStyle: super::UINT,
    pub elpColor: super::COLORREF,
    pub elpHatch: super::ULONG_PTR,
    pub elpNumEntries: super::DWORD,
    pub elpStyleEntry: [super::DWORD; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EXTLOGPEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct EXTLOGPEN32 {
    pub elpPenStyle: super::DWORD,
    pub elpWidth: super::DWORD,
    pub elpBrushStyle: super::UINT,
    pub elpColor: super::COLORREF,
    pub elpHatch: super::ULONG,
    pub elpNumEntries: super::DWORD,
    pub elpStyleEntry: [super::DWORD; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for EXTLOGPEN32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXTTEXTOUT: i32 = 512;
pub const EXT_DEVICE_CAPS: i32 = 4099;
pub const FEATURESETTING_CUSTPAPER: i32 = 3;
pub const FEATURESETTING_MIRROR: i32 = 4;
pub const FEATURESETTING_NEGATIVE: i32 = 5;
pub const FEATURESETTING_NUP: i32 = 0;
pub const FEATURESETTING_OUTPUT: i32 = 1;
pub const FEATURESETTING_PRIVATE_BEGIN: i32 = 4096;
pub const FEATURESETTING_PRIVATE_END: i32 = 8191;
pub const FEATURESETTING_PROTOCOL: i32 = 6;
pub const FEATURESETTING_PSLEVEL: i32 = 2;
pub const FF_DECORATIVE: i32 = 80;
pub const FF_DONTCARE: i32 = 0;
pub const FF_MODERN: i32 = 48;
pub const FF_ROMAN: i32 = 16;
pub const FF_SCRIPT: i32 = 64;
pub const FF_SWISS: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FIXED {
    pub fract: super::WORD,
    pub value: i16,
}
pub const FIXED_PITCH: i32 = 1;
pub const FLI_GLYPHS: i32 = 262144;
pub const FLI_MASK: i32 = 4155;
pub const FLOODFILLBORDER: i32 = 0;
pub const FLOODFILLSURFACE: i32 = 1;
pub const FLUSHOUTPUT: i32 = 6;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type FONTENUMPROC = Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: super::DWORD, param3: super::LPARAM) -> i32>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type FONTENUMPROCA = Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: super::DWORD, param3: super::LPARAM) -> i32>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type FONTENUMPROCW = Option<unsafe extern "system" fn(param0: *const LOGFONTW, param1: *const TEXTMETRICW, param2: super::DWORD, param3: super::LPARAM) -> i32>;
pub const FONTMAPPER_MAX: i32 = 10;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FONTSIGNATURE {
    pub fsUsb: [super::DWORD; 4],
    pub fsCsb: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for FONTSIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FR_NOT_ENUM: i32 = 32;
pub const FR_PRIVATE: i32 = 16;
pub const FS_ARABIC: i32 = 64;
pub const FS_BALTIC: i32 = 128;
pub const FS_CHINESESIMP: i32 = 262144;
pub const FS_CHINESETRAD: i32 = 1048576;
pub const FS_CYRILLIC: i32 = 4;
pub const FS_GREEK: i32 = 8;
pub const FS_HEBREW: i32 = 32;
pub const FS_JISJAPAN: i32 = 131072;
pub const FS_JOHAB: i32 = 2097152;
pub const FS_LATIN1: i32 = 1;
pub const FS_LATIN2: i32 = 2;
pub const FS_SYMBOL: u32 = 2147483648;
pub const FS_THAI: i32 = 65536;
pub const FS_TURKISH: i32 = 16;
pub const FS_VIETNAMESE: i32 = 256;
pub const FS_WANSUNG: i32 = 524288;
pub const FW_BLACK: i32 = 900;
pub const FW_BOLD: i32 = 700;
pub const FW_DEMIBOLD: i32 = 600;
pub const FW_DONTCARE: i32 = 0;
pub const FW_EXTRABOLD: i32 = 800;
pub const FW_EXTRALIGHT: i32 = 200;
pub const FW_HEAVY: i32 = 900;
pub const FW_LIGHT: i32 = 300;
pub const FW_MEDIUM: i32 = 500;
pub const FW_NORMAL: i32 = 400;
pub const FW_REGULAR: i32 = 400;
pub const FW_SEMIBOLD: i32 = 600;
pub const FW_THIN: i32 = 100;
pub const FW_ULTRABOLD: i32 = 800;
pub const FW_ULTRALIGHT: i32 = 200;
pub type FXPT16DOT16 = i32;
pub type FXPT2DOT30 = i32;
pub const GB2312_CHARSET: i32 = 134;
pub const GCPCLASS_ARABIC: i32 = 2;
pub const GCPCLASS_HEBREW: i32 = 2;
pub const GCPCLASS_LATIN: i32 = 1;
pub const GCPCLASS_LATINNUMBER: i32 = 5;
pub const GCPCLASS_LATINNUMERICSEPARATOR: i32 = 7;
pub const GCPCLASS_LATINNUMERICTERMINATOR: i32 = 6;
pub const GCPCLASS_LOCALNUMBER: i32 = 4;
pub const GCPCLASS_NEUTRAL: i32 = 3;
pub const GCPCLASS_NUMERICSEPARATOR: i32 = 8;
pub const GCPCLASS_POSTBOUNDLTR: i32 = 32;
pub const GCPCLASS_POSTBOUNDRTL: i32 = 16;
pub const GCPCLASS_PREBOUNDLTR: i32 = 128;
pub const GCPCLASS_PREBOUNDRTL: i32 = 64;
pub const GCPGLYPH_LINKAFTER: i32 = 16384;
pub const GCPGLYPH_LINKBEFORE: i32 = 32768;
pub const GCP_CLASSIN: i32 = 524288;
pub const GCP_DBCS: i32 = 1;
pub const GCP_DIACRITIC: i32 = 256;
pub const GCP_DISPLAYZWG: i32 = 4194304;
pub const GCP_ERROR: i32 = 32768;
pub const GCP_GLYPHSHAPE: i32 = 16;
pub const GCP_JUSTIFY: i32 = 65536;
pub const GCP_JUSTIFYIN: i32 = 2097152;
pub const GCP_KASHIDA: i32 = 1024;
pub const GCP_LIGATE: i32 = 32;
pub const GCP_MAXEXTENT: i32 = 1048576;
pub const GCP_NEUTRALOVERRIDE: i32 = 33554432;
pub const GCP_NUMERICOVERRIDE: i32 = 16777216;
pub const GCP_NUMERICSLATIN: i32 = 67108864;
pub const GCP_NUMERICSLOCAL: i32 = 134217728;
pub const GCP_REORDER: i32 = 2;
#[cfg(feature = "shared")]
pub type GCP_RESULTS = GCP_RESULTSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GCP_RESULTSA {
    pub lStructSize: super::DWORD,
    pub lpOutString: windows_sys::core::PSTR,
    pub lpOrder: *mut super::UINT,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: windows_sys::core::PSTR,
    pub lpGlyphs: windows_sys::core::PWSTR,
    pub nGlyphs: super::UINT,
    pub nMaxFit: i32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GCP_RESULTSW {
    pub lStructSize: super::DWORD,
    pub lpOutString: windows_sys::core::PWSTR,
    pub lpOrder: *mut super::UINT,
    pub lpDx: *mut i32,
    pub lpCaretPos: *mut i32,
    pub lpClass: windows_sys::core::PSTR,
    pub lpGlyphs: windows_sys::core::PWSTR,
    pub nGlyphs: super::UINT,
    pub nMaxFit: i32,
}
pub const GCP_SYMSWAPOFF: i32 = 8388608;
pub const GCP_USEKERNING: i32 = 8;
pub const GDICOMMENT_BEGINGROUP: i32 = 2;
pub const GDICOMMENT_ENDGROUP: i32 = 3;
pub const GDICOMMENT_IDENTIFIER: i32 = 1128875079;
pub const GDICOMMENT_MULTIFORMATS: i32 = 1073741828;
pub const GDICOMMENT_UNICODE_END: i32 = 128;
pub const GDICOMMENT_UNICODE_STRING: i32 = 64;
pub const GDICOMMENT_WINDOWS_METAFILE: u32 = 2147483649;
pub const GDIPLUS_TS_QUERYVER: i32 = 4122;
pub const GDIPLUS_TS_RECORD: i32 = 4123;
pub const GDI_ERROR: u32 = 4294967295;
pub const GDI_MAX_OBJ_TYPE: i32 = 14;
pub const GDI_MIN_OBJ_TYPE: i32 = 1;
pub const GDI_OBJ_LAST: i32 = 14;
pub const GETCOLORTABLE: i32 = 5;
pub const GETDEVICEUNITS: i32 = 42;
pub const GETEXTENDEDTEXTMETRICS: i32 = 256;
pub const GETEXTENTTABLE: i32 = 257;
pub const GETFACENAME: i32 = 513;
pub const GETPAIRKERNTABLE: i32 = 258;
pub const GETPENWIDTH: i32 = 16;
pub const GETPHYSPAGESIZE: i32 = 12;
pub const GETPRINTINGOFFSET: i32 = 13;
pub const GETSCALINGFACTOR: i32 = 14;
pub const GETSETPAPERBINS: i32 = 29;
pub const GETSETPAPERMETRICS: i32 = 35;
pub const GETSETPRINTORIENT: i32 = 30;
pub const GETSETSCREENPARAMS: i32 = 3072;
pub const GETTECHNOLGY: i32 = 20;
pub const GETTECHNOLOGY: i32 = 20;
pub const GETTRACKKERNTABLE: i32 = 259;
pub const GETVECTORBRUSHSIZE: i32 = 27;
pub const GETVECTORPENSIZE: i32 = 26;
pub const GET_PS_FEATURESETTING: i32 = 4121;
pub const GGI_MARK_NONEXISTING_GLYPHS: i32 = 1;
pub const GGO_BEZIER: i32 = 3;
pub const GGO_BITMAP: i32 = 1;
pub const GGO_GLYPH_INDEX: i32 = 128;
pub const GGO_GRAY2_BITMAP: i32 = 4;
pub const GGO_GRAY4_BITMAP: i32 = 5;
pub const GGO_GRAY8_BITMAP: i32 = 6;
pub const GGO_METRICS: i32 = 0;
pub const GGO_NATIVE: i32 = 2;
pub const GGO_UNHINTED: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct GLYPHMETRICS {
    pub gmBlackBoxX: super::UINT,
    pub gmBlackBoxY: super::UINT,
    pub gmptGlyphOrigin: super::POINT,
    pub gmCellIncX: i16,
    pub gmCellIncY: i16,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: super::FLOAT,
    pub gmfBlackBoxY: super::FLOAT,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: super::FLOAT,
    pub gmfCellIncY: super::FLOAT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GLYPHSET {
    pub cbThis: super::DWORD,
    pub flAccel: super::DWORD,
    pub cGlyphsSupported: super::DWORD,
    pub cRanges: super::DWORD,
    pub ranges: [WCRANGE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for GLYPHSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GM_ADVANCED: i32 = 2;
pub const GM_COMPATIBLE: i32 = 1;
pub const GM_LAST: i32 = 2;
#[cfg(feature = "minwindef")]
pub type GOBJENUMPROC = Option<unsafe extern "system" fn(param0: super::LPVOID, param1: super::LPARAM) -> i32>;
pub const GRADIENT_FILL_OP_FLAG: i32 = 255;
pub const GRADIENT_FILL_RECT_H: i32 = 0;
pub const GRADIENT_FILL_RECT_V: i32 = 1;
pub const GRADIENT_FILL_TRIANGLE: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GRADIENT_RECT {
    pub UpperLeft: super::ULONG,
    pub LowerRight: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GRADIENT_TRIANGLE {
    pub Vertex1: super::ULONG,
    pub Vertex2: super::ULONG,
    pub Vertex3: super::ULONG,
}
pub const GRAY_BRUSH: i32 = 2;
pub const GREEK_CHARSET: i32 = 161;
pub const GS_8BIT_INDICES: i32 = 1;
pub const HALFTONE: i32 = 4;
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
pub const HANGEUL_CHARSET: i32 = 129;
pub const HANGUL_CHARSET: i32 = 129;
pub const HEBREW_CHARSET: i32 = 177;
pub const HOLLOW_BRUSH: i32 = 5;
pub const HORZRES: i32 = 8;
pub const HORZSIZE: i32 = 4;
pub const HS_API_MAX: i32 = 12;
pub const HS_BDIAGONAL: i32 = 3;
pub const HS_CROSS: i32 = 4;
pub const HS_DIAGCROSS: i32 = 5;
pub const HS_FDIAGONAL: i32 = 2;
pub const HS_HORIZONTAL: i32 = 0;
pub const HS_VERTICAL: i32 = 1;
#[cfg(feature = "minwindef")]
pub type ICMENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PSTR, param1: super::LPARAM) -> i32>;
#[cfg(feature = "minwindef")]
pub type ICMENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PWSTR, param1: super::LPARAM) -> i32>;
pub const ICM_ADDPROFILE: i32 = 1;
pub const ICM_DELETEPROFILE: i32 = 2;
pub const ICM_DONE_OUTSIDEDC: i32 = 4;
pub const ICM_OFF: i32 = 1;
pub const ICM_ON: i32 = 2;
pub const ICM_QUERY: i32 = 3;
pub const ICM_QUERYMATCH: i32 = 7;
pub const ICM_QUERYPROFILE: i32 = 3;
pub const ICM_REGISTERICMATCHER: i32 = 5;
pub const ICM_SETDEFAULTPROFILE: i32 = 4;
pub const ICM_UNREGISTERICMATCHER: i32 = 6;
pub const ILLUMINANT_A: i32 = 1;
pub const ILLUMINANT_B: i32 = 2;
pub const ILLUMINANT_C: i32 = 3;
pub const ILLUMINANT_D50: i32 = 4;
pub const ILLUMINANT_D55: i32 = 5;
pub const ILLUMINANT_D65: i32 = 6;
pub const ILLUMINANT_D75: i32 = 7;
pub const ILLUMINANT_DAYLIGHT: i32 = 3;
pub const ILLUMINANT_DEVICE_DEFAULT: i32 = 0;
pub const ILLUMINANT_F2: i32 = 8;
pub const ILLUMINANT_FLUORESCENT: i32 = 8;
pub const ILLUMINANT_MAX_INDEX: i32 = 8;
pub const ILLUMINANT_NTSC: i32 = 3;
pub const ILLUMINANT_TUNGSTEN: i32 = 1;
pub const JOHAB_CHARSET: i32 = 130;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KERNINGPAIR {
    pub wFirst: super::WORD,
    pub wSecond: super::WORD,
    pub iKernAmount: i32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: super::WORD,
    pub nVersion: super::WORD,
    pub dwFlags: super::DWORD,
    pub iPixelType: super::BYTE,
    pub cColorBits: super::BYTE,
    pub cRedBits: super::BYTE,
    pub cRedShift: super::BYTE,
    pub cGreenBits: super::BYTE,
    pub cGreenShift: super::BYTE,
    pub cBlueBits: super::BYTE,
    pub cBlueShift: super::BYTE,
    pub cAlphaBits: super::BYTE,
    pub cAlphaShift: super::BYTE,
    pub cAccumBits: super::BYTE,
    pub cAccumRedBits: super::BYTE,
    pub cAccumGreenBits: super::BYTE,
    pub cAccumBlueBits: super::BYTE,
    pub cAccumAlphaBits: super::BYTE,
    pub cDepthBits: super::BYTE,
    pub cStencilBits: super::BYTE,
    pub cAuxBuffers: super::BYTE,
    pub iLayerPlane: super::BYTE,
    pub bReserved: super::BYTE,
    pub crTransparent: super::COLORREF,
}
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: i32 = 8;
pub const LAYOUT_BTT: i32 = 2;
pub const LAYOUT_ORIENTATIONMASK: i32 = 7;
pub const LAYOUT_RTL: i32 = 1;
pub const LAYOUT_VBH: i32 = 4;
pub type LCSCSTYPE = i32;
pub type LCSGAMUTMATCH = i32;
pub const LCS_CALIBRATED_RGB: i32 = 0;
pub const LCS_GM_ABS_COLORIMETRIC: i32 = 8;
pub const LCS_GM_BUSINESS: i32 = 1;
pub const LCS_GM_GRAPHICS: i32 = 2;
pub const LCS_GM_IMAGES: i32 = 4;
pub const LCS_SIGNATURE: i32 = 1347637059;
pub const LCS_WINDOWS_COLOR_SPACE: i32 = 1466527264;
pub const LCS_sRGB: i32 = 1934772034;
pub const LC_INTERIORS: i32 = 128;
pub const LC_MARKER: i32 = 4;
pub const LC_NONE: i32 = 0;
pub const LC_POLYLINE: i32 = 2;
pub const LC_POLYMARKER: i32 = 8;
pub const LC_STYLED: i32 = 32;
pub const LC_WIDE: i32 = 16;
pub const LC_WIDESTYLED: i32 = 64;
pub const LF_FACESIZE: i32 = 32;
pub const LF_FULLFACESIZE: i32 = 64;
pub const LINECAPS: i32 = 30;
#[cfg(feature = "minwindef")]
pub type LINEDDAPROC = Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: super::LPARAM)>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct LOCALESIGNATURE {
    pub lsUsb: [super::DWORD; 4],
    pub lsCsbDefault: [super::DWORD; 2],
    pub lsCsbSupported: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for LOCALESIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct LOGBRUSH {
    pub lbStyle: super::UINT,
    pub lbColor: super::COLORREF,
    pub lbHatch: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct LOGBRUSH32 {
    pub lbStyle: super::UINT,
    pub lbColor: super::COLORREF,
    pub lbHatch: super::ULONG,
}
#[cfg(feature = "shared")]
pub type LOGCOLORSPACE = LOGCOLORSPACEA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: super::DWORD,
    pub lcsVersion: super::DWORD,
    pub lcsSize: super::DWORD,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: LCSGAMUTMATCH,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: super::DWORD,
    pub lcsGammaGreen: super::DWORD,
    pub lcsGammaBlue: super::DWORD,
    pub lcsFilename: [super::CHAR; 260],
}
#[cfg(feature = "shared")]
impl Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: super::DWORD,
    pub lcsVersion: super::DWORD,
    pub lcsSize: super::DWORD,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: LCSGAMUTMATCH,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: super::DWORD,
    pub lcsGammaGreen: super::DWORD,
    pub lcsGammaBlue: super::DWORD,
    pub lcsFilename: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type LOGFONT = LOGFONTA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct LOGFONTA {
    pub lfHeight: super::LONG,
    pub lfWidth: super::LONG,
    pub lfEscapement: super::LONG,
    pub lfOrientation: super::LONG,
    pub lfWeight: super::LONG,
    pub lfItalic: super::BYTE,
    pub lfUnderline: super::BYTE,
    pub lfStrikeOut: super::BYTE,
    pub lfCharSet: super::BYTE,
    pub lfOutPrecision: super::BYTE,
    pub lfClipPrecision: super::BYTE,
    pub lfQuality: super::BYTE,
    pub lfPitchAndFamily: super::BYTE,
    pub lfFaceName: [super::CHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for LOGFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LOGFONTW {
    pub lfHeight: super::LONG,
    pub lfWidth: super::LONG,
    pub lfEscapement: super::LONG,
    pub lfOrientation: super::LONG,
    pub lfWeight: super::LONG,
    pub lfItalic: super::BYTE,
    pub lfUnderline: super::BYTE,
    pub lfStrikeOut: super::BYTE,
    pub lfCharSet: super::BYTE,
    pub lfOutPrecision: super::BYTE,
    pub lfClipPrecision: super::BYTE,
    pub lfQuality: super::BYTE,
    pub lfPitchAndFamily: super::BYTE,
    pub lfFaceName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for LOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct LOGPALETTE {
    pub palVersion: super::WORD,
    pub palNumEntries: super::WORD,
    pub palPalEntry: [PALETTEENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for LOGPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct LOGPEN {
    pub lopnStyle: super::UINT,
    pub lopnWidth: super::POINT,
    pub lopnColor: super::COLORREF,
}
pub const LOGPIXELSX: i32 = 88;
pub const LOGPIXELSY: i32 = 90;
#[cfg(feature = "shared")]
pub type LPABC = *mut ABC;
#[cfg(feature = "minwindef")]
pub type LPABCFLOAT = *mut ABCFLOAT;
#[cfg(feature = "shared")]
pub type LPAXESLIST = *mut AXESLISTA;
#[cfg(feature = "shared")]
pub type LPAXESLISTA = *mut AXESLISTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAXESLISTW = *mut AXESLISTW;
#[cfg(feature = "shared")]
pub type LPAXISINFO = *mut AXISINFOA;
#[cfg(feature = "shared")]
pub type LPAXISINFOA = *mut AXISINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAXISINFOW = *mut AXISINFOW;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBITMAP = *mut BITMAP;
#[cfg(feature = "shared")]
pub type LPBITMAPCOREHEADER = *mut BITMAPCOREHEADER;
#[cfg(feature = "shared")]
pub type LPBITMAPCOREINFO = *mut BITMAPCOREINFO;
#[cfg(feature = "shared")]
pub type LPBITMAPFILEHEADER = *mut BITMAPFILEHEADER;
#[cfg(feature = "shared")]
pub type LPBITMAPINFO = *mut BITMAPINFO;
#[cfg(feature = "shared")]
pub type LPBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
#[cfg(feature = "shared")]
pub type LPBITMAPV4HEADER = *mut BITMAPV4HEADER;
#[cfg(feature = "shared")]
pub type LPBITMAPV5HEADER = *mut BITMAPV5HEADER;
#[cfg(feature = "shared")]
pub type LPCHARSETINFO = *mut CHARSETINFO;
pub type LPCIEXYZ = *mut CIEXYZ;
pub type LPCIEXYZTRIPLE = *mut CIEXYZTRIPLE;
#[cfg(feature = "shared")]
pub type LPCOLORADJUSTMENT = *mut COLORADJUSTMENT;
#[cfg(feature = "shared")]
pub type LPDESIGNVECTOR = *mut DESIGNVECTOR;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPDEVMODE = *mut DEVMODEA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPDEVMODEA = *mut DEVMODEA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPDEVMODEW = *mut DEVMODEW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPDIBSECTION = *mut DIBSECTION;
#[cfg(feature = "shared")]
pub type LPDISPLAY_DEVICE = *mut DISPLAY_DEVICEA;
#[cfg(feature = "shared")]
pub type LPDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[cfg(feature = "shared")]
pub type LPDOCINFO = *mut DOCINFOA;
#[cfg(feature = "shared")]
pub type LPDOCINFOA = *mut DOCINFOA;
#[cfg(feature = "shared")]
pub type LPDOCINFOW = *mut DOCINFOW;
pub const LPD_DOUBLEBUFFER: i32 = 1;
pub const LPD_SHARE_ACCUM: i32 = 256;
pub const LPD_SHARE_DEPTH: i32 = 64;
pub const LPD_SHARE_STENCIL: i32 = 128;
pub const LPD_STEREO: i32 = 2;
pub const LPD_SUPPORT_GDI: i32 = 16;
pub const LPD_SUPPORT_OPENGL: i32 = 32;
pub const LPD_SWAP_COPY: i32 = 1024;
pub const LPD_SWAP_EXCHANGE: i32 = 512;
pub const LPD_TRANSPARENT: i32 = 4096;
pub const LPD_TYPE_COLORINDEX: i32 = 1;
pub const LPD_TYPE_RGBA: i32 = 0;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPENHMETAHEADER = *mut ENHMETAHEADER;
#[cfg(feature = "shared")]
pub type LPENHMETARECORD = *mut ENHMETARECORD;
#[cfg(feature = "shared")]
pub type LPENUMLOGFONT = *mut ENUMLOGFONTA;
#[cfg(feature = "shared")]
pub type LPENUMLOGFONTA = *mut ENUMLOGFONTA;
#[cfg(feature = "shared")]
pub type LPENUMLOGFONTEX = *mut ENUMLOGFONTEXA;
#[cfg(feature = "shared")]
pub type LPENUMLOGFONTEXA = *mut ENUMLOGFONTEXA;
#[cfg(feature = "shared")]
pub type LPENUMLOGFONTEXDV = *mut ENUMLOGFONTEXDVA;
#[cfg(feature = "shared")]
pub type LPENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPENUMLOGFONTEXW = *mut ENUMLOGFONTEXW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPENUMLOGFONTW = *mut ENUMLOGFONTW;
#[cfg(feature = "shared")]
pub type LPENUMTEXTMETRIC = *mut ENUMTEXTMETRICA;
#[cfg(feature = "shared")]
pub type LPENUMTEXTMETRICA = *mut ENUMTEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPENUMTEXTMETRICW = *mut ENUMTEXTMETRICW;
#[cfg(feature = "shared")]
pub type LPEXTLOGFONT = *mut EXTLOGFONTA;
#[cfg(feature = "shared")]
pub type LPEXTLOGFONTA = *mut EXTLOGFONTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPEXTLOGFONTW = *mut EXTLOGFONTW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPEXTLOGPEN = *mut EXTLOGPEN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPEXTLOGPEN32 = *mut EXTLOGPEN32;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPFNDEVCAPS = Option<unsafe extern "system" fn(param0: windows_sys::core::PSTR, param1: windows_sys::core::PSTR, param2: super::UINT, param3: windows_sys::core::PSTR, param4: LPDEVMODE) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFNDEVMODE = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::HMODULE, param2: LPDEVMODE, param3: windows_sys::core::PSTR, param4: windows_sys::core::PSTR, param5: LPDEVMODE, param6: windows_sys::core::PSTR, param7: super::UINT) -> super::UINT>;
#[cfg(feature = "shared")]
pub type LPFONTSIGNATURE = *mut FONTSIGNATURE;
pub type LPFXPT16DOT16 = *mut i32;
pub type LPFXPT2DOT30 = *mut i32;
#[cfg(feature = "shared")]
pub type LPGCP_RESULTS = *mut GCP_RESULTSA;
#[cfg(feature = "shared")]
pub type LPGCP_RESULTSA = *mut GCP_RESULTSA;
#[cfg(feature = "shared")]
pub type LPGCP_RESULTSW = *mut GCP_RESULTSW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPGLYPHMETRICS = *mut GLYPHMETRICS;
#[cfg(feature = "minwindef")]
pub type LPGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPGLYPHSET = *mut GLYPHSET;
#[cfg(feature = "shared")]
pub type LPGRADIENT_RECT = *mut GRADIENT_RECT;
#[cfg(feature = "shared")]
pub type LPGRADIENT_TRIANGLE = *mut GRADIENT_TRIANGLE;
#[cfg(feature = "windef")]
pub type LPHANDLETABLE = *mut HANDLETABLE;
#[cfg(feature = "shared")]
pub type LPKERNINGPAIR = *mut KERNINGPAIR;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
#[cfg(feature = "shared")]
pub type LPLOCALESIGNATURE = *mut LOCALESIGNATURE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPLOGBRUSH = *mut LOGBRUSH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPLOGBRUSH32 = *mut LOGBRUSH32;
#[cfg(feature = "shared")]
pub type LPLOGCOLORSPACE = *mut LOGCOLORSPACEA;
#[cfg(feature = "shared")]
pub type LPLOGCOLORSPACEA = *mut LOGCOLORSPACEA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPLOGCOLORSPACEW = *mut LOGCOLORSPACEW;
#[cfg(feature = "shared")]
pub type LPLOGFONT = *mut LOGFONTA;
#[cfg(feature = "shared")]
pub type LPLOGFONTA = *mut LOGFONTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPLOGFONTW = *mut LOGFONTW;
#[cfg(feature = "shared")]
pub type LPLOGPALETTE = *mut LOGPALETTE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPLOGPEN = *mut LOGPEN;
#[cfg(feature = "shared")]
pub type LPMAT2 = *mut MAT2;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPMETAFILEPICT = *mut METAFILEPICT;
#[cfg(feature = "shared")]
pub type LPNEWTEXTMETRIC = *mut NEWTEXTMETRICA;
#[cfg(feature = "shared")]
pub type LPNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPOUTLINETEXTMETRIC = *mut OUTLINETEXTMETRICA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
#[cfg(feature = "shared")]
pub type LPPALETTEENTRY = *mut PALETTEENTRY;
#[cfg(feature = "shared")]
pub type LPPANOSE = *mut PANOSE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPPATTERN = *mut LOGBRUSH;
#[cfg(feature = "shared")]
pub type LPPELARRAY = *mut PELARRAY;
#[cfg(feature = "shared")]
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
#[cfg(feature = "shared")]
pub type LPPOINTFX = *mut POINTFX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPPOLYTEXT = *mut POLYTEXTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPPOLYTEXTA = *mut POLYTEXTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPPOLYTEXTW = *mut POLYTEXTW;
pub type LPRASTERIZER_STATUS = *mut RASTERIZER_STATUS;
#[cfg(feature = "shared")]
pub type LPRGBQUAD = *mut RGBQUAD;
#[cfg(feature = "shared")]
pub type LPRGBTRIPLE = *mut RGBTRIPLE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPRGNDATA = *mut RGNDATA;
#[cfg(feature = "shared")]
pub type LPTEXTMETRIC = *mut TEXTMETRICA;
#[cfg(feature = "shared")]
pub type LPTEXTMETRICA = *mut TEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPTEXTMETRICW = *mut TEXTMETRICW;
#[cfg(feature = "shared")]
pub type LPTRIVERTEX = *mut TRIVERTEX;
#[cfg(feature = "shared")]
pub type LPTTPOLYCURVE = *mut TTPOLYCURVE;
#[cfg(feature = "shared")]
pub type LPTTPOLYGONHEADER = *mut TTPOLYGONHEADER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPWCRANGE = *mut WCRANGE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPWGLSWAP = *mut WGLSWAP;
#[cfg(feature = "minwindef")]
pub type LPXFORM = *mut XFORM;
pub const LTGRAY_BRUSH: i32 = 1;
pub const MAC_CHARSET: i32 = 77;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MAT2 {
    pub eM11: FIXED,
    pub eM12: FIXED,
    pub eM21: FIXED,
    pub eM22: FIXED,
}
pub const MAXSTRETCHBLTMODE: i32 = 4;
#[cfg(feature = "shared")]
pub const MERGECOPY: super::DWORD = 12583114;
#[cfg(feature = "shared")]
pub const MERGEPAINT: super::DWORD = 12255782;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct METAFILEPICT {
    pub mm: super::LONG,
    pub xExt: super::LONG,
    pub yExt: super::LONG,
    pub hMF: super::HMETAFILE,
}
pub const METAFILE_DRIVER: i32 = 2049;
#[repr(C, packed(2))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct METAHEADER {
    pub mtType: super::WORD,
    pub mtHeaderSize: super::WORD,
    pub mtVersion: super::WORD,
    pub mtSize: super::DWORD,
    pub mtNoObjects: super::WORD,
    pub mtMaxRecord: super::DWORD,
    pub mtNoParameters: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct METARECORD {
    pub rdSize: super::DWORD,
    pub rdFunction: super::WORD,
    pub rdParm: [super::WORD; 1],
}
#[cfg(feature = "shared")]
impl Default for METARECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const META_ANIMATEPALETTE: i32 = 1078;
pub const META_ARC: i32 = 2071;
pub const META_BITBLT: i32 = 2338;
pub const META_CHORD: i32 = 2096;
pub const META_CREATEBRUSHINDIRECT: i32 = 764;
pub const META_CREATEFONTINDIRECT: i32 = 763;
pub const META_CREATEPALETTE: i32 = 247;
pub const META_CREATEPATTERNBRUSH: i32 = 505;
pub const META_CREATEPENINDIRECT: i32 = 762;
pub const META_CREATEREGION: i32 = 1791;
pub const META_DELETEOBJECT: i32 = 496;
pub const META_DIBBITBLT: i32 = 2368;
pub const META_DIBCREATEPATTERNBRUSH: i32 = 322;
pub const META_DIBSTRETCHBLT: i32 = 2881;
pub const META_ELLIPSE: i32 = 1048;
pub const META_ESCAPE: i32 = 1574;
pub const META_EXCLUDECLIPRECT: i32 = 1045;
pub const META_EXTFLOODFILL: i32 = 1352;
pub const META_EXTTEXTOUT: i32 = 2610;
pub const META_FILLREGION: i32 = 552;
pub const META_FLOODFILL: i32 = 1049;
pub const META_FRAMEREGION: i32 = 1065;
pub const META_INTERSECTCLIPRECT: i32 = 1046;
pub const META_INVERTREGION: i32 = 298;
pub const META_LINETO: i32 = 531;
pub const META_MOVETO: i32 = 532;
pub const META_OFFSETCLIPRGN: i32 = 544;
pub const META_OFFSETVIEWPORTORG: i32 = 529;
pub const META_OFFSETWINDOWORG: i32 = 527;
pub const META_PAINTREGION: i32 = 299;
pub const META_PATBLT: i32 = 1565;
pub const META_PIE: i32 = 2074;
pub const META_POLYGON: i32 = 804;
pub const META_POLYLINE: i32 = 805;
pub const META_POLYPOLYGON: i32 = 1336;
pub const META_REALIZEPALETTE: i32 = 53;
pub const META_RECTANGLE: i32 = 1051;
pub const META_RESIZEPALETTE: i32 = 313;
pub const META_RESTOREDC: i32 = 295;
pub const META_ROUNDRECT: i32 = 1564;
pub const META_SAVEDC: i32 = 30;
pub const META_SCALEVIEWPORTEXT: i32 = 1042;
pub const META_SCALEWINDOWEXT: i32 = 1040;
pub const META_SELECTCLIPREGION: i32 = 300;
pub const META_SELECTOBJECT: i32 = 301;
pub const META_SELECTPALETTE: i32 = 564;
pub const META_SETBKCOLOR: i32 = 513;
pub const META_SETBKMODE: i32 = 258;
pub const META_SETDIBTODEV: i32 = 3379;
pub const META_SETLAYOUT: i32 = 329;
pub const META_SETMAPMODE: i32 = 259;
pub const META_SETMAPPERFLAGS: i32 = 561;
pub const META_SETPALENTRIES: i32 = 55;
pub const META_SETPIXEL: i32 = 1055;
pub const META_SETPOLYFILLMODE: i32 = 262;
pub const META_SETRELABS: i32 = 261;
pub const META_SETROP2: i32 = 260;
pub const META_SETSTRETCHBLTMODE: i32 = 263;
pub const META_SETTEXTALIGN: i32 = 302;
pub const META_SETTEXTCHAREXTRA: i32 = 264;
pub const META_SETTEXTCOLOR: i32 = 521;
pub const META_SETTEXTJUSTIFICATION: i32 = 522;
pub const META_SETVIEWPORTEXT: i32 = 526;
pub const META_SETVIEWPORTORG: i32 = 525;
pub const META_SETWINDOWEXT: i32 = 524;
pub const META_SETWINDOWORG: i32 = 523;
pub const META_STRETCHBLT: i32 = 2851;
pub const META_STRETCHDIB: i32 = 3907;
pub const META_TEXTOUT: i32 = 1313;
pub const MFCOMMENT: i32 = 15;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type MFENUMPROC = Option<unsafe extern "system" fn(hdc: super::HDC, lpht: *const HANDLETABLE, lpmr: *const METARECORD, nobj: i32, param: super::LPARAM) -> i32>;
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: i32 = 0;
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: i32 = 2147483647;
pub const MM_ANISOTROPIC: i32 = 8;
pub const MM_HIENGLISH: i32 = 5;
pub const MM_HIMETRIC: i32 = 3;
pub const MM_ISOTROPIC: i32 = 7;
pub const MM_LOENGLISH: i32 = 4;
pub const MM_LOMETRIC: i32 = 2;
pub const MM_MAX: i32 = 8;
pub const MM_MAX_AXES_NAMELEN: i32 = 16;
pub const MM_MAX_FIXEDSCALE: i32 = 6;
pub const MM_MAX_NUMAXES: i32 = 16;
pub const MM_MIN: i32 = 1;
pub const MM_TEXT: i32 = 1;
pub const MM_TWIPS: i32 = 6;
pub const MONO_FONT: i32 = 8;
pub const MOUSETRAILS: i32 = 39;
pub const MWT_IDENTITY: i32 = 1;
pub const MWT_LEFTMULTIPLY: i32 = 2;
pub const MWT_MAX: i32 = 3;
pub const MWT_MIN: i32 = 1;
pub const MWT_RIGHTMULTIPLY: i32 = 3;
pub const NEWFRAME: i32 = 1;
#[cfg(feature = "shared")]
pub type NEWTEXTMETRIC = NEWTEXTMETRICA;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NEWTEXTMETRICA {
    pub tmHeight: super::LONG,
    pub tmAscent: super::LONG,
    pub tmDescent: super::LONG,
    pub tmInternalLeading: super::LONG,
    pub tmExternalLeading: super::LONG,
    pub tmAveCharWidth: super::LONG,
    pub tmMaxCharWidth: super::LONG,
    pub tmWeight: super::LONG,
    pub tmOverhang: super::LONG,
    pub tmDigitizedAspectX: super::LONG,
    pub tmDigitizedAspectY: super::LONG,
    pub tmFirstChar: super::BYTE,
    pub tmLastChar: super::BYTE,
    pub tmDefaultChar: super::BYTE,
    pub tmBreakChar: super::BYTE,
    pub tmItalic: super::BYTE,
    pub tmUnderlined: super::BYTE,
    pub tmStruckOut: super::BYTE,
    pub tmPitchAndFamily: super::BYTE,
    pub tmCharSet: super::BYTE,
    pub ntmFlags: super::DWORD,
    pub ntmSizeEM: super::UINT,
    pub ntmCellHeight: super::UINT,
    pub ntmAvgWidth: super::UINT,
}
#[cfg(feature = "shared")]
pub type NEWTEXTMETRICEX = NEWTEXTMETRICEXA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NEWTEXTMETRICEXA {
    pub ntmTm: NEWTEXTMETRICA,
    pub ntmFontSig: FONTSIGNATURE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct NEWTEXTMETRICEXW {
    pub ntmTm: NEWTEXTMETRICW,
    pub ntmFontSig: FONTSIGNATURE,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct NEWTEXTMETRICW {
    pub tmHeight: super::LONG,
    pub tmAscent: super::LONG,
    pub tmDescent: super::LONG,
    pub tmInternalLeading: super::LONG,
    pub tmExternalLeading: super::LONG,
    pub tmAveCharWidth: super::LONG,
    pub tmMaxCharWidth: super::LONG,
    pub tmWeight: super::LONG,
    pub tmOverhang: super::LONG,
    pub tmDigitizedAspectX: super::LONG,
    pub tmDigitizedAspectY: super::LONG,
    pub tmFirstChar: super::WCHAR,
    pub tmLastChar: super::WCHAR,
    pub tmDefaultChar: super::WCHAR,
    pub tmBreakChar: super::WCHAR,
    pub tmItalic: super::BYTE,
    pub tmUnderlined: super::BYTE,
    pub tmStruckOut: super::BYTE,
    pub tmPitchAndFamily: super::BYTE,
    pub tmCharSet: super::BYTE,
    pub ntmFlags: super::DWORD,
    pub ntmSizeEM: super::UINT,
    pub ntmCellHeight: super::UINT,
    pub ntmAvgWidth: super::UINT,
}
pub const NEXTBAND: i32 = 3;
#[cfg(feature = "shared")]
pub const NOMIRRORBITMAP: super::DWORD = 2147483648;
pub const NONANTIALIASED_QUALITY: i32 = 3;
#[cfg(feature = "shared")]
pub const NOTSRCCOPY: super::DWORD = 3342344;
#[cfg(feature = "shared")]
pub const NOTSRCERASE: super::DWORD = 1114278;
#[cfg(feature = "shared")]
pub type NPABC = *mut ABC;
#[cfg(feature = "minwindef")]
pub type NPABCFLOAT = *mut ABCFLOAT;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type NPBITMAP = *mut BITMAP;
#[cfg(feature = "shared")]
pub type NPCHARSETINFO = *mut CHARSETINFO;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPDEVMODE = *mut DEVMODEA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPDEVMODEA = *mut DEVMODEA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type NPDEVMODEW = *mut DEVMODEW;
#[cfg(feature = "shared")]
pub type NPEXTLOGFONT = *mut EXTLOGFONTA;
#[cfg(feature = "shared")]
pub type NPEXTLOGFONTA = *mut EXTLOGFONTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPEXTLOGFONTW = *mut EXTLOGFONTW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPEXTLOGPEN = *mut EXTLOGPEN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPEXTLOGPEN32 = *mut EXTLOGPEN32;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPLOGBRUSH = *mut LOGBRUSH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPLOGBRUSH32 = *mut LOGBRUSH32;
#[cfg(feature = "shared")]
pub type NPLOGFONT = *mut LOGFONTA;
#[cfg(feature = "shared")]
pub type NPLOGFONTA = *mut LOGFONTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPLOGFONTW = *mut LOGFONTW;
#[cfg(feature = "shared")]
pub type NPLOGPALETTE = *mut LOGPALETTE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPLOGPEN = *mut LOGPEN;
#[cfg(feature = "shared")]
pub type NPNEWTEXTMETRIC = *mut NEWTEXTMETRICA;
#[cfg(feature = "shared")]
pub type NPNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPOUTLINETEXTMETRIC = *mut OUTLINETEXTMETRICA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type NPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPPATTERN = *mut LOGBRUSH;
#[cfg(feature = "shared")]
pub type NPPELARRAY = *mut PELARRAY;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPPOLYTEXT = *mut POLYTEXTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPPOLYTEXTA = *mut POLYTEXTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPPOLYTEXTW = *mut POLYTEXTW;
#[cfg(feature = "shared")]
pub type NPRGBTRIPLE = *mut RGBTRIPLE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type NPRGNDATA = *mut RGNDATA;
#[cfg(feature = "shared")]
pub type NPTEXTMETRIC = *mut TEXTMETRICA;
#[cfg(feature = "shared")]
pub type NPTEXTMETRICA = *mut TEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type NPTEXTMETRICW = *mut TEXTMETRICW;
pub const NTM_BOLD: i32 = 32;
pub const NTM_DSIG: i32 = 2097152;
pub const NTM_ITALIC: i32 = 1;
pub const NTM_MULTIPLEMASTER: i32 = 524288;
pub const NTM_NONNEGATIVE_AC: i32 = 65536;
pub const NTM_PS_OPENTYPE: i32 = 131072;
pub const NTM_REGULAR: i32 = 64;
pub const NTM_TT_OPENTYPE: i32 = 262144;
pub const NTM_TYPE1: i32 = 1048576;
pub const NULLREGION: i32 = 1;
pub const NULL_BRUSH: i32 = 5;
pub const NULL_PEN: i32 = 8;
pub const NUMBRUSHES: i32 = 16;
pub const NUMCOLORS: i32 = 24;
pub const NUMFONTS: i32 = 22;
pub const NUMMARKERS: i32 = 20;
pub const NUMPENS: i32 = 18;
pub const NUMRESERVED: i32 = 106;
pub const OBJ_BITMAP: i32 = 7;
pub const OBJ_BRUSH: i32 = 2;
pub const OBJ_COLORSPACE: i32 = 14;
pub const OBJ_DC: i32 = 3;
pub const OBJ_ENHMETADC: i32 = 12;
pub const OBJ_ENHMETAFILE: i32 = 13;
pub const OBJ_EXTPEN: i32 = 11;
pub const OBJ_FONT: i32 = 6;
pub const OBJ_MEMDC: i32 = 10;
pub const OBJ_METADC: i32 = 4;
pub const OBJ_METAFILE: i32 = 9;
pub const OBJ_PAL: i32 = 5;
pub const OBJ_PEN: i32 = 1;
pub const OBJ_REGION: i32 = 8;
pub const OEM_CHARSET: i32 = 255;
pub const OEM_FIXED_FONT: i32 = 10;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type OLDFONTENUMPROCA = Option<unsafe extern "system" fn(param0: *const LOGFONTA, param1: *const TEXTMETRICA, param2: super::DWORD, param3: super::LPARAM) -> i32>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type OLDFONTENUMPROCW = Option<unsafe extern "system" fn(param0: *const LOGFONTW, param1: *const TEXTMETRICW, param2: super::DWORD, param3: super::LPARAM) -> i32>;
pub const OPAQUE: i32 = 2;
pub const OPENCHANNEL: i32 = 4110;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type OUTLINETEXTMETRIC = OUTLINETEXTMETRICA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OUTLINETEXTMETRICA {
    pub otmSize: super::UINT,
    pub otmTextMetrics: TEXTMETRICA,
    pub otmFiller: super::BYTE,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: super::UINT,
    pub otmfsType: super::UINT,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: super::UINT,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: super::UINT,
    pub otmsCapEmHeight: super::UINT,
    pub otmsXHeight: super::UINT,
    pub otmrcFontBox: super::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: super::UINT,
    pub otmusMinimumPPEM: super::UINT,
    pub otmptSubscriptSize: super::POINT,
    pub otmptSubscriptOffset: super::POINT,
    pub otmptSuperscriptSize: super::POINT,
    pub otmptSuperscriptOffset: super::POINT,
    pub otmsStrikeoutSize: super::UINT,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: windows_sys::core::PSTR,
    pub otmpFaceName: windows_sys::core::PSTR,
    pub otmpStyleName: windows_sys::core::PSTR,
    pub otmpFullName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct OUTLINETEXTMETRICW {
    pub otmSize: super::UINT,
    pub otmTextMetrics: TEXTMETRICW,
    pub otmFiller: super::BYTE,
    pub otmPanoseNumber: PANOSE,
    pub otmfsSelection: super::UINT,
    pub otmfsType: super::UINT,
    pub otmsCharSlopeRise: i32,
    pub otmsCharSlopeRun: i32,
    pub otmItalicAngle: i32,
    pub otmEMSquare: super::UINT,
    pub otmAscent: i32,
    pub otmDescent: i32,
    pub otmLineGap: super::UINT,
    pub otmsCapEmHeight: super::UINT,
    pub otmsXHeight: super::UINT,
    pub otmrcFontBox: super::RECT,
    pub otmMacAscent: i32,
    pub otmMacDescent: i32,
    pub otmMacLineGap: super::UINT,
    pub otmusMinimumPPEM: super::UINT,
    pub otmptSubscriptSize: super::POINT,
    pub otmptSubscriptOffset: super::POINT,
    pub otmptSuperscriptSize: super::POINT,
    pub otmptSuperscriptOffset: super::POINT,
    pub otmsStrikeoutSize: super::UINT,
    pub otmsStrikeoutPosition: i32,
    pub otmsUnderscoreSize: i32,
    pub otmsUnderscorePosition: i32,
    pub otmpFamilyName: windows_sys::core::PSTR,
    pub otmpFaceName: windows_sys::core::PSTR,
    pub otmpStyleName: windows_sys::core::PSTR,
    pub otmpFullName: windows_sys::core::PSTR,
}
pub const OUT_CHARACTER_PRECIS: i32 = 2;
pub const OUT_DEFAULT_PRECIS: i32 = 0;
pub const OUT_DEVICE_PRECIS: i32 = 5;
pub const OUT_OUTLINE_PRECIS: i32 = 8;
pub const OUT_PS_ONLY_PRECIS: i32 = 10;
pub const OUT_RASTER_PRECIS: i32 = 6;
pub const OUT_SCREEN_OUTLINE_PRECIS: i32 = 9;
pub const OUT_STRING_PRECIS: i32 = 1;
pub const OUT_STROKE_PRECIS: i32 = 3;
pub const OUT_TT_ONLY_PRECIS: i32 = 7;
pub const OUT_TT_PRECIS: i32 = 4;
#[cfg(feature = "shared")]
pub type PABC = *mut ABC;
#[cfg(feature = "minwindef")]
pub type PABCFLOAT = *mut ABCFLOAT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PALETTEENTRY {
    pub peRed: super::BYTE,
    pub peGreen: super::BYTE,
    pub peBlue: super::BYTE,
    pub peFlags: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PANOSE {
    pub bFamilyType: super::BYTE,
    pub bSerifStyle: super::BYTE,
    pub bWeight: super::BYTE,
    pub bProportion: super::BYTE,
    pub bContrast: super::BYTE,
    pub bStrokeVariation: super::BYTE,
    pub bArmStyle: super::BYTE,
    pub bLetterform: super::BYTE,
    pub bMidline: super::BYTE,
    pub bXHeight: super::BYTE,
}
pub const PANOSE_COUNT: i32 = 10;
pub const PAN_ANY: i32 = 0;
pub const PAN_ARMSTYLE_INDEX: i32 = 6;
pub const PAN_BENT_ARMS_DOUBLE_SERIF: i32 = 11;
pub const PAN_BENT_ARMS_HORZ: i32 = 7;
pub const PAN_BENT_ARMS_SINGLE_SERIF: i32 = 10;
pub const PAN_BENT_ARMS_VERT: i32 = 9;
pub const PAN_BENT_ARMS_WEDGE: i32 = 8;
pub const PAN_CONTRAST_HIGH: i32 = 8;
pub const PAN_CONTRAST_INDEX: i32 = 4;
pub const PAN_CONTRAST_LOW: i32 = 4;
pub const PAN_CONTRAST_MEDIUM: i32 = 6;
pub const PAN_CONTRAST_MEDIUM_HIGH: i32 = 7;
pub const PAN_CONTRAST_MEDIUM_LOW: i32 = 5;
pub const PAN_CONTRAST_NONE: i32 = 2;
pub const PAN_CONTRAST_VERY_HIGH: i32 = 9;
pub const PAN_CONTRAST_VERY_LOW: i32 = 3;
pub const PAN_CULTURE_LATIN: i32 = 0;
pub const PAN_FAMILYTYPE_INDEX: i32 = 0;
pub const PAN_FAMILY_DECORATIVE: i32 = 4;
pub const PAN_FAMILY_PICTORIAL: i32 = 5;
pub const PAN_FAMILY_SCRIPT: i32 = 3;
pub const PAN_FAMILY_TEXT_DISPLAY: i32 = 2;
pub const PAN_LETTERFORM_INDEX: i32 = 7;
pub const PAN_LETT_NORMAL_BOXED: i32 = 4;
pub const PAN_LETT_NORMAL_CONTACT: i32 = 2;
pub const PAN_LETT_NORMAL_FLATTENED: i32 = 5;
pub const PAN_LETT_NORMAL_OFF_CENTER: i32 = 7;
pub const PAN_LETT_NORMAL_ROUNDED: i32 = 6;
pub const PAN_LETT_NORMAL_SQUARE: i32 = 8;
pub const PAN_LETT_NORMAL_WEIGHTED: i32 = 3;
pub const PAN_LETT_OBLIQUE_BOXED: i32 = 11;
pub const PAN_LETT_OBLIQUE_CONTACT: i32 = 9;
pub const PAN_LETT_OBLIQUE_FLATTENED: i32 = 12;
pub const PAN_LETT_OBLIQUE_OFF_CENTER: i32 = 14;
pub const PAN_LETT_OBLIQUE_ROUNDED: i32 = 13;
pub const PAN_LETT_OBLIQUE_SQUARE: i32 = 15;
pub const PAN_LETT_OBLIQUE_WEIGHTED: i32 = 10;
pub const PAN_MIDLINE_CONSTANT_POINTED: i32 = 9;
pub const PAN_MIDLINE_CONSTANT_SERIFED: i32 = 10;
pub const PAN_MIDLINE_CONSTANT_TRIMMED: i32 = 8;
pub const PAN_MIDLINE_HIGH_POINTED: i32 = 6;
pub const PAN_MIDLINE_HIGH_SERIFED: i32 = 7;
pub const PAN_MIDLINE_HIGH_TRIMMED: i32 = 5;
pub const PAN_MIDLINE_INDEX: i32 = 8;
pub const PAN_MIDLINE_LOW_POINTED: i32 = 12;
pub const PAN_MIDLINE_LOW_SERIFED: i32 = 13;
pub const PAN_MIDLINE_LOW_TRIMMED: i32 = 11;
pub const PAN_MIDLINE_STANDARD_POINTED: i32 = 3;
pub const PAN_MIDLINE_STANDARD_SERIFED: i32 = 4;
pub const PAN_MIDLINE_STANDARD_TRIMMED: i32 = 2;
pub const PAN_NO_FIT: i32 = 1;
pub const PAN_PROPORTION_INDEX: i32 = 3;
pub const PAN_PROP_CONDENSED: i32 = 6;
pub const PAN_PROP_EVEN_WIDTH: i32 = 4;
pub const PAN_PROP_EXPANDED: i32 = 5;
pub const PAN_PROP_MODERN: i32 = 3;
pub const PAN_PROP_MONOSPACED: i32 = 9;
pub const PAN_PROP_OLD_STYLE: i32 = 2;
pub const PAN_PROP_VERY_CONDENSED: i32 = 8;
pub const PAN_PROP_VERY_EXPANDED: i32 = 7;
pub const PAN_SERIFSTYLE_INDEX: i32 = 1;
pub const PAN_SERIF_BONE: i32 = 8;
pub const PAN_SERIF_COVE: i32 = 2;
pub const PAN_SERIF_EXAGGERATED: i32 = 9;
pub const PAN_SERIF_FLARED: i32 = 14;
pub const PAN_SERIF_NORMAL_SANS: i32 = 11;
pub const PAN_SERIF_OBTUSE_COVE: i32 = 3;
pub const PAN_SERIF_OBTUSE_SANS: i32 = 12;
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: i32 = 5;
pub const PAN_SERIF_PERP_SANS: i32 = 13;
pub const PAN_SERIF_ROUNDED: i32 = 15;
pub const PAN_SERIF_SQUARE: i32 = 6;
pub const PAN_SERIF_SQUARE_COVE: i32 = 4;
pub const PAN_SERIF_THIN: i32 = 7;
pub const PAN_SERIF_TRIANGLE: i32 = 10;
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: i32 = 6;
pub const PAN_STRAIGHT_ARMS_HORZ: i32 = 2;
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: i32 = 5;
pub const PAN_STRAIGHT_ARMS_VERT: i32 = 4;
pub const PAN_STRAIGHT_ARMS_WEDGE: i32 = 3;
pub const PAN_STROKEVARIATION_INDEX: i32 = 5;
pub const PAN_STROKE_GRADUAL_DIAG: i32 = 2;
pub const PAN_STROKE_GRADUAL_HORZ: i32 = 5;
pub const PAN_STROKE_GRADUAL_TRAN: i32 = 3;
pub const PAN_STROKE_GRADUAL_VERT: i32 = 4;
pub const PAN_STROKE_INSTANT_VERT: i32 = 8;
pub const PAN_STROKE_RAPID_HORZ: i32 = 7;
pub const PAN_STROKE_RAPID_VERT: i32 = 6;
pub const PAN_WEIGHT_BLACK: i32 = 10;
pub const PAN_WEIGHT_BOLD: i32 = 8;
pub const PAN_WEIGHT_BOOK: i32 = 5;
pub const PAN_WEIGHT_DEMI: i32 = 7;
pub const PAN_WEIGHT_HEAVY: i32 = 9;
pub const PAN_WEIGHT_INDEX: i32 = 2;
pub const PAN_WEIGHT_LIGHT: i32 = 3;
pub const PAN_WEIGHT_MEDIUM: i32 = 6;
pub const PAN_WEIGHT_NORD: i32 = 11;
pub const PAN_WEIGHT_THIN: i32 = 4;
pub const PAN_WEIGHT_VERY_LIGHT: i32 = 2;
pub const PAN_XHEIGHT_CONSTANT_LARGE: i32 = 4;
pub const PAN_XHEIGHT_CONSTANT_SMALL: i32 = 2;
pub const PAN_XHEIGHT_CONSTANT_STD: i32 = 3;
pub const PAN_XHEIGHT_DUCKING_LARGE: i32 = 7;
pub const PAN_XHEIGHT_DUCKING_SMALL: i32 = 5;
pub const PAN_XHEIGHT_DUCKING_STD: i32 = 6;
pub const PAN_XHEIGHT_INDEX: i32 = 9;
pub const PASSTHROUGH: i32 = 19;
#[cfg(feature = "shared")]
pub const PATCOPY: super::DWORD = 15728673;
#[cfg(feature = "shared")]
pub const PATINVERT: super::DWORD = 5898313;
#[cfg(feature = "shared")]
pub const PATPAINT: super::DWORD = 16452105;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PATTERN = LOGBRUSH;
#[cfg(feature = "shared")]
pub type PAXESLIST = *mut AXESLISTA;
#[cfg(feature = "shared")]
pub type PAXESLISTA = *mut AXESLISTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAXESLISTW = *mut AXESLISTW;
#[cfg(feature = "shared")]
pub type PAXISINFO = *mut AXISINFOA;
#[cfg(feature = "shared")]
pub type PAXISINFOA = *mut AXISINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAXISINFOW = *mut AXISINFOW;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBITMAP = *mut BITMAP;
#[cfg(feature = "shared")]
pub type PBITMAPCOREHEADER = *mut BITMAPCOREHEADER;
#[cfg(feature = "shared")]
pub type PBITMAPCOREINFO = *mut BITMAPCOREINFO;
#[cfg(feature = "shared")]
pub type PBITMAPFILEHEADER = *mut BITMAPFILEHEADER;
#[cfg(feature = "shared")]
pub type PBITMAPINFO = *mut BITMAPINFO;
#[cfg(feature = "shared")]
pub type PBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
#[cfg(feature = "shared")]
pub type PBITMAPV4HEADER = *mut BITMAPV4HEADER;
#[cfg(feature = "shared")]
pub type PBITMAPV5HEADER = *mut BITMAPV5HEADER;
#[cfg(feature = "shared")]
pub type PBLENDFUNCTION = *mut BLENDFUNCTION;
#[cfg(feature = "shared")]
pub type PCHARSETINFO = *mut CHARSETINFO;
#[cfg(feature = "shared")]
pub type PCOLORADJUSTMENT = *mut COLORADJUSTMENT;
pub const PC_EXPLICIT: i32 = 2;
pub const PC_INTERIORS: i32 = 128;
pub const PC_NOCOLLAPSE: i32 = 4;
pub const PC_NONE: i32 = 0;
pub const PC_PATHS: i32 = 512;
pub const PC_POLYGON: i32 = 1;
pub const PC_POLYPOLYGON: i32 = 256;
pub const PC_RECTANGLE: i32 = 2;
pub const PC_RESERVED: i32 = 1;
pub const PC_SCANLINE: i32 = 8;
pub const PC_STYLED: i32 = 32;
pub const PC_TRAPEZOID: i32 = 4;
pub const PC_WIDE: i32 = 16;
pub const PC_WIDESTYLED: i32 = 64;
pub const PC_WINDPOLYGON: i32 = 4;
#[cfg(feature = "shared")]
pub type PDESIGNVECTOR = *mut DESIGNVECTOR;
pub const PDEVICESIZE: i32 = 26;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PDEVMODE = *mut DEVMODEA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PDEVMODEA = *mut DEVMODEA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type PDEVMODEW = *mut DEVMODEW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PDIBSECTION = *mut DIBSECTION;
#[cfg(feature = "shared")]
pub type PDISPLAY_DEVICE = *mut DISPLAY_DEVICEA;
#[cfg(feature = "shared")]
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PDRAWPATRECT = *mut DRAWPATRECT;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PELARRAY {
    pub paXCount: super::LONG,
    pub paYCount: super::LONG,
    pub paXExt: super::LONG,
    pub paYExt: super::LONG,
    pub paRGBs: super::BYTE,
}
#[cfg(feature = "shared")]
pub type PEMR = *mut EMR;
#[cfg(feature = "shared")]
pub type PEMRABORTPATH = *mut tagABORTPATH;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRALPHABLEND = *mut EMRALPHABLEND;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRANGLEARC = *mut EMRANGLEARC;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRARC = *mut tagEMRARC;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRARCTO = *mut tagEMRARC;
#[cfg(feature = "shared")]
pub type PEMRBEGINPATH = *mut tagABORTPATH;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRBITBLT = *mut EMRBITBLT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRCHORD = *mut tagEMRARC;
#[cfg(feature = "shared")]
pub type PEMRCLOSEFIGURE = *mut tagABORTPATH;
#[cfg(feature = "shared")]
pub type PEMRCOLORCORRECTPALETTE = *mut EMRCOLORCORRECTPALETTE;
#[cfg(feature = "shared")]
pub type PEMRCOLORMATCHTOTARGET = *mut EMRCOLORMATCHTOTARGET;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRCREATEBRUSHINDIRECT = *mut EMRCREATEBRUSHINDIRECT;
#[cfg(feature = "shared")]
pub type PEMRCREATECOLORSPACE = *mut EMRCREATECOLORSPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEMRCREATECOLORSPACEW = *mut EMRCREATECOLORSPACEW;
#[cfg(feature = "shared")]
pub type PEMRCREATEDIBPATTERNBRUSHPT = *mut EMRCREATEDIBPATTERNBRUSHPT;
#[cfg(feature = "shared")]
pub type PEMRCREATEMONOBRUSH = *mut EMRCREATEMONOBRUSH;
#[cfg(feature = "shared")]
pub type PEMRCREATEPALETTE = *mut EMRCREATEPALETTE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRCREATEPEN = *mut EMRCREATEPEN;
#[cfg(feature = "shared")]
pub type PEMRDELETECOLORSPACE = *mut tagEMRSETCOLORSPACE;
#[cfg(feature = "shared")]
pub type PEMRDELETEOBJECT = *mut tagEMRSELECTOBJECT;
#[cfg(feature = "shared")]
pub type PEMRDRAWESCAPE = *mut tagEMREXTESCAPE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRELLIPSE = *mut tagEMRELLIPSE;
#[cfg(feature = "shared")]
pub type PEMRENDPATH = *mut tagABORTPATH;
#[cfg(feature = "shared")]
pub type PEMREOF = *mut EMREOF;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMREXCLUDECLIPRECT = *mut tagEMREXCLUDECLIPRECT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEMREXTCREATEFONTINDIRECTW = *mut EMREXTCREATEFONTINDIRECTW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMREXTCREATEPEN = *mut EMREXTCREATEPEN;
#[cfg(feature = "shared")]
pub type PEMREXTESCAPE = *mut tagEMREXTESCAPE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMREXTFLOODFILL = *mut EMREXTFLOODFILL;
#[cfg(feature = "shared")]
pub type PEMREXTSELECTCLIPRGN = *mut EMREXTSELECTCLIPRGN;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMREXTTEXTOUTA = *mut tagEMREXTTEXTOUTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMREXTTEXTOUTW = *mut tagEMREXTTEXTOUTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRFILLPATH = *mut tagEMRFILLPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRFILLRGN = *mut EMRFILLRGN;
#[cfg(feature = "shared")]
pub type PEMRFLATTENPATH = *mut tagABORTPATH;
#[cfg(feature = "shared")]
pub type PEMRFORMAT = *mut EMRFORMAT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRFRAMERGN = *mut EMRFRAMERGN;
#[cfg(feature = "shared")]
pub type PEMRGDICOMMENT = *mut EMRGDICOMMENT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRGLSBOUNDEDRECORD = *mut EMRGLSBOUNDEDRECORD;
#[cfg(feature = "shared")]
pub type PEMRGLSRECORD = *mut EMRGLSRECORD;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRGRADIENTFILL = *mut EMRGRADIENTFILL;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRINTERSECTCLIPRECT = *mut tagEMREXCLUDECLIPRECT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRINVERTRGN = *mut tagEMRINVERTRGN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRLINETO = *mut tagEMRLINETO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRMASKBLT = *mut EMRMASKBLT;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PEMRMODIFYWORLDTRANSFORM = *mut EMRMODIFYWORLDTRANSFORM;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRMOVETOEX = *mut tagEMRLINETO;
#[cfg(feature = "shared")]
pub type PEMRNAMEDESCAPE = *mut EMRNAMEDESCAPE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMROFFSETCLIPRGN = *mut EMROFFSETCLIPRGN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPAINTRGN = *mut tagEMRINVERTRGN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPIE = *mut tagEMRARC;
#[cfg(feature = "shared")]
pub type PEMRPIXELFORMAT = *mut EMRPIXELFORMAT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRPLGBLT = *mut EMRPLGBLT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYBEZIER = *mut tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYBEZIER16 = *mut tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYBEZIERTO = *mut tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYBEZIERTO16 = *mut tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYDRAW = *mut EMRPOLYDRAW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYDRAW16 = *mut EMRPOLYDRAW16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYGON = *mut tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYGON16 = *mut tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYLINE = *mut tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYLINE16 = *mut tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYLINETO = *mut tagEMRPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYLINETO16 = *mut tagEMRPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYPOLYGON = *mut tagEMRPOLYPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYPOLYGON16 = *mut tagEMRPOLYPOLYLINE16;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYPOLYLINE = *mut tagEMRPOLYPOLYLINE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRPOLYPOLYLINE16 = *mut tagEMRPOLYPOLYLINE16;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRPOLYTEXTOUTA = *mut tagEMRPOLYTEXTOUTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRPOLYTEXTOUTW = *mut tagEMRPOLYTEXTOUTA;
#[cfg(feature = "shared")]
pub type PEMRREALIZEPALETTE = *mut tagABORTPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRRECTANGLE = *mut tagEMRELLIPSE;
#[cfg(feature = "shared")]
pub type PEMRRESIZEPALETTE = *mut EMRRESIZEPALETTE;
#[cfg(feature = "shared")]
pub type PEMRRESTOREDC = *mut EMRRESTOREDC;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRROUNDRECT = *mut EMRROUNDRECT;
#[cfg(feature = "shared")]
pub type PEMRSAVEDC = *mut tagABORTPATH;
#[cfg(feature = "shared")]
pub type PEMRSCALEVIEWPORTEXTEX = *mut tagEMRSCALEVIEWPORTEXTEX;
#[cfg(feature = "shared")]
pub type PEMRSCALEWINDOWEXTEX = *mut tagEMRSCALEVIEWPORTEXTEX;
#[cfg(feature = "shared")]
pub type PEMRSELECTCLIPPATH = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSELECTCOLORSPACE = *mut tagEMRSETCOLORSPACE;
#[cfg(feature = "shared")]
pub type PEMRSELECTOBJECT = *mut tagEMRSELECTOBJECT;
#[cfg(feature = "shared")]
pub type PEMRSELECTPALETTE = *mut EMRSELECTPALETTE;
#[cfg(feature = "shared")]
pub type PEMRSETARCDIRECTION = *mut EMRSETARCDIRECTION;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETBKCOLOR = *mut tagEMRSETTEXTCOLOR;
#[cfg(feature = "shared")]
pub type PEMRSETBKMODE = *mut tagEMRSELECTCLIPPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETBRUSHORGEX = *mut tagEMRSETVIEWPORTORGEX;
#[cfg(feature = "shared")]
pub type PEMRSETCOLORADJUSTMENT = *mut EMRSETCOLORADJUSTMENT;
#[cfg(feature = "shared")]
pub type PEMRSETCOLORSPACE = *mut tagEMRSETCOLORSPACE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETDIBITSTODEVICE = *mut EMRSETDIBITSTODEVICE;
#[cfg(feature = "shared")]
pub type PEMRSETICMMODE = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSETICMPROFILE = *mut tagEMRSETICMPROFILE;
#[cfg(feature = "shared")]
pub type PEMRSETICMPROFILEA = *mut tagEMRSETICMPROFILE;
#[cfg(feature = "shared")]
pub type PEMRSETICMPROFILEW = *mut tagEMRSETICMPROFILE;
#[cfg(feature = "shared")]
pub type PEMRSETLAYOUT = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSETMAPMODE = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSETMAPPERFLAGS = *mut EMRSETMAPPERFLAGS;
#[cfg(feature = "shared")]
pub type PEMRSETMETARGN = *mut tagABORTPATH;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PEMRSETMITERLIMIT = *mut EMRSETMITERLIMIT;
#[cfg(feature = "shared")]
pub type PEMRSETPALETTEENTRIES = *mut EMRSETPALETTEENTRIES;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETPIXELV = *mut EMRSETPIXELV;
#[cfg(feature = "shared")]
pub type PEMRSETPOLYFILLMODE = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSETROP2 = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSETSTRETCHBLTMODE = *mut tagEMRSELECTCLIPPATH;
#[cfg(feature = "shared")]
pub type PEMRSETTEXTALIGN = *mut tagEMRSELECTCLIPPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETTEXTCOLOR = *mut tagEMRSETTEXTCOLOR;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETVIEWPORTEXTEX = *mut tagEMRSETVIEWPORTEXTEX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETVIEWPORTORGEX = *mut tagEMRSETVIEWPORTORGEX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETWINDOWEXTEX = *mut tagEMRSETVIEWPORTEXTEX;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSETWINDOWORGEX = *mut tagEMRSETVIEWPORTORGEX;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PEMRSETWORLDTRANSFORM = *mut EMRSETWORLDTRANSFORM;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRSTRETCHBLT = *mut EMRSTRETCHBLT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSTRETCHDIBITS = *mut EMRSTRETCHDIBITS;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSTROKEANDFILLPATH = *mut tagEMRFILLPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRSTROKEPATH = *mut tagEMRFILLPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEMRTEXT = *mut EMRTEXT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PEMRTRANSPARENTBLT = *mut EMRTRANSPARENTBLT;
#[cfg(feature = "shared")]
pub type PEMRWIDENPATH = *mut tagABORTPATH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PENHMETAHEADER = *mut ENHMETAHEADER;
#[cfg(feature = "shared")]
pub type PENHMETARECORD = *mut ENHMETARECORD;
#[cfg(feature = "shared")]
pub type PENUMLOGFONTEXDV = *mut ENUMLOGFONTEXDVA;
#[cfg(feature = "shared")]
pub type PENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
#[cfg(feature = "shared")]
pub type PENUMTEXTMETRIC = *mut ENUMTEXTMETRICA;
#[cfg(feature = "shared")]
pub type PENUMTEXTMETRICA = *mut ENUMTEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENUMTEXTMETRICW = *mut ENUMTEXTMETRICW;
#[cfg(feature = "shared")]
pub type PEXTLOGFONT = *mut EXTLOGFONTA;
#[cfg(feature = "shared")]
pub type PEXTLOGFONTA = *mut EXTLOGFONTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PEXTLOGFONTW = *mut EXTLOGFONTW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEXTLOGPEN = *mut EXTLOGPEN;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PEXTLOGPEN32 = *mut EXTLOGPEN32;
pub const PFD_DEPTH_DONTCARE: i32 = 536870912;
pub const PFD_DIRECT3D_ACCELERATED: i32 = 16384;
pub const PFD_DOUBLEBUFFER: i32 = 1;
pub const PFD_DOUBLEBUFFER_DONTCARE: i32 = 1073741824;
pub const PFD_DRAW_TO_BITMAP: i32 = 8;
pub const PFD_DRAW_TO_WINDOW: i32 = 4;
pub const PFD_GENERIC_ACCELERATED: i32 = 4096;
pub const PFD_GENERIC_FORMAT: i32 = 64;
pub const PFD_MAIN_PLANE: i32 = 0;
pub const PFD_NEED_PALETTE: i32 = 128;
pub const PFD_NEED_SYSTEM_PALETTE: i32 = 256;
pub const PFD_OVERLAY_PLANE: i32 = 1;
pub const PFD_STEREO: i32 = 2;
pub const PFD_STEREO_DONTCARE: u32 = 2147483648;
pub const PFD_SUPPORT_COMPOSITION: i32 = 32768;
pub const PFD_SUPPORT_DIRECTDRAW: i32 = 8192;
pub const PFD_SUPPORT_GDI: i32 = 16;
pub const PFD_SUPPORT_OPENGL: i32 = 32;
pub const PFD_SWAP_COPY: i32 = 1024;
pub const PFD_SWAP_EXCHANGE: i32 = 512;
pub const PFD_SWAP_LAYER_BUFFERS: i32 = 2048;
pub const PFD_TYPE_COLORINDEX: i32 = 1;
pub const PFD_TYPE_RGBA: i32 = 0;
pub const PFD_UNDERLAY_PLANE: i32 = -1;
#[cfg(feature = "shared")]
pub type PFONTSIGNATURE = *mut FONTSIGNATURE;
#[cfg(feature = "minwindef")]
pub type PGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGLYPHSET = *mut GLYPHSET;
#[cfg(feature = "shared")]
pub type PGRADIENT_RECT = *mut GRADIENT_RECT;
#[cfg(feature = "shared")]
pub type PGRADIENT_TRIANGLE = *mut GRADIENT_TRIANGLE;
#[cfg(feature = "windef")]
pub type PHANDLETABLE = *mut HANDLETABLE;
pub const PHYSICALHEIGHT: i32 = 111;
pub const PHYSICALOFFSETX: i32 = 112;
pub const PHYSICALOFFSETY: i32 = 113;
pub const PHYSICALWIDTH: i32 = 110;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: super::WORD,
    pub nVersion: super::WORD,
    pub dwFlags: super::DWORD,
    pub iPixelType: super::BYTE,
    pub cColorBits: super::BYTE,
    pub cRedBits: super::BYTE,
    pub cRedShift: super::BYTE,
    pub cGreenBits: super::BYTE,
    pub cGreenShift: super::BYTE,
    pub cBlueBits: super::BYTE,
    pub cBlueShift: super::BYTE,
    pub cAlphaBits: super::BYTE,
    pub cAlphaShift: super::BYTE,
    pub cAccumBits: super::BYTE,
    pub cAccumRedBits: super::BYTE,
    pub cAccumGreenBits: super::BYTE,
    pub cAccumBlueBits: super::BYTE,
    pub cAccumAlphaBits: super::BYTE,
    pub cDepthBits: super::BYTE,
    pub cStencilBits: super::BYTE,
    pub cAuxBuffers: super::BYTE,
    pub iLayerType: super::BYTE,
    pub bReserved: super::BYTE,
    pub dwLayerMask: super::DWORD,
    pub dwVisibleMask: super::DWORD,
    pub dwDamageMask: super::DWORD,
}
pub const PLANES: i32 = 14;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
#[cfg(feature = "shared")]
pub type PLOCALESIGNATURE = *mut LOCALESIGNATURE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PLOGBRUSH = *mut LOGBRUSH;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PLOGBRUSH32 = *mut LOGBRUSH32;
#[cfg(feature = "shared")]
pub type PLOGFONT = *mut LOGFONTA;
#[cfg(feature = "shared")]
pub type PLOGFONTA = *mut LOGFONTA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PLOGFONTW = *mut LOGFONTW;
#[cfg(feature = "shared")]
pub type PLOGPALETTE = *mut LOGPALETTE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PLOGPEN = *mut LOGPEN;
#[cfg(feature = "shared")]
pub type PNEWTEXTMETRIC = *mut NEWTEXTMETRICA;
#[cfg(feature = "shared")]
pub type PNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct POINTFLOAT {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct POINTFX {
    pub x: FIXED,
    pub y: FIXED,
}
pub const POLYFILL_LAST: i32 = 2;
pub const POLYGONALCAPS: i32 = 32;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type POLYTEXT = POLYTEXTA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct POLYTEXTA {
    pub x: i32,
    pub y: i32,
    pub n: super::UINT,
    pub lpstr: windows_sys::core::PCSTR,
    pub uiFlags: super::UINT,
    pub rcl: super::RECT,
    pub pdx: *mut i32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct POLYTEXTW {
    pub x: i32,
    pub y: i32,
    pub n: super::UINT,
    pub lpstr: windows_sys::core::PCWSTR,
    pub uiFlags: super::UINT,
    pub rcl: super::RECT,
    pub pdx: *mut i32,
}
pub const POSTSCRIPT_DATA: i32 = 37;
pub const POSTSCRIPT_IDENTIFY: i32 = 4117;
pub const POSTSCRIPT_IGNORE: i32 = 38;
pub const POSTSCRIPT_INJECTION: i32 = 4118;
pub const POSTSCRIPT_PASSTHROUGH: i32 = 4115;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type POUTLINETEXTMETRIC = *mut OUTLINETEXTMETRICA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type POUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type POUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
#[cfg(feature = "shared")]
pub type PPALETTEENTRY = *mut PALETTEENTRY;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPATTERN = *mut LOGBRUSH;
#[cfg(feature = "shared")]
pub type PPELARRAY = *mut PELARRAY;
#[cfg(feature = "shared")]
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
#[cfg(feature = "minwindef")]
pub type PPOINTFLOAT = *mut POINTFLOAT;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPOLYTEXT = *mut POLYTEXTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPOLYTEXTA = *mut POLYTEXTA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PPOLYTEXTW = *mut POLYTEXTW;
#[cfg(feature = "shared")]
pub type PPSFEATURE_CUSTPAPER = *mut PSFEATURE_CUSTPAPER;
pub type PPSFEATURE_OUTPUT = *mut PSFEATURE_OUTPUT;
#[cfg(feature = "shared")]
pub type PPSINJECTDATA = *mut PSINJECTDATA;
#[cfg(feature = "shared")]
pub type PRGBTRIPLE = *mut RGBTRIPLE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PRGNDATA = *mut RGNDATA;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PRGNDATAHEADER = *mut RGNDATAHEADER;
pub const PRINTRATEUNIT_CPS: i32 = 2;
pub const PRINTRATEUNIT_IPM: i32 = 4;
pub const PRINTRATEUNIT_LPM: i32 = 3;
pub const PRINTRATEUNIT_PPM: i32 = 1;
pub const PROFILE_EMBEDDED: i32 = 1296188740;
pub const PROFILE_LINKED: i32 = 1279872587;
pub const PROOF_QUALITY: i32 = 2;
pub const PR_JOBSTATUS: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: super::LONG,
    pub lWidth: super::LONG,
    pub lHeight: super::LONG,
    pub lWidthOffset: super::LONG,
    pub lHeightOffset: super::LONG,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: windows_sys::core::BOOL,
    pub bSetPageDevice: windows_sys::core::BOOL,
}
pub const PSIDENT_GDICENTRIC: i32 = 0;
pub const PSIDENT_PSCENTRIC: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSINJECTDATA {
    pub DataBytes: super::DWORD,
    pub InjectionPoint: super::WORD,
    pub PageNumber: super::WORD,
}
pub const PSINJECT_BEGINDEFAULTS: i32 = 12;
pub const PSINJECT_BEGINPAGESETUP: i32 = 101;
pub const PSINJECT_BEGINPROLOG: i32 = 14;
pub const PSINJECT_BEGINSETUP: i32 = 16;
pub const PSINJECT_BEGINSTREAM: i32 = 1;
pub const PSINJECT_BOUNDINGBOX: i32 = 9;
pub const PSINJECT_COMMENTS: i32 = 11;
pub const PSINJECT_DLFONT: u32 = 3722304989;
pub const PSINJECT_DOCNEEDEDRES: i32 = 5;
pub const PSINJECT_DOCSUPPLIEDRES: i32 = 6;
pub const PSINJECT_DOCUMENTPROCESSCOLORS: i32 = 10;
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: i32 = 21;
pub const PSINJECT_ENDDEFAULTS: i32 = 13;
pub const PSINJECT_ENDPAGECOMMENTS: i32 = 107;
pub const PSINJECT_ENDPAGESETUP: i32 = 102;
pub const PSINJECT_ENDPROLOG: i32 = 15;
pub const PSINJECT_ENDSETUP: i32 = 17;
pub const PSINJECT_ENDSTREAM: i32 = 20;
pub const PSINJECT_EOF: i32 = 19;
pub const PSINJECT_ORIENTATION: i32 = 8;
pub const PSINJECT_PAGEBBOX: i32 = 106;
pub const PSINJECT_PAGENUMBER: i32 = 100;
pub const PSINJECT_PAGEORDER: i32 = 7;
pub const PSINJECT_PAGES: i32 = 4;
pub const PSINJECT_PAGESATEND: i32 = 3;
pub const PSINJECT_PAGETRAILER: i32 = 103;
pub const PSINJECT_PLATECOLOR: i32 = 104;
pub const PSINJECT_PSADOBE: i32 = 2;
pub const PSINJECT_SHOWPAGE: i32 = 105;
pub const PSINJECT_TRAILER: i32 = 18;
pub const PSINJECT_VMRESTORE: i32 = 201;
pub const PSINJECT_VMSAVE: i32 = 200;
pub const PSPROTOCOL_ASCII: i32 = 0;
pub const PSPROTOCOL_BCP: i32 = 1;
pub const PSPROTOCOL_BINARY: i32 = 3;
pub const PSPROTOCOL_TBCP: i32 = 2;
pub const PS_ALTERNATE: i32 = 8;
pub const PS_COSMETIC: i32 = 0;
pub const PS_DASH: i32 = 1;
pub const PS_DASHDOT: i32 = 3;
pub const PS_DASHDOTDOT: i32 = 4;
pub const PS_DOT: i32 = 2;
pub const PS_ENDCAP_FLAT: i32 = 512;
pub const PS_ENDCAP_MASK: i32 = 3840;
pub const PS_ENDCAP_ROUND: i32 = 0;
pub const PS_ENDCAP_SQUARE: i32 = 256;
pub const PS_GEOMETRIC: i32 = 65536;
pub const PS_INSIDEFRAME: i32 = 6;
pub const PS_JOIN_BEVEL: i32 = 4096;
pub const PS_JOIN_MASK: i32 = 61440;
pub const PS_JOIN_MITER: i32 = 8192;
pub const PS_JOIN_ROUND: i32 = 0;
pub const PS_NULL: i32 = 5;
pub const PS_SOLID: i32 = 0;
pub const PS_STYLE_MASK: i32 = 15;
pub const PS_TYPE_MASK: i32 = 983040;
pub const PS_USERSTYLE: i32 = 7;
#[cfg(feature = "shared")]
pub type PTEXTMETRIC = *mut TEXTMETRICA;
#[cfg(feature = "shared")]
pub type PTEXTMETRICA = *mut TEXTMETRICA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTEXTMETRICW = *mut TEXTMETRICW;
#[cfg(feature = "shared")]
pub type PTRIVERTEX = *mut TRIVERTEX;
pub const PT_BEZIERTO: i32 = 4;
pub const PT_CLOSEFIGURE: i32 = 1;
pub const PT_LINETO: i32 = 2;
pub const PT_MOVETO: i32 = 6;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWCRANGE = *mut WCRANGE;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PWGLSWAP = *mut WGLSWAP;
#[cfg(feature = "minwindef")]
pub type PXFORM = *mut XFORM;
pub const QDC_ALL_PATHS: i32 = 1;
pub const QDC_DATABASE_CURRENT: i32 = 4;
pub const QDC_INCLUDE_HMD: i32 = 32;
pub const QDC_ONLY_ACTIVE_PATHS: i32 = 2;
pub const QDC_VIRTUAL_MODE_AWARE: i32 = 16;
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: i32 = 64;
pub const QDI_DIBTOSCREEN: i32 = 4;
pub const QDI_GETDIBITS: i32 = 2;
pub const QDI_SETDIBITS: i32 = 1;
pub const QDI_STRETCHDIB: i32 = 8;
pub const QUERYDIBSUPPORT: i32 = 3073;
pub const QUERYESCSUPPORT: i32 = 8;
pub const R2_BLACK: i32 = 1;
pub const R2_COPYPEN: i32 = 13;
pub const R2_LAST: i32 = 16;
pub const R2_MASKNOTPEN: i32 = 3;
pub const R2_MASKPEN: i32 = 9;
pub const R2_MASKPENNOT: i32 = 5;
pub const R2_MERGENOTPEN: i32 = 12;
pub const R2_MERGEPEN: i32 = 15;
pub const R2_MERGEPENNOT: i32 = 14;
pub const R2_NOP: i32 = 11;
pub const R2_NOT: i32 = 6;
pub const R2_NOTCOPYPEN: i32 = 4;
pub const R2_NOTMASKPEN: i32 = 8;
pub const R2_NOTMERGEPEN: i32 = 2;
pub const R2_NOTXORPEN: i32 = 10;
pub const R2_WHITE: i32 = 16;
pub const R2_XORPEN: i32 = 7;
pub const RASTERCAPS: i32 = 38;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RASTERIZER_STATUS {
    pub nSize: i16,
    pub wFlags: i16,
    pub nLanguageID: i16,
}
pub const RASTER_FONTTYPE: i32 = 1;
pub const RC_BANDING: i32 = 2;
pub const RC_BIGFONT: i32 = 1024;
pub const RC_BITBLT: i32 = 1;
pub const RC_BITMAP64: i32 = 8;
pub const RC_DEVBITS: i32 = 32768;
pub const RC_DIBTODEV: i32 = 512;
pub const RC_DI_BITMAP: i32 = 128;
pub const RC_FLOODFILL: i32 = 4096;
pub const RC_GDI20_OUTPUT: i32 = 16;
pub const RC_GDI20_STATE: i32 = 32;
pub const RC_OP_DX_OUTPUT: i32 = 16384;
pub const RC_PALETTE: i32 = 256;
pub const RC_SAVEBITMAP: i32 = 64;
pub const RC_SCALING: i32 = 4;
pub const RC_STRETCHBLT: i32 = 2048;
pub const RC_STRETCHDIB: i32 = 8192;
pub const RDH_RECTANGLES: i32 = 1;
#[cfg(feature = "shared")]
pub const REFERENCE_BLACK_MAX: super::WORD = 4000;
#[cfg(feature = "shared")]
pub const REFERENCE_BLACK_MIN: super::WORD = 0;
#[cfg(feature = "shared")]
pub const REFERENCE_WHITE_MAX: super::WORD = 10000;
#[cfg(feature = "shared")]
pub const REFERENCE_WHITE_MIN: super::WORD = 6000;
pub const RELATIVE: i32 = 2;
pub const RESTORE_CTM: i32 = 4100;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RGBQUAD {
    pub rgbBlue: super::BYTE,
    pub rgbGreen: super::BYTE,
    pub rgbRed: super::BYTE,
    pub rgbReserved: super::BYTE,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RGBTRIPLE {
    pub rgbtBlue: super::BYTE,
    pub rgbtGreen: super::BYTE,
    pub rgbtRed: super::BYTE,
}
#[cfg(feature = "shared")]
pub const RGB_GAMMA_MAX: super::WORD = 65000;
#[cfg(feature = "shared")]
pub const RGB_GAMMA_MIN: super::WORD = 1344;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct RGNDATA {
    pub rdh: RGNDATAHEADER,
    pub Buffer: [i8; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for RGNDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct RGNDATAHEADER {
    pub dwSize: super::DWORD,
    pub iType: super::DWORD,
    pub nCount: super::DWORD,
    pub nRgnSize: super::DWORD,
    pub rcBound: super::RECT,
}
pub const RGN_AND: i32 = 1;
pub const RGN_COPY: i32 = 5;
pub const RGN_DIFF: i32 = 4;
pub const RGN_ERROR: i32 = 0;
pub const RGN_MAX: i32 = 5;
pub const RGN_MIN: i32 = 1;
pub const RGN_OR: i32 = 2;
pub const RGN_XOR: i32 = 3;
pub const RUSSIAN_CHARSET: i32 = 204;
pub const SAVE_CTM: i32 = 4101;
pub const SB_CONST_ALPHA: i32 = 1;
pub const SB_GRAD_RECT: i32 = 16;
pub const SB_GRAD_TRI: i32 = 32;
pub const SB_NONE: i32 = 0;
pub const SB_PIXEL_ALPHA: i32 = 2;
pub const SB_PREMULT_ALPHA: i32 = 4;
pub const SCALINGFACTORX: i32 = 114;
pub const SCALINGFACTORY: i32 = 115;
pub const SDC_ALLOW_CHANGES: i32 = 1024;
pub const SDC_ALLOW_PATH_ORDER_CHANGES: i32 = 8192;
pub const SDC_APPLY: i32 = 128;
pub const SDC_FORCE_MODE_ENUMERATION: i32 = 4096;
pub const SDC_NO_OPTIMIZATION: i32 = 256;
pub const SDC_PATH_PERSIST_IF_REQUIRED: i32 = 2048;
pub const SDC_SAVE_TO_DATABASE: i32 = 512;
pub const SDC_TOPOLOGY_CLONE: i32 = 2;
pub const SDC_TOPOLOGY_EXTEND: i32 = 4;
pub const SDC_TOPOLOGY_EXTERNAL: i32 = 8;
pub const SDC_TOPOLOGY_INTERNAL: i32 = 1;
pub const SDC_TOPOLOGY_SUPPLIED: i32 = 16;
pub const SDC_USE_DATABASE_CURRENT: i32 = 15;
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: i32 = 32;
pub const SDC_VALIDATE: i32 = 64;
pub const SDC_VIRTUAL_MODE_AWARE: i32 = 32768;
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: i32 = 131072;
pub const SELECTPAPERSOURCE: i32 = 18;
pub const SETABORTPROC: i32 = 9;
pub const SETALLJUSTVALUES: i32 = 771;
pub const SETCHARSET: i32 = 772;
pub const SETCOLORTABLE: i32 = 4;
pub const SETCOPYCOUNT: i32 = 17;
pub const SETDIBSCALING: i32 = 32;
pub const SETICMPROFILE_EMBEDED: i32 = 1;
pub const SETKERNTRACK: i32 = 770;
pub const SETLINECAP: i32 = 21;
pub const SETLINEJOIN: i32 = 22;
pub const SETMITERLIMIT: i32 = 23;
pub const SET_ARC_DIRECTION: i32 = 4102;
pub const SET_BACKGROUND_COLOR: i32 = 4103;
pub const SET_BOUNDS: i32 = 4109;
pub const SET_CLIP_BOX: i32 = 4108;
pub const SET_MIRROR_MODE: i32 = 4110;
pub const SET_POLY_MODE: i32 = 4104;
pub const SET_SCREEN_ANGLE: i32 = 4105;
pub const SET_SPREAD: i32 = 4106;
pub const SHADEBLENDCAPS: i32 = 120;
pub const SHIFTJIS_CHARSET: i32 = 128;
pub const SIMPLEREGION: i32 = 2;
pub const SIZEPALETTE: i32 = 104;
pub const SPCLPASSTHROUGH2: i32 = 4568;
pub const SP_APPABORT: i32 = -2;
pub const SP_ERROR: i32 = -1;
pub const SP_NOTREPORTED: i32 = 16384;
pub const SP_OUTOFDISK: i32 = -4;
pub const SP_OUTOFMEMORY: i32 = -5;
pub const SP_USERABORT: i32 = -3;
#[cfg(feature = "shared")]
pub const SRCAND: super::DWORD = 8913094;
#[cfg(feature = "shared")]
pub const SRCCOPY: super::DWORD = 13369376;
#[cfg(feature = "shared")]
pub const SRCERASE: super::DWORD = 4457256;
#[cfg(feature = "shared")]
pub const SRCINVERT: super::DWORD = 6684742;
#[cfg(feature = "shared")]
pub const SRCPAINT: super::DWORD = 15597702;
pub const STAMP_AXESLIST: i32 = 134245473;
pub const STAMP_CFF2: i32 = 134248035;
pub const STAMP_DESIGNVECTOR: i32 = 134248036;
pub const STAMP_TRUETYPE_VARIATION: i32 = 134248052;
pub const STARTDOC: i32 = 10;
pub const STOCK_LAST: i32 = 19;
pub const STRETCHBLT: i32 = 2048;
pub const STRETCH_ANDSCANS: i32 = 1;
pub const STRETCH_DELETESCANS: i32 = 3;
pub const STRETCH_HALFTONE: i32 = 4;
pub const STRETCH_ORSCANS: i32 = 2;
pub const SYMBOL_CHARSET: i32 = 2;
pub const SYSPAL_ERROR: i32 = 0;
pub const SYSPAL_NOSTATIC: i32 = 2;
pub const SYSPAL_NOSTATIC256: i32 = 3;
pub const SYSPAL_STATIC: i32 = 1;
pub const SYSRGN: i32 = 4;
pub const SYSTEM_FIXED_FONT: i32 = 16;
pub const SYSTEM_FONT: i32 = 13;
pub const TA_BASELINE: i32 = 24;
pub const TA_BOTTOM: i32 = 8;
pub const TA_CENTER: i32 = 6;
pub const TA_LEFT: i32 = 0;
pub const TA_MASK: i32 = 287;
pub const TA_NOUPDATECP: i32 = 0;
pub const TA_RIGHT: i32 = 2;
pub const TA_RTLREADING: i32 = 256;
pub const TA_TOP: i32 = 0;
pub const TA_UPDATECP: i32 = 1;
pub const TCI_SRCCHARSET: i32 = 1;
pub const TCI_SRCCODEPAGE: i32 = 2;
pub const TCI_SRCFONTSIG: i32 = 3;
pub const TCI_SRCLOCALE: i32 = 4096;
pub const TC_CP_STROKE: i32 = 4;
pub const TC_CR_90: i32 = 8;
pub const TC_CR_ANY: i32 = 16;
pub const TC_EA_DOUBLE: i32 = 512;
pub const TC_IA_ABLE: i32 = 1024;
pub const TC_OP_CHARACTER: i32 = 1;
pub const TC_OP_STROKE: i32 = 2;
pub const TC_RA_ABLE: i32 = 8192;
pub const TC_RESERVED: i32 = 32768;
pub const TC_SA_CONTIN: i32 = 256;
pub const TC_SA_DOUBLE: i32 = 64;
pub const TC_SA_INTEGER: i32 = 128;
pub const TC_SCROLLBLT: i32 = 65536;
pub const TC_SF_X_YINDEP: i32 = 32;
pub const TC_SO_ABLE: i32 = 4096;
pub const TC_UA_ABLE: i32 = 2048;
pub const TC_VA_ABLE: i32 = 16384;
pub const TECHNOLOGY: i32 = 2;
pub const TEXTCAPS: i32 = 34;
#[cfg(feature = "shared")]
pub type TEXTMETRIC = TEXTMETRICA;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TEXTMETRICA {
    pub tmHeight: super::LONG,
    pub tmAscent: super::LONG,
    pub tmDescent: super::LONG,
    pub tmInternalLeading: super::LONG,
    pub tmExternalLeading: super::LONG,
    pub tmAveCharWidth: super::LONG,
    pub tmMaxCharWidth: super::LONG,
    pub tmWeight: super::LONG,
    pub tmOverhang: super::LONG,
    pub tmDigitizedAspectX: super::LONG,
    pub tmDigitizedAspectY: super::LONG,
    pub tmFirstChar: super::BYTE,
    pub tmLastChar: super::BYTE,
    pub tmDefaultChar: super::BYTE,
    pub tmBreakChar: super::BYTE,
    pub tmItalic: super::BYTE,
    pub tmUnderlined: super::BYTE,
    pub tmStruckOut: super::BYTE,
    pub tmPitchAndFamily: super::BYTE,
    pub tmCharSet: super::BYTE,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TEXTMETRICW {
    pub tmHeight: super::LONG,
    pub tmAscent: super::LONG,
    pub tmDescent: super::LONG,
    pub tmInternalLeading: super::LONG,
    pub tmExternalLeading: super::LONG,
    pub tmAveCharWidth: super::LONG,
    pub tmMaxCharWidth: super::LONG,
    pub tmWeight: super::LONG,
    pub tmOverhang: super::LONG,
    pub tmDigitizedAspectX: super::LONG,
    pub tmDigitizedAspectY: super::LONG,
    pub tmFirstChar: super::WCHAR,
    pub tmLastChar: super::WCHAR,
    pub tmDefaultChar: super::WCHAR,
    pub tmBreakChar: super::WCHAR,
    pub tmItalic: super::BYTE,
    pub tmUnderlined: super::BYTE,
    pub tmStruckOut: super::BYTE,
    pub tmPitchAndFamily: super::BYTE,
    pub tmCharSet: super::BYTE,
}
pub const THAI_CHARSET: i32 = 222;
pub const TMPF_DEVICE: i32 = 8;
pub const TMPF_FIXED_PITCH: i32 = 1;
pub const TMPF_TRUETYPE: i32 = 4;
pub const TMPF_VECTOR: i32 = 2;
pub const TRANSFORM_CTM: i32 = 4107;
pub const TRANSPARENT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TRIVERTEX {
    pub x: super::LONG,
    pub y: super::LONG,
    pub Red: COLOR16,
    pub Green: COLOR16,
    pub Blue: COLOR16,
    pub Alpha: COLOR16,
}
pub const TRUETYPE_FONTTYPE: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct TTPOLYCURVE {
    pub wType: super::WORD,
    pub cpfx: super::WORD,
    pub apfx: [POINTFX; 1],
}
#[cfg(feature = "shared")]
impl Default for TTPOLYCURVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TTPOLYGONHEADER {
    pub cb: super::DWORD,
    pub dwType: super::DWORD,
    pub pfxStart: POINTFX,
}
pub const TT_AVAILABLE: i32 = 1;
pub const TT_ENABLED: i32 = 2;
pub const TT_POLYGON_TYPE: i32 = 24;
pub const TT_PRIM_CSPLINE: i32 = 3;
pub const TT_PRIM_LINE: i32 = 1;
pub const TT_PRIM_QSPLINE: i32 = 2;
pub const TURKISH_CHARSET: i32 = 162;
pub const VARIABLE_PITCH: i32 = 2;
pub const VERTRES: i32 = 10;
pub const VERTSIZE: i32 = 6;
pub const VIETNAMESE_CHARSET: i32 = 163;
pub const VREFRESH: i32 = 116;
pub const VTA_BASELINE: i32 = 24;
pub const VTA_BOTTOM: i32 = 2;
pub const VTA_CENTER: i32 = 6;
pub const VTA_LEFT: i32 = 8;
pub const VTA_RIGHT: i32 = 0;
pub const VTA_TOP: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WCRANGE {
    pub wcLow: super::WCHAR,
    pub cGlyphs: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct WGLSWAP {
    pub hdc: super::HDC,
    pub uiFlags: super::UINT,
}
pub const WGL_FONT_LINES: i32 = 0;
pub const WGL_FONT_POLYGONS: i32 = 1;
pub const WGL_SWAPMULTIPLE_MAX: i32 = 16;
pub const WGL_SWAP_MAIN_PLANE: i32 = 1;
pub const WGL_SWAP_OVERLAY1: i32 = 2;
pub const WGL_SWAP_OVERLAY10: i32 = 1024;
pub const WGL_SWAP_OVERLAY11: i32 = 2048;
pub const WGL_SWAP_OVERLAY12: i32 = 4096;
pub const WGL_SWAP_OVERLAY13: i32 = 8192;
pub const WGL_SWAP_OVERLAY14: i32 = 16384;
pub const WGL_SWAP_OVERLAY15: i32 = 32768;
pub const WGL_SWAP_OVERLAY2: i32 = 4;
pub const WGL_SWAP_OVERLAY3: i32 = 8;
pub const WGL_SWAP_OVERLAY4: i32 = 16;
pub const WGL_SWAP_OVERLAY5: i32 = 32;
pub const WGL_SWAP_OVERLAY6: i32 = 64;
pub const WGL_SWAP_OVERLAY7: i32 = 128;
pub const WGL_SWAP_OVERLAY8: i32 = 256;
pub const WGL_SWAP_OVERLAY9: i32 = 512;
pub const WGL_SWAP_UNDERLAY1: i32 = 65536;
pub const WGL_SWAP_UNDERLAY10: i32 = 33554432;
pub const WGL_SWAP_UNDERLAY11: i32 = 67108864;
pub const WGL_SWAP_UNDERLAY12: i32 = 134217728;
pub const WGL_SWAP_UNDERLAY13: i32 = 268435456;
pub const WGL_SWAP_UNDERLAY14: i32 = 536870912;
pub const WGL_SWAP_UNDERLAY15: i32 = 1073741824;
pub const WGL_SWAP_UNDERLAY2: i32 = 131072;
pub const WGL_SWAP_UNDERLAY3: i32 = 262144;
pub const WGL_SWAP_UNDERLAY4: i32 = 524288;
pub const WGL_SWAP_UNDERLAY5: i32 = 1048576;
pub const WGL_SWAP_UNDERLAY6: i32 = 2097152;
pub const WGL_SWAP_UNDERLAY7: i32 = 4194304;
pub const WGL_SWAP_UNDERLAY8: i32 = 8388608;
pub const WGL_SWAP_UNDERLAY9: i32 = 16777216;
#[cfg(feature = "shared")]
pub const WHITENESS: super::DWORD = 16711778;
pub const WHITEONBLACK: i32 = 2;
pub const WHITE_BRUSH: i32 = 0;
pub const WHITE_PEN: i32 = 6;
pub const WINDING: i32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XFORM {
    pub eM11: super::FLOAT,
    pub eM12: super::FLOAT,
    pub eM21: super::FLOAT,
    pub eM22: super::FLOAT,
    pub eDx: super::FLOAT,
    pub eDy: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagABORTPATH {
    pub emr: EMR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRARC {
    pub emr: EMR,
    pub rclBox: super::RECTL,
    pub ptlStart: super::POINTL,
    pub ptlEnd: super::POINTL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRELLIPSE {
    pub emr: EMR,
    pub rclBox: super::RECTL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMREXCLUDECLIPRECT {
    pub emr: EMR,
    pub rclClip: super::RECTL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagEMREXTESCAPE {
    pub emr: EMR,
    pub iEscape: super::INT,
    pub cbEscData: super::INT,
    pub EscData: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for tagEMREXTESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMREXTTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub iGraphicsMode: super::DWORD,
    pub exScale: super::FLOAT,
    pub eyScale: super::FLOAT,
    pub emrtext: EMRTEXT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRFILLPATH {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagEMRINVERTRGN {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cbRgnData: super::DWORD,
    pub RgnData: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for tagEMRINVERTRGN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRLINETO {
    pub emr: EMR,
    pub ptl: super::POINTL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagEMRPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cptl: super::DWORD,
    pub aptl: [super::POINTL; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for tagEMRPOLYLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagEMRPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub cpts: super::DWORD,
    pub apts: [super::POINTS; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for tagEMRPOLYLINE16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagEMRPOLYPOLYLINE {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub nPolys: super::DWORD,
    pub cptl: super::DWORD,
    pub aPolyCounts: [super::DWORD; 1],
    pub aptl: [super::POINTL; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for tagEMRPOLYPOLYLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagEMRPOLYPOLYLINE16 {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub nPolys: super::DWORD,
    pub cpts: super::DWORD,
    pub aPolyCounts: [super::DWORD; 1],
    pub apts: [super::POINTS; 1],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for tagEMRPOLYPOLYLINE16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct tagEMRPOLYTEXTOUTA {
    pub emr: EMR,
    pub rclBounds: super::RECTL,
    pub iGraphicsMode: super::DWORD,
    pub exScale: super::FLOAT,
    pub eyScale: super::FLOAT,
    pub cStrings: super::LONG,
    pub aemrtext: [EMRTEXT; 1],
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl Default for tagEMRPOLYTEXTOUTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSCALEVIEWPORTEXTEX {
    pub emr: EMR,
    pub xNum: super::LONG,
    pub xDenom: super::LONG,
    pub yNum: super::LONG,
    pub yDenom: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSELECTCLIPPATH {
    pub emr: EMR,
    pub iMode: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSELECTOBJECT {
    pub emr: EMR,
    pub ihObject: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSETCOLORSPACE {
    pub emr: EMR,
    pub ihCS: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagEMRSETICMPROFILE {
    pub emr: EMR,
    pub dwFlags: super::DWORD,
    pub cbName: super::DWORD,
    pub cbData: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for tagEMRSETICMPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSETTEXTCOLOR {
    pub emr: EMR,
    pub crColor: super::COLORREF,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSETVIEWPORTEXTEX {
    pub emr: EMR,
    pub szlExtent: super::SIZEL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagEMRSETVIEWPORTORGEX {
    pub emr: EMR,
    pub ptlOrigin: super::POINTL,
}
