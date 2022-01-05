pub trait IDDVideoPortContainerImpl: Sized {
    fn CreateVideoPort();
    fn EnumVideoPorts();
    fn GetVideoPortConnectInfo();
    fn QueryVideoPortStatus();
}
pub trait IDirectDrawImpl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
}
pub trait IDirectDraw2Impl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
    fn GetAvailableVidMem();
}
pub trait IDirectDraw4Impl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
    fn GetAvailableVidMem();
    fn GetSurfaceFromDC();
    fn RestoreAllSurfaces();
    fn TestCooperativeLevel();
    fn GetDeviceIdentifier();
}
pub trait IDirectDraw7Impl: Sized {
    fn Compact();
    fn CreateClipper();
    fn CreatePalette();
    fn CreateSurface();
    fn DuplicateSurface();
    fn EnumDisplayModes();
    fn EnumSurfaces();
    fn FlipToGDISurface();
    fn GetCaps();
    fn GetDisplayMode();
    fn GetFourCCCodes();
    fn GetGDISurface();
    fn GetMonitorFrequency();
    fn GetScanLine();
    fn GetVerticalBlankStatus();
    fn Initialize();
    fn RestoreDisplayMode();
    fn SetCooperativeLevel();
    fn SetDisplayMode();
    fn WaitForVerticalBlank();
    fn GetAvailableVidMem();
    fn GetSurfaceFromDC();
    fn RestoreAllSurfaces();
    fn TestCooperativeLevel();
    fn GetDeviceIdentifier();
    fn StartModeTest();
    fn EvaluateMode();
}
pub trait IDirectDrawClipperImpl: Sized {
    fn GetClipList();
    fn GetHWnd();
    fn Initialize();
    fn IsClipListChanged();
    fn SetClipList();
    fn SetHWnd();
}
pub trait IDirectDrawColorControlImpl: Sized {
    fn GetColorControls();
    fn SetColorControls();
}
pub trait IDirectDrawGammaControlImpl: Sized {
    fn GetGammaRamp();
    fn SetGammaRamp();
}
pub trait IDirectDrawKernelImpl: Sized {
    fn GetCaps();
    fn GetKernelHandle();
    fn ReleaseKernelHandle();
}
pub trait IDirectDrawPaletteImpl: Sized {
    fn GetCaps();
    fn GetEntries();
    fn Initialize();
    fn SetEntries();
}
pub trait IDirectDrawSurfaceImpl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
}
pub trait IDirectDrawSurface2Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
}
pub trait IDirectDrawSurface3Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
    fn SetSurfaceDesc();
}
pub trait IDirectDrawSurface4Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
    fn SetSurfaceDesc();
    fn SetPrivateData();
    fn GetPrivateData();
    fn FreePrivateData();
    fn GetUniquenessValue();
    fn ChangeUniquenessValue();
}
pub trait IDirectDrawSurface7Impl: Sized {
    fn AddAttachedSurface();
    fn AddOverlayDirtyRect();
    fn Blt();
    fn BltBatch();
    fn BltFast();
    fn DeleteAttachedSurface();
    fn EnumAttachedSurfaces();
    fn EnumOverlayZOrders();
    fn Flip();
    fn GetAttachedSurface();
    fn GetBltStatus();
    fn GetCaps();
    fn GetClipper();
    fn GetColorKey();
    fn GetDC();
    fn GetFlipStatus();
    fn GetOverlayPosition();
    fn GetPalette();
    fn GetPixelFormat();
    fn GetSurfaceDesc();
    fn Initialize();
    fn IsLost();
    fn Lock();
    fn ReleaseDC();
    fn Restore();
    fn SetClipper();
    fn SetColorKey();
    fn SetOverlayPosition();
    fn SetPalette();
    fn Unlock();
    fn UpdateOverlay();
    fn UpdateOverlayDisplay();
    fn UpdateOverlayZOrder();
    fn GetDDInterface();
    fn PageLock();
    fn PageUnlock();
    fn SetSurfaceDesc();
    fn SetPrivateData();
    fn GetPrivateData();
    fn FreePrivateData();
    fn GetUniquenessValue();
    fn ChangeUniquenessValue();
    fn SetPriority();
    fn GetPriority();
    fn SetLOD();
    fn GetLOD();
}
pub trait IDirectDrawSurfaceKernelImpl: Sized {
    fn GetKernelHandle();
    fn ReleaseKernelHandle();
}
pub trait IDirectDrawVideoPortImpl: Sized {
    fn Flip();
    fn GetBandwidthInfo();
    fn GetColorControls();
    fn GetInputFormats();
    fn GetOutputFormats();
    fn GetFieldPolarity();
    fn GetVideoLine();
    fn GetVideoSignalStatus();
    fn SetColorControls();
    fn SetTargetSurface();
    fn StartVideo();
    fn StopVideo();
    fn UpdateVideo();
    fn WaitForSync();
}
pub trait IDirectDrawVideoPortNotifyImpl: Sized {
    fn AcquireNotification();
    fn ReleaseNotification();
}
