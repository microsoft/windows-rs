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
    pub const Auto: AdvancedPhotoMode = AdvancedPhotoMode(0i32);
    pub const Standard: AdvancedPhotoMode = AdvancedPhotoMode(1i32);
    pub const Hdr: AdvancedPhotoMode = AdvancedPhotoMode(2i32);
    pub const LowLight: AdvancedPhotoMode = AdvancedPhotoMode(3i32);
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
    pub const Default: AudioDeviceRole = AudioDeviceRole(0i32);
    pub const Communications: AudioDeviceRole = AudioDeviceRole(1i32);
}
#[repr(transparent)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: AutoFocusRange = AutoFocusRange(0i32);
    pub const Macro: AutoFocusRange = AutoFocusRange(1i32);
    pub const Normal: AutoFocusRange = AutoFocusRange(2i32);
}
#[repr(transparent)]
pub struct CallControl(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CallControlContract(i32);
#[repr(transparent)]
pub struct CallControlEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: CameraOcclusionKind = CameraOcclusionKind(0i32);
    pub const CameraHardware: CameraOcclusionKind = CameraOcclusionKind(1i32);
}
#[repr(transparent)]
pub struct CameraOcclusionState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: CameraStreamState = CameraStreamState(0i32);
    pub const Streaming: CameraStreamState = CameraStreamState(1i32);
    pub const BlockedForPrivacy: CameraStreamState = CameraStreamState(2i32);
    pub const Shutdown: CameraStreamState = CameraStreamState(3i32);
}
#[repr(transparent)]
pub struct CaptureSceneMode(pub i32);
impl CaptureSceneMode {
    pub const Auto: CaptureSceneMode = CaptureSceneMode(0i32);
    pub const Manual: CaptureSceneMode = CaptureSceneMode(1i32);
    pub const Macro: CaptureSceneMode = CaptureSceneMode(2i32);
    pub const Portrait: CaptureSceneMode = CaptureSceneMode(3i32);
    pub const Sport: CaptureSceneMode = CaptureSceneMode(4i32);
    pub const Snow: CaptureSceneMode = CaptureSceneMode(5i32);
    pub const Night: CaptureSceneMode = CaptureSceneMode(6i32);
    pub const Beach: CaptureSceneMode = CaptureSceneMode(7i32);
    pub const Sunset: CaptureSceneMode = CaptureSceneMode(8i32);
    pub const Candlelight: CaptureSceneMode = CaptureSceneMode(9i32);
    pub const Landscape: CaptureSceneMode = CaptureSceneMode(10i32);
    pub const NightPortrait: CaptureSceneMode = CaptureSceneMode(11i32);
    pub const Backlit: CaptureSceneMode = CaptureSceneMode(12i32);
}
#[repr(transparent)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: CaptureUse = CaptureUse(0i32);
    pub const Photo: CaptureUse = CaptureUse(1i32);
    pub const Video: CaptureUse = CaptureUse(2i32);
}
#[repr(transparent)]
pub struct ColorTemperaturePreset(pub i32);
impl ColorTemperaturePreset {
    pub const Auto: ColorTemperaturePreset = ColorTemperaturePreset(0i32);
    pub const Manual: ColorTemperaturePreset = ColorTemperaturePreset(1i32);
    pub const Cloudy: ColorTemperaturePreset = ColorTemperaturePreset(2i32);
    pub const Daylight: ColorTemperaturePreset = ColorTemperaturePreset(3i32);
    pub const Flash: ColorTemperaturePreset = ColorTemperaturePreset(4i32);
    pub const Fluorescent: ColorTemperaturePreset = ColorTemperaturePreset(5i32);
    pub const Tungsten: ColorTemperaturePreset = ColorTemperaturePreset(6i32);
    pub const Candlelight: ColorTemperaturePreset = ColorTemperaturePreset(7i32);
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
    pub const Off: DigitalWindowMode = DigitalWindowMode(0i32);
    pub const On: DigitalWindowMode = DigitalWindowMode(1i32);
    pub const Auto: DigitalWindowMode = DigitalWindowMode(2i32);
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
    pub const Auto: FocusMode = FocusMode(0i32);
    pub const Single: FocusMode = FocusMode(1i32);
    pub const Continuous: FocusMode = FocusMode(2i32);
    pub const Manual: FocusMode = FocusMode(3i32);
}
#[repr(transparent)]
pub struct FocusPreset(pub i32);
impl FocusPreset {
    pub const Auto: FocusPreset = FocusPreset(0i32);
    pub const Manual: FocusPreset = FocusPreset(1i32);
    pub const AutoMacro: FocusPreset = FocusPreset(2i32);
    pub const AutoNormal: FocusPreset = FocusPreset(3i32);
    pub const AutoInfinity: FocusPreset = FocusPreset(4i32);
    pub const AutoHyperfocal: FocusPreset = FocusPreset(5i32);
}
#[repr(transparent)]
pub struct FocusSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrVideoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: HdrVideoMode = HdrVideoMode(0i32);
    pub const On: HdrVideoMode = HdrVideoMode(1i32);
    pub const Auto: HdrVideoMode = HdrVideoMode(2i32);
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
    pub const Off: InfraredTorchMode = InfraredTorchMode(0i32);
    pub const On: InfraredTorchMode = InfraredTorchMode(1i32);
    pub const AlternatingFrameIllumination: InfraredTorchMode = InfraredTorchMode(2i32);
}
#[repr(transparent)]
pub struct IsoSpeedControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsoSpeedPreset(pub i32);
impl IsoSpeedPreset {
    pub const Auto: IsoSpeedPreset = IsoSpeedPreset(0i32);
    pub const Iso50: IsoSpeedPreset = IsoSpeedPreset(1i32);
    pub const Iso80: IsoSpeedPreset = IsoSpeedPreset(2i32);
    pub const Iso100: IsoSpeedPreset = IsoSpeedPreset(3i32);
    pub const Iso200: IsoSpeedPreset = IsoSpeedPreset(4i32);
    pub const Iso400: IsoSpeedPreset = IsoSpeedPreset(5i32);
    pub const Iso800: IsoSpeedPreset = IsoSpeedPreset(6i32);
    pub const Iso1600: IsoSpeedPreset = IsoSpeedPreset(7i32);
    pub const Iso3200: IsoSpeedPreset = IsoSpeedPreset(8i32);
    pub const Iso6400: IsoSpeedPreset = IsoSpeedPreset(9i32);
    pub const Iso12800: IsoSpeedPreset = IsoSpeedPreset(10i32);
    pub const Iso25600: IsoSpeedPreset = IsoSpeedPreset(11i32);
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
    pub const Infinity: ManualFocusDistance = ManualFocusDistance(0i32);
    pub const Hyperfocal: ManualFocusDistance = ManualFocusDistance(1i32);
    pub const Nearest: ManualFocusDistance = ManualFocusDistance(2i32);
}
#[repr(transparent)]
pub struct MediaCaptureFocusState(pub i32);
impl MediaCaptureFocusState {
    pub const Uninitialized: MediaCaptureFocusState = MediaCaptureFocusState(0i32);
    pub const Lost: MediaCaptureFocusState = MediaCaptureFocusState(1i32);
    pub const Searching: MediaCaptureFocusState = MediaCaptureFocusState(2i32);
    pub const Focused: MediaCaptureFocusState = MediaCaptureFocusState(3i32);
    pub const Failed: MediaCaptureFocusState = MediaCaptureFocusState(4i32);
}
#[repr(transparent)]
pub struct MediaCaptureOptimization(pub i32);
impl MediaCaptureOptimization {
    pub const Default: MediaCaptureOptimization = MediaCaptureOptimization(0i32);
    pub const Quality: MediaCaptureOptimization = MediaCaptureOptimization(1i32);
    pub const Latency: MediaCaptureOptimization = MediaCaptureOptimization(2i32);
    pub const Power: MediaCaptureOptimization = MediaCaptureOptimization(3i32);
    pub const LatencyThenQuality: MediaCaptureOptimization = MediaCaptureOptimization(4i32);
    pub const LatencyThenPower: MediaCaptureOptimization = MediaCaptureOptimization(5i32);
    pub const PowerAndQuality: MediaCaptureOptimization = MediaCaptureOptimization(6i32);
}
#[repr(transparent)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: MediaCapturePauseBehavior = MediaCapturePauseBehavior(0i32);
    pub const ReleaseHardwareResources: MediaCapturePauseBehavior = MediaCapturePauseBehavior(1i32);
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
    pub const Off: OpticalImageStabilizationMode = OpticalImageStabilizationMode(0i32);
    pub const On: OpticalImageStabilizationMode = OpticalImageStabilizationMode(1i32);
    pub const Auto: OpticalImageStabilizationMode = OpticalImageStabilizationMode(2i32);
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
    pub const Unknown: RegionOfInterestType = RegionOfInterestType(0i32);
    pub const Face: RegionOfInterestType = RegionOfInterestType(1i32);
}
#[repr(transparent)]
pub struct RegionsOfInterestControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneModeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: SendCommandStatus = SendCommandStatus(0i32);
    pub const DeviceNotAvailable: SendCommandStatus = SendCommandStatus(1i32);
}
#[repr(transparent)]
pub struct TelephonyKey(pub i32);
impl TelephonyKey {
    pub const D0: TelephonyKey = TelephonyKey(0i32);
    pub const D1: TelephonyKey = TelephonyKey(1i32);
    pub const D2: TelephonyKey = TelephonyKey(2i32);
    pub const D3: TelephonyKey = TelephonyKey(3i32);
    pub const D4: TelephonyKey = TelephonyKey(4i32);
    pub const D5: TelephonyKey = TelephonyKey(5i32);
    pub const D6: TelephonyKey = TelephonyKey(6i32);
    pub const D7: TelephonyKey = TelephonyKey(7i32);
    pub const D8: TelephonyKey = TelephonyKey(8i32);
    pub const D9: TelephonyKey = TelephonyKey(9i32);
    pub const Star: TelephonyKey = TelephonyKey(10i32);
    pub const Pound: TelephonyKey = TelephonyKey(11i32);
    pub const A: TelephonyKey = TelephonyKey(12i32);
    pub const B: TelephonyKey = TelephonyKey(13i32);
    pub const C: TelephonyKey = TelephonyKey(14i32);
    pub const D: TelephonyKey = TelephonyKey(15i32);
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
    pub const Success: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(0i32);
    pub const UnknownFailure: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(1i32);
    pub const BufferTooSmall: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(2i32);
    pub const NotSupported: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(3i32);
    pub const DeviceNotAvailable: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(4i32);
    pub const MaxPropertyValueSizeTooSmall: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(5i32);
    pub const MaxPropertyValueSizeRequired: VideoDeviceControllerGetDevicePropertyStatus = VideoDeviceControllerGetDevicePropertyStatus(6i32);
}
#[repr(transparent)]
pub struct VideoDeviceControllerSetDevicePropertyStatus(pub i32);
impl VideoDeviceControllerSetDevicePropertyStatus {
    pub const Success: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(0i32);
    pub const UnknownFailure: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(1i32);
    pub const NotSupported: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(2i32);
    pub const InvalidValue: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(3i32);
    pub const DeviceNotAvailable: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(4i32);
    pub const NotInControl: VideoDeviceControllerSetDevicePropertyStatus = VideoDeviceControllerSetDevicePropertyStatus(5i32);
}
#[repr(transparent)]
pub struct VideoTemporalDenoisingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: VideoTemporalDenoisingMode = VideoTemporalDenoisingMode(0i32);
    pub const On: VideoTemporalDenoisingMode = VideoTemporalDenoisingMode(1i32);
    pub const Auto: VideoTemporalDenoisingMode = VideoTemporalDenoisingMode(2i32);
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
    pub const Auto: ZoomTransitionMode = ZoomTransitionMode(0i32);
    pub const Direct: ZoomTransitionMode = ZoomTransitionMode(1i32);
    pub const Smooth: ZoomTransitionMode = ZoomTransitionMode(2i32);
}
