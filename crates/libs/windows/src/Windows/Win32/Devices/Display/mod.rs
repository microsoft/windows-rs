#[inline]
pub unsafe fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE {
    windows_link::link!("gdi32.dll" "system" fn BRUSHOBJ_hGetColorTransform(pbo : *mut BRUSHOBJ) -> super::super::Foundation:: HANDLE);
    unsafe { BRUSHOBJ_hGetColorTransform(pbo as _) }
}
#[inline]
pub unsafe fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut core::ffi::c_void {
    windows_link::link!("gdi32.dll" "system" fn BRUSHOBJ_pvAllocRbrush(pbo : *mut BRUSHOBJ, cj : u32) -> *mut core::ffi::c_void);
    unsafe { BRUSHOBJ_pvAllocRbrush(pbo as _, cj) }
}
#[inline]
pub unsafe fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut core::ffi::c_void {
    windows_link::link!("gdi32.dll" "system" fn BRUSHOBJ_pvGetRbrush(pbo : *mut BRUSHOBJ) -> *mut core::ffi::c_void);
    unsafe { BRUSHOBJ_pvGetRbrush(pbo as _) }
}
#[inline]
pub unsafe fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn BRUSHOBJ_ulGetBrushColor(pbo : *mut BRUSHOBJ) -> u32);
    unsafe { BRUSHOBJ_ulGetBrushColor(pbo as _) }
}
#[inline]
pub unsafe fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn CLIPOBJ_bEnum(pco : *mut CLIPOBJ, cj : u32, pul : *mut u32) -> windows_core::BOOL);
    unsafe { CLIPOBJ_bEnum(pco as _, cj, pul as _) }
}
#[inline]
pub unsafe fn CLIPOBJ_cEnumStart(pco: *mut CLIPOBJ, ball: bool, itype: u32, idirection: u32, climit: u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn CLIPOBJ_cEnumStart(pco : *mut CLIPOBJ, ball : windows_core::BOOL, itype : u32, idirection : u32, climit : u32) -> u32);
    unsafe { CLIPOBJ_cEnumStart(pco as _, ball.into(), itype, idirection, climit) }
}
#[inline]
pub unsafe fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ {
    windows_link::link!("gdi32.dll" "system" fn CLIPOBJ_ppoGetPath(pco : *mut CLIPOBJ) -> *mut PATHOBJ);
    unsafe { CLIPOBJ_ppoGetPath(pco as _) }
}
#[inline]
pub unsafe fn CapabilitiesRequestAndCapabilitiesReply(hmonitor: super::super::Foundation::HANDLE, pszasciicapabilitiesstring: &mut [u8]) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn CapabilitiesRequestAndCapabilitiesReply(hmonitor : super::super::Foundation:: HANDLE, pszasciicapabilitiesstring : windows_core::PSTR, dwcapabilitiesstringlengthincharacters : u32) -> i32);
    unsafe { CapabilitiesRequestAndCapabilitiesReply(hmonitor, core::mem::transmute(pszasciicapabilitiesstring.as_ptr()), pszasciicapabilitiesstring.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn DegaussMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn DegaussMonitor(hmonitor : super::super::Foundation:: HANDLE) -> i32);
    unsafe { DegaussMonitor(hmonitor) }
}
#[inline]
pub unsafe fn DestroyPhysicalMonitor(hmonitor: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_link::link!("dxva2.dll" "system" fn DestroyPhysicalMonitor(hmonitor : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { DestroyPhysicalMonitor(hmonitor).ok() }
}
#[inline]
pub unsafe fn DestroyPhysicalMonitors(pphysicalmonitorarray: &[PHYSICAL_MONITOR]) -> windows_core::Result<()> {
    windows_link::link!("dxva2.dll" "system" fn DestroyPhysicalMonitors(dwphysicalmonitorarraysize : u32, pphysicalmonitorarray : *const PHYSICAL_MONITOR) -> windows_core::BOOL);
    unsafe { DestroyPhysicalMonitors(pphysicalmonitorarray.len().try_into().unwrap(), core::mem::transmute(pphysicalmonitorarray.as_ptr())).ok() }
}
#[inline]
pub unsafe fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    windows_link::link!("user32.dll" "system" fn DisplayConfigGetDeviceInfo(requestpacket : *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32);
    unsafe { DisplayConfigGetDeviceInfo(requestpacket as _) }
}
#[inline]
pub unsafe fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    windows_link::link!("user32.dll" "system" fn DisplayConfigSetDeviceInfo(setpacket : *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32);
    unsafe { DisplayConfigSetDeviceInfo(setpacket) }
}
#[inline]
pub unsafe fn EngAcquireSemaphore(hsem: HSEMAPHORE) {
    windows_link::link!("gdi32.dll" "system" fn EngAcquireSemaphore(hsem : HSEMAPHORE));
    unsafe { EngAcquireSemaphore(hsem) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngAlphaBlend(psodest : *mut SURFOBJ, psosrc : *mut SURFOBJ, pco : *mut CLIPOBJ, pxlo : *mut XLATEOBJ, prcldest : *mut super::super::Foundation:: RECTL, prclsrc : *mut super::super::Foundation:: RECTL, pblendobj : *mut BLENDOBJ) -> windows_core::BOOL);
    unsafe { EngAlphaBlend(psodest as _, psosrc as _, pco as _, pxlo as _, prcldest as _, prclsrc as _, pblendobj as _) }
}
#[inline]
pub unsafe fn EngAssociateSurface(hsurf: HSURF, hdev: HDEV, flhooks: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngAssociateSurface(hsurf : HSURF, hdev : HDEV, flhooks : u32) -> windows_core::BOOL);
    unsafe { EngAssociateSurface(hsurf, hdev, flhooks) }
}
#[inline]
pub unsafe fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngBitBlt(psotrg : *const SURFOBJ, psosrc : *const SURFOBJ, psomask : *const SURFOBJ, pco : *const CLIPOBJ, pxlo : *const XLATEOBJ, prcltrg : *const super::super::Foundation:: RECTL, pptlsrc : *const super::super::Foundation:: POINTL, pptlmask : *const super::super::Foundation:: POINTL, pbo : *const BRUSHOBJ, pptlbrush : *const super::super::Foundation:: POINTL, rop4 : u32) -> windows_core::BOOL);
    unsafe { EngBitBlt(psotrg, psosrc, psomask, pco, pxlo, prcltrg, pptlsrc, pptlmask, pbo, pptlbrush, rop4) }
}
#[inline]
pub unsafe fn EngCheckAbort(pso: *mut SURFOBJ) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngCheckAbort(pso : *mut SURFOBJ) -> windows_core::BOOL);
    unsafe { EngCheckAbort(pso as _) }
}
#[inline]
pub unsafe fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET {
    windows_link::link!("gdi32.dll" "system" fn EngComputeGlyphSet(ncodepage : i32, nfirstchar : i32, cchars : i32) -> *mut FD_GLYPHSET);
    unsafe { EngComputeGlyphSet(ncodepage, nfirstchar, cchars) }
}
#[inline]
pub unsafe fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngCopyBits(psodest : *mut SURFOBJ, psosrc : *mut SURFOBJ, pco : *mut CLIPOBJ, pxlo : *mut XLATEOBJ, prcldest : *mut super::super::Foundation:: RECTL, pptlsrc : *mut super::super::Foundation:: POINTL) -> windows_core::BOOL);
    unsafe { EngCopyBits(psodest as _, psosrc as _, pco as _, pxlo as _, prcldest as _, pptlsrc as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngCreateBitmap(sizl: super::super::Foundation::SIZE, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut core::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn EngCreateBitmap(sizl : super::super::Foundation:: SIZE, lwidth : i32, iformat : u32, fl : u32, pvbits : *mut core::ffi::c_void) -> super::super::Graphics::Gdi:: HBITMAP);
    unsafe { EngCreateBitmap(core::mem::transmute(sizl), lwidth, iformat, fl, pvbits as _) }
}
#[inline]
pub unsafe fn EngCreateClip() -> *mut CLIPOBJ {
    windows_link::link!("gdi32.dll" "system" fn EngCreateClip() -> *mut CLIPOBJ);
    unsafe { EngCreateClip() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngCreateDeviceBitmap(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP {
    windows_link::link!("gdi32.dll" "system" fn EngCreateDeviceBitmap(dhsurf : DHSURF, sizl : super::super::Foundation:: SIZE, iformatcompat : u32) -> super::super::Graphics::Gdi:: HBITMAP);
    unsafe { EngCreateDeviceBitmap(dhsurf, core::mem::transmute(sizl), iformatcompat) }
}
#[inline]
pub unsafe fn EngCreateDeviceSurface(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> HSURF {
    windows_link::link!("gdi32.dll" "system" fn EngCreateDeviceSurface(dhsurf : DHSURF, sizl : super::super::Foundation:: SIZE, iformatcompat : u32) -> HSURF);
    unsafe { EngCreateDeviceSurface(dhsurf, core::mem::transmute(sizl), iformatcompat) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE {
    windows_link::link!("gdi32.dll" "system" fn EngCreatePalette(imode : u32, ccolors : u32, pulcolors : *mut u32, flred : u32, flgreen : u32, flblue : u32) -> super::super::Graphics::Gdi:: HPALETTE);
    unsafe { EngCreatePalette(imode, ccolors, pulcolors as _, flred, flgreen, flblue) }
}
#[inline]
pub unsafe fn EngCreateSemaphore() -> HSEMAPHORE {
    windows_link::link!("gdi32.dll" "system" fn EngCreateSemaphore() -> HSEMAPHORE);
    unsafe { EngCreateSemaphore() }
}
#[inline]
pub unsafe fn EngDeleteClip(pco: Option<*const CLIPOBJ>) {
    windows_link::link!("gdi32.dll" "system" fn EngDeleteClip(pco : *const CLIPOBJ));
    unsafe { EngDeleteClip(pco.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngDeletePalette(hpal: super::super::Graphics::Gdi::HPALETTE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngDeletePalette(hpal : super::super::Graphics::Gdi:: HPALETTE) -> windows_core::BOOL);
    unsafe { EngDeletePalette(hpal) }
}
#[inline]
pub unsafe fn EngDeletePath(ppo: *mut PATHOBJ) {
    windows_link::link!("gdi32.dll" "system" fn EngDeletePath(ppo : *mut PATHOBJ));
    unsafe { EngDeletePath(ppo as _) }
}
#[inline]
pub unsafe fn EngDeleteSemaphore(hsem: HSEMAPHORE) {
    windows_link::link!("gdi32.dll" "system" fn EngDeleteSemaphore(hsem : HSEMAPHORE));
    unsafe { EngDeleteSemaphore(hsem) }
}
#[inline]
pub unsafe fn EngDeleteSurface(hsurf: HSURF) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngDeleteSurface(hsurf : HSURF) -> windows_core::BOOL);
    unsafe { EngDeleteSurface(hsurf) }
}
#[inline]
pub unsafe fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngEraseSurface(pso : *mut SURFOBJ, prcl : *mut super::super::Foundation:: RECTL, icolor : u32) -> windows_core::BOOL);
    unsafe { EngEraseSurface(pso as _, prcl as _, icolor) }
}
#[inline]
pub unsafe fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngFillPath(pso : *mut SURFOBJ, ppo : *mut PATHOBJ, pco : *mut CLIPOBJ, pbo : *mut BRUSHOBJ, pptlbrushorg : *mut super::super::Foundation:: POINTL, mix : u32, floptions : u32) -> windows_core::BOOL);
    unsafe { EngFillPath(pso as _, ppo as _, pco as _, pbo as _, pptlbrushorg as _, mix, floptions) }
}
#[inline]
pub unsafe fn EngFindResource(h: super::super::Foundation::HANDLE, iname: i32, itype: i32, pulsize: *mut u32) -> *mut core::ffi::c_void {
    windows_link::link!("gdi32.dll" "system" fn EngFindResource(h : super::super::Foundation:: HANDLE, iname : i32, itype : i32, pulsize : *mut u32) -> *mut core::ffi::c_void);
    unsafe { EngFindResource(h, iname, itype, pulsize as _) }
}
#[inline]
pub unsafe fn EngFreeModule(h: super::super::Foundation::HANDLE) {
    windows_link::link!("gdi32.dll" "system" fn EngFreeModule(h : super::super::Foundation:: HANDLE));
    unsafe { EngFreeModule(h) }
}
#[inline]
pub unsafe fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16) {
    windows_link::link!("gdi32.dll" "system" fn EngGetCurrentCodePage(oemcodepage : *mut u16, ansicodepage : *mut u16));
    unsafe { EngGetCurrentCodePage(oemcodepage as _, ansicodepage as _) }
}
#[inline]
pub unsafe fn EngGetDriverName(hdev: HDEV) -> windows_core::PWSTR {
    windows_link::link!("gdi32.dll" "system" fn EngGetDriverName(hdev : HDEV) -> windows_core::PWSTR);
    unsafe { EngGetDriverName(hdev) }
}
#[inline]
pub unsafe fn EngGetPrinterDataFileName(hdev: HDEV) -> windows_core::PWSTR {
    windows_link::link!("gdi32.dll" "system" fn EngGetPrinterDataFileName(hdev : HDEV) -> windows_core::PWSTR);
    unsafe { EngGetPrinterDataFileName(hdev) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut core::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngGradientFill(psodest : *mut SURFOBJ, pco : *mut CLIPOBJ, pxlo : *mut XLATEOBJ, pvertex : *mut super::super::Graphics::Gdi:: TRIVERTEX, nvertex : u32, pmesh : *mut core::ffi::c_void, nmesh : u32, prclextents : *mut super::super::Foundation:: RECTL, pptlditherorg : *mut super::super::Foundation:: POINTL, ulmode : u32) -> windows_core::BOOL);
    unsafe { EngGradientFill(psodest as _, pco as _, pxlo as _, pvertex as _, nvertex, pmesh as _, nmesh, prclextents as _, pptlditherorg as _, ulmode) }
}
#[inline]
pub unsafe fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngLineTo(pso : *mut SURFOBJ, pco : *mut CLIPOBJ, pbo : *mut BRUSHOBJ, x1 : i32, y1 : i32, x2 : i32, y2 : i32, prclbounds : *mut super::super::Foundation:: RECTL, mix : u32) -> windows_core::BOOL);
    unsafe { EngLineTo(pso as _, pco as _, pbo as _, x1, y1, x2, y2, prclbounds as _, mix) }
}
#[inline]
pub unsafe fn EngLoadModule<P0>(pwsz: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EngLoadModule(pwsz : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    unsafe { EngLoadModule(pwsz.param().abi()) }
}
#[inline]
pub unsafe fn EngLockSurface(hsurf: HSURF) -> *mut SURFOBJ {
    windows_link::link!("gdi32.dll" "system" fn EngLockSurface(hsurf : HSURF) -> *mut SURFOBJ);
    unsafe { EngLockSurface(hsurf) }
}
#[inline]
pub unsafe fn EngMarkBandingSurface(hsurf: HSURF) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngMarkBandingSurface(hsurf : HSURF) -> windows_core::BOOL);
    unsafe { EngMarkBandingSurface(hsurf) }
}
#[inline]
pub unsafe fn EngMultiByteToUnicodeN(unicodestring: windows_core::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: Option<*mut u32>, multibytestring: &[u8]) {
    windows_link::link!("gdi32.dll" "system" fn EngMultiByteToUnicodeN(unicodestring : windows_core::PWSTR, maxbytesinunicodestring : u32, bytesinunicodestring : *mut u32, multibytestring : windows_core::PCSTR, bytesinmultibytestring : u32));
    unsafe { EngMultiByteToUnicodeN(core::mem::transmute(unicodestring), maxbytesinunicodestring, bytesinunicodestring.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn EngMultiByteToWideChar(codepage: u32, widecharstring: Option<windows_core::PWSTR>, bytesinwidecharstring: i32, multibytestring: Option<&[u8]>) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn EngMultiByteToWideChar(codepage : u32, widecharstring : windows_core::PWSTR, bytesinwidecharstring : i32, multibytestring : windows_core::PCSTR, bytesinmultibytestring : i32) -> i32);
    unsafe { EngMultiByteToWideChar(codepage, widecharstring.unwrap_or(core::mem::zeroed()) as _, bytesinwidecharstring, core::mem::transmute(multibytestring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), multibytestring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngPaint(pso : *mut SURFOBJ, pco : *mut CLIPOBJ, pbo : *mut BRUSHOBJ, pptlbrushorg : *mut super::super::Foundation:: POINTL, mix : u32) -> windows_core::BOOL);
    unsafe { EngPaint(pso as _, pco as _, pbo as _, pptlbrushorg as _, mix) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngPlgBlt(psotrg : *mut SURFOBJ, psosrc : *mut SURFOBJ, psomsk : *mut SURFOBJ, pco : *mut CLIPOBJ, pxlo : *mut XLATEOBJ, pca : *mut super::super::Graphics::Gdi:: COLORADJUSTMENT, pptlbrushorg : *mut super::super::Foundation:: POINTL, pptfx : *mut POINTFIX, prcl : *mut super::super::Foundation:: RECTL, pptl : *mut super::super::Foundation:: POINTL, imode : u32) -> windows_core::BOOL);
    unsafe { EngPlgBlt(psotrg as _, psosrc as _, psomsk as _, pco as _, pxlo as _, pca as _, pptlbrushorg as _, pptfx as _, prcl as _, pptl as _, imode) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngQueryEMFInfo(hdev: HDEV, pemfinfo: *mut EMFINFO) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngQueryEMFInfo(hdev : HDEV, pemfinfo : *mut EMFINFO) -> windows_core::BOOL);
    unsafe { EngQueryEMFInfo(hdev, pemfinfo as _) }
}
#[inline]
pub unsafe fn EngQueryLocalTime() -> ENG_TIME_FIELDS {
    windows_link::link!("gdi32.dll" "system" fn EngQueryLocalTime(param0 : *mut ENG_TIME_FIELDS));
    unsafe {
        let mut result__ = core::mem::zeroed();
        EngQueryLocalTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn EngReleaseSemaphore(hsem: HSEMAPHORE) {
    windows_link::link!("gdi32.dll" "system" fn EngReleaseSemaphore(hsem : HSEMAPHORE));
    unsafe { EngReleaseSemaphore(hsem) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngStretchBlt(psodest : *mut SURFOBJ, psosrc : *mut SURFOBJ, psomask : *mut SURFOBJ, pco : *mut CLIPOBJ, pxlo : *mut XLATEOBJ, pca : *mut super::super::Graphics::Gdi:: COLORADJUSTMENT, pptlhtorg : *mut super::super::Foundation:: POINTL, prcldest : *mut super::super::Foundation:: RECTL, prclsrc : *mut super::super::Foundation:: RECTL, pptlmask : *mut super::super::Foundation:: POINTL, imode : u32) -> windows_core::BOOL);
    unsafe { EngStretchBlt(psodest as _, psosrc as _, psomask as _, pco as _, pxlo as _, pca as _, pptlhtorg as _, prcldest as _, prclsrc as _, pptlmask as _, imode) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngStretchBltROP(psodest : *mut SURFOBJ, psosrc : *mut SURFOBJ, psomask : *mut SURFOBJ, pco : *mut CLIPOBJ, pxlo : *mut XLATEOBJ, pca : *mut super::super::Graphics::Gdi:: COLORADJUSTMENT, pptlhtorg : *mut super::super::Foundation:: POINTL, prcldest : *mut super::super::Foundation:: RECTL, prclsrc : *mut super::super::Foundation:: RECTL, pptlmask : *mut super::super::Foundation:: POINTL, imode : u32, pbo : *mut BRUSHOBJ, rop4 : u32) -> windows_core::BOOL);
    unsafe { EngStretchBltROP(psodest as _, psosrc as _, psomask as _, pco as _, pxlo as _, pca as _, pptlhtorg as _, prcldest as _, prclsrc as _, pptlmask as _, imode, pbo as _, rop4) }
}
#[inline]
pub unsafe fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngStrokeAndFillPath(pso : *mut SURFOBJ, ppo : *mut PATHOBJ, pco : *mut CLIPOBJ, pxo : *mut XFORMOBJ, pbostroke : *mut BRUSHOBJ, plineattrs : *mut LINEATTRS, pbofill : *mut BRUSHOBJ, pptlbrushorg : *mut super::super::Foundation:: POINTL, mixfill : u32, floptions : u32) -> windows_core::BOOL);
    unsafe { EngStrokeAndFillPath(pso as _, ppo as _, pco as _, pxo as _, pbostroke as _, plineattrs as _, pbofill as _, pptlbrushorg as _, mixfill, floptions) }
}
#[inline]
pub unsafe fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngStrokePath(pso : *mut SURFOBJ, ppo : *mut PATHOBJ, pco : *mut CLIPOBJ, pxo : *mut XFORMOBJ, pbo : *mut BRUSHOBJ, pptlbrushorg : *mut super::super::Foundation:: POINTL, plineattrs : *mut LINEATTRS, mix : u32) -> windows_core::BOOL);
    unsafe { EngStrokePath(pso as _, ppo as _, pco as _, pxo as _, pbo as _, pptlbrushorg as _, plineattrs as _, mix) }
}
#[inline]
pub unsafe fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngTextOut(pso : *mut SURFOBJ, pstro : *mut STROBJ, pfo : *mut FONTOBJ, pco : *mut CLIPOBJ, prclextra : *mut super::super::Foundation:: RECTL, prclopaque : *mut super::super::Foundation:: RECTL, pbofore : *mut BRUSHOBJ, pboopaque : *mut BRUSHOBJ, pptlorg : *mut super::super::Foundation:: POINTL, mix : u32) -> windows_core::BOOL);
    unsafe { EngTextOut(pso as _, pstro as _, pfo as _, pco as _, prclextra as _, prclopaque as _, pbofore as _, pboopaque as _, pptlorg as _, mix) }
}
#[inline]
pub unsafe fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: Option<*const CLIPOBJ>, pxlo: Option<*const XLATEOBJ>, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn EngTransparentBlt(psodst : *const SURFOBJ, psosrc : *const SURFOBJ, pco : *const CLIPOBJ, pxlo : *const XLATEOBJ, prcldst : *const super::super::Foundation:: RECTL, prclsrc : *const super::super::Foundation:: RECTL, transcolor : u32, bcalledfrombitblt : u32) -> windows_core::BOOL);
    unsafe { EngTransparentBlt(psodst, psosrc, pco.unwrap_or(core::mem::zeroed()) as _, pxlo.unwrap_or(core::mem::zeroed()) as _, prcldst, prclsrc, transcolor, bcalledfrombitblt) }
}
#[inline]
pub unsafe fn EngUnicodeToMultiByteN<P3>(multibytestring: &mut [u8], bytesinmultibytestring: Option<*mut u32>, unicodestring: P3, bytesinunicodestring: u32)
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EngUnicodeToMultiByteN(multibytestring : windows_core::PSTR, maxbytesinmultibytestring : u32, bytesinmultibytestring : *mut u32, unicodestring : windows_core::PCWSTR, bytesinunicodestring : u32));
    unsafe { EngUnicodeToMultiByteN(core::mem::transmute(multibytestring.as_ptr()), multibytestring.len().try_into().unwrap(), bytesinmultibytestring.unwrap_or(core::mem::zeroed()) as _, unicodestring.param().abi(), bytesinunicodestring) }
}
#[inline]
pub unsafe fn EngUnlockSurface(pso: *mut SURFOBJ) {
    windows_link::link!("gdi32.dll" "system" fn EngUnlockSurface(pso : *mut SURFOBJ));
    unsafe { EngUnlockSurface(pso as _) }
}
#[inline]
pub unsafe fn EngWideCharToMultiByte<P1>(codepage: u32, widecharstring: P1, bytesinwidecharstring: i32, multibytestring: Option<&mut [u8]>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_link::link!("gdi32.dll" "system" fn EngWideCharToMultiByte(codepage : u32, widecharstring : windows_core::PCWSTR, bytesinwidecharstring : i32, multibytestring : windows_core::PSTR, bytesinmultibytestring : i32) -> i32);
    unsafe { EngWideCharToMultiByte(codepage, widecharstring.param().abi(), bytesinwidecharstring, core::mem::transmute(multibytestring.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), multibytestring.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_cGetAllGlyphHandles(pfo : *mut FONTOBJ, phg : *mut u32) -> u32);
    unsafe { FONTOBJ_cGetAllGlyphHandles(pfo as _, phg as _) }
}
#[inline]
pub unsafe fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut core::ffi::c_void) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_cGetGlyphs(pfo : *mut FONTOBJ, imode : u32, cglyph : u32, phg : *mut u32, ppvglyph : *mut *mut core::ffi::c_void) -> u32);
    unsafe { FONTOBJ_cGetGlyphs(pfo as _, imode, cglyph, phg as _, ppvglyph as _) }
}
#[inline]
pub unsafe fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_pQueryGlyphAttrs(pfo : *mut FONTOBJ, imode : u32) -> *mut FD_GLYPHATTR);
    unsafe { FONTOBJ_pQueryGlyphAttrs(pfo as _, imode) }
}
#[inline]
pub unsafe fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_pfdg(pfo : *mut FONTOBJ) -> *mut FD_GLYPHSET);
    unsafe { FONTOBJ_pfdg(pfo as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_pifi(pfo : *const FONTOBJ) -> *mut IFIMETRICS);
    unsafe { FONTOBJ_pifi(pfo) }
}
#[inline]
pub unsafe fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut core::ffi::c_void {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_pvTrueTypeFontFile(pfo : *mut FONTOBJ, pcjfile : *mut u32) -> *mut core::ffi::c_void);
    unsafe { FONTOBJ_pvTrueTypeFontFile(pfo as _, pcjfile as _) }
}
#[inline]
pub unsafe fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut XFORMOBJ {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_pxoGetXform(pfo : *const FONTOBJ) -> *mut XFORMOBJ);
    unsafe { FONTOBJ_pxoGetXform(pfo) }
}
#[inline]
pub unsafe fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO) {
    windows_link::link!("gdi32.dll" "system" fn FONTOBJ_vGetInfo(pfo : *mut FONTOBJ, cjsize : u32, pfi : *mut FONTINFO));
    unsafe { FONTOBJ_vGetInfo(pfo as _, cjsize, pfi as _) }
}
#[inline]
pub unsafe fn GetAutoRotationState(pstate: *mut AR_STATE) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GetAutoRotationState(pstate : *mut AR_STATE) -> windows_core::BOOL);
    unsafe { GetAutoRotationState(pstate as _) }
}
#[inline]
pub unsafe fn GetCapabilitiesStringLength(hmonitor: super::super::Foundation::HANDLE, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetCapabilitiesStringLength(hmonitor : super::super::Foundation:: HANDLE, pdwcapabilitiesstringlengthincharacters : *mut u32) -> i32);
    unsafe { GetCapabilitiesStringLength(hmonitor, pdwcapabilitiesstringlengthincharacters as _) }
}
#[inline]
pub unsafe fn GetDisplayAutoRotationPreferences(porientation: *mut ORIENTATION_PREFERENCE) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn GetDisplayAutoRotationPreferences(porientation : *mut ORIENTATION_PREFERENCE) -> windows_core::BOOL);
    unsafe { GetDisplayAutoRotationPreferences(porientation as _) }
}
#[inline]
pub unsafe fn GetDisplayConfigBufferSizes(flags: QUERY_DISPLAY_CONFIG_FLAGS, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("user32.dll" "system" fn GetDisplayConfigBufferSizes(flags : QUERY_DISPLAY_CONFIG_FLAGS, numpatharrayelements : *mut u32, nummodeinfoarrayelements : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { GetDisplayConfigBufferSizes(flags, numpatharrayelements as _, nummodeinfoarrayelements as _) }
}
#[inline]
pub unsafe fn GetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorBrightness(hmonitor : super::super::Foundation:: HANDLE, pdwminimumbrightness : *mut u32, pdwcurrentbrightness : *mut u32, pdwmaximumbrightness : *mut u32) -> i32);
    unsafe { GetMonitorBrightness(hmonitor, pdwminimumbrightness as _, pdwcurrentbrightness as _, pdwmaximumbrightness as _) }
}
#[inline]
pub unsafe fn GetMonitorCapabilities(hmonitor: super::super::Foundation::HANDLE, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorCapabilities(hmonitor : super::super::Foundation:: HANDLE, pdwmonitorcapabilities : *mut u32, pdwsupportedcolortemperatures : *mut u32) -> i32);
    unsafe { GetMonitorCapabilities(hmonitor, pdwmonitorcapabilities as _, pdwsupportedcolortemperatures as _) }
}
#[inline]
pub unsafe fn GetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorColorTemperature(hmonitor : super::super::Foundation:: HANDLE, pctcurrentcolortemperature : *mut MC_COLOR_TEMPERATURE) -> i32);
    unsafe { GetMonitorColorTemperature(hmonitor, pctcurrentcolortemperature as _) }
}
#[inline]
pub unsafe fn GetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorContrast(hmonitor : super::super::Foundation:: HANDLE, pdwminimumcontrast : *mut u32, pdwcurrentcontrast : *mut u32, pdwmaximumcontrast : *mut u32) -> i32);
    unsafe { GetMonitorContrast(hmonitor, pdwminimumcontrast as _, pdwcurrentcontrast as _, pdwmaximumcontrast as _) }
}
#[inline]
pub unsafe fn GetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorDisplayAreaPosition(hmonitor : super::super::Foundation:: HANDLE, ptpositiontype : MC_POSITION_TYPE, pdwminimumposition : *mut u32, pdwcurrentposition : *mut u32, pdwmaximumposition : *mut u32) -> i32);
    unsafe { GetMonitorDisplayAreaPosition(hmonitor, ptpositiontype, pdwminimumposition as _, pdwcurrentposition as _, pdwmaximumposition as _) }
}
#[inline]
pub unsafe fn GetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorDisplayAreaSize(hmonitor : super::super::Foundation:: HANDLE, stsizetype : MC_SIZE_TYPE, pdwminimumwidthorheight : *mut u32, pdwcurrentwidthorheight : *mut u32, pdwmaximumwidthorheight : *mut u32) -> i32);
    unsafe { GetMonitorDisplayAreaSize(hmonitor, stsizetype, pdwminimumwidthorheight as _, pdwcurrentwidthorheight as _, pdwmaximumwidthorheight as _) }
}
#[inline]
pub unsafe fn GetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorRedGreenOrBlueDrive(hmonitor : super::super::Foundation:: HANDLE, dtdrivetype : MC_DRIVE_TYPE, pdwminimumdrive : *mut u32, pdwcurrentdrive : *mut u32, pdwmaximumdrive : *mut u32) -> i32);
    unsafe { GetMonitorRedGreenOrBlueDrive(hmonitor, dtdrivetype, pdwminimumdrive as _, pdwcurrentdrive as _, pdwmaximumdrive as _) }
}
#[inline]
pub unsafe fn GetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorRedGreenOrBlueGain(hmonitor : super::super::Foundation:: HANDLE, gtgaintype : MC_GAIN_TYPE, pdwminimumgain : *mut u32, pdwcurrentgain : *mut u32, pdwmaximumgain : *mut u32) -> i32);
    unsafe { GetMonitorRedGreenOrBlueGain(hmonitor, gtgaintype, pdwminimumgain as _, pdwcurrentgain as _, pdwmaximumgain as _) }
}
#[inline]
pub unsafe fn GetMonitorTechnologyType(hmonitor: super::super::Foundation::HANDLE, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetMonitorTechnologyType(hmonitor : super::super::Foundation:: HANDLE, pdtydisplaytechnologytype : *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32);
    unsafe { GetMonitorTechnologyType(hmonitor, pdtydisplaytechnologytype as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, pdwnumberofphysicalmonitors: *mut u32) -> windows_core::Result<()> {
    windows_link::link!("dxva2.dll" "system" fn GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor : super::super::Graphics::Gdi:: HMONITOR, pdwnumberofphysicalmonitors : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, pdwnumberofphysicalmonitors as _).ok() }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9<P0>(pdirect3ddevice9: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::super::Graphics::Direct3D9::IDirect3DDevice9>,
{
    windows_link::link!("dxva2.dll" "system" fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9 : * mut core::ffi::c_void, pdwnumberofphysicalmonitors : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, pphysicalmonitorarray: &mut [PHYSICAL_MONITOR]) -> windows_core::Result<()> {
    windows_link::link!("dxva2.dll" "system" fn GetPhysicalMonitorsFromHMONITOR(hmonitor : super::super::Graphics::Gdi:: HMONITOR, dwphysicalmonitorarraysize : u32, pphysicalmonitorarray : *mut PHYSICAL_MONITOR) -> windows_core::BOOL);
    unsafe { GetPhysicalMonitorsFromHMONITOR(hmonitor, pphysicalmonitorarray.len().try_into().unwrap(), core::mem::transmute(pphysicalmonitorarray.as_ptr())).ok() }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn GetPhysicalMonitorsFromIDirect3DDevice9<P0>(pdirect3ddevice9: P0, pphysicalmonitorarray: &mut [PHYSICAL_MONITOR]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Graphics::Direct3D9::IDirect3DDevice9>,
{
    windows_link::link!("dxva2.dll" "system" fn GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9 : * mut core::ffi::c_void, dwphysicalmonitorarraysize : u32, pphysicalmonitorarray : *mut PHYSICAL_MONITOR) -> windows_core::HRESULT);
    unsafe { GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9.param().abi(), pphysicalmonitorarray.len().try_into().unwrap(), core::mem::transmute(pphysicalmonitorarray.as_ptr())).ok() }
}
#[inline]
pub unsafe fn GetTimingReport(hmonitor: super::super::Foundation::HANDLE, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetTimingReport(hmonitor : super::super::Foundation:: HANDLE, pmtrmonitortimingreport : *mut MC_TIMING_REPORT) -> i32);
    unsafe { GetTimingReport(hmonitor, pmtrmonitortimingreport as _) }
}
#[inline]
pub unsafe fn GetVCPFeatureAndVCPFeatureReply(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, pvct: Option<*mut MC_VCP_CODE_TYPE>, pdwcurrentvalue: *mut u32, pdwmaximumvalue: Option<*mut u32>) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn GetVCPFeatureAndVCPFeatureReply(hmonitor : super::super::Foundation:: HANDLE, bvcpcode : u8, pvct : *mut MC_VCP_CODE_TYPE, pdwcurrentvalue : *mut u32, pdwmaximumvalue : *mut u32) -> i32);
    unsafe { GetVCPFeatureAndVCPFeatureReply(hmonitor, bvcpcode, pvct.unwrap_or(core::mem::zeroed()) as _, pdwcurrentvalue as _, pdwmaximumvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HT_Get8BPPFormatPalette(ppaletteentry: Option<*mut super::super::Graphics::Gdi::PALETTEENTRY>, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn HT_Get8BPPFormatPalette(ppaletteentry : *mut super::super::Graphics::Gdi:: PALETTEENTRY, redgamma : u16, greengamma : u16, bluegamma : u16) -> i32);
    unsafe { HT_Get8BPPFormatPalette(ppaletteentry.unwrap_or(core::mem::zeroed()) as _, redgamma, greengamma, bluegamma) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HT_Get8BPPMaskPalette(ppaletteentry: Option<*mut super::super::Graphics::Gdi::PALETTEENTRY>, use8bppmaskpal: bool, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32 {
    windows_link::link!("gdi32.dll" "system" fn HT_Get8BPPMaskPalette(ppaletteentry : *mut super::super::Graphics::Gdi:: PALETTEENTRY, use8bppmaskpal : windows_core::BOOL, cmymask : u8, redgamma : u16, greengamma : u16, bluegamma : u16) -> i32);
    unsafe { HT_Get8BPPMaskPalette(ppaletteentry.unwrap_or(core::mem::zeroed()) as _, use8bppmaskpal.into(), cmymask, redgamma, greengamma, bluegamma) }
}
#[inline]
pub unsafe fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PATHOBJ_bEnum(ppo : *mut PATHOBJ, ppd : *mut PATHDATA) -> windows_core::BOOL);
    unsafe { PATHOBJ_bEnum(ppo as _, ppd as _) }
}
#[inline]
pub unsafe fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn PATHOBJ_bEnumClipLines(ppo : *mut PATHOBJ, cb : u32, pcl : *mut CLIPLINE) -> windows_core::BOOL);
    unsafe { PATHOBJ_bEnumClipLines(ppo as _, cb, pcl as _) }
}
#[inline]
pub unsafe fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ) {
    windows_link::link!("gdi32.dll" "system" fn PATHOBJ_vEnumStart(ppo : *mut PATHOBJ));
    unsafe { PATHOBJ_vEnumStart(ppo as _) }
}
#[inline]
pub unsafe fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS) {
    windows_link::link!("gdi32.dll" "system" fn PATHOBJ_vEnumStartClipLines(ppo : *mut PATHOBJ, pco : *mut CLIPOBJ, pso : *mut SURFOBJ, pla : *mut LINEATTRS));
    unsafe { PATHOBJ_vEnumStartClipLines(ppo as _, pco as _, pso as _, pla as _) }
}
#[inline]
pub unsafe fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut RECTFX) {
    windows_link::link!("gdi32.dll" "system" fn PATHOBJ_vGetBounds(ppo : *mut PATHOBJ, prectfx : *mut RECTFX));
    unsafe { PATHOBJ_vGetBounds(ppo as _, prectfx as _) }
}
#[inline]
pub unsafe fn QueryDisplayConfig(flags: QUERY_DISPLAY_CONFIG_FLAGS, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>) -> super::super::Foundation::WIN32_ERROR {
    windows_link::link!("user32.dll" "system" fn QueryDisplayConfig(flags : QUERY_DISPLAY_CONFIG_FLAGS, numpatharrayelements : *mut u32, patharray : *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements : *mut u32, modeinfoarray : *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid : *mut DISPLAYCONFIG_TOPOLOGY_ID) -> super::super::Foundation:: WIN32_ERROR);
    unsafe { QueryDisplayConfig(flags, numpatharrayelements as _, patharray as _, nummodeinfoarrayelements as _, modeinfoarray as _, currenttopologyid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RestoreMonitorFactoryColorDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn RestoreMonitorFactoryColorDefaults(hmonitor : super::super::Foundation:: HANDLE) -> i32);
    unsafe { RestoreMonitorFactoryColorDefaults(hmonitor) }
}
#[inline]
pub unsafe fn RestoreMonitorFactoryDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn RestoreMonitorFactoryDefaults(hmonitor : super::super::Foundation:: HANDLE) -> i32);
    unsafe { RestoreMonitorFactoryDefaults(hmonitor) }
}
#[inline]
pub unsafe fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn STROBJ_bEnum(pstro : *mut STROBJ, pc : *mut u32, ppgpos : *mut *mut GLYPHPOS) -> windows_core::BOOL);
    unsafe { STROBJ_bEnum(pstro as _, pc as _, ppgpos as _) }
}
#[inline]
pub unsafe fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn STROBJ_bEnumPositionsOnly(pstro : *mut STROBJ, pc : *mut u32, ppgpos : *mut *mut GLYPHPOS) -> windows_core::BOOL);
    unsafe { STROBJ_bEnumPositionsOnly(pstro as _, pc as _, ppgpos as _) }
}
#[inline]
pub unsafe fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut POINTQF) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn STROBJ_bGetAdvanceWidths(pso : *mut STROBJ, ifirst : u32, c : u32, pptqd : *mut POINTQF) -> windows_core::BOOL);
    unsafe { STROBJ_bGetAdvanceWidths(pso as _, ifirst, c, pptqd as _) }
}
#[inline]
pub unsafe fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn STROBJ_dwGetCodePage(pstro : *mut STROBJ) -> u32);
    unsafe { STROBJ_dwGetCodePage(pstro as _) }
}
#[inline]
pub unsafe fn STROBJ_vEnumStart(pstro: *mut STROBJ) {
    windows_link::link!("gdi32.dll" "system" fn STROBJ_vEnumStart(pstro : *mut STROBJ));
    unsafe { STROBJ_vEnumStart(pstro as _) }
}
#[inline]
pub unsafe fn SaveCurrentMonitorSettings(hmonitor: super::super::Foundation::HANDLE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SaveCurrentMonitorSettings(hmonitor : super::super::Foundation:: HANDLE) -> i32);
    unsafe { SaveCurrentMonitorSettings(hmonitor) }
}
#[inline]
pub unsafe fn SaveCurrentSettings(hmonitor: super::super::Foundation::HANDLE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SaveCurrentSettings(hmonitor : super::super::Foundation:: HANDLE) -> i32);
    unsafe { SaveCurrentSettings(hmonitor) }
}
#[inline]
pub unsafe fn SetDisplayAutoRotationPreferences(orientation: ORIENTATION_PREFERENCE) -> windows_core::BOOL {
    windows_link::link!("user32.dll" "system" fn SetDisplayAutoRotationPreferences(orientation : ORIENTATION_PREFERENCE) -> windows_core::BOOL);
    unsafe { SetDisplayAutoRotationPreferences(orientation) }
}
#[inline]
pub unsafe fn SetDisplayConfig(patharray: Option<&[DISPLAYCONFIG_PATH_INFO]>, modeinfoarray: Option<&[DISPLAYCONFIG_MODE_INFO]>, flags: SET_DISPLAY_CONFIG_FLAGS) -> i32 {
    windows_link::link!("user32.dll" "system" fn SetDisplayConfig(numpatharrayelements : u32, patharray : *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements : u32, modeinfoarray : *const DISPLAYCONFIG_MODE_INFO, flags : SET_DISPLAY_CONFIG_FLAGS) -> i32);
    unsafe { SetDisplayConfig(patharray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(patharray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), modeinfoarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(modeinfoarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), flags) }
}
#[inline]
pub unsafe fn SetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, dwnewbrightness: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorBrightness(hmonitor : super::super::Foundation:: HANDLE, dwnewbrightness : u32) -> i32);
    unsafe { SetMonitorBrightness(hmonitor, dwnewbrightness) }
}
#[inline]
pub unsafe fn SetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorColorTemperature(hmonitor : super::super::Foundation:: HANDLE, ctcurrentcolortemperature : MC_COLOR_TEMPERATURE) -> i32);
    unsafe { SetMonitorColorTemperature(hmonitor, ctcurrentcolortemperature) }
}
#[inline]
pub unsafe fn SetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, dwnewcontrast: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorContrast(hmonitor : super::super::Foundation:: HANDLE, dwnewcontrast : u32) -> i32);
    unsafe { SetMonitorContrast(hmonitor, dwnewcontrast) }
}
#[inline]
pub unsafe fn SetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorDisplayAreaPosition(hmonitor : super::super::Foundation:: HANDLE, ptpositiontype : MC_POSITION_TYPE, dwnewposition : u32) -> i32);
    unsafe { SetMonitorDisplayAreaPosition(hmonitor, ptpositiontype, dwnewposition) }
}
#[inline]
pub unsafe fn SetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorDisplayAreaSize(hmonitor : super::super::Foundation:: HANDLE, stsizetype : MC_SIZE_TYPE, dwnewdisplayareawidthorheight : u32) -> i32);
    unsafe { SetMonitorDisplayAreaSize(hmonitor, stsizetype, dwnewdisplayareawidthorheight) }
}
#[inline]
pub unsafe fn SetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorRedGreenOrBlueDrive(hmonitor : super::super::Foundation:: HANDLE, dtdrivetype : MC_DRIVE_TYPE, dwnewdrive : u32) -> i32);
    unsafe { SetMonitorRedGreenOrBlueDrive(hmonitor, dtdrivetype, dwnewdrive) }
}
#[inline]
pub unsafe fn SetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetMonitorRedGreenOrBlueGain(hmonitor : super::super::Foundation:: HANDLE, gtgaintype : MC_GAIN_TYPE, dwnewgain : u32) -> i32);
    unsafe { SetMonitorRedGreenOrBlueGain(hmonitor, gtgaintype, dwnewgain) }
}
#[inline]
pub unsafe fn SetVCPFeature(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, dwnewvalue: u32) -> i32 {
    windows_link::link!("dxva2.dll" "system" fn SetVCPFeature(hmonitor : super::super::Foundation:: HANDLE, bvcpcode : u8, dwnewvalue : u32) -> i32);
    unsafe { SetVCPFeature(hmonitor, bvcpcode, dwnewvalue) }
}
#[inline]
pub unsafe fn XFORMOBJ_bApplyXform(pxo: *mut XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut core::ffi::c_void, pvout: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_link::link!("gdi32.dll" "system" fn XFORMOBJ_bApplyXform(pxo : *mut XFORMOBJ, imode : u32, cpoints : u32, pvin : *mut core::ffi::c_void, pvout : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { XFORMOBJ_bApplyXform(pxo as _, imode, cpoints, pvin as _, pvout as _) }
}
#[inline]
pub unsafe fn XFORMOBJ_iGetXform(pxo: *const XFORMOBJ, pxform: Option<*mut XFORML>) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn XFORMOBJ_iGetXform(pxo : *const XFORMOBJ, pxform : *mut XFORML) -> u32);
    unsafe { XFORMOBJ_iGetXform(pxo, pxform.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn XLATEOBJ_cGetPalette(pxlo : *mut XLATEOBJ, ipal : u32, cpal : u32, ppal : *mut u32) -> u32);
    unsafe { XLATEOBJ_cGetPalette(pxlo as _, ipal, cpal, ppal as _) }
}
#[inline]
pub unsafe fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE {
    windows_link::link!("gdi32.dll" "system" fn XLATEOBJ_hGetColorTransform(pxlo : *mut XLATEOBJ) -> super::super::Foundation:: HANDLE);
    unsafe { XLATEOBJ_hGetColorTransform(pxlo as _) }
}
#[inline]
pub unsafe fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32 {
    windows_link::link!("gdi32.dll" "system" fn XLATEOBJ_iXlate(pxlo : *mut XLATEOBJ, icolor : u32) -> u32);
    unsafe { XLATEOBJ_iXlate(pxlo as _, icolor) }
}
#[inline]
pub unsafe fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32 {
    windows_link::link!("gdi32.dll" "system" fn XLATEOBJ_piVector(pxlo : *mut XLATEOBJ) -> *mut u32);
    unsafe { XLATEOBJ_piVector(pxlo as _) }
}
pub const AR_DISABLED: AR_STATE = AR_STATE(1i32);
pub const AR_DOCKED: AR_STATE = AR_STATE(64i32);
pub const AR_ENABLED: AR_STATE = AR_STATE(0i32);
pub const AR_LAPTOP: AR_STATE = AR_STATE(128i32);
pub const AR_MULTIMON: AR_STATE = AR_STATE(8i32);
pub const AR_NOSENSOR: AR_STATE = AR_STATE(16i32);
pub const AR_NOT_SUPPORTED: AR_STATE = AR_STATE(32i32);
pub const AR_REMOTESESSION: AR_STATE = AR_STATE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AR_STATE(pub i32);
impl AR_STATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AR_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AR_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AR_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AR_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AR_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AR_SUPPRESSED: AR_STATE = AR_STATE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Adapter {
    pub AdapterName: [u16; 128],
    pub numSources: i32,
    pub sources: [Sources; 1],
}
impl Default for Adapter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Adapters {
    pub numAdapters: i32,
    pub adapter: [Adapter; 1],
}
impl Default for Adapters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BACKLIGHT_OPTIMIZATION_LEVEL(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BACKLIGHT_REDUCTION_GAMMA_RAMP {
    pub R: [u16; 256],
    pub G: [u16; 256],
    pub B: [u16; 256],
}
impl Default for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BANK_POSITION {
    pub ReadBankPosition: u32,
    pub WriteBankPosition: u32,
}
pub const BITMAP_ARRAY_BYTE: u32 = 3u32;
pub const BITMAP_BITS_BYTE_ALIGN: u32 = 8u32;
pub const BITMAP_BITS_PIXEL: u32 = 1u32;
pub const BITMAP_BITS_WORD_ALIGN: u32 = 16u32;
pub const BITMAP_PLANES: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BLENDOBJ {
    pub BlendFunction: super::super::Graphics::Gdi::BLENDFUNCTION,
}
pub const BMF_16BPP: i32 = 4i32;
pub const BMF_1BPP: i32 = 1i32;
pub const BMF_24BPP: i32 = 5i32;
pub const BMF_32BPP: i32 = 6i32;
pub const BMF_4BPP: i32 = 2i32;
pub const BMF_4RLE: i32 = 7i32;
pub const BMF_8BPP: i32 = 3i32;
pub const BMF_8RLE: i32 = 8i32;
pub const BMF_ACC_NOTIFY: u32 = 32768u32;
pub const BMF_DONTCACHE: u32 = 4u32;
pub const BMF_JPEG: i32 = 9i32;
pub const BMF_KMSECTION: u32 = 16u32;
pub const BMF_NOTSYSMEM: u32 = 32u32;
pub const BMF_NOZEROINIT: u32 = 2u32;
pub const BMF_PNG: i32 = 10i32;
pub const BMF_RESERVED: u32 = 15872u32;
pub const BMF_RMT_ENTER: u32 = 16384u32;
pub const BMF_TEMP_ALPHA: u32 = 256u32;
pub const BMF_TOPDOWN: u32 = 1u32;
pub const BMF_UMPDMEM: u32 = 128u32;
pub const BMF_USERMEM: u32 = 8u32;
pub const BMF_WINDOW_BLT: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BRIGHTNESS_INTERFACE_VERSION(pub i32);
pub const BRIGHTNESS_INTERFACE_VERSION_1: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(1i32);
pub const BRIGHTNESS_INTERFACE_VERSION_2: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(2i32);
pub const BRIGHTNESS_INTERFACE_VERSION_3: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BRIGHTNESS_LEVEL {
    pub Count: u8,
    pub Level: [u8; 103],
}
impl Default for BRIGHTNESS_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BRIGHTNESS_MAX_LEVEL_COUNT: u32 = 103u32;
pub const BRIGHTNESS_MAX_NIT_RANGE_COUNT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BRIGHTNESS_NIT_RANGE {
    pub MinLevelInMillinit: u32,
    pub MaxLevelInMillinit: u32,
    pub StepSizeInMillinit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BRIGHTNESS_NIT_RANGES {
    pub NormalRangeCount: u32,
    pub RangeCount: u32,
    pub PreferredMaximumBrightness: u32,
    pub SupportedRanges: [BRIGHTNESS_NIT_RANGE; 16],
}
impl Default for BRIGHTNESS_NIT_RANGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BRUSHOBJ {
    pub iSolidColor: u32,
    pub pvRbrush: *mut core::ffi::c_void,
    pub flColorType: u32,
}
impl Default for BRUSHOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BR_CMYKCOLOR: u32 = 4u32;
pub const BR_DEVICE_ICM: u32 = 1u32;
pub const BR_HOST_ICM: u32 = 2u32;
pub const BR_ORIGCOLOR: u32 = 8u32;
pub const BacklightOptimizationDesktop: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(1i32);
pub const BacklightOptimizationDimmed: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(3i32);
pub const BacklightOptimizationDisable: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(0i32);
pub const BacklightOptimizationDynamic: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(2i32);
pub const BacklightOptimizationEDR: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BlackScreenDiagnosticsCalloutParam(pub i32);
pub const BlackScreenDiagnosticsData: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(1i32);
pub const BlackScreenDisplayRecovery: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(2i32);
pub const CDBEX_CROSSADAPTER: u32 = 8u32;
pub const CDBEX_DXINTEROP: u32 = 2u32;
pub const CDBEX_NTSHAREDSURFACEHANDLE: u32 = 4u32;
pub const CDBEX_REDIRECTION: u32 = 1u32;
pub const CDBEX_REUSE: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CDDDXGK_REDIRBITMAPPRESENTINFO {
    pub NumDirtyRects: u32,
    pub DirtyRect: *mut super::super::Foundation::RECT,
    pub NumContexts: u32,
    pub hContext: [super::super::Foundation::HANDLE; 65],
    pub bDoNotSynchronizeWithDxContent: bool,
}
impl Default for CDDDXGK_REDIRBITMAPPRESENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CD_ANY: i32 = 4i32;
pub const CD_LEFTDOWN: i32 = 1i32;
pub const CD_LEFTUP: i32 = 3i32;
pub const CD_LEFTWARDS: i32 = 1i32;
pub const CD_RIGHTDOWN: i32 = 0i32;
pub const CD_RIGHTUP: i32 = 2i32;
pub const CD_UPWARDS: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy)]
pub struct CHAR_IMAGE_INFO {
    pub CharInfo: super::super::System::Console::CHAR_INFO,
    pub FontImageInfo: FONT_IMAGE_INFO,
}
#[cfg(feature = "Win32_System_Console")]
impl Default for CHAR_IMAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CHAR_TYPE_LEADING: u32 = 2u32;
pub const CHAR_TYPE_SBCS: u32 = 0u32;
pub const CHAR_TYPE_TRAILING: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHROMATICITY_COORDINATE {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CIECHROMA {
    pub x: i32,
    pub y: i32,
    pub Y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLIPLINE {
    pub ptfxA: POINTFIX,
    pub ptfxB: POINTFIX,
    pub lStyleState: i32,
    pub c: u32,
    pub arun: [RUN; 1],
}
impl Default for CLIPLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLIPOBJ {
    pub iUniq: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iDComplexity: u8,
    pub iFComplexity: u8,
    pub iMode: u8,
    pub fjOptions: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORINFO {
    pub Red: CIECHROMA,
    pub Green: CIECHROMA,
    pub Blue: CIECHROMA,
    pub Cyan: CIECHROMA,
    pub Magenta: CIECHROMA,
    pub Yellow: CIECHROMA,
    pub AlignmentWhite: CIECHROMA,
    pub RedGamma: i32,
    pub GreenGamma: i32,
    pub BlueGamma: i32,
    pub MagentaInCyanDye: i32,
    pub YellowInCyanDye: i32,
    pub CyanInMagentaDye: i32,
    pub YellowInMagentaDye: i32,
    pub CyanInYellowDye: i32,
    pub MagentaInYellowDye: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLORSPACE_TRANSFORM {
    pub Type: COLORSPACE_TRANSFORM_TYPE,
    pub Data: COLORSPACE_TRANSFORM_0,
}
impl Default for COLORSPACE_TRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COLORSPACE_TRANSFORM_0 {
    pub Rgb256x3x16: GAMMA_RAMP_RGB256x3x16,
    pub Dxgi1: GAMMA_RAMP_DXGI_1,
    pub T3x4: COLORSPACE_TRANSFORM_3x4,
    pub MatrixV2: COLORSPACE_TRANSFORM_MATRIX_V2,
}
impl Default for COLORSPACE_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLORSPACE_TRANSFORM_1DLUT_CAP {
    pub NumberOfLUTEntries: u32,
    pub DataCap: COLORSPACE_TRANSFORM_DATA_CAP,
}
impl Default for COLORSPACE_TRANSFORM_1DLUT_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COLORSPACE_TRANSFORM_3x4 {
    pub ColorMatrix3x4: [f32; 12],
    pub ScalarMultiplier: f32,
    pub LookupTable1D: [GAMMA_RAMP_RGB; 4096],
}
impl Default for COLORSPACE_TRANSFORM_3x4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLORSPACE_TRANSFORM_DATA_CAP {
    pub DataType: COLORSPACE_TRANSFORM_DATA_TYPE,
    pub Anonymous: COLORSPACE_TRANSFORM_DATA_CAP_0,
    pub NumericRangeMin: f32,
    pub NumericRangeMax: f32,
}
impl Default for COLORSPACE_TRANSFORM_DATA_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COLORSPACE_TRANSFORM_DATA_CAP_0 {
    pub Anonymous1: COLORSPACE_TRANSFORM_DATA_CAP_0_0,
    pub Anonymous2: COLORSPACE_TRANSFORM_DATA_CAP_0_1,
    pub Value: u32,
}
impl Default for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORSPACE_TRANSFORM_DATA_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FIXED_POINT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FLOAT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP {
    pub Anonymous: COLORSPACE_TRANSFORM_MATRIX_CAP_0,
    pub DataCap: COLORSPACE_TRANSFORM_DATA_CAP,
}
impl Default for COLORSPACE_TRANSFORM_MATRIX_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    pub Anonymous: COLORSPACE_TRANSFORM_MATRIX_CAP_0_0,
    pub Value: u32,
}
impl Default for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COLORSPACE_TRANSFORM_MATRIX_V2 {
    pub StageControlLookupTable1DDegamma: COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DDegamma: [GAMMA_RAMP_RGB; 4096],
    pub StageControlColorMatrix3x3: COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub ColorMatrix3x3: [f32; 9],
    pub StageControlLookupTable1DRegamma: COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DRegamma: [GAMMA_RAMP_RGB; 4096],
}
impl Default for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLORSPACE_TRANSFORM_SET_INPUT {
    pub OutputWireColorSpaceExpected: OUTPUT_WIRE_COLOR_SPACE_TYPE,
    pub OutputWireFormatExpected: OUTPUT_WIRE_FORMAT,
    pub ColorSpaceTransform: COLORSPACE_TRANSFORM,
}
impl Default for COLORSPACE_TRANSFORM_SET_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORSPACE_TRANSFORM_STAGE_CONTROL(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS {
    pub Version: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION,
    pub LookupTable1DDegammaCap: COLORSPACE_TRANSFORM_1DLUT_CAP,
    pub ColorMatrix3x3Cap: COLORSPACE_TRANSFORM_MATRIX_CAP,
    pub LookupTable1DRegammaCap: COLORSPACE_TRANSFORM_1DLUT_CAP,
}
impl Default for COLORSPACE_TRANSFORM_TARGET_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORSPACE_TRANSFORM_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_TYPE_DEFAULT: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(1i32);
pub const COLORSPACE_TRANSFORM_TYPE_DXGI_1: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(3i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_3x4: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(4i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_V2: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(5i32);
pub const COLORSPACE_TRANSFORM_TYPE_RGB256x3x16: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(2i32);
pub const COLORSPACE_TRANSFORM_TYPE_UNINITIALIZED: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_VERSION_1: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(1i32);
pub const COLORSPACE_TRANSFORM_VERSION_DEFAULT: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
pub const COLORSPACE_TRANSFORM_VERSION_NOT_SUPPORTED: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
pub const CT_RECTANGLES: i32 = 0i32;
pub const ColorSpaceTransformStageControl_Bypass: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(2i32);
pub const ColorSpaceTransformStageControl_Enable: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(1i32);
pub const ColorSpaceTransformStageControl_No_Change: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(0i32);
pub const DCR_DRIVER: u32 = 1u32;
pub const DCR_HALFTONE: u32 = 2u32;
pub const DCR_SOLID: u32 = 0u32;
pub const DCT_DEFAULT: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(0i32);
pub const DCT_FORCE_HIGH_PERFORMANCE: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(2i32);
pub const DCT_FORCE_LOW_POWER: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(1i32);
pub const DC_COMPLEX: u32 = 3u32;
pub const DC_RECT: u32 = 1u32;
pub const DC_TRIVIAL: u32 = 0u32;
pub const DDI_DRIVER_VERSION_NT4: u32 = 131072u32;
pub const DDI_DRIVER_VERSION_NT5: u32 = 196608u32;
pub const DDI_DRIVER_VERSION_NT5_01: u32 = 196864u32;
pub const DDI_DRIVER_VERSION_NT5_01_SP1: u32 = 196865u32;
pub const DDI_DRIVER_VERSION_SP3: u32 = 131075u32;
pub const DDI_ERROR: u32 = 4294967295u32;
pub const DD_FULLSCREEN_VIDEO_DEVICE_NAME: windows_core::PCWSTR = windows_core::w!("\\Device\\FSVideo");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVHTADJDATA {
    pub DeviceFlags: u32,
    pub DeviceXDPI: u32,
    pub DeviceYDPI: u32,
    pub pDefHTInfo: *mut DEVHTINFO,
    pub pAdjHTInfo: *mut DEVHTINFO,
}
impl Default for DEVHTADJDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVHTADJF_ADDITIVE_DEVICE: u32 = 2u32;
pub const DEVHTADJF_COLOR_DEVICE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVHTINFO {
    pub HTFlags: u32,
    pub HTPatternSize: u32,
    pub DevPelsDPI: u32,
    pub ColorInfo: COLORINFO,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVINFO {
    pub flGraphicsCaps: u32,
    pub lfDefaultFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfAnsiVarFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfAnsiFixFont: super::super::Graphics::Gdi::LOGFONTW,
    pub cFonts: u32,
    pub iDitherFormat: u32,
    pub cxDither: u16,
    pub cyDither: u16,
    pub hpalDefault: super::super::Graphics::Gdi::HPALETTE,
    pub flGraphicsCaps2: u32,
}
pub const DEVPKEY_Device_ActivityId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 4 };
pub const DEVPKEY_Device_AdapterLuid: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 3 };
pub const DEVPKEY_Device_TerminalLuid: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 2 };
pub const DEVPKEY_IndirectDisplay: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 1 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DHPDEV(pub *mut core::ffi::c_void);
impl DHPDEV {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for DHPDEV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DHSURF(pub *mut core::ffi::c_void);
impl DHSURF {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for DHSURF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_2DREGION {
    pub cx: u32,
    pub cy: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPLAYCONFIG_ADAPTER_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub adapterDevicePath: [u16; 128],
}
impl Default for DISPLAYCONFIG_ADAPTER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    pub PathSourceSize: super::super::Foundation::POINTL,
    pub DesktopImageRegion: super::super::Foundation::RECTL,
    pub DesktopImageClip: super::super::Foundation::RECTL,
}
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(4i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(9i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(12i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(11i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(7i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(6i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    pub r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
    pub size: u32,
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
}
pub const DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(10i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(13i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(8i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_DEVICE_INFO_TYPE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0,
    pub colorEncoding: super::super::Graphics::Gdi::DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0,
}
impl Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
impl Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_MODE_INFO {
    pub infoType: DISPLAYCONFIG_MODE_INFO_TYPE,
    pub id: u32,
    pub adapterId: super::super::Foundation::LUID,
    pub Anonymous: DISPLAYCONFIG_MODE_INFO_0,
}
impl Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_MODE_INFO_0 {
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
    pub sourceMode: DISPLAYCONFIG_SOURCE_MODE,
    pub desktopImageInfo: DISPLAYCONFIG_DESKTOP_IMAGE_INFO,
}
impl Default for DISPLAYCONFIG_MODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_MODE_INFO_TYPE(pub i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(3i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_TARGET: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(3i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(2i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(11i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(10i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_USB_TUNNEL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(18i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(4i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(8i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(0i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(5i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_VIRTUAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(17i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(16i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-2147483648i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(6i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(15i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-1i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(9i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(14i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(1i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(13i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(12i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_INFO {
    pub sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    pub flags: u32,
}
impl Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0,
    pub statusFlags: u32,
}
impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0_0,
}
impl Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO {
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub rotation: DISPLAYCONFIG_ROTATION,
    pub scaling: DISPLAYCONFIG_SCALING,
    pub refreshRate: DISPLAYCONFIG_RATIONAL,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
    pub targetAvailable: windows_core::BOOL,
    pub statusFlags: u32,
}
impl Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0_0,
}
impl Default for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_PIXELFORMAT(pub i32);
pub const DISPLAYCONFIG_PIXELFORMAT_16BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(2i32);
pub const DISPLAYCONFIG_PIXELFORMAT_24BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(3i32);
pub const DISPLAYCONFIG_PIXELFORMAT_32BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(4i32);
pub const DISPLAYCONFIG_PIXELFORMAT_8BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(1i32);
pub const DISPLAYCONFIG_PIXELFORMAT_NONGDI: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_ROTATION(pub i32);
pub const DISPLAYCONFIG_ROTATION_IDENTITY: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(1i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE180: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(3i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE270: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(4i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE90: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_SCALING(pub i32);
pub const DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(4i32);
pub const DISPLAYCONFIG_SCALING_CENTERED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(2i32);
pub const DISPLAYCONFIG_SCALING_CUSTOM: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(5i32);
pub const DISPLAYCONFIG_SCALING_IDENTITY: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(1i32);
pub const DISPLAYCONFIG_SCALING_PREFERRED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(128i32);
pub const DISPLAYCONFIG_SCALING_STRETCHED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_SCANLINE_ORDERING(pub i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(3i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(1i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub SDRWhiteLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0,
}
impl Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0,
    pub value: u32,
}
impl Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0,
    pub specializationType: windows_core::GUID,
    pub specializationSubType: windows_core::GUID,
    pub specializationApplicationName: [u16; 128],
}
impl Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
impl Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0,
}
impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0,
    pub value: u32,
}
impl Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub viewGdiDeviceName: [u16; 32],
}
impl Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_SOURCE_MODE {
    pub width: u32,
    pub height: u32,
    pub pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    pub position: super::super::Foundation::POINTL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0,
}
impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0,
    pub value: u32,
}
impl Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub baseOutputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}
#[repr(C)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    pub _bitfield: u32,
}
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
#[derive(Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub width: u32,
    pub height: u32,
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
}
impl Default for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DISPLAYCONFIG_TOPOLOGY_CLONE: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(2i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTEND: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(4i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_TOPOLOGY_ID(pub i32);
pub const DISPLAYCONFIG_TOPOLOGY_INTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(pub i32);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: u32,
}
pub const DISPLAYPOLICY_AC: u32 = 1u32;
pub const DISPLAYPOLICY_DC: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPLAY_BRIGHTNESS {
    pub ucDisplayPolicy: u8,
    pub ucACBrightness: u8,
    pub ucDCBrightness: u8,
}
pub const DM_DEFAULT: u32 = 1u32;
pub const DM_MONOCHROME: u32 = 2u32;
pub const DN_ACCELERATION_LEVEL: u32 = 1u32;
pub const DN_ASSOCIATE_WINDOW: u32 = 5u32;
pub const DN_COMPOSITION_CHANGED: u32 = 6u32;
pub const DN_DEVICE_ORIGIN: u32 = 2u32;
pub const DN_DRAWING_BEGIN: u32 = 4u32;
pub const DN_DRAWING_BEGIN_APIBITMAP: u32 = 7u32;
pub const DN_SLEEP_MODE: u32 = 3u32;
pub const DN_SURFOBJ_DESTRUCTION: u32 = 8u32;
pub const DRD_ERROR: u32 = 1u32;
pub const DRD_SUCCESS: u32 = 0u32;
pub const DRH_APIBITMAP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRH_APIBITMAPDATA {
    pub pso: *mut SURFOBJ,
    pub b: windows_core::BOOL,
}
impl Default for DRH_APIBITMAPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVEROBJ {
    pub pvObj: *mut core::ffi::c_void,
    pub pFreeProc: FREEOBJPROC,
    pub hdev: HDEV,
    pub dhpdev: DHPDEV,
}
impl Default for DRIVEROBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRVENABLEDATA {
    pub iDriverVersion: u32,
    pub c: u32,
    pub pdrvfn: *mut DRVFN,
}
impl Default for DRVENABLEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRVFN {
    pub iFunc: u32,
    pub pfn: PFN,
}
pub const DRVQUERY_USERMODE: u32 = 1u32;
pub const DSI_CHECKSUM_ERROR_CORRECTED: u32 = 256u32;
pub const DSI_CHECKSUM_ERROR_NOT_CORRECTED: u32 = 512u32;
pub const DSI_CONTENTION_DETECTED: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSI_CONTROL_TRANSMISSION_MODE(pub i32);
pub const DSI_DSI_DATA_TYPE_NOT_RECOGNIZED: u32 = 2048u32;
pub const DSI_DSI_PROTOCOL_VIOLATION: u32 = 32768u32;
pub const DSI_DSI_VC_ID_INVALID: u32 = 4096u32;
pub const DSI_EOT_SYNC_ERROR: u32 = 4u32;
pub const DSI_ESCAPE_MODE_ENTRY_COMMAND_ERROR: u32 = 8u32;
pub const DSI_FALSE_CONTROL_ERROR: u32 = 64u32;
pub const DSI_INVALID_PACKET_INDEX: u32 = 255u32;
pub const DSI_INVALID_TRANSMISSION_LENGTH: u32 = 8192u32;
pub const DSI_LONG_PACKET_PAYLOAD_CHECKSUM_ERROR: u32 = 1024u32;
pub const DSI_LOW_POWER_TRANSMIT_SYNC_ERROR: u32 = 16u32;
pub const DSI_PACKET_EMBEDDED_PAYLOAD_SIZE: u32 = 8u32;
pub const DSI_PERIPHERAL_TIMEOUT_ERROR: u32 = 32u32;
pub const DSI_SOT_ERROR: u32 = 1u32;
pub const DSI_SOT_SYNC_ERROR: u32 = 2u32;
pub const DSS_FLUSH_EVENT: u32 = 2u32;
pub const DSS_RESERVED: u32 = 4u32;
pub const DSS_RESERVED1: u32 = 8u32;
pub const DSS_RESERVED2: u32 = 16u32;
pub const DSS_TIMER_EVENT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGK_WIN32K_PARAM_DATA {
    pub PathsArray: *mut core::ffi::c_void,
    pub ModesArray: *mut core::ffi::c_void,
    pub NumPathArrayElements: u32,
    pub NumModeArrayElements: u32,
    pub SDCFlags: u32,
}
impl Default for DXGK_WIN32K_PARAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGK_WIN32K_PARAM_FLAG_DISABLEVIEW: u32 = 4u32;
pub const DXGK_WIN32K_PARAM_FLAG_MODESWITCH: u32 = 2u32;
pub const DXGK_WIN32K_PARAM_FLAG_UPDATEREGISTRY: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct DisplayMode {
    pub DeviceName: [u16; 32],
    pub devMode: super::super::Graphics::Gdi::DEVMODEW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DisplayMode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct DisplayModes {
    pub numDisplayModes: i32,
    pub displayMode: [DisplayMode; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DisplayModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ECS_REDRAW: u32 = 2u32;
pub const ECS_TEARDOWN: u32 = 1u32;
pub const ED_ABORTDOC: u32 = 1u32;
pub const EHN_ERROR: u32 = 1u32;
pub const EHN_RESTORED: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMFINFO {
    pub nSize: u32,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub pvEMF: *mut u8,
    pub pvCurrentRecord: *mut u8,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for EMFINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENDCAP_BUTT: i32 = 2i32;
pub const ENDCAP_ROUND: i32 = 0i32;
pub const ENDCAP_SQUARE: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENGSAFESEMAPHORE {
    pub hsem: HSEMAPHORE,
    pub lCount: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENG_DEVICE_ATTRIBUTE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENG_EVENT {
    pub pKEvent: *mut core::ffi::c_void,
    pub fFlags: u32,
}
impl Default for ENG_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENG_FNT_CACHE_READ_FAULT: u32 = 1u32;
pub const ENG_FNT_CACHE_WRITE_FAULT: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENG_SYSTEM_ATTRIBUTE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENG_TIME_FIELDS {
    pub usYear: u16,
    pub usMonth: u16,
    pub usDay: u16,
    pub usHour: u16,
    pub usMinute: u16,
    pub usSecond: u16,
    pub usMilliseconds: u16,
    pub usWeekday: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMRECTS {
    pub c: u32,
    pub arcl: [super::super::Foundation::RECTL; 1],
}
impl Default for ENUMRECTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EngNumberOfProcessors: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(2i32);
pub const EngOptimumAvailableSystemMemory: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(4i32);
pub const EngOptimumAvailableUserMemory: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(3i32);
pub const EngProcessorFeature: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(1i32);
pub const FC_COMPLEX: u32 = 3u32;
pub const FC_RECT: u32 = 1u32;
pub const FC_RECT4: u32 = 2u32;
pub const FDM_TYPE_BM_SIDE_CONST: u32 = 1u32;
pub const FDM_TYPE_CHAR_INC_EQUAL_BM_BASE: u32 = 4u32;
pub const FDM_TYPE_CONST_BEARINGS: u32 = 16u32;
pub const FDM_TYPE_MAXEXT_EQUAL_BM_SIDE: u32 = 2u32;
pub const FDM_TYPE_ZERO_BEARINGS: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FD_DEVICEMETRICS {
    pub flRealizedType: u32,
    pub pteBase: POINTE,
    pub pteSide: POINTE,
    pub lD: i32,
    pub fxMaxAscender: i32,
    pub fxMaxDescender: i32,
    pub ptlUnderline1: super::super::Foundation::POINTL,
    pub ptlStrikeOut: super::super::Foundation::POINTL,
    pub ptlULThickness: super::super::Foundation::POINTL,
    pub ptlSOThickness: super::super::Foundation::POINTL,
    pub cxMax: u32,
    pub cyMax: u32,
    pub cjGlyphMax: u32,
    pub fdxQuantized: FD_XFORM,
    pub lNonLinearExtLeading: i32,
    pub lNonLinearIntLeading: i32,
    pub lNonLinearMaxCharWidth: i32,
    pub lNonLinearAvgCharWidth: i32,
    pub lMinA: i32,
    pub lMinC: i32,
    pub lMinD: i32,
    pub alReserved: [i32; 1],
}
impl Default for FD_DEVICEMETRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FD_ERROR: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FD_GLYPHATTR {
    pub cjThis: u32,
    pub cGlyphs: u32,
    pub iMode: u32,
    pub aGlyphAttr: [u8; 1],
}
impl Default for FD_GLYPHATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FD_GLYPHSET {
    pub cjThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRuns: u32,
    pub awcrun: [WCRUN; 1],
}
impl Default for FD_GLYPHSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FD_KERNINGPAIR {
    pub wcFirst: u16,
    pub wcSecond: u16,
    pub fwdKern: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FD_LIGATURE {
    pub culThis: u32,
    pub ulType: u32,
    pub cLigatures: u32,
    pub alig: [LIGATURE; 1],
}
impl Default for FD_LIGATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FD_NEGATIVE_FONT: i32 = 1i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FD_XFORM {
    pub eXX: u32,
    pub eXY: u32,
    pub eYX: u32,
    pub eYY: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FD_XFORM {
    pub eXX: f32,
    pub eXY: f32,
    pub eYX: f32,
    pub eYY: f32,
}
pub const FF_IGNORED_SIGNATURE: u32 = 2u32;
pub const FF_SIGNATURE_VERIFIED: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLOATOBJ {
    pub ul1: u32,
    pub ul2: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLOATOBJ_XFORM {
    pub eM11: FLOATOBJ,
    pub eM12: FLOATOBJ,
    pub eM21: FLOATOBJ,
    pub eM22: FLOATOBJ,
    pub eDx: FLOATOBJ,
    pub eDy: FLOATOBJ,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLOATOBJ_XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union FLOAT_LONG {
    pub e: u32,
    pub l: i32,
}
#[cfg(target_arch = "x86")]
impl Default for FLOAT_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union FLOAT_LONG {
    pub e: f32,
    pub l: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FLOAT_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FL_NONPAGED_MEMORY: u32 = 2u32;
pub const FL_NON_SESSION: u32 = 4u32;
pub const FL_ZERO_MEMORY: u32 = 1u32;
pub const FM_EDITABLE_EMBED: u32 = 8u32;
pub const FM_INFO_16BPP: u32 = 256u32;
pub const FM_INFO_1BPP: u32 = 32u32;
pub const FM_INFO_24BPP: u32 = 512u32;
pub const FM_INFO_32BPP: u32 = 1024u32;
pub const FM_INFO_4BPP: u32 = 64u32;
pub const FM_INFO_8BPP: u32 = 128u32;
pub const FM_INFO_90DEGREE_ROTATIONS: u32 = 2097152u32;
pub const FM_INFO_ANISOTROPIC_SCALING_ONLY: u32 = 33554432u32;
pub const FM_INFO_ARB_XFORMS: u32 = 16u32;
pub const FM_INFO_CONSTANT_WIDTH: u32 = 4096u32;
pub const FM_INFO_DBCS_FIXED_PITCH: u32 = 268435456u32;
pub const FM_INFO_DO_NOT_ENUMERATE: u32 = 8388608u32;
pub const FM_INFO_DSIG: u32 = 262144u32;
pub const FM_INFO_FAMILY_EQUIV: u32 = 134217728u32;
pub const FM_INFO_IGNORE_TC_RA_ABLE: u32 = 1073741824u32;
pub const FM_INFO_INTEGER_WIDTH: u32 = 2048u32;
pub const FM_INFO_INTEGRAL_SCALING: u32 = 1048576u32;
pub const FM_INFO_ISOTROPIC_SCALING_ONLY: u32 = 16777216u32;
pub const FM_INFO_NONNEGATIVE_AC: u32 = 536870912u32;
pub const FM_INFO_NOT_CONTIGUOUS: u32 = 8192u32;
pub const FM_INFO_OPTICALLY_FIXED_PITCH: u32 = 4194304u32;
pub const FM_INFO_RETURNS_BITMAPS: u32 = 131072u32;
pub const FM_INFO_RETURNS_OUTLINES: u32 = 32768u32;
pub const FM_INFO_RETURNS_STROKES: u32 = 65536u32;
pub const FM_INFO_RIGHT_HANDED: u32 = 524288u32;
pub const FM_INFO_TECH_BITMAP: u32 = 2u32;
pub const FM_INFO_TECH_CFF: u32 = 67108864u32;
pub const FM_INFO_TECH_MM: u32 = 16384u32;
pub const FM_INFO_TECH_OUTLINE_NOT_TRUETYPE: u32 = 8u32;
pub const FM_INFO_TECH_STROKE: u32 = 4u32;
pub const FM_INFO_TECH_TRUETYPE: u32 = 1u32;
pub const FM_INFO_TECH_TYPE1: u32 = 2147483648u32;
pub const FM_NO_EMBEDDING: u32 = 2u32;
pub const FM_PANOSE_CULTURE_LATIN: u32 = 0u32;
pub const FM_READONLY_EMBED: u32 = 4u32;
pub const FM_SEL_BOLD: u32 = 32u32;
pub const FM_SEL_ITALIC: u32 = 1u32;
pub const FM_SEL_NEGATIVE: u32 = 4u32;
pub const FM_SEL_OUTLINED: u32 = 8u32;
pub const FM_SEL_REGULAR: u32 = 64u32;
pub const FM_SEL_STRIKEOUT: u32 = 16u32;
pub const FM_SEL_UNDERSCORE: u32 = 2u32;
pub const FM_TYPE_LICENSED: u32 = 2u32;
pub const FM_VERSION_NUMBER: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FONTDIFF {
    pub jReserved1: u8,
    pub jReserved2: u8,
    pub jReserved3: u8,
    pub bWeight: u8,
    pub usWinWeight: u16,
    pub fsSelection: u16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub ptlCaret: super::super::Foundation::POINTL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FONTINFO {
    pub cjThis: u32,
    pub flCaps: u32,
    pub cGlyphsSupported: u32,
    pub cjMaxGlyph1: u32,
    pub cjMaxGlyph4: u32,
    pub cjMaxGlyph8: u32,
    pub cjMaxGlyph32: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FONTOBJ {
    pub iUniq: u32,
    pub iFace: u32,
    pub cxMax: u32,
    pub flFontType: u32,
    pub iTTUniq: usize,
    pub iFile: usize,
    pub sizLogResPpi: super::super::Foundation::SIZE,
    pub ulStyleSize: u32,
    pub pvConsumer: *mut core::ffi::c_void,
    pub pvProducer: *mut core::ffi::c_void,
}
impl Default for FONTOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FONTSIM {
    pub dpBold: i32,
    pub dpItalic: i32,
    pub dpBoldItalic: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FONT_IMAGE_INFO {
    pub FontSize: super::super::System::Console::COORD,
    pub ImageBits: *mut u8,
}
#[cfg(feature = "Win32_System_Console")]
impl Default for FONT_IMAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FO_ATTR_MODE_ROTATE: u32 = 1u32;
pub const FO_CFF: u32 = 1048576u32;
pub const FO_CLEARTYPENATURAL_X: u32 = 1073741824u32;
pub const FO_CLEARTYPE_X: u32 = 268435456u32;
pub const FO_CLEARTYPE_Y: u32 = 536870912u32;
pub const FO_DBCS_FONT: u32 = 16777216u32;
pub const FO_DEVICE_FONT: i32 = 1i32;
pub const FO_EM_HEIGHT: u32 = 32768u32;
pub const FO_GLYPHBITS: i32 = 1i32;
pub const FO_GRAY16: u32 = 65536u32;
pub const FO_HGLYPHS: i32 = 0i32;
pub const FO_MULTIPLEMASTER: u32 = 4194304u32;
pub const FO_NOCLEARTYPE: u32 = 33554432u32;
pub const FO_NOGRAY16: u32 = 131072u32;
pub const FO_NOHINTS: u32 = 262144u32;
pub const FO_NO_CHOICE: u32 = 524288u32;
pub const FO_OUTLINE_CAPABLE: i32 = 2i32;
pub const FO_PATHOBJ: i32 = 2i32;
pub const FO_POSTSCRIPT: u32 = 2097152u32;
pub const FO_SIM_BOLD: u32 = 8192u32;
pub const FO_SIM_ITALIC: u32 = 16384u32;
pub const FO_VERT_FACE: u32 = 8388608u32;
pub const FP_ALTERNATEMODE: i32 = 1i32;
pub const FP_WINDINGMODE: i32 = 2i32;
pub type FREEOBJPROC = Option<unsafe extern "system" fn(pdriverobj: *mut DRIVEROBJ) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSCNTL_SCREEN_INFO {
    pub Position: super::super::System::Console::COORD,
    pub ScreenSize: super::super::System::Console::COORD,
    pub nNumberOfChars: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSVIDEO_COPY_FRAME_BUFFER {
    pub SrcScreen: FSCNTL_SCREEN_INFO,
    pub DestScreen: FSCNTL_SCREEN_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSVIDEO_CURSOR_POSITION {
    pub Coord: VIDEO_CURSOR_POSITION,
    pub dwType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSVIDEO_MODE_INFORMATION {
    pub VideoMode: VIDEO_MODE_INFORMATION,
    pub VideoMemory: VIDEO_MEMORY_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSVIDEO_REVERSE_MOUSE_POINTER {
    pub Screen: FSCNTL_SCREEN_INFO,
    pub dwType: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FSVIDEO_SCREEN_INFORMATION {
    pub ScreenSize: super::super::System::Console::COORD,
    pub FontSize: super::super::System::Console::COORD,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FSVIDEO_WRITE_TO_FRAME_BUFFER {
    pub SrcBuffer: *mut CHAR_IMAGE_INFO,
    pub DestScreen: FSCNTL_SCREEN_INFO,
}
#[cfg(feature = "Win32_System_Console")]
impl Default for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GAMMARAMP {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl Default for GAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GAMMA_RAMP_DXGI_1 {
    pub Scale: GAMMA_RAMP_RGB,
    pub Offset: GAMMA_RAMP_RGB,
    pub GammaCurve: [GAMMA_RAMP_RGB; 1025],
}
impl Default for GAMMA_RAMP_DXGI_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GAMMA_RAMP_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GAMMA_RAMP_RGB256x3x16 {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl Default for GAMMA_RAMP_RGB256x3x16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GCAPS2_ACC_DRIVER: u32 = 32768u32;
pub const GCAPS2_ALPHACURSOR: u32 = 32u32;
pub const GCAPS2_BITMAPEXREUSE: u32 = 65536u32;
pub const GCAPS2_CHANGEGAMMARAMP: u32 = 16u32;
pub const GCAPS2_CLEARTYPE: u32 = 16384u32;
pub const GCAPS2_EXCLUDELAYERED: u32 = 2048u32;
pub const GCAPS2_ICD_MULTIMON: u32 = 256u32;
pub const GCAPS2_INCLUDEAPIBITMAPS: u32 = 4096u32;
pub const GCAPS2_JPEGSRC: u32 = 1u32;
pub const GCAPS2_MOUSETRAILS: u32 = 512u32;
pub const GCAPS2_PNGSRC: u32 = 8u32;
pub const GCAPS2_REMOTEDRIVER: u32 = 1024u32;
pub const GCAPS2_RESERVED1: u32 = 1024u32;
pub const GCAPS2_SHOWHIDDENPOINTER: u32 = 8192u32;
pub const GCAPS2_SYNCFLUSH: u32 = 64u32;
pub const GCAPS2_SYNCTIMER: u32 = 128u32;
pub const GCAPS2_xxxx: u32 = 2u32;
pub const GCAPS_ALTERNATEFILL: u32 = 4u32;
pub const GCAPS_ARBRUSHOPAQUE: u32 = 32768u32;
pub const GCAPS_ARBRUSHTEXT: u32 = 268435456u32;
pub const GCAPS_ASYNCCHANGE: u32 = 2048u32;
pub const GCAPS_ASYNCMOVE: u32 = 4096u32;
pub const GCAPS_BEZIERS: u32 = 1u32;
pub const GCAPS_CMYKCOLOR: u32 = 67108864u32;
pub const GCAPS_COLOR_DITHER: u32 = 32u32;
pub const GCAPS_DIRECTDRAW: u32 = 16384u32;
pub const GCAPS_DITHERONREALIZE: u32 = 2097152u32;
pub const GCAPS_DONTJOURNAL: u32 = 8192u32;
pub const GCAPS_FONT_RASTERIZER: u32 = 1073741824u32;
pub const GCAPS_FORCEDITHER: u32 = 8388608u32;
pub const GCAPS_GEOMETRICWIDE: u32 = 2u32;
pub const GCAPS_GRAY16: u32 = 16777216u32;
pub const GCAPS_HALFTONE: u32 = 16u32;
pub const GCAPS_HIGHRESTEXT: u32 = 262144u32;
pub const GCAPS_HORIZSTRIKE: u32 = 64u32;
pub const GCAPS_ICM: u32 = 33554432u32;
pub const GCAPS_LAYERED: u32 = 134217728u32;
pub const GCAPS_MONO_DITHER: u32 = 1024u32;
pub const GCAPS_NO64BITMEMACCESS: u32 = 4194304u32;
pub const GCAPS_NUP: u32 = 2147483648u32;
pub const GCAPS_OPAQUERECT: u32 = 256u32;
pub const GCAPS_PALMANAGED: u32 = 524288u32;
pub const GCAPS_PANNING: u32 = 65536u32;
pub const GCAPS_SCREENPRECISION: u32 = 536870912u32;
pub const GCAPS_VECTORFONT: u32 = 512u32;
pub const GCAPS_VERTSTRIKE: u32 = 128u32;
pub const GCAPS_WINDINGFILL: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GDIINFO {
    pub ulVersion: u32,
    pub ulTechnology: u32,
    pub ulHorzSize: u32,
    pub ulVertSize: u32,
    pub ulHorzRes: u32,
    pub ulVertRes: u32,
    pub cBitsPixel: u32,
    pub cPlanes: u32,
    pub ulNumColors: u32,
    pub flRaster: u32,
    pub ulLogPixelsX: u32,
    pub ulLogPixelsY: u32,
    pub flTextCaps: u32,
    pub ulDACRed: u32,
    pub ulDACGreen: u32,
    pub ulDACBlue: u32,
    pub ulAspectX: u32,
    pub ulAspectY: u32,
    pub ulAspectXY: u32,
    pub xStyleStep: i32,
    pub yStyleStep: i32,
    pub denStyleStep: i32,
    pub ptlPhysOffset: super::super::Foundation::POINTL,
    pub szlPhysSize: super::super::Foundation::SIZE,
    pub ulNumPalReg: u32,
    pub ciDevice: COLORINFO,
    pub ulDevicePelsDPI: u32,
    pub ulPrimaryOrder: u32,
    pub ulHTPatternSize: u32,
    pub ulHTOutputFormat: u32,
    pub flHTFlags: u32,
    pub ulVRefresh: u32,
    pub ulBltAlignment: u32,
    pub ulPanningHorzRes: u32,
    pub ulPanningVertRes: u32,
    pub xPanningAlignment: u32,
    pub yPanningAlignment: u32,
    pub cxHTPat: u32,
    pub cyHTPat: u32,
    pub pHTPatA: *mut u8,
    pub pHTPatB: *mut u8,
    pub pHTPatC: *mut u8,
    pub flShadeBlend: u32,
    pub ulPhysicalPixelCharacteristics: u32,
    pub ulPhysicalPixelGamma: u32,
}
impl Default for GDIINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GDI_DRIVER_VERSION: u32 = 16384u32;
pub const GETCONNECTEDIDS_SOURCE: u32 = 1u32;
pub const GETCONNECTEDIDS_TARGET: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GLYPHBITS {
    pub ptlOrigin: super::super::Foundation::POINTL,
    pub sizlBitmap: super::super::Foundation::SIZE,
    pub aj: [u8; 1],
}
impl Default for GLYPHBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GLYPHDATA {
    pub gdf: GLYPHDEF,
    pub hg: u32,
    pub fxD: i32,
    pub fxA: i32,
    pub fxAB: i32,
    pub fxInkTop: i32,
    pub fxInkBottom: i32,
    pub rclInk: super::super::Foundation::RECTL,
    pub ptqD: POINTQF,
}
impl Default for GLYPHDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GLYPHDEF {
    pub pgb: *mut GLYPHBITS,
    pub ppo: *mut PATHOBJ,
}
impl Default for GLYPHDEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GLYPHPOS {
    pub hg: u32,
    pub pgdf: *mut GLYPHDEF,
    pub ptl: super::super::Foundation::POINTL,
}
impl Default for GLYPHPOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GS_16BIT_HANDLES: u32 = 4u32;
pub const GS_8BIT_HANDLES: u32 = 2u32;
pub const GS_UNICODE_HANDLES: u32 = 1u32;
pub const GUID_DEVINTERFACE_DISPLAY_ADAPTER: windows_core::GUID = windows_core::GUID::from_u128(0x5b45201d_f2f2_4f3b_85bb_30ff1f953599);
pub const GUID_DEVINTERFACE_MONITOR: windows_core::GUID = windows_core::GUID::from_u128(0xe6f07b5f_ee97_4a90_b076_33f57bf4eaa7);
pub const GUID_DEVINTERFACE_VIDEO_OUTPUT_ARRIVAL: windows_core::GUID = windows_core::GUID::from_u128(0x1ad9e4f0_f88d_4360_bab9_4c2d55e564cd);
pub const GUID_DISPLAY_DEVICE_ARRIVAL: windows_core::GUID = windows_core::GUID::from_u128(0x1ca05180_a699_450a_9a0c_de4fbe3ddd89);
pub const GUID_MONITOR_OVERRIDE_PSEUDO_SPECIALIZED: windows_core::GUID = windows_core::GUID::from_u128(0xf196c02f_f86f_4f9a_aa15_e9cebdfe3b96);
pub const GX_GENERAL: i32 = 3i32;
pub const GX_IDENTITY: i32 = 0i32;
pub const GX_OFFSET: i32 = 1i32;
pub const GX_SCALE: i32 = 2i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HBM(pub *mut core::ffi::c_void);
impl HBM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HBM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDEV(pub *mut core::ffi::c_void);
impl HDEV {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HDEV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVOBJ(pub *mut core::ffi::c_void);
impl HDRVOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HDRVOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HFASTMUTEX(pub *mut core::ffi::c_void);
impl HFASTMUTEX {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HFASTMUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HOOK_ALPHABLEND: u32 = 65536u32;
pub const HOOK_BITBLT: u32 = 1u32;
pub const HOOK_COPYBITS: u32 = 1024u32;
pub const HOOK_FILLPATH: u32 = 64u32;
pub const HOOK_FLAGS: u32 = 243199u32;
pub const HOOK_GRADIENTFILL: u32 = 131072u32;
pub const HOOK_LINETO: u32 = 256u32;
pub const HOOK_MOVEPANNING: u32 = 2048u32;
pub const HOOK_PAINT: u32 = 16u32;
pub const HOOK_PLGBLT: u32 = 4u32;
pub const HOOK_STRETCHBLT: u32 = 2u32;
pub const HOOK_STRETCHBLTROP: u32 = 8192u32;
pub const HOOK_STROKEANDFILLPATH: u32 = 128u32;
pub const HOOK_STROKEPATH: u32 = 32u32;
pub const HOOK_SYNCHRONIZE: u32 = 4096u32;
pub const HOOK_SYNCHRONIZEACCESS: u32 = 16384u32;
pub const HOOK_TEXTOUT: u32 = 8u32;
pub const HOOK_TRANSPARENTBLT: u32 = 32768u32;
pub const HOST_DSI_BAD_TRANSMISSION_MODE: u32 = 4096u32;
pub const HOST_DSI_DEVICE_NOT_READY: u32 = 1u32;
pub const HOST_DSI_DEVICE_RESET: u32 = 4u32;
pub const HOST_DSI_DRIVER_REJECTED_PACKET: u32 = 1024u32;
pub const HOST_DSI_INTERFACE_RESET: u32 = 2u32;
pub const HOST_DSI_INVALID_TRANSMISSION: u32 = 256u32;
pub const HOST_DSI_OS_REJECTED_PACKET: u32 = 512u32;
pub const HOST_DSI_TRANSMISSION_CANCELLED: u32 = 16u32;
pub const HOST_DSI_TRANSMISSION_DROPPED: u32 = 32u32;
pub const HOST_DSI_TRANSMISSION_TIMEOUT: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSEMAPHORE(pub *mut core::ffi::c_void);
impl HSEMAPHORE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HSEMAPHORE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_link::link!("gdi32.dll" "system" fn EngDeleteSemaphore(hsem : *mut core::ffi::c_void));
            unsafe {
                EngDeleteSemaphore(self.0);
            }
        }
    }
}
impl Default for HSEMAPHORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSURF(pub *mut core::ffi::c_void);
impl HSURF {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HSURF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HS_DDI_MAX: u32 = 6u32;
pub const HT_FLAG_8BPP_CMY332_MASK: u32 = 4278190080u32;
pub const HT_FLAG_ADDITIVE_PRIMS: u32 = 4u32;
pub const HT_FLAG_DO_DEVCLR_XFORM: u32 = 128u32;
pub const HT_FLAG_HAS_BLACK_DYE: u32 = 2u32;
pub const HT_FLAG_INK_ABSORPTION_IDX0: u32 = 0u32;
pub const HT_FLAG_INK_ABSORPTION_IDX1: u32 = 32u32;
pub const HT_FLAG_INK_ABSORPTION_IDX2: u32 = 64u32;
pub const HT_FLAG_INK_ABSORPTION_IDX3: u32 = 96u32;
pub const HT_FLAG_INK_ABSORPTION_INDICES: u32 = 96u32;
pub const HT_FLAG_INK_HIGH_ABSORPTION: u32 = 16u32;
pub const HT_FLAG_INVERT_8BPP_BITMASK_IDX: u32 = 1024u32;
pub const HT_FLAG_LOWER_INK_ABSORPTION: u32 = 64u32;
pub const HT_FLAG_LOWEST_INK_ABSORPTION: u32 = 96u32;
pub const HT_FLAG_LOW_INK_ABSORPTION: u32 = 32u32;
pub const HT_FLAG_NORMAL_INK_ABSORPTION: u32 = 0u32;
pub const HT_FLAG_OUTPUT_CMY: u32 = 256u32;
pub const HT_FLAG_PRINT_DRAFT_MODE: u32 = 512u32;
pub const HT_FLAG_SQUARE_DEVICE_PEL: u32 = 1u32;
pub const HT_FLAG_USE_8BPP_BITMASK: u32 = 8u32;
pub const HT_FORMAT_16BPP: u32 = 5u32;
pub const HT_FORMAT_1BPP: u32 = 0u32;
pub const HT_FORMAT_24BPP: u32 = 6u32;
pub const HT_FORMAT_32BPP: u32 = 7u32;
pub const HT_FORMAT_4BPP: u32 = 2u32;
pub const HT_FORMAT_4BPP_IRGB: u32 = 3u32;
pub const HT_FORMAT_8BPP: u32 = 4u32;
pub const HT_PATSIZE_10x10: u32 = 8u32;
pub const HT_PATSIZE_10x10_M: u32 = 9u32;
pub const HT_PATSIZE_12x12: u32 = 10u32;
pub const HT_PATSIZE_12x12_M: u32 = 11u32;
pub const HT_PATSIZE_14x14: u32 = 12u32;
pub const HT_PATSIZE_14x14_M: u32 = 13u32;
pub const HT_PATSIZE_16x16: u32 = 14u32;
pub const HT_PATSIZE_16x16_M: u32 = 15u32;
pub const HT_PATSIZE_2x2: u32 = 0u32;
pub const HT_PATSIZE_2x2_M: u32 = 1u32;
pub const HT_PATSIZE_4x4: u32 = 2u32;
pub const HT_PATSIZE_4x4_M: u32 = 3u32;
pub const HT_PATSIZE_6x6: u32 = 4u32;
pub const HT_PATSIZE_6x6_M: u32 = 5u32;
pub const HT_PATSIZE_8x8: u32 = 6u32;
pub const HT_PATSIZE_8x8_M: u32 = 7u32;
pub const HT_PATSIZE_DEFAULT: u32 = 17u32;
pub const HT_PATSIZE_MAX_INDEX: u32 = 18u32;
pub const HT_PATSIZE_SUPERCELL: u32 = 16u32;
pub const HT_PATSIZE_SUPERCELL_M: u32 = 17u32;
pub const HT_PATSIZE_USER: u32 = 18u32;
pub const HT_USERPAT_CX_MAX: u32 = 256u32;
pub const HT_USERPAT_CX_MIN: u32 = 4u32;
pub const HT_USERPAT_CY_MAX: u32 = 256u32;
pub const HT_USERPAT_CY_MIN: u32 = 4u32;
windows_core::imp::define_interface!(ICloneViewHelper, ICloneViewHelper_Vtbl, 0xf6a3d4c4_5632_4d83_b0a1_fb88712b1eb7);
windows_core::imp::interface_hierarchy!(ICloneViewHelper, windows_core::IUnknown);
impl ICloneViewHelper {
    pub unsafe fn GetConnectedIDs<P0>(&self, wszadaptorname: P0, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConnectedIDs)(windows_core::Interface::as_raw(self), wszadaptorname.param().abi(), pulcount as _, pulid as _, ulflags).ok() }
    }
    pub unsafe fn GetActiveTopology<P0>(&self, wszadaptorname: P0, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetActiveTopology)(windows_core::Interface::as_raw(self), wszadaptorname.param().abi(), ulsourceid, pulcount as _, pultargetid as _).ok() }
    }
    pub unsafe fn SetActiveTopology<P0>(&self, wszadaptorname: P0, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveTopology)(windows_core::Interface::as_raw(self), wszadaptorname.param().abi(), ulsourceid, ulcount, pultargetid).ok() }
    }
    pub unsafe fn Commit(&self, ffinalcall: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), ffinalcall.into()).ok() }
    }
}
#[repr(C)]
pub struct ICloneViewHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectedIDs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut u32, u32) -> windows_core::HRESULT,
    pub GetActiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetActiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ICloneViewHelper_Impl: windows_core::IUnknownImpl {
    fn GetConnectedIDs(&self, wszadaptorname: &windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::Result<()>;
    fn GetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::Result<()>;
    fn SetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self, ffinalcall: windows_core::BOOL) -> windows_core::Result<()>;
}
impl ICloneViewHelper_Vtbl {
    pub const fn new<Identity: ICloneViewHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectedIDs<Identity: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICloneViewHelper_Impl::GetConnectedIDs(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pulid), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetActiveTopology<Identity: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICloneViewHelper_Impl::GetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pultargetid)).into()
            }
        }
        unsafe extern "system" fn SetActiveTopology<Identity: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICloneViewHelper_Impl::SetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pultargetid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffinalcall: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICloneViewHelper_Impl::Commit(this, core::mem::transmute_copy(&ffinalcall)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Identity, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICloneViewHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICloneViewHelper {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IFIEXTRA {
    pub ulIdentifier: u32,
    pub dpFontSig: i32,
    pub cig: u32,
    pub dpDesignVector: i32,
    pub dpAxesInfoW: i32,
    pub aulReserved: [u32; 1],
}
impl Default for IFIEXTRA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IFIMETRICS {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::super::Graphics::Gdi::PANOSE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IFIMETRICS {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::super::Graphics::Gdi::PANOSE,
    pub Align: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IGRF_RGB_256BYTES: u32 = 0u32;
pub const IGRF_RGB_256WORDS: u32 = 1u32;
pub const INDEX_DrvAccumulateD3DDirtyRect: i32 = 98i32;
pub const INDEX_DrvAlphaBlend: i32 = 71i32;
pub const INDEX_DrvAssertMode: i32 = 5i32;
pub const INDEX_DrvAssociateSharedSurface: i32 = 96i32;
pub const INDEX_DrvBitBlt: i32 = 18i32;
pub const INDEX_DrvCompletePDEV: i32 = 1i32;
pub const INDEX_DrvCopyBits: i32 = 19i32;
pub const INDEX_DrvCreateDeviceBitmap: i32 = 10i32;
pub const INDEX_DrvCreateDeviceBitmapEx: i32 = 94i32;
pub const INDEX_DrvDeleteDeviceBitmap: i32 = 11i32;
pub const INDEX_DrvDeleteDeviceBitmapEx: i32 = 95i32;
pub const INDEX_DrvDeriveSurface: i32 = 85i32;
pub const INDEX_DrvDescribePixelFormat: i32 = 55i32;
pub const INDEX_DrvDestroyFont: i32 = 43i32;
pub const INDEX_DrvDisableDirectDraw: i32 = 61i32;
pub const INDEX_DrvDisableDriver: i32 = 8i32;
pub const INDEX_DrvDisablePDEV: i32 = 2i32;
pub const INDEX_DrvDisableSurface: i32 = 4i32;
pub const INDEX_DrvDitherColor: i32 = 13i32;
pub const INDEX_DrvDrawEscape: i32 = 25i32;
pub const INDEX_DrvEnableDirectDraw: i32 = 60i32;
pub const INDEX_DrvEnablePDEV: i32 = 0i32;
pub const INDEX_DrvEnableSurface: i32 = 3i32;
pub const INDEX_DrvEndDoc: i32 = 34i32;
pub const INDEX_DrvEndDxInterop: i32 = 100i32;
pub const INDEX_DrvEscape: i32 = 24i32;
pub const INDEX_DrvFillPath: i32 = 15i32;
pub const INDEX_DrvFontManagement: i32 = 47i32;
pub const INDEX_DrvFree: i32 = 42i32;
pub const INDEX_DrvGetDirectDrawInfo: i32 = 59i32;
pub const INDEX_DrvGetGlyphMode: i32 = 37i32;
pub const INDEX_DrvGetModes: i32 = 41i32;
pub const INDEX_DrvGetSynthesizedFontFiles: i32 = 73i32;
pub const INDEX_DrvGetTrueTypeFile: i32 = 50i32;
pub const INDEX_DrvGradientFill: i32 = 68i32;
pub const INDEX_DrvIcmCheckBitmapBits: i32 = 66i32;
pub const INDEX_DrvIcmCreateColorTransform: i32 = 64i32;
pub const INDEX_DrvIcmDeleteColorTransform: i32 = 65i32;
pub const INDEX_DrvIcmSetDeviceGammaRamp: i32 = 67i32;
pub const INDEX_DrvLineTo: i32 = 31i32;
pub const INDEX_DrvLoadFontFile: i32 = 45i32;
pub const INDEX_DrvLockDisplayArea: i32 = 101i32;
pub const INDEX_DrvMovePanning: i32 = 52i32;
pub const INDEX_DrvMovePointer: i32 = 30i32;
pub const INDEX_DrvNextBand: i32 = 58i32;
pub const INDEX_DrvNotify: i32 = 87i32;
pub const INDEX_DrvOffset: i32 = 6i32;
pub const INDEX_DrvPaint: i32 = 17i32;
pub const INDEX_DrvPlgBlt: i32 = 70i32;
pub const INDEX_DrvQueryAdvanceWidths: i32 = 53i32;
pub const INDEX_DrvQueryDeviceSupport: i32 = 76i32;
pub const INDEX_DrvQueryFont: i32 = 26i32;
pub const INDEX_DrvQueryFontCaps: i32 = 44i32;
pub const INDEX_DrvQueryFontData: i32 = 28i32;
pub const INDEX_DrvQueryFontFile: i32 = 51i32;
pub const INDEX_DrvQueryFontTree: i32 = 27i32;
pub const INDEX_DrvQueryGlyphAttrs: i32 = 86i32;
pub const INDEX_DrvQueryPerBandInfo: i32 = 75i32;
pub const INDEX_DrvQuerySpoolType: i32 = 62i32;
pub const INDEX_DrvQueryTrueTypeOutline: i32 = 49i32;
pub const INDEX_DrvQueryTrueTypeTable: i32 = 48i32;
pub const INDEX_DrvRealizeBrush: i32 = 12i32;
pub const INDEX_DrvRenderHint: i32 = 93i32;
pub const INDEX_DrvReserved1: i32 = 77i32;
pub const INDEX_DrvReserved10: i32 = 91i32;
pub const INDEX_DrvReserved11: i32 = 92i32;
pub const INDEX_DrvReserved2: i32 = 78i32;
pub const INDEX_DrvReserved3: i32 = 79i32;
pub const INDEX_DrvReserved4: i32 = 80i32;
pub const INDEX_DrvReserved5: i32 = 81i32;
pub const INDEX_DrvReserved6: i32 = 82i32;
pub const INDEX_DrvReserved7: i32 = 83i32;
pub const INDEX_DrvReserved8: i32 = 84i32;
pub const INDEX_DrvReserved9: i32 = 90i32;
pub const INDEX_DrvResetDevice: i32 = 89i32;
pub const INDEX_DrvResetPDEV: i32 = 7i32;
pub const INDEX_DrvSaveScreenBits: i32 = 40i32;
pub const INDEX_DrvSendPage: i32 = 32i32;
pub const INDEX_DrvSetPalette: i32 = 22i32;
pub const INDEX_DrvSetPixelFormat: i32 = 54i32;
pub const INDEX_DrvSetPointerShape: i32 = 29i32;
pub const INDEX_DrvStartBanding: i32 = 57i32;
pub const INDEX_DrvStartDoc: i32 = 35i32;
pub const INDEX_DrvStartDxInterop: i32 = 99i32;
pub const INDEX_DrvStartPage: i32 = 33i32;
pub const INDEX_DrvStretchBlt: i32 = 20i32;
pub const INDEX_DrvStretchBltROP: i32 = 69i32;
pub const INDEX_DrvStrokeAndFillPath: i32 = 16i32;
pub const INDEX_DrvStrokePath: i32 = 14i32;
pub const INDEX_DrvSurfaceComplete: i32 = 103i32;
pub const INDEX_DrvSwapBuffers: i32 = 56i32;
pub const INDEX_DrvSynchronize: i32 = 38i32;
pub const INDEX_DrvSynchronizeRedirectionBitmaps: i32 = 97i32;
pub const INDEX_DrvSynchronizeSurface: i32 = 88i32;
pub const INDEX_DrvSynthesizeFont: i32 = 72i32;
pub const INDEX_DrvTextOut: i32 = 23i32;
pub const INDEX_DrvTransparentBlt: i32 = 74i32;
pub const INDEX_DrvUnloadFontFile: i32 = 46i32;
pub const INDEX_DrvUnlockDisplayArea: i32 = 102i32;
pub const INDEX_LAST: i32 = 89i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INDIRECT_DISPLAY_INFO {
    pub DisplayAdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub NumMonitors: u32,
    pub DisplayAdapterTargetBase: u32,
}
pub const INDIRECT_DISPLAY_INFO_FLAGS_CREATED_IDDCX_ADAPTER: u32 = 1u32;
pub const IOCTL_COLORSPACE_TRANSFORM_QUERY_TARGET_CAPS: u32 = 2297856u32;
pub const IOCTL_COLORSPACE_TRANSFORM_SET: u32 = 2297860u32;
pub const IOCTL_FSVIDEO_COPY_FRAME_BUFFER: u32 = 3409920u32;
pub const IOCTL_FSVIDEO_REVERSE_MOUSE_POINTER: u32 = 3409928u32;
pub const IOCTL_FSVIDEO_SET_CURRENT_MODE: u32 = 3409932u32;
pub const IOCTL_FSVIDEO_SET_CURSOR_POSITION: u32 = 3409940u32;
pub const IOCTL_FSVIDEO_SET_SCREEN_INFORMATION: u32 = 3409936u32;
pub const IOCTL_FSVIDEO_WRITE_TO_FRAME_BUFFER: u32 = 3409924u32;
pub const IOCTL_MIPI_DSI_QUERY_CAPS: u32 = 2298880u32;
pub const IOCTL_MIPI_DSI_RESET: u32 = 2298888u32;
pub const IOCTL_MIPI_DSI_TRANSMISSION: u32 = 2298884u32;
pub const IOCTL_PANEL_GET_BACKLIGHT_REDUCTION: u32 = 2296856u32;
pub const IOCTL_PANEL_GET_BRIGHTNESS: u32 = 2296840u32;
pub const IOCTL_PANEL_GET_MANUFACTURING_MODE: u32 = 2296860u32;
pub const IOCTL_PANEL_QUERY_BRIGHTNESS_CAPS: u32 = 2296832u32;
pub const IOCTL_PANEL_QUERY_BRIGHTNESS_RANGES: u32 = 2296836u32;
pub const IOCTL_PANEL_SET_BACKLIGHT_OPTIMIZATION: u32 = 2296852u32;
pub const IOCTL_PANEL_SET_BRIGHTNESS: u32 = 2296844u32;
pub const IOCTL_PANEL_SET_BRIGHTNESS_STATE: u32 = 2296848u32;
pub const IOCTL_SET_ACTIVE_COLOR_PROFILE_NAME: u32 = 2297864u32;
pub const IOCTL_VIDEO_DISABLE_CURSOR: u32 = 2294820u32;
pub const IOCTL_VIDEO_DISABLE_POINTER: u32 = 2294844u32;
pub const IOCTL_VIDEO_DISABLE_VDM: u32 = 2293764u32;
pub const IOCTL_VIDEO_ENABLE_CURSOR: u32 = 2294816u32;
pub const IOCTL_VIDEO_ENABLE_POINTER: u32 = 2294840u32;
pub const IOCTL_VIDEO_ENABLE_VDM: u32 = 2293760u32;
pub const IOCTL_VIDEO_ENUM_MONITOR_PDO: u32 = 2293784u32;
pub const IOCTL_VIDEO_FREE_PUBLIC_ACCESS_RANGES: u32 = 2294884u32;
pub const IOCTL_VIDEO_GET_BANK_SELECT_CODE: u32 = 2294868u32;
pub const IOCTL_VIDEO_GET_CHILD_STATE: u32 = 2294912u32;
pub const IOCTL_VIDEO_GET_OUTPUT_DEVICE_POWER_STATE: u32 = 2293776u32;
pub const IOCTL_VIDEO_GET_POWER_MANAGEMENT: u32 = 2294896u32;
pub const IOCTL_VIDEO_HANDLE_VIDEOPARAMETERS: u32 = 2293792u32;
pub const IOCTL_VIDEO_INIT_WIN32K_CALLBACKS: u32 = 2293788u32;
pub const IOCTL_VIDEO_IS_VGA_DEVICE: u32 = 2293796u32;
pub const IOCTL_VIDEO_LOAD_AND_SET_FONT: u32 = 2294804u32;
pub const IOCTL_VIDEO_MAP_VIDEO_MEMORY: u32 = 2294872u32;
pub const IOCTL_VIDEO_MONITOR_DEVICE: u32 = 2293780u32;
pub const IOCTL_VIDEO_PREPARE_FOR_EARECOVERY: u32 = 2293804u32;
pub const IOCTL_VIDEO_QUERY_AVAIL_MODES: u32 = 2294784u32;
pub const IOCTL_VIDEO_QUERY_COLOR_CAPABILITIES: u32 = 2294888u32;
pub const IOCTL_VIDEO_QUERY_CURRENT_MODE: u32 = 2294792u32;
pub const IOCTL_VIDEO_QUERY_CURSOR_ATTR: u32 = 2294828u32;
pub const IOCTL_VIDEO_QUERY_CURSOR_POSITION: u32 = 2294836u32;
pub const IOCTL_VIDEO_QUERY_DISPLAY_BRIGHTNESS: u32 = 2294936u32;
pub const IOCTL_VIDEO_QUERY_NUM_AVAIL_MODES: u32 = 2294788u32;
pub const IOCTL_VIDEO_QUERY_POINTER_ATTR: u32 = 2294852u32;
pub const IOCTL_VIDEO_QUERY_POINTER_CAPABILITIES: u32 = 2294864u32;
pub const IOCTL_VIDEO_QUERY_POINTER_POSITION: u32 = 2294860u32;
pub const IOCTL_VIDEO_QUERY_PUBLIC_ACCESS_RANGES: u32 = 2294880u32;
pub const IOCTL_VIDEO_QUERY_SUPPORTED_BRIGHTNESS: u32 = 2294932u32;
pub const IOCTL_VIDEO_REGISTER_VDM: u32 = 2293768u32;
pub const IOCTL_VIDEO_RESET_DEVICE: u32 = 2294800u32;
pub const IOCTL_VIDEO_RESTORE_HARDWARE_STATE: u32 = 2294276u32;
pub const IOCTL_VIDEO_SAVE_HARDWARE_STATE: u32 = 2294272u32;
pub const IOCTL_VIDEO_SET_BANK_POSITION: u32 = 2294928u32;
pub const IOCTL_VIDEO_SET_CHILD_STATE_CONFIGURATION: u32 = 2294920u32;
pub const IOCTL_VIDEO_SET_COLOR_LUT_DATA: u32 = 2294908u32;
pub const IOCTL_VIDEO_SET_COLOR_REGISTERS: u32 = 2294812u32;
pub const IOCTL_VIDEO_SET_CURRENT_MODE: u32 = 2294796u32;
pub const IOCTL_VIDEO_SET_CURSOR_ATTR: u32 = 2294824u32;
pub const IOCTL_VIDEO_SET_CURSOR_POSITION: u32 = 2294832u32;
pub const IOCTL_VIDEO_SET_DISPLAY_BRIGHTNESS: u32 = 2294940u32;
pub const IOCTL_VIDEO_SET_OUTPUT_DEVICE_POWER_STATE: u32 = 2293772u32;
pub const IOCTL_VIDEO_SET_PALETTE_REGISTERS: u32 = 2294808u32;
pub const IOCTL_VIDEO_SET_POINTER_ATTR: u32 = 2294848u32;
pub const IOCTL_VIDEO_SET_POINTER_POSITION: u32 = 2294856u32;
pub const IOCTL_VIDEO_SET_POWER_MANAGEMENT: u32 = 2294892u32;
pub const IOCTL_VIDEO_SHARE_VIDEO_MEMORY: u32 = 2294900u32;
pub const IOCTL_VIDEO_SWITCH_DUALVIEW: u32 = 2294924u32;
pub const IOCTL_VIDEO_UNMAP_VIDEO_MEMORY: u32 = 2294876u32;
pub const IOCTL_VIDEO_UNSHARE_VIDEO_MEMORY: u32 = 2294904u32;
pub const IOCTL_VIDEO_USE_DEVICE_IN_SESSION: u32 = 2293800u32;
pub const IOCTL_VIDEO_VALIDATE_CHILD_STATE_CONFIGURATION: u32 = 2294916u32;
windows_core::imp::define_interface!(IViewHelper, IViewHelper_Vtbl, 0xe85ccef5_aaaa_47f0_b5e3_61f7aecdc4c1);
windows_core::imp::interface_hierarchy!(IViewHelper, windows_core::IUnknown);
impl IViewHelper {
    pub unsafe fn GetConnectedIDs<P0>(&self, wszadaptorname: P0, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConnectedIDs)(windows_core::Interface::as_raw(self), wszadaptorname.param().abi(), pulcount as _, pulid as _, ulflags).ok() }
    }
    pub unsafe fn GetActiveTopology<P0>(&self, wszadaptorname: P0, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetActiveTopology)(windows_core::Interface::as_raw(self), wszadaptorname.param().abi(), ulsourceid, pulcount as _, pultargetid as _).ok() }
    }
    pub unsafe fn SetActiveTopology<P0>(&self, wszadaptorname: P0, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActiveTopology)(windows_core::Interface::as_raw(self), wszadaptorname.param().abi(), ulsourceid, ulcount, pultargetid).ok() }
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetConfiguration<P0>(&self, pistream: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), pistream.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProceedOnNewConfiguration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProceedOnNewConfiguration)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
pub struct IViewHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectedIDs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut u32, u32) -> windows_core::HRESULT,
    pub GetActiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetActiveTopology: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetConfiguration: usize,
    pub GetProceedOnNewConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IViewHelper_Impl: windows_core::IUnknownImpl {
    fn GetConnectedIDs(&self, wszadaptorname: &windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::Result<()>;
    fn GetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::Result<()>;
    fn SetActiveTopology(&self, wszadaptorname: &windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn SetConfiguration(&self, pistream: windows_core::Ref<'_, super::super::System::Com::IStream>) -> windows_core::Result<u32>;
    fn GetProceedOnNewConfiguration(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IViewHelper_Vtbl {
    pub const fn new<Identity: IViewHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectedIDs<Identity: IViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewHelper_Impl::GetConnectedIDs(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pulid), core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetActiveTopology<Identity: IViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewHelper_Impl::GetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&pulcount), core::mem::transmute_copy(&pultargetid)).into()
            }
        }
        unsafe extern "system" fn SetActiveTopology<Identity: IViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszadaptorname: windows_core::PCWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewHelper_Impl::SetActiveTopology(this, core::mem::transmute(&wszadaptorname), core::mem::transmute_copy(&ulsourceid), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&pultargetid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewHelper_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: IViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistream: *mut core::ffi::c_void, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewHelper_Impl::SetConfiguration(this, core::mem::transmute_copy(&pistream)) {
                    Ok(ok__) => {
                        pulstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProceedOnNewConfiguration<Identity: IViewHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewHelper_Impl::GetProceedOnNewConfiguration(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Identity, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, OFFSET>,
            GetProceedOnNewConfiguration: GetProceedOnNewConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewHelper as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IViewHelper {}
pub const JOIN_BEVEL: i32 = 1i32;
pub const JOIN_MITER: i32 = 2i32;
pub const JOIN_ROUND: i32 = 0i32;
pub const LA_ALTERNATE: u32 = 2u32;
pub const LA_GEOMETRIC: u32 = 1u32;
pub const LA_STARTGAP: u32 = 4u32;
pub const LA_STYLED: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LIGATURE {
    pub culSize: u32,
    pub pwsz: windows_core::PWSTR,
    pub chglyph: u32,
    pub ahglyph: [u32; 1],
}
impl Default for LIGATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct LINEATTRS {
    pub fl: u32,
    pub iJoin: u32,
    pub iEndCap: u32,
    pub elWidth: FLOAT_LONG,
    pub eMiterLimit: u32,
    pub cstyle: u32,
    pub pstyle: *mut FLOAT_LONG,
    pub elStyleState: FLOAT_LONG,
}
#[cfg(target_arch = "x86")]
impl Default for LINEATTRS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct LINEATTRS {
    pub fl: u32,
    pub iJoin: u32,
    pub iEndCap: u32,
    pub elWidth: FLOAT_LONG,
    pub eMiterLimit: f32,
    pub cstyle: u32,
    pub pstyle: *mut FLOAT_LONG,
    pub elStyleState: FLOAT_LONG,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for LINEATTRS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXCHARSETS: u32 = 16u32;
pub const MAX_PACKET_COUNT: u32 = 128u32;
pub const MC_APERTURE_GRILL_CATHODE_RAY_TUBE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(1i32);
pub const MC_BLUE_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(2i32);
pub const MC_BLUE_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(2i32);
pub const MC_CAPS_BRIGHTNESS: u32 = 2u32;
pub const MC_CAPS_COLOR_TEMPERATURE: u32 = 8u32;
pub const MC_CAPS_CONTRAST: u32 = 4u32;
pub const MC_CAPS_DEGAUSS: u32 = 64u32;
pub const MC_CAPS_DISPLAY_AREA_POSITION: u32 = 128u32;
pub const MC_CAPS_DISPLAY_AREA_SIZE: u32 = 256u32;
pub const MC_CAPS_MONITOR_TECHNOLOGY_TYPE: u32 = 1u32;
pub const MC_CAPS_NONE: u32 = 0u32;
pub const MC_CAPS_RED_GREEN_BLUE_DRIVE: u32 = 32u32;
pub const MC_CAPS_RED_GREEN_BLUE_GAIN: u32 = 16u32;
pub const MC_CAPS_RESTORE_FACTORY_COLOR_DEFAULTS: u32 = 2048u32;
pub const MC_CAPS_RESTORE_FACTORY_DEFAULTS: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_COLOR_TEMPERATURE(pub i32);
pub const MC_COLOR_TEMPERATURE_10000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(7i32);
pub const MC_COLOR_TEMPERATURE_11500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(8i32);
pub const MC_COLOR_TEMPERATURE_4000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(1i32);
pub const MC_COLOR_TEMPERATURE_5000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(2i32);
pub const MC_COLOR_TEMPERATURE_6500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(3i32);
pub const MC_COLOR_TEMPERATURE_7500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(4i32);
pub const MC_COLOR_TEMPERATURE_8200K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(5i32);
pub const MC_COLOR_TEMPERATURE_9300K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(6i32);
pub const MC_COLOR_TEMPERATURE_UNKNOWN: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_DISPLAY_TECHNOLOGY_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_DRIVE_TYPE(pub i32);
pub const MC_ELECTROLUMINESCENT: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(6i32);
pub const MC_FIELD_EMISSION_DEVICE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_GAIN_TYPE(pub i32);
pub const MC_GREEN_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(1i32);
pub const MC_GREEN_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(1i32);
pub const MC_HEIGHT: MC_SIZE_TYPE = MC_SIZE_TYPE(1i32);
pub const MC_HORIZONTAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(0i32);
pub const MC_LIQUID_CRYSTAL_ON_SILICON: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(3i32);
pub const MC_MICROELECTROMECHANICAL: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(7i32);
pub const MC_MOMENTARY: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(0i32);
pub const MC_ORGANIC_LIGHT_EMITTING_DIODE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(5i32);
pub const MC_PLASMA: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_POSITION_TYPE(pub i32);
pub const MC_RED_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(0i32);
pub const MC_RED_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(0i32);
pub const MC_RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS: u32 = 4096u32;
pub const MC_SET_PARAMETER: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(1i32);
pub const MC_SHADOW_MASK_CATHODE_RAY_TUBE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_SIZE_TYPE(pub i32);
pub const MC_SUPPORTED_COLOR_TEMPERATURE_10000K: u32 = 64u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_11500K: u32 = 128u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_4000K: u32 = 1u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_5000K: u32 = 2u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_6500K: u32 = 4u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_7500K: u32 = 8u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_8200K: u32 = 16u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_9300K: u32 = 32u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_NONE: u32 = 0u32;
pub const MC_THIN_FILM_TRANSISTOR: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(2i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MC_TIMING_REPORT {
    pub dwHorizontalFrequencyInHZ: u32,
    pub dwVerticalFrequencyInHZ: u32,
    pub bTimingStatusByte: u8,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MC_VCP_CODE_TYPE(pub i32);
pub const MC_VERTICAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(1i32);
pub const MC_WIDTH: MC_SIZE_TYPE = MC_SIZE_TYPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIPI_DSI_CAPS {
    pub DSITypeMajor: u8,
    pub DSITypeMinor: u8,
    pub SpecVersionMajor: u8,
    pub SpecVersionMinor: u8,
    pub SpecVersionPatch: u8,
    pub TargetMaximumReturnPacketSize: u16,
    pub ResultCodeFlags: u8,
    pub ResultCodeStatus: u8,
    pub Revision: u8,
    pub Level: u8,
    pub DeviceClassHi: u8,
    pub DeviceClassLo: u8,
    pub ManufacturerHi: u8,
    pub ManufacturerLo: u8,
    pub ProductHi: u8,
    pub ProductLo: u8,
    pub LengthHi: u8,
    pub LengthLo: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIPI_DSI_PACKET {
    pub Anonymous1: MIPI_DSI_PACKET_0,
    pub Anonymous2: MIPI_DSI_PACKET_1,
    pub EccFiller: u8,
    pub Payload: [u8; 8],
}
impl Default for MIPI_DSI_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIPI_DSI_PACKET_0 {
    pub DataId: u8,
    pub Anonymous: MIPI_DSI_PACKET_0_0,
}
impl Default for MIPI_DSI_PACKET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIPI_DSI_PACKET_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIPI_DSI_PACKET_1 {
    pub Anonymous: MIPI_DSI_PACKET_1_0,
    pub LongWriteWordCount: u16,
}
impl Default for MIPI_DSI_PACKET_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIPI_DSI_PACKET_1_0 {
    pub Data0: u8,
    pub Data1: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIPI_DSI_RESET {
    pub Flags: u32,
    pub Anonymous: MIPI_DSI_RESET_0,
}
impl Default for MIPI_DSI_RESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIPI_DSI_RESET_0 {
    pub Anonymous: MIPI_DSI_RESET_0_0,
    pub Results: u32,
}
impl Default for MIPI_DSI_RESET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIPI_DSI_RESET_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIPI_DSI_TRANSMISSION {
    pub TotalBufferSize: u32,
    pub PacketCount: u8,
    pub FailedPacket: u8,
    pub Anonymous: MIPI_DSI_TRANSMISSION_0,
    pub ReadWordCount: u16,
    pub FinalCommandExtraPayload: u16,
    pub MipiErrors: u16,
    pub HostErrors: u16,
    pub Packets: [MIPI_DSI_PACKET; 1],
}
impl Default for MIPI_DSI_TRANSMISSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIPI_DSI_TRANSMISSION_0 {
    pub _bitfield: u16,
}
pub const MS_CDDDEVICEBITMAP: u32 = 4u32;
pub const MS_NOTSYSTEMMEMORY: u32 = 1u32;
pub const MS_REUSEDDEVICEBITMAP: u32 = 8u32;
pub const MS_SHAREDACCESS: u32 = 2u32;
pub const NumVideoBankTypes: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(4i32);
pub const OC_BANK_CLIP: u32 = 1u32;
pub const OPENGL_CMD: u32 = 4352u32;
pub const OPENGL_GETINFO: u32 = 4353u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ORIENTATION_PREFERENCE(pub i32);
impl ORIENTATION_PREFERENCE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ORIENTATION_PREFERENCE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ORIENTATION_PREFERENCE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ORIENTATION_PREFERENCE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ORIENTATION_PREFERENCE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ORIENTATION_PREFERENCE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ORIENTATION_PREFERENCE_LANDSCAPE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(1i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(4i32);
pub const ORIENTATION_PREFERENCE_NONE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(0i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(2i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OUTPUT_COLOR_ENCODING(pub i32);
pub const OUTPUT_COLOR_ENCODING_INTENSITY: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(4i32);
pub const OUTPUT_COLOR_ENCODING_RGB: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(0i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR420: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(3i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR422: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(2i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR444: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(1i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(12i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_DVLL: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(33i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_HDR10PLUS: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(32i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(31i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(0i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709_WCG: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(30i32);
pub const OUTPUT_WIRE_COLOR_SPACE_RESERVED: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OUTPUT_WIRE_COLOR_SPACE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OUTPUT_WIRE_FORMAT {
    pub ColorEncoding: OUTPUT_COLOR_ENCODING,
    pub BitsPerPixel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PALOBJ {
    pub ulReserved: u32,
}
pub const PAL_BGR: u32 = 8u32;
pub const PAL_BITFIELDS: u32 = 2u32;
pub const PAL_CMYK: u32 = 16u32;
pub const PAL_INDEXED: u32 = 1u32;
pub const PAL_RGB: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA {
    pub Anonymous: PANEL_BRIGHTNESS_SENSOR_DATA_0,
    pub AlsReading: f32,
    pub ChromaticityCoordinate: CHROMATICITY_COORDINATE,
    pub ColorTemperature: f32,
}
impl Default for PANEL_BRIGHTNESS_SENSOR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    pub Anonymous: PANEL_BRIGHTNESS_SENSOR_DATA_0_0,
    pub Value: u32,
}
impl Default for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANEL_GET_BACKLIGHT_REDUCTION {
    pub BacklightUsersetting: u16,
    pub BacklightEffective: u16,
    pub GammaRamp: BACKLIGHT_REDUCTION_GAMMA_RAMP,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_GET_BRIGHTNESS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_GET_BRIGHTNESS_0,
}
impl Default for PANEL_GET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PANEL_GET_BRIGHTNESS_0 {
    pub Level: u8,
    pub Anonymous: PANEL_GET_BRIGHTNESS_0_0,
}
impl Default for PANEL_GET_BRIGHTNESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANEL_GET_BRIGHTNESS_0_0 {
    pub CurrentInMillinits: u32,
    pub TargetInMillinits: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_CAPS_0,
}
impl Default for PANEL_QUERY_BRIGHTNESS_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_CAPS_0_0,
    pub Value: u32,
}
impl Default for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_QUERY_BRIGHTNESS_RANGES {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_RANGES_0,
}
impl Default for PANEL_QUERY_BRIGHTNESS_RANGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    pub BrightnessLevel: BRIGHTNESS_LEVEL,
    pub NitRanges: BRIGHTNESS_NIT_RANGES,
}
impl Default for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANEL_SET_BACKLIGHT_OPTIMIZATION {
    pub Level: BACKLIGHT_OPTIMIZATION_LEVEL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_SET_BRIGHTNESS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_SET_BRIGHTNESS_0,
}
impl Default for PANEL_SET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PANEL_SET_BRIGHTNESS_0 {
    pub Level: u8,
    pub Anonymous: PANEL_SET_BRIGHTNESS_0_0,
}
impl Default for PANEL_SET_BRIGHTNESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_SET_BRIGHTNESS_0_0 {
    pub Millinits: u32,
    pub TransitionTimeInMs: u32,
    pub SensorData: PANEL_BRIGHTNESS_SENSOR_DATA,
}
impl Default for PANEL_SET_BRIGHTNESS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PANEL_SET_BRIGHTNESS_STATE {
    pub Anonymous: PANEL_SET_BRIGHTNESS_STATE_0,
}
impl Default for PANEL_SET_BRIGHTNESS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PANEL_SET_BRIGHTNESS_STATE_0 {
    pub Anonymous: PANEL_SET_BRIGHTNESS_STATE_0_0,
    pub Value: u32,
}
impl Default for PANEL_SET_BRIGHTNESS_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PANEL_SET_BRIGHTNESS_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATHDATA {
    pub flags: u32,
    pub count: u32,
    pub pptfx: *mut POINTFIX,
}
impl Default for PATHDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATHOBJ {
    pub fl: u32,
    pub cCurves: u32,
}
pub const PD_BEGINSUBPATH: u32 = 1u32;
pub const PD_BEZIERS: u32 = 16u32;
pub const PD_CLOSEFIGURE: u32 = 8u32;
pub const PD_ENDSUBPATH: u32 = 2u32;
pub const PD_RESETSTYLE: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERBANDINFO {
    pub bRepeatThisBand: windows_core::BOOL,
    pub szlBand: super::super::Foundation::SIZE,
    pub ulHorzRes: u32,
    pub ulVertRes: u32,
}
pub type PFN = Option<unsafe extern "system" fn() -> isize>;
pub type PFN_DrvAccumulateD3DDirtyRect = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CDDDXGK_REDIRBITMAPPRESENTINFO) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvAlphaBlend = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: *mut BLENDOBJ) -> windows_core::BOOL>;
pub type PFN_DrvAssertMode = Option<unsafe extern "system" fn(param0: DHPDEV, param1: windows_core::BOOL) -> windows_core::BOOL>;
pub type PFN_DrvAssociateSharedSurface = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::HANDLE, param2: super::super::Foundation::HANDLE, param3: super::super::Foundation::SIZE) -> windows_core::BOOL>;
pub type PFN_DrvBitBlt = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Foundation::RECTL, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::POINTL, param8: *mut BRUSHOBJ, param9: *mut super::super::Foundation::POINTL, param10: u32) -> windows_core::BOOL>;
pub type PFN_DrvCompletePDEV = Option<unsafe extern "system" fn(param0: DHPDEV, param1: HDEV)>;
pub type PFN_DrvCopyBits = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::POINTL) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvCreateDeviceBitmap = Option<unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::SIZE, param2: u32) -> super::super::Graphics::Gdi::HBITMAP>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvCreateDeviceBitmapEx = Option<unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::SIZE, param2: u32, param3: u32, param4: DHSURF, param5: u32, param6: u32, param7: *mut super::super::Foundation::HANDLE) -> super::super::Graphics::Gdi::HBITMAP>;
pub type PFN_DrvDeleteDeviceBitmap = Option<unsafe extern "system" fn(param0: DHSURF)>;
pub type PFN_DrvDeleteDeviceBitmapEx = Option<unsafe extern "system" fn(param0: DHSURF)>;
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvDeriveSurface = Option<unsafe extern "system" fn(param0: *mut super::super::Graphics::DirectDraw::DD_DIRECTDRAW_GLOBAL, param1: *mut super::super::Graphics::DirectDraw::DD_SURFACE_LOCAL) -> super::super::Graphics::Gdi::HBITMAP>;
#[cfg(feature = "Win32_Graphics_OpenGL")]
pub type PFN_DrvDescribePixelFormat = Option<unsafe extern "system" fn(param0: DHPDEV, param1: i32, param2: u32, param3: *mut super::super::Graphics::OpenGL::PIXELFORMATDESCRIPTOR) -> i32>;
pub type PFN_DrvDestroyFont = Option<unsafe extern "system" fn(param0: *mut FONTOBJ)>;
pub type PFN_DrvDisableDirectDraw = Option<unsafe extern "system" fn(param0: DHPDEV)>;
pub type PFN_DrvDisableDriver = Option<unsafe extern "system" fn()>;
pub type PFN_DrvDisablePDEV = Option<unsafe extern "system" fn(param0: DHPDEV)>;
pub type PFN_DrvDisableSurface = Option<unsafe extern "system" fn(param0: DHPDEV)>;
pub type PFN_DrvDitherColor = Option<unsafe extern "system" fn(param0: DHPDEV, param1: u32, param2: u32, param3: *mut u32) -> u32>;
pub type PFN_DrvDrawEscape = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: *mut CLIPOBJ, param3: *mut super::super::Foundation::RECTL, param4: u32, param5: *mut core::ffi::c_void) -> u32>;
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvEnableDirectDraw = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Graphics::DirectDraw::DD_CALLBACKS, param2: *mut super::super::Graphics::DirectDraw::DD_SURFACECALLBACKS, param3: *mut super::super::Graphics::DirectDraw::DD_PALETTECALLBACKS) -> windows_core::BOOL>;
pub type PFN_DrvEnableDriver = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut DRVENABLEDATA) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvEnablePDEV = Option<unsafe extern "system" fn(param0: *mut super::super::Graphics::Gdi::DEVMODEW, param1: windows_core::PCWSTR, param2: u32, param3: *mut HSURF, param4: u32, param5: *mut GDIINFO, param6: u32, param7: *mut DEVINFO, param8: HDEV, param9: windows_core::PCWSTR, param10: super::super::Foundation::HANDLE) -> DHPDEV>;
pub type PFN_DrvEnableSurface = Option<unsafe extern "system" fn(param0: DHPDEV) -> HSURF>;
pub type PFN_DrvEndDoc = Option<unsafe extern "system" fn(pso: *mut SURFOBJ, fl: u32) -> windows_core::BOOL>;
pub type PFN_DrvEndDxInterop = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: windows_core::BOOL, param2: *mut windows_core::BOOL, kernelmodedevicehandle: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_DrvEscape = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: u32, param3: *mut core::ffi::c_void, param4: u32, param5: *mut core::ffi::c_void) -> u32>;
pub type PFN_DrvFillPath = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut BRUSHOBJ, param4: *mut super::super::Foundation::POINTL, param5: u32, param6: u32) -> windows_core::BOOL>;
pub type PFN_DrvFontManagement = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut FONTOBJ, param2: u32, param3: u32, param4: *mut core::ffi::c_void, param5: u32, param6: *mut core::ffi::c_void) -> u32>;
pub type PFN_DrvFree = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: usize)>;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PFN_DrvGetDirectDrawInfo = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Graphics::DirectDraw::DD_HALINFO, param2: *mut u32, param3: *mut super::super::Graphics::DirectDraw::VIDEOMEMORY, param4: *mut u32, param5: *mut u32) -> windows_core::BOOL>;
pub type PFN_DrvGetGlyphMode = Option<unsafe extern "system" fn(dhpdev: DHPDEV, pfo: *mut FONTOBJ) -> u32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvGetModes = Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut super::super::Graphics::Gdi::DEVMODEW) -> u32>;
pub type PFN_DrvGetTrueTypeFile = Option<unsafe extern "system" fn(param0: usize, param1: *mut u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvGradientFill = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut XLATEOBJ, param3: *mut super::super::Graphics::Gdi::TRIVERTEX, param4: u32, param5: *mut core::ffi::c_void, param6: u32, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::POINTL, param9: u32) -> windows_core::BOOL>;
pub type PFN_DrvIcmCheckBitmapBits = Option<unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE, param2: *mut SURFOBJ, param3: *mut u8) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_ColorSystem"))]
pub type PFN_DrvIcmCreateColorTransform = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::UI::ColorSystem::LOGCOLORSPACEW, param2: *mut core::ffi::c_void, param3: u32, param4: *mut core::ffi::c_void, param5: u32, param6: *mut core::ffi::c_void, param7: u32, param8: u32) -> super::super::Foundation::HANDLE>;
pub type PFN_DrvIcmDeleteColorTransform = Option<unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_DrvIcmSetDeviceGammaRamp = Option<unsafe extern "system" fn(param0: DHPDEV, param1: u32, param2: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_DrvLineTo = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut BRUSHOBJ, param3: i32, param4: i32, param5: i32, param6: i32, param7: *mut super::super::Foundation::RECTL, param8: u32) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvLoadFontFile = Option<unsafe extern "system" fn(param0: u32, param1: *mut usize, param2: *mut *mut core::ffi::c_void, param3: *mut u32, param4: *mut super::super::Graphics::Gdi::DESIGNVECTOR, param5: u32, param6: u32) -> usize>;
pub type PFN_DrvLockDisplayArea = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL)>;
pub type PFN_DrvMovePointer = Option<unsafe extern "system" fn(pso: *mut SURFOBJ, x: i32, y: i32, prcl: *mut super::super::Foundation::RECTL)>;
pub type PFN_DrvNextBand = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, ppointl: *mut super::super::Foundation::POINTL) -> windows_core::BOOL>;
pub type PFN_DrvNotify = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: *mut core::ffi::c_void)>;
pub type PFN_DrvPaint = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut BRUSHOBJ, param3: *mut super::super::Foundation::POINTL, param4: u32) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvPlgBlt = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut POINTFIX, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32) -> windows_core::BOOL>;
pub type PFN_DrvQueryAdvanceWidths = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: *mut u32, param4: *mut core::ffi::c_void, param5: u32) -> windows_core::BOOL>;
pub type PFN_DrvQueryDeviceSupport = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut XLATEOBJ, param2: *mut XFORMOBJ, param3: u32, param4: u32, param5: *mut core::ffi::c_void, param6: u32, param7: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvQueryFont = Option<unsafe extern "system" fn(param0: DHPDEV, param1: usize, param2: u32, param3: *mut usize) -> *mut IFIMETRICS>;
pub type PFN_DrvQueryFontCaps = Option<unsafe extern "system" fn(param0: u32, param1: *mut u32) -> i32>;
pub type PFN_DrvQueryFontData = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: u32, param4: *mut GLYPHDATA, param5: *mut core::ffi::c_void, param6: u32) -> i32>;
pub type PFN_DrvQueryFontFile = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: u32, param3: *mut u32) -> i32>;
pub type PFN_DrvQueryFontTree = Option<unsafe extern "system" fn(param0: DHPDEV, param1: usize, param2: u32, param3: u32, param4: *mut usize) -> *mut core::ffi::c_void>;
pub type PFN_DrvQueryGlyphAttrs = Option<unsafe extern "system" fn(param0: *mut FONTOBJ, param1: u32) -> *mut FD_GLYPHATTR>;
pub type PFN_DrvQueryPerBandInfo = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PERBANDINFO) -> windows_core::BOOL>;
pub type PFN_DrvQuerySpoolType = Option<unsafe extern "system" fn(dhpdev: DHPDEV, pwchtype: windows_core::PCWSTR) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvQueryTrueTypeOutline = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: windows_core::BOOL, param4: *mut GLYPHDATA, param5: u32, param6: *mut super::super::Graphics::Gdi::TTPOLYGONHEADER) -> i32>;
pub type PFN_DrvQueryTrueTypeSection = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: u32, param3: *mut super::super::Foundation::HANDLE, param4: *mut i32) -> i32>;
pub type PFN_DrvQueryTrueTypeTable = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: u32, param3: i32, param4: u32, param5: *mut u8, param6: *mut *mut u8, param7: *mut u32) -> i32>;
pub type PFN_DrvRealizeBrush = Option<unsafe extern "system" fn(param0: *mut BRUSHOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut SURFOBJ, param4: *mut XLATEOBJ, param5: u32) -> windows_core::BOOL>;
pub type PFN_DrvRenderHint = Option<unsafe extern "system" fn(dhpdev: DHPDEV, notifycode: u32, length: usize, data: *const core::ffi::c_void) -> i32>;
pub type PFN_DrvResetDevice = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut core::ffi::c_void) -> u32>;
pub type PFN_DrvResetPDEV = Option<unsafe extern "system" fn(dhpdevold: DHPDEV, dhpdevnew: DHPDEV) -> windows_core::BOOL>;
pub type PFN_DrvSaveScreenBits = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: usize, param3: *mut super::super::Foundation::RECTL) -> usize>;
pub type PFN_DrvSendPage = Option<unsafe extern "system" fn(param0: *mut SURFOBJ) -> windows_core::BOOL>;
pub type PFN_DrvSetPalette = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut PALOBJ, param2: u32, param3: u32, param4: u32) -> windows_core::BOOL>;
pub type PFN_DrvSetPixelFormat = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: i32, param2: super::super::Foundation::HWND) -> windows_core::BOOL>;
pub type PFN_DrvSetPointerShape = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut XLATEOBJ, param4: i32, param5: i32, param6: i32, param7: i32, param8: *mut super::super::Foundation::RECTL, param9: u32) -> u32>;
pub type PFN_DrvStartBanding = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, ppointl: *mut super::super::Foundation::POINTL) -> windows_core::BOOL>;
pub type PFN_DrvStartDoc = Option<unsafe extern "system" fn(pso: *mut SURFOBJ, pwszdocname: windows_core::PCWSTR, dwjobid: u32) -> windows_core::BOOL>;
pub type PFN_DrvStartDxInterop = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: windows_core::BOOL, kernelmodedevicehandle: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type PFN_DrvStartPage = Option<unsafe extern "system" fn(pso: *mut SURFOBJ) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvStretchBlt = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32) -> windows_core::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvStretchBltROP = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32, param11: *mut BRUSHOBJ, param12: u32) -> windows_core::BOOL>;
pub type PFN_DrvStrokeAndFillPath = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut XFORMOBJ, param4: *mut BRUSHOBJ, param5: *mut LINEATTRS, param6: *mut BRUSHOBJ, param7: *mut super::super::Foundation::POINTL, param8: u32, param9: u32) -> windows_core::BOOL>;
pub type PFN_DrvStrokePath = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut XFORMOBJ, param4: *mut BRUSHOBJ, param5: *mut super::super::Foundation::POINTL, param6: *mut LINEATTRS, param7: u32) -> windows_core::BOOL>;
pub type PFN_DrvSurfaceComplete = Option<unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE) -> windows_core::BOOL>;
pub type PFN_DrvSwapBuffers = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut WNDOBJ) -> windows_core::BOOL>;
pub type PFN_DrvSynchronize = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL)>;
pub type PFN_DrvSynchronizeRedirectionBitmaps = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut u64) -> super::super::Foundation::NTSTATUS>;
pub type PFN_DrvSynchronizeSurface = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut super::super::Foundation::RECTL, param2: u32)>;
pub type PFN_DrvTextOut = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut STROBJ, param2: *mut FONTOBJ, param3: *mut CLIPOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: *mut BRUSHOBJ, param7: *mut BRUSHOBJ, param8: *mut super::super::Foundation::POINTL, param9: u32) -> windows_core::BOOL>;
pub type PFN_DrvTransparentBlt = Option<unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: u32, param7: u32) -> windows_core::BOOL>;
pub type PFN_DrvUnloadFontFile = Option<unsafe extern "system" fn(param0: usize) -> windows_core::BOOL>;
pub type PFN_DrvUnlockDisplayArea = Option<unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL)>;
pub type PFN_EngCombineRgn = Option<unsafe extern "system" fn(hrgntrg: super::super::Foundation::HANDLE, hrgnsrc1: super::super::Foundation::HANDLE, hrgnsrc2: super::super::Foundation::HANDLE, imode: i32) -> i32>;
pub type PFN_EngCopyRgn = Option<unsafe extern "system" fn(hrgndst: super::super::Foundation::HANDLE, hrgnsrc: super::super::Foundation::HANDLE) -> i32>;
pub type PFN_EngCreateRectRgn = Option<unsafe extern "system" fn(left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::HANDLE>;
pub type PFN_EngDeleteRgn = Option<unsafe extern "system" fn(hrgn: super::super::Foundation::HANDLE)>;
pub type PFN_EngIntersectRgn = Option<unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32>;
pub type PFN_EngSubtractRgn = Option<unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32>;
pub type PFN_EngUnionRgn = Option<unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32>;
pub type PFN_EngXorRgn = Option<unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PHYSICAL_MONITOR {
    pub hPhysicalMonitor: super::super::Foundation::HANDLE,
    pub szPhysicalMonitorDescription: [u16; 128],
}
impl Default for PHYSICAL_MONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PHYSICAL_MONITOR_DESCRIPTION_SIZE: u32 = 128u32;
pub const PLANAR_HC: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTE {
    pub x: u32,
    pub y: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTE {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTFIX {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POINTQF {
    pub x: i64,
    pub y: i64,
}
pub const PO_ALL_INTEGERS: u32 = 4u32;
pub const PO_BEZIERS: u32 = 1u32;
pub const PO_ELLIPSE: u32 = 2u32;
pub const PO_ENUM_AS_INTEGERS: u32 = 8u32;
pub const PO_WIDENED: u32 = 16u32;
pub const PPC_BGR_ORDER_HORIZONTAL_STRIPES: u32 = 5u32;
pub const PPC_BGR_ORDER_VERTICAL_STRIPES: u32 = 3u32;
pub const PPC_DEFAULT: u32 = 0u32;
pub const PPC_RGB_ORDER_HORIZONTAL_STRIPES: u32 = 4u32;
pub const PPC_RGB_ORDER_VERTICAL_STRIPES: u32 = 2u32;
pub const PPC_UNDEFINED: u32 = 1u32;
pub const PPG_DEFAULT: u32 = 0u32;
pub const PPG_SRGB: u32 = 1u32;
pub const PRIMARY_ORDER_ABC: u32 = 0u32;
pub const PRIMARY_ORDER_ACB: u32 = 1u32;
pub const PRIMARY_ORDER_BAC: u32 = 2u32;
pub const PRIMARY_ORDER_BCA: u32 = 3u32;
pub const PRIMARY_ORDER_CAB: u32 = 5u32;
pub const PRIMARY_ORDER_CBA: u32 = 4u32;
pub type PVIDEO_WIN32K_CALLOUT = Option<unsafe extern "system" fn(params: *const core::ffi::c_void)>;
pub const QAW_GETEASYWIDTHS: u32 = 1u32;
pub const QAW_GETWIDTHS: u32 = 0u32;
pub const QC_1BIT: u32 = 2u32;
pub const QC_4BIT: u32 = 4u32;
pub const QC_OUTLINES: u32 = 1u32;
pub const QDA_ACCELERATION_LEVEL: ENG_DEVICE_ATTRIBUTE = ENG_DEVICE_ATTRIBUTE(1i32);
pub const QDA_RESERVED: ENG_DEVICE_ATTRIBUTE = ENG_DEVICE_ATTRIBUTE(0i32);
pub const QDC_ALL_PATHS: QUERY_DISPLAY_CONFIG_FLAGS = QUERY_DISPLAY_CONFIG_FLAGS(1u32);
pub const QDC_DATABASE_CURRENT: QUERY_DISPLAY_CONFIG_FLAGS = QUERY_DISPLAY_CONFIG_FLAGS(4u32);
pub const QDC_INCLUDE_HMD: QUERY_DISPLAY_CONFIG_FLAGS = QUERY_DISPLAY_CONFIG_FLAGS(32u32);
pub const QDC_ONLY_ACTIVE_PATHS: QUERY_DISPLAY_CONFIG_FLAGS = QUERY_DISPLAY_CONFIG_FLAGS(2u32);
pub const QDC_VIRTUAL_MODE_AWARE: QUERY_DISPLAY_CONFIG_FLAGS = QUERY_DISPLAY_CONFIG_FLAGS(16u32);
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: QUERY_DISPLAY_CONFIG_FLAGS = QUERY_DISPLAY_CONFIG_FLAGS(64u32);
pub const QDS_CHECKJPEGFORMAT: u32 = 0u32;
pub const QDS_CHECKPNGFORMAT: u32 = 1u32;
pub const QFD_GLYPHANDBITMAP: i32 = 1i32;
pub const QFD_GLYPHANDOUTLINE: i32 = 2i32;
pub const QFD_MAXEXTENTS: i32 = 3i32;
pub const QFD_TT_GLYPHANDBITMAP: i32 = 4i32;
pub const QFD_TT_GRAY1_BITMAP: i32 = 5i32;
pub const QFD_TT_GRAY2_BITMAP: i32 = 6i32;
pub const QFD_TT_GRAY4_BITMAP: i32 = 8i32;
pub const QFD_TT_GRAY8_BITMAP: i32 = 9i32;
pub const QFD_TT_MONO_BITMAP: i32 = 5i32;
pub const QFF_DESCRIPTION: i32 = 1i32;
pub const QFF_NUMFACES: i32 = 2i32;
pub const QFT_GLYPHSET: i32 = 3i32;
pub const QFT_KERNPAIRS: i32 = 2i32;
pub const QFT_LIGATURES: i32 = 1i32;
pub const QSA_3DNOW: u32 = 16384u32;
pub const QSA_MMX: u32 = 256u32;
pub const QSA_SSE: u32 = 8192u32;
pub const QSA_SSE1: u32 = 8192u32;
pub const QSA_SSE2: u32 = 65536u32;
pub const QSA_SSE3: u32 = 524288u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_DISPLAY_CONFIG_FLAGS(pub u32);
impl QUERY_DISPLAY_CONFIG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for QUERY_DISPLAY_CONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for QUERY_DISPLAY_CONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for QUERY_DISPLAY_CONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for QUERY_DISPLAY_CONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for QUERY_DISPLAY_CONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RB_DITHERCOLOR: i32 = -2147483648i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECTFX {
    pub xLeft: i32,
    pub yTop: i32,
    pub xRight: i32,
    pub yBottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RUN {
    pub iStart: i32,
    pub iStop: i32,
}
pub const SDC_ALLOW_CHANGES: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(1024u32);
pub const SDC_ALLOW_PATH_ORDER_CHANGES: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(8192u32);
pub const SDC_APPLY: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(128u32);
pub const SDC_FORCE_MODE_ENUMERATION: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(4096u32);
pub const SDC_NO_OPTIMIZATION: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(256u32);
pub const SDC_PATH_PERSIST_IF_REQUIRED: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(2048u32);
pub const SDC_SAVE_TO_DATABASE: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(512u32);
pub const SDC_TOPOLOGY_CLONE: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(2u32);
pub const SDC_TOPOLOGY_EXTEND: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(4u32);
pub const SDC_TOPOLOGY_EXTERNAL: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(8u32);
pub const SDC_TOPOLOGY_INTERNAL: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(1u32);
pub const SDC_TOPOLOGY_SUPPLIED: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(16u32);
pub const SDC_USE_DATABASE_CURRENT: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(15u32);
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(32u32);
pub const SDC_VALIDATE: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(64u32);
pub const SDC_VIRTUAL_MODE_AWARE: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(32768u32);
pub const SDC_VIRTUAL_REFRESH_RATE_AWARE: SET_DISPLAY_CONFIG_FLAGS = SET_DISPLAY_CONFIG_FLAGS(131072u32);
pub const SETCONFIGURATION_STATUS_ADDITIONAL: u32 = 1u32;
pub const SETCONFIGURATION_STATUS_APPLIED: u32 = 0u32;
pub const SETCONFIGURATION_STATUS_OVERRIDDEN: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SET_ACTIVE_COLOR_PROFILE_NAME {
    pub ColorProfileName: [u16; 1],
}
impl Default for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_DISPLAY_CONFIG_FLAGS(pub u32);
impl SET_DISPLAY_CONFIG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SET_DISPLAY_CONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SET_DISPLAY_CONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SET_DISPLAY_CONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SET_DISPLAY_CONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SET_DISPLAY_CONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SGI_EXTRASPACE: u32 = 0u32;
pub type SORTCOMP = Option<unsafe extern "system" fn(pv1: *const core::ffi::c_void, pv2: *const core::ffi::c_void) -> i32>;
pub const SO_BREAK_EXTRA: u32 = 4096u32;
pub const SO_CHARACTER_EXTRA: u32 = 2048u32;
pub const SO_CHAR_INC_EQUAL_BM_BASE: u32 = 32u32;
pub const SO_DO_NOT_SUBSTITUTE_DEVICE_FONT: u32 = 128u32;
pub const SO_DXDY: u32 = 1024u32;
pub const SO_ESC_NOT_ORIENT: u32 = 512u32;
pub const SO_FLAG_DEFAULT_PLACEMENT: u32 = 1u32;
pub const SO_GLYPHINDEX_TEXTOUT: u32 = 256u32;
pub const SO_HORIZONTAL: u32 = 2u32;
pub const SO_MAXEXT_EQUAL_BM_SIDE: u32 = 64u32;
pub const SO_REVERSED: u32 = 8u32;
pub const SO_VERTICAL: u32 = 4u32;
pub const SO_ZERO_BEARINGS: u32 = 16u32;
pub const SPS_ACCEPT_EXCLUDE: u32 = 3u32;
pub const SPS_ACCEPT_NOEXCLUDE: u32 = 2u32;
pub const SPS_ACCEPT_SYNCHRONOUS: u32 = 4u32;
pub const SPS_ALPHA: i32 = 16i32;
pub const SPS_ANIMATESTART: i32 = 4i32;
pub const SPS_ANIMATEUPDATE: i32 = 8i32;
pub const SPS_ASYNCCHANGE: i32 = 2i32;
pub const SPS_CHANGE: i32 = 1i32;
pub const SPS_DECLINE: u32 = 1u32;
pub const SPS_ERROR: u32 = 0u32;
pub const SPS_FLAGSMASK: i32 = 255i32;
pub const SPS_FREQMASK: i32 = 1044480i32;
pub const SPS_LENGTHMASK: i32 = 3840i32;
pub const SPS_RESERVED: i32 = 32i32;
pub const SPS_RESERVED1: i32 = 64i32;
pub const SS_FREE: u32 = 2u32;
pub const SS_RESTORE: u32 = 1u32;
pub const SS_SAVE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STROBJ {
    pub cGlyphs: u32,
    pub flAccel: u32,
    pub ulCharInc: u32,
    pub rclBkGround: super::super::Foundation::RECTL,
    pub pgp: *mut GLYPHPOS,
    pub pwszOrg: windows_core::PWSTR,
}
impl Default for STROBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STYPE_BITMAP: i32 = 0i32;
pub const STYPE_DEVBITMAP: i32 = 3i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SURFOBJ {
    pub dhsurf: DHSURF,
    pub hsurf: HSURF,
    pub dhpdev: DHPDEV,
    pub hdev: HDEV,
    pub sizlBitmap: super::super::Foundation::SIZE,
    pub cjBits: u32,
    pub pvBits: *mut core::ffi::c_void,
    pub pvScan0: *mut core::ffi::c_void,
    pub lDelta: i32,
    pub iUniq: u32,
    pub iBitmapFormat: u32,
    pub iType: u16,
    pub fjBitmap: u16,
}
impl Default for SURFOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const S_INIT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sources {
    pub sourceId: u32,
    pub numTargets: i32,
    pub aTargets: [u32; 1],
}
impl Default for Sources {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TC_PATHOBJ: u32 = 2u32;
pub const TC_RECTANGLES: u32 = 0u32;
pub const TTO_METRICS_ONLY: u32 = 1u32;
pub const TTO_QUBICS: u32 = 2u32;
pub const TTO_UNHINTED: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TYPE1_FONT {
    pub hPFM: super::super::Foundation::HANDLE,
    pub hPFB: super::super::Foundation::HANDLE,
    pub ulIdentifier: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VGA_CHAR {
    pub Char: i8,
    pub Attributes: i8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEOPARAMETERS {
    pub Guid: windows_core::GUID,
    pub dwOffset: u32,
    pub dwCommand: u32,
    pub dwFlags: u32,
    pub dwMode: u32,
    pub dwTVStandard: u32,
    pub dwAvailableModes: u32,
    pub dwAvailableTVStandard: u32,
    pub dwFlickerFilter: u32,
    pub dwOverScanX: u32,
    pub dwOverScanY: u32,
    pub dwMaxUnscaledX: u32,
    pub dwMaxUnscaledY: u32,
    pub dwPositionX: u32,
    pub dwPositionY: u32,
    pub dwBrightness: u32,
    pub dwContrast: u32,
    pub dwCPType: u32,
    pub dwCPCommand: u32,
    pub dwCPStandard: u32,
    pub dwCPKey: u32,
    pub bCP_APSTriggerBits: u32,
    pub bOEMCopyProtection: [u8; 256],
}
impl Default for VIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_BANK_SELECT {
    pub Length: u32,
    pub Size: u32,
    pub BankingFlags: u32,
    pub BankingType: u32,
    pub PlanarHCBankingType: u32,
    pub BitmapWidthInBytes: u32,
    pub BitmapSize: u32,
    pub Granularity: u32,
    pub PlanarHCGranularity: u32,
    pub CodeOffset: u32,
    pub PlanarHCBankCodeOffset: u32,
    pub PlanarHCEnableCodeOffset: u32,
    pub PlanarHCDisableCodeOffset: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIDEO_BANK_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_BRIGHTNESS_POLICY {
    pub DefaultToBiosPolicy: bool,
    pub LevelCount: u8,
    pub Level: [VIDEO_BRIGHTNESS_POLICY_0; 1],
}
impl Default for VIDEO_BRIGHTNESS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_BRIGHTNESS_POLICY_0 {
    pub BatteryLevel: u8,
    pub Brightness: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIDEO_CLUT {
    pub NumEntries: u16,
    pub FirstEntry: u16,
    pub LookupTable: [VIDEO_CLUT_0; 1],
}
impl Default for VIDEO_CLUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIDEO_CLUT_0 {
    pub RgbArray: VIDEO_CLUTDATA,
    pub RgbLong: u32,
}
impl Default for VIDEO_CLUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_CLUTDATA {
    pub Red: u8,
    pub Green: u8,
    pub Blue: u8,
    pub Unused: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_COLOR_CAPABILITIES {
    pub Length: u32,
    pub AttributeFlags: u32,
    pub RedPhosphoreDecay: i32,
    pub GreenPhosphoreDecay: i32,
    pub BluePhosphoreDecay: i32,
    pub WhiteChromaticity_x: i32,
    pub WhiteChromaticity_y: i32,
    pub WhiteChromaticity_Y: i32,
    pub RedChromaticity_x: i32,
    pub RedChromaticity_y: i32,
    pub GreenChromaticity_x: i32,
    pub GreenChromaticity_y: i32,
    pub BlueChromaticity_x: i32,
    pub BlueChromaticity_y: i32,
    pub WhiteGamma: i32,
    pub RedGamma: i32,
    pub GreenGamma: i32,
    pub BlueGamma: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_COLOR_LUT_DATA {
    pub Length: u32,
    pub LutDataFormat: u32,
    pub LutData: [u8; 1],
}
impl Default for VIDEO_COLOR_LUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VIDEO_COLOR_LUT_DATA_FORMAT_PRIVATEFORMAT: u32 = 2147483648u32;
pub const VIDEO_COLOR_LUT_DATA_FORMAT_RGB256WORDS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_CURSOR_ATTRIBUTES {
    pub Width: u16,
    pub Height: u16,
    pub Column: i16,
    pub Row: i16,
    pub Rate: u8,
    pub Enable: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_CURSOR_POSITION {
    pub Column: i16,
    pub Row: i16,
}
pub const VIDEO_DEVICE_COLOR: u32 = 1u32;
pub const VIDEO_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("DISPLAY%d");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_DEVICE_SESSION_STATUS {
    pub bEnable: u32,
    pub bSuccess: u32,
}
pub const VIDEO_DUALVIEW_PRIMARY: u32 = 2147483648u32;
pub const VIDEO_DUALVIEW_REMOVABLE: u32 = 1u32;
pub const VIDEO_DUALVIEW_SECONDARY: u32 = 1073741824u32;
pub const VIDEO_DUALVIEW_WDDM_VGA: u32 = 536870912u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_HARDWARE_STATE {
    pub StateHeader: *mut VIDEO_HARDWARE_STATE_HEADER,
    pub StateLength: u32,
}
impl Default for VIDEO_HARDWARE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_HARDWARE_STATE_HEADER {
    pub Length: u32,
    pub PortValue: [u8; 48],
    pub AttribIndexDataState: u32,
    pub BasicSequencerOffset: u32,
    pub BasicCrtContOffset: u32,
    pub BasicGraphContOffset: u32,
    pub BasicAttribContOffset: u32,
    pub BasicDacOffset: u32,
    pub BasicLatchesOffset: u32,
    pub ExtendedSequencerOffset: u32,
    pub ExtendedCrtContOffset: u32,
    pub ExtendedGraphContOffset: u32,
    pub ExtendedAttribContOffset: u32,
    pub ExtendedDacOffset: u32,
    pub ExtendedValidatorStateOffset: u32,
    pub ExtendedMiscDataOffset: u32,
    pub PlaneLength: u32,
    pub Plane1Offset: u32,
    pub Plane2Offset: u32,
    pub Plane3Offset: u32,
    pub Plane4Offset: u32,
    pub VGAStateFlags: u32,
    pub DIBOffset: u32,
    pub DIBBitsPerPixel: u32,
    pub DIBXResolution: u32,
    pub DIBYResolution: u32,
    pub DIBXlatOffset: u32,
    pub DIBXlatLength: u32,
    pub VesaInfoOffset: u32,
    pub FrameBufferData: *mut core::ffi::c_void,
}
impl Default for VIDEO_HARDWARE_STATE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_LOAD_FONT_INFORMATION {
    pub WidthInPixels: u16,
    pub HeightInPixels: u16,
    pub FontSize: u32,
    pub Font: [u8; 1],
}
impl Default for VIDEO_LOAD_FONT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_LUT_RGB256WORDS {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl Default for VIDEO_LUT_RGB256WORDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VIDEO_MAX_REASON: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_MEMORY {
    pub RequestedVirtualAddress: *mut core::ffi::c_void,
}
impl Default for VIDEO_MEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_MEMORY_INFORMATION {
    pub VideoRamBase: *mut core::ffi::c_void,
    pub VideoRamLength: u32,
    pub FrameBufferBase: *mut core::ffi::c_void,
    pub FrameBufferLength: u32,
}
impl Default for VIDEO_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_MODE {
    pub RequestedMode: u32,
}
pub const VIDEO_MODE_ANIMATE_START: u32 = 8u32;
pub const VIDEO_MODE_ANIMATE_UPDATE: u32 = 16u32;
pub const VIDEO_MODE_ASYNC_POINTER: u32 = 1u32;
pub const VIDEO_MODE_BANKED: u32 = 128u32;
pub const VIDEO_MODE_COLOR: u32 = 1u32;
pub const VIDEO_MODE_COLOR_POINTER: u32 = 4u32;
pub const VIDEO_MODE_GRAPHICS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_MODE_INFORMATION {
    pub Length: u32,
    pub ModeIndex: u32,
    pub VisScreenWidth: u32,
    pub VisScreenHeight: u32,
    pub ScreenStride: u32,
    pub NumberOfPlanes: u32,
    pub BitsPerPlane: u32,
    pub Frequency: u32,
    pub XMillimeter: u32,
    pub YMillimeter: u32,
    pub NumberRedBits: u32,
    pub NumberGreenBits: u32,
    pub NumberBlueBits: u32,
    pub RedMask: u32,
    pub GreenMask: u32,
    pub BlueMask: u32,
    pub AttributeFlags: u32,
    pub VideoMemoryBitmapWidth: u32,
    pub VideoMemoryBitmapHeight: u32,
    pub DriverSpecificAttributeFlags: u32,
}
pub const VIDEO_MODE_INTERLACED: u32 = 16u32;
pub const VIDEO_MODE_LINEAR: u32 = 256u32;
pub const VIDEO_MODE_MANAGED_PALETTE: u32 = 8u32;
pub const VIDEO_MODE_MAP_MEM_LINEAR: u32 = 1073741824u32;
pub const VIDEO_MODE_MONO_POINTER: u32 = 2u32;
pub const VIDEO_MODE_NO_64_BIT_ACCESS: u32 = 64u32;
pub const VIDEO_MODE_NO_OFF_SCREEN: u32 = 32u32;
pub const VIDEO_MODE_NO_ZERO_MEMORY: u32 = 2147483648u32;
pub const VIDEO_MODE_PALETTE_DRIVEN: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_MONITOR_DESCRIPTOR {
    pub DescriptorSize: u32,
    pub Descriptor: [u8; 1],
}
impl Default for VIDEO_MONITOR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_NUM_MODES {
    pub NumModes: u32,
    pub ModeInformationLength: u32,
}
pub const VIDEO_OPTIONAL_GAMMET_TABLE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_PALETTE_DATA {
    pub NumEntries: u16,
    pub FirstEntry: u16,
    pub Colors: [u16; 1],
}
impl Default for VIDEO_PALETTE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_PERFORMANCE_COUNTER {
    pub NbOfAllocationEvicted: [u64; 10],
    pub NbOfAllocationMarked: [u64; 10],
    pub NbOfAllocationRestored: [u64; 10],
    pub KBytesEvicted: [u64; 10],
    pub KBytesMarked: [u64; 10],
    pub KBytesRestored: [u64; 10],
    pub NbProcessCommited: u64,
    pub NbAllocationCommited: u64,
    pub NbAllocationMarked: u64,
    pub KBytesAllocated: u64,
    pub KBytesAvailable: u64,
    pub KBytesCurMarked: u64,
    pub Reference: u64,
    pub Unreference: u64,
    pub TrueReference: u64,
    pub NbOfPageIn: u64,
    pub KBytesPageIn: u64,
    pub NbOfPageOut: u64,
    pub KBytesPageOut: u64,
    pub NbOfRotateOut: u64,
    pub KBytesRotateOut: u64,
}
impl Default for VIDEO_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_POINTER_ATTRIBUTES {
    pub Flags: u32,
    pub Width: u32,
    pub Height: u32,
    pub WidthInBytes: u32,
    pub Enable: u32,
    pub Column: i16,
    pub Row: i16,
    pub Pixels: [u8; 1],
}
impl Default for VIDEO_POINTER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_POINTER_CAPABILITIES {
    pub Flags: u32,
    pub MaxWidth: u32,
    pub MaxHeight: u32,
    pub HWPtrBitmapStart: u32,
    pub HWPtrBitmapEnd: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_POINTER_POSITION {
    pub Column: i16,
    pub Row: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_POWER_MANAGEMENT {
    pub Length: u32,
    pub DPMSVersion: u32,
    pub PowerState: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIDEO_POWER_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_PUBLIC_ACCESS_RANGES {
    pub InIoSpace: u32,
    pub MappedInIoSpace: u32,
    pub VirtualAddress: *mut core::ffi::c_void,
}
impl Default for VIDEO_PUBLIC_ACCESS_RANGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_QUERY_PERFORMANCE_COUNTER {
    pub BufferSize: u32,
    pub Buffer: *mut VIDEO_PERFORMANCE_COUNTER,
}
impl Default for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VIDEO_REASON_ALLOCATION: u32 = 6u32;
pub const VIDEO_REASON_CONFIGURATION: u32 = 9u32;
pub const VIDEO_REASON_FAILED_ROTATION: u32 = 5u32;
pub const VIDEO_REASON_LOCK: u32 = 5u32;
pub const VIDEO_REASON_NONE: u32 = 0u32;
pub const VIDEO_REASON_POLICY1: u32 = 1u32;
pub const VIDEO_REASON_POLICY2: u32 = 2u32;
pub const VIDEO_REASON_POLICY3: u32 = 3u32;
pub const VIDEO_REASON_POLICY4: u32 = 4u32;
pub const VIDEO_REASON_SCRATCH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_REGISTER_VDM {
    pub MinimumStateSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_SHARE_MEMORY {
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub ViewOffset: u32,
    pub ViewSize: u32,
    pub RequestedVirtualAddress: *mut core::ffi::c_void,
}
impl Default for VIDEO_SHARE_MEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_SHARE_MEMORY_INFORMATION {
    pub SharedViewOffset: u32,
    pub SharedViewSize: u32,
    pub VirtualAddress: *mut core::ffi::c_void,
}
impl Default for VIDEO_SHARE_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VIDEO_STATE_NON_STANDARD_VGA: u32 = 1u32;
pub const VIDEO_STATE_PACKED_CHAIN4_MODE: u32 = 4u32;
pub const VIDEO_STATE_UNEMULATED_VGA_STATE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIDEO_VDM {
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_WIN32K_CALLBACKS {
    pub PhysDisp: *mut core::ffi::c_void,
    pub Callout: PVIDEO_WIN32K_CALLOUT,
    pub bACPI: u32,
    pub pPhysDeviceObject: super::super::Foundation::HANDLE,
    pub DualviewFlags: u32,
}
impl Default for VIDEO_WIN32K_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS {
    pub CalloutType: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE,
    pub PhysDisp: *mut core::ffi::c_void,
    pub Param: usize,
    pub Status: i32,
    pub LockUserSession: bool,
    pub IsPostDevice: bool,
    pub SurpriseRemoval: bool,
    pub WaitForQueueReady: bool,
}
impl Default for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(pub i32);
pub const VideoBanked1R1W: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(2i32);
pub const VideoBanked1RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(1i32);
pub const VideoBanked2RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(3i32);
pub const VideoBlackScreenDiagnostics: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(16i32);
pub const VideoDesktopDuplicationChange: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(15i32);
pub const VideoDisableMultiPlaneOverlay: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(14i32);
pub const VideoDxgkDisplaySwitchCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(8i32);
pub const VideoDxgkFindAdapterTdrCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(10i32);
pub const VideoDxgkHardwareProtectionTeardown: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(11i32);
pub const VideoEnumChildPdoNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(3i32);
pub const VideoFindAdapterCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(4i32);
pub const VideoNotBanked: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(0i32);
pub const VideoPnpNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(7i32);
pub const VideoPowerHibernate: VIDEO_POWER_STATE = VIDEO_POWER_STATE(5i32);
pub const VideoPowerMaximum: VIDEO_POWER_STATE = VIDEO_POWER_STATE(7i32);
pub const VideoPowerNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(1i32);
pub const VideoPowerOff: VIDEO_POWER_STATE = VIDEO_POWER_STATE(4i32);
pub const VideoPowerOn: VIDEO_POWER_STATE = VIDEO_POWER_STATE(1i32);
pub const VideoPowerShutdown: VIDEO_POWER_STATE = VIDEO_POWER_STATE(6i32);
pub const VideoPowerStandBy: VIDEO_POWER_STATE = VIDEO_POWER_STATE(2i32);
pub const VideoPowerSuspend: VIDEO_POWER_STATE = VIDEO_POWER_STATE(3i32);
pub const VideoPowerUnspecified: VIDEO_POWER_STATE = VIDEO_POWER_STATE(0i32);
pub const VideoRepaintDesktop: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(12i32);
pub const VideoUpdateCursor: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(13i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCRUN {
    pub wcLow: u16,
    pub cGlyphs: u16,
    pub phg: *mut u32,
}
impl Default for WCRUN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINDDI_MAXSETPALETTECOLORINDEX: u32 = 255u32;
pub const WINDDI_MAXSETPALETTECOLORS: u32 = 256u32;
pub const WINDDI_MAX_BROADCAST_CONTEXT: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WNDOBJ {
    pub coClient: CLIPOBJ,
    pub pvConsumer: *mut core::ffi::c_void,
    pub rclClient: super::super::Foundation::RECTL,
    pub psoOwner: *mut SURFOBJ,
}
impl Default for WNDOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WNDOBJCHANGEPROC = Option<unsafe extern "system" fn(pwo: *mut WNDOBJ, fl: u32)>;
pub const WNDOBJ_SETUP: u32 = 4354u32;
pub const WOC_CHANGED: u32 = 16u32;
pub const WOC_DELETE: u32 = 32u32;
pub const WOC_DRAWN: u32 = 64u32;
pub const WOC_RGN_CLIENT: u32 = 2u32;
pub const WOC_RGN_CLIENT_DELTA: u32 = 1u32;
pub const WOC_RGN_SPRITE: u32 = 512u32;
pub const WOC_RGN_SURFACE: u32 = 8u32;
pub const WOC_RGN_SURFACE_DELTA: u32 = 4u32;
pub const WOC_SPRITE_NO_OVERLAP: u32 = 256u32;
pub const WOC_SPRITE_OVERLAP: u32 = 128u32;
pub const WO_DRAW_NOTIFY: u32 = 64u32;
pub const WO_RGN_CLIENT: u32 = 2u32;
pub const WO_RGN_CLIENT_DELTA: u32 = 1u32;
pub const WO_RGN_DESKTOP_COORD: u32 = 256u32;
pub const WO_RGN_SPRITE: u32 = 512u32;
pub const WO_RGN_SURFACE: u32 = 8u32;
pub const WO_RGN_SURFACE_DELTA: u32 = 4u32;
pub const WO_RGN_UPDATE_ALL: u32 = 16u32;
pub const WO_RGN_WINDOW: u32 = 32u32;
pub const WO_SPRITE_NOTIFY: u32 = 128u32;
pub const WVIDEO_DEVICE_NAME: windows_core::PCWSTR = windows_core::w!("DISPLAY%d");
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XFORML {
    pub eM11: u32,
    pub eM12: u32,
    pub eM21: u32,
    pub eM22: u32,
    pub eDx: u32,
    pub eDy: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XFORML {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XFORMOBJ {
    pub ulReserved: u32,
}
pub const XF_INV_FXTOL: i32 = 3i32;
pub const XF_INV_LTOL: i32 = 1i32;
pub const XF_LTOFX: i32 = 2i32;
pub const XF_LTOL: i32 = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XLATEOBJ {
    pub iUniq: u32,
    pub flXlate: u32,
    pub iSrcType: u16,
    pub iDstType: u16,
    pub cEntries: u32,
    pub pulXlate: *mut u32,
}
impl Default for XLATEOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XO_DESTBITFIELDS: u32 = 5u32;
pub const XO_DESTDCPALETTE: u32 = 3u32;
pub const XO_DESTPALETTE: u32 = 2u32;
pub const XO_DEVICE_ICM: u32 = 16u32;
pub const XO_FROM_CMYK: u32 = 8u32;
pub const XO_HOST_ICM: u32 = 32u32;
pub const XO_SRCBITFIELDS: u32 = 4u32;
pub const XO_SRCPALETTE: u32 = 1u32;
pub const XO_TABLE: u32 = 2u32;
pub const XO_TO_MONO: u32 = 4u32;
pub const XO_TRIVIAL: u32 = 1u32;
