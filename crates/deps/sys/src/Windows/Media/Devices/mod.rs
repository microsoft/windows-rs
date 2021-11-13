#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdvancedPhotoCaptureSettings {}
impl ::core::clone::Clone for AdvancedPhotoCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdvancedPhotoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdvancedPhotoControl {}
impl ::core::clone::Clone for AdvancedPhotoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdvancedPhotoMode(pub i32);
impl AdvancedPhotoMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Hdr: Self = Self(2i32);
    pub const LowLight: Self = Self(3i32);
}
impl ::core::marker::Copy for AdvancedPhotoMode {}
impl ::core::clone::Clone for AdvancedPhotoMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioDeviceController {}
impl ::core::clone::Clone for AudioDeviceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceModule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioDeviceModule {}
impl ::core::clone::Clone for AudioDeviceModule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceModuleNotificationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioDeviceModuleNotificationEventArgs {}
impl ::core::clone::Clone for AudioDeviceModuleNotificationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceModulesManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioDeviceModulesManager {}
impl ::core::clone::Clone for AudioDeviceModulesManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AudioDeviceRole(pub i32);
impl AudioDeviceRole {
    pub const Default: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioDeviceRole {}
impl ::core::clone::Clone for AudioDeviceRole {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoFocusRange(pub i32);
impl AutoFocusRange {
    pub const FullRange: Self = Self(0i32);
    pub const Macro: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoFocusRange {}
impl ::core::clone::Clone for AutoFocusRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CallControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CallControl {}
impl ::core::clone::Clone for CallControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CallControlEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CallControlEventHandler {}
impl ::core::clone::Clone for CallControlEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraOcclusionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraOcclusionInfo {}
impl ::core::clone::Clone for CameraOcclusionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraOcclusionKind(pub i32);
impl CameraOcclusionKind {
    pub const Lid: Self = Self(0i32);
    pub const CameraHardware: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraOcclusionKind {}
impl ::core::clone::Clone for CameraOcclusionKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraOcclusionState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraOcclusionState {}
impl ::core::clone::Clone for CameraOcclusionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraOcclusionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraOcclusionStateChangedEventArgs {}
impl ::core::clone::Clone for CameraOcclusionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraStreamState(pub i32);
impl CameraStreamState {
    pub const NotStreaming: Self = Self(0i32);
    pub const Streaming: Self = Self(1i32);
    pub const BlockedForPrivacy: Self = Self(2i32);
    pub const Shutdown: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraStreamState {}
impl ::core::clone::Clone for CameraStreamState {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for CaptureSceneMode {}
impl ::core::clone::Clone for CaptureSceneMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CaptureUse(pub i32);
impl CaptureUse {
    pub const None: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CaptureUse {}
impl ::core::clone::Clone for CaptureUse {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for ColorTemperaturePreset {}
impl ::core::clone::Clone for ColorTemperaturePreset {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DefaultAudioCaptureDeviceChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DefaultAudioCaptureDeviceChangedEventArgs {}
impl ::core::clone::Clone for DefaultAudioCaptureDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DefaultAudioRenderDeviceChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DefaultAudioRenderDeviceChangedEventArgs {}
impl ::core::clone::Clone for DefaultAudioRenderDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialRequestedEventArgs {}
impl ::core::clone::Clone for DialRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialRequestedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialRequestedEventHandler {}
impl ::core::clone::Clone for DialRequestedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DigitalWindowBounds(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DigitalWindowBounds {}
impl ::core::clone::Clone for DigitalWindowBounds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DigitalWindowCapability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DigitalWindowCapability {}
impl ::core::clone::Clone for DigitalWindowCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DigitalWindowControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DigitalWindowControl {}
impl ::core::clone::Clone for DigitalWindowControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DigitalWindowMode(pub i32);
impl DigitalWindowMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for DigitalWindowMode {}
impl ::core::clone::Clone for DigitalWindowMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExposureCompensationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExposureCompensationControl {}
impl ::core::clone::Clone for ExposureCompensationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExposureControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExposureControl {}
impl ::core::clone::Clone for ExposureControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExposurePriorityVideoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExposurePriorityVideoControl {}
impl ::core::clone::Clone for ExposurePriorityVideoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlashControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlashControl {}
impl ::core::clone::Clone for FlashControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FocusControl {}
impl ::core::clone::Clone for FocusControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusMode(pub i32);
impl FocusMode {
    pub const Auto: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Continuous: Self = Self(2i32);
    pub const Manual: Self = Self(3i32);
}
impl ::core::marker::Copy for FocusMode {}
impl ::core::clone::Clone for FocusMode {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for FocusPreset {}
impl ::core::clone::Clone for FocusPreset {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FocusSettings {}
impl ::core::clone::Clone for FocusSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HdrVideoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HdrVideoControl {}
impl ::core::clone::Clone for HdrVideoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HdrVideoMode(pub i32);
impl HdrVideoMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for HdrVideoMode {}
impl ::core::clone::Clone for HdrVideoMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedPhotoCaptureSettings {}
impl ::core::clone::Clone for IAdvancedPhotoCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedPhotoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedPhotoControl {}
impl ::core::clone::Clone for IAdvancedPhotoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController10(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController10 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController2 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController3 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController4 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController5 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController6 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController7 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController8 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController9(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedVideoCaptureDeviceController9 {}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceController {}
impl ::core::clone::Clone for IAudioDeviceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceModule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceModule {}
impl ::core::clone::Clone for IAudioDeviceModule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceModuleNotificationEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceModuleNotificationEventArgs {}
impl ::core::clone::Clone for IAudioDeviceModuleNotificationEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceModulesManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceModulesManager {}
impl ::core::clone::Clone for IAudioDeviceModulesManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceModulesManagerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceModulesManagerFactory {}
impl ::core::clone::Clone for IAudioDeviceModulesManagerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallControl {}
impl ::core::clone::Clone for ICallControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallControlStatics {}
impl ::core::clone::Clone for ICallControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraOcclusionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraOcclusionInfo {}
impl ::core::clone::Clone for ICameraOcclusionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraOcclusionState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraOcclusionState {}
impl ::core::clone::Clone for ICameraOcclusionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraOcclusionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraOcclusionStateChangedEventArgs {}
impl ::core::clone::Clone for ICameraOcclusionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDefaultAudioDeviceChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDefaultAudioDeviceChangedEventArgs {}
impl ::core::clone::Clone for IDefaultAudioDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialRequestedEventArgs {}
impl ::core::clone::Clone for IDialRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDigitalWindowBounds(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDigitalWindowBounds {}
impl ::core::clone::Clone for IDigitalWindowBounds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDigitalWindowCapability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDigitalWindowCapability {}
impl ::core::clone::Clone for IDigitalWindowCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDigitalWindowControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDigitalWindowControl {}
impl ::core::clone::Clone for IDigitalWindowControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExposureCompensationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExposureCompensationControl {}
impl ::core::clone::Clone for IExposureCompensationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExposureControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExposureControl {}
impl ::core::clone::Clone for IExposureControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExposurePriorityVideoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExposurePriorityVideoControl {}
impl ::core::clone::Clone for IExposurePriorityVideoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlashControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlashControl {}
impl ::core::clone::Clone for IFlashControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlashControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlashControl2 {}
impl ::core::clone::Clone for IFlashControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFocusControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFocusControl {}
impl ::core::clone::Clone for IFocusControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFocusControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFocusControl2 {}
impl ::core::clone::Clone for IFocusControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFocusSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFocusSettings {}
impl ::core::clone::Clone for IFocusSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHdrVideoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHdrVideoControl {}
impl ::core::clone::Clone for IHdrVideoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInfraredTorchControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInfraredTorchControl {}
impl ::core::clone::Clone for IInfraredTorchControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsoSpeedControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsoSpeedControl {}
impl ::core::clone::Clone for IIsoSpeedControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsoSpeedControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsoSpeedControl2 {}
impl ::core::clone::Clone for IIsoSpeedControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeypadPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeypadPressedEventArgs {}
impl ::core::clone::Clone for IKeypadPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagPhotoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagPhotoControl {}
impl ::core::clone::Clone for ILowLagPhotoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagPhotoSequenceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagPhotoSequenceControl {}
impl ::core::clone::Clone for ILowLagPhotoSequenceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaDeviceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaDeviceControl {}
impl ::core::clone::Clone for IMediaDeviceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaDeviceControlCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaDeviceControlCapabilities {}
impl ::core::clone::Clone for IMediaDeviceControlCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaDeviceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaDeviceController {}
impl ::core::clone::Clone for IMediaDeviceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaDeviceStatics {}
impl ::core::clone::Clone for IMediaDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IModuleCommandResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IModuleCommandResult {}
impl ::core::clone::Clone for IModuleCommandResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOpticalImageStabilizationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOpticalImageStabilizationControl {}
impl ::core::clone::Clone for IOpticalImageStabilizationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPanelBasedOptimizationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPanelBasedOptimizationControl {}
impl ::core::clone::Clone for IPanelBasedOptimizationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoConfirmationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoConfirmationControl {}
impl ::core::clone::Clone for IPhotoConfirmationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRedialRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRedialRequestedEventArgs {}
impl ::core::clone::Clone for IRedialRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegionOfInterest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegionOfInterest {}
impl ::core::clone::Clone for IRegionOfInterest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegionOfInterest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegionOfInterest2 {}
impl ::core::clone::Clone for IRegionOfInterest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegionsOfInterestControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegionsOfInterestControl {}
impl ::core::clone::Clone for IRegionsOfInterestControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISceneModeControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISceneModeControl {}
impl ::core::clone::Clone for ISceneModeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITorchControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITorchControl {}
impl ::core::clone::Clone for ITorchControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoDeviceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoDeviceController {}
impl ::core::clone::Clone for IVideoDeviceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoDeviceControllerGetDevicePropertyResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoDeviceControllerGetDevicePropertyResult {}
impl ::core::clone::Clone for IVideoDeviceControllerGetDevicePropertyResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoTemporalDenoisingControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoTemporalDenoisingControl {}
impl ::core::clone::Clone for IVideoTemporalDenoisingControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWhiteBalanceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWhiteBalanceControl {}
impl ::core::clone::Clone for IWhiteBalanceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IZoomControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IZoomControl {}
impl ::core::clone::Clone for IZoomControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IZoomControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IZoomControl2 {}
impl ::core::clone::Clone for IZoomControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IZoomSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IZoomSettings {}
impl ::core::clone::Clone for IZoomSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InfraredTorchControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InfraredTorchControl {}
impl ::core::clone::Clone for InfraredTorchControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InfraredTorchMode(pub i32);
impl InfraredTorchMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const AlternatingFrameIllumination: Self = Self(2i32);
}
impl ::core::marker::Copy for InfraredTorchMode {}
impl ::core::clone::Clone for InfraredTorchMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsoSpeedControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsoSpeedControl {}
impl ::core::clone::Clone for IsoSpeedControl {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IsoSpeedPreset {}
impl ::core::clone::Clone for IsoSpeedPreset {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeypadPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeypadPressedEventArgs {}
impl ::core::clone::Clone for KeypadPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeypadPressedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeypadPressedEventHandler {}
impl ::core::clone::Clone for KeypadPressedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLagPhotoControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLagPhotoControl {}
impl ::core::clone::Clone for LowLagPhotoControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLagPhotoSequenceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLagPhotoSequenceControl {}
impl ::core::clone::Clone for LowLagPhotoSequenceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManualFocusDistance(pub i32);
impl ManualFocusDistance {
    pub const Infinity: Self = Self(0i32);
    pub const Hyperfocal: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for ManualFocusDistance {}
impl ::core::clone::Clone for ManualFocusDistance {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for MediaCaptureFocusState {}
impl ::core::clone::Clone for MediaCaptureFocusState {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for MediaCaptureOptimization {}
impl ::core::clone::Clone for MediaCaptureOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCapturePauseBehavior(pub i32);
impl MediaCapturePauseBehavior {
    pub const RetainHardwareResources: Self = Self(0i32);
    pub const ReleaseHardwareResources: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCapturePauseBehavior {}
impl ::core::clone::Clone for MediaCapturePauseBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaDeviceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaDeviceControl {}
impl ::core::clone::Clone for MediaDeviceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaDeviceControlCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaDeviceControlCapabilities {}
impl ::core::clone::Clone for MediaDeviceControlCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ModuleCommandResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ModuleCommandResult {}
impl ::core::clone::Clone for ModuleCommandResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OpticalImageStabilizationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OpticalImageStabilizationControl {}
impl ::core::clone::Clone for OpticalImageStabilizationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OpticalImageStabilizationMode(pub i32);
impl OpticalImageStabilizationMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for OpticalImageStabilizationMode {}
impl ::core::clone::Clone for OpticalImageStabilizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PanelBasedOptimizationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PanelBasedOptimizationControl {}
impl ::core::clone::Clone for PanelBasedOptimizationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoConfirmationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoConfirmationControl {}
impl ::core::clone::Clone for PhotoConfirmationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RedialRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RedialRequestedEventArgs {}
impl ::core::clone::Clone for RedialRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RedialRequestedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RedialRequestedEventHandler {}
impl ::core::clone::Clone for RedialRequestedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RegionOfInterest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RegionOfInterest {}
impl ::core::clone::Clone for RegionOfInterest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RegionOfInterestType(pub i32);
impl RegionOfInterestType {
    pub const Unknown: Self = Self(0i32);
    pub const Face: Self = Self(1i32);
}
impl ::core::marker::Copy for RegionOfInterestType {}
impl ::core::clone::Clone for RegionOfInterestType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RegionsOfInterestControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RegionsOfInterestControl {}
impl ::core::clone::Clone for RegionsOfInterestControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SceneModeControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SceneModeControl {}
impl ::core::clone::Clone for SceneModeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SendCommandStatus(pub i32);
impl SendCommandStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for SendCommandStatus {}
impl ::core::clone::Clone for SendCommandStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for TelephonyKey {}
impl ::core::clone::Clone for TelephonyKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TorchControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TorchControl {}
impl ::core::clone::Clone for TorchControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoDeviceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoDeviceController {}
impl ::core::clone::Clone for VideoDeviceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoDeviceControllerGetDevicePropertyResult {}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VideoDeviceControllerGetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for VideoDeviceControllerSetDevicePropertyStatus {}
impl ::core::clone::Clone for VideoDeviceControllerSetDevicePropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTemporalDenoisingControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoTemporalDenoisingControl {}
impl ::core::clone::Clone for VideoTemporalDenoisingControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoTemporalDenoisingMode(pub i32);
impl VideoTemporalDenoisingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoTemporalDenoisingMode {}
impl ::core::clone::Clone for VideoTemporalDenoisingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WhiteBalanceControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WhiteBalanceControl {}
impl ::core::clone::Clone for WhiteBalanceControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ZoomControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ZoomControl {}
impl ::core::clone::Clone for ZoomControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ZoomSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ZoomSettings {}
impl ::core::clone::Clone for ZoomSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ZoomTransitionMode(pub i32);
impl ZoomTransitionMode {
    pub const Auto: Self = Self(0i32);
    pub const Direct: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
impl ::core::marker::Copy for ZoomTransitionMode {}
impl ::core::clone::Clone for ZoomTransitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
