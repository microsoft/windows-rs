#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdvancedPhotoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Hdr: Self = Self(2i32);
    pub const LowLight: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AudioDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceModule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceModuleNotificationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceModulesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: Self = Self(0i32);
    pub const Macro: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
}
#[repr(transparent)]
pub struct CallControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CallControlEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: Self = Self(0i32);
    pub const CameraHardware: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CameraOcclusionState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: Self = Self(0i32);
    pub const Streaming: Self = Self(1i32);
    pub const BlockedForPrivacy: Self = Self(2i32);
    pub const Shutdown: Self = Self(3i32);
}
#[repr(transparent)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Macro: Self = Self(2i32);
    pub const Portrait: Self = Self(3i32);
    pub const Sport: Self = Self(4i32);
    pub const Snow: Self = Self(5i32);
    pub const Night: Self = Self(6i32);
    pub const Beach: Self = Self(7i32);
    pub const Sunset: Self = Self(8i32);
    pub const Candlelight: Self = Self(9i32);
    pub const Landscape: Self = Self(10i32);
    pub const NightPortrait: Self = Self(11i32);
    pub const Backlit: Self = Self(12i32);
}
#[repr(transparent)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const Cloudy: Self = Self(2i32);
    pub const Daylight: Self = Self(3i32);
    pub const Flash: Self = Self(4i32);
    pub const Fluorescent: Self = Self(5i32);
    pub const Tungsten: Self = Self(6i32);
    pub const Candlelight: Self = Self(7i32);
}
#[repr(transparent)]
pub struct DefaultAudioCaptureDeviceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DefaultAudioRenderDeviceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DialRequestedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DigitalWindowBounds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DigitalWindowCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DigitalWindowControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ExposureCompensationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExposureControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExposurePriorityVideoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlashControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Continuous: Self = Self(2i32);
    pub const Manual: Self = Self(3i32);
}
#[repr(transparent)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
    pub const AutoMacro: Self = Self(2i32);
    pub const AutoNormal: Self = Self(3i32);
    pub const AutoInfinity: Self = Self(4i32);
    pub const AutoHyperfocal: Self = Self(5i32);
}
#[repr(transparent)]
pub struct FocusSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrVideoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IAdvancedPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedPhotoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController10(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceModule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceModuleNotificationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceModulesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceModulesManagerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraOcclusionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraOcclusionState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraOcclusionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDefaultAudioDeviceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDialRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalWindowBounds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalWindowCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalWindowControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExposureCompensationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExposureControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExposurePriorityVideoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlashControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlashControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHdrVideoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInfraredTorchControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsoSpeedControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsoSpeedControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeypadPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagPhotoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagPhotoSequenceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaDeviceControlCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModuleCommandResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpticalImageStabilizationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPanelBasedOptimizationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoConfirmationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRedialRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegionOfInterest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegionOfInterest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegionsOfInterestControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneModeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITorchControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoDeviceControllerGetDevicePropertyResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoTemporalDenoisingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWhiteBalanceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IZoomControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IZoomControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IZoomSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InfraredTorchControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const AlternatingFrameIllumination: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IsoSpeedControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsoSpeedPreset(pub i32);
impl IsoSpeedPreset {
    pub const Auto: Self = Self(0i32);
    pub const Iso50: Self = Self(1i32);
    pub const Iso80: Self = Self(2i32);
    pub const Iso100: Self = Self(3i32);
    pub const Iso200: Self = Self(4i32);
    pub const Iso400: Self = Self(5i32);
    pub const Iso800: Self = Self(6i32);
    pub const Iso1600: Self = Self(7i32);
    pub const Iso3200: Self = Self(8i32);
    pub const Iso6400: Self = Self(9i32);
    pub const Iso12800: Self = Self(10i32);
    pub const Iso25600: Self = Self(11i32);
}
#[repr(transparent)]
pub struct KeypadPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeypadPressedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoSequenceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: Self = Self(0i32);
    pub const Hyperfocal: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: Self = Self(0i32);
    pub const Lost: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Focused: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
#[repr(transparent)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: Self = Self(0i32);
    pub const Quality: Self = Self(1i32);
    pub const Latency: Self = Self(2i32);
    pub const Power: Self = Self(3i32);
    pub const LatencyThenQuality: Self = Self(4i32);
    pub const LatencyThenPower: Self = Self(5i32);
    pub const PowerAndQuality: Self = Self(6i32);
}
#[repr(transparent)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: Self = Self(0i32);
    pub const ReleaseHardwareResources: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MediaDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaDeviceControlCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ModuleCommandResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OpticalImageStabilizationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PanelBasedOptimizationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoConfirmationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RedialRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RedialRequestedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RegionOfInterest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: Self = Self(0i32);
    pub const Face: Self = Self(1i32);
}
#[repr(transparent)]
pub struct RegionsOfInterestControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneModeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
}
#[repr(transparent)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
    pub const A: Self = Self(12i32);
    pub const B: Self = Self(13i32);
    pub const C: Self = Self(14i32);
    pub const D: Self = Self(15i32);
}
#[repr(transparent)]
pub struct TorchControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerGetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const BufferTooSmall: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(5i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(6i32);
}
#[repr(transparent)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
#[repr(transparent)]
pub struct VideoTemporalDenoisingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
#[repr(transparent)]
pub struct WhiteBalanceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ZoomControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ZoomSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: Self = Self(0i32);
    pub const Direct: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
