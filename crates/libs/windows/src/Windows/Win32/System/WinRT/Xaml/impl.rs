pub trait IDesktopWindowXamlSourceNativeImpl: Sized {
    fn AttachToWindow();
    fn WindowHandle();
}
pub trait IDesktopWindowXamlSourceNative2Impl: Sized + IDesktopWindowXamlSourceNativeImpl {
    fn PreTranslateMessage();
}
pub trait IFindReferenceTargetsCallbackImpl: Sized {
    fn FoundTrackerTarget();
}
pub trait IReferenceTrackerImpl: Sized {
    fn ConnectFromTrackerSource();
    fn DisconnectFromTrackerSource();
    fn FindTrackerTargets();
    fn GetReferenceTrackerManager();
    fn AddRefFromTrackerSource();
    fn ReleaseFromTrackerSource();
    fn PegFromTrackerSource();
}
pub trait IReferenceTrackerExtensionImpl: Sized {}
pub trait IReferenceTrackerHostImpl: Sized {
    fn DisconnectUnusedReferenceSources();
    fn ReleaseDisconnectedReferenceSources();
    fn NotifyEndOfReferenceTrackingOnThread();
    fn GetTrackerTarget();
    fn AddMemoryPressure();
    fn RemoveMemoryPressure();
}
pub trait IReferenceTrackerManagerImpl: Sized {
    fn ReferenceTrackingStarted();
    fn FindTrackerTargetsCompleted();
    fn ReferenceTrackingCompleted();
    fn SetReferenceTrackerHost();
}
pub trait IReferenceTrackerTargetImpl: Sized {
    fn AddRefFromReferenceTracker();
    fn ReleaseFromReferenceTracker();
    fn Peg();
    fn Unpeg();
}
pub trait ISurfaceImageSourceManagerNativeImpl: Sized {
    fn FlushAllSurfacesWithDevice();
}
pub trait ISurfaceImageSourceNativeImpl: Sized {
    fn SetDevice();
    fn BeginDraw();
    fn EndDraw();
}
pub trait ISurfaceImageSourceNativeWithD2DImpl: Sized {
    fn SetDevice();
    fn BeginDraw();
    fn EndDraw();
    fn SuspendDraw();
    fn ResumeDraw();
}
pub trait ISwapChainBackgroundPanelNativeImpl: Sized {
    fn SetSwapChain();
}
pub trait ISwapChainPanelNativeImpl: Sized {
    fn SetSwapChain();
}
pub trait ISwapChainPanelNative2Impl: Sized + ISwapChainPanelNativeImpl {
    fn SetSwapChainHandle();
}
pub trait ITrackerOwnerImpl: Sized {
    fn CreateTrackerHandle();
    fn DeleteTrackerHandle();
    fn SetTrackerValue();
    fn TryGetSafeTrackerValue();
}
pub trait IVirtualSurfaceImageSourceNativeImpl: Sized + ISurfaceImageSourceNativeImpl {
    fn Invalidate();
    fn GetUpdateRectCount();
    fn GetUpdateRects();
    fn GetVisibleBounds();
    fn RegisterForUpdatesNeeded();
    fn Resize();
}
pub trait IVirtualSurfaceUpdatesCallbackNativeImpl: Sized {
    fn UpdatesNeeded();
}
