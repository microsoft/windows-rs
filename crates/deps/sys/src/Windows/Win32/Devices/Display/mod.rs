#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_cEnumStart(pco: *mut CLIPOBJ, ball: super::super::Foundation::BOOL, itype: u32, idirection: u32, climit: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CapabilitiesRequestAndCapabilitiesReply(hmonitor: super::super::Foundation::HANDLE, pszasciicapabilitiesstring: super::super::Foundation::PSTR, dwcapabilitiesstringlengthincharacters: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DegaussMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPhysicalMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPhysicalMonitors(dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *const PHYSICAL_MONITOR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngAcquireSemaphore(hsem: HSEMAPHORE);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngAssociateSurface(hsurf: HSURF, hdev: HDEV, flhooks: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCheckAbort(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngCreateBitmap(sizl: super::super::Foundation::SIZE, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCreateClip() -> *mut CLIPOBJ;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngCreateDeviceBitmap(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCreateDeviceSurface(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> HSURF;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngCreateSemaphore() -> HSEMAPHORE;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngDeleteClip(pco: *const CLIPOBJ);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngDeletePalette(hpal: super::super::Graphics::Gdi::HPALETTE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngDeletePath(ppo: *mut PATHOBJ);
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngDeleteSemaphore(hsem: HSEMAPHORE);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngDeleteSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFindResource(h: super::super::Foundation::HANDLE, iname: i32, itype: i32, pulsize: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFreeModule(h: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngGetDriverName(hdev: HDEV) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngGetPrinterDataFileName(hdev: HDEV) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut ::core::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLoadModule(pwsz: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLockSurface(hsurf: HSURF) -> *mut SURFOBJ;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMarkBandingSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMultiByteToUnicodeN(unicodestring: super::super::Foundation::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: *mut u32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: u32);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMultiByteToWideChar(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngQueryEMFInfo(hdev: HDEV, pemfinfo: *mut EMFINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngQueryLocalTime(param0: *mut ENG_TIME_FIELDS);
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngReleaseSemaphore(hsem: HSEMAPHORE);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngUnicodeToMultiByteN(multibytestring: super::super::Foundation::PSTR, maxbytesinmultibytestring: u32, bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngUnlockSurface(pso: *mut SURFOBJ);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngWideCharToMultiByte(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut XFORMOBJ;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAutoRotationState(pstate: *mut AR_STATE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapabilitiesStringLength(hmonitor: super::super::Foundation::HANDLE, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDisplayAutoRotationPreferences(porientation: *mut ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorCapabilities(hmonitor: super::super::Foundation::HANDLE, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorTechnologyType(hmonitor: super::super::Foundation::HANDLE, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, pdwnumberofphysicalmonitors: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: ::windows::runtime::RawPtr, pdwnumberofphysicalmonitors: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: ::windows::runtime::RawPtr, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimingReport(hmonitor: super::super::Foundation::HANDLE, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVCPFeatureAndVCPFeatureReply(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, pvct: *mut MC_VCP_CODE_TYPE, pdwcurrentvalue: *mut u32, pdwmaximumvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HT_Get8BPPFormatPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn HT_Get8BPPMaskPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, use8bppmaskpal: super::super::Foundation::BOOL, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS);
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut RECTFX);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreMonitorFactoryColorDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreMonitorFactoryDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut POINTQF) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_vEnumStart(pstro: *mut STROBJ);
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaveCurrentMonitorSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaveCurrentSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDisplayAutoRotationPreferences(orientation: ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDisplayConfig(numpatharrayelements: u32, patharray: *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: u32, modeinfoarray: *const DISPLAYCONFIG_MODE_INFO, flags: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, dwnewbrightness: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, dwnewcontrast: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVCPFeature(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, dwnewvalue: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XFORMOBJ_bApplyXform(pxo: *mut XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut ::core::ffi::c_void, pvout: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XFORMOBJ_iGetXform(pxo: *const XFORMOBJ, pxform: *mut XFORML) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32;
}
