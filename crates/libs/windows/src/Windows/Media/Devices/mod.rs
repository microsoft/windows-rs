#[cfg(feature = "Media_Devices_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_Vtbl;
}
impl ::core::clone::Clone for IAdvancedPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedPhotoCaptureSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08f3863a_0018_445b_93d2_646d1c5ed05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AdvancedPhotoMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_Vtbl;
}
impl ::core::clone::Clone for IAdvancedPhotoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedPhotoControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5b15486_9001_4682_9309_68eae0080eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdvancedPhotoMode) -> ::windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController {
    type Vtable = IAdvancedVideoCaptureDeviceController_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde6ff4d3_2b96_4583_80ab_b5b01dc6a8d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDeviceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController10(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController10 {
    type Vtable = IAdvancedVideoCaptureDeviceController10_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController10 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController10 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc621b82d_d6f0_5c1b_a388_a6e938407146);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController10_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CameraOcclusionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController11(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController11 {
    type Vtable = IAdvancedVideoCaptureDeviceController11_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController11 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController11 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5b65ae2_3772_580c_a630_e75de9106904);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController11_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture")]
    pub TryAcquireExclusiveControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, mode: super::Capture::MediaCaptureDeviceExclusiveControlReleaseMode, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryAcquireExclusiveControl: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController2 {
    type Vtable = IAdvancedVideoCaptureDeviceController2_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bb94f8f_f11a_43db_b402_11930b80ae56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LowLagPhotoSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LowLagPhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SceneModeControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TorchControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FlashControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WhiteBalanceControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExposureControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegionsOfInterestControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrimaryUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CaptureUse) -> ::windows_core::HRESULT,
    pub SetPrimaryUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CaptureUse) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController3 {
    type Vtable = IAdvancedVideoCaptureDeviceController3_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa98b8f34_ee0d_470c_b9f0_4229c4bbd089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices_Core")]
    pub VariablePhotoSequenceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices_Core"))]
    VariablePhotoSequenceController: usize,
    pub PhotoConfirmationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ZoomControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController4 {
    type Vtable = IAdvancedVideoCaptureDeviceController4_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea9fbfaf_d371_41c3_9a17_824a87ebdfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExposurePriorityVideoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DesiredOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureOptimization) -> ::windows_core::HRESULT,
    pub SetDesiredOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureOptimization) -> ::windows_core::HRESULT,
    pub HdrVideoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpticalImageStabilizationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AdvancedPhotoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController5 {
    type Vtable = IAdvancedVideoCaptureDeviceController5_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33512b17_b9cb_4a23_b875_f9eaab535492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyById: usize,
    pub SetDevicePropertyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDevicePropertyByExtendedId: usize,
    pub SetDevicePropertyByExtendedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut VideoDeviceControllerSetDevicePropertyStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController6 {
    type Vtable = IAdvancedVideoCaptureDeviceController6_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6563a53_68a1_44b7_9f89_b5fa97ac0cbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoTemporalDenoisingControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController7 {
    type Vtable = IAdvancedVideoCaptureDeviceController7_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d2927f0_a054_50e7_b7df_7c04234d10f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InfraredTorchControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController8 {
    type Vtable = IAdvancedVideoCaptureDeviceController8_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController8 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd843f010_e7fb_595b_9a78_0e54c4532b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PanelBasedOptimizationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedVideoCaptureDeviceController9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedVideoCaptureDeviceController9 {
    type Vtable = IAdvancedVideoCaptureDeviceController9_Vtbl;
}
impl ::core::clone::Clone for IAdvancedVideoCaptureDeviceController9 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedVideoCaptureDeviceController9 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bdca95d_0255_51bc_a10d_5a169ec1625a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedVideoCaptureDeviceController9_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DigitalWindowControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceController {
    type Vtable = IAudioDeviceController_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioDeviceController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedd4a388_79c7_4f7c_90e8_ef934b21580a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Muted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetVolumePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub VolumePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModule(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceModule {
    type Vtable = IAudioDeviceModule_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceModule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioDeviceModule {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86cfac36_47c1_4b33_9852_8773ec4be123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModule_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendCommandAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModuleNotificationEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceModuleNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioDeviceModuleNotificationEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3e3ccaf_224c_48be_956b_9a13134e96e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModuleNotificationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Module: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub NotificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    NotificationData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModulesManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceModulesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioDeviceModulesManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6aa40c4d_960a_4d1c_b318_0022604547ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ModuleNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveModuleNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveModuleNotificationReceived: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllById: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceModulesManagerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioDeviceModulesManagerFactory {
    type Vtable = IAudioDeviceModulesManagerFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceModulesManagerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioDeviceModulesManagerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8db03670_e64d_4773_96c0_bc7ebf0e063f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesManagerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICallControl {
    type Vtable = ICallControl_Vtbl;
}
impl ::core::clone::Clone for ICallControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICallControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa520d0d6_ae8d_45db_8011_ca49d3b3e578);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IndicateNewIncomingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enableringer: bool, callerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IndicateNewOutgoingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IndicateActiveCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows_core::HRESULT,
    pub EndCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calltoken: u64) -> ::windows_core::HRESULT,
    pub HasRinger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HangUpRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHangUpRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHangUpRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RedialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRedialRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRedialRequested: usize,
    #[cfg(feature = "Foundation")]
    pub KeypadPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    KeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveKeypadPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveKeypadPressed: usize,
    #[cfg(feature = "Foundation")]
    pub AudioTransferRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioTransferRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioTransferRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioTransferRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICallControlStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICallControlStatics {
    type Vtable = ICallControlStatics_Vtbl;
}
impl ::core::clone::Clone for ICallControlStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICallControlStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03945ad5_85ab_40e1_af19_56c94303b019);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallControlStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOcclusionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_Vtbl;
}
impl ::core::clone::Clone for ICameraOcclusionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraOcclusionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf6c4ad0_a84d_5db6_be58_a5da21cfe011);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsOcclusionKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOcclusionState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraOcclusionState {
    type Vtable = ICameraOcclusionState_Vtbl;
}
impl ::core::clone::Clone for ICameraOcclusionState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraOcclusionState {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x430adeb8_6842_5e55_9bde_04b4ef3a8a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionState_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsOccluded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsOcclusionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, occlusionkind: CameraOcclusionKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOcclusionStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICameraOcclusionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraOcclusionStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8512d848_c0de_57ca_a1ca_fb2c3d23df55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOcclusionStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct IDefaultAudioDeviceChangedEventArgs(::windows_core::IUnknown);
impl IDefaultAudioDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Role(&self) -> ::windows_core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Role)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IDefaultAudioDeviceChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IDefaultAudioDeviceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDefaultAudioDeviceChangedEventArgs {}
impl ::core::fmt::Debug for IDefaultAudioDeviceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDefaultAudioDeviceChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IDefaultAudioDeviceChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{110f882f-1c05-4657-a18e-47c9b69f07ab}");
}
unsafe impl ::windows_core::Interface for IDefaultAudioDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDefaultAudioDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDefaultAudioDeviceChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x110f882f_1c05_4657_a18e_47c9b69f07ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultAudioDeviceChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceRole) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDialRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDialRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x037b929e_953c_4286_8866_4f0f376c855a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDigitalWindowBounds(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_Vtbl;
}
impl ::core::clone::Clone for IDigitalWindowBounds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDigitalWindowBounds {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd4f21dd_d173_5c6b_8c25_bdd26d5122b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowBounds_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NormalizedOriginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetNormalizedOriginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub NormalizedOriginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetNormalizedOriginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDigitalWindowCapability(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_Vtbl;
}
impl ::core::clone::Clone for IDigitalWindowCapability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDigitalWindowCapability {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd78bad2c_f721_5244_a196_b56ccbec606c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowCapability_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MinScaleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MaxScaleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MinScaleValueWithoutUpsampling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedFieldOfViewLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedFieldOfViewLimit: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDigitalWindowControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDigitalWindowControl {
    type Vtable = IDigitalWindowControl_Vtbl;
}
impl ::core::clone::Clone for IDigitalWindowControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDigitalWindowControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23b69eff_65d2_53ea_8780_de582b48b544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDigitalWindowControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut DigitalWindowMode) -> ::windows_core::HRESULT,
    pub CurrentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DigitalWindowMode) -> ::windows_core::HRESULT,
    pub GetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode) -> ::windows_core::HRESULT,
    pub ConfigureWithBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCapabilities: usize,
    pub GetCapabilityForSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExposureCompensationControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExposureCompensationControl {
    type Vtable = IExposureCompensationControl_Vtbl;
}
impl ::core::clone::Clone for IExposureCompensationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IExposureCompensationControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81c8e834_dcec_4011_a610_1f3847e64aca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposureCompensationControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExposureControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExposureControl {
    type Vtable = IExposureControl_Vtbl;
}
impl ::core::clone::Clone for IExposureControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IExposureControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09e8cbe2_ad96_4f28_a0e0_96ed7e1b5fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposureControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutterduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExposurePriorityVideoControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_Vtbl;
}
impl ::core::clone::Clone for IExposurePriorityVideoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IExposurePriorityVideoControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2cb240a3_5168_4271_9ea5_47621a98a352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExposurePriorityVideoControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlashControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlashControl {
    type Vtable = IFlashControl_Vtbl;
}
impl ::core::clone::Clone for IFlashControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFlashControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdef41dbe_7d68_45e3_8c0f_be7bb32837d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlashControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlashControl2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlashControl2 {
    type Vtable = IFlashControl2_Vtbl;
}
impl ::core::clone::Clone for IFlashControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFlashControl2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d29cc9e_75e1_4af7_bd7d_4e38e1c06cd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlashControl2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AssistantLightSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AssistantLightEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAssistantLightEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusControl {
    type Vtable = IFocusControl_Vtbl;
}
impl ::core::clone::Clone for IFocusControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFocusControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0d889f6_5228_4453_b153_85606592b238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPresets: usize,
    pub Preset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusPreset) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: FocusPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetPresetWithCompletionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: FocusPreset, completebeforefocus: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetWithCompletionOptionAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focus: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusControl2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusControl2 {
    type Vtable = IFocusControl2_Vtbl;
}
impl ::core::clone::Clone for IFocusControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFocusControl2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f7cff48_c534_4e9e_94c3_52ef2afd5d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusControl2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FocusChangedSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub WaitForFocusSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusModes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusDistances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusDistances: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFocusRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFocusRanges: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureFocusState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnlockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockAsync: usize,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusSettings {
    type Vtable = IFocusSettings_Vtbl;
}
impl ::core::clone::Clone for IFocusSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFocusSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79958f6b_3263_4275_85d6_aeae891c96ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FocusMode) -> ::windows_core::HRESULT,
    pub AutoFocusRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoFocusRange) -> ::windows_core::HRESULT,
    pub SetAutoFocusRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoFocusRange) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Foundation")]
    pub Distance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Distance: usize,
    #[cfg(feature = "Foundation")]
    pub SetDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDistance: usize,
    pub WaitForFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetWaitForFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DisableDriverFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDisableDriverFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdrVideoControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdrVideoControl {
    type Vtable = IHdrVideoControl_Vtbl;
}
impl ::core::clone::Clone for IHdrVideoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHdrVideoControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55d8e2d0_30c0_43bf_9b9a_9799d70ced94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdrVideoControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdrVideoMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HdrVideoMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInfraredTorchControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInfraredTorchControl {
    type Vtable = IInfraredTorchControl_Vtbl;
}
impl ::core::clone::Clone for IInfraredTorchControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInfraredTorchControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cba2c83_6cb6_5a04_a6fc_3be7b33ff056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInfraredTorchControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub CurrentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InfraredTorchMode) -> ::windows_core::HRESULT,
    pub SetCurrentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InfraredTorchMode) -> ::windows_core::HRESULT,
    pub MinPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MaxPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PowerStep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Power: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsoSpeedControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsoSpeedControl {
    type Vtable = IIsoSpeedControl_Vtbl;
}
impl ::core::clone::Clone for IIsoSpeedControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIsoSpeedControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27b6c322_25ad_4f1b_aaab_524ab376ca33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoSpeedControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedPresets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedPresets: usize,
    #[cfg(feature = "deprecated")]
    pub Preset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsoSpeedPreset) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Preset: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: IsoSpeedPreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPresetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsoSpeedControl2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsoSpeedControl2 {
    type Vtable = IIsoSpeedControl2_Vtbl;
}
impl ::core::clone::Clone for IIsoSpeedControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIsoSpeedControl2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1578f2_6d77_4f8a_8c2f_6130b6395053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsoSpeedControl2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isospeed: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetAutoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeypadPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IKeypadPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKeypadPressedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3a43900_b4fa_49cd_9442_89af6568f601);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeypadPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TelephonyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TelephonyKey) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_Vtbl;
}
impl ::core::clone::Clone for ILowLagPhotoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagPhotoControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d5c4dd0_fadf_415d_aee6_3baa529300c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoSequenceControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_Vtbl;
}
impl ::core::clone::Clone for ILowLagPhotoSequenceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagPhotoSequenceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3dcf909d_6d16_409c_bafe_b9a594c6fde6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxPastPhotos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub PastPhotoLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetPastPhotoLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub ThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetThumbnailEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub ThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaThumbnailFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    ThumbnailFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetThumbnailFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetThumbnailFormat: usize,
    pub DesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDesiredThumbnailSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HardwareAcceleratedThumbnailSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaDeviceControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaDeviceControl {
    type Vtable = IMediaDeviceControl_Vtbl;
}
impl ::core::clone::Clone for IMediaDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaDeviceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefa8dfa9_6f75_4863_ba0b_583f3036b4de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryGetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut bool, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TrySetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaDeviceControlCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_Vtbl;
}
impl ::core::clone::Clone for IMediaDeviceControlCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaDeviceControlCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23005816_eb85_43e2_b92b_8240d5ee70ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceControlCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AutoModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct IMediaDeviceController(::windows_core::IUnknown);
impl IMediaDeviceController {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableMediaStreamProperties)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMediaStreamProperties)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<P0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetMediaStreamPropertiesAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaDeviceController, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaDeviceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaDeviceController {}
impl ::core::fmt::Debug for IMediaDeviceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaDeviceController").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaDeviceController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{f6f8f5ce-209a-48fb-86fc-d44578f317e6}");
}
unsafe impl ::windows_core::Interface for IMediaDeviceController {
    type Vtable = IMediaDeviceController_Vtbl;
}
impl ::core::clone::Clone for IMediaDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaDeviceController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6f8f5ce_209a_48fb_86fc_d44578f317e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetAvailableMediaStreamProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetAvailableMediaStreamProperties: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub GetMediaStreamProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_MediaProperties")))]
    GetMediaStreamProperties: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub SetMediaStreamPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    SetMediaStreamPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaDeviceStatics {
    type Vtable = IMediaDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IMediaDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa2d9a40_909f_4bba_bf8b_0c0d296f14f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAudioCaptureSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAudioRenderSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetVideoCaptureSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDefaultAudioCaptureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDefaultAudioRenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: AudioDeviceRole, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioCaptureDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioCaptureDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultAudioRenderDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultAudioRenderDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultAudioRenderDeviceChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IModuleCommandResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IModuleCommandResult {
    type Vtable = IModuleCommandResult_Vtbl;
}
impl ::core::clone::Clone for IModuleCommandResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IModuleCommandResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520d1eb4_1374_4c7d_b1e4_39dcdf3eae4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IModuleCommandResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SendCommandStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOpticalImageStabilizationControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_Vtbl;
}
impl ::core::clone::Clone for IOpticalImageStabilizationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOpticalImageStabilizationControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfad9c1d_00bc_423b_8eb2_a0178ca94247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpticalImageStabilizationControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OpticalImageStabilizationMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: OpticalImageStabilizationMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPanelBasedOptimizationControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_Vtbl;
}
impl ::core::clone::Clone for IPanelBasedOptimizationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPanelBasedOptimizationControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33323223_6247_5419_a5a4_3d808645d917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPanelBasedOptimizationControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Panel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Panel: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Devices::Enumeration::Panel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPanel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoConfirmationControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_Vtbl;
}
impl ::core::clone::Clone for IPhotoConfirmationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhotoConfirmationControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8f3f363_ff5e_4582_a9a8_0550f85a4a76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub PixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PixelFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::MediaProperties::MediaPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPixelFormat: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRedialRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRedialRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRedialRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7eb55209_76ab_4c31_b40e_4b58379d580c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedialRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegionOfInterest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRegionOfInterest {
    type Vtable = IRegionOfInterest_Vtbl;
}
impl ::core::clone::Clone for IRegionOfInterest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRegionOfInterest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5ecc834_ce66_4e05_a78f_cf391a5ec2d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionOfInterest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AutoFocusEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoFocusEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoWhiteBalanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegionOfInterest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRegionOfInterest2 {
    type Vtable = IRegionOfInterest2_Vtbl;
}
impl ::core::clone::Clone for IRegionOfInterest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRegionOfInterest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19fe2a91_73aa_4d51_8a9d_56ccf7db7f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionOfInterest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RegionOfInterestType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RegionOfInterestType) -> ::windows_core::HRESULT,
    pub BoundsNormalized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBoundsNormalized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegionsOfInterestControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_Vtbl;
}
impl ::core::clone::Clone for IRegionsOfInterestControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRegionsOfInterestControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc323f527_ab0b_4558_8b5b_df5693db0378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegionsOfInterestControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRegionsWithLockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regions: *mut ::core::ffi::c_void, lockvalues: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRegionsWithLockAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearRegionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearRegionsAsync: usize,
    pub AutoFocusSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AutoWhiteBalanceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AutoExposureSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneModeControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneModeControl {
    type Vtable = ISceneModeControl_Vtbl;
}
impl ::core::clone::Clone for ISceneModeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneModeControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd48e5af7_8d59_4854_8c62_12c70ba89b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModeControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CaptureSceneMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenemode: CaptureSceneMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITorchControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITorchControl {
    type Vtable = ITorchControl_Vtbl;
}
impl ::core::clone::Clone for ITorchControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITorchControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6053665_8250_416c_919a_724296afa306);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITorchControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDeviceController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoDeviceController {
    type Vtable = IVideoDeviceController_Vtbl;
}
impl ::core::clone::Clone for IVideoDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoDeviceController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99555575_2e2e_40b8_b6c7_f82d10013210);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDeviceController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Contrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Hue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WhiteBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BacklightCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Tilt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Zoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Roll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub TrySetPowerlineFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TrySetPowerlineFrequency: usize,
    #[cfg(feature = "Media_Capture")]
    pub TryGetPowerlineFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Capture::PowerlineFrequency, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    TryGetPowerlineFrequency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDeviceControllerGetDevicePropertyResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_Vtbl;
}
impl ::core::clone::Clone for IVideoDeviceControllerGetDevicePropertyResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoDeviceControllerGetDevicePropertyResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5d88395_6ed5_4790_8b5d_0ef13935d0f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDeviceControllerGetDevicePropertyResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceControllerGetDevicePropertyStatus) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTemporalDenoisingControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_Vtbl;
}
impl ::core::clone::Clone for IVideoTemporalDenoisingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoTemporalDenoisingControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ab34735_3e2a_4a32_baff_4358c4fbdd57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTemporalDenoisingControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoTemporalDenoisingMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoTemporalDenoisingMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWhiteBalanceControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_Vtbl;
}
impl ::core::clone::Clone for IWhiteBalanceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWhiteBalanceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x781f047e_7162_49c8_a8f9_9481c565363e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWhiteBalanceControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Preset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ColorTemperaturePreset) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPresetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preset: ColorTemperaturePreset, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPresetAsync: usize,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetValueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temperature: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IZoomControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IZoomControl {
    type Vtable = IZoomControl_Vtbl;
}
impl ::core::clone::Clone for IZoomControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IZoomControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a1e0b12_32da_4c17_bfd7_8d0c73c8f5a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IZoomControl2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IZoomControl2 {
    type Vtable = IZoomControl2_Vtbl;
}
impl ::core::clone::Clone for IZoomControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IZoomControl2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69843db0_2e99_4641_8529_184f319d1671);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomControl2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModes: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows_core::HRESULT,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IZoomSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IZoomSettings {
    type Vtable = IZoomSettings_Vtbl;
}
impl ::core::clone::Clone for IZoomSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IZoomSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ad66b24_14b4_4bfd_b18f_88fe24463b52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ZoomTransitionMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ZoomTransitionMode) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoCaptureSettings(::windows_core::IUnknown);
impl AdvancedPhotoCaptureSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AdvancedPhotoCaptureSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows_core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: AdvancedPhotoMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AdvancedPhotoCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedPhotoCaptureSettings {}
impl ::core::fmt::Debug for AdvancedPhotoCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoCaptureSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AdvancedPhotoCaptureSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AdvancedPhotoCaptureSettings;{08f3863a-0018-445b-93d2-646d1c5ed05c})");
}
impl ::core::clone::Clone for AdvancedPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AdvancedPhotoCaptureSettings {
    type Vtable = IAdvancedPhotoCaptureSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AdvancedPhotoCaptureSettings {
    const IID: ::windows_core::GUID = <IAdvancedPhotoCaptureSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoCaptureSettings";
}
::windows_core::imp::interface_hierarchy!(AdvancedPhotoCaptureSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AdvancedPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCaptureSettings {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoControl(::windows_core::IUnknown);
impl AdvancedPhotoControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configure<P0>(&self, settings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AdvancedPhotoCaptureSettings>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Configure)(::windows_core::Interface::as_raw(this), settings.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for AdvancedPhotoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedPhotoControl {}
impl ::core::fmt::Debug for AdvancedPhotoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AdvancedPhotoControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AdvancedPhotoControl;{c5b15486-9001-4682-9309-68eae0080eec})");
}
impl ::core::clone::Clone for AdvancedPhotoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AdvancedPhotoControl {
    type Vtable = IAdvancedPhotoControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AdvancedPhotoControl {
    const IID: ::windows_core::GUID = <IAdvancedPhotoControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.AdvancedPhotoControl";
}
::windows_core::imp::interface_hierarchy!(AdvancedPhotoControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AdvancedPhotoControl {}
unsafe impl ::core::marker::Sync for AdvancedPhotoControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceController(::windows_core::IUnknown);
impl AudioDeviceController {
    pub fn SetMuted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMuted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Muted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Muted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVolumePercent(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVolumePercent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VolumePercent(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VolumePercent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &::windows_core::ComInterface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableMediaStreamProperties)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMediaStreamProperties)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<P0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetMediaStreamPropertiesAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioDeviceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceController {}
impl ::core::fmt::Debug for AudioDeviceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceController").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDeviceController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceController;{edd4a388-79c7-4f7c-90e8-ef934b21580a})");
}
impl ::core::clone::Clone for AudioDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioDeviceController {
    type Vtable = IAudioDeviceController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioDeviceController {
    const IID: ::windows_core::GUID = <IAudioDeviceController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceController";
}
::windows_core::imp::interface_hierarchy!(AudioDeviceController, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaDeviceController> for AudioDeviceController {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceModule(::windows_core::IUnknown);
impl AudioDeviceModule {
    pub fn ClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClassId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MajorVersion(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MajorVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinorVersion(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinorVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendCommandAsync<P0>(&self, command: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendCommandAsync)(::windows_core::Interface::as_raw(this), command.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioDeviceModule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceModule {}
impl ::core::fmt::Debug for AudioDeviceModule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceModule").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDeviceModule {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModule;{86cfac36-47c1-4b33-9852-8773ec4be123})");
}
impl ::core::clone::Clone for AudioDeviceModule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioDeviceModule {
    type Vtable = IAudioDeviceModule_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioDeviceModule {
    const IID: ::windows_core::GUID = <IAudioDeviceModule as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioDeviceModule {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModule";
}
::windows_core::imp::interface_hierarchy!(AudioDeviceModule, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceModuleNotificationEventArgs(::windows_core::IUnknown);
impl AudioDeviceModuleNotificationEventArgs {
    pub fn Module(&self) -> ::windows_core::Result<AudioDeviceModule> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Module)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn NotificationData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotificationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioDeviceModuleNotificationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceModuleNotificationEventArgs {}
impl ::core::fmt::Debug for AudioDeviceModuleNotificationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceModuleNotificationEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDeviceModuleNotificationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs;{e3e3ccaf-224c-48be-956b-9a13134e96e8})");
}
impl ::core::clone::Clone for AudioDeviceModuleNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioDeviceModuleNotificationEventArgs {
    type Vtable = IAudioDeviceModuleNotificationEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioDeviceModuleNotificationEventArgs {
    const IID: ::windows_core::GUID = <IAudioDeviceModuleNotificationEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioDeviceModuleNotificationEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModuleNotificationEventArgs";
}
::windows_core::imp::interface_hierarchy!(AudioDeviceModuleNotificationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AudioDeviceModuleNotificationEventArgs {}
unsafe impl ::core::marker::Sync for AudioDeviceModuleNotificationEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioDeviceModulesManager(::windows_core::IUnknown);
impl AudioDeviceModulesManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ModuleNotificationReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModuleNotificationReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveModuleNotificationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveModuleNotificationReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllById(&self, moduleid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllById)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(moduleid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAll)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<AudioDeviceModulesManager> {
        Self::IAudioDeviceModulesManagerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioDeviceModulesManagerFactory<R, F: FnOnce(&IAudioDeviceModulesManagerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioDeviceModulesManager, IAudioDeviceModulesManagerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioDeviceModulesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceModulesManager {}
impl ::core::fmt::Debug for AudioDeviceModulesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceModulesManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDeviceModulesManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.AudioDeviceModulesManager;{6aa40c4d-960a-4d1c-b318-0022604547ed})");
}
impl ::core::clone::Clone for AudioDeviceModulesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioDeviceModulesManager {
    type Vtable = IAudioDeviceModulesManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioDeviceModulesManager {
    const IID: ::windows_core::GUID = <IAudioDeviceModulesManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioDeviceModulesManager {
    const NAME: &'static str = "Windows.Media.Devices.AudioDeviceModulesManager";
}
::windows_core::imp::interface_hierarchy!(AudioDeviceModulesManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AudioDeviceModulesManager {}
unsafe impl ::core::marker::Sync for AudioDeviceModulesManager {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CallControl(::windows_core::IUnknown);
impl CallControl {
    pub fn IndicateNewIncomingCall(&self, enableringer: bool, callerid: &::windows_core::HSTRING) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndicateNewIncomingCall)(::windows_core::Interface::as_raw(this), enableringer, ::core::mem::transmute_copy(callerid), &mut result__).from_abi(result__)
        }
    }
    pub fn IndicateNewOutgoingCall(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndicateNewOutgoingCall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IndicateActiveCall(&self, calltoken: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).IndicateActiveCall)(::windows_core::Interface::as_raw(this), calltoken).ok() }
    }
    pub fn EndCall(&self, calltoken: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndCall)(::windows_core::Interface::as_raw(this), calltoken).ok() }
    }
    pub fn HasRinger(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasRinger)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnswerRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<CallControlEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnswerRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnswerRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAnswerRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HangUpRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<CallControlEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HangUpRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHangUpRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHangUpRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DialRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<DialRequestedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DialRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDialRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDialRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RedialRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<RedialRequestedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RedialRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRedialRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRedialRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn KeypadPressed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<KeypadPressedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeypadPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeypadPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveKeypadPressed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioTransferRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<CallControlEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioTransferRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioTransferRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioTransferRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromId(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<CallControl> {
        Self::ICallControlStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICallControlStatics<R, F: FnOnce(&ICallControlStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CallControl, ICallControlStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CallControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallControl {}
impl ::core::fmt::Debug for CallControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CallControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CallControl;{a520d0d6-ae8d-45db-8011-ca49d3b3e578})");
}
impl ::core::clone::Clone for CallControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CallControl {
    type Vtable = ICallControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CallControl {
    const IID: ::windows_core::GUID = <ICallControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CallControl {
    const NAME: &'static str = "Windows.Media.Devices.CallControl";
}
::windows_core::imp::interface_hierarchy!(CallControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CallControl {}
unsafe impl ::core::marker::Sync for CallControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionInfo(::windows_core::IUnknown);
impl CameraOcclusionInfo {
    pub fn GetState(&self) -> ::windows_core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOcclusionKindSupported(&self, occlusionkind: CameraOcclusionKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOcclusionKindSupported)(::windows_core::Interface::as_raw(this), occlusionkind, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for CameraOcclusionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraOcclusionInfo {}
impl ::core::fmt::Debug for CameraOcclusionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraOcclusionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionInfo;{af6c4ad0-a84d-5db6-be58-a5da21cfe011})");
}
impl ::core::clone::Clone for CameraOcclusionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraOcclusionInfo {
    type Vtable = ICameraOcclusionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraOcclusionInfo {
    const IID: ::windows_core::GUID = <ICameraOcclusionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraOcclusionInfo {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionInfo";
}
::windows_core::imp::interface_hierarchy!(CameraOcclusionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CameraOcclusionInfo {}
unsafe impl ::core::marker::Sync for CameraOcclusionInfo {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionState(::windows_core::IUnknown);
impl CameraOcclusionState {
    pub fn IsOccluded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOccluded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOcclusionKind(&self, occlusionkind: CameraOcclusionKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOcclusionKind)(::windows_core::Interface::as_raw(this), occlusionkind, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CameraOcclusionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraOcclusionState {}
impl ::core::fmt::Debug for CameraOcclusionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraOcclusionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionState;{430adeb8-6842-5e55-9bde-04b4ef3a8a57})");
}
impl ::core::clone::Clone for CameraOcclusionState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraOcclusionState {
    type Vtable = ICameraOcclusionState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraOcclusionState {
    const IID: ::windows_core::GUID = <ICameraOcclusionState as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraOcclusionState {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionState";
}
::windows_core::imp::interface_hierarchy!(CameraOcclusionState, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CameraOcclusionState {}
unsafe impl ::core::marker::Sync for CameraOcclusionState {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CameraOcclusionStateChangedEventArgs(::windows_core::IUnknown);
impl CameraOcclusionStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<CameraOcclusionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CameraOcclusionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraOcclusionStateChangedEventArgs {}
impl ::core::fmt::Debug for CameraOcclusionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraOcclusionStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.CameraOcclusionStateChangedEventArgs;{8512d848-c0de-57ca-a1ca-fb2c3d23df55})");
}
impl ::core::clone::Clone for CameraOcclusionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraOcclusionStateChangedEventArgs {
    type Vtable = ICameraOcclusionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraOcclusionStateChangedEventArgs {
    const IID: ::windows_core::GUID = <ICameraOcclusionStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraOcclusionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.CameraOcclusionStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CameraOcclusionStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CameraOcclusionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for CameraOcclusionStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DefaultAudioCaptureDeviceChangedEventArgs(::windows_core::IUnknown);
impl DefaultAudioCaptureDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Role(&self) -> ::windows_core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Role)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DefaultAudioCaptureDeviceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultAudioCaptureDeviceChangedEventArgs {}
impl ::core::fmt::Debug for DefaultAudioCaptureDeviceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultAudioCaptureDeviceChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DefaultAudioCaptureDeviceChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs;{110f882f-1c05-4657-a18e-47c9b69f07ab})");
}
impl ::core::clone::Clone for DefaultAudioCaptureDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DefaultAudioCaptureDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DefaultAudioCaptureDeviceChangedEventArgs {
    const IID: ::windows_core::GUID = <IDefaultAudioDeviceChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DefaultAudioCaptureDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioCaptureDeviceChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DefaultAudioCaptureDeviceChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IDefaultAudioDeviceChangedEventArgs> for DefaultAudioCaptureDeviceChangedEventArgs {}
unsafe impl ::core::marker::Send for DefaultAudioCaptureDeviceChangedEventArgs {}
unsafe impl ::core::marker::Sync for DefaultAudioCaptureDeviceChangedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DefaultAudioRenderDeviceChangedEventArgs(::windows_core::IUnknown);
impl DefaultAudioRenderDeviceChangedEventArgs {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Role(&self) -> ::windows_core::Result<AudioDeviceRole> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Role)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DefaultAudioRenderDeviceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultAudioRenderDeviceChangedEventArgs {}
impl ::core::fmt::Debug for DefaultAudioRenderDeviceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultAudioRenderDeviceChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DefaultAudioRenderDeviceChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs;{110f882f-1c05-4657-a18e-47c9b69f07ab})");
}
impl ::core::clone::Clone for DefaultAudioRenderDeviceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DefaultAudioRenderDeviceChangedEventArgs {
    type Vtable = IDefaultAudioDeviceChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DefaultAudioRenderDeviceChangedEventArgs {
    const IID: ::windows_core::GUID = <IDefaultAudioDeviceChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DefaultAudioRenderDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DefaultAudioRenderDeviceChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DefaultAudioRenderDeviceChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IDefaultAudioDeviceChangedEventArgs> for DefaultAudioRenderDeviceChangedEventArgs {}
unsafe impl ::core::marker::Send for DefaultAudioRenderDeviceChangedEventArgs {}
unsafe impl ::core::marker::Sync for DefaultAudioRenderDeviceChangedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DialRequestedEventArgs(::windows_core::IUnknown);
impl DialRequestedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Contact(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DialRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialRequestedEventArgs {}
impl ::core::fmt::Debug for DialRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DialRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DialRequestedEventArgs;{037b929e-953c-4286-8866-4f0f376c855a})");
}
impl ::core::clone::Clone for DialRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DialRequestedEventArgs {
    type Vtable = IDialRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DialRequestedEventArgs {
    const IID: ::windows_core::GUID = <IDialRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.DialRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DialRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DialRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DialRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowBounds(::windows_core::IUnknown);
impl DigitalWindowBounds {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DigitalWindowBounds, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn NormalizedOriginTop(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedOriginTop)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNormalizedOriginTop(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalizedOriginTop)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalizedOriginLeft(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedOriginLeft)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNormalizedOriginLeft(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalizedOriginLeft)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for DigitalWindowBounds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DigitalWindowBounds {}
impl ::core::fmt::Debug for DigitalWindowBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowBounds").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DigitalWindowBounds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowBounds;{dd4f21dd-d173-5c6b-8c25-bdd26d5122b1})");
}
impl ::core::clone::Clone for DigitalWindowBounds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DigitalWindowBounds {
    type Vtable = IDigitalWindowBounds_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DigitalWindowBounds {
    const IID: ::windows_core::GUID = <IDigitalWindowBounds as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DigitalWindowBounds {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowBounds";
}
::windows_core::imp::interface_hierarchy!(DigitalWindowBounds, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DigitalWindowBounds {}
unsafe impl ::core::marker::Sync for DigitalWindowBounds {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowCapability(::windows_core::IUnknown);
impl DigitalWindowCapability {
    pub fn Width(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinScaleValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinScaleValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxScaleValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxScaleValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinScaleValueWithoutUpsampling(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinScaleValueWithoutUpsampling)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NormalizedFieldOfViewLimit(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalizedFieldOfViewLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DigitalWindowCapability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DigitalWindowCapability {}
impl ::core::fmt::Debug for DigitalWindowCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowCapability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DigitalWindowCapability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowCapability;{d78bad2c-f721-5244-a196-b56ccbec606c})");
}
impl ::core::clone::Clone for DigitalWindowCapability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DigitalWindowCapability {
    type Vtable = IDigitalWindowCapability_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DigitalWindowCapability {
    const IID: ::windows_core::GUID = <IDigitalWindowCapability as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DigitalWindowCapability {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowCapability";
}
::windows_core::imp::interface_hierarchy!(DigitalWindowCapability, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DigitalWindowCapability {}
unsafe impl ::core::marker::Sync for DigitalWindowCapability {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DigitalWindowControl(::windows_core::IUnknown);
impl DigitalWindowControl {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportedModes(&self) -> ::windows_core::Result<::windows_core::Array<DigitalWindowMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<DigitalWindowMode>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn CurrentMode(&self) -> ::windows_core::Result<DigitalWindowMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetBounds(&self) -> ::windows_core::Result<DigitalWindowBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBounds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configure(&self, digitalwindowmode: DigitalWindowMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Configure)(::windows_core::Interface::as_raw(this), digitalwindowmode).ok() }
    }
    pub fn ConfigureWithBounds<P0>(&self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<DigitalWindowBounds>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureWithBounds)(::windows_core::Interface::as_raw(this), digitalwindowmode, digitalwindowbounds.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCapabilities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCapabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCapabilityForSize(&self, width: i32, height: i32) -> ::windows_core::Result<DigitalWindowCapability> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCapabilityForSize)(::windows_core::Interface::as_raw(this), width, height, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DigitalWindowControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DigitalWindowControl {}
impl ::core::fmt::Debug for DigitalWindowControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DigitalWindowControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.DigitalWindowControl;{23b69eff-65d2-53ea-8780-de582b48b544})");
}
impl ::core::clone::Clone for DigitalWindowControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DigitalWindowControl {
    type Vtable = IDigitalWindowControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DigitalWindowControl {
    const IID: ::windows_core::GUID = <IDigitalWindowControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DigitalWindowControl {
    const NAME: &'static str = "Windows.Media.Devices.DigitalWindowControl";
}
::windows_core::imp::interface_hierarchy!(DigitalWindowControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DigitalWindowControl {}
unsafe impl ::core::marker::Sync for DigitalWindowControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ExposureCompensationControl(::windows_core::IUnknown);
impl ExposureCompensationControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, value: f32) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueAsync)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ExposureCompensationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExposureCompensationControl {}
impl ::core::fmt::Debug for ExposureCompensationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExposureCompensationControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExposureCompensationControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposureCompensationControl;{81c8e834-dcec-4011-a610-1f3847e64aca})");
}
impl ::core::clone::Clone for ExposureCompensationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ExposureCompensationControl {
    type Vtable = IExposureCompensationControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExposureCompensationControl {
    const IID: ::windows_core::GUID = <IExposureCompensationControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureCompensationControl";
}
::windows_core::imp::interface_hierarchy!(ExposureCompensationControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ExposureControl(::windows_core::IUnknown);
impl ExposureControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Auto(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Auto)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoAsync(&self, value: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAutoAsync)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Min(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Max(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Step(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, shutterduration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueAsync)(::windows_core::Interface::as_raw(this), shutterduration, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ExposureControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExposureControl {}
impl ::core::fmt::Debug for ExposureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExposureControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExposureControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposureControl;{09e8cbe2-ad96-4f28-a0e0-96ed7e1b5fd2})");
}
impl ::core::clone::Clone for ExposureControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ExposureControl {
    type Vtable = IExposureControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExposureControl {
    const IID: ::windows_core::GUID = <IExposureControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposureControl";
}
::windows_core::imp::interface_hierarchy!(ExposureControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ExposurePriorityVideoControl(::windows_core::IUnknown);
impl ExposurePriorityVideoControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ExposurePriorityVideoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExposurePriorityVideoControl {}
impl ::core::fmt::Debug for ExposurePriorityVideoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExposurePriorityVideoControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExposurePriorityVideoControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ExposurePriorityVideoControl;{2cb240a3-5168-4271-9ea5-47621a98a352})");
}
impl ::core::clone::Clone for ExposurePriorityVideoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ExposurePriorityVideoControl {
    type Vtable = IExposurePriorityVideoControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExposurePriorityVideoControl {
    const IID: ::windows_core::GUID = <IExposurePriorityVideoControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ExposurePriorityVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.ExposurePriorityVideoControl";
}
::windows_core::imp::interface_hierarchy!(ExposurePriorityVideoControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ExposurePriorityVideoControl {}
unsafe impl ::core::marker::Sync for ExposurePriorityVideoControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FlashControl(::windows_core::IUnknown);
impl FlashControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RedEyeReductionSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Auto(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Auto)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuto)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RedEyeReduction)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRedEyeReduction)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerPercent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPowerPercent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AssistantLightSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AssistantLightSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AssistantLightEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFlashControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AssistantLightEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAssistantLightEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFlashControl2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAssistantLightEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for FlashControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlashControl {}
impl ::core::fmt::Debug for FlashControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlashControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FlashControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FlashControl;{def41dbe-7d68-45e3-8c0f-be7bb32837d0})");
}
impl ::core::clone::Clone for FlashControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FlashControl {
    type Vtable = IFlashControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FlashControl {
    const IID: ::windows_core::GUID = <IFlashControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FlashControl {
    const NAME: &'static str = "Windows.Media.Devices.FlashControl";
}
::windows_core::imp::interface_hierarchy!(FlashControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusControl(::windows_core::IUnknown);
impl FocusControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPresets(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPresets)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Preset(&self) -> ::windows_core::Result<FocusPreset> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Preset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: FocusPreset) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPresetAsync)(::windows_core::Interface::as_raw(this), preset, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetWithCompletionOptionAsync(&self, preset: FocusPreset, completebeforefocus: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPresetWithCompletionOptionAsync)(::windows_core::Interface::as_raw(this), preset, completebeforefocus, &mut result__).from_abi(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, focus: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueAsync)(::windows_core::Interface::as_raw(this), focus, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FocusAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FocusChangedSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusChangedSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WaitForFocusSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WaitForFocusSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFocusModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusDistances(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFocusDistances)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFocusRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFocusRanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<FocusMode> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FocusState(&self) -> ::windows_core::Result<MediaCaptureFocusState> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnlockAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnlockAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LockAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configure<P0>(&self, settings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<FocusSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<IFocusControl2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Configure)(::windows_core::Interface::as_raw(this), settings.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for FocusControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusControl {}
impl ::core::fmt::Debug for FocusControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FocusControl;{c0d889f6-5228-4453-b153-85606592b238})");
}
impl ::core::clone::Clone for FocusControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FocusControl {
    type Vtable = IFocusControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusControl {
    const IID: ::windows_core::GUID = <IFocusControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusControl {
    const NAME: &'static str = "Windows.Media.Devices.FocusControl";
}
::windows_core::imp::interface_hierarchy!(FocusControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct FocusSettings(::windows_core::IUnknown);
impl FocusSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FocusSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows_core::Result<FocusMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: FocusMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoFocusRange(&self) -> ::windows_core::Result<AutoFocusRange> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoFocusRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoFocusRange(&self, value: AutoFocusRange) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoFocusRange)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Distance(&self) -> ::windows_core::Result<super::super::Foundation::IReference<ManualFocusDistance>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Distance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDistance<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<ManualFocusDistance>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDistance)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn WaitForFocus(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WaitForFocus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWaitForFocus(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWaitForFocus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableDriverFallback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableDriverFallback)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisableDriverFallback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisableDriverFallback)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for FocusSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusSettings {}
impl ::core::fmt::Debug for FocusSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.FocusSettings;{79958f6b-3263-4275-85d6-aeae891c96ee})");
}
impl ::core::clone::Clone for FocusSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FocusSettings {
    type Vtable = IFocusSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusSettings {
    const IID: ::windows_core::GUID = <IFocusSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusSettings {
    const NAME: &'static str = "Windows.Media.Devices.FocusSettings";
}
::windows_core::imp::interface_hierarchy!(FocusSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FocusSettings {}
unsafe impl ::core::marker::Sync for FocusSettings {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct HdrVideoControl(::windows_core::IUnknown);
impl HdrVideoControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<HdrVideoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: HdrVideoMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for HdrVideoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdrVideoControl {}
impl ::core::fmt::Debug for HdrVideoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdrVideoControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HdrVideoControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.HdrVideoControl;{55d8e2d0-30c0-43bf-9b9a-9799d70ced94})");
}
impl ::core::clone::Clone for HdrVideoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for HdrVideoControl {
    type Vtable = IHdrVideoControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HdrVideoControl {
    const IID: ::windows_core::GUID = <IHdrVideoControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HdrVideoControl {
    const NAME: &'static str = "Windows.Media.Devices.HdrVideoControl";
}
::windows_core::imp::interface_hierarchy!(HdrVideoControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for HdrVideoControl {}
unsafe impl ::core::marker::Sync for HdrVideoControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct InfraredTorchControl(::windows_core::IUnknown);
impl InfraredTorchControl {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentMode(&self) -> ::windows_core::Result<InfraredTorchMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCurrentMode(&self, value: InfraredTorchMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCurrentMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinPower(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinPower)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPower(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPower)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PowerStep(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerStep)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Power(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Power)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPower(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPower)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for InfraredTorchControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InfraredTorchControl {}
impl ::core::fmt::Debug for InfraredTorchControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InfraredTorchControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InfraredTorchControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.InfraredTorchControl;{1cba2c83-6cb6-5a04-a6fc-3be7b33ff056})");
}
impl ::core::clone::Clone for InfraredTorchControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InfraredTorchControl {
    type Vtable = IInfraredTorchControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InfraredTorchControl {
    const IID: ::windows_core::GUID = <IInfraredTorchControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InfraredTorchControl {
    const NAME: &'static str = "Windows.Media.Devices.InfraredTorchControl";
}
::windows_core::imp::interface_hierarchy!(InfraredTorchControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InfraredTorchControl {}
unsafe impl ::core::marker::Sync for InfraredTorchControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct IsoSpeedControl(::windows_core::IUnknown);
impl IsoSpeedControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SupportedPresets(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPresets)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Preset(&self) -> ::windows_core::Result<IsoSpeedPreset> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Preset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetPresetAsync(&self, preset: IsoSpeedPreset) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPresetAsync)(::windows_core::Interface::as_raw(this), preset, &mut result__).from_abi(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, isospeed: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueAsync)(::windows_core::Interface::as_raw(this), isospeed, &mut result__).from_abi(result__)
        }
    }
    pub fn Auto(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Auto)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IIsoSpeedControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAutoAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for IsoSpeedControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsoSpeedControl {}
impl ::core::fmt::Debug for IsoSpeedControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsoSpeedControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IsoSpeedControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.IsoSpeedControl;{27b6c322-25ad-4f1b-aaab-524ab376ca33})");
}
impl ::core::clone::Clone for IsoSpeedControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IsoSpeedControl {
    type Vtable = IIsoSpeedControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IsoSpeedControl {
    const IID: ::windows_core::GUID = <IIsoSpeedControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.IsoSpeedControl";
}
::windows_core::imp::interface_hierarchy!(IsoSpeedControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct KeypadPressedEventArgs(::windows_core::IUnknown);
impl KeypadPressedEventArgs {
    pub fn TelephonyKey(&self) -> ::windows_core::Result<TelephonyKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TelephonyKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for KeypadPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeypadPressedEventArgs {}
impl ::core::fmt::Debug for KeypadPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeypadPressedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeypadPressedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.KeypadPressedEventArgs;{d3a43900-b4fa-49cd-9442-89af6568f601})");
}
impl ::core::clone::Clone for KeypadPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for KeypadPressedEventArgs {
    type Vtable = IKeypadPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeypadPressedEventArgs {
    const IID: ::windows_core::GUID = <IKeypadPressedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeypadPressedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.KeypadPressedEventArgs";
}
::windows_core::imp::interface_hierarchy!(KeypadPressedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for KeypadPressedEventArgs {}
unsafe impl ::core::marker::Sync for KeypadPressedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoControl(::windows_core::IUnknown);
impl LowLagPhotoControl {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<P0>(&self, captureproperties: P0) -> ::windows_core::Result<super::MediaProperties::MediaRatio>
    where
        P0: ::windows_core::TryIntoParam<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHighestConcurrentFrameRate)(::windows_core::Interface::as_raw(this), captureproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentFrameRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ThumbnailEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> ::windows_core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredThumbnailSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredThumbnailSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareAcceleratedThumbnailSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoControl {}
impl ::core::fmt::Debug for LowLagPhotoControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LowLagPhotoControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.LowLagPhotoControl;{6d5c4dd0-fadf-415d-aee6-3baa529300c9})");
}
impl ::core::clone::Clone for LowLagPhotoControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LowLagPhotoControl {
    type Vtable = ILowLagPhotoControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LowLagPhotoControl {
    const IID: ::windows_core::GUID = <ILowLagPhotoControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LowLagPhotoControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoControl";
}
::windows_core::imp::interface_hierarchy!(LowLagPhotoControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoSequenceControl(::windows_core::IUnknown);
impl LowLagPhotoSequenceControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPastPhotos(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPastPhotos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPhotosPerSecond)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PastPhotoLimit(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PastPhotoLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPastPhotoLimit(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPastPhotoLimit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhotosPerSecondLimit(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotosPerSecondLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotosPerSecondLimit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<P0>(&self, captureproperties: P0) -> ::windows_core::Result<super::MediaProperties::MediaRatio>
    where
        P0: ::windows_core::TryIntoParam<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHighestConcurrentFrameRate)(::windows_core::Interface::as_raw(this), captureproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentFrameRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ThumbnailEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetThumbnailEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn ThumbnailFormat(&self) -> ::windows_core::Result<super::MediaProperties::MediaThumbnailFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ThumbnailFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredThumbnailSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredThumbnailSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredThumbnailSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareAcceleratedThumbnailSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoSequenceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoSequenceControl {}
impl ::core::fmt::Debug for LowLagPhotoSequenceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoSequenceControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LowLagPhotoSequenceControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.LowLagPhotoSequenceControl;{3dcf909d-6d16-409c-bafe-b9a594c6fde6})");
}
impl ::core::clone::Clone for LowLagPhotoSequenceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LowLagPhotoSequenceControl {
    type Vtable = ILowLagPhotoSequenceControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LowLagPhotoSequenceControl {
    const IID: ::windows_core::GUID = <ILowLagPhotoSequenceControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LowLagPhotoSequenceControl {
    const NAME: &'static str = "Windows.Media.Devices.LowLagPhotoSequenceControl";
}
::windows_core::imp::interface_hierarchy!(LowLagPhotoSequenceControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
pub struct MediaDevice;
impl MediaDevice {
    pub fn GetAudioCaptureSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioCaptureSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAudioRenderSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioRenderSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetVideoCaptureSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVideoCaptureSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultAudioCaptureId(role: AudioDeviceRole) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAudioCaptureId)(::windows_core::Interface::as_raw(this), role, &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefaultAudioRenderId(role: AudioDeviceRole) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAudioRenderId)(::windows_core::Interface::as_raw(this), role, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultAudioCaptureDeviceChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>,
    {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultAudioCaptureDeviceChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultAudioCaptureDeviceChanged(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveDefaultAudioCaptureDeviceChanged)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultAudioRenderDeviceChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>,
    {
        Self::IMediaDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultAudioRenderDeviceChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefaultAudioRenderDeviceChanged(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaDeviceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveDefaultAudioRenderDeviceChanged)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc(hidden)]
    pub fn IMediaDeviceStatics<R, F: FnOnce(&IMediaDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaDevice, IMediaDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MediaDevice {
    const NAME: &'static str = "Windows.Media.Devices.MediaDevice";
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaDeviceControl(::windows_core::IUnknown);
impl MediaDeviceControl {
    pub fn Capabilities(&self) -> ::windows_core::Result<MediaDeviceControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetValue(&self, value: &mut f64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetValue)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue(&self, value: f64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAuto(&self, value: &mut bool) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAuto)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetAuto(&self, value: bool) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetAuto)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaDeviceControl {}
impl ::core::fmt::Debug for MediaDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDeviceControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaDeviceControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.MediaDeviceControl;{efa8dfa9-6f75-4863-ba0b-583f3036b4de})");
}
impl ::core::clone::Clone for MediaDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaDeviceControl {
    type Vtable = IMediaDeviceControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaDeviceControl {
    const IID: ::windows_core::GUID = <IMediaDeviceControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaDeviceControl {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControl";
}
::windows_core::imp::interface_hierarchy!(MediaDeviceControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct MediaDeviceControlCapabilities(::windows_core::IUnknown);
impl MediaDeviceControlCapabilities {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Default(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Default)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AutoModeSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoModeSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaDeviceControlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaDeviceControlCapabilities {}
impl ::core::fmt::Debug for MediaDeviceControlCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDeviceControlCapabilities").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaDeviceControlCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.MediaDeviceControlCapabilities;{23005816-eb85-43e2-b92b-8240d5ee70ec})");
}
impl ::core::clone::Clone for MediaDeviceControlCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaDeviceControlCapabilities {
    type Vtable = IMediaDeviceControlCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaDeviceControlCapabilities {
    const IID: ::windows_core::GUID = <IMediaDeviceControlCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaDeviceControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.MediaDeviceControlCapabilities";
}
::windows_core::imp::interface_hierarchy!(MediaDeviceControlCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ModuleCommandResult(::windows_core::IUnknown);
impl ModuleCommandResult {
    pub fn Status(&self) -> ::windows_core::Result<SendCommandStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ModuleCommandResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ModuleCommandResult {}
impl ::core::fmt::Debug for ModuleCommandResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ModuleCommandResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ModuleCommandResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ModuleCommandResult;{520d1eb4-1374-4c7d-b1e4-39dcdf3eae4e})");
}
impl ::core::clone::Clone for ModuleCommandResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ModuleCommandResult {
    type Vtable = IModuleCommandResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ModuleCommandResult {
    const IID: ::windows_core::GUID = <IModuleCommandResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ModuleCommandResult {
    const NAME: &'static str = "Windows.Media.Devices.ModuleCommandResult";
}
::windows_core::imp::interface_hierarchy!(ModuleCommandResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct OpticalImageStabilizationControl(::windows_core::IUnknown);
impl OpticalImageStabilizationControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<OpticalImageStabilizationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: OpticalImageStabilizationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for OpticalImageStabilizationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OpticalImageStabilizationControl {}
impl ::core::fmt::Debug for OpticalImageStabilizationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpticalImageStabilizationControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for OpticalImageStabilizationControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.OpticalImageStabilizationControl;{bfad9c1d-00bc-423b-8eb2-a0178ca94247})");
}
impl ::core::clone::Clone for OpticalImageStabilizationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OpticalImageStabilizationControl {
    type Vtable = IOpticalImageStabilizationControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OpticalImageStabilizationControl {
    const IID: ::windows_core::GUID = <IOpticalImageStabilizationControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OpticalImageStabilizationControl {
    const NAME: &'static str = "Windows.Media.Devices.OpticalImageStabilizationControl";
}
::windows_core::imp::interface_hierarchy!(OpticalImageStabilizationControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OpticalImageStabilizationControl {}
unsafe impl ::core::marker::Sync for OpticalImageStabilizationControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct PanelBasedOptimizationControl(::windows_core::IUnknown);
impl PanelBasedOptimizationControl {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Panel(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Panel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPanel(&self, value: super::super::Devices::Enumeration::Panel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPanel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for PanelBasedOptimizationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PanelBasedOptimizationControl {}
impl ::core::fmt::Debug for PanelBasedOptimizationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PanelBasedOptimizationControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PanelBasedOptimizationControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.PanelBasedOptimizationControl;{33323223-6247-5419-a5a4-3d808645d917})");
}
impl ::core::clone::Clone for PanelBasedOptimizationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PanelBasedOptimizationControl {
    type Vtable = IPanelBasedOptimizationControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PanelBasedOptimizationControl {
    const IID: ::windows_core::GUID = <IPanelBasedOptimizationControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PanelBasedOptimizationControl {
    const NAME: &'static str = "Windows.Media.Devices.PanelBasedOptimizationControl";
}
::windows_core::imp::interface_hierarchy!(PanelBasedOptimizationControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PanelBasedOptimizationControl {}
unsafe impl ::core::marker::Sync for PanelBasedOptimizationControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct PhotoConfirmationControl(::windows_core::IUnknown);
impl PhotoConfirmationControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PixelFormat(&self) -> ::windows_core::Result<super::MediaProperties::MediaPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPixelFormat(&self, format: super::MediaProperties::MediaPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPixelFormat)(::windows_core::Interface::as_raw(this), format).ok() }
    }
}
impl ::core::cmp::PartialEq for PhotoConfirmationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoConfirmationControl {}
impl ::core::fmt::Debug for PhotoConfirmationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoConfirmationControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhotoConfirmationControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.PhotoConfirmationControl;{c8f3f363-ff5e-4582-a9a8-0550f85a4a76})");
}
impl ::core::clone::Clone for PhotoConfirmationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhotoConfirmationControl {
    type Vtable = IPhotoConfirmationControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhotoConfirmationControl {
    const IID: ::windows_core::GUID = <IPhotoConfirmationControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhotoConfirmationControl {
    const NAME: &'static str = "Windows.Media.Devices.PhotoConfirmationControl";
}
::windows_core::imp::interface_hierarchy!(PhotoConfirmationControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RedialRequestedEventArgs(::windows_core::IUnknown);
impl RedialRequestedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for RedialRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RedialRequestedEventArgs {}
impl ::core::fmt::Debug for RedialRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RedialRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RedialRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RedialRequestedEventArgs;{7eb55209-76ab-4c31-b40e-4b58379d580c})");
}
impl ::core::clone::Clone for RedialRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RedialRequestedEventArgs {
    type Vtable = IRedialRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RedialRequestedEventArgs {
    const IID: ::windows_core::GUID = <IRedialRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RedialRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.RedialRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(RedialRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RedialRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RedialRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RegionOfInterest(::windows_core::IUnknown);
impl RegionOfInterest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RegionOfInterest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AutoFocusEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoFocusEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoFocusEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoFocusEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoWhiteBalanceEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoWhiteBalanceEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoWhiteBalanceEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoWhiteBalanceEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutoExposureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoExposureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoExposureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoExposureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBounds(&self, value: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBounds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<RegionOfInterestType> {
        let this = &::windows_core::ComInterface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: RegionOfInterestType) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BoundsNormalized(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundsNormalized)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBoundsNormalized(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBoundsNormalized)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Weight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IRegionOfInterest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Weight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRegionOfInterest2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for RegionOfInterest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegionOfInterest {}
impl ::core::fmt::Debug for RegionOfInterest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegionOfInterest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RegionOfInterest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RegionOfInterest;{e5ecc834-ce66-4e05-a78f-cf391a5ec2d1})");
}
impl ::core::clone::Clone for RegionOfInterest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RegionOfInterest {
    type Vtable = IRegionOfInterest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RegionOfInterest {
    const IID: ::windows_core::GUID = <IRegionOfInterest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RegionOfInterest {
    const NAME: &'static str = "Windows.Media.Devices.RegionOfInterest";
}
::windows_core::imp::interface_hierarchy!(RegionOfInterest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RegionOfInterest {}
unsafe impl ::core::marker::Sync for RegionOfInterest {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RegionsOfInterestControl(::windows_core::IUnknown);
impl RegionsOfInterestControl {
    pub fn MaxRegions(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxRegions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRegionsAsync<P0>(&self, regions: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<RegionOfInterest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetRegionsAsync)(::windows_core::Interface::as_raw(this), regions.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetRegionsWithLockAsync<P0>(&self, regions: P0, lockvalues: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<RegionOfInterest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetRegionsWithLockAsync)(::windows_core::Interface::as_raw(this), regions.try_into_param()?.abi(), lockvalues, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearRegionsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearRegionsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AutoFocusSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoFocusSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AutoWhiteBalanceSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoWhiteBalanceSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AutoExposureSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoExposureSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RegionsOfInterestControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegionsOfInterestControl {}
impl ::core::fmt::Debug for RegionsOfInterestControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegionsOfInterestControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RegionsOfInterestControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.RegionsOfInterestControl;{c323f527-ab0b-4558-8b5b-df5693db0378})");
}
impl ::core::clone::Clone for RegionsOfInterestControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RegionsOfInterestControl {
    type Vtable = IRegionsOfInterestControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RegionsOfInterestControl {
    const IID: ::windows_core::GUID = <IRegionsOfInterestControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RegionsOfInterestControl {
    const NAME: &'static str = "Windows.Media.Devices.RegionsOfInterestControl";
}
::windows_core::imp::interface_hierarchy!(RegionsOfInterestControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct SceneModeControl(::windows_core::IUnknown);
impl SceneModeControl {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<CaptureSceneMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, scenemode: CaptureSceneMode) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueAsync)(::windows_core::Interface::as_raw(this), scenemode, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SceneModeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneModeControl {}
impl ::core::fmt::Debug for SceneModeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneModeControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneModeControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.SceneModeControl;{d48e5af7-8d59-4854-8c62-12c70ba89b7c})");
}
impl ::core::clone::Clone for SceneModeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneModeControl {
    type Vtable = ISceneModeControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneModeControl {
    const IID: ::windows_core::GUID = <ISceneModeControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneModeControl {
    const NAME: &'static str = "Windows.Media.Devices.SceneModeControl";
}
::windows_core::imp::interface_hierarchy!(SceneModeControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct TorchControl(::windows_core::IUnknown);
impl TorchControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerPercent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPowerPercent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for TorchControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TorchControl {}
impl ::core::fmt::Debug for TorchControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TorchControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TorchControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.TorchControl;{a6053665-8250-416c-919a-724296afa306})");
}
impl ::core::clone::Clone for TorchControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TorchControl {
    type Vtable = ITorchControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TorchControl {
    const IID: ::windows_core::GUID = <ITorchControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TorchControl {
    const NAME: &'static str = "Windows.Media.Devices.TorchControl";
}
::windows_core::imp::interface_hierarchy!(TorchControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceController(::windows_core::IUnknown);
impl VideoDeviceController {
    pub fn SetDeviceProperty<P0>(&self, propertyid: &::windows_core::HSTRING, propertyvalue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceProperty)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyid), propertyvalue.into_param().abi()).ok() }
    }
    pub fn GetDeviceProperty(&self, propertyid: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceProperty)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyid), &mut result__).from_abi(result__)
        }
    }
    pub fn CameraOcclusionInfo(&self) -> ::windows_core::Result<CameraOcclusionInfo> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController10>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraOcclusionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn TryAcquireExclusiveControl(&self, deviceid: &::windows_core::HSTRING, mode: super::Capture::MediaCaptureDeviceExclusiveControlReleaseMode) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController11>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryAcquireExclusiveControl)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), mode, &mut result__).from_abi(result__)
        }
    }
    pub fn LowLagPhotoSequence(&self) -> ::windows_core::Result<LowLagPhotoSequenceControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LowLagPhotoSequence)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LowLagPhoto(&self) -> ::windows_core::Result<LowLagPhotoControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LowLagPhoto)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SceneModeControl(&self) -> ::windows_core::Result<SceneModeControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SceneModeControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TorchControl(&self) -> ::windows_core::Result<TorchControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TorchControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FlashControl(&self) -> ::windows_core::Result<FlashControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlashControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WhiteBalanceControl(&self) -> ::windows_core::Result<WhiteBalanceControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WhiteBalanceControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExposureControl(&self) -> ::windows_core::Result<ExposureControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExposureControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FocusControl(&self) -> ::windows_core::Result<FocusControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExposureCompensationControl(&self) -> ::windows_core::Result<ExposureCompensationControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensationControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsoSpeedControl(&self) -> ::windows_core::Result<IsoSpeedControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsoSpeedControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegionsOfInterestControl(&self) -> ::windows_core::Result<RegionsOfInterestControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegionsOfInterestControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrimaryUse(&self) -> ::windows_core::Result<CaptureUse> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryUse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPrimaryUse(&self, value: CaptureUse) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPrimaryUse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Devices_Core\"`*"]
    #[cfg(feature = "Media_Devices_Core")]
    pub fn VariablePhotoSequenceController(&self) -> ::windows_core::Result<Core::VariablePhotoSequenceController> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VariablePhotoSequenceController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhotoConfirmationControl(&self) -> ::windows_core::Result<PhotoConfirmationControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoConfirmationControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ZoomControl(&self) -> ::windows_core::Result<ZoomControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExposurePriorityVideoControl(&self) -> ::windows_core::Result<ExposurePriorityVideoControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExposurePriorityVideoControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DesiredOptimization(&self) -> ::windows_core::Result<MediaCaptureOptimization> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredOptimization)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredOptimization(&self, value: MediaCaptureOptimization) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredOptimization)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HdrVideoControl(&self) -> ::windows_core::Result<HdrVideoControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HdrVideoControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpticalImageStabilizationControl(&self) -> ::windows_core::Result<OpticalImageStabilizationControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpticalImageStabilizationControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdvancedPhotoControl(&self) -> ::windows_core::Result<AdvancedPhotoControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdvancedPhotoControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDevicePropertyById<P0>(&self, propertyid: &::windows_core::HSTRING, maxpropertyvaluesize: P0) -> ::windows_core::Result<VideoDeviceControllerGetDevicePropertyResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDevicePropertyById)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyid), maxpropertyvaluesize.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDevicePropertyById<P0>(&self, propertyid: &::windows_core::HSTRING, propertyvalue: P0) -> ::windows_core::Result<VideoDeviceControllerSetDevicePropertyStatus>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetDevicePropertyById)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyid), propertyvalue.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDevicePropertyByExtendedId<P0>(&self, extendedpropertyid: &[u8], maxpropertyvaluesize: P0) -> ::windows_core::Result<VideoDeviceControllerGetDevicePropertyResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDevicePropertyByExtendedId)(::windows_core::Interface::as_raw(this), extendedpropertyid.len() as u32, extendedpropertyid.as_ptr(), maxpropertyvaluesize.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDevicePropertyByExtendedId(&self, extendedpropertyid: &[u8], propertyvalue: &[u8]) -> ::windows_core::Result<VideoDeviceControllerSetDevicePropertyStatus> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetDevicePropertyByExtendedId)(::windows_core::Interface::as_raw(this), extendedpropertyid.len() as u32, extendedpropertyid.as_ptr(), propertyvalue.len() as u32, propertyvalue.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoTemporalDenoisingControl(&self) -> ::windows_core::Result<VideoTemporalDenoisingControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoTemporalDenoisingControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InfraredTorchControl(&self) -> ::windows_core::Result<InfraredTorchControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InfraredTorchControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PanelBasedOptimizationControl(&self) -> ::windows_core::Result<PanelBasedOptimizationControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PanelBasedOptimizationControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DigitalWindowControl(&self) -> ::windows_core::Result<DigitalWindowControl> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedVideoCaptureDeviceController9>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DigitalWindowControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>> {
        let this = &::windows_core::ComInterface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableMediaStreamProperties)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows_core::Result<super::MediaProperties::IMediaEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMediaStreamProperties)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn SetMediaStreamPropertiesAsync<P0>(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::MediaProperties::IMediaEncodingProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaDeviceController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetMediaStreamPropertiesAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Brightness(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Brightness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contrast(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Hue(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WhiteBalance(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WhiteBalance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BacklightCompensation(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BacklightCompensation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Pan(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Tilt(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tilt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Zoom(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Zoom)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Roll(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Roll)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Exposure(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Exposure)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Focus(&self) -> ::windows_core::Result<MediaDeviceControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Focus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn TrySetPowerlineFrequency(&self, value: super::Capture::PowerlineFrequency) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetPowerlineFrequency)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn TryGetPowerlineFrequency(&self, value: &mut super::Capture::PowerlineFrequency) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPowerlineFrequency)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoDeviceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoDeviceController {}
impl ::core::fmt::Debug for VideoDeviceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceController").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoDeviceController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoDeviceController;{99555575-2e2e-40b8-b6c7-f82d10013210})");
}
impl ::core::clone::Clone for VideoDeviceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoDeviceController {
    type Vtable = IVideoDeviceController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoDeviceController {
    const IID: ::windows_core::GUID = <IVideoDeviceController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceController";
}
::windows_core::imp::interface_hierarchy!(VideoDeviceController, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaDeviceController> for VideoDeviceController {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoDeviceControllerGetDevicePropertyResult(::windows_core::IUnknown);
impl VideoDeviceControllerGetDevicePropertyResult {
    pub fn Status(&self) -> ::windows_core::Result<VideoDeviceControllerGetDevicePropertyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoDeviceControllerGetDevicePropertyResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoDeviceControllerGetDevicePropertyResult {}
impl ::core::fmt::Debug for VideoDeviceControllerGetDevicePropertyResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerGetDevicePropertyResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoDeviceControllerGetDevicePropertyResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult;{c5d88395-6ed5-4790-8b5d-0ef13935d0f8})");
}
impl ::core::clone::Clone for VideoDeviceControllerGetDevicePropertyResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoDeviceControllerGetDevicePropertyResult {
    type Vtable = IVideoDeviceControllerGetDevicePropertyResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoDeviceControllerGetDevicePropertyResult {
    const IID: ::windows_core::GUID = <IVideoDeviceControllerGetDevicePropertyResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoDeviceControllerGetDevicePropertyResult {
    const NAME: &'static str = "Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyResult";
}
::windows_core::imp::interface_hierarchy!(VideoDeviceControllerGetDevicePropertyResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoDeviceControllerGetDevicePropertyResult {}
unsafe impl ::core::marker::Sync for VideoDeviceControllerGetDevicePropertyResult {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct VideoTemporalDenoisingControl(::windows_core::IUnknown);
impl VideoTemporalDenoisingControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<VideoTemporalDenoisingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: VideoTemporalDenoisingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for VideoTemporalDenoisingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTemporalDenoisingControl {}
impl ::core::fmt::Debug for VideoTemporalDenoisingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTemporalDenoisingControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoTemporalDenoisingControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.VideoTemporalDenoisingControl;{7ab34735-3e2a-4a32-baff-4358c4fbdd57})");
}
impl ::core::clone::Clone for VideoTemporalDenoisingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoTemporalDenoisingControl {
    type Vtable = IVideoTemporalDenoisingControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoTemporalDenoisingControl {
    const IID: ::windows_core::GUID = <IVideoTemporalDenoisingControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoTemporalDenoisingControl {
    const NAME: &'static str = "Windows.Media.Devices.VideoTemporalDenoisingControl";
}
::windows_core::imp::interface_hierarchy!(VideoTemporalDenoisingControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoTemporalDenoisingControl {}
unsafe impl ::core::marker::Sync for VideoTemporalDenoisingControl {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct WhiteBalanceControl(::windows_core::IUnknown);
impl WhiteBalanceControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Preset(&self) -> ::windows_core::Result<ColorTemperaturePreset> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Preset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPresetAsync(&self, preset: ColorTemperaturePreset) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPresetAsync)(::windows_core::Interface::as_raw(this), preset, &mut result__).from_abi(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValueAsync(&self, temperature: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueAsync)(::windows_core::Interface::as_raw(this), temperature, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WhiteBalanceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WhiteBalanceControl {}
impl ::core::fmt::Debug for WhiteBalanceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WhiteBalanceControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WhiteBalanceControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.WhiteBalanceControl;{781f047e-7162-49c8-a8f9-9481c565363e})");
}
impl ::core::clone::Clone for WhiteBalanceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WhiteBalanceControl {
    type Vtable = IWhiteBalanceControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WhiteBalanceControl {
    const IID: ::windows_core::GUID = <IWhiteBalanceControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WhiteBalanceControl {
    const NAME: &'static str = "Windows.Media.Devices.WhiteBalanceControl";
}
::windows_core::imp::interface_hierarchy!(WhiteBalanceControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ZoomControl(::windows_core::IUnknown);
impl ZoomControl {
    pub fn Supported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Supported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Min(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Max(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Step(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Step)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>> {
        let this = &::windows_core::ComInterface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedModes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<ZoomTransitionMode> {
        let this = &::windows_core::ComInterface::cast::<IZoomControl2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configure<P0>(&self, settings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ZoomSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<IZoomControl2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Configure)(::windows_core::Interface::as_raw(this), settings.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for ZoomControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ZoomControl {}
impl ::core::fmt::Debug for ZoomControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ZoomControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ZoomControl;{3a1e0b12-32da-4c17-bfd7-8d0c73c8f5a5})");
}
impl ::core::clone::Clone for ZoomControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ZoomControl {
    type Vtable = IZoomControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ZoomControl {
    const IID: ::windows_core::GUID = <IZoomControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ZoomControl {
    const NAME: &'static str = "Windows.Media.Devices.ZoomControl";
}
::windows_core::imp::interface_hierarchy!(ZoomControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct ZoomSettings(::windows_core::IUnknown);
impl ZoomSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ZoomSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows_core::Result<ZoomTransitionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: ZoomTransitionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for ZoomSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ZoomSettings {}
impl ::core::fmt::Debug for ZoomSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ZoomSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.ZoomSettings;{6ad66b24-14b4-4bfd-b18f-88fe24463b52})");
}
impl ::core::clone::Clone for ZoomSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ZoomSettings {
    type Vtable = IZoomSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ZoomSettings {
    const IID: ::windows_core::GUID = <IZoomSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ZoomSettings {
    const NAME: &'static str = "Windows.Media.Devices.ZoomSettings";
}
::windows_core::imp::interface_hierarchy!(ZoomSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ZoomSettings {}
unsafe impl ::core::marker::Sync for ZoomSettings {}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for AdvancedPhotoMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AdvancedPhotoMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AdvancedPhotoMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AdvancedPhotoMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AdvancedPhotoMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for AudioDeviceRole {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioDeviceRole {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioDeviceRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceRole").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDeviceRole {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AudioDeviceRole;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for AutoFocusRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutoFocusRange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutoFocusRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoFocusRange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutoFocusRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.AutoFocusRange;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CameraOcclusionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraOcclusionKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraOcclusionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraOcclusionKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraOcclusionKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraOcclusionKind;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CameraStreamState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraStreamState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraStreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraStreamState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraStreamState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CameraStreamState;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CaptureSceneMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CaptureSceneMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CaptureSceneMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaptureSceneMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CaptureSceneMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureSceneMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CaptureUse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CaptureUse {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CaptureUse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CaptureUse").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CaptureUse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.CaptureUse;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ColorTemperaturePreset {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ColorTemperaturePreset {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ColorTemperaturePreset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorTemperaturePreset").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ColorTemperaturePreset {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ColorTemperaturePreset;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for DigitalWindowMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DigitalWindowMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DigitalWindowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DigitalWindowMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DigitalWindowMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.DigitalWindowMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for FocusMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FocusMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FocusMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for FocusPreset {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FocusPreset {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FocusPreset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusPreset").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusPreset {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.FocusPreset;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for HdrVideoMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HdrVideoMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HdrVideoMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdrVideoMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HdrVideoMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.HdrVideoMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for InfraredTorchMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InfraredTorchMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InfraredTorchMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InfraredTorchMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InfraredTorchMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.InfraredTorchMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IsoSpeedPreset(pub i32);
#[cfg(feature = "deprecated")]
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
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for IsoSpeedPreset {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IsoSpeedPreset {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for IsoSpeedPreset {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for IsoSpeedPreset {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IsoSpeedPreset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsoSpeedPreset").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for IsoSpeedPreset {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.IsoSpeedPreset;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ManualFocusDistance {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ManualFocusDistance {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ManualFocusDistance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManualFocusDistance").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ManualFocusDistance {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ManualFocusDistance;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for MediaCaptureFocusState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureFocusState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureFocusState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFocusState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureFocusState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureFocusState;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for MediaCaptureOptimization {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureOptimization {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureOptimization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureOptimization").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureOptimization {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCaptureOptimization;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for MediaCapturePauseBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCapturePauseBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCapturePauseBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCapturePauseBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCapturePauseBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.MediaCapturePauseBehavior;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for OpticalImageStabilizationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OpticalImageStabilizationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OpticalImageStabilizationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpticalImageStabilizationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for OpticalImageStabilizationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.OpticalImageStabilizationMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for RegionOfInterestType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RegionOfInterestType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RegionOfInterestType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegionOfInterestType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RegionOfInterestType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.RegionOfInterestType;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for SendCommandStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SendCommandStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SendCommandStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SendCommandStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SendCommandStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.SendCommandStatus;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for TelephonyKey {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TelephonyKey {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TelephonyKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TelephonyKey").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TelephonyKey {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.TelephonyKey;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VideoDeviceControllerGetDevicePropertyStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VideoDeviceControllerGetDevicePropertyStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoDeviceControllerGetDevicePropertyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerGetDevicePropertyStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoDeviceControllerGetDevicePropertyStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerGetDevicePropertyStatus;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VideoDeviceControllerSetDevicePropertyStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VideoDeviceControllerSetDevicePropertyStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoDeviceControllerSetDevicePropertyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceControllerSetDevicePropertyStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoDeviceControllerSetDevicePropertyStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoDeviceControllerSetDevicePropertyStatus;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VideoTemporalDenoisingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VideoTemporalDenoisingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoTemporalDenoisingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTemporalDenoisingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoTemporalDenoisingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.VideoTemporalDenoisingMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ZoomTransitionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ZoomTransitionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ZoomTransitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomTransitionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ZoomTransitionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.ZoomTransitionMode;i4)");
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct CallControlEventHandler(pub ::windows_core::IUnknown);
impl CallControlEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&CallControl>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = CallControlEventHandlerBox::<F> { vtable: &CallControlEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CallControl>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct CallControlEventHandlerBox<F: FnMut(::core::option::Option<&CallControl>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const CallControlEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&CallControl>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> CallControlEventHandlerBox<F> {
    const VTABLE: CallControlEventHandler_Vtbl = CallControlEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<CallControlEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender)).into()
    }
}
impl ::core::cmp::PartialEq for CallControlEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CallControlEventHandler {}
impl ::core::fmt::Debug for CallControlEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CallControlEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for CallControlEventHandler {
    type Vtable = CallControlEventHandler_Vtbl;
}
impl ::core::clone::Clone for CallControlEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for CallControlEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x596f759f_50df_4454_bc63_4d3d01b61958);
}
impl ::windows_core::RuntimeType for CallControlEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{596f759f-50df-4454-bc63-4d3d01b61958}");
}
#[repr(C)]
#[doc(hidden)]
pub struct CallControlEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct DialRequestedEventHandler(pub ::windows_core::IUnknown);
impl DialRequestedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&DialRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DialRequestedEventHandlerBox::<F> { vtable: &DialRequestedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CallControl>,
        P1: ::windows_core::IntoParam<DialRequestedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DialRequestedEventHandlerBox<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&DialRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DialRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&DialRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DialRequestedEventHandlerBox<F> {
    const VTABLE: DialRequestedEventHandler_Vtbl = DialRequestedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DialRequestedEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
impl ::core::cmp::PartialEq for DialRequestedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialRequestedEventHandler {}
impl ::core::fmt::Debug for DialRequestedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialRequestedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DialRequestedEventHandler {
    type Vtable = DialRequestedEventHandler_Vtbl;
}
impl ::core::clone::Clone for DialRequestedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for DialRequestedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5abbffdb_c21f_4bc4_891b_257e28c1b1a4);
}
impl ::windows_core::RuntimeType for DialRequestedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5abbffdb-c21f-4bc4-891b-257e28c1b1a4}");
}
#[repr(C)]
#[doc(hidden)]
pub struct DialRequestedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct KeypadPressedEventHandler(pub ::windows_core::IUnknown);
impl KeypadPressedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&KeypadPressedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = KeypadPressedEventHandlerBox::<F> { vtable: &KeypadPressedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CallControl>,
        P1: ::windows_core::IntoParam<KeypadPressedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct KeypadPressedEventHandlerBox<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&KeypadPressedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const KeypadPressedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&KeypadPressedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> KeypadPressedEventHandlerBox<F> {
    const VTABLE: KeypadPressedEventHandler_Vtbl = KeypadPressedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<KeypadPressedEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
impl ::core::cmp::PartialEq for KeypadPressedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeypadPressedEventHandler {}
impl ::core::fmt::Debug for KeypadPressedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeypadPressedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for KeypadPressedEventHandler {
    type Vtable = KeypadPressedEventHandler_Vtbl;
}
impl ::core::clone::Clone for KeypadPressedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for KeypadPressedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe637a454_c527_422c_8926_c9af83b559a0);
}
impl ::windows_core::RuntimeType for KeypadPressedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e637a454-c527-422c-8926-c9af83b559a0}");
}
#[repr(C)]
#[doc(hidden)]
pub struct KeypadPressedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Devices\"`*"]
#[repr(transparent)]
pub struct RedialRequestedEventHandler(pub ::windows_core::IUnknown);
impl RedialRequestedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&RedialRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RedialRequestedEventHandlerBox::<F> { vtable: &RedialRequestedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CallControl>,
        P1: ::windows_core::IntoParam<RedialRequestedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RedialRequestedEventHandlerBox<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&RedialRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RedialRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&CallControl>, ::core::option::Option<&RedialRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RedialRequestedEventHandlerBox<F> {
    const VTABLE: RedialRequestedEventHandler_Vtbl = RedialRequestedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<RedialRequestedEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
impl ::core::cmp::PartialEq for RedialRequestedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RedialRequestedEventHandler {}
impl ::core::fmt::Debug for RedialRequestedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RedialRequestedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RedialRequestedEventHandler {
    type Vtable = RedialRequestedEventHandler_Vtbl;
}
impl ::core::clone::Clone for RedialRequestedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for RedialRequestedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbaf257d1_4ebd_4b84_9f47_6ec43d75d8b1);
}
impl ::windows_core::RuntimeType for RedialRequestedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{baf257d1-4ebd-4b84-9f47-6ec43d75d8b1}");
}
#[repr(C)]
#[doc(hidden)]
pub struct RedialRequestedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
