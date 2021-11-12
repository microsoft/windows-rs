#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: DisplayBitsPerChannel = DisplayBitsPerChannel(0u32);
    pub const Bpc6: DisplayBitsPerChannel = DisplayBitsPerChannel(1u32);
    pub const Bpc8: DisplayBitsPerChannel = DisplayBitsPerChannel(2u32);
    pub const Bpc10: DisplayBitsPerChannel = DisplayBitsPerChannel(4u32);
    pub const Bpc12: DisplayBitsPerChannel = DisplayBitsPerChannel(8u32);
    pub const Bpc14: DisplayBitsPerChannel = DisplayBitsPerChannel(16u32);
    pub const Bpc16: DisplayBitsPerChannel = DisplayBitsPerChannel(32u32);
}
#[repr(transparent)]
pub struct DisplayDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: DisplayDeviceCapability = DisplayDeviceCapability(0i32);
}
#[repr(transparent)]
pub struct DisplayFence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerDisabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerEnabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerOptions(pub u32);
impl DisplayManagerOptions {
    pub const None: DisplayManagerOptions = DisplayManagerOptions(0u32);
    pub const EnforceSourceOwnership: DisplayManagerOptions = DisplayManagerOptions(1u32);
    pub const VirtualRefreshRateAware: DisplayManagerOptions = DisplayManagerOptions(2u32);
}
#[repr(transparent)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: DisplayManagerResult = DisplayManagerResult(0i32);
    pub const UnknownFailure: DisplayManagerResult = DisplayManagerResult(1i32);
    pub const TargetAccessDenied: DisplayManagerResult = DisplayManagerResult(2i32);
    pub const TargetStale: DisplayManagerResult = DisplayManagerResult(3i32);
    pub const RemoteSessionNotSupported: DisplayManagerResult = DisplayManagerResult(4i32);
}
#[repr(transparent)]
pub struct DisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayModeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: DisplayModeQueryOptions = DisplayModeQueryOptions(0u32);
    pub const OnlyPreferredResolution: DisplayModeQueryOptions = DisplayModeQueryOptions(1u32);
}
#[repr(transparent)]
pub struct DisplayPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: DisplayPathScaling = DisplayPathScaling(0i32);
    pub const Centered: DisplayPathScaling = DisplayPathScaling(1i32);
    pub const Stretched: DisplayPathScaling = DisplayPathScaling(2i32);
    pub const AspectRatioStretched: DisplayPathScaling = DisplayPathScaling(3i32);
    pub const Custom: DisplayPathScaling = DisplayPathScaling(4i32);
    pub const DriverPreferred: DisplayPathScaling = DisplayPathScaling(5i32);
}
#[repr(transparent)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: DisplayPathStatus = DisplayPathStatus(0i32);
    pub const Succeeded: DisplayPathStatus = DisplayPathStatus(1i32);
    pub const Pending: DisplayPathStatus = DisplayPathStatus(2i32);
    pub const Failed: DisplayPathStatus = DisplayPathStatus(3i32);
    pub const FailedAsync: DisplayPathStatus = DisplayPathStatus(4i32);
    pub const InvalidatedAsync: DisplayPathStatus = DisplayPathStatus(5i32);
}
#[repr(transparent)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: DisplayPresentStatus = DisplayPresentStatus(0i32);
    pub const SourceStatusPreventedPresent: DisplayPresentStatus = DisplayPresentStatus(1i32);
    pub const ScanoutInvalid: DisplayPresentStatus = DisplayPresentStatus(2i32);
    pub const SourceInvalid: DisplayPresentStatus = DisplayPresentStatus(3i32);
    pub const DeviceInvalid: DisplayPresentStatus = DisplayPresentStatus(4i32);
    pub const UnknownFailure: DisplayPresentStatus = DisplayPresentStatus(5i32);
}
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct DisplayPresentationRate(i32);
#[repr(transparent)]
pub struct DisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: DisplayRotation = DisplayRotation(0i32);
    pub const Clockwise90Degrees: DisplayRotation = DisplayRotation(1i32);
    pub const Clockwise180Degrees: DisplayRotation = DisplayRotation(2i32);
    pub const Clockwise270Degrees: DisplayRotation = DisplayRotation(3i32);
}
#[repr(transparent)]
pub struct DisplayScanout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: DisplayScanoutOptions = DisplayScanoutOptions(0u32);
    pub const AllowTearing: DisplayScanoutOptions = DisplayScanoutOptions(2u32);
}
#[repr(transparent)]
pub struct DisplaySource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: DisplaySourceStatus = DisplaySourceStatus(0i32);
    pub const PoweredOff: DisplaySourceStatus = DisplaySourceStatus(1i32);
    pub const Invalid: DisplaySourceStatus = DisplaySourceStatus(2i32);
    pub const OwnedByAnotherDevice: DisplaySourceStatus = DisplaySourceStatus(3i32);
    pub const Unowned: DisplaySourceStatus = DisplaySourceStatus(4i32);
}
#[repr(transparent)]
pub struct DisplayState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: DisplayStateApplyOptions = DisplayStateApplyOptions(0u32);
    pub const FailIfStateChanged: DisplayStateApplyOptions = DisplayStateApplyOptions(1u32);
    pub const ForceReapply: DisplayStateApplyOptions = DisplayStateApplyOptions(2u32);
    pub const ForceModeEnumeration: DisplayStateApplyOptions = DisplayStateApplyOptions(4u32);
}
#[repr(transparent)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: DisplayStateFunctionalizeOptions = DisplayStateFunctionalizeOptions(0u32);
    pub const FailIfStateChanged: DisplayStateFunctionalizeOptions = DisplayStateFunctionalizeOptions(1u32);
    pub const ValidateTopologyOnly: DisplayStateFunctionalizeOptions = DisplayStateFunctionalizeOptions(2u32);
}
#[repr(transparent)]
pub struct DisplayStateOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: DisplayStateOperationStatus = DisplayStateOperationStatus(0i32);
    pub const PartialFailure: DisplayStateOperationStatus = DisplayStateOperationStatus(1i32);
    pub const UnknownFailure: DisplayStateOperationStatus = DisplayStateOperationStatus(2i32);
    pub const TargetOwnershipLost: DisplayStateOperationStatus = DisplayStateOperationStatus(3i32);
    pub const SystemStateChanged: DisplayStateOperationStatus = DisplayStateOperationStatus(4i32);
    pub const TooManyPathsForAdapter: DisplayStateOperationStatus = DisplayStateOperationStatus(5i32);
    pub const ModesNotSupported: DisplayStateOperationStatus = DisplayStateOperationStatus(6i32);
    pub const RemoteSessionNotSupported: DisplayStateOperationStatus = DisplayStateOperationStatus(7i32);
}
#[repr(transparent)]
pub struct DisplaySurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: DisplayTargetPersistence = DisplayTargetPersistence(0i32);
    pub const BootPersisted: DisplayTargetPersistence = DisplayTargetPersistence(1i32);
    pub const TemporaryPersisted: DisplayTargetPersistence = DisplayTargetPersistence(2i32);
    pub const PathPersisted: DisplayTargetPersistence = DisplayTargetPersistence(3i32);
}
#[repr(transparent)]
pub struct DisplayTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTaskPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTaskResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTaskSignalKind(pub i32);
impl DisplayTaskSignalKind {
    pub const OnPresentFlipAway: DisplayTaskSignalKind = DisplayTaskSignalKind(0i32);
    pub const OnPresentFlipTo: DisplayTaskSignalKind = DisplayTaskSignalKind(1i32);
}
#[repr(transparent)]
pub struct DisplayView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayWireFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: DisplayWireFormatColorSpace = DisplayWireFormatColorSpace(0i32);
    pub const BT2020: DisplayWireFormatColorSpace = DisplayWireFormatColorSpace(1i32);
    pub const ProfileDefinedWideColorGamut: DisplayWireFormatColorSpace = DisplayWireFormatColorSpace(2i32);
}
#[repr(transparent)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: DisplayWireFormatEotf = DisplayWireFormatEotf(0i32);
    pub const HdrSmpte2084: DisplayWireFormatEotf = DisplayWireFormatEotf(1i32);
}
#[repr(transparent)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(0i32);
    pub const Hdr10: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(1i32);
    pub const Hdr10Plus: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(2i32);
    pub const DolbyVisionLowLatency: DisplayWireFormatHdrMetadata = DisplayWireFormatHdrMetadata(3i32);
}
#[repr(transparent)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(0i32);
    pub const Ycc444: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(1i32);
    pub const Ycc422: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(2i32);
    pub const Ycc420: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(3i32);
    pub const Intensity: DisplayWireFormatPixelEncoding = DisplayWireFormatPixelEncoding(4i32);
}
#[repr(transparent)]
pub struct IDisplayAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayAdapterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayFence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerDisabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerEnabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayModeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayModeInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPath2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPrimaryDescriptionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayScanout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplaySource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplaySource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayStateOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplaySurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTask2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTaskPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTaskPool2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayTaskResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayWireFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayWireFormatFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayWireFormatStatics(pub *mut ::core::ffi::c_void);
