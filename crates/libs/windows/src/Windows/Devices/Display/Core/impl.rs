#[cfg(feature = "implement_exclusive")]
pub trait IDisplayAdapterImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId>;
    fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceCount(&self) -> ::windows::core::Result<u32>;
    fn PciVendorId(&self) -> ::windows::core::Result<u32>;
    fn PciDeviceId(&self) -> ::windows::core::Result<u32>;
    fn PciSubSystemId(&self) -> ::windows::core::Result<u32>;
    fn PciRevision(&self) -> ::windows::core::Result<u32>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayAdapterStaticsImpl: Sized {
    fn FromId(&self, id: &super::super::super::Graphics::DisplayAdapterId) -> ::windows::core::Result<DisplayAdapter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayDeviceImpl: Sized {
    fn CreateScanoutSource(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplaySource>;
    fn CreatePrimary(&self, target: &::core::option::Option<DisplayTarget>, desc: &::core::option::Option<DisplayPrimaryDescription>) -> ::windows::core::Result<DisplaySurface>;
    fn CreateTaskPool(&self) -> ::windows::core::Result<DisplayTaskPool>;
    fn CreatePeriodicFence(&self, target: &::core::option::Option<DisplayTarget>, offsetfromvblank: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<DisplayFence>;
    fn WaitForVBlank(&self, source: &::core::option::Option<DisplaySource>) -> ::windows::core::Result<()>;
    fn CreateSimpleScanout(&self, psource: &::core::option::Option<DisplaySource>, psurface: &::core::option::Option<DisplaySurface>, subresourceindex: u32, syncinterval: u32) -> ::windows::core::Result<DisplayScanout>;
    fn IsCapabilitySupported(&self, capability: DisplayDeviceCapability) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayDevice2Impl: Sized {
    fn CreateSimpleScanoutWithDirtyRectsAndOptions(&self, source: &::core::option::Option<DisplaySource>, surface: &::core::option::Option<DisplaySurface>, subresourceindex: u32, syncinterval: u32, dirtyrects: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Graphics::RectInt32>>, options: DisplayScanoutOptions) -> ::windows::core::Result<DisplayScanout>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayFenceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerImpl: Sized {
    fn GetCurrentTargets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>;
    fn GetCurrentAdapters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayAdapter>>;
    fn TryAcquireTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayManagerResult>;
    fn ReleaseTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<()>;
    fn TryReadCurrentStateForAllTargets(&self) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndReadCurrentState(&self, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndCreateEmptyState(&self, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn TryAcquireTargetsAndCreateSubstate(&self, existingstate: &::core::option::Option<DisplayState>, targets: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DisplayTarget>>) -> ::windows::core::Result<DisplayManagerResultWithState>;
    fn CreateDisplayDevice(&self, adapter: &::core::option::Option<DisplayAdapter>) -> ::windows::core::Result<DisplayDevice>;
    fn Enabled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerEnabledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnabled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disabled(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerDisabledEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisabled(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Changed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PathsFailedOrInvalidated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplayManager, DisplayManagerPathsFailedOrInvalidatedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePathsFailedOrInvalidated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerChangedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerDisabledEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerEnabledEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerPathsFailedOrInvalidatedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerResultWithStateImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<DisplayManagerResult>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn State(&self) -> ::windows::core::Result<DisplayState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayManagerStaticsImpl: Sized {
    fn Create(&self, options: DisplayManagerOptions) -> ::windows::core::Result<DisplayManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayModeInfoImpl: Sized {
    fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Graphics::SizeInt32>;
    fn PresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate>;
    fn IsInterlaced(&self) -> ::windows::core::Result<bool>;
    fn GetWireFormatSupportedBitsPerChannel(&self, encoding: DisplayWireFormatPixelEncoding) -> ::windows::core::Result<DisplayBitsPerChannel>;
    fn IsWireFormatSupported(&self, wireformat: &::core::option::Option<DisplayWireFormat>) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayModeInfo2Impl: Sized {
    fn PhysicalPresentationRate(&self) -> ::windows::core::Result<DisplayPresentationRate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayPathImpl: Sized {
    fn View(&self) -> ::windows::core::Result<DisplayView>;
    fn Target(&self) -> ::windows::core::Result<DisplayTarget>;
    fn Status(&self) -> ::windows::core::Result<DisplayPathStatus>;
    fn SourceResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetSourceResolution(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn SourcePixelFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn SetSourcePixelFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn SetIsStereo(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetTargetResolution(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn PresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>>;
    fn SetPresentationRate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<DisplayPresentationRate>>) -> ::windows::core::Result<()>;
    fn IsInterlaced(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>>;
    fn SetIsInterlaced(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn WireFormat(&self) -> ::windows::core::Result<DisplayWireFormat>;
    fn SetWireFormat(&self, value: &::core::option::Option<DisplayWireFormat>) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<DisplayRotation>;
    fn SetRotation(&self, value: DisplayRotation) -> ::windows::core::Result<()>;
    fn Scaling(&self) -> ::windows::core::Result<DisplayPathScaling>;
    fn SetScaling(&self, value: DisplayPathScaling) -> ::windows::core::Result<()>;
    fn FindModes(&self, flags: DisplayModeQueryOptions) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayModeInfo>>;
    fn ApplyPropertiesFromMode(&self, moderesult: &::core::option::Option<DisplayModeInfo>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayPath2Impl: Sized {
    fn PhysicalPresentationRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<DisplayPresentationRate>>;
    fn SetPhysicalPresentationRate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<DisplayPresentationRate>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayPrimaryDescriptionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat>;
    fn ColorSpace(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXColorSpace>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn MultisampleDescription(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayPrimaryDescriptionFactoryImpl: Sized {
    fn CreateInstance(&self, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: &super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::Result<DisplayPrimaryDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayPrimaryDescriptionStaticsImpl: Sized {
    fn CreateWithProperties(&self, extraproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: &super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows::core::Result<DisplayPrimaryDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayScanoutImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplaySourceImpl: Sized {
    fn AdapterId(&self) -> ::windows::core::Result<super::super::super::Graphics::DisplayAdapterId>;
    fn SourceId(&self) -> ::windows::core::Result<u32>;
    fn GetMetadata(&self, key: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplaySource2Impl: Sized {
    fn Status(&self) -> ::windows::core::Result<DisplaySourceStatus>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DisplaySource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayStateImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn IsStale(&self) -> ::windows::core::Result<bool>;
    fn Targets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayTarget>>;
    fn Views(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayView>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn ConnectTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayPath>;
    fn ConnectTargetToView(&self, target: &::core::option::Option<DisplayTarget>, view: &::core::option::Option<DisplayView>) -> ::windows::core::Result<DisplayPath>;
    fn CanConnectTargetToView(&self, target: &::core::option::Option<DisplayTarget>, view: &::core::option::Option<DisplayView>) -> ::windows::core::Result<bool>;
    fn GetViewForTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayView>;
    fn GetPathForTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<DisplayPath>;
    fn DisconnectTarget(&self, target: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<()>;
    fn TryFunctionalize(&self, options: DisplayStateFunctionalizeOptions) -> ::windows::core::Result<DisplayStateOperationResult>;
    fn TryApply(&self, options: DisplayStateApplyOptions) -> ::windows::core::Result<DisplayStateOperationResult>;
    fn Clone(&self) -> ::windows::core::Result<DisplayState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayStateOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DisplayStateOperationStatus>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplaySurfaceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTargetImpl: Sized {
    fn Adapter(&self) -> ::windows::core::Result<DisplayAdapter>;
    fn DeviceInterfacePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdapterRelativeId(&self) -> ::windows::core::Result<u32>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
    fn IsVirtualModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsVirtualTopologyEnabled(&self) -> ::windows::core::Result<bool>;
    fn UsageKind(&self) -> ::windows::core::Result<super::DisplayMonitorUsageKind>;
    fn MonitorPersistence(&self) -> ::windows::core::Result<DisplayTargetPersistence>;
    fn StableMonitorId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryGetMonitor(&self) -> ::windows::core::Result<super::DisplayMonitor>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn IsStale(&self) -> ::windows::core::Result<bool>;
    fn IsSame(&self, othertarget: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<bool>;
    fn IsEqual(&self, othertarget: &::core::option::Option<DisplayTarget>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskImpl: Sized {
    fn SetScanout(&self, scanout: &::core::option::Option<DisplayScanout>) -> ::windows::core::Result<()>;
    fn SetWait(&self, readyfence: &::core::option::Option<DisplayFence>, readyfencevalue: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTask2Impl: Sized {
    fn SetSignal(&self, signalkind: DisplayTaskSignalKind, fence: &::core::option::Option<DisplayFence>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskPoolImpl: Sized {
    fn CreateTask(&self) -> ::windows::core::Result<DisplayTask>;
    fn ExecuteTask(&self, task: &::core::option::Option<DisplayTask>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskPool2Impl: Sized {
    fn TryExecuteTask(&self, task: &::core::option::Option<DisplayTask>) -> ::windows::core::Result<DisplayTaskResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayTaskResultImpl: Sized {
    fn PresentStatus(&self) -> ::windows::core::Result<DisplayPresentStatus>;
    fn PresentId(&self) -> ::windows::core::Result<u64>;
    fn SourceStatus(&self) -> ::windows::core::Result<DisplaySourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayViewImpl: Sized {
    fn Paths(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<DisplayPath>>;
    fn ContentResolution(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>;
    fn SetContentResolution(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Graphics::SizeInt32>>) -> ::windows::core::Result<()>;
    fn SetPrimaryPath(&self, path: &::core::option::Option<DisplayPath>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayWireFormatImpl: Sized {
    fn PixelEncoding(&self) -> ::windows::core::Result<DisplayWireFormatPixelEncoding>;
    fn BitsPerChannel(&self) -> ::windows::core::Result<i32>;
    fn ColorSpace(&self) -> ::windows::core::Result<DisplayWireFormatColorSpace>;
    fn Eotf(&self) -> ::windows::core::Result<DisplayWireFormatEotf>;
    fn HdrMetadata(&self) -> ::windows::core::Result<DisplayWireFormatHdrMetadata>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayWireFormatFactoryImpl: Sized {
    fn CreateInstance(&self, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayWireFormatStaticsImpl: Sized {
    fn CreateWithProperties(&self, extraproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::GUID, ::windows::core::IInspectable>>>, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata) -> ::windows::core::Result<DisplayWireFormat>;
}
