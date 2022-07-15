#[cfg(feature = "Devices_Perception_Provider")]
pub mod Provider;
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownCameraIntrinsicsPropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownCameraIntrinsicsPropertiesStatics {
    type Vtable = IKnownCameraIntrinsicsPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c03978_437a_4d97_a663_fd3195600249);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownCameraIntrinsicsPropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FocalLength: usize,
    #[cfg(feature = "deprecated")]
    pub PrincipalPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PrincipalPoint: usize,
    #[cfg(feature = "deprecated")]
    pub RadialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RadialDistortion: usize,
    #[cfg(feature = "deprecated")]
    pub TangentialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TangentialDistortion: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionColorFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionColorFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Exposure: usize,
    #[cfg(feature = "deprecated")]
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoExposureEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExposureCompensation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionDepthFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionDepthFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de4a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub MinDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MinDepth: usize,
    #[cfg(feature = "deprecated")]
    pub MaxDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxDepth: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de47);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub PhysicalDeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhysicalDeviceIds: usize,
    #[cfg(feature = "deprecated")]
    pub FrameKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameKind: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceModelVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceModelVersion: usize,
    #[cfg(feature = "deprecated")]
    pub EnclosureLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EnclosureLocation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionFrameSourcePropertiesStatics2 {
    type Vtable = IKnownPerceptionFrameSourcePropertiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9c86871_05dc_4a4d_8a5c_a4ecf26bbc46);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionInfraredFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionInfraredFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de49);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Exposure: usize,
    #[cfg(feature = "deprecated")]
    pub AutoExposureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoExposureEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExposureCompensation: usize,
    #[cfg(feature = "deprecated")]
    pub ActiveIlluminationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ActiveIlluminationEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub AmbientSubtractionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AmbientSubtractionEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub StructureLightPatternEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StructureLightPatternEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub InterleavedIlluminationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InterleavedIlluminationEnabled: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionVideoFrameSourcePropertiesStatics {
    type Vtable = IKnownPerceptionVideoFrameSourcePropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df1cca2_01f8_4a87_b859_d5e5b7e1de48);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(feature = "deprecated")]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportedVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub IsMirrored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsMirrored: usize,
    #[cfg(feature = "deprecated")]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CameraIntrinsics: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IKnownPerceptionVideoProfilePropertiesStatics {
    type Vtable = IKnownPerceptionVideoProfilePropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f08e2e7_5a76_43e3_a13a_da3d91a9ef98);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "deprecated")]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BitmapAlphaMode: usize,
    #[cfg(feature = "deprecated")]
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Width: usize,
    #[cfg(feature = "deprecated")]
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Height: usize,
    #[cfg(feature = "deprecated")]
    pub FrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameDuration: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrame(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrame {
    type Vtable = IPerceptionColorFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe621549_2cbf_4f94_9861_f817ea317747);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrame_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameArrivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameArrivedEventArgs {
    type Vtable = IPerceptionColorFrameArrivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fad02d5_86f7_4d8d_b966_5a3761ba9f59);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(feature = "deprecated")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryOpenFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameReader(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameReader {
    type Vtable = IPerceptionColorFrameReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7650f56e_b9f5_461b_83ad_f222af2aaadc);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameReader_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameArrived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "deprecated")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Source: usize,
    #[cfg(feature = "deprecated")]
    pub IsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryReadLatestFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameSource {
    type Vtable = IPerceptionColorFrameSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc6dba7c_0b58_468d_9ca1_6db04cc0477c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CameraIntrinsicsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(feature = "deprecated")]
    pub Active: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Active: usize,
    #[cfg(feature = "deprecated")]
    pub IsControlled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsControlled: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "deprecated")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "deprecated")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AcquireControlSession: usize,
    #[cfg(feature = "deprecated")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "deprecated")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    TryGetTransformTo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlateddepthframesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetsourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlateddepthframesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlsession: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub OpenReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OpenReader: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSource2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameSource2 {
    type Vtable = IPerceptionColorFrameSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf88008e5_5631_45ed_ad98_8c6aa04cfb91);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSource2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceAddedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameSourceAddedEventArgs {
    type Vtable = IPerceptionColorFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd16bf4e6_da24_442c_bbd5_55549b5b94f3);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionColorFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd277fa69_eb4c_42ef_ba4f_288f615c93c1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameSourceStatics {
    type Vtable = IPerceptionColorFrameSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df3cca2_01f8_4a87_b859_d5e5b7e1de49);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceWatcher(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionColorFrameSourceWatcher {
    type Vtable = IPerceptionColorFrameSourceWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96bd1392_e667_40c4_89f9_1462dea6a9cc);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionColorFrameSourceWatcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EnumerationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "deprecated")))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionControlSession(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionControlSession {
    type Vtable = IPerceptionControlSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99998653_5a3d_417f_9239_f1889e548b48);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlSession_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ControlLost: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveControlLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveControlLost: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetPropertyAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthCorrelatedCameraIntrinsics {
    type Vtable = IPerceptionDepthCorrelatedCameraIntrinsics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6548ca01_86de_5be1_6582_807fcf4c95cf);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectPixelAtCorrelatedDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelcoordinate: super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectPixelAtCorrelatedDepth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectPixelsAtCorrelatedDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceCoordinates_array_size: u32, sourcecoordinates: *const super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectPixelsAtCorrelatedDepth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectRegionPixelsAtCorrelatedDepthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: super::super::Foundation::Rect, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectRegionPixelsAtCorrelatedDepthAsync: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub UnprojectAllPixelsAtCorrelatedDepthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    UnprojectAllPixelsAtCorrelatedDepthAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthCorrelatedCoordinateMapper {
    type Vtable = IPerceptionDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b4d9d1d_b5f6_469c_b8c2_b97a45e6863b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapPixelToTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepixelcoordinate: super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapPixelToTarget: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapPixelsToTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceCoordinates_array_size: u32, sourcecoordinates: *const super::super::Foundation::Point, depthframe: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapPixelsToTarget: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapRegionOfPixelsToTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: super::super::Foundation::Rect, depthframe: *mut ::core::ffi::c_void, targetCoordinates_array_size: u32, targetcoordinates: *mut super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapRegionOfPixelsToTargetAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MapAllPixelsToTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depthframe: *mut ::core::ffi::c_void, targetCoordinates_array_size: u32, targetcoordinates: *mut super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MapAllPixelsToTargetAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrame(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrame {
    type Vtable = IPerceptionDepthFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa37b81fc_9906_4ffd_9161_0024b360b657);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrame_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameArrivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameArrivedEventArgs {
    type Vtable = IPerceptionDepthFrameArrivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x443d25b2_b282_4637_9173_ac978435c985);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(feature = "deprecated")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryOpenFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameReader(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameReader {
    type Vtable = IPerceptionDepthFrameReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1a3c09f_299b_4612_a4f7_270f25a096ec);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameReader_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameArrived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "deprecated")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Source: usize,
    #[cfg(feature = "deprecated")]
    pub IsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryReadLatestFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameSource {
    type Vtable = IPerceptionDepthFrameSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79d433d6_47fb_4df1_bfc9_f01d40bd9942);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CameraIntrinsicsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(feature = "deprecated")]
    pub Active: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Active: usize,
    #[cfg(feature = "deprecated")]
    pub IsControlled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsControlled: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "deprecated")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "deprecated")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AcquireControlSession: usize,
    #[cfg(feature = "deprecated")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "deprecated")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    TryGetTransformTo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, depthframesourcetomapwith: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlsession: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub OpenReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OpenReader: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameSource2 {
    type Vtable = IPerceptionDepthFrameSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d23d2e_6e2c_4e6d_91d9_704cd8dff79d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSource2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameSourceAddedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a48168_8bf8_45d2_a2f8_4ac0931cc7a6);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0c0cc4d_e96c_4d81_86dd_38b95e49c6df);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameSourceStatics {
    type Vtable = IPerceptionDepthFrameSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df3cca2_01f8_4a87_b859_d5e5b7e1de48);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceWatcher(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionDepthFrameSourceWatcher {
    type Vtable = IPerceptionDepthFrameSourceWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x780e96d1_8d02_4d2b_ada4_5ba624a0eb10);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionDepthFrameSourceWatcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EnumerationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "deprecated")))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionFrameSourcePropertiesChangedEventArgs {
    type Vtable = IPerceptionFrameSourcePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c68e068_bcf1_4ecc_b891_7625d1244b6b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub CollectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Collections::CollectionChange) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    CollectionChange: usize,
    #[cfg(feature = "deprecated")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Key: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertyChangeResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionFrameSourcePropertyChangeResult {
    type Vtable = IPerceptionFrameSourcePropertyChangeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e33390a_3c90_4d22_b898_f42bba6418ff);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameSourcePropertyChangeResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NewValue: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrame(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrame {
    type Vtable = IPerceptionInfraredFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0886276_849e_4c7a_8ae6_b56064532153);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrame_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub VideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    VideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameArrivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameArrivedEventArgs {
    type Vtable = IPerceptionInfraredFrameArrivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f77fac7_b4bd_4857_9d50_be8ef075daef);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameArrivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(feature = "deprecated")]
    pub TryOpenFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryOpenFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameReader(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameReader {
    type Vtable = IPerceptionInfraredFrameReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7960ce18_d39b_4fc8_a04a_929734c6756c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameReader_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameArrived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFrameArrived: usize,
    #[cfg(feature = "deprecated")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Source: usize,
    #[cfg(feature = "deprecated")]
    pub IsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPaused: usize,
    #[cfg(feature = "deprecated")]
    pub TryReadLatestFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryReadLatestFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameSource {
    type Vtable = IPerceptionInfraredFrameSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55b08742_1808_494e_9e30_9d2a7be8f700);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAvailableChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveActiveChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePropertiesChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVideoProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVideoProfileChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CameraIntrinsicsChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCameraIntrinsicsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCameraIntrinsicsChanged: usize,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(feature = "deprecated")]
    pub Active: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Active: usize,
    #[cfg(feature = "deprecated")]
    pub IsControlled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsControlled: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SupportedVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SupportedVideoProfiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AvailableVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AvailableVideoProfiles: usize,
    #[cfg(feature = "deprecated")]
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    VideoProfile: usize,
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub CameraIntrinsics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices_Core", feature = "deprecated")))]
    CameraIntrinsics: usize,
    #[cfg(feature = "deprecated")]
    pub AcquireControlSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AcquireControlSession: usize,
    #[cfg(feature = "deprecated")]
    pub CanControlIndependentlyFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanControlIndependentlyFrom: usize,
    #[cfg(feature = "deprecated")]
    pub IsCorrelatedWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCorrelatedWith: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: *mut super::super::Foundation::Numerics::Matrix4x4, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    TryGetTransformTo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCameraIntrinsicsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCameraIntrinsicsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TryGetDepthCorrelatedCoordinateMapperAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, depthframesourcetomapwith: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TryGetDepthCorrelatedCoordinateMapperAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TrySetVideoProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controlsession: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TrySetVideoProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub OpenReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OpenReader: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameSource2 {
    type Vtable = IPerceptionInfraredFrameSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd4d798_4b0b_4300_8d85_410817faa032);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSource2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameSourceAddedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d334120_95ce_4660_907a_d98035aa2b7c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1a8071_7a70_4a61_af94_07303853f695);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameSourceStatics {
    type Vtable = IPerceptionInfraredFrameSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5df3cca2_01f8_4a87_b859_d5e5b7e1de47);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWatcher: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceWatcher(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionInfraredFrameSourceWatcher {
    type Vtable = IPerceptionInfraredFrameSourceWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x383cff99_d70c_444d_a8b0_720c2e66fe3b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionInfraredFrameSourceWatcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EnumerationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveEnumerationCompleted: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Enumeration::DeviceWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "deprecated")))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionVideoProfile(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPerceptionVideoProfile {
    type Vtable = IPerceptionVideoProfile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75763ea3_011a_470e_8225_6f05ade25648);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoProfile_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapPixelFormat: usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapAlphaMode: usize,
    #[cfg(feature = "deprecated")]
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Width: usize,
    #[cfg(feature = "deprecated")]
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Height: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameDuration: usize,
    #[cfg(feature = "deprecated")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsEqual: usize,
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownCameraIntrinsicsProperties;
#[cfg(feature = "deprecated")]
impl KnownCameraIntrinsicsProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FocalLength() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FocalLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PrincipalPoint() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrincipalPoint)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RadialDistortion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RadialDistortion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TangentialDistortion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownCameraIntrinsicsPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TangentialDistortion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownCameraIntrinsicsPropertiesStatics<R, F: FnOnce(&IKnownCameraIntrinsicsPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownCameraIntrinsicsProperties, IKnownCameraIntrinsicsPropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownCameraIntrinsicsProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownCameraIntrinsicsProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionColorFrameSourceProperties;
#[cfg(feature = "deprecated")]
impl KnownPerceptionColorFrameSourceProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Exposure() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionColorFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Exposure)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AutoExposureEnabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionColorFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AutoExposureEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ExposureCompensation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionColorFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExposureCompensation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionColorFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionColorFrameSourcePropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionColorFrameSourceProperties, IKnownPerceptionColorFrameSourcePropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownPerceptionColorFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionColorFrameSourceProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionDepthFrameSourceProperties;
#[cfg(feature = "deprecated")]
impl KnownPerceptionDepthFrameSourceProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MinDepth() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionDepthFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MinDepth)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn MaxDepth() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionDepthFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxDepth)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionDepthFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionDepthFrameSourcePropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionDepthFrameSourceProperties, IKnownPerceptionDepthFrameSourcePropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownPerceptionDepthFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionDepthFrameSourceProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionFrameSourceProperties;
#[cfg(feature = "deprecated")]
impl KnownPerceptionFrameSourceProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PhysicalDeviceIds() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalDeviceIds)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameKind() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceModelVersion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceModelVersion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn EnclosureLocation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnclosureLocation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionFrameSourcePropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionFrameSourcePropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionFrameSourceProperties, IKnownPerceptionFrameSourcePropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionFrameSourcePropertiesStatics2<R, F: FnOnce(&IKnownPerceptionFrameSourcePropertiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionFrameSourceProperties, IKnownPerceptionFrameSourcePropertiesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownPerceptionFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionFrameSourceProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionInfraredFrameSourceProperties;
#[cfg(feature = "deprecated")]
impl KnownPerceptionInfraredFrameSourceProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Exposure() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Exposure)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AutoExposureEnabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AutoExposureEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ExposureCompensation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExposureCompensation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ActiveIlluminationEnabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActiveIlluminationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AmbientSubtractionEnabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AmbientSubtractionEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn StructureLightPatternEnabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StructureLightPatternEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn InterleavedIlluminationEnabled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionInfraredFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InterleavedIlluminationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionInfraredFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionInfraredFrameSourcePropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionInfraredFrameSourceProperties, IKnownPerceptionInfraredFrameSourcePropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownPerceptionInfraredFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionInfraredFrameSourceProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionVideoFrameSourceProperties;
#[cfg(feature = "deprecated")]
impl KnownPerceptionVideoFrameSourceProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn VideoProfile() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfile)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportedVideoProfiles() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AvailableVideoProfiles() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsMirrored() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsMirrored)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CameraIntrinsics() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoFrameSourcePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsics)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionVideoFrameSourcePropertiesStatics<R, F: FnOnce(&IKnownPerceptionVideoFrameSourcePropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionVideoFrameSourceProperties, IKnownPerceptionVideoFrameSourcePropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownPerceptionVideoFrameSourceProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionVideoFrameSourceProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionVideoProfileProperties;
#[cfg(feature = "deprecated")]
impl KnownPerceptionVideoProfileProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BitmapPixelFormat() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BitmapPixelFormat)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BitmapAlphaMode() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BitmapAlphaMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Width() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Height() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameDuration() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownPerceptionVideoProfilePropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameDuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionVideoProfilePropertiesStatics<R, F: FnOnce(&IKnownPerceptionVideoProfilePropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownPerceptionVideoProfileProperties, IKnownPerceptionVideoProfilePropertiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for KnownPerceptionVideoProfileProperties {
    const NAME: &'static str = "Windows.Devices.Perception.KnownPerceptionVideoProfileProperties";
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrame(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::VideoFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrame {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrame;{fe621549-2cbf-4f94-9861-f817ea317747})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrame {
    type Vtable = IPerceptionColorFrame_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrame as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrame {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrame";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrame> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrame> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrame> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrame> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrame> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrame> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionColorFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionColorFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionColorFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionColorFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionColorFrame> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionColorFrame) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrame {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrame {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrameArrivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrameArrivedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryOpenFrame(&self) -> ::windows::core::Result<PerceptionColorFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryOpenFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrameArrivedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameArrivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrameArrivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameArrivedEventArgs;{8fad02d5-86f7-4d8d-b966-5a3761ba9f59})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrameArrivedEventArgs {
    type Vtable = IPerceptionColorFrameArrivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrameArrivedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameArrivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameArrivedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameArrivedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrameArrivedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameArrivedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrameReader(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrameReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FrameArrived<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameReader, PerceptionColorFrameArrivedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameArrived)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameArrived)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Source(&self) -> ::windows::core::Result<PerceptionColorFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSource>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsPaused(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPaused)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsPaused(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPaused)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryReadLatestFrame(&self) -> ::windows::core::Result<PerceptionColorFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryReadLatestFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrameReader {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameReader").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrameReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameReader;{7650f56e-b9f5-461b-83ad-f222af2aaadc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrameReader {
    type Vtable = IPerceptionColorFrameReader_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrameReader as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameReader";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameReader> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameReader> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameReader> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameReader> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameReader> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameReader> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionColorFrameReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionColorFrameReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionColorFrameReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionColorFrameReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionColorFrameReader> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionColorFrameReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrameReader {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameReader {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrameSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrameSource {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AvailableChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAvailableChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAvailableChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ActiveChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActiveChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActiveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PropertiesChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn VideoProfileChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfileChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveVideoProfileChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoProfileChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CameraIntrinsicsChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsicsChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCameraIntrinsicsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCameraIntrinsicsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Available(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Available)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Active(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Active)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsControlled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsControlled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SupportedVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AvailableVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn VideoProfile(&self) -> ::windows::core::Result<PerceptionVideoProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfile)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionVideoProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Devices_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsics)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AcquireControlSession(&self) -> ::windows::core::Result<PerceptionControlSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AcquireControlSession)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionControlSession>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CanControlIndependentlyFrom(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanControlIndependentlyFrom)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsCorrelatedWith(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCorrelatedWith)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn TryGetTransformTo(&self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetTransformTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TryGetDepthCorrelatedCameraIntrinsicsAsync<'a, P0>(&self, correlateddepthframesource: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrameSource>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDepthCorrelatedCameraIntrinsicsAsync)(::windows::core::Interface::as_raw(this), correlateddepthframesource.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TryGetDepthCorrelatedCoordinateMapperAsync<'a, P0>(&self, targetsourceid: &::windows::core::HSTRING, correlateddepthframesource: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrameSource>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDepthCorrelatedCoordinateMapperAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetsourceid), correlateddepthframesource.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TrySetVideoProfileAsync<'a, P0, P1>(&self, controlsession: P0, profile: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionControlSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PerceptionVideoProfile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrySetVideoProfileAsync)(::windows::core::Interface::as_raw(this), controlsession.into().abi(), profile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OpenReader(&self) -> ::windows::core::Result<PerceptionColorFrameReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenReader)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameReader>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPerceptionColorFrameSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateWatcher() -> ::windows::core::Result<PerceptionColorFrameSourceWatcher> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSourceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionColorFrameSource>>> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionColorFrameSource>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FromIdAsync(id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionColorFrameSource>> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionColorFrameSource>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>> {
        Self::IPerceptionColorFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionColorFrameSourceStatics<R, F: FnOnce(&IPerceptionColorFrameSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PerceptionColorFrameSource, IPerceptionColorFrameSourceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrameSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrameSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSource;{dc6dba7c-0b58-468d-9ca1-6db04cc0477c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrameSource {
    type Vtable = IPerceptionColorFrameSource_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrameSource as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSource";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSource> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSource> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSource> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSource> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSource> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSource> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSource {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSource {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrameSourceAddedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrameSourceAddedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameSource(&self) -> ::windows::core::Result<PerceptionColorFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSourceAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrameSourceAddedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrameSourceAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSourceAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrameSourceAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSourceAddedEventArgs;{d16bf4e6-da24-442c-bbd5-55549b5b94f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrameSourceAddedEventArgs {
    type Vtable = IPerceptionColorFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrameSourceAddedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSourceAddedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSourceAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceAddedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSourceAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceAddedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSourceAddedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSourceAddedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrameSourceRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrameSourceRemovedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameSource(&self) -> ::windows::core::Result<PerceptionColorFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionColorFrameSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSourceRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrameSourceRemovedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrameSourceRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSourceRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrameSourceRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSourceRemovedEventArgs;{d277fa69-eb4c-42ef-ba4f-288f615c93c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionColorFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrameSourceRemovedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSourceRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSourceRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceRemovedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSourceRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceRemovedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSourceRemovedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSourceRemovedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionColorFrameSourceWatcher(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionColorFrameSourceWatcher {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceAdded<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceAddedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SourceAdded)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceAdded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceRemoved<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceRemovedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SourceRemoved)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Stopped<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn EnumerationCompleted<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub fn Status(&self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionColorFrameSourceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionColorFrameSourceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionColorFrameSourceWatcher {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionColorFrameSourceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionColorFrameSourceWatcher").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionColorFrameSourceWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionColorFrameSourceWatcher;{96bd1392-e667-40c4-89f9-1462dea6a9cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionColorFrameSourceWatcher {
    type Vtable = IPerceptionColorFrameSourceWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionColorFrameSourceWatcher as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionColorFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionColorFrameSourceWatcher";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSourceWatcher> for ::windows::core::IUnknown {
    fn from(value: PerceptionColorFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceWatcher> for ::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceWatcher> for &::windows::core::IUnknown {
    fn from(value: &PerceptionColorFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionColorFrameSourceWatcher> for ::windows::core::IInspectable {
    fn from(value: PerceptionColorFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceWatcher> for ::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionColorFrameSourceWatcher> for &::windows::core::IInspectable {
    fn from(value: &PerceptionColorFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionColorFrameSourceWatcher {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionColorFrameSourceWatcher {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionControlSession(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionControlSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ControlLost<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionControlSession, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ControlLost)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveControlLost(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveControlLost)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TrySetPropertyAsync<'a, P0>(&self, name: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrySetPropertyAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionControlSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionControlSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionControlSession {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionControlSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionControlSession").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionControlSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionControlSession;{99998653-5a3d-417f-9239-f1889e548b48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionControlSession {
    type Vtable = IPerceptionControlSession_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionControlSession as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionControlSession {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionControlSession";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionControlSession> for ::windows::core::IUnknown {
    fn from(value: PerceptionControlSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionControlSession> for ::windows::core::IUnknown {
    fn from(value: &PerceptionControlSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionControlSession> for &::windows::core::IUnknown {
    fn from(value: &PerceptionControlSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionControlSession> for ::windows::core::IInspectable {
    fn from(value: PerceptionControlSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionControlSession> for ::windows::core::IInspectable {
    fn from(value: &PerceptionControlSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionControlSession> for &::windows::core::IInspectable {
    fn from(value: &PerceptionControlSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionControlSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionControlSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionControlSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionControlSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionControlSession> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionControlSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionControlSession {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionControlSession {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCameraIntrinsics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthCorrelatedCameraIntrinsics {
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn UnprojectPixelAtCorrelatedDepth<'a, P0>(&self, pixelcoordinate: super::super::Foundation::Point, depthframe: P0) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnprojectPixelAtCorrelatedDepth)(::windows::core::Interface::as_raw(this), pixelcoordinate, depthframe.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn UnprojectPixelsAtCorrelatedDepth<'a, P0>(&self, sourcecoordinates: &[super::super::Foundation::Point], depthframe: P0, results: &mut [super::super::Foundation::Numerics::Vector3]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnprojectPixelsAtCorrelatedDepth)(::windows::core::Interface::as_raw(this), sourcecoordinates.len() as u32, sourcecoordinates.as_ptr(), depthframe.into().abi(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn UnprojectRegionPixelsAtCorrelatedDepthAsync<'a, P0>(&self, region: super::super::Foundation::Rect, depthframe: P0, results: &mut [super::super::Foundation::Numerics::Vector3]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnprojectRegionPixelsAtCorrelatedDepthAsync)(::windows::core::Interface::as_raw(this), region, depthframe.into().abi(), results.len() as u32, results.as_mut_ptr(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn UnprojectAllPixelsAtCorrelatedDepthAsync<'a, P0>(&self, depthframe: P0, results: &mut [super::super::Foundation::Numerics::Vector3]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnprojectAllPixelsAtCorrelatedDepthAsync)(::windows::core::Interface::as_raw(this), depthframe.into().abi(), results.len() as u32, results.as_mut_ptr(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthCorrelatedCameraIntrinsics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthCorrelatedCameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthCorrelatedCameraIntrinsics {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthCorrelatedCameraIntrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthCorrelatedCameraIntrinsics").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthCorrelatedCameraIntrinsics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthCorrelatedCameraIntrinsics;{6548ca01-86de-5be1-6582-807fcf4c95cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthCorrelatedCameraIntrinsics {
    type Vtable = IPerceptionDepthCorrelatedCameraIntrinsics_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthCorrelatedCameraIntrinsics as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthCorrelatedCameraIntrinsics {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthCorrelatedCameraIntrinsics";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthCorrelatedCameraIntrinsics> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCameraIntrinsics> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCameraIntrinsics> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthCorrelatedCameraIntrinsics> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCameraIntrinsics> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCameraIntrinsics> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthCorrelatedCameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthCorrelatedCameraIntrinsics {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthCorrelatedCameraIntrinsics {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCoordinateMapper(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthCorrelatedCoordinateMapper {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MapPixelToTarget<'a, P0>(&self, sourcepixelcoordinate: super::super::Foundation::Point, depthframe: P0) -> ::windows::core::Result<super::super::Foundation::Point>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MapPixelToTarget)(::windows::core::Interface::as_raw(this), sourcepixelcoordinate, depthframe.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MapPixelsToTarget<'a, P0>(&self, sourcecoordinates: &[super::super::Foundation::Point], depthframe: P0, results: &mut [super::super::Foundation::Point]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).MapPixelsToTarget)(::windows::core::Interface::as_raw(this), sourcecoordinates.len() as u32, sourcecoordinates.as_ptr(), depthframe.into().abi(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MapRegionOfPixelsToTargetAsync<'a, P0>(&self, region: super::super::Foundation::Rect, depthframe: P0, targetcoordinates: &mut [super::super::Foundation::Point]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MapRegionOfPixelsToTargetAsync)(::windows::core::Interface::as_raw(this), region, depthframe.into().abi(), targetcoordinates.len() as u32, targetcoordinates.as_mut_ptr(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MapAllPixelsToTargetAsync<'a, P0>(&self, depthframe: P0, targetcoordinates: &mut [super::super::Foundation::Point]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrame>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MapAllPixelsToTargetAsync)(::windows::core::Interface::as_raw(this), depthframe.into().abi(), targetcoordinates.len() as u32, targetcoordinates.as_mut_ptr(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthCorrelatedCoordinateMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthCorrelatedCoordinateMapper {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthCorrelatedCoordinateMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthCorrelatedCoordinateMapper").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthCorrelatedCoordinateMapper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthCorrelatedCoordinateMapper;{5b4d9d1d-b5f6-469c-b8c2-b97a45e6863b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthCorrelatedCoordinateMapper {
    type Vtable = IPerceptionDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthCorrelatedCoordinateMapper as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthCorrelatedCoordinateMapper";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthCorrelatedCoordinateMapper> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCoordinateMapper> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCoordinateMapper> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthCorrelatedCoordinateMapper> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCoordinateMapper> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthCorrelatedCoordinateMapper> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthCorrelatedCoordinateMapper {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthCorrelatedCoordinateMapper {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrame(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::VideoFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrame {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrame;{a37b81fc-9906-4ffd-9161-0024b360b657})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrame {
    type Vtable = IPerceptionDepthFrame_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrame as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrame {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrame";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrame> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrame> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrame> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrame> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrame> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrame> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionDepthFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionDepthFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionDepthFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionDepthFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionDepthFrame> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionDepthFrame) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrame {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrame {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrameArrivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrameArrivedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryOpenFrame(&self) -> ::windows::core::Result<PerceptionDepthFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryOpenFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrameArrivedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameArrivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrameArrivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameArrivedEventArgs;{443d25b2-b282-4637-9173-ac978435c985})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrameArrivedEventArgs {
    type Vtable = IPerceptionDepthFrameArrivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrameArrivedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameArrivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameArrivedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameArrivedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameArrivedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameArrivedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrameReader(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrameReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FrameArrived<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameReader, PerceptionDepthFrameArrivedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameArrived)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameArrived)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Source(&self) -> ::windows::core::Result<PerceptionDepthFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSource>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsPaused(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPaused)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsPaused(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPaused)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryReadLatestFrame(&self) -> ::windows::core::Result<PerceptionDepthFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryReadLatestFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrameReader {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameReader").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrameReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameReader;{b1a3c09f-299b-4612-a4f7-270f25a096ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrameReader {
    type Vtable = IPerceptionDepthFrameReader_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrameReader as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameReader";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameReader> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameReader> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameReader> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameReader> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameReader> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameReader> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionDepthFrameReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionDepthFrameReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionDepthFrameReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionDepthFrameReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionDepthFrameReader> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionDepthFrameReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameReader {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameReader {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrameSource {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AvailableChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAvailableChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAvailableChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ActiveChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActiveChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActiveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PropertiesChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn VideoProfileChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfileChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveVideoProfileChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoProfileChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CameraIntrinsicsChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsicsChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCameraIntrinsicsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCameraIntrinsicsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Available(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Available)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Active(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Active)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsControlled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsControlled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SupportedVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AvailableVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn VideoProfile(&self) -> ::windows::core::Result<PerceptionVideoProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfile)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionVideoProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Devices_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsics)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AcquireControlSession(&self) -> ::windows::core::Result<PerceptionControlSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AcquireControlSession)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionControlSession>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CanControlIndependentlyFrom(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanControlIndependentlyFrom)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsCorrelatedWith(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCorrelatedWith)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn TryGetTransformTo(&self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetTransformTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TryGetDepthCorrelatedCameraIntrinsicsAsync<'a, P0>(&self, target: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrameSource>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDepthCorrelatedCameraIntrinsicsAsync)(::windows::core::Interface::as_raw(this), target.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TryGetDepthCorrelatedCoordinateMapperAsync<'a, P0>(&self, targetid: &::windows::core::HSTRING, depthframesourcetomapwith: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrameSource>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDepthCorrelatedCoordinateMapperAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), depthframesourcetomapwith.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TrySetVideoProfileAsync<'a, P0, P1>(&self, controlsession: P0, profile: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionControlSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PerceptionVideoProfile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrySetVideoProfileAsync)(::windows::core::Interface::as_raw(this), controlsession.into().abi(), profile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OpenReader(&self) -> ::windows::core::Result<PerceptionDepthFrameReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenReader)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameReader>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPerceptionDepthFrameSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateWatcher() -> ::windows::core::Result<PerceptionDepthFrameSourceWatcher> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSourceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionDepthFrameSource>>> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionDepthFrameSource>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FromIdAsync(id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthFrameSource>> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthFrameSource>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>> {
        Self::IPerceptionDepthFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionDepthFrameSourceStatics<R, F: FnOnce(&IPerceptionDepthFrameSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PerceptionDepthFrameSource, IPerceptionDepthFrameSourceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrameSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrameSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSource;{79d433d6-47fb-4df1-bfc9-f01d40bd9942})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrameSource {
    type Vtable = IPerceptionDepthFrameSource_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrameSource as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSource";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSource> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSource> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSource> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSource> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSource> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSource> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSource {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSource {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceAddedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrameSourceAddedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameSource(&self) -> ::windows::core::Result<PerceptionDepthFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSourceAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrameSourceAddedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrameSourceAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSourceAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrameSourceAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSourceAddedEventArgs;{93a48168-8bf8-45d2-a2f8-4ac0931cc7a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrameSourceAddedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrameSourceAddedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSourceAddedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSourceAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceAddedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSourceAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceAddedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSourceAddedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSourceAddedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrameSourceRemovedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameSource(&self) -> ::windows::core::Result<PerceptionDepthFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionDepthFrameSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSourceRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrameSourceRemovedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrameSourceRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSourceRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrameSourceRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSourceRemovedEventArgs;{a0c0cc4d-e96c-4d81-86dd-38b95e49c6df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionDepthFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrameSourceRemovedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSourceRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSourceRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceRemovedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSourceRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceRemovedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSourceRemovedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSourceRemovedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceWatcher(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionDepthFrameSourceWatcher {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceAdded<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceAddedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SourceAdded)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceAdded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceRemoved<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceRemovedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SourceRemoved)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Stopped<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn EnumerationCompleted<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub fn Status(&self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionDepthFrameSourceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionDepthFrameSourceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionDepthFrameSourceWatcher {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionDepthFrameSourceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionDepthFrameSourceWatcher").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionDepthFrameSourceWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionDepthFrameSourceWatcher;{780e96d1-8d02-4d2b-ada4-5ba624a0eb10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionDepthFrameSourceWatcher {
    type Vtable = IPerceptionDepthFrameSourceWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionDepthFrameSourceWatcher as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionDepthFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionDepthFrameSourceWatcher";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSourceWatcher> for ::windows::core::IUnknown {
    fn from(value: PerceptionDepthFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceWatcher> for ::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceWatcher> for &::windows::core::IUnknown {
    fn from(value: &PerceptionDepthFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionDepthFrameSourceWatcher> for ::windows::core::IInspectable {
    fn from(value: PerceptionDepthFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceWatcher> for ::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionDepthFrameSourceWatcher> for &::windows::core::IInspectable {
    fn from(value: &PerceptionDepthFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionDepthFrameSourceWatcher {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionDepthFrameSourceWatcher {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PerceptionFrameSourceAccessStatus(pub i32);
#[cfg(feature = "deprecated")]
impl PerceptionFrameSourceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PerceptionFrameSourceAccessStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameSourceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PerceptionFrameSourceAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for PerceptionFrameSourceAccessStatus {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFrameSourceAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourceAccessStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionFrameSourceAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Perception.PerceptionFrameSourceAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertiesChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionFrameSourcePropertiesChangedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn CollectionChange(&self) -> ::windows::core::Result<super::super::Foundation::Collections::CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CollectionChange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::CollectionChange>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Key)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionFrameSourcePropertiesChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourcePropertiesChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionFrameSourcePropertiesChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionFrameSourcePropertiesChangedEventArgs;{6c68e068-bcf1-4ecc-b891-7625d1244b6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionFrameSourcePropertiesChangedEventArgs {
    type Vtable = IPerceptionFrameSourcePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionFrameSourcePropertiesChangedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionFrameSourcePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionFrameSourcePropertiesChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertiesChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertiesChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionFrameSourcePropertiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionFrameSourcePropertiesChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionFrameSourcePropertiesChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionFrameSourcePropertyChangeResult {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows::core::Result<PerceptionFrameSourcePropertyChangeStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionFrameSourcePropertyChangeStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NewValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NewValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionFrameSourcePropertyChangeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionFrameSourcePropertyChangeResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFrameSourcePropertyChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourcePropertyChangeResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionFrameSourcePropertyChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionFrameSourcePropertyChangeResult;{1e33390a-3c90-4d22-b898-f42bba6418ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionFrameSourcePropertyChangeResult {
    type Vtable = IPerceptionFrameSourcePropertyChangeResult_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionFrameSourcePropertyChangeResult as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionFrameSourcePropertyChangeResult {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionFrameSourcePropertyChangeResult";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrameSourcePropertyChangeResult> for ::windows::core::IUnknown {
    fn from(value: PerceptionFrameSourcePropertyChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertyChangeResult> for ::windows::core::IUnknown {
    fn from(value: &PerceptionFrameSourcePropertyChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertyChangeResult> for &::windows::core::IUnknown {
    fn from(value: &PerceptionFrameSourcePropertyChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrameSourcePropertyChangeResult> for ::windows::core::IInspectable {
    fn from(value: PerceptionFrameSourcePropertyChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertyChangeResult> for ::windows::core::IInspectable {
    fn from(value: &PerceptionFrameSourcePropertyChangeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameSourcePropertyChangeResult> for &::windows::core::IInspectable {
    fn from(value: &PerceptionFrameSourcePropertyChangeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionFrameSourcePropertyChangeResult {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionFrameSourcePropertyChangeResult {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PerceptionFrameSourcePropertyChangeStatus(pub i32);
#[cfg(feature = "deprecated")]
impl PerceptionFrameSourcePropertyChangeStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Accepted: Self = Self(1i32);
    pub const LostControl: Self = Self(2i32);
    pub const PropertyNotSupported: Self = Self(3i32);
    pub const PropertyReadOnly: Self = Self(4i32);
    pub const ValueOutOfRange: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PerceptionFrameSourcePropertyChangeStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PerceptionFrameSourcePropertyChangeStatus {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for PerceptionFrameSourcePropertyChangeStatus {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFrameSourcePropertyChangeStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameSourcePropertyChangeStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionFrameSourcePropertyChangeStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Perception.PerceptionFrameSourcePropertyChangeStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrame(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::VideoFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrame {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrame;{b0886276-849e-4c7a-8ae6-b56064532153})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrame {
    type Vtable = IPerceptionInfraredFrame_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrame as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrame {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrame";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrame> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrame> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrame> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrame> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrame> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrame> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionInfraredFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionInfraredFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionInfraredFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionInfraredFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionInfraredFrame> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionInfraredFrame) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrame {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrame {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameArrivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrameArrivedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryOpenFrame(&self) -> ::windows::core::Result<PerceptionInfraredFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryOpenFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameArrivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrameArrivedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrameArrivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameArrivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrameArrivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameArrivedEventArgs;{9f77fac7-b4bd-4857-9d50-be8ef075daef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrameArrivedEventArgs {
    type Vtable = IPerceptionInfraredFrameArrivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrameArrivedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameArrivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameArrivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameArrivedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameArrivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameArrivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameArrivedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameArrivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameArrivedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameArrivedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameReader(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrameReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FrameArrived<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameReader, PerceptionInfraredFrameArrivedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameArrived)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameArrived)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Source(&self) -> ::windows::core::Result<PerceptionInfraredFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSource>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsPaused(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsPaused)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsPaused(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPaused)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryReadLatestFrame(&self) -> ::windows::core::Result<PerceptionInfraredFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryReadLatestFrame)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrame>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrameReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrameReader {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrameReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameReader").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrameReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameReader;{7960ce18-d39b-4fc8-a04a-929734c6756c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrameReader {
    type Vtable = IPerceptionInfraredFrameReader_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrameReader as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrameReader {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameReader";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameReader> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameReader> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameReader> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameReader> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameReader> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameReader> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionInfraredFrameReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PerceptionInfraredFrameReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionInfraredFrameReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionInfraredFrameReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&PerceptionInfraredFrameReader> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PerceptionInfraredFrameReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameReader {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameReader {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrameSource {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AvailableChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAvailableChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAvailableChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ActiveChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActiveChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActiveChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PropertiesChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn VideoProfileChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfileChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveVideoProfileChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoProfileChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CameraIntrinsicsChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsicsChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCameraIntrinsicsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCameraIntrinsicsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Available(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Available)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Active(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Active)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsControlled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsControlled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SupportedVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AvailableVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableVideoProfiles)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn VideoProfile(&self) -> ::windows::core::Result<PerceptionVideoProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfile)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionVideoProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Devices_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Devices_Core", feature = "deprecated"))]
    pub fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CameraIntrinsics)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Devices::Core::CameraIntrinsics>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AcquireControlSession(&self) -> ::windows::core::Result<PerceptionControlSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AcquireControlSession)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionControlSession>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CanControlIndependentlyFrom(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanControlIndependentlyFrom)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsCorrelatedWith(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCorrelatedWith)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn TryGetTransformTo(&self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetTransformTo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), result, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TryGetDepthCorrelatedCameraIntrinsicsAsync<'a, P0>(&self, target: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrameSource>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDepthCorrelatedCameraIntrinsicsAsync)(::windows::core::Interface::as_raw(this), target.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TryGetDepthCorrelatedCoordinateMapperAsync<'a, P0>(&self, targetid: &::windows::core::HSTRING, depthframesourcetomapwith: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionDepthFrameSource>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetDepthCorrelatedCoordinateMapperAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(targetid), depthframesourcetomapwith.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TrySetVideoProfileAsync<'a, P0, P1>(&self, controlsession: P0, profile: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionControlSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, PerceptionVideoProfile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrySetVideoProfileAsync)(::windows::core::Interface::as_raw(this), controlsession.into().abi(), profile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OpenReader(&self) -> ::windows::core::Result<PerceptionInfraredFrameReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenReader)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameReader>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPerceptionInfraredFrameSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateWatcher() -> ::windows::core::Result<PerceptionInfraredFrameSourceWatcher> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSourceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionInfraredFrameSource>>> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionInfraredFrameSource>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FromIdAsync(id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionInfraredFrameSource>> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionInfraredFrameSource>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>> {
        Self::IPerceptionInfraredFrameSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionInfraredFrameSourceStatics<R, F: FnOnce(&IPerceptionInfraredFrameSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PerceptionInfraredFrameSource, IPerceptionInfraredFrameSourceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrameSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrameSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSource;{55b08742-1808-494e-9e30-9d2a7be8f700})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrameSource {
    type Vtable = IPerceptionInfraredFrameSource_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrameSource as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrameSource {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSource";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSource> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSource> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSource> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSource> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSource> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSource> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSource {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSource {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceAddedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrameSourceAddedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameSource(&self) -> ::windows::core::Result<PerceptionInfraredFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSourceAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSourceAddedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSourceAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSourceAddedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrameSourceAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSourceAddedEventArgs;{6d334120-95ce-4660-907a-d98035aa2b7c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrameSourceAddedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrameSourceAddedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrameSourceAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSourceAddedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSourceAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceAddedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSourceAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceAddedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSourceAddedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSourceAddedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceRemovedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrameSourceRemovedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameSource(&self) -> ::windows::core::Result<PerceptionInfraredFrameSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionInfraredFrameSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSourceRemovedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSourceRemovedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrameSourceRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSourceRemovedEventArgs;{ea1a8071-7a70-4a61-af94-07303853f695})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrameSourceRemovedEventArgs {
    type Vtable = IPerceptionInfraredFrameSourceRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrameSourceRemovedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrameSourceRemovedEventArgs {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSourceRemovedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceRemovedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceRemovedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSourceRemovedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSourceRemovedEventArgs {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceWatcher(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionInfraredFrameSourceWatcher {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceAdded<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceAddedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SourceAdded)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceAdded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceRemoved<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceRemovedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SourceRemoved)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Stopped<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn EnumerationCompleted<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "deprecated"))]
    pub fn Status(&self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Enumeration::DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionInfraredFrameSourceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionInfraredFrameSourceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionInfraredFrameSourceWatcher {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionInfraredFrameSourceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionInfraredFrameSourceWatcher").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionInfraredFrameSourceWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionInfraredFrameSourceWatcher;{383cff99-d70c-444d-a8b0-720c2e66fe3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionInfraredFrameSourceWatcher {
    type Vtable = IPerceptionInfraredFrameSourceWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionInfraredFrameSourceWatcher as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionInfraredFrameSourceWatcher {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionInfraredFrameSourceWatcher";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSourceWatcher> for ::windows::core::IUnknown {
    fn from(value: PerceptionInfraredFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceWatcher> for ::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceWatcher> for &::windows::core::IUnknown {
    fn from(value: &PerceptionInfraredFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionInfraredFrameSourceWatcher> for ::windows::core::IInspectable {
    fn from(value: PerceptionInfraredFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceWatcher> for ::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionInfraredFrameSourceWatcher> for &::windows::core::IInspectable {
    fn from(value: &PerceptionInfraredFrameSourceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionInfraredFrameSourceWatcher {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionInfraredFrameSourceWatcher {}
#[doc = "*Required features: `\"Devices_Perception\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionVideoProfile(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionVideoProfile {
    #[doc = "*Required features: `\"Graphics_Imaging\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BitmapPixelFormat)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BitmapAlphaMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Width(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FrameDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameDuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsEqual<'a, P0>(&self, other: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PerceptionVideoProfile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::windows::core::Interface::as_raw(this), other.into().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionVideoProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionVideoProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionVideoProfile {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionVideoProfile").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PerceptionVideoProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.PerceptionVideoProfile;{75763ea3-011a-470e-8225-6f05ade25648})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PerceptionVideoProfile {
    type Vtable = IPerceptionVideoProfile_Vtbl;
    const IID: ::windows::core::GUID = <IPerceptionVideoProfile as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PerceptionVideoProfile {
    const NAME: &'static str = "Windows.Devices.Perception.PerceptionVideoProfile";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionVideoProfile> for ::windows::core::IUnknown {
    fn from(value: PerceptionVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionVideoProfile> for ::windows::core::IUnknown {
    fn from(value: &PerceptionVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionVideoProfile> for &::windows::core::IUnknown {
    fn from(value: &PerceptionVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionVideoProfile> for ::windows::core::IInspectable {
    fn from(value: PerceptionVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionVideoProfile> for ::windows::core::IInspectable {
    fn from(value: &PerceptionVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionVideoProfile> for &::windows::core::IInspectable {
    fn from(value: &PerceptionVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionVideoProfile {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionVideoProfile {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
