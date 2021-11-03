#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AdvancedCapturedPhoto(::windows::runtime::IInspectable);
impl AdvancedCapturedPhoto {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Frame(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture`, `Media_Devices`*"]
    pub fn Mode(&self) -> ::windows::runtime::Result<super::Devices::AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__: super::Devices::AdvancedPhotoMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::AdvancedPhotoMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Context(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = &::windows::runtime::Interface::cast::<IAdvancedCapturedPhoto2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Rect>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdvancedCapturedPhoto {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedCapturedPhoto;{f072728b-b292-4491-9d41-99807a550bbf})");
}
unsafe impl ::windows::runtime::Interface for AdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4034032267, 45714, 17553, [157, 65, 153, 128, 122, 85, 11, 191]);
}
impl ::windows::runtime::RuntimeName for AdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedCapturedPhoto";
}
unsafe impl ::std::marker::Send for AdvancedCapturedPhoto {}
unsafe impl ::std::marker::Sync for AdvancedCapturedPhoto {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AdvancedPhotoCapture(::windows::runtime::IInspectable);
impl AdvancedPhotoCapture {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureWithContextAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, context: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn OptionalReferencePhotoCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveOptionalReferencePhotoCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn AllPhotosCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveAllPhotosCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FinishAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdvancedPhotoCapture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedPhotoCapture;{83ffaafa-6667-44dc-973c-a6bce596aa0f})");
}
unsafe impl ::windows::runtime::Interface for AdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2214570746, 26215, 17628, [151, 60, 166, 188, 229, 150, 170, 15]);
}
impl ::windows::runtime::RuntimeName for AdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedPhotoCapture";
}
unsafe impl ::std::marker::Send for AdvancedPhotoCapture {}
unsafe impl ::std::marker::Sync for AdvancedPhotoCapture {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastBackgroundService(::windows::runtime::IInspectable);
impl AppBroadcastBackgroundService {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PlugInState(&self) -> ::windows::runtime::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetSignInInfo<'a, Param0: ::windows::runtime::IntoParam<'a, AppBroadcastBackgroundServiceSignInInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SignInInfo(&self) -> ::windows::runtime::Result<AppBroadcastBackgroundServiceSignInInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundServiceSignInInfo>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetStreamInfo<'a, Param0: ::windows::runtime::IntoParam<'a, AppBroadcastBackgroundServiceStreamInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StreamInfo(&self) -> ::windows::runtime::Result<AppBroadcastBackgroundServiceStreamInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundServiceStreamInfo>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BroadcastTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetViewerCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ViewerCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), reason, providerspecificreason).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn HeartbeatRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveHeartbeatRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TitleId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetBroadcastTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BroadcastLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetBroadcastLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BroadcastChannel(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetBroadcastChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn BroadcastTitleChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveBroadcastTitleChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn BroadcastLanguageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveBroadcastLanguageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn BroadcastChannelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveBroadcastChannelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastBackgroundService {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundService;{bad1e72a-fa94-46f9-95fc-d71511cda70b})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134318378, 64148, 18169, [149, 252, 215, 21, 17, 205, 167, 11]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundService";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastBackgroundServiceSignInInfo(::windows::runtime::IInspectable);
impl AppBroadcastBackgroundServiceSignInInfo {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SignInState(&self) -> ::windows::runtime::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetOAuthRequestUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn OAuthRequestUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetOAuthCallbackUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn OAuthCallbackUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `Media_Capture`, `Security_Authentication_Web`*"]
    pub fn AuthenticationResult(&self) -> ::windows::runtime::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetUserName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SignInStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveSignInStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn UserNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveUserNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo;{5e735275-88c8-4eca-89ba-4825985db880})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1584616053, 35016, 20170, [137, 186, 72, 37, 152, 93, 184, 128]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastBackgroundServiceStreamInfo(::windows::runtime::IInspectable);
impl AppBroadcastBackgroundServiceStreamInfo {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StreamState(&self) -> ::windows::runtime::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DesiredVideoEncodingBitrate(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BandwidthTestBitrate(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAudioCodec<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioCodec(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BroadcastStreamReader(&self) -> ::windows::runtime::Result<AppBroadcastStreamReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StreamStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn VideoEncodingResolutionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveVideoEncodingResolutionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn VideoEncodingBitrateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveVideoEncodingBitrateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ReportProblemWithStream(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppBroadcastBackgroundServiceStreamInfo2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo;{31dc02bc-990a-4904-aa96-fe364381f136})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(836502204, 39178, 18692, [170, 150, 254, 54, 67, 129, 241, 54]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo";
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(0i32);
    pub const Started: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(1i32);
    pub const Failed: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(2i32);
}
impl ::std::convert::From<i32> for AppBroadcastCameraCaptureState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastCameraCaptureState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastCameraCaptureState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraCaptureState;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastCameraCaptureState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastCameraCaptureStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraCaptureState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs;{1e334cd0-b882-4b88-8692-05999aceb70f})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(506678480, 47234, 19336, [134, 146, 5, 153, 154, 206, 183, 15]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppBroadcastCameraCaptureStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppBroadcastCameraCaptureStateChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(0i32);
    pub const TopCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(1i32);
    pub const TopRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(2i32);
    pub const MiddleLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(3i32);
    pub const MiddleCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(4i32);
    pub const MiddleRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(5i32);
    pub const BottomLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(6i32);
    pub const BottomCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(7i32);
    pub const BottomRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(8i32);
}
impl ::std::convert::From<i32> for AppBroadcastCameraOverlayLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastCameraOverlayLocation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastCameraOverlayLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlayLocation;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastCameraOverlayLocation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(0i32);
    pub const Medium: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(1i32);
    pub const Large: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(2i32);
}
impl ::std::convert::From<i32> for AppBroadcastCameraOverlaySize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastCameraOverlaySize {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastCameraOverlaySize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlaySize;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastCameraOverlaySize {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: AppBroadcastCaptureTargetType = AppBroadcastCaptureTargetType(0i32);
    pub const EntireDisplay: AppBroadcastCaptureTargetType = AppBroadcastCaptureTargetType(1i32);
}
impl ::std::convert::From<i32> for AppBroadcastCaptureTargetType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastCaptureTargetType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastCaptureTargetType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCaptureTargetType;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastCaptureTargetType {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct AppBroadcastContract(pub u8);
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(0i32);
    pub const UserCanceled: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(1i32);
    pub const AuthorizationFail: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(2i32);
    pub const ForegroundAppActivated: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(3i32);
}
impl ::std::convert::From<i32> for AppBroadcastExitBroadcastModeReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastExitBroadcastModeReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastExitBroadcastModeReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastExitBroadcastModeReason;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastExitBroadcastModeReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastGlobalSettings(::windows::runtime::IInspectable);
impl AppBroadcastGlobalSettings {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsBroadcastEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsDisabledByPolicy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn HasHardwareEncoder(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsAudioCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsEchoCancellationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SystemAudioGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MicrophoneGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCameraCaptureEnabledByDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetSelectedCameraId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SelectedCameraId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CameraOverlayLocation(&self) -> ::windows::runtime::Result<AppBroadcastCameraOverlayLocation> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraOverlayLocation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraOverlayLocation>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CameraOverlaySize(&self) -> ::windows::runtime::Result<AppBroadcastCameraOverlaySize> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraOverlaySize = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraOverlaySize>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastGlobalSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastGlobalSettings;{b2cb27a5-70fc-4e17-80bd-6ba0fd3ff3a0})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2999658405, 28924, 19991, [128, 189, 107, 160, 253, 63, 243, 160]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastGlobalSettings";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastHeartbeatRequestedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs;{cea54283-ee51-4dbf-9472-79a9ed4e2165})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3466936963, 61009, 19903, [148, 114, 121, 169, 237, 78, 33, 101]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs";
}
#[doc = "*Required features: `Media_Capture`*"]
pub struct AppBroadcastManager {}
impl AppBroadcastManager {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetGlobalSettings() -> ::windows::runtime::Result<AppBroadcastGlobalSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastGlobalSettings>(result__)
        })
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ApplyGlobalSettings<'a, Param0: ::windows::runtime::IntoParam<'a, AppBroadcastGlobalSettings>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetProviderSettings() -> ::windows::runtime::Result<AppBroadcastProviderSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastProviderSettings>(result__)
        })
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ApplyProviderSettings<'a, Param0: ::windows::runtime::IntoParam<'a, AppBroadcastProviderSettings>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn IAppBroadcastManagerStatics<R, F: FnOnce(&IAppBroadcastManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppBroadcastManager, IAppBroadcastManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AppBroadcastManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastManager";
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(0i32);
    pub const Started: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(1i32);
    pub const Failed: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(2i32);
}
impl ::std::convert::From<i32> for AppBroadcastMicrophoneCaptureState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastMicrophoneCaptureState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastMicrophoneCaptureState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastMicrophoneCaptureState;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastMicrophoneCaptureState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastMicrophoneCaptureState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs;{a86ad5e9-9440-4908-9d09-65b7e315d795})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2825573865, 37952, 18696, [157, 9, 101, 183, 227, 21, 215, 149]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPlugIn(::windows::runtime::IInspectable);
impl AppBroadcastPlugIn {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ProviderSettings(&self) -> ::windows::runtime::Result<AppBroadcastProviderSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastProviderSettings>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn Logo(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPlugIn {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugIn;{520c1e66-6513-4574-ac54-23b79729615b})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1376525926, 25875, 17780, [172, 84, 35, 183, 151, 41, 97, 91]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugIn";
}
unsafe impl ::std::marker::Send for AppBroadcastPlugIn {}
unsafe impl ::std::marker::Sync for AppBroadcastPlugIn {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPlugInManager(::windows::runtime::IInspectable);
impl AppBroadcastPlugInManager {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsBroadcastProviderAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn PlugInList(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DefaultPlugIn(&self) -> ::windows::runtime::Result<AppBroadcastPlugIn> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugIn>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetDefaultPlugIn<'a, Param0: ::windows::runtime::IntoParam<'a, AppBroadcastPlugIn>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    pub fn IAppBroadcastPlugInManagerStatics<R, F: FnOnce(&IAppBroadcastPlugInManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppBroadcastPlugInManager, IAppBroadcastPlugInManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPlugInManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInManager;{e550d979-27a1-49a7-bbf4-d7a9e9d07668})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3847281017, 10145, 18855, [187, 244, 215, 169, 233, 208, 118, 104]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInManager";
}
unsafe impl ::std::marker::Send for AppBroadcastPlugInManager {}
unsafe impl ::std::marker::Sync for AppBroadcastPlugInManager {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: AppBroadcastPlugInState = AppBroadcastPlugInState(0i32);
    pub const Initialized: AppBroadcastPlugInState = AppBroadcastPlugInState(1i32);
    pub const MicrosoftSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(2i32);
    pub const OAuthSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(3i32);
    pub const ProviderSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(4i32);
    pub const InBandwidthTest: AppBroadcastPlugInState = AppBroadcastPlugInState(5i32);
    pub const ReadyToBroadcast: AppBroadcastPlugInState = AppBroadcastPlugInState(6i32);
}
impl ::std::convert::From<i32> for AppBroadcastPlugInState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastPlugInState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPlugInState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPlugInState;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastPlugInState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPlugInStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastPlugInStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PlugInState(&self) -> ::windows::runtime::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs;{4881d0f2-abc5-4fc6-84b0-89370bb47212})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1216467186, 43973, 20422, [132, 176, 137, 55, 11, 180, 114, 18]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppBroadcastPlugInStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppBroadcastPlugInStateChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPreview(::windows::runtime::IInspectable);
impl AppBroadcastPreview {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StopPreview(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PreviewState(&self) -> ::windows::runtime::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPreviewState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn PreviewStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemovePreviewStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PreviewStreamReader(&self) -> ::windows::runtime::Result<AppBroadcastPreviewStreamReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamReader>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreview;{14b60f5a-6e4a-4b80-a14f-67ee77d153e7})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(347475802, 28234, 19328, [161, 79, 103, 238, 119, 209, 83, 231]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreview";
}
unsafe impl ::std::marker::Send for AppBroadcastPreview {}
unsafe impl ::std::marker::Sync for AppBroadcastPreview {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: AppBroadcastPreviewState = AppBroadcastPreviewState(0i32);
    pub const Stopped: AppBroadcastPreviewState = AppBroadcastPreviewState(1i32);
    pub const Failed: AppBroadcastPreviewState = AppBroadcastPreviewState(2i32);
}
impl ::std::convert::From<i32> for AppBroadcastPreviewState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastPreviewState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPreviewState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPreviewState;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastPreviewState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPreviewStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastPreviewStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PreviewState(&self) -> ::windows::runtime::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPreviewState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs;{5a57f2de-8dea-4e86-90ad-03fc26b9653c})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1515713246, 36330, 20102, [144, 173, 3, 252, 38, 185, 101, 60]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppBroadcastPreviewStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppBroadcastPreviewStateChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPreviewStreamReader(::windows::runtime::IInspectable);
impl AppBroadcastPreviewStreamReader {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoStride(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_Capture`, `Graphics_Imaging`*"]
    pub fn VideoBitmapPixelFormat(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapPixelFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_Capture`, `Graphics_Imaging`*"]
    pub fn VideoBitmapAlphaMode(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapAlphaMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TryGetNextVideoFrame(&self) -> ::windows::runtime::Result<AppBroadcastPreviewStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamVideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn VideoFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPreviewStreamReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamReader;{92228d50-db3f-40a8-8cd4-f4e371ddab37})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2451737936, 56127, 16552, [140, 212, 244, 227, 113, 221, 171, 55]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamReader";
}
unsafe impl ::std::marker::Send for AppBroadcastPreviewStreamReader {}
unsafe impl ::std::marker::Sync for AppBroadcastPreviewStreamReader {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPreviewStreamVideoFrame(::windows::runtime::IInspectable);
impl AppBroadcastPreviewStreamVideoFrame {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoHeader(&self) -> ::windows::runtime::Result<AppBroadcastPreviewStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamVideoHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn VideoBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame;{010fbea1-94fe-4499-b8c0-8d244279fb12})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(17809057, 38142, 17561, [184, 192, 141, 36, 66, 121, 251, 18]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame";
}
unsafe impl ::std::marker::Send for AppBroadcastPreviewStreamVideoFrame {}
unsafe impl ::std::marker::Sync for AppBroadcastPreviewStreamVideoFrame {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastPreviewStreamVideoHeader(::windows::runtime::IInspectable);
impl AppBroadcastPreviewStreamVideoHeader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn AbsoluteTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RelativeTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn FrameId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader;{8bef6113-da84-4499-a7ab-87118cb4a157})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2347720979, 55940, 17561, [167, 171, 135, 17, 140, 180, 161, 87]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader";
}
unsafe impl ::std::marker::Send for AppBroadcastPreviewStreamVideoHeader {}
unsafe impl ::std::marker::Sync for AppBroadcastPreviewStreamVideoHeader {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastProviderSettings(::windows::runtime::IInspectable);
impl AppBroadcastProviderSettings {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetDefaultBroadcastTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DefaultBroadcastTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioEncodingBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CustomVideoEncodingHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CustomVideoEncodingWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoEncodingBitrateMode(&self) -> ::windows::runtime::Result<AppBroadcastVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastVideoEncodingBitrateMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastVideoEncodingBitrateMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoEncodingResolutionMode(&self) -> ::windows::runtime::Result<AppBroadcastVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastVideoEncodingResolutionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastVideoEncodingResolutionMode>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastProviderSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastProviderSettings;{c30bdf62-9948-458f-ad50-aa06ec03da08})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3272335202, 39240, 17807, [173, 80, 170, 6, 236, 3, 218, 8]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastProviderSettings";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastServices(::windows::runtime::IInspectable);
impl AppBroadcastServices {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CaptureTargetType(&self) -> ::windows::runtime::Result<AppBroadcastCaptureTargetType> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCaptureTargetType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCaptureTargetType>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BroadcastTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetBroadcastTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BroadcastLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetBroadcastLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CanCapture(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn EnterBroadcastModeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, AppBroadcastPlugIn>>(&self, plugin: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), plugin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), reason).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StartBroadcast(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PauseBroadcast(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ResumeBroadcast(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StartPreview<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, desiredsize: Param0) -> ::windows::runtime::Result<AppBroadcastPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), desiredsize.into_param().abi(), &mut result__).from_abi::<AppBroadcastPreview>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppBroadcastState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastServices {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastServices;{8660b4d6-969b-4e3c-ac3a-8b042ee4ee63})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastServices {
    type Vtable = IAppBroadcastServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2254484694, 38555, 20028, [172, 58, 139, 4, 46, 228, 238, 99]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastServices";
}
unsafe impl ::std::marker::Send for AppBroadcastServices {}
unsafe impl ::std::marker::Sync for AppBroadcastServices {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: AppBroadcastSignInResult = AppBroadcastSignInResult(0i32);
    pub const AuthenticationFailed: AppBroadcastSignInResult = AppBroadcastSignInResult(1i32);
    pub const Unauthorized: AppBroadcastSignInResult = AppBroadcastSignInResult(2i32);
    pub const ServiceUnavailable: AppBroadcastSignInResult = AppBroadcastSignInResult(3i32);
    pub const Unknown: AppBroadcastSignInResult = AppBroadcastSignInResult(4i32);
}
impl ::std::convert::From<i32> for AppBroadcastSignInResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastSignInResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastSignInResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInResult;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastSignInResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: AppBroadcastSignInState = AppBroadcastSignInState(0i32);
    pub const MicrosoftSignInInProgress: AppBroadcastSignInState = AppBroadcastSignInState(1i32);
    pub const MicrosoftSignInComplete: AppBroadcastSignInState = AppBroadcastSignInState(2i32);
    pub const OAuthSignInInProgress: AppBroadcastSignInState = AppBroadcastSignInState(3i32);
    pub const OAuthSignInComplete: AppBroadcastSignInState = AppBroadcastSignInState(4i32);
}
impl ::std::convert::From<i32> for AppBroadcastSignInState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastSignInState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastSignInState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInState;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastSignInState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastSignInStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastSignInStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SignInState(&self) -> ::windows::runtime::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<AppBroadcastSignInResult> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs;{02b692a4-5919-4a9e-8d5e-c9bb0dd3377a})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(45519524, 22809, 19102, [141, 94, 201, 187, 13, 211, 55, 122]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastState(::windows::runtime::IInspectable);
impl AppBroadcastState {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCaptureTargetRunning(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ViewerCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ShouldCaptureMicrophone(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn RestartMicrophoneCapture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ShouldCaptureCamera(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetShouldCaptureCamera(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn RestartCameraCapture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn EncodedVideoSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MicrophoneCaptureState(&self) -> ::windows::runtime::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastMicrophoneCaptureState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MicrophoneCaptureError(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CameraCaptureState(&self) -> ::windows::runtime::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraCaptureState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CameraCaptureError(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StreamState(&self) -> ::windows::runtime::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PlugInState(&self) -> ::windows::runtime::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn OAuthRequestUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn OAuthCallbackUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `Media_Capture`, `Security_Authentication_Web`*"]
    pub fn AuthenticationResult(&self) -> ::windows::runtime::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `Media_Capture`, `Security_Authentication_Web`*"]
    pub fn SetAuthenticationResult<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Authentication::Web::WebAuthenticationResult>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SignInState(&self) -> ::windows::runtime::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TerminationReason(&self) -> ::windows::runtime::Result<AppBroadcastTerminationReason> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastTerminationReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastTerminationReason>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TerminationReasonPlugInSpecific(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ViewerCountChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveViewerCountChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CameraCaptureStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCameraCaptureStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn PlugInStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemovePlugInStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StreamStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureTargetClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastState;{ee08056d-8099-4ddd-922e-c56dac58abfb})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastState {
    type Vtable = IAppBroadcastState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3993503085, 32921, 19933, [146, 46, 197, 109, 172, 88, 171, 251]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastState";
}
unsafe impl ::std::marker::Send for AppBroadcastState {}
unsafe impl ::std::marker::Sync for AppBroadcastState {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastStreamAudioFrame(::windows::runtime::IInspectable);
impl AppBroadcastStreamAudioFrame {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioHeader(&self) -> ::windows::runtime::Result<AppBroadcastStreamAudioHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamAudioHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn AudioBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamAudioFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioFrame;{efab4ac8-21ba-453f-8bb7-5e938a2e9a74})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020980424, 8634, 17727, [139, 183, 94, 147, 138, 46, 154, 116]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioFrame";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastStreamAudioHeader(::windows::runtime::IInspectable);
impl AppBroadcastStreamAudioHeader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn AbsoluteTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RelativeTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn HasDiscontinuity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn FrameId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamAudioHeader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioHeader;{bf21a570-6b78-4216-9f07-5aff5256f1b7})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3206653296, 27512, 16918, [159, 7, 90, 255, 82, 86, 241, 183]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioHeader";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastStreamReader(::windows::runtime::IInspectable);
impl AppBroadcastStreamReader {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioChannels(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioSampleRate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn AudioAacSequence(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TryGetNextAudioFrame(&self) -> ::windows::runtime::Result<AppBroadcastStreamAudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamAudioFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TryGetNextVideoFrame(&self) -> ::windows::runtime::Result<AppBroadcastStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamVideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn AudioFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveAudioFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn VideoFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamReader;{b338bcf9-3364-4460-b5f1-3cc2796a8aa2})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3006840057, 13156, 17504, [181, 241, 60, 194, 121, 106, 138, 162]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamReader";
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: AppBroadcastStreamState = AppBroadcastStreamState(0i32);
    pub const StreamReady: AppBroadcastStreamState = AppBroadcastStreamState(1i32);
    pub const Started: AppBroadcastStreamState = AppBroadcastStreamState(2i32);
    pub const Paused: AppBroadcastStreamState = AppBroadcastStreamState(3i32);
    pub const Terminated: AppBroadcastStreamState = AppBroadcastStreamState(4i32);
}
impl ::std::convert::From<i32> for AppBroadcastStreamState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastStreamState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastStreamState;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastStreamState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastStreamStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastStreamStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StreamState(&self) -> ::windows::runtime::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs;{5108a733-d008-4a89-93be-58aed961374e})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1359521587, 53256, 19081, [147, 190, 88, 174, 217, 97, 55, 78]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastStreamVideoFrame(::windows::runtime::IInspectable);
impl AppBroadcastStreamVideoFrame {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoHeader(&self) -> ::windows::runtime::Result<AppBroadcastStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamVideoHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn VideoBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamVideoFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoFrame;{0f97cf2b-c9e4-4e88-8194-d814cbd585d8})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(261607211, 51684, 20104, [129, 148, 216, 20, 203, 213, 133, 216]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoFrame";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastStreamVideoHeader(::windows::runtime::IInspectable);
impl AppBroadcastStreamVideoHeader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn AbsoluteTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RelativeTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsKeyFrame(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn HasDiscontinuity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn FrameId(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastStreamVideoHeader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoHeader;{0b9ebece-7e32-432d-8ca2-36bf10b9f462})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(194952910, 32306, 17197, [140, 162, 54, 191, 16, 185, 244, 98]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoHeader";
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: AppBroadcastTerminationReason = AppBroadcastTerminationReason(0i32);
    pub const LostConnectionToService: AppBroadcastTerminationReason = AppBroadcastTerminationReason(1i32);
    pub const NoNetworkConnectivity: AppBroadcastTerminationReason = AppBroadcastTerminationReason(2i32);
    pub const ServiceAbort: AppBroadcastTerminationReason = AppBroadcastTerminationReason(3i32);
    pub const ServiceError: AppBroadcastTerminationReason = AppBroadcastTerminationReason(4i32);
    pub const ServiceUnavailable: AppBroadcastTerminationReason = AppBroadcastTerminationReason(5i32);
    pub const InternalError: AppBroadcastTerminationReason = AppBroadcastTerminationReason(6i32);
    pub const UnsupportedFormat: AppBroadcastTerminationReason = AppBroadcastTerminationReason(7i32);
    pub const BackgroundTaskTerminated: AppBroadcastTerminationReason = AppBroadcastTerminationReason(8i32);
    pub const BackgroundTaskUnresponsive: AppBroadcastTerminationReason = AppBroadcastTerminationReason(9i32);
}
impl ::std::convert::From<i32> for AppBroadcastTerminationReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastTerminationReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastTerminationReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastTerminationReason;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastTerminationReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastTriggerDetails(::windows::runtime::IInspectable);
impl AppBroadcastTriggerDetails {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn BackgroundService(&self) -> ::windows::runtime::Result<AppBroadcastBackgroundService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundService>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastTriggerDetails;{deebab35-ec5e-4d8f-b1c0-5da6e8c75638})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3739986741, 60510, 19855, [177, 192, 93, 166, 232, 199, 86, 56]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastTriggerDetails";
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: AppBroadcastVideoEncodingBitrateMode = AppBroadcastVideoEncodingBitrateMode(0i32);
    pub const Auto: AppBroadcastVideoEncodingBitrateMode = AppBroadcastVideoEncodingBitrateMode(1i32);
}
impl ::std::convert::From<i32> for AppBroadcastVideoEncodingBitrateMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastVideoEncodingBitrateMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastVideoEncodingBitrateMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingBitrateMode;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastVideoEncodingBitrateMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: AppBroadcastVideoEncodingResolutionMode = AppBroadcastVideoEncodingResolutionMode(0i32);
    pub const Auto: AppBroadcastVideoEncodingResolutionMode = AppBroadcastVideoEncodingResolutionMode(1i32);
}
impl ::std::convert::From<i32> for AppBroadcastVideoEncodingResolutionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppBroadcastVideoEncodingResolutionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastVideoEncodingResolutionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingResolutionMode;i4)");
}
impl ::windows::runtime::DefaultType for AppBroadcastVideoEncodingResolutionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastViewerCountChangedEventArgs(::windows::runtime::IInspectable);
impl AppBroadcastViewerCountChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ViewerCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs;{e6e11825-5401-4ade-8bd2-c14ecee6807d})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3873511461, 21505, 19166, [139, 210, 193, 78, 206, 230, 128, 125]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppBroadcastViewerCountChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppBroadcastViewerCountChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCapture(::windows::runtime::IInspectable);
impl AppCapture {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCapturingAudio(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCapturingVideo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CapturingChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCapture, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCapturingChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<AppCapture> {
        Self::IAppCaptureStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCapture>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetAllowedAsync(allowed: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppCaptureStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), allowed, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IAppCaptureStatics<R, F: FnOnce(&IAppCaptureStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppCapture, IAppCaptureStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppCaptureStatics2<R, F: FnOnce(&IAppCaptureStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppCapture, IAppCaptureStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCapture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCapture;{9749d453-a29a-45ed-8f29-22d09942cff7})");
}
unsafe impl ::windows::runtime::Interface for AppCapture {
    type Vtable = IAppCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2538198099, 41626, 17901, [143, 41, 34, 208, 153, 66, 207, 247]);
}
impl ::windows::runtime::RuntimeName for AppCapture {
    const NAME: &'static str = "Windows.Media.Capture.AppCapture";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureAlternateShortcutKeys(::windows::runtime::IInspectable);
impl AppCaptureAlternateShortcutKeys {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleGameBarKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleGameBarKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SaveHistoricalVideoKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleRecordingKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleRecordingKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn TakeScreenshotKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn TakeScreenshotKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleRecordingIndicatorKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleMicrophoneCaptureKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleCameraCaptureKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleBroadcastKey(&self) -> ::windows::runtime::Result<super::super::System::VirtualKey> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_Capture`, `System`*"]
    pub fn ToggleBroadcastKeyModifiers(&self) -> ::windows::runtime::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureAlternateShortcutKeys {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureAlternateShortcutKeys;{19e8e0ef-236c-40f9-b38f-9b7dd65d1ccc})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(434692335, 9068, 16633, [179, 143, 155, 125, 214, 93, 28, 204]);
}
impl ::windows::runtime::RuntimeName for AppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureAlternateShortcutKeys";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct AppCaptureContract(pub u8);
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureDurationGeneratedEventArgs(::windows::runtime::IInspectable);
impl AppCaptureDurationGeneratedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs;{c1f5563b-ffa1-44c9-975f-27fbeb553b35})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3254081083, 65441, 17609, [151, 95, 39, 251, 235, 85, 59, 53]);
}
impl ::windows::runtime::RuntimeName for AppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs";
}
unsafe impl ::std::marker::Send for AppCaptureDurationGeneratedEventArgs {}
unsafe impl ::std::marker::Sync for AppCaptureDurationGeneratedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureFileGeneratedEventArgs(::windows::runtime::IInspectable);
impl AppCaptureFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_Capture`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureFileGeneratedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureFileGeneratedEventArgs;{4189fbf4-465e-45bf-907f-165b3fb23758})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099561972, 18014, 17855, [144, 127, 22, 91, 63, 178, 55, 88]);
}
impl ::windows::runtime::RuntimeName for AppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureFileGeneratedEventArgs";
}
unsafe impl ::std::marker::Send for AppCaptureFileGeneratedEventArgs {}
unsafe impl ::std::marker::Sync for AppCaptureFileGeneratedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: AppCaptureHistoricalBufferLengthUnit = AppCaptureHistoricalBufferLengthUnit(0i32);
    pub const Seconds: AppCaptureHistoricalBufferLengthUnit = AppCaptureHistoricalBufferLengthUnit(1i32);
}
impl ::std::convert::From<i32> for AppCaptureHistoricalBufferLengthUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureHistoricalBufferLengthUnit {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureHistoricalBufferLengthUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureHistoricalBufferLengthUnit;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureHistoricalBufferLengthUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
pub struct AppCaptureManager {}
impl AppCaptureManager {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetCurrentSettings() -> ::windows::runtime::Result<AppCaptureSettings> {
        Self::IAppCaptureManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureSettings>(result__)
        })
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ApplySettings<'a, Param0: ::windows::runtime::IntoParam<'a, AppCaptureSettings>>(appcapturesettings: Param0) -> ::windows::runtime::Result<()> {
        Self::IAppCaptureManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), appcapturesettings.into_param().abi()).ok() })
    }
    pub fn IAppCaptureManagerStatics<R, F: FnOnce(&IAppCaptureManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppCaptureManager, IAppCaptureManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AppCaptureManager {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureManager";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct AppCaptureMetadataContract(pub u8);
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: AppCaptureMetadataPriority = AppCaptureMetadataPriority(0i32);
    pub const Important: AppCaptureMetadataPriority = AppCaptureMetadataPriority(1i32);
}
impl ::std::convert::From<i32> for AppCaptureMetadataPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureMetadataPriority {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureMetadataPriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMetadataPriority;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureMetadataPriority {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureMetadataWriter(::windows::runtime::IInspectable);
impl AppCaptureMetadataWriter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppCaptureMetadataWriter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AddStringEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AddInt32Event<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AddDoubleEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StartStringState<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StartInt32State<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StartDoubleState<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StopState<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StopAllStates(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn RemainingStorageBytesAvailable(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn MetadataPurged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveMetadataPurged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureMetadataWriter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMetadataWriter;{e0ce4877-9aaf-46b4-ad31-6a60b441c780})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3771615351, 39599, 18100, [173, 49, 106, 96, 180, 65, 199, 128]);
}
impl ::windows::runtime::RuntimeName for AppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMetadataWriter";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AppCaptureMetadataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppCaptureMetadataWriter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AppCaptureMetadataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppCaptureMetadataWriter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AppCaptureMetadataWriter {}
unsafe impl ::std::marker::Sync for AppCaptureMetadataWriter {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(0i32);
    pub const Started: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(1i32);
    pub const Failed: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(2i32);
}
impl ::std::convert::From<i32> for AppCaptureMicrophoneCaptureState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureMicrophoneCaptureState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureMicrophoneCaptureState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMicrophoneCaptureState;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureMicrophoneCaptureState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppCaptureMicrophoneCaptureStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureMicrophoneCaptureState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs;{324d249e-45bc-4c35-bc35-e469fc7a69e0})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(843916446, 17852, 19509, [188, 53, 228, 105, 252, 122, 105, 224]);
}
impl ::windows::runtime::RuntimeName for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureRecordOperation(::windows::runtime::IInspectable);
impl AppCaptureRecordOperation {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StopRecording(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureRecordingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_Capture`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn IsFileTruncated(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn DurationGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveDurationGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureRecordOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordOperation;{c66020a9-1538-495c-9bbb-2ba870ec5861})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3328188585, 5432, 18780, [155, 187, 43, 168, 112, 236, 88, 97]);
}
impl ::windows::runtime::RuntimeName for AppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordOperation";
}
unsafe impl ::std::marker::Send for AppCaptureRecordOperation {}
unsafe impl ::std::marker::Sync for AppCaptureRecordOperation {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: AppCaptureRecordingState = AppCaptureRecordingState(0i32);
    pub const Completed: AppCaptureRecordingState = AppCaptureRecordingState(1i32);
    pub const Failed: AppCaptureRecordingState = AppCaptureRecordingState(2i32);
}
impl ::std::convert::From<i32> for AppCaptureRecordingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureRecordingState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureRecordingState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureRecordingState;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureRecordingState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureRecordingStateChangedEventArgs(::windows::runtime::IInspectable);
impl AppCaptureRecordingStateChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureRecordingState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs;{24fc8712-e305-490d-b415-6b1c9049736b})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(620529426, 58117, 18701, [180, 21, 107, 28, 144, 73, 115, 107]);
}
impl ::windows::runtime::RuntimeName for AppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for AppCaptureRecordingStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AppCaptureRecordingStateChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureServices(::windows::runtime::IInspectable);
impl AppCaptureServices {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Record(&self) -> ::windows::runtime::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RecordTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, starttime: Param0, duration: Param1) -> ::windows::runtime::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CanCapture(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn State(&self) -> ::windows::runtime::Result<AppCaptureState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureServices {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureServices;{44fec0b5-34f5-4f18-ae8c-b9123abbfc0d})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureServices {
    type Vtable = IAppCaptureServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1157546165, 13557, 20248, [174, 140, 185, 18, 58, 187, 252, 13]);
}
impl ::windows::runtime::RuntimeName for AppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureServices";
}
unsafe impl ::std::marker::Send for AppCaptureServices {}
unsafe impl ::std::marker::Sync for AppCaptureServices {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureSettings(::windows::runtime::IInspectable);
impl AppCaptureSettings {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_Capture`, `Storage`*"]
    pub fn SetAppCaptureDestinationFolder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_Capture`, `Storage`*"]
    pub fn AppCaptureDestinationFolder(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioEncodingBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsAudioCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CustomVideoEncodingHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CustomVideoEncodingWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetHistoricalBufferLength(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn HistoricalBufferLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn HistoricalBufferLengthUnit(&self) -> ::windows::runtime::Result<AppCaptureHistoricalBufferLengthUnit> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureHistoricalBufferLengthUnit = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureHistoricalBufferLengthUnit>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetMaximumRecordLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn MaximumRecordLength(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_Capture`, `Storage`*"]
    pub fn SetScreenshotDestinationFolder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Media_Capture`, `Storage`*"]
    pub fn ScreenshotDestinationFolder(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoEncodingBitrateMode(&self) -> ::windows::runtime::Result<AppCaptureVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureVideoEncodingBitrateMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingBitrateMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoEncodingResolutionMode(&self) -> ::windows::runtime::Result<AppCaptureVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureVideoEncodingResolutionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingResolutionMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsAppCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCpuConstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsDisabledByPolicy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsMemoryConstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn HasHardwareEncoder(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AlternateShortcutKeys(&self) -> ::windows::runtime::Result<AppCaptureAlternateShortcutKeys> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureAlternateShortcutKeys>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsMicrophoneCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SystemAudioGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MicrophoneGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoEncodingFrameRateMode(&self) -> ::windows::runtime::Result<AppCaptureVideoEncodingFrameRateMode> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: AppCaptureVideoEncodingFrameRateMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingFrameRateMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsEchoCancellationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureSettings;{14683a86-8807-48d3-883a-970ee4532a39})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureSettings {
    type Vtable = IAppCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(342375046, 34823, 18643, [136, 58, 151, 14, 228, 83, 42, 57]);
}
impl ::windows::runtime::RuntimeName for AppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureSettings";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppCaptureState(::windows::runtime::IInspectable);
impl AppCaptureState {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsTargetRunning(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ShouldCaptureMicrophone(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn RestartMicrophoneCapture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MicrophoneCaptureState(&self) -> ::windows::runtime::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureMicrophoneCaptureState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MicrophoneCaptureError(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureTargetClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureState;{73134372-d4eb-44ce-9538-465f506ac4ea})");
}
unsafe impl ::windows::runtime::Interface for AppCaptureState {
    type Vtable = IAppCaptureState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930642290, 54507, 17614, [149, 56, 70, 95, 80, 106, 196, 234]);
}
impl ::windows::runtime::RuntimeName for AppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureState";
}
unsafe impl ::std::marker::Send for AppCaptureState {}
unsafe impl ::std::marker::Sync for AppCaptureState {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(0i32);
    pub const High: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(1i32);
    pub const Standard: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(2i32);
}
impl ::std::convert::From<i32> for AppCaptureVideoEncodingBitrateMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureVideoEncodingBitrateMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureVideoEncodingBitrateMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingBitrateMode;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureVideoEncodingBitrateMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: AppCaptureVideoEncodingFrameRateMode = AppCaptureVideoEncodingFrameRateMode(0i32);
    pub const High: AppCaptureVideoEncodingFrameRateMode = AppCaptureVideoEncodingFrameRateMode(1i32);
}
impl ::std::convert::From<i32> for AppCaptureVideoEncodingFrameRateMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureVideoEncodingFrameRateMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureVideoEncodingFrameRateMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingFrameRateMode;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureVideoEncodingFrameRateMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(0i32);
    pub const High: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(1i32);
    pub const Standard: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(2i32);
}
impl ::std::convert::From<i32> for AppCaptureVideoEncodingResolutionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppCaptureVideoEncodingResolutionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AppCaptureVideoEncodingResolutionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingResolutionMode;i4)");
}
impl ::windows::runtime::DefaultType for AppCaptureVideoEncodingResolutionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CameraCaptureUI(::windows::runtime::IInspectable);
impl CameraCaptureUI {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CameraCaptureUI, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PhotoSettings(&self) -> ::windows::runtime::Result<CameraCaptureUIPhotoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIPhotoCaptureSettings>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoSettings(&self) -> ::windows::runtime::Result<CameraCaptureUIVideoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIVideoCaptureSettings>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Storage`*"]
    pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUI {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUI;{48587540-6f93-4bb4-b8f3-e89e48948c91})");
}
unsafe impl ::windows::runtime::Interface for CameraCaptureUI {
    type Vtable = ICameraCaptureUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1213756736, 28563, 19380, [184, 243, 232, 158, 72, 148, 140, 145]);
}
impl ::windows::runtime::RuntimeName for CameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUI";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct CameraCaptureUIContract(pub u8);
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(0i32);
    pub const VerySmallQvga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(1i32);
    pub const SmallVga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(2i32);
    pub const MediumXga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(3i32);
    pub const Large3M: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(4i32);
    pub const VeryLarge5M: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(5i32);
}
impl ::std::convert::From<i32> for CameraCaptureUIMaxPhotoResolution {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraCaptureUIMaxPhotoResolution {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIMaxPhotoResolution {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxPhotoResolution;i4)");
}
impl ::windows::runtime::DefaultType for CameraCaptureUIMaxPhotoResolution {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(0i32);
    pub const LowDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(1i32);
    pub const StandardDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(2i32);
    pub const HighDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(3i32);
}
impl ::std::convert::From<i32> for CameraCaptureUIMaxVideoResolution {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraCaptureUIMaxVideoResolution {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIMaxVideoResolution {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxVideoResolution;i4)");
}
impl ::windows::runtime::DefaultType for CameraCaptureUIMaxVideoResolution {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: CameraCaptureUIMode = CameraCaptureUIMode(0i32);
    pub const Photo: CameraCaptureUIMode = CameraCaptureUIMode(1i32);
    pub const Video: CameraCaptureUIMode = CameraCaptureUIMode(2i32);
}
impl ::std::convert::From<i32> for CameraCaptureUIMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraCaptureUIMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMode;i4)");
}
impl ::windows::runtime::DefaultType for CameraCaptureUIMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CameraCaptureUIPhotoCaptureSettings(::windows::runtime::IInspectable);
impl CameraCaptureUIPhotoCaptureSettings {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<CameraCaptureUIPhotoFormat> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIPhotoFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIPhotoFormat>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MaxResolution(&self) -> ::windows::runtime::Result<CameraCaptureUIMaxPhotoResolution> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIMaxPhotoResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIMaxPhotoResolution>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CroppedSizeInPixels(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetCroppedSizeInPixels<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CroppedAspectRatio(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetCroppedAspectRatio<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AllowCropping(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAllowCropping(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings;{b9f5be97-3472-46a8-8a9e-04ce42ccc97d})");
}
unsafe impl ::windows::runtime::Interface for CameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3119890071, 13426, 18088, [138, 158, 4, 206, 66, 204, 201, 125]);
}
impl ::windows::runtime::RuntimeName for CameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings";
}
unsafe impl ::std::marker::Send for CameraCaptureUIPhotoCaptureSettings {}
unsafe impl ::std::marker::Sync for CameraCaptureUIPhotoCaptureSettings {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(0i32);
    pub const Png: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(1i32);
    pub const JpegXR: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(2i32);
}
impl ::std::convert::From<i32> for CameraCaptureUIPhotoFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraCaptureUIPhotoFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIPhotoFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIPhotoFormat;i4)");
}
impl ::windows::runtime::DefaultType for CameraCaptureUIPhotoFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CameraCaptureUIVideoCaptureSettings(::windows::runtime::IInspectable);
impl CameraCaptureUIVideoCaptureSettings {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<CameraCaptureUIVideoFormat> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIVideoFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIVideoFormat>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MaxResolution(&self) -> ::windows::runtime::Result<CameraCaptureUIMaxVideoResolution> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIMaxVideoResolution = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIMaxVideoResolution>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MaxDurationInSeconds(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AllowTrimming(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAllowTrimming(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings;{64e92d1f-a28d-425a-b84f-e568335ff24e})");
}
unsafe impl ::windows::runtime::Interface for CameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1693003039, 41613, 16986, [184, 79, 229, 104, 51, 95, 242, 78]);
}
impl ::windows::runtime::RuntimeName for CameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings";
}
unsafe impl ::std::marker::Send for CameraCaptureUIVideoCaptureSettings {}
unsafe impl ::std::marker::Sync for CameraCaptureUIVideoCaptureSettings {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: CameraCaptureUIVideoFormat = CameraCaptureUIVideoFormat(0i32);
    pub const Wmv: CameraCaptureUIVideoFormat = CameraCaptureUIVideoFormat(1i32);
}
impl ::std::convert::From<i32> for CameraCaptureUIVideoFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraCaptureUIVideoFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CameraCaptureUIVideoFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIVideoFormat;i4)");
}
impl ::windows::runtime::DefaultType for CameraCaptureUIVideoFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
pub struct CameraOptionsUI {}
impl CameraOptionsUI {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCapture>>(mediacapture: Param0) -> ::windows::runtime::Result<()> {
        Self::ICameraOptionsUIStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mediacapture.into_param().abi()).ok() })
    }
    pub fn ICameraOptionsUIStatics<R, F: FnOnce(&ICameraOptionsUIStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CameraOptionsUI, ICameraOptionsUIStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CameraOptionsUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraOptionsUI";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CapturedFrame(::windows::runtime::IInspectable);
impl CapturedFrame {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_Capture`, `Graphics_Imaging`*"]
    pub fn SoftwareBitmap(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Storage_Streams`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Capture`, `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ControlValues(&self) -> ::windows::runtime::Result<CapturedFrameControlValues> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrameControlValues>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`, `Graphics_Imaging`*"]
    pub fn BitmapProperties(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapPropertySet> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CapturedFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrame;{1dd2de1f-571b-44d8-8e80-a08a1578766e})");
}
unsafe impl ::windows::runtime::Interface for CapturedFrame {
    type Vtable = ICapturedFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(500358687, 22299, 17624, [142, 128, 160, 138, 21, 120, 118, 110]);
}
impl ::windows::runtime::RuntimeName for CapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrame";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<CapturedFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&CapturedFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::std::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream> for CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::std::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::std::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::std::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::std::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &CapturedFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::std::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CapturedFrame {}
unsafe impl ::std::marker::Sync for CapturedFrame {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CapturedFrameControlValues(::windows::runtime::IInspectable);
impl CapturedFrameControlValues {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Exposure(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ExposureCompensation(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn IsoSpeed(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Focus(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Devices`*"]
    pub fn SceneMode(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Flashed(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FlashPowerPercent(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn WhiteBalance(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ZoomFactor(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Devices`*"]
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn IsoDigitalGain(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn IsoAnalogGain(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Capture`, `Media_MediaProperties`*"]
    pub fn SensorFrameRate(&self) -> ::windows::runtime::Result<super::MediaProperties::MediaRatio> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn WhiteBalanceGain(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<WhiteBalanceGain>> {
        let this = &::windows::runtime::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<WhiteBalanceGain>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CapturedFrameControlValues {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrameControlValues;{90c65b7f-4e0d-4ca4-882d-7a144fed0a90})");
}
unsafe impl ::windows::runtime::Interface for CapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2428918655, 19981, 19620, [136, 45, 122, 20, 79, 237, 10, 144]);
}
impl ::windows::runtime::RuntimeName for CapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrameControlValues";
}
unsafe impl ::std::marker::Send for CapturedFrameControlValues {}
unsafe impl ::std::marker::Sync for CapturedFrameControlValues {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CapturedPhoto(::windows::runtime::IInspectable);
impl CapturedPhoto {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Frame(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CapturedPhoto {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedPhoto;{b0ce7e5a-cfcc-4d6c-8ad1-0869208aca16})");
}
unsafe impl ::windows::runtime::Interface for CapturedPhoto {
    type Vtable = ICapturedPhoto_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2966322778, 53196, 19820, [138, 209, 8, 105, 32, 138, 202, 22]);
}
impl ::windows::runtime::RuntimeName for CapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.CapturedPhoto";
}
unsafe impl ::std::marker::Send for CapturedPhoto {}
unsafe impl ::std::marker::Sync for CapturedPhoto {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: ForegroundActivationArgument = ForegroundActivationArgument(0i32);
    pub const MoreSettings: ForegroundActivationArgument = ForegroundActivationArgument(1i32);
}
impl ::std::convert::From<i32> for ForegroundActivationArgument {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ForegroundActivationArgument {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ForegroundActivationArgument {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.ForegroundActivationArgument;i4)");
}
impl ::windows::runtime::DefaultType for ForegroundActivationArgument {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: GameBarCommand = GameBarCommand(0i32);
    pub const RecordHistoricalBuffer: GameBarCommand = GameBarCommand(1i32);
    pub const ToggleStartStopRecord: GameBarCommand = GameBarCommand(2i32);
    pub const StartRecord: GameBarCommand = GameBarCommand(3i32);
    pub const StopRecord: GameBarCommand = GameBarCommand(4i32);
    pub const TakeScreenshot: GameBarCommand = GameBarCommand(5i32);
    pub const StartBroadcast: GameBarCommand = GameBarCommand(6i32);
    pub const StopBroadcast: GameBarCommand = GameBarCommand(7i32);
    pub const PauseBroadcast: GameBarCommand = GameBarCommand(8i32);
    pub const ResumeBroadcast: GameBarCommand = GameBarCommand(9i32);
    pub const ToggleStartStopBroadcast: GameBarCommand = GameBarCommand(10i32);
    pub const ToggleMicrophoneCapture: GameBarCommand = GameBarCommand(11i32);
    pub const ToggleCameraCapture: GameBarCommand = GameBarCommand(12i32);
    pub const ToggleRecordingIndicator: GameBarCommand = GameBarCommand(13i32);
}
impl ::std::convert::From<i32> for GameBarCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameBarCommand {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameBarCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommand;i4)");
}
impl ::windows::runtime::DefaultType for GameBarCommand {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: GameBarCommandOrigin = GameBarCommandOrigin(0i32);
    pub const Cortana: GameBarCommandOrigin = GameBarCommandOrigin(1i32);
    pub const AppCommand: GameBarCommandOrigin = GameBarCommandOrigin(2i32);
}
impl ::std::convert::From<i32> for GameBarCommandOrigin {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameBarCommandOrigin {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameBarCommandOrigin {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommandOrigin;i4)");
}
impl ::windows::runtime::DefaultType for GameBarCommandOrigin {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct GameBarContract(pub u8);
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameBarServices(::windows::runtime::IInspectable);
impl GameBarServices {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TargetCapturePolicy(&self) -> ::windows::runtime::Result<GameBarTargetCapturePolicy> {
        let this = self;
        unsafe {
            let mut result__: GameBarTargetCapturePolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarTargetCapturePolicy>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn EnableCapture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DisableCapture(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TargetInfo(&self) -> ::windows::runtime::Result<GameBarServicesTargetInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesTargetInfo>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SessionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AppBroadcastServices(&self) -> ::windows::runtime::Result<AppBroadcastServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastServices>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AppCaptureServices(&self) -> ::windows::runtime::Result<AppCaptureServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureServices>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CommandReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCommandReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameBarServices {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServices;{2dbead57-50a6-499e-8c6c-d330a7311796})");
}
unsafe impl ::windows::runtime::Interface for GameBarServices {
    type Vtable = IGameBarServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(767470935, 20646, 18846, [140, 108, 211, 48, 167, 49, 23, 150]);
}
impl ::windows::runtime::RuntimeName for GameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServices";
}
unsafe impl ::std::marker::Send for GameBarServices {}
unsafe impl ::std::marker::Sync for GameBarServices {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameBarServicesCommandEventArgs(::windows::runtime::IInspectable);
impl GameBarServicesCommandEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Command(&self) -> ::windows::runtime::Result<GameBarCommand> {
        let this = self;
        unsafe {
            let mut result__: GameBarCommand = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarCommand>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Origin(&self) -> ::windows::runtime::Result<GameBarCommandOrigin> {
        let this = self;
        unsafe {
            let mut result__: GameBarCommandOrigin = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarCommandOrigin>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameBarServicesCommandEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesCommandEventArgs;{a74226b2-f176-4fcf-8fbb-cf698b2eb8e0})");
}
unsafe impl ::windows::runtime::Interface for GameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2806130354, 61814, 20431, [143, 187, 207, 105, 139, 46, 184, 224]);
}
impl ::windows::runtime::RuntimeName for GameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesCommandEventArgs";
}
unsafe impl ::std::marker::Send for GameBarServicesCommandEventArgs {}
unsafe impl ::std::marker::Sync for GameBarServicesCommandEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: GameBarServicesDisplayMode = GameBarServicesDisplayMode(0i32);
    pub const FullScreenExclusive: GameBarServicesDisplayMode = GameBarServicesDisplayMode(1i32);
}
impl ::std::convert::From<i32> for GameBarServicesDisplayMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameBarServicesDisplayMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameBarServicesDisplayMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarServicesDisplayMode;i4)");
}
impl ::windows::runtime::DefaultType for GameBarServicesDisplayMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameBarServicesManager(::windows::runtime::IInspectable);
impl GameBarServicesManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn GameBarServicesCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveGameBarServicesCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<GameBarServicesManager> {
        Self::IGameBarServicesManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesManager>(result__)
        })
    }
    pub fn IGameBarServicesManagerStatics<R, F: FnOnce(&IGameBarServicesManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameBarServicesManager, IGameBarServicesManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameBarServicesManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManager;{3a4b9cfa-7f8b-4c60-9dbb-0bcd262dffc6})");
}
unsafe impl ::windows::runtime::Interface for GameBarServicesManager {
    type Vtable = IGameBarServicesManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(978033914, 32651, 19552, [157, 187, 11, 205, 38, 45, 255, 198]);
}
impl ::windows::runtime::RuntimeName for GameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManager";
}
unsafe impl ::std::marker::Send for GameBarServicesManager {}
unsafe impl ::std::marker::Sync for GameBarServicesManager {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(::windows::runtime::IInspectable);
impl GameBarServicesManagerGameBarServicesCreatedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GameBarServices(&self) -> ::windows::runtime::Result<GameBarServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServices>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs;{ededbd9c-143e-49a3-a5ea-0b1995c8d46e})");
}
unsafe impl ::windows::runtime::Interface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3991780764, 5182, 18851, [165, 234, 11, 25, 149, 200, 212, 110]);
}
impl ::windows::runtime::RuntimeName for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs";
}
unsafe impl ::std::marker::Send for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
unsafe impl ::std::marker::Sync for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct GameBarServicesTargetInfo(::windows::runtime::IInspectable);
impl GameBarServicesTargetInfo {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AppId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn TitleId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DisplayMode(&self) -> ::windows::runtime::Result<GameBarServicesDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: GameBarServicesDisplayMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesDisplayMode>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GameBarServicesTargetInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesTargetInfo;{b4202f92-1611-4e05-b6ef-dfd737ae33b0})");
}
unsafe impl ::windows::runtime::Interface for GameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3022008210, 5649, 19973, [182, 239, 223, 215, 55, 174, 51, 176]);
}
impl ::windows::runtime::RuntimeName for GameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesTargetInfo";
}
unsafe impl ::std::marker::Send for GameBarServicesTargetInfo {}
unsafe impl ::std::marker::Sync for GameBarServicesTargetInfo {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(0i32);
    pub const EnabledByUser: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(1i32);
    pub const NotEnabled: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(2i32);
    pub const ProhibitedBySystem: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(3i32);
    pub const ProhibitedByPublisher: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(4i32);
}
impl ::std::convert::From<i32> for GameBarTargetCapturePolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GameBarTargetCapturePolicy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GameBarTargetCapturePolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarTargetCapturePolicy;i4)");
}
impl ::windows::runtime::DefaultType for GameBarTargetCapturePolicy {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4034032267, 45714, 17553, [157, 65, 153, 128, 122, 85, 11, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvancedCapturedPhoto2 {
    type Vtable = IAdvancedCapturedPhoto2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(416247000, 53246, 17112, [129, 4, 1, 123, 179, 24, 244, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2214570746, 26215, 17628, [151, 60, 166, 188, 229, 150, 170, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134318378, 64148, 18169, [149, 252, 215, 21, 17, 205, 167, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastPlugInState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastPlugInState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastBackgroundService2 {
    type Vtable = IAppBroadcastBackgroundService2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4237085631, 21833, 19335, [149, 159, 35, 202, 64, 31, 212, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1584616053, 35016, 20170, [137, 186, 72, 37, 152, 93, 184, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastSignInState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastBackgroundServiceSignInInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2432968796, 25295, 19004, [167, 238, 174, 181, 7, 64, 70, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(836502204, 39178, 18692, [170, 150, 254, 54, 67, 129, 241, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastStreamState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastBackgroundServiceStreamInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3172900717, 38108, 20430, [149, 65, 169, 241, 41, 89, 99, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(506678480, 47234, 19336, [134, 146, 5, 153, 154, 206, 183, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2999658405, 28924, 19991, [128, 189, 107, 160, 253, 63, 243, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastCameraOverlayLocation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastCameraOverlaySize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3466936963, 61009, 19903, [148, 114, 121, 169, 237, 78, 33, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastManagerStatics {
    type Vtable = IAppBroadcastManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(911081867, 7758, 16671, [171, 62, 146, 149, 152, 68, 193, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2825573865, 37952, 18696, [157, 9, 101, 183, 227, 21, 215, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1376525926, 25875, 17780, [172, 84, 35, 183, 151, 41, 97, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3847281017, 10145, 18855, [187, 244, 215, 169, 233, 208, 118, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPlugInManagerStatics {
    type Vtable = IAppBroadcastPlugInManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4066663456, 23670, 19676, [147, 100, 130, 254, 158, 182, 83, 77]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1216467186, 43973, 20422, [132, 176, 137, 55, 11, 180, 114, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastPlugInState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(347475802, 28234, 19328, [161, 79, 103, 238, 119, 209, 83, 231]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastPreviewState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1515713246, 36330, 20102, [144, 173, 3, 252, 38, 185, 101, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastPreviewState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2451737936, 56127, 16552, [140, 212, 244, 227, 113, 221, 171, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(17809057, 38142, 17561, [184, 192, 141, 36, 66, 121, 251, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2347720979, 55940, 17561, [167, 171, 135, 17, 140, 180, 161, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3272335202, 39240, 17807, [173, 80, 170, 6, 236, 3, 218, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastServices(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastServices {
    type Vtable = IAppBroadcastServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2254484694, 38555, 20028, [172, 58, 139, 4, 46, 228, 238, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastCaptureTargetType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastCaptureTargetType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plugin: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desiredsize: super::super::Foundation::Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(45519524, 22809, 19102, [141, 94, 201, 187, 13, 211, 55, 122]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastSignInState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastSignInResult) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastState(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastState {
    type Vtable = IAppBroadcastState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3993503085, 32921, 19933, [146, 46, 197, 109, 172, 88, 171, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastStreamState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastPlugInState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppBroadcastSignInState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastSignInState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastTerminationReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020980424, 8634, 17727, [139, 183, 94, 147, 138, 46, 154, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3206653296, 27512, 16918, [159, 7, 90, 255, 82, 86, 241, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3006840057, 13156, 17504, [181, 241, 60, 194, 121, 106, 138, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1359521587, 53256, 19081, [147, 190, 88, 174, 217, 97, 55, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppBroadcastStreamState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(261607211, 51684, 20104, [129, 148, 216, 20, 203, 213, 133, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(194952910, 32306, 17197, [140, 162, 54, 191, 16, 185, 244, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3739986741, 60510, 19855, [177, 192, 93, 166, 232, 199, 86, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3873511461, 21505, 19166, [139, 210, 193, 78, 206, 230, 128, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCapture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCapture {
    type Vtable = IAppCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2538198099, 41626, 17901, [143, 41, 34, 208, 153, 66, 207, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(434692335, 9068, 16633, [179, 143, 155, 125, 214, 93, 28, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureAlternateShortcutKeys2 {
    type Vtable = IAppCaptureAlternateShortcutKeys2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3278278800, 56599, 18416, [149, 229, 206, 66, 40, 108, 243, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureAlternateShortcutKeys3 {
    type Vtable = IAppCaptureAlternateShortcutKeys3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2072069260, 16782, 18076, [164, 154, 69, 181, 151, 200, 38, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3254081083, 65441, 17609, [151, 95, 39, 251, 235, 85, 59, 53]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099561972, 18014, 17855, [144, 127, 22, 91, 63, 178, 55, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureManagerStatics {
    type Vtable = IAppCaptureManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2107522727, 25218, 18229, [141, 78, 170, 69, 249, 15, 103, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appcapturesettings: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3771615351, 39599, 18100, [173, 49, 106, 96, 180, 65, 199, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(843916446, 17852, 19509, [188, 53, 228, 105, 252, 122, 105, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3328188585, 5432, 18780, [155, 187, 43, 168, 112, 236, 88, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureRecordingState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(620529426, 58117, 18701, [180, 21, 107, 28, 144, 73, 115, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureRecordingState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureServices(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureServices {
    type Vtable = IAppCaptureServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1157546165, 13557, 20248, [174, 140, 185, 18, 58, 187, 252, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureSettings {
    type Vtable = IAppCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(342375046, 34823, 18643, [136, 58, 151, 14, 228, 83, 42, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureSettings2 {
    type Vtable = IAppCaptureSettings2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4239970023, 57963, 18287, [155, 26, 236, 52, 45, 42, 143, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureSettings3 {
    type Vtable = IAppCaptureSettings3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2838823678, 35010, 17110, [170, 170, 64, 254, 255, 215, 90, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureSettings4 {
    type Vtable = IAppCaptureSettings4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(130185036, 6785, 18479, [162, 68, 4, 157, 149, 242, 91, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureSettings5 {
    type Vtable = IAppCaptureSettings5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(411649314, 45288, 19360, [143, 19, 62, 170, 95, 164, 1, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureState(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureState {
    type Vtable = IAppCaptureState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930642290, 54507, 17614, [149, 56, 70, 95, 80, 106, 196, 234]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureStatics {
    type Vtable = IAppCaptureStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4179811692, 2686, 20084, [139, 32, 156, 31, 144, 45, 8, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppCaptureStatics2 {
    type Vtable = IAppCaptureStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3000533460, 33644, 19876, [175, 215, 250, 204, 4, 30, 28, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allowed: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraCaptureUI(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICameraCaptureUI {
    type Vtable = ICameraCaptureUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1213756736, 28563, 19380, [184, 243, 232, 158, 72, 148, 140, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: CameraCaptureUIMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3119890071, 13426, 18088, [138, 158, 4, 206, 66, 204, 201, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CameraCaptureUIPhotoFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1693003039, 41613, 16986, [184, 79, 229, 104, 51, 95, 242, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CameraCaptureUIVideoFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CameraCaptureUIVideoFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CameraCaptureUIMaxVideoResolution) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICameraOptionsUIStatics {
    type Vtable = ICameraOptionsUIStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(990731828, 14598, 19325, [148, 108, 123, 222, 132, 68, 153, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediacapture: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICapturedFrame {
    type Vtable = ICapturedFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(500358687, 22299, 17624, [142, 128, 160, 138, 21, 120, 118, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrame2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICapturedFrame2 {
    type Vtable = ICapturedFrame2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1413457617, 48504, 18534, [173, 218, 36, 49, 75, 198, 93, 234]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2428918655, 19981, 19620, [136, 45, 122, 20, 79, 237, 10, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICapturedFrameControlValues2 {
    type Vtable = ICapturedFrameControlValues2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1342909320, 1746, 19111, [167, 219, 211, 122, 247, 51, 33, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICapturedFrameWithSoftwareBitmap {
    type Vtable = ICapturedFrameWithSoftwareBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3046017902, 34051, 18869, [158, 134, 137, 125, 38, 163, 255, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedPhoto(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICapturedPhoto {
    type Vtable = ICapturedPhoto_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2966322778, 53196, 19820, [138, 209, 8, 105, 32, 138, 202, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedPhoto_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServices(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarServices {
    type Vtable = IGameBarServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(767470935, 20646, 18846, [140, 108, 211, 48, 167, 49, 23, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameBarTargetCapturePolicy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2806130354, 61814, 20431, [143, 187, 207, 105, 139, 46, 184, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameBarCommand) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameBarCommandOrigin) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarServicesManager {
    type Vtable = IGameBarServicesManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(978033914, 32651, 19552, [157, 187, 11, 205, 38, 45, 255, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3991780764, 5182, 18851, [165, 234, 11, 25, 149, 200, 212, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarServicesManagerStatics {
    type Vtable = IGameBarServicesManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(885110294, 65317, 18322, [152, 242, 211, 117, 63, 21, 172, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3022008210, 5649, 19973, [182, 239, 223, 215, 55, 174, 51, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GameBarServicesDisplayMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagMediaRecording(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1103674103, 65343, 18928, [164, 119, 241, 149, 227, 206, 81, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLagMediaRecording2 {
    type Vtable = ILowLagMediaRecording2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1667876696, 22084, 16866, [151, 175, 142, 245, 106, 37, 226, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLagMediaRecording3 {
    type Vtable = ILowLagMediaRecording3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1546890002, 18679, 18394, [180, 30, 144, 136, 10, 95, 224, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2742178231, 27460, 18237, [143, 36, 247, 3, 214, 192, 236, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2093172411, 47529, 19601, [143, 250, 40, 126, 156, 102, 134, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture {
    type Vtable = IMediaCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3323657140, 64272, 18996, [172, 24, 202, 128, 217, 200, 231, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediacaptureinitializationsettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, custommediasink: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, customsinkactivationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, customsinksettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediastreamtype: MediaStreamType, effectactivationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, effectsettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediastreamtype: MediaStreamType, propertyid: ::windows::runtime::GUID, propertyvalue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediastreamtype: MediaStreamType, propertyid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, erroreventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recordlimitationexceededeventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: VideoRotation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VideoRotation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: VideoRotation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VideoRotation) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture2 {
    type Vtable = IMediaCapture2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2630255200, 32161, 16451, [182, 82, 33, 184, 135, 141, 175, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, custommediasink: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, customsinkactivationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, customsinksettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mediastreamtype: MediaStreamType, mediaencodingproperties: ::windows::runtime::RawPtr, encoderproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture3 {
    type Vtable = IMediaCapture3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3558043440, 5476, 18030, [188, 10, 175, 148, 224, 42, 176, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture4 {
    type Vtable = IMediaCapture4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134025686, 64264, 18759, [174, 162, 206, 20, 239, 240, 206, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, definition: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, definition: ::windows::runtime::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Devices::CameraStreamState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaCaptureThermalStatus) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture5 {
    type Vtable = IMediaCapture5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3665329186, 15003, 18208, [167, 30, 151, 144, 10, 49, 110, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effect: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputsource: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputsource: ::windows::runtime::RawPtr, outputsubtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputsource: ::windows::runtime::RawPtr, outputsubtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture6 {
    type Vtable = IMediaCapture6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(579422397, 19232, 19377, [159, 214, 165, 131, 33, 42, 16, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputsources: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture7(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapture7 {
    type Vtable = IMediaCapture7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2439639298, 34952, 21530, [149, 188, 36, 228, 212, 98, 84, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capturemode: StreamingCaptureMode, displayregion: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2637140493, 42376, 17350, [137, 214, 90, 211, 34, 175, 0, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2164122612, 21700, 17088, [141, 25, 206, 161, 168, 124, 161, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2179054719, 8823, 18750, [171, 238, 211, 244, 79, 249, 140, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541927024, 60005, 18688, [147, 86, 140, 168, 135, 114, 104, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: StreamingCaptureMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StreamingCaptureMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PhotoCaptureSource) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhotoCaptureSource) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings2 {
    type Vtable = IMediaCaptureInitializationSettings2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1078855206, 51676, 17385, [174, 228, 230, 191, 27, 87, 180, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::AudioProcessing) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::AudioProcessing) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings3 {
    type Vtable = IMediaCaptureInitializationSettings3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1096831389, 48712, 18224, [129, 4, 12, 246, 233, 233, 121, 72]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings4 {
    type Vtable = IMediaCaptureInitializationSettings4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4110591287, 19639, 19752, [149, 237, 79, 159, 1, 46, 5, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings5 {
    type Vtable = IMediaCaptureInitializationSettings5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3584222136, 9766, 20116, [183, 179, 83, 8, 160, 246, 75, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Capture_Frames")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))] usize,
    #[cfg(feature = "Media_Capture_Frames")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaCaptureSharingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaCaptureSharingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaCaptureMemoryPreference) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaCaptureMemoryPreference) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings6 {
    type Vtable = IMediaCaptureInitializationSettings6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3001183047, 15793, 19763, [171, 99, 15, 250, 9, 5, 101, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureInitializationSettings7 {
    type Vtable = IMediaCaptureInitializationSettings7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1096051047, 62858, 23938, [158, 244, 237, 87, 47, 181, 227, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2932112547, 17527, 19204, [160, 111, 44, 28, 81, 130, 254, 157]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2106156390, 1214, 23433, [179, 14, 189, 52, 169, 241, 45, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(495168254, 27973, 17527, [141, 196, 172, 91, 192, 28, 64, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StreamingCaptureMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PhotoCaptureSource) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut VideoDeviceCharacteristic) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureSettings2 {
    type Vtable = IMediaCaptureSettings2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1872657659, 64159, 19219, [156, 190, 90, 185, 79, 31, 52, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::AudioProcessing) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureSettings3 {
    type Vtable = IMediaCaptureSettings3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(809265090, 32856, 19227, [184, 119, 140, 46, 243, 82, 132, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureStatics {
    type Vtable = IMediaCaptureStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2901377535, 39405, 17989, [150, 94, 25, 37, 207, 198, 56, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, videodeviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, videodeviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, videodeviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, videodeviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, name: KnownVideoProfile, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4191906346, 41106, 19153, [151, 212, 242, 1, 249, 208, 130, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureVideoPreview {
    type Vtable = IMediaCaptureVideoPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(661811315, 21662, 17535, [162, 10, 79, 3, 196, 121, 216, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, custommediasink: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, customsinkactivationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, customsinksettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(564163519, 41966, 20175, [158, 246, 80, 176, 188, 78, 19, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureVideoProfile2 {
    type Vtable = IMediaCaptureVideoProfile2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2547894623, 38094, 18063, [147, 22, 252, 91, 194, 99, 143, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2148708335, 46737, 18943, [131, 242, 193, 231, 110, 170, 234, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCaptureVideoProfileMediaDescription2 {
    type Vtable = IMediaCaptureVideoProfileMediaDescription2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3332828947, 12845, 16698, [184, 90, 104, 168, 142, 2, 244, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1192200371, 7789, 16465, [156, 139, 241, 216, 90, 240, 71, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(926677953, 38990, 20464, [191, 133, 28, 0, 170, 188, 90, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2873570930, 49802, 18471, [143, 141, 54, 54, 211, 190, 181, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScreenCapture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScreenCapture {
    type Vtable = IScreenCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2300026615, 52498, 19982, [166, 212, 91, 61, 233, 139, 46, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScreenCaptureStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScreenCaptureStatics {
    type Vtable = IScreenCaptureStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3365454768, 51365, 4578, [139, 139, 8, 0, 32, 12, 154, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCaptureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(785283934, 54427, 17300, [188, 50, 249, 125, 108, 237, 236, 28]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3631680111, 17296, 19294, [173, 62, 15, 138, 240, 150, 52, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: KnownVideoProfile = KnownVideoProfile(0i32);
    pub const HighQualityPhoto: KnownVideoProfile = KnownVideoProfile(1i32);
    pub const BalancedVideoAndPhoto: KnownVideoProfile = KnownVideoProfile(2i32);
    pub const VideoConferencing: KnownVideoProfile = KnownVideoProfile(3i32);
    pub const PhotoSequence: KnownVideoProfile = KnownVideoProfile(4i32);
    pub const HighFrameRate: KnownVideoProfile = KnownVideoProfile(5i32);
    pub const VariablePhotoSequence: KnownVideoProfile = KnownVideoProfile(6i32);
    pub const HdrWithWcgVideo: KnownVideoProfile = KnownVideoProfile(7i32);
    pub const HdrWithWcgPhoto: KnownVideoProfile = KnownVideoProfile(8i32);
    pub const VideoHdr8: KnownVideoProfile = KnownVideoProfile(9i32);
    pub const CompressedCamera: KnownVideoProfile = KnownVideoProfile(10i32);
}
impl ::std::convert::From<i32> for KnownVideoProfile {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KnownVideoProfile {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KnownVideoProfile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.KnownVideoProfile;i4)");
}
impl ::windows::runtime::DefaultType for KnownVideoProfile {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LowLagMediaRecording(::windows::runtime::IInspectable);
impl LowLagMediaRecording {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FinishAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Devices`*"]
    pub fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ResumeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Devices`*"]
    pub fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows::runtime::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StopWithResultAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows::runtime::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LowLagMediaRecording {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagMediaRecording;{41c8baf7-ff3f-49f0-a477-f195e3ce5108})");
}
unsafe impl ::windows::runtime::Interface for LowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1103674103, 65343, 18928, [164, 119, 241, 149, 227, 206, 81, 8]);
}
impl ::windows::runtime::RuntimeName for LowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.LowLagMediaRecording";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LowLagPhotoCapture(::windows::runtime::IInspectable);
impl LowLagPhotoCapture {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CapturedPhoto>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FinishAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LowLagPhotoCapture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoCapture;{a37251b7-6b44-473d-8f24-f703d6c0ec44})");
}
unsafe impl ::windows::runtime::Interface for LowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2742178231, 27460, 18237, [143, 36, 247, 3, 214, 192, 236, 68]);
}
impl ::windows::runtime::RuntimeName for LowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoCapture";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LowLagPhotoSequenceCapture(::windows::runtime::IInspectable);
impl LowLagPhotoSequenceCapture {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FinishAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn PhotoCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemovePhotoCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LowLagPhotoSequenceCapture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoSequenceCapture;{7cc346bb-b9a9-4c91-8ffa-287e9c668669})");
}
unsafe impl ::windows::runtime::Interface for LowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2093172411, 47529, 19601, [143, 250, 40, 126, 156, 102, 134, 105]);
}
impl ::windows::runtime::RuntimeName for LowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoSequenceCapture";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCapture(::windows::runtime::IInspectable);
impl MediaCapture {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaCapture, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn InitializeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn InitializeWithSettingsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCaptureInitializationSettings>>(&self, mediacaptureinitializationsettings: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mediacaptureinitializationsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`, `Storage`*"]
    pub fn StartRecordToStorageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`, `Storage_Streams`*"]
    pub fn StartRecordToStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`*"]
    pub fn StartRecordToCustomSinkAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Foundation_Collections`, `Media_MediaProperties`*"]
    pub fn StartRecordToCustomSinkIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StopRecordAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`, `Storage`*"]
    pub fn CapturePhotoToStorageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, r#type: Param0, file: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), r#type.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`, `Storage_Streams`*"]
    pub fn CapturePhotoToStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, r#type: Param0, stream: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), r#type.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Foundation_Collections`*"]
    pub fn AddEffectAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, mediastreamtype: MediaStreamType, effectactivationid: Param1, effectsettings: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), mediastreamtype, effectactivationid.into_param().abi(), effectsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetEncoderProperty<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1, propertyvalue: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), mediastreamtype, propertyid.into_param().abi(), propertyvalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetEncoderProperty<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), mediastreamtype, propertyid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Failed<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCaptureFailedEventHandler>>(&self, erroreventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), erroreventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RecordLimitationExceeded<'a, Param0: ::windows::runtime::IntoParam<'a, RecordLimitationExceededEventHandler>>(&self, recordlimitationexceededeventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), recordlimitationexceededeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveRecordLimitationExceeded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MediaCaptureSettings(&self) -> ::windows::runtime::Result<MediaCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureSettings>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture`, `Media_Devices`*"]
    pub fn AudioDeviceController(&self) -> ::windows::runtime::Result<super::Devices::AudioDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::AudioDeviceController>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture`, `Media_Devices`*"]
    pub fn VideoDeviceController(&self) -> ::windows::runtime::Result<super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::VideoDeviceController>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetPreviewMirroring(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetPreviewMirroring(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetPreviewRotation(&self) -> ::windows::runtime::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__: VideoRotation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoRotation>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetRecordRotation(&self, value: VideoRotation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetRecordRotation(&self) -> ::windows::runtime::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__: VideoRotation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoRotation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`, `Storage`*"]
    pub fn PrepareLowLagRecordToStorageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`, `Storage_Streams`*"]
    pub fn PrepareLowLagRecordToStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`*"]
    pub fn PrepareLowLagRecordToCustomSinkAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Foundation_Collections`, `Media_MediaProperties`*"]
    pub fn PrepareLowLagRecordToCustomSinkIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`*"]
    pub fn PrepareLowLagPhotoCaptureAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`*"]
    pub fn PrepareLowLagPhotoSequenceCaptureAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Foundation_Collections`, `Media_MediaProperties`*"]
    pub fn SetEncodingPropertiesAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaPropertySet>>(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: Param1, encoderproperties: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), mediastreamtype, mediaencodingproperties.into_param().abi(), encoderproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Capture_Core`, `Media_MediaProperties`*"]
    pub fn PrepareVariablePhotoSequenceCaptureAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn FocusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveFocusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn PhotoConfirmationCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemovePhotoConfirmationCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Effects`*"]
    pub fn AddAudioEffectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), definition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Effects`*"]
    pub fn AddVideoEffectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Effects::IVideoEffectDefinition>>(&self, definition: Param0, mediastreamtype: MediaStreamType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), definition.into_param().abi(), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Devices`*"]
    pub fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ResumeRecordAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CameraStreamStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCameraStreamStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture`, `Media_Devices`*"]
    pub fn CameraStreamState(&self) -> ::windows::runtime::Result<super::Devices::CameraStreamState> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::Devices::CameraStreamState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::CameraStreamState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn GetPreviewFrameAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn GetPreviewFrameCopyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::VideoFrame>>(&self, destination: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn ThermalStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveThermalStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ThermalStatus(&self) -> ::windows::runtime::Result<MediaCaptureThermalStatus> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: MediaCaptureThermalStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureThermalStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`*"]
    pub fn PrepareAdvancedPhotoCaptureAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StartPreviewAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_MediaProperties`*"]
    pub fn StartPreviewToCustomSinkAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Foundation_Collections`, `Media_MediaProperties`*"]
    pub fn StartPreviewToCustomSinkIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StopPreviewAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsVideoProfileSupported<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(videodeviceid: Param0) -> ::windows::runtime::Result<bool> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn FindAllVideoProfiles<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(videodeviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn FindConcurrentProfiles<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(videodeviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn FindKnownVideoProfiles<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(videodeviceid: Param0, name: KnownVideoProfile) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), videodeviceid.into_param().abi(), name, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveEffectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::IMediaExtension>>(&self, effect: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Devices`*"]
    pub fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn StopRecordWithResultAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`, `Media_Capture_Frames`*"]
    pub fn FrameSources(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, Frames::MediaFrameSource>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, Frames::MediaFrameSource>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Capture_Frames`*"]
    pub fn CreateFrameReaderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Frames::MediaFrameSource>>(&self, inputsource: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), inputsource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Media_Capture_Frames`*"]
    pub fn CreateFrameReaderWithSubtypeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, inputsource: Param0, outputsubtype: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Graphics_Imaging`, `Media_Capture_Frames`*"]
    pub fn CreateFrameReaderWithSubtypeAndSizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, inputsource: Param0, outputsubtype: Param1, outputsize: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), outputsize.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveCaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture6>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation`, `Foundation_Collections`, `Media_Capture_Frames`*"]
    pub fn CreateMultiSourceFrameReaderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>>(&self, inputsources: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), inputsources.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>(result__)
        }
    }
    #[cfg(feature = "UI_WindowManagement")]
    #[doc = "*Required features: `Media_Capture`, `UI_WindowManagement`*"]
    pub fn CreateRelativePanelWatcher<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::UI::WindowManagement::DisplayRegion>>(&self, capturemode: StreamingCaptureMode, displayregion: Param1) -> ::windows::runtime::Result<MediaCaptureRelativePanelWatcher> {
        let this = &::windows::runtime::Interface::cast::<IMediaCapture7>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), capturemode, displayregion.into_param().abi(), &mut result__).from_abi::<MediaCaptureRelativePanelWatcher>(result__)
        }
    }
    pub fn IMediaCaptureStatics<R, F: FnOnce(&IMediaCaptureStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaCapture, IMediaCaptureStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCapture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapture;{c61afbb4-fb10-4a34-ac18-ca80d9c8e7ee})");
}
unsafe impl ::windows::runtime::Interface for MediaCapture {
    type Vtable = IMediaCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3323657140, 64272, 18996, [172, 24, 202, 128, 217, 200, 231, 238]);
}
impl ::windows::runtime::RuntimeName for MediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapture";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MediaCapture> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaCapture) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MediaCapture> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaCapture) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MediaCapture {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCapture {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: MediaCaptureDeviceExclusiveControlStatus = MediaCaptureDeviceExclusiveControlStatus(0i32);
    pub const SharedReadOnlyAvailable: MediaCaptureDeviceExclusiveControlStatus = MediaCaptureDeviceExclusiveControlStatus(1i32);
}
impl ::std::convert::From<i32> for MediaCaptureDeviceExclusiveControlStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaCaptureDeviceExclusiveControlStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureDeviceExclusiveControlStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatus;i4)");
}
impl ::windows::runtime::DefaultType for MediaCaptureDeviceExclusiveControlStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows::runtime::IInspectable);
impl MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<MediaCaptureDeviceExclusiveControlStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaCaptureDeviceExclusiveControlStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureDeviceExclusiveControlStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs;{9d2f920d-a588-43c6-89d6-5ad322af006a})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2637140493, 42376, 17350, [137, 214, 90, 211, 34, 175, 0, 106]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
unsafe impl ::std::marker::Send for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
unsafe impl ::std::marker::Sync for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureFailedEventArgs(::windows::runtime::IInspectable);
impl MediaCaptureFailedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Code(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFailedEventArgs;{80fde3f4-54c4-42c0-8d19-cea1a87ca18b})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2164122612, 21700, 17088, [141, 25, 206, 161, 168, 124, 161, 139]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFailedEventArgs";
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaCaptureFailedEventHandler(::windows::runtime::IUnknown);
impl MediaCaptureFailedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<MediaCapture>, &::std::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = MediaCaptureFailedEventHandler_box::<F> {
            vtable: &MediaCaptureFailedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCapture>, Param1: ::windows::runtime::IntoParam<'a, MediaCaptureFailedEventArgs>>(&self, sender: Param0, erroreventargs: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), erroreventargs.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureFailedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({2014effb-5cd8-4f08-a314-0d360da59f14})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureFailedEventHandler {
    type Vtable = MediaCaptureFailedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(538243067, 23768, 20232, [163, 20, 13, 54, 13, 165, 159, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct MediaCaptureFailedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, erroreventargs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct MediaCaptureFailedEventHandler_box<F: FnMut(&::std::option::Option<MediaCapture>, &::std::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const MediaCaptureFailedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<MediaCapture>, &::std::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static> MediaCaptureFailedEventHandler_box<F> {
    const VTABLE: MediaCaptureFailedEventHandler_abi = MediaCaptureFailedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MediaCaptureFailedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, erroreventargs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <MediaCapture as ::windows::runtime::Abi>::Abi as *const <MediaCapture as ::windows::runtime::DefaultType>::DefaultType),
            &*(&erroreventargs as *const <MediaCaptureFailedEventArgs as ::windows::runtime::Abi>::Abi as *const <MediaCaptureFailedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureFocusChangedEventArgs(::windows::runtime::IInspectable);
impl MediaCaptureFocusChangedEventArgs {
    #[cfg(feature = "Media_Devices")]
    #[doc = "*Required features: `Media_Capture`, `Media_Devices`*"]
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::Devices::MediaCaptureFocusState> {
        let this = self;
        unsafe {
            let mut result__: super::Devices::MediaCaptureFocusState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::MediaCaptureFocusState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureFocusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFocusChangedEventArgs;{81e1bc7f-2277-493e-abee-d3f44ff98c04})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2179054719, 8823, 18750, [171, 238, 211, 244, 79, 249, 140, 4]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFocusChangedEventArgs";
}
unsafe impl ::std::marker::Send for MediaCaptureFocusChangedEventArgs {}
unsafe impl ::std::marker::Sync for MediaCaptureFocusChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureInitializationSettings(::windows::runtime::IInspectable);
impl MediaCaptureInitializationSettings {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaCaptureInitializationSettings, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAudioDeviceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoDeviceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StreamingCaptureMode(&self) -> ::windows::runtime::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__: StreamingCaptureMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PhotoCaptureSource(&self) -> ::windows::runtime::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoCaptureSource = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMediaCategory(&self, value: MediaCategory) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MediaCategory(&self) -> ::windows::runtime::Result<MediaCategory> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__: MediaCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCategory>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioProcessing(&self) -> ::windows::runtime::Result<super::AudioProcessing> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__: super::AudioProcessing = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Capture`, `Media_Core`*"]
    pub fn SetAudioSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Capture`, `Media_Core`*"]
    pub fn AudioSource(&self) -> ::windows::runtime::Result<super::Core::IMediaSource> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Capture`, `Media_Core`*"]
    pub fn SetVideoSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Capture`, `Media_Core`*"]
    pub fn VideoSource(&self) -> ::windows::runtime::Result<super::Core::IMediaSource> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoProfile(&self) -> ::windows::runtime::Result<MediaCaptureVideoProfile> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfile>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetVideoProfile<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCaptureVideoProfile>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PreviewMediaDescription(&self) -> ::windows::runtime::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetPreviewMediaDescription<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn RecordMediaDescription(&self) -> ::windows::runtime::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetRecordMediaDescription<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PhotoMediaDescription(&self) -> ::windows::runtime::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetPhotoMediaDescription<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    #[doc = "*Required features: `Media_Capture`, `Media_Capture_Frames`*"]
    pub fn SourceGroup(&self) -> ::windows::runtime::Result<Frames::MediaFrameSourceGroup> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Frames::MediaFrameSourceGroup>(result__)
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    #[doc = "*Required features: `Media_Capture`, `Media_Capture_Frames`*"]
    pub fn SetSourceGroup<'a, Param0: ::windows::runtime::IntoParam<'a, Frames::MediaFrameSourceGroup>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SharingMode(&self) -> ::windows::runtime::Result<MediaCaptureSharingMode> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: MediaCaptureSharingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureSharingMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MemoryPreference(&self) -> ::windows::runtime::Result<MediaCaptureMemoryPreference> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: MediaCaptureMemoryPreference = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureMemoryPreference>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AlwaysPlaySystemShutterSound(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Media_Capture`, `Security_Credentials`*"]
    pub fn DeviceUriPasswordCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Media_Capture`, `Security_Credentials`*"]
    pub fn SetDeviceUriPasswordCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn DeviceUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SetDeviceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureInitializationSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureInitializationSettings;{9782ba70-ea65-4900-9356-8ca887726884})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541927024, 60005, 18688, [147, 86, 140, 168, 135, 114, 104, 132]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureInitializationSettings";
}
unsafe impl ::std::marker::Send for MediaCaptureInitializationSettings {}
unsafe impl ::std::marker::Sync for MediaCaptureInitializationSettings {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: MediaCaptureMemoryPreference = MediaCaptureMemoryPreference(0i32);
    pub const Cpu: MediaCaptureMemoryPreference = MediaCaptureMemoryPreference(1i32);
}
impl ::std::convert::From<i32> for MediaCaptureMemoryPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaCaptureMemoryPreference {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureMemoryPreference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureMemoryPreference;i4)");
}
impl ::windows::runtime::DefaultType for MediaCaptureMemoryPreference {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCapturePauseResult(::windows::runtime::IInspectable);
impl MediaCapturePauseResult {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn LastFrame(&self) -> ::windows::runtime::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RecordDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCapturePauseResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapturePauseResult;{aec47ca3-4477-4b04-a06f-2c1c5182fe9d})");
}
unsafe impl ::windows::runtime::Interface for MediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2932112547, 17527, 19204, [160, 111, 44, 28, 81, 130, 254, 157]);
}
impl ::windows::runtime::RuntimeName for MediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapturePauseResult";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MediaCapturePauseResult> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaCapturePauseResult) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MediaCapturePauseResult> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaCapturePauseResult) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCapturePauseResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureRelativePanelWatcher(::windows::runtime::IInspectable);
impl MediaCaptureRelativePanelWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Media_Capture`, `Devices_Enumeration`*"]
    pub fn RelativePanel(&self) -> ::windows::runtime::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Enumeration::Panel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::Panel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Changed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureRelativePanelWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureRelativePanelWatcher;{7d896566-04be-5b89-b30e-bd34a9f12db0})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2106156390, 1214, 23433, [179, 14, 189, 52, 169, 241, 45, 176]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureRelativePanelWatcher";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MediaCaptureRelativePanelWatcher> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaCaptureRelativePanelWatcher) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MediaCaptureRelativePanelWatcher> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaCaptureRelativePanelWatcher) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MediaCaptureRelativePanelWatcher {}
unsafe impl ::std::marker::Sync for MediaCaptureRelativePanelWatcher {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureSettings(::windows::runtime::IInspectable);
impl MediaCaptureSettings {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn StreamingCaptureMode(&self) -> ::windows::runtime::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__: StreamingCaptureMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn PhotoCaptureSource(&self) -> ::windows::runtime::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoCaptureSource = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoDeviceCharacteristic(&self) -> ::windows::runtime::Result<VideoDeviceCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: VideoDeviceCharacteristic = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoDeviceCharacteristic>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn CameraSoundRequiredForRegion(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn PitchOffsetDegrees(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Vertical35mmEquivalentFocalLength(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn MediaCategory(&self) -> ::windows::runtime::Result<MediaCategory> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: MediaCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaCategory>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn AudioProcessing(&self) -> ::windows::runtime::Result<super::AudioProcessing> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: super::AudioProcessing = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Media_Capture`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Direct3D11Device(&self) -> ::windows::runtime::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureSettings;{1d83aafe-6d45-4477-8dc4-ac5bc01c4091})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(495168254, 27973, 17527, [141, 196, 172, 91, 192, 28, 64, 145]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureSettings";
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: MediaCaptureSharingMode = MediaCaptureSharingMode(0i32);
    pub const SharedReadOnly: MediaCaptureSharingMode = MediaCaptureSharingMode(1i32);
}
impl ::std::convert::From<i32> for MediaCaptureSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaCaptureSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureSharingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureSharingMode;i4)");
}
impl ::windows::runtime::DefaultType for MediaCaptureSharingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureStopResult(::windows::runtime::IInspectable);
impl MediaCaptureStopResult {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn LastFrame(&self) -> ::windows::runtime::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RecordDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureStopResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureStopResult;{f9db6a2a-a092-4ad1-97d4-f201f9d082db})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4191906346, 41106, 19153, [151, 212, 242, 1, 249, 208, 130, 219]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureStopResult";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MediaCaptureStopResult> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaCaptureStopResult) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MediaCaptureStopResult> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaCaptureStopResult) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCaptureStopResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: MediaCaptureThermalStatus = MediaCaptureThermalStatus(0i32);
    pub const Overheated: MediaCaptureThermalStatus = MediaCaptureThermalStatus(1i32);
}
impl ::std::convert::From<i32> for MediaCaptureThermalStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaCaptureThermalStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureThermalStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureThermalStatus;i4)");
}
impl ::windows::runtime::DefaultType for MediaCaptureThermalStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureVideoProfile(::windows::runtime::IInspectable);
impl MediaCaptureVideoProfile {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn VideoDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn SupportedPreviewMediaDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn SupportedRecordMediaDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn SupportedPhotoMediaDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn GetConcurrency(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`, `Media_Capture_Frames`*"]
    pub fn FrameSourceInfos(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureVideoProfile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfile;{21a073bf-a3ee-4ecf-9ef6-50b0bc4e1305})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(564163519, 41966, 20175, [158, 246, 80, 176, 188, 78, 19, 5]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfile";
}
unsafe impl ::std::marker::Send for MediaCaptureVideoProfile {}
unsafe impl ::std::marker::Sync for MediaCaptureVideoProfile {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MediaCaptureVideoProfileMediaDescription(::windows::runtime::IInspectable);
impl MediaCaptureVideoProfileMediaDescription {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn FrameRate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsVariablePhotoSequenceSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsHdrVideoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Capture`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription;{8012afef-b691-49ff-83f2-c1e76eaaea1b})");
}
unsafe impl ::windows::runtime::Interface for MediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2148708335, 46737, 18943, [131, 242, 193, 231, 110, 170, 234, 27]);
}
impl ::windows::runtime::RuntimeName for MediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription";
}
unsafe impl ::std::marker::Send for MediaCaptureVideoProfileMediaDescription {}
unsafe impl ::std::marker::Sync for MediaCaptureVideoProfileMediaDescription {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: MediaCategory = MediaCategory(0i32);
    pub const Communications: MediaCategory = MediaCategory(1i32);
    pub const Media: MediaCategory = MediaCategory(2i32);
    pub const GameChat: MediaCategory = MediaCategory(3i32);
    pub const Speech: MediaCategory = MediaCategory(4i32);
    pub const FarFieldSpeech: MediaCategory = MediaCategory(5i32);
    pub const UniformSpeech: MediaCategory = MediaCategory(6i32);
    pub const VoiceTyping: MediaCategory = MediaCategory(7i32);
}
impl ::std::convert::From<i32> for MediaCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaCategory {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCategory;i4)");
}
impl ::windows::runtime::DefaultType for MediaCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: MediaStreamType = MediaStreamType(0i32);
    pub const VideoRecord: MediaStreamType = MediaStreamType(1i32);
    pub const Audio: MediaStreamType = MediaStreamType(2i32);
    pub const Photo: MediaStreamType = MediaStreamType(3i32);
    pub const Metadata: MediaStreamType = MediaStreamType(4i32);
}
impl ::std::convert::From<i32> for MediaStreamType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaStreamType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaStreamType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaStreamType;i4)");
}
impl ::windows::runtime::DefaultType for MediaStreamType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct OptionalReferencePhotoCapturedEventArgs(::windows::runtime::IInspectable);
impl OptionalReferencePhotoCapturedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Frame(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Context(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs;{470f88b3-1e6d-4051-9c8b-f1d85af047b7})");
}
unsafe impl ::windows::runtime::Interface for OptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1192200371, 7789, 16465, [156, 139, 241, 216, 90, 240, 71, 183]);
}
impl ::windows::runtime::RuntimeName for OptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs";
}
unsafe impl ::std::marker::Send for OptionalReferencePhotoCapturedEventArgs {}
unsafe impl ::std::marker::Sync for OptionalReferencePhotoCapturedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: PhotoCaptureSource = PhotoCaptureSource(0i32);
    pub const VideoPreview: PhotoCaptureSource = PhotoCaptureSource(1i32);
    pub const Photo: PhotoCaptureSource = PhotoCaptureSource(2i32);
}
impl ::std::convert::From<i32> for PhotoCaptureSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PhotoCaptureSource {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PhotoCaptureSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PhotoCaptureSource;i4)");
}
impl ::windows::runtime::DefaultType for PhotoCaptureSource {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PhotoCapturedEventArgs(::windows::runtime::IInspectable);
impl PhotoCapturedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Frame(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureTimeOffset(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhotoCapturedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoCapturedEventArgs;{373bfbc1-984e-4ff0-bf85-1c00aabc5a45})");
}
unsafe impl ::windows::runtime::Interface for PhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(926677953, 38990, 20464, [191, 133, 28, 0, 170, 188, 90, 69]);
}
impl ::windows::runtime::RuntimeName for PhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoCapturedEventArgs";
}
unsafe impl ::std::marker::Send for PhotoCapturedEventArgs {}
unsafe impl ::std::marker::Sync for PhotoCapturedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PhotoConfirmationCapturedEventArgs(::windows::runtime::IInspectable);
impl PhotoConfirmationCapturedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Frame(&self) -> ::windows::runtime::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn CaptureTimeOffset(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhotoConfirmationCapturedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoConfirmationCapturedEventArgs;{ab473672-c28a-4827-8f8d-3636d3beb51e})");
}
unsafe impl ::windows::runtime::Interface for PhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2873570930, 49802, 18471, [143, 141, 54, 54, 211, 190, 181, 30]);
}
impl ::windows::runtime::RuntimeName for PhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoConfirmationCapturedEventArgs";
}
unsafe impl ::std::marker::Send for PhotoConfirmationCapturedEventArgs {}
unsafe impl ::std::marker::Sync for PhotoConfirmationCapturedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: PowerlineFrequency = PowerlineFrequency(0i32);
    pub const FiftyHertz: PowerlineFrequency = PowerlineFrequency(1i32);
    pub const SixtyHertz: PowerlineFrequency = PowerlineFrequency(2i32);
    pub const Auto: PowerlineFrequency = PowerlineFrequency(3i32);
}
impl ::std::convert::From<i32> for PowerlineFrequency {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PowerlineFrequency {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PowerlineFrequency {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PowerlineFrequency;i4)");
}
impl ::windows::runtime::DefaultType for PowerlineFrequency {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RecordLimitationExceededEventHandler(::windows::runtime::IUnknown);
impl RecordLimitationExceededEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<MediaCapture>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = RecordLimitationExceededEventHandler_box::<F> {
            vtable: &RecordLimitationExceededEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, MediaCapture>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RecordLimitationExceededEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({3fae8f2e-4fe1-4ffd-aaba-e1f1337d4e53})");
}
unsafe impl ::windows::runtime::Interface for RecordLimitationExceededEventHandler {
    type Vtable = RecordLimitationExceededEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1068404526, 20449, 20477, [170, 186, 225, 241, 51, 125, 78, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct RecordLimitationExceededEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct RecordLimitationExceededEventHandler_box<F: FnMut(&::std::option::Option<MediaCapture>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const RecordLimitationExceededEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<MediaCapture>) -> ::windows::runtime::Result<()> + 'static> RecordLimitationExceededEventHandler_box<F> {
    const VTABLE: RecordLimitationExceededEventHandler_abi = RecordLimitationExceededEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<RecordLimitationExceededEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <MediaCapture as ::windows::runtime::Abi>::Abi as *const <MediaCapture as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ScreenCapture(::windows::runtime::IInspectable);
impl ScreenCapture {
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Capture`, `Media_Core`*"]
    pub fn AudioSource(&self) -> ::windows::runtime::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Capture`, `Media_Core`*"]
    pub fn VideoSource(&self) -> ::windows::runtime::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsAudioSuspended(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsVideoSuspended(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn SourceSuspensionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Capture`, `Foundation`*"]
    pub fn RemoveSourceSuspensionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<ScreenCapture> {
        Self::IScreenCaptureStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ScreenCapture>(result__)
        })
    }
    pub fn IScreenCaptureStatics<R, F: FnOnce(&IScreenCaptureStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ScreenCapture, IScreenCaptureStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScreenCapture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.ScreenCapture;{89179ef7-cd12-4e0e-a6d4-5b3de98b2e9b})");
}
unsafe impl ::windows::runtime::Interface for ScreenCapture {
    type Vtable = IScreenCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2300026615, 52498, 19982, [166, 212, 91, 61, 233, 139, 46, 155]);
}
impl ::windows::runtime::RuntimeName for ScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.ScreenCapture";
}
unsafe impl ::std::marker::Send for ScreenCapture {}
unsafe impl ::std::marker::Sync for ScreenCapture {}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SourceSuspensionChangedEventArgs(::windows::runtime::IInspectable);
impl SourceSuspensionChangedEventArgs {
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsAudioSuspended(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Capture`*"]
    pub fn IsVideoSuspended(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SourceSuspensionChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.SourceSuspensionChangedEventArgs;{2ece7b5e-d49b-4394-bc32-f97d6cedec1c})");
}
unsafe impl ::windows::runtime::Interface for SourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(785283934, 54427, 17300, [188, 50, 249, 125, 108, 237, 236, 28]);
}
impl ::windows::runtime::RuntimeName for SourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.SourceSuspensionChangedEventArgs";
}
unsafe impl ::std::marker::Send for SourceSuspensionChangedEventArgs {}
unsafe impl ::std::marker::Sync for SourceSuspensionChangedEventArgs {}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: StreamingCaptureMode = StreamingCaptureMode(0i32);
    pub const Audio: StreamingCaptureMode = StreamingCaptureMode(1i32);
    pub const Video: StreamingCaptureMode = StreamingCaptureMode(2i32);
}
impl ::std::convert::From<i32> for StreamingCaptureMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StreamingCaptureMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StreamingCaptureMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.StreamingCaptureMode;i4)");
}
impl ::windows::runtime::DefaultType for StreamingCaptureMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: VideoDeviceCharacteristic = VideoDeviceCharacteristic(0i32);
    pub const PreviewRecordStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(1i32);
    pub const PreviewPhotoStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(2i32);
    pub const RecordPhotoStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(3i32);
    pub const AllStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(4i32);
}
impl ::std::convert::From<i32> for VideoDeviceCharacteristic {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VideoDeviceCharacteristic {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VideoDeviceCharacteristic {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoDeviceCharacteristic;i4)");
}
impl ::windows::runtime::DefaultType for VideoDeviceCharacteristic {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: VideoRotation = VideoRotation(0i32);
    pub const Clockwise90Degrees: VideoRotation = VideoRotation(1i32);
    pub const Clockwise180Degrees: VideoRotation = VideoRotation(2i32);
    pub const Clockwise270Degrees: VideoRotation = VideoRotation(3i32);
}
impl ::std::convert::From<i32> for VideoRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VideoRotation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VideoRotation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoRotation;i4)");
}
impl ::windows::runtime::DefaultType for VideoRotation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Capture`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct VideoStreamConfiguration(::windows::runtime::IInspectable);
impl VideoStreamConfiguration {
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Capture`, `Media_MediaProperties`*"]
    pub fn InputProperties(&self) -> ::windows::runtime::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Capture`, `Media_MediaProperties`*"]
    pub fn OutputProperties(&self) -> ::windows::runtime::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoStreamConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.VideoStreamConfiguration;{d8770a6f-4390-4b5e-ad3e-0f8af0963490})");
}
unsafe impl ::windows::runtime::Interface for VideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3631680111, 17296, 19294, [173, 62, 15, 138, 240, 150, 52, 144]);
}
impl ::windows::runtime::RuntimeName for VideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.VideoStreamConfiguration";
}
unsafe impl ::std::marker::Send for VideoStreamConfiguration {}
unsafe impl ::std::marker::Sync for VideoStreamConfiguration {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Media_Capture`*"]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl WhiteBalanceGain {}
impl ::std::default::Default for WhiteBalanceGain {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WhiteBalanceGain {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WhiteBalanceGain").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::std::cmp::PartialEq for WhiteBalanceGain {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::std::cmp::Eq for WhiteBalanceGain {}
unsafe impl ::windows::runtime::Abi for WhiteBalanceGain {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WhiteBalanceGain {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Media.Capture.WhiteBalanceGain;f8;f8;f8)");
}
impl ::windows::runtime::DefaultType for WhiteBalanceGain {
    type DefaultType = Self;
}
