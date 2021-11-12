#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdvancedPhotoControl(pub *mut ::core::ffi::c_void);
pub struct AdvancedPhotoMode(i32);
#[repr(transparent)]
pub struct AudioDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceModule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceModuleNotificationEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceModulesManager(pub *mut ::core::ffi::c_void);
pub struct AudioDeviceRole(i32);
pub struct AutoFocusRange(i32);
#[repr(transparent)]
pub struct CallControl(pub *mut ::core::ffi::c_void);
pub struct CallControlContract(i32);
#[repr(transparent)]
pub struct CallControlEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionInfo(pub *mut ::core::ffi::c_void);
pub struct CameraOcclusionKind(i32);
#[repr(transparent)]
pub struct CameraOcclusionState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraOcclusionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct CameraStreamState(i32);
pub struct CaptureSceneMode(i32);
pub struct CaptureUse(i32);
pub struct ColorTemperaturePreset(i32);
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
pub struct DigitalWindowMode(i32);
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
pub struct FocusMode(i32);
pub struct FocusPreset(i32);
#[repr(transparent)]
pub struct FocusSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrVideoControl(pub *mut ::core::ffi::c_void);
pub struct HdrVideoMode(i32);
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
pub struct InfraredTorchMode(i32);
#[repr(transparent)]
pub struct IsoSpeedControl(pub *mut ::core::ffi::c_void);
pub struct IsoSpeedPreset(i32);
#[repr(transparent)]
pub struct KeypadPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeypadPressedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoSequenceControl(pub *mut ::core::ffi::c_void);
pub struct ManualFocusDistance(i32);
pub struct MediaCaptureFocusState(i32);
pub struct MediaCaptureOptimization(i32);
pub struct MediaCapturePauseBehavior(i32);
#[repr(transparent)]
pub struct MediaDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaDeviceControlCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ModuleCommandResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OpticalImageStabilizationControl(pub *mut ::core::ffi::c_void);
pub struct OpticalImageStabilizationMode(i32);
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
pub struct RegionOfInterestType(i32);
#[repr(transparent)]
pub struct RegionsOfInterestControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneModeControl(pub *mut ::core::ffi::c_void);
pub struct SendCommandStatus(i32);
pub struct TelephonyKey(i32);
#[repr(transparent)]
pub struct TorchControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDeviceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyResult(pub *mut ::core::ffi::c_void);
pub struct VideoDeviceControllerGetDevicePropertyStatus(i32);
pub struct VideoDeviceControllerSetDevicePropertyStatus(i32);
#[repr(transparent)]
pub struct VideoTemporalDenoisingControl(pub *mut ::core::ffi::c_void);
pub struct VideoTemporalDenoisingMode(i32);
#[repr(transparent)]
pub struct WhiteBalanceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ZoomControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ZoomSettings(pub *mut ::core::ffi::c_void);
pub struct ZoomTransitionMode(i32);
