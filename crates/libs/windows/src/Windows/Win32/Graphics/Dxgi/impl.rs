pub trait IDXGIAdapterImpl: Sized + IDXGIObjectImpl {
    fn EnumOutputs();
    fn GetDesc();
    fn CheckInterfaceSupport();
}
pub trait IDXGIAdapter1Impl: Sized + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc1();
}
pub trait IDXGIAdapter2Impl: Sized + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc2();
}
pub trait IDXGIAdapter3Impl: Sized + IDXGIAdapter2Impl + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent();
    fn UnregisterHardwareContentProtectionTeardownStatus();
    fn QueryVideoMemoryInfo();
    fn SetVideoMemoryReservation();
    fn RegisterVideoMemoryBudgetChangeNotificationEvent();
    fn UnregisterVideoMemoryBudgetChangeNotification();
}
pub trait IDXGIAdapter4Impl: Sized + IDXGIAdapter3Impl + IDXGIAdapter2Impl + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc3();
}
pub trait IDXGIDebugImpl: Sized {
    fn ReportLiveObjects();
}
pub trait IDXGIDebug1Impl: Sized + IDXGIDebugImpl {
    fn EnableLeakTrackingForThread();
    fn DisableLeakTrackingForThread();
    fn IsLeakTrackingEnabledForThread();
}
pub trait IDXGIDecodeSwapChainImpl: Sized {
    fn PresentBuffer();
    fn SetSourceRect();
    fn SetTargetRect();
    fn SetDestSize();
    fn GetSourceRect();
    fn GetTargetRect();
    fn GetDestSize();
    fn SetColorSpace();
    fn GetColorSpace();
}
pub trait IDXGIDeviceImpl: Sized + IDXGIObjectImpl {
    fn GetAdapter();
    fn CreateSurface();
    fn QueryResourceResidency();
    fn SetGPUThreadPriority();
    fn GetGPUThreadPriority();
}
pub trait IDXGIDevice1Impl: Sized + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
}
pub trait IDXGIDevice2Impl: Sized + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn OfferResources();
    fn ReclaimResources();
    fn EnqueueSetEvent();
}
pub trait IDXGIDevice3Impl: Sized + IDXGIDevice2Impl + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn Trim();
}
pub trait IDXGIDevice4Impl: Sized + IDXGIDevice3Impl + IDXGIDevice2Impl + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn OfferResources1();
    fn ReclaimResources1();
}
pub trait IDXGIDeviceSubObjectImpl: Sized + IDXGIObjectImpl {
    fn GetDevice();
}
pub trait IDXGIDisplayControlImpl: Sized {
    fn IsStereoEnabled();
    fn SetStereoEnabled();
}
pub trait IDXGIFactoryImpl: Sized + IDXGIObjectImpl {
    fn EnumAdapters();
    fn MakeWindowAssociation();
    fn GetWindowAssociation();
    fn CreateSwapChain();
    fn CreateSoftwareAdapter();
}
pub trait IDXGIFactory1Impl: Sized + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapters1();
    fn IsCurrent();
}
pub trait IDXGIFactory2Impl: Sized + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn IsWindowedStereoEnabled();
    fn CreateSwapChainForHwnd();
    fn CreateSwapChainForCoreWindow();
    fn GetSharedResourceAdapterLuid();
    fn RegisterStereoStatusWindow();
    fn RegisterStereoStatusEvent();
    fn UnregisterStereoStatus();
    fn RegisterOcclusionStatusWindow();
    fn RegisterOcclusionStatusEvent();
    fn UnregisterOcclusionStatus();
    fn CreateSwapChainForComposition();
}
pub trait IDXGIFactory3Impl: Sized + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn GetCreationFlags();
}
pub trait IDXGIFactory4Impl: Sized + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapterByLuid();
    fn EnumWarpAdapter();
}
pub trait IDXGIFactory5Impl: Sized + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn CheckFeatureSupport();
}
pub trait IDXGIFactory6Impl: Sized + IDXGIFactory5Impl + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapterByGpuPreference();
}
pub trait IDXGIFactory7Impl: Sized + IDXGIFactory6Impl + IDXGIFactory5Impl + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn RegisterAdaptersChangedEvent();
    fn UnregisterAdaptersChangedEvent();
}
pub trait IDXGIFactoryMediaImpl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle();
    fn CreateDecodeSwapChainForCompositionSurfaceHandle();
}
pub trait IDXGIInfoQueueImpl: Sized {
    fn SetMessageCountLimit();
    fn ClearStoredMessages();
    fn GetMessage();
    fn GetNumStoredMessagesAllowedByRetrievalFilters();
    fn GetNumStoredMessages();
    fn GetNumMessagesDiscardedByMessageCountLimit();
    fn GetMessageCountLimit();
    fn GetNumMessagesAllowedByStorageFilter();
    fn GetNumMessagesDeniedByStorageFilter();
    fn AddStorageFilterEntries();
    fn GetStorageFilter();
    fn ClearStorageFilter();
    fn PushEmptyStorageFilter();
    fn PushDenyAllStorageFilter();
    fn PushCopyOfStorageFilter();
    fn PushStorageFilter();
    fn PopStorageFilter();
    fn GetStorageFilterStackSize();
    fn AddRetrievalFilterEntries();
    fn GetRetrievalFilter();
    fn ClearRetrievalFilter();
    fn PushEmptyRetrievalFilter();
    fn PushDenyAllRetrievalFilter();
    fn PushCopyOfRetrievalFilter();
    fn PushRetrievalFilter();
    fn PopRetrievalFilter();
    fn GetRetrievalFilterStackSize();
    fn AddMessage();
    fn AddApplicationMessage();
    fn SetBreakOnCategory();
    fn SetBreakOnSeverity();
    fn SetBreakOnID();
    fn GetBreakOnCategory();
    fn GetBreakOnSeverity();
    fn GetBreakOnID();
    fn SetMuteDebugOutput();
    fn GetMuteDebugOutput();
}
pub trait IDXGIKeyedMutexImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn AcquireSync();
    fn ReleaseSync();
}
pub trait IDXGIObjectImpl: Sized {
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn GetPrivateData();
    fn GetParent();
}
pub trait IDXGIOutputImpl: Sized + IDXGIObjectImpl {
    fn GetDesc();
    fn GetDisplayModeList();
    fn FindClosestMatchingMode();
    fn WaitForVBlank();
    fn TakeOwnership();
    fn ReleaseOwnership();
    fn GetGammaControlCapabilities();
    fn SetGammaControl();
    fn GetGammaControl();
    fn SetDisplaySurface();
    fn GetDisplaySurfaceData();
    fn GetFrameStatistics();
}
pub trait IDXGIOutput1Impl: Sized + IDXGIOutputImpl + IDXGIObjectImpl {
    fn GetDisplayModeList1();
    fn FindClosestMatchingMode1();
    fn GetDisplaySurfaceData1();
    fn DuplicateOutput();
}
pub trait IDXGIOutput2Impl: Sized + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn SupportsOverlays();
}
pub trait IDXGIOutput3Impl: Sized + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn CheckOverlaySupport();
}
pub trait IDXGIOutput4Impl: Sized + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn CheckOverlayColorSpaceSupport();
}
pub trait IDXGIOutput5Impl: Sized + IDXGIOutput4Impl + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn DuplicateOutput1();
}
pub trait IDXGIOutput6Impl: Sized + IDXGIOutput5Impl + IDXGIOutput4Impl + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn GetDesc1();
    fn CheckHardwareCompositionSupport();
}
pub trait IDXGIOutputDuplicationImpl: Sized + IDXGIObjectImpl {
    fn GetDesc();
    fn AcquireNextFrame();
    fn GetFrameDirtyRects();
    fn GetFrameMoveRects();
    fn GetFramePointerShape();
    fn MapDesktopSurface();
    fn UnMapDesktopSurface();
    fn ReleaseFrame();
}
pub trait IDXGIResourceImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetSharedHandle();
    fn GetUsage();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
pub trait IDXGIResource1Impl: Sized + IDXGIResourceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn CreateSubresourceSurface();
    fn CreateSharedHandle();
}
pub trait IDXGISurfaceImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDesc();
    fn Map();
    fn Unmap();
}
pub trait IDXGISurface1Impl: Sized + IDXGISurfaceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDC();
    fn ReleaseDC();
}
pub trait IDXGISurface2Impl: Sized + IDXGISurface1Impl + IDXGISurfaceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetResource();
}
pub trait IDXGISwapChainImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn Present();
    fn GetBuffer();
    fn SetFullscreenState();
    fn GetFullscreenState();
    fn GetDesc();
    fn ResizeBuffers();
    fn ResizeTarget();
    fn GetContainingOutput();
    fn GetFrameStatistics();
    fn GetLastPresentCount();
}
pub trait IDXGISwapChain1Impl: Sized + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDesc1();
    fn GetFullscreenDesc();
    fn GetHwnd();
    fn GetCoreWindow();
    fn Present1();
    fn IsTemporaryMonoSupported();
    fn GetRestrictToOutput();
    fn SetBackgroundColor();
    fn GetBackgroundColor();
    fn SetRotation();
    fn GetRotation();
}
pub trait IDXGISwapChain2Impl: Sized + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn SetSourceSize();
    fn GetSourceSize();
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
    fn GetFrameLatencyWaitableObject();
    fn SetMatrixTransform();
    fn GetMatrixTransform();
}
pub trait IDXGISwapChain3Impl: Sized + IDXGISwapChain2Impl + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetCurrentBackBufferIndex();
    fn CheckColorSpaceSupport();
    fn SetColorSpace1();
    fn ResizeBuffers1();
}
pub trait IDXGISwapChain4Impl: Sized + IDXGISwapChain3Impl + IDXGISwapChain2Impl + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn SetHDRMetaData();
}
pub trait IDXGISwapChainMediaImpl: Sized {
    fn GetFrameStatisticsMedia();
    fn SetPresentDuration();
    fn CheckPresentDurationSupport();
}
pub trait IDXGraphicsAnalysisImpl: Sized {
    fn BeginCapture();
    fn EndCapture();
}
