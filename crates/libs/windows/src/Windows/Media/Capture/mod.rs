#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
}
impl ::core::clone::Clone for IAdvancedCapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedCapturedPhoto {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf072728b_b292_4491_9d41_99807a550bbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    Mode: usize,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedCapturedPhoto2 {
    type Vtable = IAdvancedCapturedPhoto2_Vtbl;
}
impl ::core::clone::Clone for IAdvancedCapturedPhoto2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedCapturedPhoto2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18cf6cd8_cffe_42d8_8104_017bb318f4a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FrameBoundsRelativeToReferencePhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameBoundsRelativeToReferencePhoto: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
}
impl ::core::clone::Clone for IAdvancedPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAdvancedPhotoCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83ffaafa_6667_44dc_973c_a6bce596aa0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureWithContextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureWithContextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionalReferencePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionalReferencePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub AllPhotosCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AllPhotosCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAllPhotosCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAllPhotosCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastBackgroundService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastBackgroundService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbad1e72a_fa94_46f9_95fc_d71511cda70b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetPlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastPlugInState) -> ::windows_core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows_core::HRESULT,
    pub SetSignInInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SignInInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TerminateBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HeartbeatRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeartbeatRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeartbeatRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeartbeatRequested: usize,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundService2 {
    type Vtable = IAppBroadcastBackgroundService2_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastBackgroundService2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastBackgroundService2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc8ccbbf_5549_4b87_959f_23ca401fd473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BroadcastTitleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastTitleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastTitleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastTitleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BroadcastLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BroadcastChannelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastChannelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastChannelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastChannelChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceSignInInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastBackgroundServiceSignInInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e735275_88c8_4eca_89ba_4825985db880);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetOAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetOAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOAuthCallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthCallbackUri: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SignInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSignInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSignInStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceSignInInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo2_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceSignInInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastBackgroundServiceSignInInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9104285c_62cf_4a3c_a7ee_aeb507404645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub UserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserNameChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceStreamInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastBackgroundServiceStreamInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31dc02bc_990a_4904_aa96_fe364381f136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows_core::HRESULT,
    pub SetDesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub DesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetBandwidthTestBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub BandwidthTestBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetAudioCodec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioCodec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastStreamReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoEncodingResolutionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoEncodingResolutionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoEncodingBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoEncodingBitrateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastBackgroundServiceStreamInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo2_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceStreamInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastBackgroundServiceStreamInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd1e9f6d_94dc_4fce_9541_a9f129596334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportProblemWithStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastCameraCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e334cd0_b882_4b88_8692_05999aceb70f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastGlobalSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastGlobalSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastGlobalSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2cb27a5_70fc_4e17_80bd_6ba0fd3ff3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsBroadcastEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetIsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSelectedCameraId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SelectedCameraId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCameraOverlayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlayLocation) -> ::windows_core::HRESULT,
    pub CameraOverlayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows_core::HRESULT,
    pub SetCameraOverlaySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlaySize) -> ::windows_core::HRESULT,
    pub CameraOverlaySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows_core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastHeartbeatRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastHeartbeatRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcea54283_ee51_4dbf_9472_79a9ed4e2165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastManagerStatics {
    type Vtable = IAppBroadcastManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x364e018b_1e4e_411f_ab3e_92959844c156);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetGlobalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplyGlobalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplyProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa86ad5e9_9440_4908_9d09_65b7e315d795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugIn(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPlugIn {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520c1e66_6513_4574_ac54_23b79729615b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPlugInManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPlugInManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe550d979_27a1_49a7_bbf4_d7a9e9d07668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsBroadcastProviderAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PlugInList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PlugInList: usize,
    pub DefaultPlugIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultPlugIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugInManagerStatics {
    type Vtable = IAppBroadcastPlugInManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPlugInManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPlugInManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2645c20_5c76_4cdc_9364_82fe9eb6534d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPlugInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPlugInStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4881d0f2_abc5_4fc6_84b0_89370bb47212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14b60f5a_6e4a_4b80_a14f_67ee77d153e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StopPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreviewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorCode: usize,
    #[cfg(feature = "Foundation")]
    pub PreviewStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviewStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewStateChanged: usize,
    pub PreviewStreamReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPreviewStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPreviewStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a57f2de_8dea_4e86_90ad_03fc26b9653c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PreviewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPreviewStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPreviewStreamReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92228d50_db3f_40a8_8cd4_f4e371ddab37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoStride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapAlphaMode: usize,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameArrived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPreviewStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPreviewStreamVideoFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x010fbea1_94fe_4499_b8c0_8d244279fb12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastPreviewStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastPreviewStreamVideoHeader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bef6113_da84_4499_a7ab_87118cb4a157);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastProviderSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastProviderSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastProviderSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc30bdf62_9948_458f_ad50_aa06ec03da08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDefaultBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8660b4d6_969b_4e3c_ac3a_8b042ee4ee63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastServices_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CaptureTargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCaptureTargetType) -> ::windows_core::HRESULT,
    pub SetCaptureTargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCaptureTargetType) -> ::windows_core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterBroadcastModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plugin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterBroadcastModeAsync: usize,
    pub ExitBroadcastMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AppBroadcastExitBroadcastModeReason) -> ::windows_core::HRESULT,
    pub StartBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PauseBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPreview: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastSignInStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastSignInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastSignInStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02b692a4_5919_4a9e_8d5e_c9bb0dd3377a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastState {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee08056d_8099_4ddd_922e_c56dac58abfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastState_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCaptureTargetRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShouldCaptureCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldCaptureCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestartCameraCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EncodedVideoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EncodedVideoSize: usize,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CameraCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_core::HRESULT,
    pub CameraCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows_core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthCallbackUri: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub SetAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    SetAuthenticationResult: usize,
    pub SetSignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows_core::HRESULT,
    pub TerminationReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastTerminationReason) -> ::windows_core::HRESULT,
    pub TerminationReasonPlugInSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewerCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewerCountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewerCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewerCountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CameraCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlugInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlugInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlugInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlugInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTargetClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureTargetClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastStreamAudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastStreamAudioFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefab4ac8_21ba_453f_8bb7_5e938a2e9a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AudioHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastStreamAudioHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastStreamAudioHeader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf21a570_6b78_4216_9f07_5aff5256f1b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastStreamReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb338bcf9_3364_4460_b5f1_3cc2796a8aa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AudioChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AudioSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioAacSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioAacSequence: usize,
    pub AudioBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TryGetNextAudioFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub VideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameArrived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastStreamStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastStreamStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5108a733_d008_4a89_93be_58aed961374e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastStreamVideoFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f97cf2b_c9e4_4e88_8194_d814cbd585d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastStreamVideoHeader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b9ebece_7e32_432d_8ca2_36bf10b9f462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsKeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdeebab35_ec5e_4d8f_b1c0_5da6e8c75638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BackgroundService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastViewerCountChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastViewerCountChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastViewerCountChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6e11825_5401_4ade_8bd2_c14ecee6807d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCapture {
    type Vtable = IAppCapture_Vtbl;
}
impl ::core::clone::Clone for IAppCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9749d453_a29a_45ed_8f29_22d09942cff7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCapturingAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCapturingVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CapturingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CapturingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCapturingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCapturingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureAlternateShortcutKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureAlternateShortcutKeys {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19e8e0ef_236c_40f9_b38f_9b7dd65d1ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureAlternateShortcutKeys2 {
    type Vtable = IAppCaptureAlternateShortcutKeys2_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureAlternateShortcutKeys2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureAlternateShortcutKeys2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3669090_dd17_47f0_95e5_ce42286cf338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureAlternateShortcutKeys3 {
    type Vtable = IAppCaptureAlternateShortcutKeys3_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureAlternateShortcutKeys3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureAlternateShortcutKeys3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b81448c_418e_469c_a49a_45b597c826b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureDurationGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureDurationGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureDurationGeneratedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1f5563b_ffa1_44c9_975f_27fbeb553b35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureFileGeneratedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureFileGeneratedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4189fbf4_465e_45bf_907f_165b3fb23758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureManagerStatics {
    type Vtable = IAppCaptureManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9e3ea7_6282_4735_8d4e_aa45f90f6723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appcapturesettings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureMetadataWriter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureMetadataWriter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0ce4877_9aaf_46b4_ad31_6a60b441c780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddStringEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub AddInt32Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub AddDoubleEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StartStringState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StartInt32State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StartDoubleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::HRESULT,
    pub StopState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StopAllStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemainingStorageBytesAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MetadataPurged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MetadataPurged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMetadataPurged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMetadataPurged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x324d249e_45bc_4c35_bc35_e469fc7a69e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureRecordOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureRecordOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureRecordOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc66020a9_1538_495c_9bbb_2ba870ec5861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StopRecording: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorCode: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Foundation")]
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsFileTruncated: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DurationGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDurationGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDurationGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub FileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileGenerated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureRecordingStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureRecordingStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureRecordingStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24fc8712_e305_490d_b415_6b1c9049736b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44fec0b5_34f5_4f18_ae8c_b9123abbfc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureServices_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordTimeSpan: usize,
    pub CanCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14683a86_8807_48d3_883a_970ee4532a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub SetAppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetAppCaptureDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub AppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AppCaptureDestinationFolder: usize,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHistoricalBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub HistoricalBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows_core::HRESULT,
    pub HistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows_core::HRESULT,
    pub SetIsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetMaximumRecordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaximumRecordLength: usize,
    #[cfg(feature = "Foundation")]
    pub MaximumRecordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaximumRecordLength: usize,
    #[cfg(feature = "Storage")]
    pub SetScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetScreenshotDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub ScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScreenshotDestinationFolder: usize,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows_core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows_core::HRESULT,
    pub SetIsAppCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAppCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMemoryConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings2 {
    type Vtable = IAppCaptureSettings2_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcb8cee7_e26b_476f_9b1a_ec342d2a8fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AlternateShortcutKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings3 {
    type Vtable = IAppCaptureSettings3_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureSettings3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureSettings3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa93502fe_88c2_42d6_aaaa_40feffd75aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings4 {
    type Vtable = IAppCaptureSettings4_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureSettings4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureSettings4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07c2774c_1a81_482f_a244_049d95f25b0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetVideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows_core::HRESULT,
    pub VideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureSettings5 {
    type Vtable = IAppCaptureSettings5_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureSettings5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureSettings5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18894522_b0e8_4ba0_8f13_3eaa5fa4013b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureState(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureState {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73134372_d4eb_44ce_9538_465f506ac4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureState_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTargetRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTargetClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureTargetClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureStatics {
    type Vtable = IAppCaptureStatics_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf922dd6c_0a7e_4e74_8b20_9c1f902d08a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCaptureStatics2 {
    type Vtable = IAppCaptureStatics2_Vtbl;
}
impl ::core::clone::Clone for IAppCaptureStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppCaptureStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2d881d4_836c_4da4_afd7_facc041e1cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetAllowedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowed: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAllowedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUI(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
}
impl ::core::clone::Clone for ICameraCaptureUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraCaptureUI {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48587540_6f93_4bb4_b8f3_e89e48948c91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUI_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PhotoSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VideoSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CaptureFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: CameraCaptureUIMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CaptureFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUIPhotoCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
}
impl ::core::clone::Clone for ICameraCaptureUIPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraCaptureUIPhotoCaptureSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9f5be97_3472_46a8_8a9e_04ce42ccc97d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIPhotoFormat) -> ::windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows_core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxPhotoResolution) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CroppedSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CroppedSizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub SetCroppedSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCroppedSizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub CroppedAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CroppedAspectRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetCroppedAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCroppedAspectRatio: usize,
    pub AllowCropping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowCropping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUIVideoCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
}
impl ::core::clone::Clone for ICameraCaptureUIVideoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraCaptureUIVideoCaptureSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e92d1f_a28d_425a_b84f_e568335ff24e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIVideoFormat) -> ::windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIVideoFormat) -> ::windows_core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows_core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxVideoResolution) -> ::windows_core::HRESULT,
    pub MaxDurationInSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMaxDurationInSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub AllowTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOptionsUIStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraOptionsUIStatics {
    type Vtable = ICameraOptionsUIStatics_Vtbl;
}
impl ::core::clone::Clone for ICameraOptionsUIStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICameraOptionsUIStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b0d5e34_3906_4b7d_946c_7bde844499ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediacapture: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
}
impl ::core::clone::Clone for ICapturedFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICapturedFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dd2de1f_571b_44d8_8e80_a08a1578766e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrame2 {
    type Vtable = ICapturedFrame2_Vtbl;
}
impl ::core::clone::Clone for ICapturedFrame2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICapturedFrame2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x543fa6d1_bd78_4866_adda_24314bc65dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    BitmapProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameControlValues(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
}
impl ::core::clone::Clone for ICapturedFrameControlValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICapturedFrameControlValues {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90c65b7f_4e0d_4ca4_882d_7a144fed0a90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Exposure: usize,
    #[cfg(feature = "Foundation")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExposureCompensation: usize,
    #[cfg(feature = "Foundation")]
    pub IsoSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoSpeed: usize,
    #[cfg(feature = "Foundation")]
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Focus: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub SceneMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    SceneMode: usize,
    #[cfg(feature = "Foundation")]
    pub Flashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Flashed: usize,
    #[cfg(feature = "Foundation")]
    pub FlashPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlashPowerPercent: usize,
    #[cfg(feature = "Foundation")]
    pub WhiteBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhiteBalance: usize,
    #[cfg(feature = "Foundation")]
    pub ZoomFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZoomFactor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameControlValues2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrameControlValues2 {
    type Vtable = ICapturedFrameControlValues2_Vtbl;
}
impl ::core::clone::Clone for ICapturedFrameControlValues2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICapturedFrameControlValues2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x500b2b88_06d2_4aa7_a7db_d37af73321d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    FocusState: usize,
    #[cfg(feature = "Foundation")]
    pub IsoDigitalGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoDigitalGain: usize,
    #[cfg(feature = "Foundation")]
    pub IsoAnalogGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoAnalogGain: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SensorFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SensorFrameRate: usize,
    #[cfg(feature = "Foundation")]
    pub WhiteBalanceGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhiteBalanceGain: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameWithSoftwareBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedFrameWithSoftwareBitmap {
    type Vtable = ICapturedFrameWithSoftwareBitmap_Vtbl;
}
impl ::core::clone::Clone for ICapturedFrameWithSoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICapturedFrameWithSoftwareBitmap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb58e8b6e_8503_49b5_9e86_897d26a3ff3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedPhoto(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
}
impl ::core::clone::Clone for ICapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICapturedPhoto {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0ce7e5a_cfcc_4d6c_8ad1_0869208aca16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedPhoto_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServices {
    type Vtable = IGameBarServices_Vtbl;
}
impl ::core::clone::Clone for IGameBarServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameBarServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dbead57_50a6_499e_8c6c_d330a7311796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServices_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TargetCapturePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarTargetCapturePolicy) -> ::windows_core::HRESULT,
    pub EnableCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppBroadcastServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppCaptureServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommandReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesCommandEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGameBarServicesCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameBarServicesCommandEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74226b2_f176_4fcf_8fbb_cf698b2eb8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommand) -> ::windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommandOrigin) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
}
impl ::core::clone::Clone for IGameBarServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameBarServicesManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a4b9cfa_7f8b_4c60_9dbb_0bcd262dffc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GameBarServicesCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameBarServicesCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameBarServicesCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameBarServicesCreated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xededbd9c_143e_49a3_a5ea_0b1995c8d46e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GameBarServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesManagerStatics {
    type Vtable = IGameBarServicesManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IGameBarServicesManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameBarServicesManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34c1b616_ff25_4792_98f2_d3753f15ac13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesTargetInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
}
impl ::core::clone::Clone for IGameBarServicesTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGameBarServicesTargetInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4202f92_1611_4e05_b6ef_dfd737ae33b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarServicesDisplayMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
}
impl ::core::clone::Clone for ILowLagMediaRecording {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagMediaRecording {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41c8baf7_ff3f_49f0_a477_f195e3ce5108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagMediaRecording2 {
    type Vtable = ILowLagMediaRecording2_Vtbl;
}
impl ::core::clone::Clone for ILowLagMediaRecording2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagMediaRecording2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6369c758_5644_41e2_97af_8ef56a25e225);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagMediaRecording3 {
    type Vtable = ILowLagMediaRecording3_Vtbl;
}
impl ::core::clone::Clone for ILowLagMediaRecording3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagMediaRecording3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c33ab12_48f7_47da_b41e_90880a5fe0ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopWithResultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
}
impl ::core::clone::Clone for ILowLagPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagPhotoCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa37251b7_6b44_473d_8f24_f703d6c0ec44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoSequenceCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
}
impl ::core::clone::Clone for ILowLagPhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLagPhotoSequenceCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cc346bb_b9a9_4c91_8ffa_287e9c668669);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoCaptured: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture {
    type Vtable = IMediaCapture_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc61afbb4_fb10_4a34_ac18_ca80d9c8e7ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InitializeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InitializeWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediacaptureinitializationsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeWithSettingsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub StartRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    StartRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub StartRecordToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    StartRecordToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, custommediasink: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, customsinkactivationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, customsinksettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecordAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub CapturePhotoToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    CapturePhotoToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub CapturePhotoToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    CapturePhotoToStreamAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AddEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, effectactivationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, effectsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AddEffectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearEffectsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearEffectsAsync: usize,
    pub SetEncoderProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows_core::GUID, propertyvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEncoderProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erroreventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RecordLimitationExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordlimitationexceededeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordLimitationExceeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecordLimitationExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecordLimitationExceeded: usize,
    pub MediaCaptureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
    pub SetPreviewMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetPreviewMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPreviewRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows_core::HRESULT,
    pub GetPreviewRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows_core::HRESULT,
    pub SetRecordRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows_core::HRESULT,
    pub GetRecordRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture2 {
    type Vtable = IMediaCapture2_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cc68260_7da1_4043_b652_21b8878daff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareLowLagRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareLowLagRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareLowLagRecordToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareLowLagRecordToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, custommediasink: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, customsinkactivationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, customsinksettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagPhotoCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagPhotoCaptureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagPhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagPhotoSequenceCaptureAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SetEncodingPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, encoderproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SetEncodingPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture3 {
    type Vtable = IMediaCapture3_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4136f30_1564_466e_bc0a_af94e02ab016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub PrepareVariablePhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties")))]
    PrepareVariablePhotoSequenceCaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoConfirmationCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoConfirmationCaptured: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture4 {
    type Vtable = IMediaCapture4_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbacd6fd6_fb08_4947_aea2_ce14eff0ce13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub AddAudioEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))]
    AddAudioEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub AddVideoEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))]
    AddVideoEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CameraStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraStreamStateChanged: usize,
    #[cfg(feature = "Media_Devices")]
    pub CameraStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::CameraStreamState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    CameraStreamState: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewFrameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewFrameCopyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewFrameCopyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ThermalStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThermalStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThermalStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThermalStatusChanged: usize,
    pub ThermalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureThermalStatus) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareAdvancedPhotoCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareAdvancedPhotoCaptureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture5 {
    type Vtable = IMediaCapture5_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda787c22_3a9b_4720_a71e_97900a316e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RemoveEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseRecordWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseRecordWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopRecordWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecordWithResultAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSources: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))]
    CreateFrameReaderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: *mut ::core::ffi::c_void, outputsubtype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAndSizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: *mut ::core::ffi::c_void, outputsubtype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAndSizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture6 {
    type Vtable = IMediaCapture6_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x228948bd_4b20_4bb1_9fd6_a583212a1012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureDeviceExclusiveControlStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureDeviceExclusiveControlStatusChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub CreateMultiSourceFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsources: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    CreateMultiSourceFrameReaderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapture7 {
    type Vtable = IMediaCapture7_Vtbl;
}
impl ::core::clone::Clone for IMediaCapture7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapture7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9169f102_8888_541a_95bc_24e4d462542a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateRelativePanelWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capturemode: StreamingCaptureMode, displayregion: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateRelativePanelWatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d2f920d_a588_43c6_89d6_5ad322af006a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80fde3f4_54c4_42c0_8d19_cea1a87ca18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureFocusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureFocusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureFocusChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81e1bc7f_2277_493e_abee_d3f44ff98c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9782ba70_ea65_4900_9356_8ca887726884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetAudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetStreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StreamingCaptureMode) -> ::windows_core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows_core::HRESULT,
    pub SetPhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoCaptureSource) -> ::windows_core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings2 {
    type Vtable = IMediaCaptureInitializationSettings2_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x404e0626_c9dc_43e9_aee4_e6bf1b57b44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetMediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCategory) -> ::windows_core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows_core::HRESULT,
    pub SetAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows_core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings3 {
    type Vtable = IMediaCaptureInitializationSettings3_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4160519d_be48_4730_8104_0cf6e9e97948);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub SetAudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetAudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub SetVideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetVideoSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings4 {
    type Vtable = IMediaCaptureInitializationSettings4_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf502a537_4cb7_4d28_95ed_4f9f012e0518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetVideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings5 {
    type Vtable = IMediaCaptureInitializationSettings5_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5a2e3b8_2626_4e94_b7b3_5308a0f64b1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SourceGroup: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SetSourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SetSourceGroup: usize,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureSharingMode) -> ::windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureSharingMode) -> ::windows_core::HRESULT,
    pub MemoryPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureMemoryPreference) -> ::windows_core::HRESULT,
    pub SetMemoryPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureMemoryPreference) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings6 {
    type Vtable = IMediaCaptureInitializationSettings6_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2e26b47_3db1_4d33_ab63_0ffa09056585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureInitializationSettings7 {
    type Vtable = IMediaCaptureInitializationSettings7_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureInitializationSettings7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41546967_f58a_5d82_9ef4_ed572fb5e34e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub DeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    DeviceUriPasswordCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetDeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetDeviceUriPasswordCredential: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetDeviceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDeviceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapturePauseResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
}
impl ::core::clone::Clone for IMediaCapturePauseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCapturePauseResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaec47ca3_4477_4b04_a06f_2c1c5182fe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LastFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureRelativePanelWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureRelativePanelWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureRelativePanelWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d896566_04be_5b89_b30e_bd34a9f12db0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub RelativePanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RelativePanel: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d83aafe_6d45_4477_8dc4_ac5bc01c4091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows_core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows_core::HRESULT,
    pub VideoDeviceCharacteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceCharacteristic) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureSettings2 {
    type Vtable = IMediaCaptureSettings2_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureSettings2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f9e7cfb_fa9f_4b13_9cbe_5ab94f1f3493);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConcurrentRecordAndPhotoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ConcurrentRecordAndPhotoSequenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CameraSoundRequiredForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Horizontal35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Horizontal35mmEquivalentFocalLength: usize,
    #[cfg(feature = "Foundation")]
    pub PitchOffsetDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PitchOffsetDegrees: usize,
    #[cfg(feature = "Foundation")]
    pub Vertical35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Vertical35mmEquivalentFocalLength: usize,
    pub MediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows_core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureSettings3 {
    type Vtable = IMediaCaptureSettings3_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureSettings3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureSettings3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x303c67c2_8058_4b1b_b877_8c2ef3528440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureStatics {
    type Vtable = IMediaCaptureStatics_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacef81ff_99ed_4645_965e_1925cfc63834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsVideoProfileSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllVideoProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConcurrentProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConcurrentProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindKnownVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, name: KnownVideoProfile, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindKnownVideoProfiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureStopResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureStopResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureStopResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9db6a2a_a092_4ad1_97d4_f201f9d082db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LastFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoPreview {
    type Vtable = IMediaCaptureVideoPreview_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureVideoPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureVideoPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27727073_549e_447f_a20a_4f03c479d8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPreviewAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, custommediasink: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, customsinkactivationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, customsinksettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopPreviewAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureVideoProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureVideoProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21a073bf_a3ee_4ecf_9ef6_50b0bc4e1305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPreviewMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedRecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedRecordMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPhotoMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConcurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConcurrency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfile2 {
    type Vtable = IMediaCaptureVideoProfile2_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureVideoProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureVideoProfile2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97ddc95f_94ce_468f_9316_fc5bc2638f6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSourceInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSourceInfos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureVideoProfileMediaDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureVideoProfileMediaDescription {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8012afef_b691_49ff_83f2_c1e76eaaea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsVariablePhotoSequenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVariablePhotoSequenceSupported: usize,
    #[cfg(feature = "deprecated")]
    pub IsHdrVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsHdrVideoSupported: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCaptureVideoProfileMediaDescription2 {
    type Vtable = IMediaCaptureVideoProfileMediaDescription2_Vtbl;
}
impl ::core::clone::Clone for IMediaCaptureVideoProfileMediaDescription2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCaptureVideoProfileMediaDescription2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6a6ef13_322d_413a_b85a_68a88e02f4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Subtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOptionalReferencePhotoCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IOptionalReferencePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOptionalReferencePhotoCapturedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x470f88b3_1e6d_4051_9c8b_f1d85af047b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhotoCapturedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x373bfbc1_984e_4ff0_bf85_1c00aabc5a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoConfirmationCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPhotoConfirmationCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPhotoConfirmationCapturedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab473672_c28a_4827_8f8d_3636d3beb51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
}
impl ::core::clone::Clone for IScreenCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IScreenCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89179ef7_cd12_4e0e_a6d4_5b3de98b2e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SourceSuspensionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceSuspensionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceSuspensionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceSuspensionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenCaptureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenCaptureStatics {
    type Vtable = IScreenCaptureStatics_Vtbl;
}
impl ::core::clone::Clone for IScreenCaptureStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IScreenCaptureStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc898c3b0_c8a5_11e2_8b8b_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCaptureStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISourceSuspensionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISourceSuspensionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISourceSuspensionChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ece7b5e_d49b_4394_bc32_f97d6cedec1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
}
impl ::core::clone::Clone for IVideoStreamConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoStreamConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8770a6f_4390_4b5e_ad3e_0f8af0963490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub InputProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    InputProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub OutputProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    OutputProperties: usize,
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AdvancedCapturedPhoto(::windows_core::IUnknown);
impl AdvancedCapturedPhoto {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn Mode(&self) -> ::windows_core::Result<super::Devices::AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Context(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Context)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = &::windows_core::ComInterface::cast::<IAdvancedCapturedPhoto2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameBoundsRelativeToReferencePhoto)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AdvancedCapturedPhoto {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedCapturedPhoto {}
impl ::core::fmt::Debug for AdvancedCapturedPhoto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedCapturedPhoto").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AdvancedCapturedPhoto {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedCapturedPhoto;{f072728b-b292-4491-9d41-99807a550bbf})");
}
impl ::core::clone::Clone for AdvancedCapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AdvancedCapturedPhoto {
    const IID: ::windows_core::GUID = <IAdvancedCapturedPhoto as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedCapturedPhoto";
}
::windows_core::imp::interface_hierarchy!(AdvancedCapturedPhoto, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AdvancedCapturedPhoto {}
unsafe impl ::core::marker::Sync for AdvancedCapturedPhoto {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoCapture(::windows_core::IUnknown);
impl AdvancedPhotoCapture {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureWithContextAsync<P0>(&self, context: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureWithContextAsync)(::windows_core::Interface::as_raw(this), context.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionalReferencePhotoCaptured<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionalReferencePhotoCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionalReferencePhotoCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionalReferencePhotoCaptured)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AllPhotosCaptured<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllPhotosCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAllPhotosCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAllPhotosCaptured)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AdvancedPhotoCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedPhotoCapture {}
impl ::core::fmt::Debug for AdvancedPhotoCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedPhotoCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AdvancedPhotoCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedPhotoCapture;{83ffaafa-6667-44dc-973c-a6bce596aa0f})");
}
impl ::core::clone::Clone for AdvancedPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AdvancedPhotoCapture {
    const IID: ::windows_core::GUID = <IAdvancedPhotoCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedPhotoCapture";
}
::windows_core::imp::interface_hierarchy!(AdvancedPhotoCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AdvancedPhotoCapture {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCapture {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastBackgroundService(::windows_core::IUnknown);
impl AppBroadcastBackgroundService {
    pub fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlugInState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlugInState(&self) -> ::windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlugInState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSignInInfo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppBroadcastBackgroundServiceSignInInfo>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignInInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SignInInfo(&self) -> ::windows_core::Result<AppBroadcastBackgroundServiceSignInInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignInInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStreamInfo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppBroadcastBackgroundServiceStreamInfo>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StreamInfo(&self) -> ::windows_core::Result<AppBroadcastBackgroundServiceStreamInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BroadcastTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetViewerCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewerCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ViewerCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).TerminateBroadcast)(::windows_core::Interface::as_raw(this), reason, providerspecificreason).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeartbeatRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeartbeatRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeartbeatRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHeartbeatRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TitleId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TitleId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBroadcastTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBroadcastLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastChannel(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastChannel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBroadcastChannel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastChannel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BroadcastTitleChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTitleChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastTitleChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBroadcastTitleChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BroadcastLanguageChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastLanguageChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastLanguageChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBroadcastLanguageChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BroadcastChannelChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastChannelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastChannelChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBroadcastChannelChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastBackgroundService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastBackgroundService {}
impl ::core::fmt::Debug for AppBroadcastBackgroundService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastBackgroundService").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastBackgroundService {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundService;{bad1e72a-fa94-46f9-95fc-d71511cda70b})");
}
impl ::core::clone::Clone for AppBroadcastBackgroundService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastBackgroundService {
    const IID: ::windows_core::GUID = <IAppBroadcastBackgroundService as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundService";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastBackgroundService, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceSignInInfo(::windows_core::IUnknown);
impl AppBroadcastBackgroundServiceSignInInfo {
    pub fn SignInState(&self) -> ::windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignInState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOAuthRequestUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOAuthRequestUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthRequestUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OAuthRequestUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOAuthCallbackUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOAuthCallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthCallbackUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OAuthCallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUserName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignInStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignInStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSignInStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSignInStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserNameChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserNameChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserNameChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUserNameChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastBackgroundServiceSignInInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastBackgroundServiceSignInInfo {}
impl ::core::fmt::Debug for AppBroadcastBackgroundServiceSignInInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastBackgroundServiceSignInInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo;{5e735275-88c8-4eca-89ba-4825985db880})");
}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceSignInInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastBackgroundServiceSignInInfo {
    const IID: ::windows_core::GUID = <IAppBroadcastBackgroundServiceSignInInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastBackgroundServiceSignInInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceStreamInfo(::windows_core::IUnknown);
impl AppBroadcastBackgroundServiceStreamInfo {
    pub fn StreamState(&self) -> ::windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredVideoEncodingBitrate(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBandwidthTestBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BandwidthTestBitrate(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BandwidthTestBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioCodec(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioCodec)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudioCodec(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioCodec)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BroadcastStreamReader(&self) -> ::windows_core::Result<AppBroadcastStreamReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastStreamReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StreamStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStreamStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStreamStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoEncodingResolutionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingResolutionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoEncodingResolutionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoEncodingResolutionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoEncodingBitrateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingBitrateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoEncodingBitrateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoEncodingBitrateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReportProblemWithStream(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppBroadcastBackgroundServiceStreamInfo2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportProblemWithStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastBackgroundServiceStreamInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastBackgroundServiceStreamInfo {}
impl ::core::fmt::Debug for AppBroadcastBackgroundServiceStreamInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastBackgroundServiceStreamInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo;{31dc02bc-990a-4904-aa96-fe364381f136})");
}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceStreamInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastBackgroundServiceStreamInfo {
    const IID: ::windows_core::GUID = <IAppBroadcastBackgroundServiceStreamInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastBackgroundServiceStreamInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastCameraCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastCameraCaptureStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraCaptureStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs;{1e334cd0-b882-4b88-8692-05999aceb70f})");
}
impl ::core::clone::Clone for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastCameraCaptureStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastCameraCaptureStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastCameraCaptureStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastCameraCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastCameraCaptureStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(::windows_core::IUnknown);
impl AppBroadcastGlobalSettings {
    pub fn IsBroadcastEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBroadcastEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByPolicy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasHardwareEncoder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemAudioGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemAudioGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMicrophoneGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCameraCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCameraCaptureEnabledByDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCameraCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSelectedCameraId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedCameraId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SelectedCameraId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedCameraId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCameraOverlayLocation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CameraOverlayLocation(&self) -> ::windows_core::Result<AppBroadcastCameraOverlayLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraOverlayLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCameraOverlaySize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CameraOverlaySize(&self) -> ::windows_core::Result<AppBroadcastCameraOverlaySize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraOverlaySize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastGlobalSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastGlobalSettings {}
impl ::core::fmt::Debug for AppBroadcastGlobalSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastGlobalSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastGlobalSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastGlobalSettings;{b2cb27a5-70fc-4e17-80bd-6ba0fd3ff3a0})");
}
impl ::core::clone::Clone for AppBroadcastGlobalSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastGlobalSettings {
    const IID: ::windows_core::GUID = <IAppBroadcastGlobalSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastGlobalSettings";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastGlobalSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(::windows_core::IUnknown);
impl AppBroadcastHeartbeatRequestedEventArgs {
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastHeartbeatRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastHeartbeatRequestedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastHeartbeatRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastHeartbeatRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs;{cea54283-ee51-4dbf-9472-79a9ed4e2165})");
}
impl ::core::clone::Clone for AppBroadcastHeartbeatRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastHeartbeatRequestedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastHeartbeatRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastHeartbeatRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct AppBroadcastManager;
impl AppBroadcastManager {
    pub fn GetGlobalSettings() -> ::windows_core::Result<AppBroadcastGlobalSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGlobalSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ApplyGlobalSettings<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppBroadcastGlobalSettings>,
    {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ApplyGlobalSettings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    pub fn GetProviderSettings() -> ::windows_core::Result<AppBroadcastProviderSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProviderSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ApplyProviderSettings<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppBroadcastProviderSettings>,
    {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ApplyProviderSettings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastManagerStatics<R, F: FnOnce(&IAppBroadcastManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppBroadcastManager, IAppBroadcastManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AppBroadcastManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastManager";
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastMicrophoneCaptureStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs;{a86ad5e9-9440-4908-9d09-65b7e315d795})");
}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastMicrophoneCaptureStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastMicrophoneCaptureStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugIn(::windows_core::IUnknown);
impl AppBroadcastPlugIn {
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderSettings(&self) -> ::windows_core::Result<AppBroadcastProviderSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPlugIn {}
impl ::core::fmt::Debug for AppBroadcastPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugIn").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPlugIn {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugIn;{520c1e66-6513-4574-ac54-23b79729615b})");
}
impl ::core::clone::Clone for AppBroadcastPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPlugIn {
    const IID: ::windows_core::GUID = <IAppBroadcastPlugIn as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugIn";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPlugIn, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPlugIn {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugIn {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugInManager(::windows_core::IUnknown);
impl AppBroadcastPlugInManager {
    pub fn IsBroadcastProviderAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBroadcastProviderAvailable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PlugInList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlugInList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultPlugIn(&self) -> ::windows_core::Result<AppBroadcastPlugIn> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultPlugIn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultPlugIn<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppBroadcastPlugIn>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultPlugIn)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<AppBroadcastPlugInManager>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastPlugInManagerStatics<R, F: FnOnce(&IAppBroadcastPlugInManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppBroadcastPlugInManager, IAppBroadcastPlugInManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPlugInManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPlugInManager {}
impl ::core::fmt::Debug for AppBroadcastPlugInManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPlugInManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInManager;{e550d979-27a1-49a7-bbf4-d7a9e9d07668})");
}
impl ::core::clone::Clone for AppBroadcastPlugInManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPlugInManager {
    const IID: ::windows_core::GUID = <IAppBroadcastPlugInManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInManager";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPlugInManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPlugInManager {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInManager {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastPlugInStateChangedEventArgs {
    pub fn PlugInState(&self) -> ::windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlugInState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPlugInStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPlugInStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastPlugInStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs;{4881d0f2-abc5-4fc6-84b0-89370bb47212})");
}
impl ::core::clone::Clone for AppBroadcastPlugInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPlugInStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastPlugInStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPlugInStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPlugInStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreview(::windows_core::IUnknown);
impl AppBroadcastPreview {
    pub fn StopPreview(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopPreview)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PreviewState(&self) -> ::windows_core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreviewStateChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviewStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePreviewStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PreviewStreamReader(&self) -> ::windows_core::Result<AppBroadcastPreviewStreamReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewStreamReader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreview {}
impl ::core::fmt::Debug for AppBroadcastPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreview").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreview;{14b60f5a-6e4a-4b80-a14f-67ee77d153e7})");
}
impl ::core::clone::Clone for AppBroadcastPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPreview {
    const IID: ::windows_core::GUID = <IAppBroadcastPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreview";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPreview {}
unsafe impl ::core::marker::Sync for AppBroadcastPreview {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastPreviewStateChangedEventArgs {
    pub fn PreviewState(&self) -> ::windows_core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastPreviewStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs;{5a57f2de-8dea-4e86-90ad-03fc26b9653c})");
}
impl ::core::clone::Clone for AppBroadcastPreviewStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPreviewStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPreviewStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamReader(::windows_core::IUnknown);
impl AppBroadcastPreviewStreamReader {
    pub fn VideoWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoStride(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoStride)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapPixelFormat(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoBitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapAlphaMode(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoBitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> ::windows_core::Result<AppBroadcastPreviewStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextVideoFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameArrived<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrameArrived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoFrameArrived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStreamReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStreamReader {}
impl ::core::fmt::Debug for AppBroadcastPreviewStreamReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStreamReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPreviewStreamReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamReader;{92228d50-db3f-40a8-8cd4-f4e371ddab37})");
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPreviewStreamReader {
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStreamReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamReader";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStreamReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamReader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamReader {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoFrame(::windows_core::IUnknown);
impl AppBroadcastPreviewStreamVideoFrame {
    pub fn VideoHeader(&self) -> ::windows_core::Result<AppBroadcastPreviewStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStreamVideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStreamVideoFrame {}
impl ::core::fmt::Debug for AppBroadcastPreviewStreamVideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStreamVideoFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame;{010fbea1-94fe-4499-b8c0-8d244279fb12})");
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPreviewStreamVideoFrame {
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStreamVideoFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStreamVideoFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoFrame {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoFrame {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoHeader(::windows_core::IUnknown);
impl AppBroadcastPreviewStreamVideoHeader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastPreviewStreamVideoHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastPreviewStreamVideoHeader {}
impl ::core::fmt::Debug for AppBroadcastPreviewStreamVideoHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewStreamVideoHeader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader;{8bef6113-da84-4499-a7ab-87118cb4a157})");
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastPreviewStreamVideoHeader {
    const IID: ::windows_core::GUID = <IAppBroadcastPreviewStreamVideoHeader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastPreviewStreamVideoHeader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoHeader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoHeader {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastProviderSettings(::windows_core::IUnknown);
impl AppBroadcastProviderSettings {
    pub fn SetDefaultBroadcastTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultBroadcastTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DefaultBroadcastTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBroadcastTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioEncodingBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> ::windows_core::Result<AppBroadcastVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> ::windows_core::Result<AppBroadcastVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastProviderSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastProviderSettings {}
impl ::core::fmt::Debug for AppBroadcastProviderSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastProviderSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastProviderSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastProviderSettings;{c30bdf62-9948-458f-ad50-aa06ec03da08})");
}
impl ::core::clone::Clone for AppBroadcastProviderSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastProviderSettings {
    const IID: ::windows_core::GUID = <IAppBroadcastProviderSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastProviderSettings";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastProviderSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastServices(::windows_core::IUnknown);
impl AppBroadcastServices {
    pub fn CaptureTargetType(&self) -> ::windows_core::Result<AppBroadcastCaptureTargetType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTargetType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCaptureTargetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BroadcastTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBroadcastTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn BroadcastLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BroadcastLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBroadcastLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBroadcastLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanCapture(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanCapture)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterBroadcastModeAsync<P0>(&self, plugin: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: ::windows_core::IntoParam<AppBroadcastPlugIn>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterBroadcastModeAsync)(::windows_core::Interface::as_raw(this), plugin.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ExitBroadcastMode)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn StartBroadcast(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartBroadcast)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PauseBroadcast(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PauseBroadcast)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeBroadcast(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResumeBroadcast)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPreview(&self, desiredsize: super::super::Foundation::Size) -> ::windows_core::Result<AppBroadcastPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPreview)(::windows_core::Interface::as_raw(this), desiredsize, &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<AppBroadcastState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastServices {}
impl ::core::fmt::Debug for AppBroadcastServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastServices").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastServices {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastServices;{8660b4d6-969b-4e3c-ac3a-8b042ee4ee63})");
}
impl ::core::clone::Clone for AppBroadcastServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastServices {
    const IID: ::windows_core::GUID = <IAppBroadcastServices as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastServices";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastServices, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastServices {}
unsafe impl ::core::marker::Sync for AppBroadcastServices {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastSignInStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastSignInStateChangedEventArgs {
    pub fn SignInState(&self) -> ::windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignInState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Result(&self) -> ::windows_core::Result<AppBroadcastSignInResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastSignInStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastSignInStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastSignInStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs;{02b692a4-5919-4a9e-8d5e-c9bb0dd3377a})");
}
impl ::core::clone::Clone for AppBroadcastSignInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastSignInStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastSignInStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastSignInStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastState(::windows_core::IUnknown);
impl AppBroadcastState {
    pub fn IsCaptureTargetRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptureTargetRunning)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ViewerCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RestartMicrophoneCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ShouldCaptureCamera(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldCaptureCamera)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShouldCaptureCamera(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldCaptureCamera)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartCameraCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RestartCameraCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EncodedVideoSize(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodedVideoSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MicrophoneCaptureState(&self) -> ::windows_core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CameraCaptureState(&self) -> ::windows_core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraCaptureState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CameraCaptureError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraCaptureError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreamState(&self) -> ::windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PlugInState(&self) -> ::windows_core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlugInState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthRequestUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OAuthRequestUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthCallbackUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OAuthCallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn SetAuthenticationResult<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Authentication::Web::WebAuthenticationResult>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthenticationResult)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignInState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignInState(&self) -> ::windows_core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignInState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TerminationReason(&self) -> ::windows_core::Result<AppBroadcastTerminationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TerminationReason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TerminationReasonPlugInSpecific(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TerminationReasonPlugInSpecific)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ViewerCountChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCountChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveViewerCountChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveViewerCountChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MicrophoneCaptureStateChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMicrophoneCaptureStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraCaptureStateChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraCaptureStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraCaptureStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraCaptureStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlugInStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlugInStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlugInStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlugInStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StreamStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStreamStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStreamStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTargetClosed<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTargetClosed)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureTargetClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCaptureTargetClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastState {}
impl ::core::fmt::Debug for AppBroadcastState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastState;{ee08056d-8099-4ddd-922e-c56dac58abfb})");
}
impl ::core::clone::Clone for AppBroadcastState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastState {
    const IID: ::windows_core::GUID = <IAppBroadcastState as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastState";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastState, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastState {}
unsafe impl ::core::marker::Sync for AppBroadcastState {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamAudioFrame(::windows_core::IUnknown);
impl AppBroadcastStreamAudioFrame {
    pub fn AudioHeader(&self) -> ::windows_core::Result<AppBroadcastStreamAudioHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioHeader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioBuffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamAudioFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamAudioFrame {}
impl ::core::fmt::Debug for AppBroadcastStreamAudioFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamAudioFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamAudioFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioFrame;{efab4ac8-21ba-453f-8bb7-5e938a2e9a74})");
}
impl ::core::clone::Clone for AppBroadcastStreamAudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastStreamAudioFrame {
    const IID: ::windows_core::GUID = <IAppBroadcastStreamAudioFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioFrame";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastStreamAudioFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamAudioHeader(::windows_core::IUnknown);
impl AppBroadcastStreamAudioHeader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasDiscontinuity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamAudioHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamAudioHeader {}
impl ::core::fmt::Debug for AppBroadcastStreamAudioHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamAudioHeader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamAudioHeader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioHeader;{bf21a570-6b78-4216-9f07-5aff5256f1b7})");
}
impl ::core::clone::Clone for AppBroadcastStreamAudioHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastStreamAudioHeader {
    const IID: ::windows_core::GUID = <IAppBroadcastStreamAudioHeader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioHeader";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastStreamAudioHeader, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamReader(::windows_core::IUnknown);
impl AppBroadcastStreamReader {
    pub fn AudioChannels(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioChannels)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioSampleRate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioSampleRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioAacSequence(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioAacSequence)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetNextAudioFrame(&self) -> ::windows_core::Result<AppBroadcastStreamAudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextAudioFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> ::windows_core::Result<AppBroadcastStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetNextVideoFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameArrived<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFrameArrived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAudioFrameArrived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameArrived<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFrameArrived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVideoFrameArrived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamReader {}
impl ::core::fmt::Debug for AppBroadcastStreamReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamReader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamReader;{b338bcf9-3364-4460-b5f1-3cc2796a8aa2})");
}
impl ::core::clone::Clone for AppBroadcastStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastStreamReader {
    const IID: ::windows_core::GUID = <IAppBroadcastStreamReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamReader";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastStreamReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamStateChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastStreamStateChangedEventArgs {
    pub fn StreamState(&self) -> ::windows_core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamStateChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastStreamStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs;{5108a733-d008-4a89-93be-58aed961374e})");
}
impl ::core::clone::Clone for AppBroadcastStreamStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastStreamStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastStreamStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastStreamStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamVideoFrame(::windows_core::IUnknown);
impl AppBroadcastStreamVideoFrame {
    pub fn VideoHeader(&self) -> ::windows_core::Result<AppBroadcastStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoHeader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamVideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamVideoFrame {}
impl ::core::fmt::Debug for AppBroadcastStreamVideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamVideoFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamVideoFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoFrame;{0f97cf2b-c9e4-4e88-8194-d814cbd585d8})");
}
impl ::core::clone::Clone for AppBroadcastStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastStreamVideoFrame {
    const IID: ::windows_core::GUID = <IAppBroadcastStreamVideoFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoFrame";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastStreamVideoFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamVideoHeader(::windows_core::IUnknown);
impl AppBroadcastStreamVideoHeader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsKeyFrame(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsKeyFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasDiscontinuity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastStreamVideoHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastStreamVideoHeader {}
impl ::core::fmt::Debug for AppBroadcastStreamVideoHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamVideoHeader").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamVideoHeader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoHeader;{0b9ebece-7e32-432d-8ca2-36bf10b9f462})");
}
impl ::core::clone::Clone for AppBroadcastStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastStreamVideoHeader {
    const IID: ::windows_core::GUID = <IAppBroadcastStreamVideoHeader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoHeader";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastStreamVideoHeader, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(::windows_core::IUnknown);
impl AppBroadcastTriggerDetails {
    pub fn BackgroundService(&self) -> ::windows_core::Result<AppBroadcastBackgroundService> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundService)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastTriggerDetails {}
impl ::core::fmt::Debug for AppBroadcastTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastTriggerDetails;{deebab35-ec5e-4d8f-b1c0-5da6e8c75638})");
}
impl ::core::clone::Clone for AppBroadcastTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastTriggerDetails {
    const IID: ::windows_core::GUID = <IAppBroadcastTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(::windows_core::IUnknown);
impl AppBroadcastViewerCountChangedEventArgs {
    pub fn ViewerCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewerCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastViewerCountChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastViewerCountChangedEventArgs {}
impl ::core::fmt::Debug for AppBroadcastViewerCountChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastViewerCountChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs;{e6e11825-5401-4ade-8bd2-c14ecee6807d})");
}
impl ::core::clone::Clone for AppBroadcastViewerCountChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastViewerCountChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppBroadcastViewerCountChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastViewerCountChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastViewerCountChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastViewerCountChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCapture(::windows_core::IUnknown);
impl AppCapture {
    pub fn IsCapturingAudio(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCapturingAudio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsCapturingVideo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCapturingVideo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CapturingChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCapture, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCapturingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCapturingChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<AppCapture> {
        Self::IAppCaptureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAllowedAsync(allowed: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppCaptureStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAllowedAsync)(::windows_core::Interface::as_raw(this), allowed, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppCaptureStatics<R, F: FnOnce(&IAppCaptureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppCapture, IAppCaptureStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppCaptureStatics2<R, F: FnOnce(&IAppCaptureStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppCapture, IAppCaptureStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapture {}
impl ::core::fmt::Debug for AppCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCapture;{9749d453-a29a-45ed-8f29-22d09942cff7})");
}
impl ::core::clone::Clone for AppCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCapture {
    type Vtable = IAppCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCapture {
    const IID: ::windows_core::GUID = <IAppCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCapture {
    const NAME: &'static str = "Windows.Media.Capture.AppCapture";
}
::windows_core::imp::interface_hierarchy!(AppCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(::windows_core::IUnknown);
impl AppCaptureAlternateShortcutKeys {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleGameBarKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleGameBarKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleGameBarKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleGameBarKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSaveHistoricalVideoKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveHistoricalVideoKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSaveHistoricalVideoKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveHistoricalVideoKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTakeScreenshotKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TakeScreenshotKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTakeScreenshotKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TakeScreenshotKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingIndicatorKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingIndicatorKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleRecordingIndicatorKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleRecordingIndicatorKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleMicrophoneCaptureKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleMicrophoneCaptureKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleMicrophoneCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleMicrophoneCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleCameraCaptureKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleCameraCaptureKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleCameraCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleCameraCaptureKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleBroadcastKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKey(&self) -> ::windows_core::Result<super::super::System::VirtualKey> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleBroadcastKey)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetToggleBroadcastKeyModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKeyModifiers(&self) -> ::windows_core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleBroadcastKeyModifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureAlternateShortcutKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureAlternateShortcutKeys {}
impl ::core::fmt::Debug for AppCaptureAlternateShortcutKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureAlternateShortcutKeys").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureAlternateShortcutKeys {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureAlternateShortcutKeys;{19e8e0ef-236c-40f9-b38f-9b7dd65d1ccc})");
}
impl ::core::clone::Clone for AppCaptureAlternateShortcutKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureAlternateShortcutKeys {
    const IID: ::windows_core::GUID = <IAppCaptureAlternateShortcutKeys as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureAlternateShortcutKeys";
}
::windows_core::imp::interface_hierarchy!(AppCaptureAlternateShortcutKeys, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(::windows_core::IUnknown);
impl AppCaptureDurationGeneratedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureDurationGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureDurationGeneratedEventArgs {}
impl ::core::fmt::Debug for AppCaptureDurationGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureDurationGeneratedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs;{c1f5563b-ffa1-44c9-975f-27fbeb553b35})");
}
impl ::core::clone::Clone for AppCaptureDurationGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureDurationGeneratedEventArgs {
    const IID: ::windows_core::GUID = <IAppCaptureDurationGeneratedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppCaptureDurationGeneratedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureDurationGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureDurationGeneratedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(::windows_core::IUnknown);
impl AppCaptureFileGeneratedEventArgs {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureFileGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureFileGeneratedEventArgs {}
impl ::core::fmt::Debug for AppCaptureFileGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureFileGeneratedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureFileGeneratedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureFileGeneratedEventArgs;{4189fbf4-465e-45bf-907f-165b3fb23758})");
}
impl ::core::clone::Clone for AppCaptureFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureFileGeneratedEventArgs {
    const IID: ::windows_core::GUID = <IAppCaptureFileGeneratedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureFileGeneratedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppCaptureFileGeneratedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureFileGeneratedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct AppCaptureManager;
impl AppCaptureManager {
    pub fn GetCurrentSettings() -> ::windows_core::Result<AppCaptureSettings> {
        Self::IAppCaptureManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ApplySettings<P0>(appcapturesettings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppCaptureSettings>,
    {
        Self::IAppCaptureManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ApplySettings)(::windows_core::Interface::as_raw(this), appcapturesettings.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppCaptureManagerStatics<R, F: FnOnce(&IAppCaptureManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppCaptureManager, IAppCaptureManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AppCaptureManager {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureManager";
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(::windows_core::IUnknown);
impl AppCaptureMetadataWriter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppCaptureMetadataWriter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddStringEvent(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStringEvent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), priority).ok() }
    }
    pub fn AddInt32Event(&self, name: &::windows_core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInt32Event)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn AddDoubleEvent(&self, name: &::windows_core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDoubleEvent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn StartStringState(&self, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartStringState)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), priority).ok() }
    }
    pub fn StartInt32State(&self, name: &::windows_core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartInt32State)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn StartDoubleState(&self, name: &::windows_core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartDoubleState)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), value, priority).ok() }
    }
    pub fn StopState(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopState)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn StopAllStates(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopAllStates)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemainingStorageBytesAvailable(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemainingStorageBytesAvailable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MetadataPurged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MetadataPurged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMetadataPurged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMetadataPurged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AppCaptureMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureMetadataWriter {}
impl ::core::fmt::Debug for AppCaptureMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMetadataWriter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureMetadataWriter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMetadataWriter;{e0ce4877-9aaf-46b4-ad31-6a60b441c780})");
}
impl ::core::clone::Clone for AppCaptureMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureMetadataWriter {
    const IID: ::windows_core::GUID = <IAppCaptureMetadataWriter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMetadataWriter";
}
::windows_core::imp::interface_hierarchy!(AppCaptureMetadataWriter, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for AppCaptureMetadataWriter {}
unsafe impl ::core::marker::Send for AppCaptureMetadataWriter {}
unsafe impl ::core::marker::Sync for AppCaptureMetadataWriter {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(::windows_core::IUnknown);
impl AppCaptureMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
impl ::core::fmt::Debug for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMicrophoneCaptureStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs;{324d249e-45bc-4c35-bc35-e469fc7a69e0})");
}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppCaptureMicrophoneCaptureStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppCaptureMicrophoneCaptureStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureRecordOperation(::windows_core::IUnknown);
impl AppCaptureRecordOperation {
    pub fn StopRecording(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopRecording)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsFileTruncated(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFileTruncated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DurationGenerated<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DurationGenerated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDurationGenerated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDurationGenerated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileGenerated<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileGenerated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileGenerated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFileGenerated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppCaptureRecordOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureRecordOperation {}
impl ::core::fmt::Debug for AppCaptureRecordOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureRecordOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordOperation;{c66020a9-1538-495c-9bbb-2ba870ec5861})");
}
impl ::core::clone::Clone for AppCaptureRecordOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureRecordOperation {
    const IID: ::windows_core::GUID = <IAppCaptureRecordOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordOperation";
}
::windows_core::imp::interface_hierarchy!(AppCaptureRecordOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureRecordOperation {}
unsafe impl ::core::marker::Sync for AppCaptureRecordOperation {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureRecordingStateChangedEventArgs(::windows_core::IUnknown);
impl AppCaptureRecordingStateChangedEventArgs {
    pub fn State(&self) -> ::windows_core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureRecordingStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureRecordingStateChangedEventArgs {}
impl ::core::fmt::Debug for AppCaptureRecordingStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordingStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs;{24fc8712-e305-490d-b415-6b1c9049736b})");
}
impl ::core::clone::Clone for AppCaptureRecordingStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureRecordingStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppCaptureRecordingStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppCaptureRecordingStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureRecordingStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureRecordingStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureServices(::windows_core::IUnknown);
impl AppCaptureServices {
    pub fn Record(&self) -> ::windows_core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Record)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordTimeSpan(&self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordTimeSpan)(::windows_core::Interface::as_raw(this), starttime, duration, &mut result__).from_abi(result__)
        }
    }
    pub fn CanCapture(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanCapture)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<AppCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureServices {}
impl ::core::fmt::Debug for AppCaptureServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureServices").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureServices {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureServices;{44fec0b5-34f5-4f18-ae8c-b9123abbfc0d})");
}
impl ::core::clone::Clone for AppCaptureServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureServices {
    const IID: ::windows_core::GUID = <IAppCaptureServices as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureServices";
}
::windows_core::imp::interface_hierarchy!(AppCaptureServices, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureServices {}
unsafe impl ::core::marker::Sync for AppCaptureServices {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureSettings(::windows_core::IUnknown);
impl AppCaptureSettings {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetAppCaptureDestinationFolder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFolder>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppCaptureDestinationFolder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn AppCaptureDestinationFolder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppCaptureDestinationFolder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioEncodingBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomVideoEncodingWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHistoricalBufferLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHistoricalBufferLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HistoricalBufferLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalBufferLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHistoricalBufferLengthUnit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HistoricalBufferLengthUnit(&self) -> ::windows_core::Result<AppCaptureHistoricalBufferLengthUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalBufferLengthUnit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHistoricalCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHistoricalCaptureOnBatteryAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureOnBatteryAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHistoricalCaptureOnWirelessDisplayAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureOnWirelessDisplayAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaximumRecordLength(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaximumRecordLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaximumRecordLength(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaximumRecordLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetScreenshotDestinationFolder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFolder>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScreenshotDestinationFolder)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn ScreenshotDestinationFolder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScreenshotDestinationFolder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> ::windows_core::Result<AppCaptureVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingBitrateMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> ::windows_core::Result<AppCaptureVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingResolutionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAppCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAppCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAppCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsCpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCpuConstrained)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByPolicy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMemoryConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMemoryConstrained)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasHardwareEncoder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlternateShortcutKeys(&self) -> ::windows_core::Result<AppCaptureAlternateShortcutKeys> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlternateShortcutKeys)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemAudioGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemAudioGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMicrophoneGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoEncodingFrameRateMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoEncodingFrameRateMode(&self) -> ::windows_core::Result<AppCaptureVideoEncodingFrameRateMode> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoEncodingFrameRateMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEchoCancellationEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCursorImageCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureSettings {}
impl ::core::fmt::Debug for AppCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureSettings;{14683a86-8807-48d3-883a-970ee4532a39})");
}
impl ::core::clone::Clone for AppCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureSettings {
    const IID: ::windows_core::GUID = <IAppCaptureSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureSettings";
}
::windows_core::imp::interface_hierarchy!(AppCaptureSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureState(::windows_core::IUnknown);
impl AppCaptureState {
    pub fn IsTargetRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTargetRunning)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHistoricalCaptureEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldCaptureMicrophone)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RestartMicrophoneCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MicrophoneCaptureState(&self) -> ::windows_core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MicrophoneCaptureStateChanged<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMicrophoneCaptureStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTargetClosed<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTargetClosed)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureTargetClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCaptureTargetClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for AppCaptureState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCaptureState {}
impl ::core::fmt::Debug for AppCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureState;{73134372-d4eb-44ce-9538-465f506ac4ea})");
}
impl ::core::clone::Clone for AppCaptureState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppCaptureState {
    const IID: ::windows_core::GUID = <IAppCaptureState as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureState";
}
::windows_core::imp::interface_hierarchy!(AppCaptureState, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppCaptureState {}
unsafe impl ::core::marker::Sync for AppCaptureState {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUI(::windows_core::IUnknown);
impl CameraCaptureUI {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CameraCaptureUI, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PhotoSettings(&self) -> ::windows_core::Result<CameraCaptureUIPhotoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoSettings(&self) -> ::windows_core::Result<CameraCaptureUIVideoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureFileAsync)(::windows_core::Interface::as_raw(this), mode, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CameraCaptureUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraCaptureUI {}
impl ::core::fmt::Debug for CameraCaptureUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUI").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUI {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUI;{48587540-6f93-4bb4-b8f3-e89e48948c91})");
}
impl ::core::clone::Clone for CameraCaptureUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraCaptureUI {
    const IID: ::windows_core::GUID = <ICameraCaptureUI as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUI";
}
::windows_core::imp::interface_hierarchy!(CameraCaptureUI, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(::windows_core::IUnknown);
impl CameraCaptureUIPhotoCaptureSettings {
    pub fn Format(&self) -> ::windows_core::Result<CameraCaptureUIPhotoFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<CameraCaptureUIMaxPhotoResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CroppedSizeInPixels(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CroppedSizeInPixels)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCroppedSizeInPixels(&self, value: super::super::Foundation::Size) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCroppedSizeInPixels)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CroppedAspectRatio(&self) -> ::windows_core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CroppedAspectRatio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCroppedAspectRatio(&self, value: super::super::Foundation::Size) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCroppedAspectRatio)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCropping(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowCropping)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowCropping(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowCropping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CameraCaptureUIPhotoCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraCaptureUIPhotoCaptureSettings {}
impl ::core::fmt::Debug for CameraCaptureUIPhotoCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIPhotoCaptureSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings;{b9f5be97-3472-46a8-8a9e-04ce42ccc97d})");
}
impl ::core::clone::Clone for CameraCaptureUIPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraCaptureUIPhotoCaptureSettings {
    const IID: ::windows_core::GUID = <ICameraCaptureUIPhotoCaptureSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings";
}
::windows_core::imp::interface_hierarchy!(CameraCaptureUIPhotoCaptureSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CameraCaptureUIPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIPhotoCaptureSettings {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(::windows_core::IUnknown);
impl CameraCaptureUIVideoCaptureSettings {
    pub fn Format(&self) -> ::windows_core::Result<CameraCaptureUIVideoFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> ::windows_core::Result<CameraCaptureUIMaxVideoResolution> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxResolution)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxResolution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxDurationInSeconds(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxDurationInSeconds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxDurationInSeconds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowTrimming(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowTrimming)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowTrimming(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowTrimming)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for CameraCaptureUIVideoCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraCaptureUIVideoCaptureSettings {}
impl ::core::fmt::Debug for CameraCaptureUIVideoCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIVideoCaptureSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings;{64e92d1f-a28d-425a-b84f-e568335ff24e})");
}
impl ::core::clone::Clone for CameraCaptureUIVideoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraCaptureUIVideoCaptureSettings {
    const IID: ::windows_core::GUID = <ICameraCaptureUIVideoCaptureSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings";
}
::windows_core::imp::interface_hierarchy!(CameraCaptureUIVideoCaptureSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CameraCaptureUIVideoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIVideoCaptureSettings {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct CameraOptionsUI;
impl CameraOptionsUI {
    pub fn Show<P0>(mediacapture: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCapture>,
    {
        Self::ICameraOptionsUIStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), mediacapture.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICameraOptionsUIStatics<R, F: FnOnce(&ICameraOptionsUIStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CameraOptionsUI, ICameraOptionsUIStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CameraOptionsUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraOptionsUI";
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CapturedFrame(::windows_core::IUnknown);
impl CapturedFrame {
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ControlValues(&self) -> ::windows_core::Result<CapturedFrameControlValues> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlValues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn BitmapProperties(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::BitmapPropertySet> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CapturedFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CapturedFrame {}
impl ::core::fmt::Debug for CapturedFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapturedFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CapturedFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrame;{1dd2de1f-571b-44d8-8e80-a08a1578766e})");
}
impl ::core::clone::Clone for CapturedFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CapturedFrame {
    const IID: ::windows_core::GUID = <ICapturedFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrame";
}
::windows_core::imp::interface_hierarchy!(CapturedFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for CapturedFrame {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IContentTypeProvider> for CapturedFrame {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IInputStream> for CapturedFrame {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IOutputStream> for CapturedFrame {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStream> for CapturedFrame {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStreamWithContentType> for CapturedFrame {}
unsafe impl ::core::marker::Send for CapturedFrame {}
unsafe impl ::core::marker::Sync for CapturedFrame {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CapturedFrameControlValues(::windows_core::IUnknown);
impl CapturedFrameControlValues {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Exposure(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Exposure)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExposureCompensation(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExposureCompensation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsoSpeed(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsoSpeed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Focus(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Focus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn SceneMode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SceneMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Flashed(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Flashed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlashPowerPercent(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlashPowerPercent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WhiteBalance(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WhiteBalance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ZoomFactor(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomFactor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn FocusState(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsoDigitalGain(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsoDigitalGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsoAnalogGain(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsoAnalogGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SensorFrameRate(&self) -> ::windows_core::Result<super::MediaProperties::MediaRatio> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SensorFrameRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WhiteBalanceGain(&self) -> ::windows_core::Result<super::super::Foundation::IReference<WhiteBalanceGain>> {
        let this = &::windows_core::ComInterface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WhiteBalanceGain)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CapturedFrameControlValues {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CapturedFrameControlValues {}
impl ::core::fmt::Debug for CapturedFrameControlValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapturedFrameControlValues").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CapturedFrameControlValues {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrameControlValues;{90c65b7f-4e0d-4ca4-882d-7a144fed0a90})");
}
impl ::core::clone::Clone for CapturedFrameControlValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CapturedFrameControlValues {
    const IID: ::windows_core::GUID = <ICapturedFrameControlValues as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrameControlValues";
}
::windows_core::imp::interface_hierarchy!(CapturedFrameControlValues, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CapturedFrameControlValues {}
unsafe impl ::core::marker::Sync for CapturedFrameControlValues {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CapturedPhoto(::windows_core::IUnknown);
impl CapturedPhoto {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CapturedPhoto {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CapturedPhoto {}
impl ::core::fmt::Debug for CapturedPhoto {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CapturedPhoto").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CapturedPhoto {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedPhoto;{b0ce7e5a-cfcc-4d6c-8ad1-0869208aca16})");
}
impl ::core::clone::Clone for CapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CapturedPhoto {
    const IID: ::windows_core::GUID = <ICapturedPhoto as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.CapturedPhoto";
}
::windows_core::imp::interface_hierarchy!(CapturedPhoto, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CapturedPhoto {}
unsafe impl ::core::marker::Sync for CapturedPhoto {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServices(::windows_core::IUnknown);
impl GameBarServices {
    pub fn TargetCapturePolicy(&self) -> ::windows_core::Result<GameBarTargetCapturePolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetCapturePolicy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnableCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EnableCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DisableCapture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DisableCapture)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TargetInfo(&self) -> ::windows_core::Result<GameBarServicesTargetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppBroadcastServices(&self) -> ::windows_core::Result<AppBroadcastServices> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppBroadcastServices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppCaptureServices(&self) -> ::windows_core::Result<AppCaptureServices> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppCaptureServices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommandReceived<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandReceived)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCommandReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for GameBarServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServices {}
impl ::core::fmt::Debug for GameBarServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServices").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarServices {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServices;{2dbead57-50a6-499e-8c6c-d330a7311796})");
}
impl ::core::clone::Clone for GameBarServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GameBarServices {
    type Vtable = IGameBarServices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GameBarServices {
    const IID: ::windows_core::GUID = <IGameBarServices as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServices";
}
::windows_core::imp::interface_hierarchy!(GameBarServices, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GameBarServices {}
unsafe impl ::core::marker::Sync for GameBarServices {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(::windows_core::IUnknown);
impl GameBarServicesCommandEventArgs {
    pub fn Command(&self) -> ::windows_core::Result<GameBarCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Command)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Origin(&self) -> ::windows_core::Result<GameBarCommandOrigin> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Origin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GameBarServicesCommandEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesCommandEventArgs {}
impl ::core::fmt::Debug for GameBarServicesCommandEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesCommandEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarServicesCommandEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesCommandEventArgs;{a74226b2-f176-4fcf-8fbb-cf698b2eb8e0})");
}
impl ::core::clone::Clone for GameBarServicesCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GameBarServicesCommandEventArgs {
    const IID: ::windows_core::GUID = <IGameBarServicesCommandEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesCommandEventArgs";
}
::windows_core::imp::interface_hierarchy!(GameBarServicesCommandEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GameBarServicesCommandEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesCommandEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesManager(::windows_core::IUnknown);
impl GameBarServicesManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GameBarServicesCreated<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GameBarServicesCreated)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameBarServicesCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGameBarServicesCreated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<GameBarServicesManager> {
        Self::IGameBarServicesManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameBarServicesManagerStatics<R, F: FnOnce(&IGameBarServicesManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GameBarServicesManager, IGameBarServicesManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GameBarServicesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesManager {}
impl ::core::fmt::Debug for GameBarServicesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarServicesManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManager;{3a4b9cfa-7f8b-4c60-9dbb-0bcd262dffc6})");
}
impl ::core::clone::Clone for GameBarServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GameBarServicesManager {
    const IID: ::windows_core::GUID = <IGameBarServicesManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManager";
}
::windows_core::imp::interface_hierarchy!(GameBarServicesManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GameBarServicesManager {}
unsafe impl ::core::marker::Sync for GameBarServicesManager {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(::windows_core::IUnknown);
impl GameBarServicesManagerGameBarServicesCreatedEventArgs {
    pub fn GameBarServices(&self) -> ::windows_core::Result<GameBarServices> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GameBarServices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
impl ::core::fmt::Debug for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesManagerGameBarServicesCreatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs;{ededbd9c-143e-49a3-a5ea-0b1995c8d46e})");
}
impl ::core::clone::Clone for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const IID: ::windows_core::GUID = <IGameBarServicesManagerGameBarServicesCreatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(GameBarServicesManagerGameBarServicesCreatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesTargetInfo(::windows_core::IUnknown);
impl GameBarServicesTargetInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TitleId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TitleId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayMode(&self) -> ::windows_core::Result<GameBarServicesDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GameBarServicesTargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameBarServicesTargetInfo {}
impl ::core::fmt::Debug for GameBarServicesTargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesTargetInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarServicesTargetInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesTargetInfo;{b4202f92-1611-4e05-b6ef-dfd737ae33b0})");
}
impl ::core::clone::Clone for GameBarServicesTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GameBarServicesTargetInfo {
    const IID: ::windows_core::GUID = <IGameBarServicesTargetInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesTargetInfo";
}
::windows_core::imp::interface_hierarchy!(GameBarServicesTargetInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GameBarServicesTargetInfo {}
unsafe impl ::core::marker::Sync for GameBarServicesTargetInfo {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct LowLagMediaRecording(::windows_core::IUnknown);
impl LowLagMediaRecording {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseAsync)(::windows_core::Interface::as_raw(this), behavior, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows_core::ComInterface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseWithResultAsync)(::windows_core::Interface::as_raw(this), behavior, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopWithResultAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows_core::ComInterface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopWithResultAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LowLagMediaRecording {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagMediaRecording {}
impl ::core::fmt::Debug for LowLagMediaRecording {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagMediaRecording").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LowLagMediaRecording {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagMediaRecording;{41c8baf7-ff3f-49f0-a477-f195e3ce5108})");
}
impl ::core::clone::Clone for LowLagMediaRecording {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LowLagMediaRecording {
    const IID: ::windows_core::GUID = <ILowLagMediaRecording as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.LowLagMediaRecording";
}
::windows_core::imp::interface_hierarchy!(LowLagMediaRecording, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoCapture(::windows_core::IUnknown);
impl LowLagPhotoCapture {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoCapture {}
impl ::core::fmt::Debug for LowLagPhotoCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LowLagPhotoCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoCapture;{a37251b7-6b44-473d-8f24-f703d6c0ec44})");
}
impl ::core::clone::Clone for LowLagPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LowLagPhotoCapture {
    const IID: ::windows_core::GUID = <ILowLagPhotoCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoCapture";
}
::windows_core::imp::interface_hierarchy!(LowLagPhotoCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoSequenceCapture(::windows_core::IUnknown);
impl LowLagPhotoSequenceCapture {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoCaptured<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePhotoCaptured)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for LowLagPhotoSequenceCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLagPhotoSequenceCapture {}
impl ::core::fmt::Debug for LowLagPhotoSequenceCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLagPhotoSequenceCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LowLagPhotoSequenceCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoSequenceCapture;{7cc346bb-b9a9-4c91-8ffa-287e9c668669})");
}
impl ::core::clone::Clone for LowLagPhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LowLagPhotoSequenceCapture {
    const IID: ::windows_core::GUID = <ILowLagPhotoSequenceCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoSequenceCapture";
}
::windows_core::imp::interface_hierarchy!(LowLagPhotoSequenceCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCapture(::windows_core::IUnknown);
impl MediaCapture {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaCapture, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitializeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitializeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitializeWithSettingsAsync<P0>(&self, mediacaptureinitializationsettings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<MediaCaptureInitializationSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitializeWithSettingsAsync)(::windows_core::Interface::as_raw(this), mediacaptureinitializationsettings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn StartRecordToStorageFileAsync<P0, P1>(&self, encodingprofile: P0, file: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToStorageFileAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn StartRecordToStreamAsync<P0, P1>(&self, encodingprofile: P0, stream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToStreamAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkAsync<P0, P1>(&self, encodingprofile: P0, custommediasink: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::IMediaExtension>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToCustomSinkAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), custommediasink.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkIdAsync<P0, P1>(&self, encodingprofile: P0, customsinkactivationid: &::windows_core::HSTRING, customsinksettings: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordToCustomSinkIdAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), ::core::mem::transmute_copy(customsinkactivationid), customsinksettings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecordAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopRecordAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CapturePhotoToStorageFileAsync<P0, P1>(&self, r#type: P0, file: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::ImageEncodingProperties>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturePhotoToStorageFileAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn CapturePhotoToStreamAsync<P0, P1>(&self, r#type: P0, stream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::ImageEncodingProperties>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturePhotoToStreamAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AddEffectAsync<P0>(&self, mediastreamtype: MediaStreamType, effectactivationid: &::windows_core::HSTRING, effectsettings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddEffectAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, ::core::mem::transmute_copy(effectactivationid), effectsettings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearEffectsAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    pub fn SetEncoderProperty<P0>(&self, mediastreamtype: MediaStreamType, propertyid: ::windows_core::GUID, propertyvalue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEncoderProperty)(::windows_core::Interface::as_raw(this), mediastreamtype, propertyid, propertyvalue.into_param().abi()).ok() }
    }
    pub fn GetEncoderProperty(&self, mediastreamtype: MediaStreamType, propertyid: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEncoderProperty)(::windows_core::Interface::as_raw(this), mediastreamtype, propertyid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Failed<P0>(&self, erroreventhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<MediaCaptureFailedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Failed)(::windows_core::Interface::as_raw(this), erroreventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFailed)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordLimitationExceeded<P0>(&self, recordlimitationexceededeventhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<RecordLimitationExceededEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordLimitationExceeded)(::windows_core::Interface::as_raw(this), recordlimitationexceededeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecordLimitationExceeded(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRecordLimitationExceeded)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn MediaCaptureSettings(&self) -> ::windows_core::Result<MediaCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaCaptureSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> ::windows_core::Result<super::Devices::AudioDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreviewMirroring(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewMirroring)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPreviewMirroring(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewMirroring)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPreviewRotation(&self) -> ::windows_core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewRotation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRecordRotation(&self, value: VideoRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRecordRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetRecordRotation(&self) -> ::windows_core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRecordRotation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareLowLagRecordToStorageFileAsync<P0, P1>(&self, encodingprofile: P0, file: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToStorageFileAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareLowLagRecordToStreamAsync<P0, P1>(&self, encodingprofile: P0, stream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToStreamAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkAsync<P0, P1>(&self, encodingprofile: P0, custommediasink: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::IMediaExtension>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), custommediasink.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkIdAsync<P0, P1>(&self, encodingprofile: P0, customsinkactivationid: &::windows_core::HSTRING, customsinksettings: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkIdAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), ::core::mem::transmute_copy(customsinkactivationid), customsinksettings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagPhotoCaptureAsync<P0>(&self, r#type: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagPhotoCaptureAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagPhotoSequenceCaptureAsync<P0>(&self, r#type: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareLowLagPhotoSequenceCaptureAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SetEncodingPropertiesAsync<P0, P1>(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: P0, encoderproperties: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::MediaProperties::IMediaEncodingProperties>,
        P1: ::windows_core::IntoParam<super::MediaProperties::MediaPropertySet>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetEncodingPropertiesAsync)(::windows_core::Interface::as_raw(this), mediastreamtype, mediaencodingproperties.try_into_param()?.abi(), encoderproperties.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture_Core\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub fn PrepareVariablePhotoSequenceCaptureAsync<P0>(&self, r#type: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareVariablePhotoSequenceCaptureAsync)(::windows_core::Interface::as_raw(this), r#type.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FocusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFocusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFocusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoConfirmationCaptured<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoConfirmationCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoConfirmationCaptured(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePhotoConfirmationCaptured)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub fn AddAudioEffectAsync<P0>(&self, definition: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>
    where
        P0: ::windows_core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddAudioEffectAsync)(::windows_core::Interface::as_raw(this), definition.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub fn AddVideoEffectAsync<P0>(&self, definition: P0, mediastreamtype: MediaStreamType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>
    where
        P0: ::windows_core::TryIntoParam<super::Effects::IVideoEffectDefinition>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddVideoEffectAsync)(::windows_core::Interface::as_raw(this), definition.try_into_param()?.abi(), mediastreamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseRecordAsync)(::windows_core::Interface::as_raw(this), behavior, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeRecordAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumeRecordAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraStreamStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraStreamStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraStreamStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCameraStreamStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn CameraStreamState(&self) -> ::windows_core::Result<super::Devices::CameraStreamState> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraStreamState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewFrameAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewFrameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewFrameCopyAsync<P0>(&self, destination: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>
    where
        P0: ::windows_core::IntoParam<super::VideoFrame>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewFrameCopyAsync)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThermalStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ThermalStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThermalStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveThermalStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ThermalStatus(&self) -> ::windows_core::Result<MediaCaptureThermalStatus> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ThermalStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareAdvancedPhotoCaptureAsync<P0>(&self, encodingproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::ImageEncodingProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareAdvancedPhotoCaptureAsync)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEffectAsync<P0>(&self, effect: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::IMediaExtension>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveEffectAsync)(::windows_core::Interface::as_raw(this), effect.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseRecordWithResultAsync)(::windows_core::Interface::as_raw(this), behavior, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecordWithResultAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopRecordWithResultAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSources(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, Frames::MediaFrameSource>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameSources)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderAsync<P0>(&self, inputsource: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>
    where
        P0: ::windows_core::IntoParam<Frames::MediaFrameSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderAsync)(::windows_core::Interface::as_raw(this), inputsource.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAsync<P0>(&self, inputsource: P0, outputsubtype: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>
    where
        P0: ::windows_core::IntoParam<Frames::MediaFrameSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithSubtypeAsync)(::windows_core::Interface::as_raw(this), inputsource.into_param().abi(), ::core::mem::transmute_copy(outputsubtype), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAndSizeAsync<P0>(&self, inputsource: P0, outputsubtype: &::windows_core::HSTRING, outputsize: super::super::Graphics::Imaging::BitmapSize) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>
    where
        P0: ::windows_core::IntoParam<Frames::MediaFrameSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFrameReaderWithSubtypeAndSizeAsync)(::windows_core::Interface::as_raw(this), inputsource.into_param().abi(), ::core::mem::transmute_copy(outputsubtype), outputsize, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureDeviceExclusiveControlStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureDeviceExclusiveControlStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureDeviceExclusiveControlStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCaptureDeviceExclusiveControlStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn CreateMultiSourceFrameReaderAsync<P0>(&self, inputsources: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMultiSourceFrameReaderAsync)(::windows_core::Interface::as_raw(this), inputsources.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateRelativePanelWatcher<P0>(&self, capturemode: StreamingCaptureMode, displayregion: P0) -> ::windows_core::Result<MediaCaptureRelativePanelWatcher>
    where
        P0: ::windows_core::IntoParam<super::super::UI::WindowManagement::DisplayRegion>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCapture7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateRelativePanelWatcher)(::windows_core::Interface::as_raw(this), capturemode, displayregion.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVideoProfileSupported(videodeviceid: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoProfileSupported)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(videodeviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllVideoProfiles(videodeviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllVideoProfiles)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(videodeviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConcurrentProfiles(videodeviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindConcurrentProfiles)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(videodeviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindKnownVideoProfiles(videodeviceid: &::windows_core::HSTRING, name: KnownVideoProfile) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindKnownVideoProfiles)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(videodeviceid), name, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPreviewAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPreviewAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkAsync<P0, P1>(&self, encodingprofile: P0, custommediasink: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::IMediaExtension>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPreviewToCustomSinkAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), custommediasink.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkIdAsync<P0, P1>(&self, encodingprofile: P0, customsinkactivationid: &::windows_core::HSTRING, customsinksettings: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPreviewToCustomSinkIdAsync)(::windows_core::Interface::as_raw(this), encodingprofile.into_param().abi(), ::core::mem::transmute_copy(customsinkactivationid), customsinksettings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopPreviewAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopPreviewAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMediaCaptureStatics<R, F: FnOnce(&IMediaCaptureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaCapture, IMediaCaptureStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MediaCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCapture {}
impl ::core::fmt::Debug for MediaCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapture;{c61afbb4-fb10-4a34-ac18-ca80d9c8e7ee})");
}
impl ::core::clone::Clone for MediaCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCapture {
    type Vtable = IMediaCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCapture {
    const IID: ::windows_core::GUID = <IMediaCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapture";
}
::windows_core::imp::interface_hierarchy!(MediaCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MediaCapture {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows_core::IUnknown);
impl MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<MediaCaptureDeviceExclusiveControlStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
impl ::core::fmt::Debug for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs;{9d2f920d-a588-43c6-89d6-5ad322af006a})");
}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const IID: ::windows_core::GUID = <IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureDeviceExclusiveControlStatusChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFailedEventArgs(::windows_core::IUnknown);
impl MediaCaptureFailedEventArgs {
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Code(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Code)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureFailedEventArgs {}
impl ::core::fmt::Debug for MediaCaptureFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFailedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFailedEventArgs;{80fde3f4-54c4-42c0-8d19-cea1a87ca18b})");
}
impl ::core::clone::Clone for MediaCaptureFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureFailedEventArgs {
    const IID: ::windows_core::GUID = <IMediaCaptureFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFocusChangedEventArgs(::windows_core::IUnknown);
impl MediaCaptureFocusChangedEventArgs {
    #[doc = "*Required features: `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> ::windows_core::Result<super::Devices::MediaCaptureFocusState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureFocusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureFocusChangedEventArgs {}
impl ::core::fmt::Debug for MediaCaptureFocusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFocusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureFocusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFocusChangedEventArgs;{81e1bc7f-2277-493e-abee-d3f44ff98c04})");
}
impl ::core::clone::Clone for MediaCaptureFocusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureFocusChangedEventArgs {
    const IID: ::windows_core::GUID = <IMediaCaptureFocusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFocusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureFocusChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaCaptureFocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureFocusChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureInitializationSettings(::windows_core::IUnknown);
impl MediaCaptureInitializationSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaCaptureInitializationSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetAudioDeviceId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioDeviceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AudioDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoDeviceId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoDeviceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamingCaptureMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StreamingCaptureMode(&self) -> ::windows_core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamingCaptureMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotoCaptureSource)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhotoCaptureSource(&self) -> ::windows_core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptureSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMediaCategory(&self, value: MediaCategory) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MediaCategory(&self) -> ::windows_core::Result<MediaCategory> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaCategory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioProcessing)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioProcessing(&self) -> ::windows_core::Result<super::AudioProcessing> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SetAudioSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Core::IMediaSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioSource)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SetVideoSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Core::IMediaSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoSource)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoProfile(&self) -> ::windows_core::Result<MediaCaptureVideoProfile> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoProfile)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoProfile<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCaptureVideoProfile>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoProfile)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PreviewMediaDescription(&self) -> ::windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewMediaDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreviewMediaDescription<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCaptureVideoProfileMediaDescription>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewMediaDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RecordMediaDescription(&self) -> ::windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordMediaDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRecordMediaDescription<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCaptureVideoProfileMediaDescription>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRecordMediaDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PhotoMediaDescription(&self) -> ::windows_core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoMediaDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPhotoMediaDescription<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCaptureVideoProfileMediaDescription>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPhotoMediaDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SourceGroup(&self) -> ::windows_core::Result<Frames::MediaFrameSourceGroup> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceGroup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SetSourceGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<Frames::MediaFrameSourceGroup>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SharingMode(&self) -> ::windows_core::Result<MediaCaptureSharingMode> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSharingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MemoryPreference(&self) -> ::windows_core::Result<MediaCaptureMemoryPreference> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MemoryPreference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMemoryPreference)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysPlaySystemShutterSound(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysPlaySystemShutterSound)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysPlaySystemShutterSound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn DeviceUriPasswordCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceUriPasswordCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetDeviceUriPasswordCredential<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceUriPasswordCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDeviceUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureInitializationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureInitializationSettings {}
impl ::core::fmt::Debug for MediaCaptureInitializationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureInitializationSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureInitializationSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureInitializationSettings;{9782ba70-ea65-4900-9356-8ca887726884})");
}
impl ::core::clone::Clone for MediaCaptureInitializationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureInitializationSettings {
    const IID: ::windows_core::GUID = <IMediaCaptureInitializationSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureInitializationSettings";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureInitializationSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaCaptureInitializationSettings {}
unsafe impl ::core::marker::Sync for MediaCaptureInitializationSettings {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCapturePauseResult(::windows_core::IUnknown);
impl MediaCapturePauseResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LastFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCapturePauseResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCapturePauseResult {}
impl ::core::fmt::Debug for MediaCapturePauseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCapturePauseResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCapturePauseResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapturePauseResult;{aec47ca3-4477-4b04-a06f-2c1c5182fe9d})");
}
impl ::core::clone::Clone for MediaCapturePauseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCapturePauseResult {
    const IID: ::windows_core::GUID = <IMediaCapturePauseResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapturePauseResult";
}
::windows_core::imp::interface_hierarchy!(MediaCapturePauseResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MediaCapturePauseResult {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureRelativePanelWatcher(::windows_core::IUnknown);
impl MediaCaptureRelativePanelWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RelativePanel(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativePanel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureRelativePanelWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureRelativePanelWatcher {}
impl ::core::fmt::Debug for MediaCaptureRelativePanelWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureRelativePanelWatcher").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureRelativePanelWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureRelativePanelWatcher;{7d896566-04be-5b89-b30e-bd34a9f12db0})");
}
impl ::core::clone::Clone for MediaCaptureRelativePanelWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureRelativePanelWatcher {
    const IID: ::windows_core::GUID = <IMediaCaptureRelativePanelWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureRelativePanelWatcher";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureRelativePanelWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MediaCaptureRelativePanelWatcher {}
unsafe impl ::core::marker::Send for MediaCaptureRelativePanelWatcher {}
unsafe impl ::core::marker::Sync for MediaCaptureRelativePanelWatcher {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureSettings(::windows_core::IUnknown);
impl MediaCaptureSettings {
    pub fn AudioDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreamingCaptureMode(&self) -> ::windows_core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamingCaptureMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhotoCaptureSource(&self) -> ::windows_core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptureSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceCharacteristic(&self) -> ::windows_core::Result<VideoDeviceCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceCharacteristic)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConcurrentRecordAndPhotoSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConcurrentRecordAndPhotoSequenceSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CameraSoundRequiredForRegion(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CameraSoundRequiredForRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Horizontal35mmEquivalentFocalLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PitchOffsetDegrees(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PitchOffsetDegrees)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Vertical35mmEquivalentFocalLength(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Vertical35mmEquivalentFocalLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaCategory(&self) -> ::windows_core::Result<MediaCategory> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaCategory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioProcessing(&self) -> ::windows_core::Result<super::AudioProcessing> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioProcessing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureSettings3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureSettings {}
impl ::core::fmt::Debug for MediaCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureSettings;{1d83aafe-6d45-4477-8dc4-ac5bc01c4091})");
}
impl ::core::clone::Clone for MediaCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureSettings {
    const IID: ::windows_core::GUID = <IMediaCaptureSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureSettings";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureStopResult(::windows_core::IUnknown);
impl MediaCaptureStopResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LastFrame(&self) -> ::windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureStopResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureStopResult {}
impl ::core::fmt::Debug for MediaCaptureStopResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureStopResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureStopResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureStopResult;{f9db6a2a-a092-4ad1-97d4-f201f9d082db})");
}
impl ::core::clone::Clone for MediaCaptureStopResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureStopResult {
    const IID: ::windows_core::GUID = <IMediaCaptureStopResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureStopResult";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureStopResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MediaCaptureStopResult {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(::windows_core::IUnknown);
impl MediaCaptureVideoProfile {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPreviewMediaDescription(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPreviewMediaDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedRecordMediaDescription(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedRecordMediaDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPhotoMediaDescription(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPhotoMediaDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConcurrency(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetConcurrency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSourceInfos(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameSourceInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureVideoProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureVideoProfile {}
impl ::core::fmt::Debug for MediaCaptureVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureVideoProfile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureVideoProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfile;{21a073bf-a3ee-4ecf-9ef6-50b0bc4e1305})");
}
impl ::core::clone::Clone for MediaCaptureVideoProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureVideoProfile {
    const IID: ::windows_core::GUID = <IMediaCaptureVideoProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfile";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureVideoProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaCaptureVideoProfile {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfile {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(::windows_core::IUnknown);
impl MediaCaptureVideoProfileMediaDescription {
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsVariablePhotoSequenceSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVariablePhotoSequenceSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsHdrVideoSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHdrVideoSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCaptureVideoProfileMediaDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureVideoProfileMediaDescription {}
impl ::core::fmt::Debug for MediaCaptureVideoProfileMediaDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureVideoProfileMediaDescription").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription;{8012afef-b691-49ff-83f2-c1e76eaaea1b})");
}
impl ::core::clone::Clone for MediaCaptureVideoProfileMediaDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCaptureVideoProfileMediaDescription {
    const IID: ::windows_core::GUID = <IMediaCaptureVideoProfileMediaDescription as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription";
}
::windows_core::imp::interface_hierarchy!(MediaCaptureVideoProfileMediaDescription, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaCaptureVideoProfileMediaDescription {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfileMediaDescription {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(::windows_core::IUnknown);
impl OptionalReferencePhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Context(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Context)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OptionalReferencePhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OptionalReferencePhotoCapturedEventArgs {}
impl ::core::fmt::Debug for OptionalReferencePhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OptionalReferencePhotoCapturedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for OptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs;{470f88b3-1e6d-4051-9c8b-f1d85af047b7})");
}
impl ::core::clone::Clone for OptionalReferencePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OptionalReferencePhotoCapturedEventArgs {
    const IID: ::windows_core::GUID = <IOptionalReferencePhotoCapturedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs";
}
::windows_core::imp::interface_hierarchy!(OptionalReferencePhotoCapturedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OptionalReferencePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for OptionalReferencePhotoCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(::windows_core::IUnknown);
impl PhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTimeOffset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoCapturedEventArgs {}
impl ::core::fmt::Debug for PhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoCapturedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhotoCapturedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoCapturedEventArgs;{373bfbc1-984e-4ff0-bf85-1c00aabc5a45})");
}
impl ::core::clone::Clone for PhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhotoCapturedEventArgs {
    const IID: ::windows_core::GUID = <IPhotoCapturedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoCapturedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PhotoCapturedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(::windows_core::IUnknown);
impl PhotoConfirmationCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTimeOffset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhotoConfirmationCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoConfirmationCapturedEventArgs {}
impl ::core::fmt::Debug for PhotoConfirmationCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoConfirmationCapturedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhotoConfirmationCapturedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoConfirmationCapturedEventArgs;{ab473672-c28a-4827-8f8d-3636d3beb51e})");
}
impl ::core::clone::Clone for PhotoConfirmationCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhotoConfirmationCapturedEventArgs {
    const IID: ::windows_core::GUID = <IPhotoConfirmationCapturedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoConfirmationCapturedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PhotoConfirmationCapturedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhotoConfirmationCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoConfirmationCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct ScreenCapture(::windows_core::IUnknown);
impl ScreenCapture {
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows_core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsAudioSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioSuspended)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoSuspended)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceSuspensionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceSuspensionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceSuspensionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceSuspensionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ScreenCapture> {
        Self::IScreenCaptureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScreenCaptureStatics<R, F: FnOnce(&IScreenCaptureStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ScreenCapture, IScreenCaptureStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ScreenCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScreenCapture {}
impl ::core::fmt::Debug for ScreenCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScreenCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ScreenCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.ScreenCapture;{89179ef7-cd12-4e0e-a6d4-5b3de98b2e9b})");
}
impl ::core::clone::Clone for ScreenCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScreenCapture {
    const IID: ::windows_core::GUID = <IScreenCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.ScreenCapture";
}
::windows_core::imp::interface_hierarchy!(ScreenCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ScreenCapture {}
unsafe impl ::core::marker::Sync for ScreenCapture {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct SourceSuspensionChangedEventArgs(::windows_core::IUnknown);
impl SourceSuspensionChangedEventArgs {
    pub fn IsAudioSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAudioSuspended)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVideoSuspended)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SourceSuspensionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SourceSuspensionChangedEventArgs {}
impl ::core::fmt::Debug for SourceSuspensionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SourceSuspensionChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SourceSuspensionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.SourceSuspensionChangedEventArgs;{2ece7b5e-d49b-4394-bc32-f97d6cedec1c})");
}
impl ::core::clone::Clone for SourceSuspensionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SourceSuspensionChangedEventArgs {
    const IID: ::windows_core::GUID = <ISourceSuspensionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.SourceSuspensionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SourceSuspensionChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SourceSuspensionChangedEventArgs {}
unsafe impl ::core::marker::Sync for SourceSuspensionChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct VideoStreamConfiguration(::windows_core::IUnknown);
impl VideoStreamConfiguration {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn InputProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn OutputProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutputProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoStreamConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStreamConfiguration {}
impl ::core::fmt::Debug for VideoStreamConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStreamConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoStreamConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.VideoStreamConfiguration;{d8770a6f-4390-4b5e-ad3e-0f8af0963490})");
}
impl ::core::clone::Clone for VideoStreamConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoStreamConfiguration {
    const IID: ::windows_core::GUID = <IVideoStreamConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.VideoStreamConfiguration";
}
::windows_core::imp::interface_hierarchy!(VideoStreamConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoStreamConfiguration {}
unsafe impl ::core::marker::Sync for VideoStreamConfiguration {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraCaptureState {}
impl ::core::clone::Clone for AppBroadcastCameraCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCameraCaptureState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastCameraCaptureState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastCameraCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraCaptureState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastCameraCaptureState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraCaptureState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: Self = Self(0i32);
    pub const TopCenter: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const MiddleLeft: Self = Self(3i32);
    pub const MiddleCenter: Self = Self(4i32);
    pub const MiddleRight: Self = Self(5i32);
    pub const BottomLeft: Self = Self(6i32);
    pub const BottomCenter: Self = Self(7i32);
    pub const BottomRight: Self = Self(8i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlayLocation {}
impl ::core::clone::Clone for AppBroadcastCameraOverlayLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCameraOverlayLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastCameraOverlayLocation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastCameraOverlayLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlayLocation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastCameraOverlayLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlayLocation;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlaySize {}
impl ::core::clone::Clone for AppBroadcastCameraOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCameraOverlaySize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastCameraOverlaySize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastCameraOverlaySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlaySize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastCameraOverlaySize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlaySize;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastCaptureTargetType {}
impl ::core::clone::Clone for AppBroadcastCaptureTargetType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastCaptureTargetType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastCaptureTargetType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastCaptureTargetType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCaptureTargetType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastCaptureTargetType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCaptureTargetType;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const AuthorizationFail: Self = Self(2i32);
    pub const ForegroundAppActivated: Self = Self(3i32);
}
impl ::core::marker::Copy for AppBroadcastExitBroadcastModeReason {}
impl ::core::clone::Clone for AppBroadcastExitBroadcastModeReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastExitBroadcastModeReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastExitBroadcastModeReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastExitBroadcastModeReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastExitBroadcastModeReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastExitBroadcastModeReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastExitBroadcastModeReason;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastMicrophoneCaptureState {}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastMicrophoneCaptureState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastMicrophoneCaptureState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastMicrophoneCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastMicrophoneCaptureState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastMicrophoneCaptureState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastMicrophoneCaptureState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: Self = Self(0i32);
    pub const Initialized: Self = Self(1i32);
    pub const MicrosoftSignInRequired: Self = Self(2i32);
    pub const OAuthSignInRequired: Self = Self(3i32);
    pub const ProviderSignInRequired: Self = Self(4i32);
    pub const InBandwidthTest: Self = Self(5i32);
    pub const ReadyToBroadcast: Self = Self(6i32);
}
impl ::core::marker::Copy for AppBroadcastPlugInState {}
impl ::core::clone::Clone for AppBroadcastPlugInState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastPlugInState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastPlugInState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastPlugInState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPlugInState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPlugInState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastPreviewState {}
impl ::core::clone::Clone for AppBroadcastPreviewState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastPreviewState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastPreviewState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastPreviewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastPreviewState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPreviewState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: Self = Self(0i32);
    pub const AuthenticationFailed: Self = Self(1i32);
    pub const Unauthorized: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInResult {}
impl ::core::clone::Clone for AppBroadcastSignInResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastSignInResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastSignInResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastSignInResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastSignInResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInResult;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: Self = Self(0i32);
    pub const MicrosoftSignInInProgress: Self = Self(1i32);
    pub const MicrosoftSignInComplete: Self = Self(2i32);
    pub const OAuthSignInInProgress: Self = Self(3i32);
    pub const OAuthSignInComplete: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInState {}
impl ::core::clone::Clone for AppBroadcastSignInState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastSignInState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastSignInState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastSignInState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastSignInState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: Self = Self(0i32);
    pub const StreamReady: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Terminated: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastStreamState {}
impl ::core::clone::Clone for AppBroadcastStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastStreamState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastStreamState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastStreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastStreamState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastStreamState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: Self = Self(0i32);
    pub const LostConnectionToService: Self = Self(1i32);
    pub const NoNetworkConnectivity: Self = Self(2i32);
    pub const ServiceAbort: Self = Self(3i32);
    pub const ServiceError: Self = Self(4i32);
    pub const ServiceUnavailable: Self = Self(5i32);
    pub const InternalError: Self = Self(6i32);
    pub const UnsupportedFormat: Self = Self(7i32);
    pub const BackgroundTaskTerminated: Self = Self(8i32);
    pub const BackgroundTaskUnresponsive: Self = Self(9i32);
}
impl ::core::marker::Copy for AppBroadcastTerminationReason {}
impl ::core::clone::Clone for AppBroadcastTerminationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastTerminationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastTerminationReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastTerminationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTerminationReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastTerminationReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastTerminationReason;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastVideoEncodingBitrateMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastVideoEncodingBitrateMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastVideoEncodingBitrateMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingBitrateMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppBroadcastVideoEncodingResolutionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppBroadcastVideoEncodingResolutionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppBroadcastVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppBroadcastVideoEncodingResolutionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingResolutionMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: Self = Self(0i32);
    pub const Seconds: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureHistoricalBufferLengthUnit {}
impl ::core::clone::Clone for AppCaptureHistoricalBufferLengthUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureHistoricalBufferLengthUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureHistoricalBufferLengthUnit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureHistoricalBufferLengthUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureHistoricalBufferLengthUnit").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureHistoricalBufferLengthUnit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureHistoricalBufferLengthUnit;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureMetadataPriority {}
impl ::core::clone::Clone for AppCaptureMetadataPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureMetadataPriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureMetadataPriority {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureMetadataPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMetadataPriority").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureMetadataPriority {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMetadataPriority;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureMicrophoneCaptureState {}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureMicrophoneCaptureState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureMicrophoneCaptureState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureMicrophoneCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMicrophoneCaptureState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureMicrophoneCaptureState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMicrophoneCaptureState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureRecordingState {}
impl ::core::clone::Clone for AppCaptureRecordingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureRecordingState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureRecordingState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureRecordingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordingState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureRecordingState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureRecordingState;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureVideoEncodingBitrateMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureVideoEncodingBitrateMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureVideoEncodingBitrateMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingBitrateMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingFrameRateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingFrameRateMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureVideoEncodingFrameRateMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureVideoEncodingFrameRateMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingFrameRateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingFrameRateMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureVideoEncodingFrameRateMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingFrameRateMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCaptureVideoEncodingResolutionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppCaptureVideoEncodingResolutionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppCaptureVideoEncodingResolutionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingResolutionMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const VerySmallQvga: Self = Self(1i32);
    pub const SmallVga: Self = Self(2i32);
    pub const MediumXga: Self = Self(3i32);
    pub const Large3M: Self = Self(4i32);
    pub const VeryLarge5M: Self = Self(5i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxPhotoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxPhotoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIMaxPhotoResolution {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraCaptureUIMaxPhotoResolution {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraCaptureUIMaxPhotoResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxPhotoResolution").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIMaxPhotoResolution {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxPhotoResolution;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const LowDefinition: Self = Self(1i32);
    pub const StandardDefinition: Self = Self(2i32);
    pub const HighDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxVideoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxVideoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIMaxVideoResolution {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraCaptureUIMaxVideoResolution {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraCaptureUIMaxVideoResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxVideoResolution").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIMaxVideoResolution {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxVideoResolution;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIMode {}
impl ::core::clone::Clone for CameraCaptureUIMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraCaptureUIMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraCaptureUIMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIPhotoFormat {}
impl ::core::clone::Clone for CameraCaptureUIPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIPhotoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraCaptureUIPhotoFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraCaptureUIPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIPhotoFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIPhotoFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIPhotoFormat;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraCaptureUIVideoFormat {}
impl ::core::clone::Clone for CameraCaptureUIVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CameraCaptureUIVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CameraCaptureUIVideoFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CameraCaptureUIVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIVideoFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CameraCaptureUIVideoFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIVideoFormat;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: Self = Self(0i32);
    pub const MoreSettings: Self = Self(1i32);
}
impl ::core::marker::Copy for ForegroundActivationArgument {}
impl ::core::clone::Clone for ForegroundActivationArgument {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForegroundActivationArgument {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ForegroundActivationArgument {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ForegroundActivationArgument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForegroundActivationArgument").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ForegroundActivationArgument {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.ForegroundActivationArgument;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: Self = Self(0i32);
    pub const RecordHistoricalBuffer: Self = Self(1i32);
    pub const ToggleStartStopRecord: Self = Self(2i32);
    pub const StartRecord: Self = Self(3i32);
    pub const StopRecord: Self = Self(4i32);
    pub const TakeScreenshot: Self = Self(5i32);
    pub const StartBroadcast: Self = Self(6i32);
    pub const StopBroadcast: Self = Self(7i32);
    pub const PauseBroadcast: Self = Self(8i32);
    pub const ResumeBroadcast: Self = Self(9i32);
    pub const ToggleStartStopBroadcast: Self = Self(10i32);
    pub const ToggleMicrophoneCapture: Self = Self(11i32);
    pub const ToggleCameraCapture: Self = Self(12i32);
    pub const ToggleRecordingIndicator: Self = Self(13i32);
}
impl ::core::marker::Copy for GameBarCommand {}
impl ::core::clone::Clone for GameBarCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarCommand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GameBarCommand {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GameBarCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarCommand").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarCommand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommand;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: Self = Self(0i32);
    pub const Cortana: Self = Self(1i32);
    pub const AppCommand: Self = Self(2i32);
}
impl ::core::marker::Copy for GameBarCommandOrigin {}
impl ::core::clone::Clone for GameBarCommandOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarCommandOrigin {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GameBarCommandOrigin {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GameBarCommandOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarCommandOrigin").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarCommandOrigin {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommandOrigin;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: Self = Self(0i32);
    pub const FullScreenExclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for GameBarServicesDisplayMode {}
impl ::core::clone::Clone for GameBarServicesDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarServicesDisplayMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GameBarServicesDisplayMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GameBarServicesDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesDisplayMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarServicesDisplayMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarServicesDisplayMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: Self = Self(0i32);
    pub const EnabledByUser: Self = Self(1i32);
    pub const NotEnabled: Self = Self(2i32);
    pub const ProhibitedBySystem: Self = Self(3i32);
    pub const ProhibitedByPublisher: Self = Self(4i32);
}
impl ::core::marker::Copy for GameBarTargetCapturePolicy {}
impl ::core::clone::Clone for GameBarTargetCapturePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameBarTargetCapturePolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GameBarTargetCapturePolicy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GameBarTargetCapturePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarTargetCapturePolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameBarTargetCapturePolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarTargetCapturePolicy;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: Self = Self(0i32);
    pub const HighQualityPhoto: Self = Self(1i32);
    pub const BalancedVideoAndPhoto: Self = Self(2i32);
    pub const VideoConferencing: Self = Self(3i32);
    pub const PhotoSequence: Self = Self(4i32);
    pub const HighFrameRate: Self = Self(5i32);
    pub const VariablePhotoSequence: Self = Self(6i32);
    pub const HdrWithWcgVideo: Self = Self(7i32);
    pub const HdrWithWcgPhoto: Self = Self(8i32);
    pub const VideoHdr8: Self = Self(9i32);
    pub const CompressedCamera: Self = Self(10i32);
}
impl ::core::marker::Copy for KnownVideoProfile {}
impl ::core::clone::Clone for KnownVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownVideoProfile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KnownVideoProfile {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KnownVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownVideoProfile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KnownVideoProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.KnownVideoProfile;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureDeviceExclusiveControlReleaseMode(pub i32);
impl MediaCaptureDeviceExclusiveControlReleaseMode {
    pub const OnDispose: Self = Self(0i32);
    pub const OnAllStreamsStopped: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureDeviceExclusiveControlReleaseMode {}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlReleaseMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureDeviceExclusiveControlReleaseMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureDeviceExclusiveControlReleaseMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureDeviceExclusiveControlReleaseMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlReleaseMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlReleaseMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlReleaseMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: Self = Self(0i32);
    pub const SharedReadOnlyAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureDeviceExclusiveControlStatus {}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureDeviceExclusiveControlStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureDeviceExclusiveControlStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureDeviceExclusiveControlStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureDeviceExclusiveControlStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatus;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureMemoryPreference {}
impl ::core::clone::Clone for MediaCaptureMemoryPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureMemoryPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureMemoryPreference {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureMemoryPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureMemoryPreference").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureMemoryPreference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureMemoryPreference;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureSharingMode {}
impl ::core::clone::Clone for MediaCaptureSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureSharingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureSharingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureSharingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureSharingMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: Self = Self(0i32);
    pub const Overheated: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureThermalStatus {}
impl ::core::clone::Clone for MediaCaptureThermalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCaptureThermalStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCaptureThermalStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCaptureThermalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureThermalStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCaptureThermalStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureThermalStatus;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const GameChat: Self = Self(3i32);
    pub const Speech: Self = Self(4i32);
    pub const FarFieldSpeech: Self = Self(5i32);
    pub const UniformSpeech: Self = Self(6i32);
    pub const VoiceTyping: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaCategory {}
impl ::core::clone::Clone for MediaCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCategory;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: Self = Self(0i32);
    pub const VideoRecord: Self = Self(1i32);
    pub const Audio: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Metadata: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaStreamType {}
impl ::core::clone::Clone for MediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaStreamType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaStreamType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaStreamType;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: Self = Self(0i32);
    pub const VideoPreview: Self = Self(1i32);
    pub const Photo: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoCaptureSource {}
impl ::core::clone::Clone for PhotoCaptureSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoCaptureSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhotoCaptureSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhotoCaptureSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoCaptureSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhotoCaptureSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PhotoCaptureSource;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: Self = Self(0i32);
    pub const FiftyHertz: Self = Self(1i32);
    pub const SixtyHertz: Self = Self(2i32);
    pub const Auto: Self = Self(3i32);
}
impl ::core::marker::Copy for PowerlineFrequency {}
impl ::core::clone::Clone for PowerlineFrequency {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerlineFrequency {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PowerlineFrequency {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PowerlineFrequency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerlineFrequency").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PowerlineFrequency {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PowerlineFrequency;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamingCaptureMode {}
impl ::core::clone::Clone for StreamingCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StreamingCaptureMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StreamingCaptureMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StreamingCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamingCaptureMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StreamingCaptureMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.StreamingCaptureMode;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: Self = Self(0i32);
    pub const PreviewRecordStreamsIdentical: Self = Self(1i32);
    pub const PreviewPhotoStreamsIdentical: Self = Self(2i32);
    pub const RecordPhotoStreamsIdentical: Self = Self(3i32);
    pub const AllStreamsIdentical: Self = Self(4i32);
}
impl ::core::marker::Copy for VideoDeviceCharacteristic {}
impl ::core::clone::Clone for VideoDeviceCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoDeviceCharacteristic {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VideoDeviceCharacteristic {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoDeviceCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceCharacteristic").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoDeviceCharacteristic {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoDeviceCharacteristic;i4)");
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for VideoRotation {}
impl ::core::clone::Clone for VideoRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoRotation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VideoRotation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoRotation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoRotation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoRotation;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl ::core::marker::Copy for WhiteBalanceGain {}
impl ::core::clone::Clone for WhiteBalanceGain {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WhiteBalanceGain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WhiteBalanceGain").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::windows_core::TypeKind for WhiteBalanceGain {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for WhiteBalanceGain {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Capture.WhiteBalanceGain;f8;f8;f8)");
}
impl ::core::cmp::PartialEq for WhiteBalanceGain {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for WhiteBalanceGain {}
impl ::core::default::Default for WhiteBalanceGain {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFailedEventHandler(pub ::windows_core::IUnknown);
impl MediaCaptureFailedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaCapture>, ::core::option::Option<&MediaCaptureFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MediaCaptureFailedEventHandlerBox::<F> { vtable: &MediaCaptureFailedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, erroreventargs: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCapture>,
        P1: ::windows_core::IntoParam<MediaCaptureFailedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), erroreventargs.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct MediaCaptureFailedEventHandlerBox<F: FnMut(::core::option::Option<&MediaCapture>, ::core::option::Option<&MediaCaptureFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MediaCaptureFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaCapture>, ::core::option::Option<&MediaCaptureFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> MediaCaptureFailedEventHandlerBox<F> {
    const VTABLE: MediaCaptureFailedEventHandler_Vtbl = MediaCaptureFailedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MediaCaptureFailedEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, erroreventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&erroreventargs)).into()
    }
}
impl ::core::cmp::PartialEq for MediaCaptureFailedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCaptureFailedEventHandler {}
impl ::core::fmt::Debug for MediaCaptureFailedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureFailedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for MediaCaptureFailedEventHandler {
    type Vtable = MediaCaptureFailedEventHandler_Vtbl;
}
impl ::core::clone::Clone for MediaCaptureFailedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for MediaCaptureFailedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2014effb_5cd8_4f08_a314_0d360da59f14);
}
impl ::windows_core::RuntimeType for MediaCaptureFailedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2014effb-5cd8-4f08-a314-0d360da59f14}");
}
#[repr(C)]
#[doc(hidden)]
pub struct MediaCaptureFailedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, erroreventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct RecordLimitationExceededEventHandler(pub ::windows_core::IUnknown);
impl RecordLimitationExceededEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaCapture>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RecordLimitationExceededEventHandlerBox::<F> { vtable: &RecordLimitationExceededEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaCapture>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RecordLimitationExceededEventHandlerBox<F: FnMut(::core::option::Option<&MediaCapture>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RecordLimitationExceededEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaCapture>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RecordLimitationExceededEventHandlerBox<F> {
    const VTABLE: RecordLimitationExceededEventHandler_Vtbl = RecordLimitationExceededEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<RecordLimitationExceededEventHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::cmp::PartialEq for RecordLimitationExceededEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RecordLimitationExceededEventHandler {}
impl ::core::fmt::Debug for RecordLimitationExceededEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecordLimitationExceededEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RecordLimitationExceededEventHandler {
    type Vtable = RecordLimitationExceededEventHandler_Vtbl;
}
impl ::core::clone::Clone for RecordLimitationExceededEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for RecordLimitationExceededEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fae8f2e_4fe1_4ffd_aaba_e1f1337d4e53);
}
impl ::windows_core::RuntimeType for RecordLimitationExceededEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3fae8f2e-4fe1-4ffd-aaba-e1f1337d4e53}");
}
#[repr(C)]
#[doc(hidden)]
pub struct RecordLimitationExceededEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
