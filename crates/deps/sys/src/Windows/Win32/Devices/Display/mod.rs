#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE;
    pub fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut ::core::ffi::c_void;
    pub fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut ::core::ffi::c_void;
    pub fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_cEnumStart(pco: *mut CLIPOBJ, ball: super::super::Foundation::BOOL, itype: u32, idirection: u32, climit: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CapabilitiesRequestAndCapabilitiesReply(hmonitor: super::super::Foundation::HANDLE, pszasciicapabilitiesstring: super::super::Foundation::PSTR, dwcapabilitiesstringlengthincharacters: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DegaussMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPhysicalMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPhysicalMonitors(dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *const PHYSICAL_MONITOR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
    pub fn EngAcquireSemaphore(hsem: HSEMAPHORE);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngAssociateSurface(hsurf: HSURF, hdev: HDEV, flhooks: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCheckAbort(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL;
    pub fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngCreateBitmap(sizl: super::super::Foundation::SIZE, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCreateClip() -> *mut CLIPOBJ;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngCreateDeviceBitmap(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCreateDeviceSurface(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> HSURF;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE;
    pub fn EngCreateSemaphore() -> HSEMAPHORE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngDeleteClip(pco: *const CLIPOBJ);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngDeletePalette(hpal: super::super::Graphics::Gdi::HPALETTE) -> super::super::Foundation::BOOL;
    pub fn EngDeletePath(ppo: *mut PATHOBJ);
    pub fn EngDeleteSemaphore(hsem: HSEMAPHORE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngDeleteSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFindResource(h: super::super::Foundation::HANDLE, iname: i32, itype: i32, pulsize: *mut u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFreeModule(h: super::super::Foundation::HANDLE);
    pub fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngGetDriverName(hdev: HDEV) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngGetPrinterDataFileName(hdev: HDEV) -> super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut ::core::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLoadModule(pwsz: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLockSurface(hsurf: HSURF) -> *mut SURFOBJ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMarkBandingSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMultiByteToUnicodeN(unicodestring: super::super::Foundation::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: *mut u32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMultiByteToWideChar(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngQueryEMFInfo(hdev: HDEV, pemfinfo: *mut EMFINFO) -> super::super::Foundation::BOOL;
    pub fn EngQueryLocalTime(param0: *mut ENG_TIME_FIELDS);
    pub fn EngReleaseSemaphore(hsem: HSEMAPHORE);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngUnicodeToMultiByteN(multibytestring: super::super::Foundation::PSTR, maxbytesinmultibytestring: u32, bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngUnlockSurface(pso: *mut SURFOBJ);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngWideCharToMultiByte(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut XFORMOBJ;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAutoRotationState(pstate: *mut AR_STATE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapabilitiesStringLength(hmonitor: super::super::Foundation::HANDLE, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDisplayAutoRotationPreferences(porientation: *mut ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL;
    pub fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorCapabilities(hmonitor: super::super::Foundation::HANDLE, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorTechnologyType(hmonitor: super::super::Foundation::HANDLE, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, pdwnumberofphysicalmonitors: *mut u32) -> i32;
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: super::super::Graphics::Direct3D9::IDirect3DDevice9, pdwnumberofphysicalmonitors: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: super::super::Graphics::Direct3D9::IDirect3DDevice9, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimingReport(hmonitor: super::super::Foundation::HANDLE, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVCPFeatureAndVCPFeatureReply(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, pvct: *mut MC_VCP_CODE_TYPE, pdwcurrentvalue: *mut u32, pdwmaximumvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HT_Get8BPPFormatPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn HT_Get8BPPMaskPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, use8bppmaskpal: super::super::Foundation::BOOL, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> super::super::Foundation::BOOL;
    pub fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS);
    pub fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut RECTFX);
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreMonitorFactoryColorDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreMonitorFactoryDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut POINTQF) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_vEnumStart(pstro: *mut STROBJ);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaveCurrentMonitorSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaveCurrentSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDisplayAutoRotationPreferences(orientation: ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDisplayConfig(numpatharrayelements: u32, patharray: *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: u32, modeinfoarray: *const DISPLAYCONFIG_MODE_INFO, flags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, dwnewbrightness: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, dwnewcontrast: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVCPFeature(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, dwnewvalue: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn XFORMOBJ_bApplyXform(pxo: *mut XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut ::core::ffi::c_void, pvout: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn XFORMOBJ_iGetXform(pxo: *const XFORMOBJ, pxform: *mut XFORML) -> u32;
    pub fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE;
    pub fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32;
    pub fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32;
}
#[repr(transparent)]
pub struct AR_STATE(pub i32);
pub const AR_ENABLED: AR_STATE = AR_STATE(0i32);
pub const AR_DISABLED: AR_STATE = AR_STATE(1i32);
pub const AR_SUPPRESSED: AR_STATE = AR_STATE(2i32);
pub const AR_REMOTESESSION: AR_STATE = AR_STATE(4i32);
pub const AR_MULTIMON: AR_STATE = AR_STATE(8i32);
pub const AR_NOSENSOR: AR_STATE = AR_STATE(16i32);
pub const AR_NOT_SUPPORTED: AR_STATE = AR_STATE(32i32);
pub const AR_DOCKED: AR_STATE = AR_STATE(64i32);
pub const AR_LAPTOP: AR_STATE = AR_STATE(128i32);
#[repr(C)]
pub struct Adapter(i32);
#[repr(C)]
pub struct Adapters(i32);
#[repr(transparent)]
pub struct BACKLIGHT_OPTIMIZATION_LEVEL(pub i32);
pub const BacklightOptimizationDisable: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(0i32);
pub const BacklightOptimizationDesktop: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(1i32);
pub const BacklightOptimizationDynamic: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(2i32);
pub const BacklightOptimizationDimmed: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(3i32);
pub const BacklightOptimizationEDR: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(4i32);
#[repr(C)]
pub struct BACKLIGHT_REDUCTION_GAMMA_RAMP(i32);
#[repr(C)]
pub struct BANK_POSITION(i32);
pub const BITMAP_ARRAY_BYTE: u32 = 3u32;
pub const BITMAP_BITS_BYTE_ALIGN: u32 = 8u32;
pub const BITMAP_BITS_PIXEL: u32 = 1u32;
pub const BITMAP_BITS_WORD_ALIGN: u32 = 16u32;
pub const BITMAP_PLANES: u32 = 1u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct BLENDOBJ(i32);
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
pub struct BRIGHTNESS_INTERFACE_VERSION(pub i32);
pub const BRIGHTNESS_INTERFACE_VERSION_1: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(1i32);
pub const BRIGHTNESS_INTERFACE_VERSION_2: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(2i32);
pub const BRIGHTNESS_INTERFACE_VERSION_3: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(3i32);
#[repr(C)]
pub struct BRIGHTNESS_LEVEL(i32);
pub const BRIGHTNESS_MAX_LEVEL_COUNT: u32 = 103u32;
pub const BRIGHTNESS_MAX_NIT_RANGE_COUNT: u32 = 16u32;
#[repr(C)]
pub struct BRIGHTNESS_NIT_RANGE(i32);
#[repr(C)]
pub struct BRIGHTNESS_NIT_RANGES(i32);
#[repr(C)]
pub struct BRUSHOBJ(i32);
pub const BR_CMYKCOLOR: u32 = 4u32;
pub const BR_DEVICE_ICM: u32 = 1u32;
pub const BR_HOST_ICM: u32 = 2u32;
pub const BR_ORIGCOLOR: u32 = 8u32;
#[repr(transparent)]
pub struct BlackScreenDiagnosticsCalloutParam(pub i32);
pub const BlackScreenDiagnosticsData: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(1i32);
pub const BlackScreenDisplayRecovery: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(2i32);
pub const CDBEX_CROSSADAPTER: u32 = 8u32;
pub const CDBEX_DXINTEROP: u32 = 2u32;
pub const CDBEX_NTSHAREDSURFACEHANDLE: u32 = 4u32;
pub const CDBEX_REDIRECTION: u32 = 1u32;
pub const CDBEX_REUSE: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CDDDXGK_REDIRBITMAPPRESENTINFO(i32);
pub const CD_ANY: i32 = 4i32;
pub const CD_LEFTDOWN: i32 = 1i32;
pub const CD_LEFTUP: i32 = 3i32;
pub const CD_LEFTWARDS: i32 = 1i32;
pub const CD_RIGHTDOWN: i32 = 0i32;
pub const CD_RIGHTUP: i32 = 2i32;
pub const CD_UPWARDS: i32 = 2i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
#[repr(C)]
pub struct CHAR_IMAGE_INFO(i32);
pub const CHAR_TYPE_LEADING: u32 = 2u32;
pub const CHAR_TYPE_SBCS: u32 = 0u32;
pub const CHAR_TYPE_TRAILING: u32 = 3u32;
#[repr(C)]
pub struct CHROMATICITY_COORDINATE(i32);
#[repr(C)]
pub struct CIECHROMA(i32);
#[repr(C)]
pub struct CLIPLINE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CLIPOBJ(i32);
#[repr(C)]
pub struct COLORINFO(i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM(i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_1DLUT_CAP(i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_3x4(i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_DATA_CAP(i32);
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_DATA_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FIXED_POINT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FLOAT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(1i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP(i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_MATRIX_V2(i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_SET_INPUT(i32);
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_STAGE_CONTROL(pub i32);
pub const ColorSpaceTransformStageControl_No_Change: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(0i32);
pub const ColorSpaceTransformStageControl_Enable: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(1i32);
pub const ColorSpaceTransformStageControl_Bypass: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(2i32);
#[repr(C)]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS(i32);
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(pub i32);
pub const COLORSPACE_TRANSFORM_VERSION_DEFAULT: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
pub const COLORSPACE_TRANSFORM_VERSION_1: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(1i32);
pub const COLORSPACE_TRANSFORM_VERSION_NOT_SUPPORTED: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_TYPE_UNINITIALIZED: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_TYPE_DEFAULT: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(1i32);
pub const COLORSPACE_TRANSFORM_TYPE_RGB256x3x16: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(2i32);
pub const COLORSPACE_TRANSFORM_TYPE_DXGI_1: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(3i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_3x4: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(4i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_V2: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(5i32);
pub const CT_RECTANGLES: i32 = 0i32;
pub const DCR_DRIVER: u32 = 1u32;
pub const DCR_HALFTONE: u32 = 2u32;
pub const DCR_SOLID: u32 = 0u32;
pub const DC_COMPLEX: u32 = 3u32;
pub const DC_RECT: u32 = 1u32;
pub const DC_TRIVIAL: u32 = 0u32;
pub const DDI_DRIVER_VERSION_NT4: u32 = 131072u32;
pub const DDI_DRIVER_VERSION_NT5: u32 = 196608u32;
pub const DDI_DRIVER_VERSION_NT5_01: u32 = 196864u32;
pub const DDI_DRIVER_VERSION_NT5_01_SP1: u32 = 196865u32;
pub const DDI_DRIVER_VERSION_SP3: u32 = 131075u32;
pub const DDI_ERROR: u32 = 4294967295u32;
#[repr(C)]
pub struct DEVHTADJDATA(i32);
pub const DEVHTADJF_ADDITIVE_DEVICE: u32 = 2u32;
pub const DEVHTADJF_COLOR_DEVICE: u32 = 1u32;
#[repr(C)]
pub struct DEVHTINFO(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct DEVINFO(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_Device_ActivityId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3305783056,
        data2: 43612,
        data3: 16967,
        data4: [184, 48, 214, 166, 248, 234, 163, 16],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_Device_AdapterLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3305783056,
        data2: 43612,
        data3: 16967,
        data4: [184, 48, 214, 166, 248, 234, 163, 16],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_Device_TerminalLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3305783056,
        data2: 43612,
        data3: 16967,
        data4: [184, 48, 214, 166, 248, 234, 163, 16],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_IndirectDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 3305783056,
        data2: 43612,
        data3: 16967,
        data4: [184, 48, 214, 166, 248, 234, 163, 16],
    },
    pid: 1u32,
};
#[repr(C)]
pub struct DHPDEV(i32);
#[repr(C)]
pub struct DHSURF(i32);
#[repr(C)]
pub struct DISPLAYCONFIG_2DREGION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_ADAPTER_NAME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER(i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_DEVICE_INFO_TYPE(pub i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(3i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(4i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(5i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(6i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(7i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(8i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(9i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(10i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(11i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(12i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(13i32);
pub const DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(-1i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_MODE_INFO(i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_MODE_INFO_TYPE(pub i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_TARGET: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(3i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(-1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_PATH_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO(i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_PIXELFORMAT(pub i32);
pub const DISPLAYCONFIG_PIXELFORMAT_8BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(1i32);
pub const DISPLAYCONFIG_PIXELFORMAT_16BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(2i32);
pub const DISPLAYCONFIG_PIXELFORMAT_24BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(3i32);
pub const DISPLAYCONFIG_PIXELFORMAT_32BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(4i32);
pub const DISPLAYCONFIG_PIXELFORMAT_NONGDI: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(5i32);
pub const DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(-1i32);
#[repr(C)]
pub struct DISPLAYCONFIG_RATIONAL(i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_ROTATION(pub i32);
pub const DISPLAYCONFIG_ROTATION_IDENTITY: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(1i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE90: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(2i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE180: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(3i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE270: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(4i32);
pub const DISPLAYCONFIG_ROTATION_FORCE_UINT32: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(-1i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_SCALING(pub i32);
pub const DISPLAYCONFIG_SCALING_IDENTITY: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(1i32);
pub const DISPLAYCONFIG_SCALING_CENTERED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(2i32);
pub const DISPLAYCONFIG_SCALING_STRETCHED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(3i32);
pub const DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(4i32);
pub const DISPLAYCONFIG_SCALING_CUSTOM: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(5i32);
pub const DISPLAYCONFIG_SCALING_PREFERRED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(128i32);
pub const DISPLAYCONFIG_SCALING_FORCE_UINT32: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(-1i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_SCANLINE_ORDERING(pub i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(0i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(1i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(3i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(-1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SOURCE_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME(i32);
#[repr(C)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS(i32);
#[repr(C)]
pub struct DISPLAYCONFIG_TARGET_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE(i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_TOPOLOGY_ID(pub i32);
pub const DISPLAYCONFIG_TOPOLOGY_INTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(1i32);
pub const DISPLAYCONFIG_TOPOLOGY_CLONE: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(2i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTEND: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(4i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(8i32);
pub const DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(-1i32);
#[repr(transparent)]
pub struct DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(pub i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-1i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(0i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(1i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(2i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(3i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(4i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(5i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(6i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(8i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(9i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(10i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(11i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(12i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(13i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(14i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(15i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(16i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_VIRTUAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(17i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_USB_TUNNEL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(18i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-2147483648i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_FORCE_UINT32: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-1i32);
#[repr(C)]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO(i32);
pub const DISPLAYPOLICY_AC: u32 = 1u32;
pub const DISPLAYPOLICY_DC: u32 = 2u32;
#[repr(C)]
pub struct DISPLAY_BRIGHTNESS(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRH_APIBITMAPDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRIVEROBJ(i32);
#[repr(C)]
pub struct DRVENABLEDATA(i32);
#[repr(C)]
pub struct DRVFN(i32);
pub const DRVQUERY_USERMODE: u32 = 1u32;
pub const DSI_CHECKSUM_ERROR_CORRECTED: u32 = 256u32;
pub const DSI_CHECKSUM_ERROR_NOT_CORRECTED: u32 = 512u32;
pub const DSI_CONTENTION_DETECTED: u32 = 128u32;
#[repr(transparent)]
pub struct DSI_CONTROL_TRANSMISSION_MODE(pub i32);
pub const DCT_DEFAULT: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(0i32);
pub const DCT_FORCE_LOW_POWER: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(1i32);
pub const DCT_FORCE_HIGH_PERFORMANCE: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(2i32);
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
pub struct DXGK_WIN32K_PARAM_DATA(i32);
pub const DXGK_WIN32K_PARAM_FLAG_DISABLEVIEW: u32 = 4u32;
pub const DXGK_WIN32K_PARAM_FLAG_MODESWITCH: u32 = 2u32;
pub const DXGK_WIN32K_PARAM_FLAG_UPDATEREGISTRY: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DisplayMode(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct DisplayModes(i32);
pub const ECS_REDRAW: u32 = 2u32;
pub const ECS_TEARDOWN: u32 = 1u32;
pub const ED_ABORTDOC: u32 = 1u32;
pub const EHN_ERROR: u32 = 1u32;
pub const EHN_RESTORED: u32 = 0u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct EMFINFO(i32);
pub const ENDCAP_BUTT: i32 = 2i32;
pub const ENDCAP_ROUND: i32 = 0i32;
pub const ENDCAP_SQUARE: i32 = 1i32;
#[repr(C)]
pub struct ENGSAFESEMAPHORE(i32);
#[repr(transparent)]
pub struct ENG_DEVICE_ATTRIBUTE(pub i32);
pub const QDA_RESERVED: ENG_DEVICE_ATTRIBUTE = ENG_DEVICE_ATTRIBUTE(0i32);
pub const QDA_ACCELERATION_LEVEL: ENG_DEVICE_ATTRIBUTE = ENG_DEVICE_ATTRIBUTE(1i32);
#[repr(C)]
pub struct ENG_EVENT(i32);
pub const ENG_FNT_CACHE_READ_FAULT: u32 = 1u32;
pub const ENG_FNT_CACHE_WRITE_FAULT: u32 = 2u32;
#[repr(transparent)]
pub struct ENG_SYSTEM_ATTRIBUTE(pub i32);
pub const EngProcessorFeature: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(1i32);
pub const EngNumberOfProcessors: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(2i32);
pub const EngOptimumAvailableUserMemory: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(3i32);
pub const EngOptimumAvailableSystemMemory: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(4i32);
#[repr(C)]
pub struct ENG_TIME_FIELDS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ENUMRECTS(i32);
pub const FC_COMPLEX: u32 = 3u32;
pub const FC_RECT: u32 = 1u32;
pub const FC_RECT4: u32 = 2u32;
pub const FDM_TYPE_BM_SIDE_CONST: u32 = 1u32;
pub const FDM_TYPE_CHAR_INC_EQUAL_BM_BASE: u32 = 4u32;
pub const FDM_TYPE_CONST_BEARINGS: u32 = 16u32;
pub const FDM_TYPE_MAXEXT_EQUAL_BM_SIDE: u32 = 2u32;
pub const FDM_TYPE_ZERO_BEARINGS: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FD_DEVICEMETRICS(i32);
pub const FD_ERROR: u32 = 4294967295u32;
#[repr(C)]
pub struct FD_GLYPHATTR(i32);
#[repr(C)]
pub struct FD_GLYPHSET(i32);
#[repr(C)]
pub struct FD_KERNINGPAIR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FD_LIGATURE(i32);
pub const FD_NEGATIVE_FONT: i32 = 1i32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct FD_XFORM(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct FD_XFORM(i32);
pub const FF_IGNORED_SIGNATURE: u32 = 2u32;
pub const FF_SIGNATURE_VERIFIED: u32 = 1u32;
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct FLOATOBJ(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct FLOATOBJ_XFORM(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct FLOATOBJ_XFORM(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct FLOAT_LONG(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct FLOAT_LONG(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FONTDIFF(i32);
#[repr(C)]
pub struct FONTINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FONTOBJ(i32);
#[repr(C)]
pub struct FONTSIM(i32);
#[cfg(feature = "Win32_System_Console")]
#[repr(C)]
pub struct FONT_IMAGE_INFO(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub type FREEOBJPROC = unsafe extern "system" fn(pdriverobj: *mut DRIVEROBJ) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_System_Console")]
#[repr(C)]
pub struct FSCNTL_SCREEN_INFO(i32);
#[cfg(feature = "Win32_System_Console")]
#[repr(C)]
pub struct FSVIDEO_COPY_FRAME_BUFFER(i32);
#[repr(C)]
pub struct FSVIDEO_CURSOR_POSITION(i32);
#[repr(C)]
pub struct FSVIDEO_MODE_INFORMATION(i32);
#[cfg(feature = "Win32_System_Console")]
#[repr(C)]
pub struct FSVIDEO_REVERSE_MOUSE_POINTER(i32);
#[cfg(feature = "Win32_System_Console")]
#[repr(C)]
pub struct FSVIDEO_SCREEN_INFORMATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
#[repr(C)]
pub struct FSVIDEO_WRITE_TO_FRAME_BUFFER(i32);
#[repr(C)]
pub struct GAMMARAMP(i32);
#[repr(C)]
pub struct GAMMA_RAMP_DXGI_1(i32);
#[repr(C)]
pub struct GAMMA_RAMP_RGB(i32);
#[repr(C)]
pub struct GAMMA_RAMP_RGB256x3x16(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GDIINFO(i32);
pub const GDI_DRIVER_VERSION: u32 = 16384u32;
pub const GETCONNECTEDIDS_SOURCE: u32 = 1u32;
pub const GETCONNECTEDIDS_TARGET: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GLYPHBITS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GLYPHDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GLYPHDEF(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GLYPHPOS(i32);
pub const GS_16BIT_HANDLES: u32 = 4u32;
pub const GS_8BIT_HANDLES: u32 = 2u32;
pub const GS_UNICODE_HANDLES: u32 = 1u32;
pub const GUID_DEVINTERFACE_DISPLAY_ADAPTER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1531256861,
    data2: 62194,
    data3: 20283,
    data4: [133, 187, 48, 255, 31, 149, 53, 153],
};
pub const GUID_DEVINTERFACE_MONITOR: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3874519903,
    data2: 61079,
    data3: 19088,
    data4: [176, 118, 51, 245, 123, 244, 234, 167],
};
pub const GUID_DEVINTERFACE_VIDEO_OUTPUT_ARRIVAL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 450487536, data2: 63629, data3: 17248, data4: [186, 185, 76, 45, 85, 229, 100, 205] };
pub const GUID_DISPLAY_DEVICE_ARRIVAL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 480268672, data2: 42649, data3: 17674, data4: [154, 12, 222, 79, 190, 61, 221, 137] };
pub const GUID_MONITOR_OVERRIDE_PSEUDO_SPECIALIZED: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4053188655,
    data2: 63599,
    data3: 20378,
    data4: [170, 21, 233, 206, 189, 254, 59, 150],
};
pub const GX_GENERAL: i32 = 3i32;
pub const GX_IDENTITY: i32 = 0i32;
pub const GX_OFFSET: i32 = 1i32;
pub const GX_SCALE: i32 = 2i32;
#[repr(C)]
pub struct HBM(i32);
#[repr(C)]
pub struct HDEV(i32);
#[repr(C)]
pub struct HDRVOBJ(i32);
#[repr(C)]
pub struct HFASTMUTEX(i32);
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
#[repr(C)]
pub struct HSEMAPHORE(i32);
#[repr(C)]
pub struct HSURF(i32);
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
#[repr(transparent)]
pub struct ICloneViewHelper(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IFIEXTRA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct IFIMETRICS(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct IFIMETRICS(i32);
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INDIRECT_DISPLAY_INFO(i32);
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
#[repr(transparent)]
pub struct IViewHelper(pub *mut ::core::ffi::c_void);
pub const JOIN_BEVEL: i32 = 1i32;
pub const JOIN_MITER: i32 = 2i32;
pub const JOIN_ROUND: i32 = 0i32;
pub const LA_ALTERNATE: u32 = 2u32;
pub const LA_GEOMETRIC: u32 = 1u32;
pub const LA_STARTGAP: u32 = 4u32;
pub const LA_STYLED: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LIGATURE(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct LINEATTRS(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct LINEATTRS(i32);
pub const MAXCHARSETS: u32 = 16u32;
pub const MAX_PACKET_COUNT: u32 = 128u32;
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
pub struct MC_COLOR_TEMPERATURE(pub i32);
pub const MC_COLOR_TEMPERATURE_UNKNOWN: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(0i32);
pub const MC_COLOR_TEMPERATURE_4000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(1i32);
pub const MC_COLOR_TEMPERATURE_5000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(2i32);
pub const MC_COLOR_TEMPERATURE_6500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(3i32);
pub const MC_COLOR_TEMPERATURE_7500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(4i32);
pub const MC_COLOR_TEMPERATURE_8200K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(5i32);
pub const MC_COLOR_TEMPERATURE_9300K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(6i32);
pub const MC_COLOR_TEMPERATURE_10000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(7i32);
pub const MC_COLOR_TEMPERATURE_11500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(8i32);
#[repr(transparent)]
pub struct MC_DISPLAY_TECHNOLOGY_TYPE(pub i32);
pub const MC_SHADOW_MASK_CATHODE_RAY_TUBE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(0i32);
pub const MC_APERTURE_GRILL_CATHODE_RAY_TUBE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(1i32);
pub const MC_THIN_FILM_TRANSISTOR: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(2i32);
pub const MC_LIQUID_CRYSTAL_ON_SILICON: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(3i32);
pub const MC_PLASMA: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(4i32);
pub const MC_ORGANIC_LIGHT_EMITTING_DIODE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(5i32);
pub const MC_ELECTROLUMINESCENT: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(6i32);
pub const MC_MICROELECTROMECHANICAL: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(7i32);
pub const MC_FIELD_EMISSION_DEVICE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(8i32);
#[repr(transparent)]
pub struct MC_DRIVE_TYPE(pub i32);
pub const MC_RED_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(0i32);
pub const MC_GREEN_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(1i32);
pub const MC_BLUE_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(2i32);
#[repr(transparent)]
pub struct MC_GAIN_TYPE(pub i32);
pub const MC_RED_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(0i32);
pub const MC_GREEN_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(1i32);
pub const MC_BLUE_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(2i32);
#[repr(transparent)]
pub struct MC_POSITION_TYPE(pub i32);
pub const MC_HORIZONTAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(0i32);
pub const MC_VERTICAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(1i32);
pub const MC_RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS: u32 = 4096u32;
#[repr(transparent)]
pub struct MC_SIZE_TYPE(pub i32);
pub const MC_WIDTH: MC_SIZE_TYPE = MC_SIZE_TYPE(0i32);
pub const MC_HEIGHT: MC_SIZE_TYPE = MC_SIZE_TYPE(1i32);
pub const MC_SUPPORTED_COLOR_TEMPERATURE_10000K: u32 = 64u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_11500K: u32 = 128u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_4000K: u32 = 1u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_5000K: u32 = 2u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_6500K: u32 = 4u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_7500K: u32 = 8u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_8200K: u32 = 16u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_9300K: u32 = 32u32;
pub const MC_SUPPORTED_COLOR_TEMPERATURE_NONE: u32 = 0u32;
#[repr(C)]
pub struct MC_TIMING_REPORT(i32);
#[repr(transparent)]
pub struct MC_VCP_CODE_TYPE(pub i32);
pub const MC_MOMENTARY: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(0i32);
pub const MC_SET_PARAMETER: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(1i32);
#[repr(C)]
pub struct MIPI_DSI_CAPS(i32);
#[repr(C)]
pub struct MIPI_DSI_PACKET(i32);
#[repr(C)]
pub struct MIPI_DSI_RESET(i32);
#[repr(C)]
pub struct MIPI_DSI_TRANSMISSION(i32);
pub const MS_CDDDEVICEBITMAP: u32 = 4u32;
pub const MS_NOTSYSTEMMEMORY: u32 = 1u32;
pub const MS_REUSEDDEVICEBITMAP: u32 = 8u32;
pub const MS_SHAREDACCESS: u32 = 2u32;
pub const OC_BANK_CLIP: u32 = 1u32;
pub const OPENGL_CMD: u32 = 4352u32;
pub const OPENGL_GETINFO: u32 = 4353u32;
#[repr(transparent)]
pub struct ORIENTATION_PREFERENCE(pub i32);
pub const ORIENTATION_PREFERENCE_NONE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(0i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(1i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(2i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(4i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(8i32);
#[repr(transparent)]
pub struct OUTPUT_COLOR_ENCODING(pub i32);
pub const OUTPUT_COLOR_ENCODING_RGB: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(0i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR444: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(1i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR422: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(2i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR420: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(3i32);
pub const OUTPUT_COLOR_ENCODING_INTENSITY: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(4i32);
pub const OUTPUT_COLOR_ENCODING_FORCE_UINT32: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(-1i32);
#[repr(transparent)]
pub struct OUTPUT_WIRE_COLOR_SPACE_TYPE(pub i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(0i32);
pub const OUTPUT_WIRE_COLOR_SPACE_RESERVED: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(4i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(12i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709_WCG: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(30i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(31i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_HDR10PLUS: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(32i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_DVLL: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(33i32);
#[repr(C)]
pub struct OUTPUT_WIRE_FORMAT(i32);
#[repr(C)]
pub struct PALOBJ(i32);
pub const PAL_BGR: u32 = 8u32;
pub const PAL_BITFIELDS: u32 = 2u32;
pub const PAL_CMYK: u32 = 16u32;
pub const PAL_INDEXED: u32 = 1u32;
pub const PAL_RGB: u32 = 4u32;
#[repr(C)]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA(i32);
#[repr(C)]
pub struct PANEL_GET_BACKLIGHT_REDUCTION(i32);
#[repr(C)]
pub struct PANEL_GET_BRIGHTNESS(i32);
#[repr(C)]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS(i32);
#[repr(C)]
pub struct PANEL_QUERY_BRIGHTNESS_RANGES(i32);
#[repr(C)]
pub struct PANEL_SET_BACKLIGHT_OPTIMIZATION(i32);
#[repr(C)]
pub struct PANEL_SET_BRIGHTNESS(i32);
#[repr(C)]
pub struct PANEL_SET_BRIGHTNESS_STATE(i32);
#[repr(C)]
pub struct PATHDATA(i32);
#[repr(C)]
pub struct PATHOBJ(i32);
pub const PD_BEGINSUBPATH: u32 = 1u32;
pub const PD_BEZIERS: u32 = 16u32;
pub const PD_CLOSEFIGURE: u32 = 8u32;
pub const PD_ENDSUBPATH: u32 = 2u32;
pub const PD_RESETSTYLE: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PERBANDINFO(i32);
pub type PFN = unsafe extern "system" fn() -> isize;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvAccumulateD3DDirtyRect = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CDDDXGK_REDIRBITMAPPRESENTINFO) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvAlphaBlend = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: *mut BLENDOBJ) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvAssertMode = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvAssociateSharedSurface = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::HANDLE, param2: super::super::Foundation::HANDLE, param3: super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvBitBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Foundation::RECTL, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::POINTL, param8: *mut BRUSHOBJ, param9: *mut super::super::Foundation::POINTL, param10: u32) -> super::super::Foundation::BOOL;
pub type PFN_DrvCompletePDEV = unsafe extern "system" fn(param0: DHPDEV, param1: HDEV);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvCopyBits = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvCreateDeviceBitmap = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::SIZE, param2: u32) -> super::super::Graphics::Gdi::HBITMAP;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvCreateDeviceBitmapEx = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::SIZE, param2: u32, param3: u32, param4: DHSURF, param5: u32, param6: u32, param7: *mut super::super::Foundation::HANDLE) -> super::super::Graphics::Gdi::HBITMAP;
pub type PFN_DrvDeleteDeviceBitmap = unsafe extern "system" fn(param0: DHSURF);
pub type PFN_DrvDeleteDeviceBitmapEx = unsafe extern "system" fn(param0: DHSURF);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvDeriveSurface = unsafe extern "system" fn(param0: *mut super::super::Graphics::DirectDraw::DD_DIRECTDRAW_GLOBAL, param1: *mut super::super::Graphics::DirectDraw::DD_SURFACE_LOCAL) -> super::super::Graphics::Gdi::HBITMAP;
#[cfg(feature = "Win32_Graphics_OpenGL")]
pub type PFN_DrvDescribePixelFormat = unsafe extern "system" fn(param0: DHPDEV, param1: i32, param2: u32, param3: *mut super::super::Graphics::OpenGL::PIXELFORMATDESCRIPTOR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvDestroyFont = unsafe extern "system" fn(param0: *mut FONTOBJ);
pub type PFN_DrvDisableDirectDraw = unsafe extern "system" fn(param0: DHPDEV);
pub type PFN_DrvDisableDriver = unsafe extern "system" fn();
pub type PFN_DrvDisablePDEV = unsafe extern "system" fn(param0: DHPDEV);
pub type PFN_DrvDisableSurface = unsafe extern "system" fn(param0: DHPDEV);
pub type PFN_DrvDitherColor = unsafe extern "system" fn(param0: DHPDEV, param1: u32, param2: u32, param3: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvDrawEscape = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: *mut CLIPOBJ, param3: *mut super::super::Foundation::RECTL, param4: u32, param5: *mut ::core::ffi::c_void) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvEnableDirectDraw = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Graphics::DirectDraw::DD_CALLBACKS, param2: *mut super::super::Graphics::DirectDraw::DD_SURFACECALLBACKS, param3: *mut super::super::Graphics::DirectDraw::DD_PALETTECALLBACKS) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEnableDriver = unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut DRVENABLEDATA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvEnablePDEV = unsafe extern "system" fn(param0: *mut super::super::Graphics::Gdi::DEVMODEW, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut HSURF, param4: u32, param5: *mut GDIINFO, param6: u32, param7: *mut DEVINFO, param8: HDEV, param9: super::super::Foundation::PWSTR, param10: super::super::Foundation::HANDLE) -> DHPDEV;
pub type PFN_DrvEnableSurface = unsafe extern "system" fn(param0: DHPDEV) -> HSURF;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEndDoc = unsafe extern "system" fn(pso: *mut SURFOBJ, fl: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEndDxInterop = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::BOOL, param2: *mut super::super::Foundation::BOOL, kernelmodedevicehandle: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEscape = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: u32, param3: *mut ::core::ffi::c_void, param4: u32, param5: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvFillPath = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut BRUSHOBJ, param4: *mut super::super::Foundation::POINTL, param5: u32, param6: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvFontManagement = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut FONTOBJ, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: *mut ::core::ffi::c_void) -> u32;
pub type PFN_DrvFree = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: usize);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PFN_DrvGetDirectDrawInfo = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Graphics::DirectDraw::DD_HALINFO, param2: *mut u32, param3: *mut super::super::Graphics::DirectDraw::VIDEOMEMORY, param4: *mut u32, param5: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvGetGlyphMode = unsafe extern "system" fn(dhpdev: DHPDEV, pfo: *mut FONTOBJ) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvGetModes = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut super::super::Graphics::Gdi::DEVMODEW) -> u32;
pub type PFN_DrvGetTrueTypeFile = unsafe extern "system" fn(param0: usize, param1: *mut u32) -> *mut ::core::ffi::c_void;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvGradientFill = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut XLATEOBJ, param3: *mut super::super::Graphics::Gdi::TRIVERTEX, param4: u32, param5: *mut ::core::ffi::c_void, param6: u32, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::POINTL, param9: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvIcmCheckBitmapBits = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE, param2: *mut SURFOBJ, param3: *mut u8) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_ColorSystem"))]
pub type PFN_DrvIcmCreateColorTransform = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::UI::ColorSystem::LOGCOLORSPACEW, param2: *mut ::core::ffi::c_void, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: *mut ::core::ffi::c_void, param7: u32, param8: u32) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvIcmDeleteColorTransform = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvIcmSetDeviceGammaRamp = unsafe extern "system" fn(param0: DHPDEV, param1: u32, param2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvLineTo = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut BRUSHOBJ, param3: i32, param4: i32, param5: i32, param6: i32, param7: *mut super::super::Foundation::RECTL, param8: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvLoadFontFile = unsafe extern "system" fn(param0: u32, param1: *mut usize, param2: *mut *mut ::core::ffi::c_void, param3: *mut u32, param4: *mut super::super::Graphics::Gdi::DESIGNVECTOR, param5: u32, param6: u32) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvLockDisplayArea = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvMovePointer = unsafe extern "system" fn(pso: *mut SURFOBJ, x: i32, y: i32, prcl: *mut super::super::Foundation::RECTL);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvNextBand = unsafe extern "system" fn(param0: *mut SURFOBJ, ppointl: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvNotify = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvPaint = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut BRUSHOBJ, param3: *mut super::super::Foundation::POINTL, param4: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvPlgBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut POINTFIX, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryAdvanceWidths = unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: *mut u32, param4: *mut ::core::ffi::c_void, param5: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryDeviceSupport = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut XLATEOBJ, param2: *mut XFORMOBJ, param3: u32, param4: u32, param5: *mut ::core::ffi::c_void, param6: u32, param7: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvQueryFont = unsafe extern "system" fn(param0: DHPDEV, param1: usize, param2: u32, param3: *mut usize) -> *mut IFIMETRICS;
pub type PFN_DrvQueryFontCaps = unsafe extern "system" fn(param0: u32, param1: *mut u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryFontData = unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: u32, param4: *mut GLYPHDATA, param5: *mut ::core::ffi::c_void, param6: u32) -> i32;
pub type PFN_DrvQueryFontFile = unsafe extern "system" fn(param0: usize, param1: u32, param2: u32, param3: *mut u32) -> i32;
pub type PFN_DrvQueryFontTree = unsafe extern "system" fn(param0: DHPDEV, param1: usize, param2: u32, param3: u32, param4: *mut usize) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryGlyphAttrs = unsafe extern "system" fn(param0: *mut FONTOBJ, param1: u32) -> *mut FD_GLYPHATTR;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryPerBandInfo = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PERBANDINFO) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQuerySpoolType = unsafe extern "system" fn(dhpdev: DHPDEV, pwchtype: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvQueryTrueTypeOutline = unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: super::super::Foundation::BOOL, param4: *mut GLYPHDATA, param5: u32, param6: *mut super::super::Graphics::Gdi::TTPOLYGONHEADER) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryTrueTypeSection = unsafe extern "system" fn(param0: u32, param1: u32, param2: u32, param3: *mut super::super::Foundation::HANDLE, param4: *mut i32) -> i32;
pub type PFN_DrvQueryTrueTypeTable = unsafe extern "system" fn(param0: usize, param1: u32, param2: u32, param3: i32, param4: u32, param5: *mut u8, param6: *mut *mut u8, param7: *mut u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvRealizeBrush = unsafe extern "system" fn(param0: *mut BRUSHOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut SURFOBJ, param4: *mut XLATEOBJ, param5: u32) -> super::super::Foundation::BOOL;
pub type PFN_DrvRenderHint = unsafe extern "system" fn(dhpdev: DHPDEV, notifycode: u32, length: usize, data: *const ::core::ffi::c_void) -> i32;
pub type PFN_DrvResetDevice = unsafe extern "system" fn(param0: DHPDEV, param1: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvResetPDEV = unsafe extern "system" fn(dhpdevold: DHPDEV, dhpdevnew: DHPDEV) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSaveScreenBits = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: usize, param3: *mut super::super::Foundation::RECTL) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSendPage = unsafe extern "system" fn(param0: *mut SURFOBJ) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSetPalette = unsafe extern "system" fn(param0: DHPDEV, param1: *mut PALOBJ, param2: u32, param3: u32, param4: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSetPixelFormat = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: i32, param2: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSetPointerShape = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut XLATEOBJ, param4: i32, param5: i32, param6: i32, param7: i32, param8: *mut super::super::Foundation::RECTL, param9: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartBanding = unsafe extern "system" fn(param0: *mut SURFOBJ, ppointl: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartDoc = unsafe extern "system" fn(pso: *mut SURFOBJ, pwszdocname: super::super::Foundation::PWSTR, dwjobid: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartDxInterop = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::BOOL, kernelmodedevicehandle: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartPage = unsafe extern "system" fn(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvStretchBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvStretchBltROP = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32, param11: *mut BRUSHOBJ, param12: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStrokeAndFillPath = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut XFORMOBJ, param4: *mut BRUSHOBJ, param5: *mut LINEATTRS, param6: *mut BRUSHOBJ, param7: *mut super::super::Foundation::POINTL, param8: u32, param9: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStrokePath = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut XFORMOBJ, param4: *mut BRUSHOBJ, param5: *mut super::super::Foundation::POINTL, param6: *mut LINEATTRS, param7: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSurfaceComplete = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSwapBuffers = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut WNDOBJ) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSynchronize = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSynchronizeRedirectionBitmaps = unsafe extern "system" fn(param0: DHPDEV, param1: *mut u64) -> super::super::Foundation::NTSTATUS;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSynchronizeSurface = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut super::super::Foundation::RECTL, param2: u32);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvTextOut = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut STROBJ, param2: *mut FONTOBJ, param3: *mut CLIPOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: *mut BRUSHOBJ, param7: *mut BRUSHOBJ, param8: *mut super::super::Foundation::POINTL, param9: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvTransparentBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: u32, param7: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUnloadFontFile = unsafe extern "system" fn(param0: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUnlockDisplayArea = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngCombineRgn = unsafe extern "system" fn(hrgntrg: super::super::Foundation::HANDLE, hrgnsrc1: super::super::Foundation::HANDLE, hrgnsrc2: super::super::Foundation::HANDLE, imode: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngCopyRgn = unsafe extern "system" fn(hrgndst: super::super::Foundation::HANDLE, hrgnsrc: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngCreateRectRgn = unsafe extern "system" fn(left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngDeleteRgn = unsafe extern "system" fn(hrgn: super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngIntersectRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngSubtractRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngUnionRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngXorRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PHYSICAL_MONITOR(i32);
pub const PHYSICAL_MONITOR_DESCRIPTION_SIZE: u32 = 128u32;
pub const PLANAR_HC: u32 = 1u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct POINTE(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct POINTE(i32);
#[repr(C)]
pub struct POINTFIX(i32);
#[repr(C)]
pub struct POINTQF(i32);
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
pub type PVIDEO_WIN32K_CALLOUT = unsafe extern "system" fn(params: *mut ::core::ffi::c_void);
pub const QAW_GETEASYWIDTHS: u32 = 1u32;
pub const QAW_GETWIDTHS: u32 = 0u32;
pub const QC_1BIT: u32 = 2u32;
pub const QC_4BIT: u32 = 4u32;
pub const QC_OUTLINES: u32 = 1u32;
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
pub const RB_DITHERCOLOR: i32 = -2147483648i32;
#[repr(C)]
pub struct RECTFX(i32);
#[repr(C)]
pub struct RUN(i32);
pub const SETCONFIGURATION_STATUS_ADDITIONAL: u32 = 1u32;
pub const SETCONFIGURATION_STATUS_APPLIED: u32 = 0u32;
pub const SETCONFIGURATION_STATUS_OVERRIDDEN: u32 = 2u32;
#[repr(C)]
pub struct SET_ACTIVE_COLOR_PROFILE_NAME(i32);
pub const SGI_EXTRASPACE: u32 = 0u32;
pub type SORTCOMP = unsafe extern "system" fn(pv1: *const ::core::ffi::c_void, pv2: *const ::core::ffi::c_void) -> i32;
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
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STROBJ(i32);
pub const STYPE_BITMAP: i32 = 0i32;
pub const STYPE_DEVBITMAP: i32 = 3i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SURFOBJ(i32);
pub const S_INIT: u32 = 2u32;
#[repr(C)]
pub struct Sources(i32);
pub const TC_PATHOBJ: u32 = 2u32;
pub const TC_RECTANGLES: u32 = 0u32;
pub const TTO_METRICS_ONLY: u32 = 1u32;
pub const TTO_QUBICS: u32 = 2u32;
pub const TTO_UNHINTED: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TYPE1_FONT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VGA_CHAR(i32);
#[repr(C)]
pub struct VIDEOPARAMETERS(i32);
#[repr(C)]
pub struct VIDEO_BANK_SELECT(i32);
#[repr(transparent)]
pub struct VIDEO_BANK_TYPE(pub i32);
pub const VideoNotBanked: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(0i32);
pub const VideoBanked1RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(1i32);
pub const VideoBanked1R1W: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(2i32);
pub const VideoBanked2RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(3i32);
pub const NumVideoBankTypes: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEO_BRIGHTNESS_POLICY(i32);
#[repr(C)]
pub struct VIDEO_CLUT(i32);
#[repr(C)]
pub struct VIDEO_CLUTDATA(i32);
#[repr(C)]
pub struct VIDEO_COLOR_CAPABILITIES(i32);
#[repr(C)]
pub struct VIDEO_COLOR_LUT_DATA(i32);
pub const VIDEO_COLOR_LUT_DATA_FORMAT_PRIVATEFORMAT: u32 = 2147483648u32;
pub const VIDEO_COLOR_LUT_DATA_FORMAT_RGB256WORDS: u32 = 1u32;
#[repr(C)]
pub struct VIDEO_CURSOR_ATTRIBUTES(i32);
#[repr(C)]
pub struct VIDEO_CURSOR_POSITION(i32);
pub const VIDEO_DEVICE_COLOR: u32 = 1u32;
#[repr(C)]
pub struct VIDEO_DEVICE_SESSION_STATUS(i32);
pub const VIDEO_DUALVIEW_PRIMARY: u32 = 2147483648u32;
pub const VIDEO_DUALVIEW_REMOVABLE: u32 = 1u32;
pub const VIDEO_DUALVIEW_SECONDARY: u32 = 1073741824u32;
pub const VIDEO_DUALVIEW_WDDM_VGA: u32 = 536870912u32;
#[repr(C)]
pub struct VIDEO_HARDWARE_STATE(i32);
#[repr(C)]
pub struct VIDEO_HARDWARE_STATE_HEADER(i32);
#[repr(C)]
pub struct VIDEO_LOAD_FONT_INFORMATION(i32);
#[repr(C)]
pub struct VIDEO_LUT_RGB256WORDS(i32);
pub const VIDEO_MAX_REASON: u32 = 9u32;
#[repr(C)]
pub struct VIDEO_MEMORY(i32);
#[repr(C)]
pub struct VIDEO_MEMORY_INFORMATION(i32);
#[repr(C)]
pub struct VIDEO_MODE(i32);
pub const VIDEO_MODE_ANIMATE_START: u32 = 8u32;
pub const VIDEO_MODE_ANIMATE_UPDATE: u32 = 16u32;
pub const VIDEO_MODE_ASYNC_POINTER: u32 = 1u32;
pub const VIDEO_MODE_BANKED: u32 = 128u32;
pub const VIDEO_MODE_COLOR: u32 = 1u32;
pub const VIDEO_MODE_COLOR_POINTER: u32 = 4u32;
pub const VIDEO_MODE_GRAPHICS: u32 = 2u32;
#[repr(C)]
pub struct VIDEO_MODE_INFORMATION(i32);
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
pub struct VIDEO_MONITOR_DESCRIPTOR(i32);
#[repr(C)]
pub struct VIDEO_NUM_MODES(i32);
pub const VIDEO_OPTIONAL_GAMMET_TABLE: u32 = 2u32;
#[repr(C)]
pub struct VIDEO_PALETTE_DATA(i32);
#[repr(C)]
pub struct VIDEO_PERFORMANCE_COUNTER(i32);
#[repr(C)]
pub struct VIDEO_POINTER_ATTRIBUTES(i32);
#[repr(C)]
pub struct VIDEO_POINTER_CAPABILITIES(i32);
#[repr(C)]
pub struct VIDEO_POINTER_POSITION(i32);
#[repr(C)]
pub struct VIDEO_POWER_MANAGEMENT(i32);
#[repr(transparent)]
pub struct VIDEO_POWER_STATE(pub i32);
pub const VideoPowerUnspecified: VIDEO_POWER_STATE = VIDEO_POWER_STATE(0i32);
pub const VideoPowerOn: VIDEO_POWER_STATE = VIDEO_POWER_STATE(1i32);
pub const VideoPowerStandBy: VIDEO_POWER_STATE = VIDEO_POWER_STATE(2i32);
pub const VideoPowerSuspend: VIDEO_POWER_STATE = VIDEO_POWER_STATE(3i32);
pub const VideoPowerOff: VIDEO_POWER_STATE = VIDEO_POWER_STATE(4i32);
pub const VideoPowerHibernate: VIDEO_POWER_STATE = VIDEO_POWER_STATE(5i32);
pub const VideoPowerShutdown: VIDEO_POWER_STATE = VIDEO_POWER_STATE(6i32);
pub const VideoPowerMaximum: VIDEO_POWER_STATE = VIDEO_POWER_STATE(7i32);
#[repr(C)]
pub struct VIDEO_PUBLIC_ACCESS_RANGES(i32);
#[repr(C)]
pub struct VIDEO_QUERY_PERFORMANCE_COUNTER(i32);
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
pub struct VIDEO_REGISTER_VDM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEO_SHARE_MEMORY(i32);
#[repr(C)]
pub struct VIDEO_SHARE_MEMORY_INFORMATION(i32);
pub const VIDEO_STATE_NON_STANDARD_VGA: u32 = 1u32;
pub const VIDEO_STATE_PACKED_CHAIN4_MODE: u32 = 4u32;
pub const VIDEO_STATE_UNEMULATED_VGA_STATE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEO_VDM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEO_WIN32K_CALLBACKS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS(i32);
#[repr(transparent)]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(pub i32);
pub const VideoPowerNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(1i32);
pub const VideoEnumChildPdoNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(3i32);
pub const VideoFindAdapterCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(4i32);
pub const VideoPnpNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(7i32);
pub const VideoDxgkDisplaySwitchCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(8i32);
pub const VideoDxgkFindAdapterTdrCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(10i32);
pub const VideoDxgkHardwareProtectionTeardown: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(11i32);
pub const VideoRepaintDesktop: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(12i32);
pub const VideoUpdateCursor: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(13i32);
pub const VideoDisableMultiPlaneOverlay: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(14i32);
pub const VideoDesktopDuplicationChange: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(15i32);
pub const VideoBlackScreenDiagnostics: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(16i32);
#[repr(C)]
pub struct WCRUN(i32);
pub const WINDDI_MAXSETPALETTECOLORINDEX: u32 = 255u32;
pub const WINDDI_MAXSETPALETTECOLORS: u32 = 256u32;
pub const WINDDI_MAX_BROADCAST_CONTEXT: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WNDOBJ(i32);
#[cfg(feature = "Win32_Foundation")]
pub type WNDOBJCHANGEPROC = unsafe extern "system" fn(pwo: *mut WNDOBJ, fl: u32);
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
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct XFORML(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct XFORML(i32);
#[repr(C)]
pub struct XFORMOBJ(i32);
pub const XF_INV_FXTOL: i32 = 3i32;
pub const XF_INV_LTOL: i32 = 1i32;
pub const XF_LTOFX: i32 = 2i32;
pub const XF_LTOL: i32 = 0i32;
#[repr(C)]
pub struct XLATEOBJ(i32);
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
