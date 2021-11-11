#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BRUSHOBJ_hGetColorTransform();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn BRUSHOBJ_pvAllocRbrush();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn BRUSHOBJ_pvGetRbrush();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn BRUSHOBJ_ulGetBrushColor();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_bEnum();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_cEnumStart();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CLIPOBJ_ppoGetPath();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CapabilitiesRequestAndCapabilitiesReply();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DegaussMonitor();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPhysicalMonitor();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPhysicalMonitors();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisplayConfigGetDeviceInfo();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisplayConfigSetDeviceInfo();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngAcquireSemaphore();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngAlphaBlend();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngAssociateSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngBitBlt();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCheckAbort();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngComputeGlyphSet();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCopyBits();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngCreateBitmap();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCreateClip();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngCreateDeviceBitmap();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngCreateDeviceSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn EngCreatePalette();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngCreateSemaphore();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngDeleteClip();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngDeletePalette();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngDeletePath();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngDeleteSemaphore();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngDeleteSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngEraseSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFillPath();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFindResource();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngFreeModule();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngGetCurrentCodePage();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngGetDriverName();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngGetPrinterDataFileName();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngGradientFill();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLineTo();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLoadModule();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngLockSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMarkBandingSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMultiByteToUnicodeN();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngMultiByteToWideChar();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngPaint();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngPlgBlt();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngQueryEMFInfo();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngQueryLocalTime();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn EngReleaseSemaphore();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngStretchBlt();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EngStretchBltROP();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngStrokeAndFillPath();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngStrokePath();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngTextOut();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngTransparentBlt();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngUnicodeToMultiByteN();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngUnlockSurface();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EngWideCharToMultiByte();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_cGetAllGlyphHandles();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_cGetGlyphs();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pQueryGlyphAttrs();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pfdg();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FONTOBJ_pifi();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pvTrueTypeFontFile();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_pxoGetXform();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FONTOBJ_vGetInfo();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAutoRotationState();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapabilitiesStringLength();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDisplayAutoRotationPreferences();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn GetDisplayConfigBufferSizes();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorBrightness();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorCapabilities();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorColorTemperature();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorContrast();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorDisplayAreaPosition();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorDisplayAreaSize();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorRedGreenOrBlueDrive();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorRedGreenOrBlueGain();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorTechnologyType();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetNumberOfPhysicalMonitorsFromHMONITOR();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetPhysicalMonitorsFromHMONITOR();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn GetPhysicalMonitorsFromIDirect3DDevice9();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimingReport();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVCPFeatureAndVCPFeatureReply();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HT_Get8BPPFormatPalette();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn HT_Get8BPPMaskPalette();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_bEnum();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_bEnumClipLines();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn PATHOBJ_vEnumStart();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PATHOBJ_vEnumStartClipLines();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn PATHOBJ_vGetBounds();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDisplayConfig();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreMonitorFactoryColorDefaults();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreMonitorFactoryDefaults();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bEnum();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bEnumPositionsOnly();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_bGetAdvanceWidths();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_dwGetCodePage();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn STROBJ_vEnumStart();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaveCurrentMonitorSettings();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaveCurrentSettings();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDisplayAutoRotationPreferences();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDisplayConfig();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorBrightness();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorColorTemperature();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorContrast();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorDisplayAreaPosition();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorDisplayAreaSize();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorRedGreenOrBlueDrive();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMonitorRedGreenOrBlueGain();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVCPFeature();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XFORMOBJ_bApplyXform();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XFORMOBJ_iGetXform();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XLATEOBJ_cGetPalette();
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn XLATEOBJ_hGetColorTransform();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XLATEOBJ_iXlate();
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub fn XLATEOBJ_piVector();
}
