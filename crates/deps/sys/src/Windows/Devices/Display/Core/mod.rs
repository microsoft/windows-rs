#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: Self = Self(0u32);
    pub const Bpc6: Self = Self(1u32);
    pub const Bpc8: Self = Self(2u32);
    pub const Bpc10: Self = Self(4u32);
    pub const Bpc12: Self = Self(8u32);
    pub const Bpc14: Self = Self(16u32);
    pub const Bpc16: Self = Self(32u32);
}
#[repr(transparent)]
pub struct DisplayDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: Self = Self(0i32);
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
    pub const None: Self = Self(0u32);
    pub const EnforceSourceOwnership: Self = Self(1u32);
    pub const VirtualRefreshRateAware: Self = Self(2u32);
}
#[repr(transparent)]
pub struct DisplayManagerPathsFailedOrInvalidatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const TargetAccessDenied: Self = Self(2i32);
    pub const TargetStale: Self = Self(3i32);
    pub const RemoteSessionNotSupported: Self = Self(4i32);
}
#[repr(transparent)]
pub struct DisplayManagerResultWithState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayModeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: Self = Self(0u32);
    pub const OnlyPreferredResolution: Self = Self(1u32);
}
#[repr(transparent)]
pub struct DisplayPath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: Self = Self(0i32);
    pub const Centered: Self = Self(1i32);
    pub const Stretched: Self = Self(2i32);
    pub const AspectRatioStretched: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const DriverPreferred: Self = Self(5i32);
}
#[repr(transparent)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const FailedAsync: Self = Self(4i32);
    pub const InvalidatedAsync: Self = Self(5i32);
}
#[repr(transparent)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: Self = Self(0i32);
    pub const SourceStatusPreventedPresent: Self = Self(1i32);
    pub const ScanoutInvalid: Self = Self(2i32);
    pub const SourceInvalid: Self = Self(3i32);
    pub const DeviceInvalid: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct DisplayPresentationRate {
    pub VerticalSyncRate: super::super::super::Foundation::Numerics::Rational,
    pub VerticalSyncsPerPresentation: i32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for DisplayPresentationRate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPrimaryDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DisplayScanout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: Self = Self(0u32);
    pub const AllowTearing: Self = Self(2u32);
}
#[repr(transparent)]
pub struct DisplaySource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: Self = Self(0i32);
    pub const PoweredOff: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
    pub const OwnedByAnotherDevice: Self = Self(3i32);
    pub const Unowned: Self = Self(4i32);
}
#[repr(transparent)]
pub struct DisplayState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ForceReapply: Self = Self(2u32);
    pub const ForceModeEnumeration: Self = Self(4u32);
}
#[repr(transparent)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ValidateTopologyOnly: Self = Self(2u32);
}
#[repr(transparent)]
pub struct DisplayStateOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const PartialFailure: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
    pub const TargetOwnershipLost: Self = Self(3i32);
    pub const SystemStateChanged: Self = Self(4i32);
    pub const TooManyPathsForAdapter: Self = Self(5i32);
    pub const ModesNotSupported: Self = Self(6i32);
    pub const RemoteSessionNotSupported: Self = Self(7i32);
}
#[repr(transparent)]
pub struct DisplaySurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: Self = Self(0i32);
    pub const BootPersisted: Self = Self(1i32);
    pub const TemporaryPersisted: Self = Self(2i32);
    pub const PathPersisted: Self = Self(3i32);
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
    pub const OnPresentFlipAway: Self = Self(0i32);
    pub const OnPresentFlipTo: Self = Self(1i32);
}
#[repr(transparent)]
pub struct DisplayView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayWireFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: Self = Self(0i32);
    pub const BT2020: Self = Self(1i32);
    pub const ProfileDefinedWideColorGamut: Self = Self(2i32);
}
#[repr(transparent)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: Self = Self(0i32);
    pub const HdrSmpte2084: Self = Self(1i32);
}
#[repr(transparent)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: Self = Self(0i32);
    pub const Hdr10: Self = Self(1i32);
    pub const Hdr10Plus: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
    pub const Intensity: Self = Self(4i32);
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
